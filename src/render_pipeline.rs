use std::num::NonZero;

use wgpu::{
    Device,
    VertexBufferLayout,
};

pub struct RenderPipeline {
    render_pipeline: wgpu::RenderPipeline,
}

impl RenderPipeline {
    pub fn render_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.render_pipeline
    }
}

#[derive(Default)]
pub struct Base;

#[derive(Default)]
pub struct WithLabel;

#[derive(Default)]
pub struct WithVertexState;

#[derive(Default)]
pub struct WithPrimitiveState;

#[derive(Default)]
pub struct WithMultisampleState;

#[derive(Default)]
pub struct RenderPipelineBuilder<'a, S> {
    label: &'a str,
    layout: Option<&'a wgpu::PipelineLayout>,
    vertex_state: Option<wgpu::VertexState<'a>>,
    primitive_state: Option<wgpu::PrimitiveState>,
    multisample_state: Option<wgpu::MultisampleState>,
    fragment_state: Option<wgpu::FragmentState<'a>>,
    depth_stencil_state: Option<wgpu::DepthStencilState>,
    multiview: Option<NonZero<u32>>,
    _state: std::marker::PhantomData<S>,
}

impl<'a> RenderPipelineBuilder<'a, Base> {
    pub fn new(pipeline_label: &'a str) -> RenderPipelineBuilder<'a, WithLabel> {
        RenderPipelineBuilder {
            label: pipeline_label,
            ..Default::default()
        }
    }
}

impl<'a> RenderPipelineBuilder<'a, WithLabel> {
    pub fn vertex_state(
        mut self,
        shader_module: &'a wgpu::ShaderModule,
        entry_point: &'a str,
        buffer_layouts: &'a [VertexBufferLayout<'a>],
    ) -> RenderPipelineBuilder<'a, WithVertexState> {
        self.vertex_state = Some(wgpu::VertexState {
            module: shader_module,
            entry_point: Some(entry_point),
            buffers: buffer_layouts,
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        });

        RenderPipelineBuilder {
            label: self.label,
            layout: self.layout,
            vertex_state: self.vertex_state,
            ..Default::default()
        }
    }
}

impl<'a> RenderPipelineBuilder<'a, WithVertexState> {
    pub fn primitive_state(
        mut self,
        primitive_state: wgpu::PrimitiveState,
    ) -> RenderPipelineBuilder<'a, WithPrimitiveState> {
        self.primitive_state = Some(primitive_state);

        RenderPipelineBuilder {
            label: self.label,
            layout: self.layout,
            vertex_state: self.vertex_state,
            primitive_state: self.primitive_state,
            ..Default::default()
        }
    }
}

impl<'a> RenderPipelineBuilder<'a, WithPrimitiveState> {
    pub fn multisample_state(
        mut self,
        multisample_state: wgpu::MultisampleState,
    ) -> RenderPipelineBuilder<'a, WithMultisampleState> {
        self.multisample_state = Some(multisample_state);

        RenderPipelineBuilder {
            label: self.label,
            layout: self.layout,
            vertex_state: self.vertex_state,
            primitive_state: self.primitive_state,
            multisample_state: self.multisample_state,
            ..Default::default()
        }
    }
}

impl<'a> RenderPipelineBuilder<'a, WithMultisampleState> {
    pub fn build(self, device: &wgpu::Device) -> RenderPipeline {
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(self.label),
            layout: self.layout,
            vertex: self.vertex_state.unwrap(),
            primitive: self.primitive_state.unwrap(),
            multisample: self.multisample_state.unwrap(),
            fragment: self.fragment_state,
            depth_stencil: self.depth_stencil_state,
            multiview: self.multiview,
            cache: None,
        });

        RenderPipeline { render_pipeline }
    }

    pub fn layout(mut self, layout: &'a wgpu::PipelineLayout) -> Self {
        self.layout = Some(layout);
        self
    }

    pub fn fragment_state(
        mut self,
        shader_module: &'a wgpu::ShaderModule,
        entry_point: &'a str,
        color_targets: &'a [Option<wgpu::ColorTargetState>],
    ) -> Self {
        self.fragment_state = Some(wgpu::FragmentState {
            module: shader_module,
            entry_point: Some(entry_point),
            targets: color_targets,
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        });
        self
    }

    pub fn depth_stencil_state(self) -> Self {
        todo!("Implement depth stencil support");
        self
    }

    pub fn multiview(self) -> Self {
        todo!("Implement multiview support");
        self
    }
}
