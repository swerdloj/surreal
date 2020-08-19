use wgpu::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Uniforms {
    pub color: cgmath::Vector4<f32>,
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}

#[repr(C)]
#[derive(Copy, Clone)]
/// repr(C)
struct QuadVertex {
    position: cgmath::Vector3<f32>,
}

unsafe impl bytemuck::Pod for QuadVertex {}
unsafe impl bytemuck::Zeroable for QuadVertex {}

impl QuadVertex {
    pub fn new(position: (f32, f32, f32)) -> Self {
        Self {
            position: position.into(),
        }
    }

    pub fn descriptor<'a>() -> VertexBufferDescriptor<'a> {
        VertexBufferDescriptor {
            stride: size_of!(Self),
            step_mode: InputStepMode::Vertex,
            attributes: &vertex_attr_array![
                // Vertex location
                0 => Float3
            ],
        }
    }
}

pub struct Quad {
    pub vertex_buffer: Buffer,
    pub uniform_buffer: Buffer,
    pub index_buffer: Buffer,
    
    pub uniform_bind_group: BindGroup,
}

impl Quad {
    fn vertices_from_rect(window_dimensions: (u32, u32), mut top_left: (i32, i32), width: u32, height: u32) -> [QuadVertex; 4] {
        // Make top-left (0, 0) where down is +y-axis
        top_left.1 = window_dimensions.1 as i32 - top_left.1;
        
        let quad_top_left = super::screen_space_to_draw_space(top_left, window_dimensions);
        
        let top_right = (top_left.0 + width as i32, top_left.1);
        let quad_top_right = super::screen_space_to_draw_space(top_right, window_dimensions);
        
        let bottom_left = (top_left.0, top_left.1 - height as i32);
        let quad_bottom_left = super::screen_space_to_draw_space(bottom_left, window_dimensions);
        
        let bottom_right = (top_left.0 + width as i32, top_left.1 - height as i32);
        let quad_bottom_right = super::screen_space_to_draw_space(bottom_right, window_dimensions);
    
        [
            QuadVertex::new((quad_top_right.0, quad_top_right.1, 0.0)), // Top right
            QuadVertex::new((quad_top_left.0, quad_top_left.1, 0.0)), // Top left
            QuadVertex::new((quad_bottom_left.0, quad_bottom_left.1, 0.0)), // Bottom left
            QuadVertex::new((quad_bottom_right.0, quad_bottom_right.1, 0.0)), // Bottom right
        ]
    }

    pub fn new(device: &Device, layout: &BindGroupLayout, window_dimensions: (u32, u32), top_left: (i32, i32), width: u32, height: u32) -> Self {
        let index_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(&[0u32, 1, 2, 0, 2, 3]), 
            BufferUsage::INDEX,
        );

        let vertices = Self::vertices_from_rect(window_dimensions, top_left, width, height);

