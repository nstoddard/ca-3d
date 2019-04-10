#![deny(bare_trait_objects)]

mod ca;

use cgmath::*;
use fnv::*;
use itertools::*;
use rand::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;
use web_sys::*;
use webgl_gui::Color4;
use webgl_gui::*;
use webgl_wrapper::uniforms::*;
use webgl_wrapper::*;

use ca::*;

const CA_SIZE: usize = 100;

const MESH_SIZE_PER_AXIS: usize = 20;

// This is called "plain" because it doesn't include a texture
struct PlainVert3D {
    pos: Point3<f32>,
    color: Color4,
}

impl VertexData for PlainVert3D {
    const ATTRIBUTES: Attributes = &[("pos", 3), ("color", 4)];
}

impl VertexComponent for PlainVert3D {
    fn add_to_mesh(&self, f: &mut dyn FnMut(f32)) {
        self.pos.add_to_mesh(f);
        self.color.add_to_mesh(f);
    }
}

struct PlainUniforms {
    matrix: Matrix4<f32>,
    color: Color4,
}

struct PlainUniformsGl {
    matrix: Matrix4Uniform,
    color: Color4Uniform,
}

impl Uniforms for PlainUniforms {
    type GlUniforms = PlainUniformsGl;

    fn update(&self, context: &GlContext, gl_uniforms: &Self::GlUniforms) {
        gl_uniforms.matrix.set(context, &self.matrix);
        gl_uniforms.color.set(context, &self.color, false);
    }
}

impl GlUniforms for PlainUniformsGl {
    fn new(context: &GlContext, program: &WebGlProgram) -> Self {
        let matrix = Matrix4Uniform::new("matrix", context, program);
        let color = Color4Uniform::new("uniColor", context, program);
        PlainUniformsGl { matrix, color }
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Trace).unwrap();

    if let Ok((context, screen_surface)) = GlContext::new(CANVAS_ID) {
        let ca_gui = CaGui::new(context, screen_surface);
        start_main_loop(CANVAS_ID, Box::new(ca_gui));
    } else {
        window().unwrap().alert_with_message("Unable to create WebGL 2 context. Try reloading the page; if that doesn't work, switch to Firefox or Chrome.").unwrap();
    }

    Ok(())
}

const CANVAS_ID: &str = "canvas";

struct CaGui {
    context: GlContext,
    screen_surface: ScreenSurface,
    draw_2d: Draw2d,
    ca: CA,
    ca_mesh_builder: MeshBuilder<PlainVert3D, Triangles>,
    ca_mesh: Mesh<PlainVert3D, PlainUniformsGl, Triangles>,
    ca_mesh_edges_builder: MeshBuilder<PlainVert3D, Lines>,
    ca_mesh_edges: Mesh<PlainVert3D, PlainUniformsGl, Lines>,
}

impl CaGui {
    pub fn new(context: GlContext, screen_surface: ScreenSurface) -> Self {
        let plain_program: GlProgram<PlainVert3D, PlainUniformsGl> = GlProgram::new_with_header(
            &context,
            include_str!("../shaders/plain_vert_3d.glsl"),
            include_str!("../shaders/plain_frag.glsl"),
            true,
        );
        let mut rng = SmallRng::from_entropy();
        Self {
            ca: CA::new(&mut rng, vec3(CA_SIZE, CA_SIZE, CA_SIZE)),
            ca_mesh_builder: MeshBuilder::new(),
            ca_mesh: Mesh::new(&context, &plain_program, DrawMode::Draw3D),
            ca_mesh_edges_builder: MeshBuilder::new(),
            ca_mesh_edges: Mesh::new(&context, &plain_program, DrawMode::Draw3D),
            draw_2d: Draw2d::new(&context),
            context,
            screen_surface,
        }
    }

    pub fn draw(&mut self, _cursor_pos: Option<Point2<i32>>) {
        self.screen_surface
            .clear(&self.context, &[ClearBuffer::Color(Color4::BLACK.into()), ClearBuffer::Depth]);

        self.ca.update();

        // TODO: don't draw faces between adjacent cubes
        let surface_size = self.screen_surface.size();
        let look_at = Matrix4::look_at(
            point3(CA_SIZE as f32 * 0.5, CA_SIZE as f32 * 0.5, -(CA_SIZE as f32) * 0.5),
            point3(CA_SIZE as f32 * 0.5, CA_SIZE as f32 * 0.5, CA_SIZE as f32 * 0.5),
            vec3(0.0, 1.0, 0.0),
        );
        let perspective =
            perspective(Deg(90.0), surface_size.x as f32 / surface_size.y as f32, 1.0, 200.0);
        let matrix = perspective * look_at;

        // Each mesh can only contain 2^16 vertices, so MESH_SIZE_PER_AXIS limits the size of
        // each mesh so this limit is never exceeded.
        for z in &(0..self.ca.size.z).chunks(MESH_SIZE_PER_AXIS) {
            // TODO: can these calls to collect() be avoided?
            let z: Vec<usize> = z.collect();
            for y in &(0..self.ca.size.y).chunks(MESH_SIZE_PER_AXIS) {
                let y: Vec<usize> = y.collect();
                for x in &(0..self.ca.size.x).chunks(MESH_SIZE_PER_AXIS) {
                    let x: Vec<usize> = x.collect();
                    // TODO: optimize rendering; can this be done more efficiently with
                    // instancing or other techniques?
                    self.ca_mesh_builder.clear();
                    for &z in &z {
                        for &y in &y {
                            for &x in &x {
                                if self.ca.grid[z][y][x] {
                                    let color = compute_cell_color(x, y, z);
                                    let pos = point3(x as f32, y as f32, z as f32);
                                    cube(&mut self.ca_mesh_builder, pos, color, 1.0);
                                }
                            }
                        }
                    }
                    self.ca_mesh.build_from(&self.ca_mesh_builder, MeshUsage::DynamicDraw);
                    self.ca_mesh.draw(
                        &self.screen_surface,
                        &PlainUniforms { matrix, color: Color4::WHITE },
                    );

                    self.ca_mesh_edges_builder.clear();
                    for &z in &z {
                        for &y in &y {
                            for &x in &x {
                                if self.ca.grid[z][y][x] {
                                    let pos = point3(x as f32, y as f32, z as f32);
                                    cube_edges(
                                        &mut self.ca_mesh_edges_builder,
                                        pos - vec3(0.05, 0.05, 0.05),
                                        Color4::BLACK,
                                        1.1,
                                    );
                                }
                            }
                        }
                    }
                    self.ca_mesh_edges
                        .build_from(&self.ca_mesh_edges_builder, MeshUsage::DynamicDraw);
                    self.ca_mesh_edges.draw(
                        &self.screen_surface,
                        &PlainUniforms { matrix, color: Color4::WHITE },
                    );
                }
            }
        }

        self.draw_2d.render_queued(&self.screen_surface);
    }
}

