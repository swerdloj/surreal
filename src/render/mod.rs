/*
TODO:

This module will be for everything related to rendering.
This includes quads and shaders.

The idea is to create a basic canvas-like API for rendering.
Example of what this would look like:

// Example 1
Button::render(render_target, ...) {
    render_target.rounded_rect(RoundedRect {
        at: self.rect.position,
        size: self.rect.size,
        color: Color::Red,
        border: Color::Black,
        border_width: 5,
    });
}

// Example 2
Thing::render(render_target, ...) {
    render_target.circle(Circle {
        radius: 10,
        ...
    });

    render_target.rect(Rect {
        ...
    });
}

*/

macro_rules! size_of {
    // Size of type
    ($T:ty) => {
        std::mem::size_of::<$T>()
    };
    
    // (Dynamic) Size of pointed-to value
    (var $I:ident) => {
        std::mem::size_of_val(&$I)
    };
}

// TEMP: Experimenting with quads for rendering

use wgpu::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Uniforms {
    color: cgmath::Vector3<f32>,
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
            stride: size_of!(Self) as _,
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
    pub index_buffer: Buffer,
    pub uniform_bind_group: BindGroup,
}

impl Quad {
    pub fn new(device: &Device, layout: &BindGroupLayout) -> Self {
        let index_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(&[0u32, 1, 2, 0, 2, 3]), 
            BufferUsage::INDEX,
        );

        let vertices = [
            QuadVertex::new(( 0.5,  1., 0.0)), // Top right
            QuadVertex::new((-0.5,  1., 0.0)), // Top left
            QuadVertex::new((-0.5, -0.5, 0.0)), // Bottom left
            QuadVertex::new(( 0.5, -0.5, 0.0)), // Bottom right
        ];

        let vertex_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(&vertices), 
            BufferUsage::VERTEX,
        );

        let uniform_data = Uniforms {
            color: (0.2, 0.2, 0.5).into(),
        };

        let uniform_bind_group = Self::bind_group(device, layout, uniform_data);

        Self {
            vertex_buffer,
            index_buffer,
            uniform_bind_group,
        }
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

    pub fn bind_group(device: &Device, layout: &BindGroupLayout, uniform_data: Uniforms) -> BindGroup {
        let buffer = &device.create_buffer_with_data(
            bytemuck::cast_slice(&[uniform_data]), 
            BufferUsage::UNIFORM,
        );
        
        device.create_bind_group(&BindGroupDescriptor {
            layout,
            bindings: &[
                Binding {
                    binding: 0,
                    resource: BindingResource::Buffer {
                        buffer,
                        range: 0..size_of!(var uniform_data) as _,
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
                format: crate::TEXTURE_FORMAT,
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