        let vertex_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(&vertices), 
            BufferUsage::VERTEX, // | BufferUsage::COPY_DST,
        );

        let uniform_data = Uniforms {
            color: (1.0, 0.0, 0.0, 1.0).into(),
        };

        let uniform_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(&[uniform_data]),
            BufferUsage::UNIFORM | BufferUsage::COPY_DST, 
        );

        let uniform_bind_group = Self::bind_group(device, layout, &uniform_buffer);

        Self {
            vertex_buffer,
            uniform_buffer,
            index_buffer,
            uniform_bind_group,
        }
    }

    pub fn update_vertices(&mut self, device: &Device, window_dimensions: (u32, u32), top_left: (i32, i32), width: u32, height: u32) {
        let vertices = Self::vertices_from_rect(window_dimensions, top_left, width, height);
    
        // FIXME: Should I use a staging buffer or map_write() instead?
        self.vertex_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(&vertices),
            BufferUsage::VERTEX, // BufferUsage::COPY_SRC
        );
    }

    pub fn update_uniforms(&mut self, device: &Device, encoder: &mut CommandEncoder, uniforms: Uniforms) {
        // FIXME: Would map_write() be easier here?

        let staging_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(&[uniforms]), 
            BufferUsage::COPY_SRC,
        );

        encoder.copy_buffer_to_buffer(
            &staging_buffer, 0, 
            &self.uniform_buffer, 0, 
            size_of!(Uniforms)
        );
    }

    // Quad will always outlive the RenderPass
    pub fn render<'a, 'b>(&'a self, render_pass: &mut RenderPass<'b>/*, pipeline: &'b RenderPipeline */)
    where 'a: 'b {
        // render_pass.set_pipeline(pipeline);
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
        render_pass.set_index_buffer(&self.index_buffer, 0, 0);
        render_pass.draw_indexed(0..6, 0, 0..1);
    }

    pub fn bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            bindings: &[
                // Uniform buffer
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStage::FRAGMENT,
                    ty: BindingType::UniformBuffer {
                        dynamic: false,
                    },
                },
            ],
            label: Some("quad_bind_group_layout"),
        })
    }

    fn bind_group(device: &Device, layout: &BindGroupLayout, uniform_buffer: &Buffer) -> BindGroup {      
        device.create_bind_group(&BindGroupDescriptor {
            layout,
            bindings: &[
                Binding {
                    binding: 0,
                    resource: BindingResource::Buffer {
                        buffer: uniform_buffer,
                        range: 0..size_of!(Uniforms),
                    },
                },
            ],
            label: Some("quad_bind_group"),
        })
    }

    pub fn create_render_pipeline(device: &Device, 
                                  layout: &BindGroupLayout,
                                  color_format: TextureFormat,
    ) -> RenderPipeline {
        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            bind_group_layouts: &[
                layout,
            ],
        });

        let vert_shader = include_str!("../../shaders/quad.vert");
        let vert_spirv = glsl_to_spirv::compile(vert_shader, glsl_to_spirv::ShaderType::Vertex).unwrap();
        let vert_data = read_spirv(vert_spirv).unwrap();
        
        let frag_shader = include_str!("../../shaders/quad.frag");
        let frag_spirv = glsl_to_spirv::compile(frag_shader, glsl_to_spirv::ShaderType::Fragment).unwrap();
        let frag_data = read_spirv(frag_spirv).unwrap();

        let vert_module = device.create_shader_module(&vert_data);
        let frag_module = device.create_shader_module(&frag_data);

        device.create_render_pipeline(&RenderPipelineDescriptor {
            layout: &layout,
            vertex_stage: ProgrammableStageDescriptor {
                module: &vert_module,
                entry_point: "main",
            },
            fragment_stage: Some(ProgrammableStageDescriptor {
                module: &frag_module,
                entry_point: "main",
            }),
            rasterization_state: Some(RasterizationStateDescriptor {
                front_face: FrontFace::Ccw,
                cull_mode: CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            primitive_topology: PrimitiveTopology::TriangleList,
            // color_states: &[
            //     ColorStateDescriptor {
            //         format: color_format,
            //         alpha_blend: BlendDescriptor::REPLACE,
            //         color_blend: BlendDescriptor::REPLACE,
            //         write_mask: ColorWrite::ALL,
            //     },
            // ],
            // NOTE: The following enables simple alpha-blending
            color_states: &[wgpu::ColorStateDescriptor {
                format: color_format,
                color_blend: wgpu::BlendDescriptor {
                    src_factor: BlendFactor::SrcAlpha,
                    dst_factor: BlendFactor::OneMinusSrcAlpha,
                    operation: BlendOperation::Add,                 
                },
                alpha_blend: wgpu::BlendDescriptor {
                    src_factor: BlendFactor::One,
                    dst_factor: BlendFactor::One,
                    operation: BlendOperation::Add,                 
                },
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            vertex_state: VertexStateDescriptor {
                index_format: IndexFormat::Uint32,
                vertex_buffers: &[
                    QuadVertex::descriptor(),
                ],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        })
    }
}