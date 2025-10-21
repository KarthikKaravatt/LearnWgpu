use std::{env::current_dir, fs::read_to_string};

use wgpu::{
    PipelineCompilationOptions, PipelineLayoutDescriptor, RenderPipelineDescriptor,
    ShaderModuleDescriptor,
};

pub struct PipelineBuilder {
    shader_filename: String,
    vertex_entry: String,
    fragment_entry: String,
    pixel_format: wgpu::TextureFormat,
}

impl PipelineBuilder {
    pub fn new() -> Self {
        PipelineBuilder {
            shader_filename: "dummy".to_string(),
            vertex_entry: "dummy".to_string(),
            fragment_entry: "dummy".to_string(),
            pixel_format: wgpu::TextureFormat::Rgba8Unorm,
        }
    }
    pub fn set_shader_module(
        &mut self,
        shader_filename: &str,
        vertex_entry: &str,
        fragment_entry: &str,
    ) {
        self.shader_filename = shader_filename.to_string();
        self.vertex_entry = vertex_entry.to_string();
        self.fragment_entry = fragment_entry.to_string();
    }
    pub fn set_pixel_format(&mut self, pixel_format: wgpu::TextureFormat) {
        self.pixel_format = pixel_format;
    }

    pub fn build_pipeline(&mut self, device: &wgpu::Device) -> wgpu::RenderPipeline {
        let mut file_path = current_dir().unwrap();
        file_path.push("src/");
        file_path.push(self.shader_filename.as_str());
        let file_path = file_path.into_os_string().into_string().unwrap();
        println!("{file_path}");
        let source_code = read_to_string(file_path).expect("Can't read source code!");
        let shader_module_descriptor = ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: wgpu::ShaderSource::Wgsl(source_code.into()),
        };
        let shader_module = device.create_shader_module(shader_module_descriptor);
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor::default());
        let render_tarets = [Some(wgpu::ColorTargetState {
            format: self.pixel_format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        let render_pipeline_descriptor = RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: Some(&self.vertex_entry),
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: Some(&self.fragment_entry),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &render_tarets,
            }),
            multiview: None,
            cache: None,
        };
        device.create_render_pipeline(&render_pipeline_descriptor)
    }
}
