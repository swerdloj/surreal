use wgpu::*;
use wgpu::util::{DeviceExt, BufferInitDescriptor};
use wgpu::Texture as wgpu_Texture;

pub struct TextureMap {
    textures: std::collections::HashMap<&'static str, Texture>,
}

impl TextureMap {
    pub fn from_images(images: crate::widget::IncludedImages, device: &Device, queue: &Queue) -> Self {
        let mut textures = std::collections::HashMap::new();

        for (alias, image) in images {
            let texture = Texture::new(image, device, queue);
            textures.insert(alias, texture);
        }

        Self {
            textures,
        }
    }

    pub fn get<'a>(&'a self, alias: &str) -> &'a Texture {
        if let Some(texture) = self.textures.get(alias) {
            texture
        } else {
            panic!("No such texture exists: `{}`", alias);
        }
    }
}

pub struct Texture {
    width: u32,
    height: u32,
    texture: wgpu_Texture,
    view: TextureView,
    sampler: Sampler,
}

// TODO: Pipelines
// TODO: Vertices
// TODO: Vertex Buffers
// TODO: Uniforms
// TODO: Bind Groups

impl Texture {
    pub fn new(image_resource: image::DynamicImage, device: &Device, queue: &Queue) -> Self {
        let rgba_image = image_resource.as_rgba8().unwrap();

        let (width, height) = rgba_image.dimensions();

        let size = Extent3d {
            width,
            height,
            depth: 1, // 2d texture
        };

        let texture = device.create_texture(&TextureDescriptor {
            label: Some("image_resource"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: crate::TEXTURE_FORMAT,
            // Sampled => use in shaders
            usage: TextureUsage::SAMPLED | TextureUsage::COPY_DST,
        });

        // Writes to texture from buffer (rgba_image)
        queue.write_texture(
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            rgba_image,
            wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: 4 * width, // RGBA8
                rows_per_image: height,
            },
            size
        );

        // FIXME: Either use the same sampler for all images, or give user some control
        //        over this. Making the same sampler for each image is wasteful.
        let sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("image_sampler"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            width,
            height,
            view: texture.create_view(&TextureViewDescriptor::default()),
            texture,
            sampler,
        }
    }

    fn bind_group_layout(device: &Device) -> BindGroupLayout {
        let layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("image_bind_group_layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStage::FRAGMENT,
                    ty: BindingType::SampledTexture {
                        dimension: TextureViewDimension::D2,
                        component_type: TextureComponentType::Uint,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStage::FRAGMENT,
                    ty: BindingType::Sampler {
                        comparison: false,
                    },
                    count: None,
                }
            ],
        });

        todo!()
        // TODO: Do I need these to be in separate functions?

        // device.create_bind_group(&BindGroupDescriptor {
        //     label: Some("image_bind_group"),
        //     layout: &layout,
        //     entries: &[
        //         BindGroupEntry {
        //             binding: 0,
        //             resource: BindingResource::TextureView(),
        //         },
        //         BindGroupEntry {
        //             binding: 1,
        //             resource: BindingResource::Sampler(),
                    
        //         }
        //     ],
        // })
    }

    fn bind_group(device: &Device, layout: &BindGroupLayout, uniform_buffer: &Buffer) -> BindGroup {
        todo!()
    }

    fn create_render_pipeline(device: &Device, layout: &BindGroupLayout) -> RenderPipeline {
        todo!()
    }
}