/*
MIT License

Copyright (c) 2021, 2022 Vincent Hiribarren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use demo_cube_wgpu::draw_context::DrawContext;
use demo_cube_wgpu::primitives::{cube, Object3D};
use demo_cube_wgpu::scenario::{Scenario, UpdateInterval};

const DEFAULT_SHADER: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/shaders/default.wgsl"
));
const DEFAULT_SHADER_MAIN_FRG: &str = "frg_main";
const DEFAULT_SHADER_MAIN_VTX: &str = "vtx_main";

const FLAT_SHADER: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/shaders/flat.wgsl"
));
const FLAT_SHADER_MAIN_FRG: &str = "frg_main";
const FLAT_SHADER_MAIN_VTX: &str = "vtx_main";

const ROTATION_DEG_PER_S: f32 = 45.0;

pub struct MainScenario {
    pub cube_left: Object3D,
    pub cube_right: Object3D,
}

impl Scenario for MainScenario {
    fn new(draw_context: &DrawContext) -> Self {
        let default_shader_module =
            draw_context
                .device
                .create_shader_module(&wgpu::ShaderModuleDescriptor {
                    label: Some("Vertex Shader"),
                    source: wgpu::ShaderSource::Wgsl(DEFAULT_SHADER.into()),
                });
        let default_vertex_state = wgpu::VertexState {
            module: &default_shader_module,
            entry_point: DEFAULT_SHADER_MAIN_VTX,
            buffers: &[draw_context.vertex_buffer_layout.clone()],
        };
        let default_fragment_state = wgpu::FragmentState {
            module: &default_shader_module,
            entry_point: DEFAULT_SHADER_MAIN_FRG,
            targets: &[wgpu::ColorTargetState {
                format: draw_context.surface_config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            }],
        };
        let flat_shader_module =
            draw_context
                .device
                .create_shader_module(&wgpu::ShaderModuleDescriptor {
                    label: Some("Vertex Shader"),
                    source: wgpu::ShaderSource::Wgsl(FLAT_SHADER.into()),
                });
        let flat_vertex_state = wgpu::VertexState {
            module: &flat_shader_module,
            entry_point: FLAT_SHADER_MAIN_VTX,
            buffers: &[draw_context.vertex_buffer_layout.clone()],
        };
        let flat_fragment_state = wgpu::FragmentState {
            module: &flat_shader_module,
            entry_point: FLAT_SHADER_MAIN_FRG,
            targets: &[wgpu::ColorTargetState {
                format: draw_context.surface_config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            }],
        };
        let mut cube_left =
            cube::create_cube(draw_context, default_vertex_state, default_fragment_state);
        let mut cube_right =
            cube::create_cube(draw_context, flat_vertex_state, flat_fragment_state);
        cube_left.apply_transform(
            draw_context,
            cgmath::Matrix4::from_translation(cgmath::Vector3::new(-0.5, 0.0, 5.0)),
        );
        cube_right.apply_transform(
            draw_context,
            cgmath::Matrix4::from_translation(cgmath::Vector3::new(0.5, 0.0, 0.0)),
        );
        Self {
            cube_left,
            cube_right,
        }
    }
    fn update(&mut self, context: &DrawContext, update_interval: &UpdateInterval) {
        let delta_rotation = ROTATION_DEG_PER_S * update_interval.update_delta.as_secs_f32();
        self.cube_left.apply_transform(
            context,
            cgmath::Matrix4::from_angle_z(cgmath::Deg(delta_rotation)),
        );
        self.cube_right.apply_transform(
            context,
            cgmath::Matrix4::from_angle_y(cgmath::Deg(delta_rotation)),
        );
    }
    fn render<'drawable, 'render>(
        &'drawable self,
        render_pass: &'render mut wgpu::RenderPass<'drawable>,
    ) {
        self.cube_right.as_ref().render(render_pass);
        self.cube_left.as_ref().render(render_pass);
    }
}