fn cube(
    mesh_builder: &mut MeshBuilder<PlainVert3D, Triangles>,
    pos: Point3<f32>,
    color: Color4,
    scale: f32,
) {
    let a = mesh_builder.vert(PlainVert3D { pos: pos + vec3(0.0, 0.0, 0.0) * scale, color });
    let b = mesh_builder.vert(PlainVert3D { pos: pos + vec3(1.0, 0.0, 0.0) * scale, color });
    let c = mesh_builder.vert(PlainVert3D { pos: pos + vec3(0.0, 1.0, 0.0) * scale, color });
    let d = mesh_builder.vert(PlainVert3D { pos: pos + vec3(1.0, 1.0, 0.0) * scale, color });
    let e = mesh_builder.vert(PlainVert3D { pos: pos + vec3(0.0, 0.0, 1.0) * scale, color });
    let f = mesh_builder.vert(PlainVert3D { pos: pos + vec3(1.0, 0.0, 1.0) * scale, color });
    let g = mesh_builder.vert(PlainVert3D { pos: pos + vec3(0.0, 1.0, 1.0) * scale, color });
    let h = mesh_builder.vert(PlainVert3D { pos: pos + vec3(1.0, 1.0, 1.0) * scale, color });
    mesh_builder.triangle(a, c, b);
    mesh_builder.triangle(b, c, d);

    mesh_builder.triangle(a, b, e);
    mesh_builder.triangle(b, f, e);

    mesh_builder.triangle(a, e, c);
    mesh_builder.triangle(c, e, g);

    mesh_builder.triangle(h, g, f);
    mesh_builder.triangle(g, e, f);

    mesh_builder.triangle(h, d, g);
    mesh_builder.triangle(g, d, c);

    mesh_builder.triangle(h, f, d);
    mesh_builder.triangle(f, b, d);
}

fn cube_edges(
    mesh_builder: &mut MeshBuilder<PlainVert3D, Lines>,
    pos: Point3<f32>,
    color: Color4,
    scale: f32,
) {
    let a = mesh_builder.vert(PlainVert3D { pos: pos + vec3(0.0, 0.0, 0.0) * scale, color });
    let b = mesh_builder.vert(PlainVert3D { pos: pos + vec3(1.0, 0.0, 0.0) * scale, color });
    let c = mesh_builder.vert(PlainVert3D { pos: pos + vec3(0.0, 1.0, 0.0) * scale, color });
    let d = mesh_builder.vert(PlainVert3D { pos: pos + vec3(1.0, 1.0, 0.0) * scale, color });
    let e = mesh_builder.vert(PlainVert3D { pos: pos + vec3(0.0, 0.0, 1.0) * scale, color });
    let f = mesh_builder.vert(PlainVert3D { pos: pos + vec3(1.0, 0.0, 1.0) * scale, color });
    let g = mesh_builder.vert(PlainVert3D { pos: pos + vec3(0.0, 1.0, 1.0) * scale, color });
    let h = mesh_builder.vert(PlainVert3D { pos: pos + vec3(1.0, 1.0, 1.0) * scale, color });
    mesh_builder.line(a, b);
    mesh_builder.line(a, c);
    mesh_builder.line(b, d);
    mesh_builder.line(c, d);

    mesh_builder.line(a, e);
    mesh_builder.line(b, f);
    mesh_builder.line(c, g);
    mesh_builder.line(d, h);

    mesh_builder.line(e, f);
    mesh_builder.line(e, g);
    mesh_builder.line(f, h);
    mesh_builder.line(g, h);
}

fn compute_cell_color(_x: usize, _y: usize, z: usize) -> Color4 {
    Color4::WHITE.lerp(Color4::WHITE.mul_srgb(0.3), z as f32 / CA_SIZE as f32)
}

impl App for CaGui {
    fn render_frame(&mut self, _events: Vec<Event>, event_state: &EventState, _dt: f64) {
        self.draw(event_state.cursor_pos);
    }
}
