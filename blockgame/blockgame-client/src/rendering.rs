use {
    ggez::{
        Context, GameResult,
        graphics,
    },
    gfx::{
        traits::{FactoryExt},
        texture::{SamplerInfo, Kind, Mipmap, AaMode, FilterMethod, WrapMode},
        self, PipelineState, Slice, Factory, VertexBuffer, ConstantBuffer, TextureSampler,
        RenderTarget, DepthTarget,
    },
    gfx_device_gl::{Resources},

    nalgebra::{Perspective3, Isometry3, Point3, Vector3, Matrix4},
};

type ColorFormat = gfx::format::Srgba8;
type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 4] = "a_Pos",
        tex_coord: [f32; 2] = "a_TexCoord",
    }

    constant Locals {
        transform: [[f32; 4]; 4] = "u_Transform",
    }

    pipeline pipe {
        vbuf: VertexBuffer<Vertex> = (),
        locals: ConstantBuffer<Locals> = "Locals",
        color: TextureSampler<[f32; 4]> = "t_Color",
        out_color: RenderTarget<ColorFormat> = "Target0",
        out_depth: DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

fn default_view(height: f32) -> Isometry3<f32> {
    Isometry3::look_at_rh(
        // Eye location
        &Point3::new(1.5f32, 3.0 + height, -5.0),
        // Target location
        &Point3::new(0f32, 0.0 + height, 0.0),
        // Up Vector
        &Vector3::y_axis(),
    )
}

pub struct Renderer {
    data: pipe::Data<Resources>,
    pso: PipelineState<Resources, pipe::Meta>,
    slice: Slice<Resources>,
}

impl Renderer {
    pub fn new(ctx: &mut Context) -> Self {
        let color_view = graphics::get_screen_render_target(ctx);
        let depth_view = graphics::get_depth_view(ctx);
        let factory = graphics::get_factory(ctx);

        let vs =
br#"#version 150 core

in vec4 a_Pos;
in vec2 a_TexCoord;
out vec2 v_TexCoord;

uniform Locals {
    mat4 u_Transform;
};

void main() {
    v_TexCoord = a_TexCoord;
    gl_Position = u_Transform * a_Pos ;
    gl_ClipDistance[0] = 1.0;
}"#;
        let fs =
br#"#version 150 core

in vec2 v_TexCoord;
out vec4 Target0;

uniform sampler2D t_Color;

void main() {
    vec4 tex = texture(t_Color, v_TexCoord);
    float blend = dot(v_TexCoord-vec2(0.5,0.5), v_TexCoord-vec2(0.5,0.5));
    Target0 = mix(tex, vec4(0.0,0.0,0.0,0.0), blend*1.0);
}"#;

        // Add some cubes
        let mut vertices = Vec::new();
        for x in 0..10 {
            for z in 0..10 {
                add_cube_vertices(&mut vertices, Vector3::new(x as f32, 0.0, z as f32));
            }
        }

        // Create vertex buffer
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertices, ());

        // Create 1-pixel blue texture.
        let texels = [[0x20, 0xA0, 0xC0, 0x00]];
        let (_, texture_view) = factory
            .create_texture_immutable::<gfx::format::Rgba8>(
                Kind::D2(1, 1, AaMode::Single),
                Mipmap::Provided,
                &[&texels],
            )
            .unwrap();

        let sinfo = SamplerInfo::new(FilterMethod::Bilinear, WrapMode::Clamp);

        // Create pipeline state object
        let set = factory.create_shader_set(vs, fs).unwrap();
        let pso = factory.create_pipeline_state(
            &set,
            gfx::Primitive::TriangleList,
            gfx::state::Rasterizer::new_fill().with_cull_back(),
            pipe::new()
        ).unwrap();

        // Bundle all the data together.
        let data = pipe::Data {
            vbuf,
            locals: factory.create_constant_buffer(1),
            color: (texture_view, factory.create_sampler(sinfo)),
            out_color: color_view,
            out_depth: depth_view,
        };

        Renderer {
            data,
            pso,
            slice,
        }
    }

    pub fn draw(&self, ctx: &mut Context, rotation: f32) -> GameResult<()> {
        graphics::set_background_color(ctx, (10, 10, 15).into());
        graphics::clear(ctx);

        {
            let (_factory, device, encoder, _depthview, _colorview) =
                graphics::get_gfx_objects(ctx);
            encoder.clear(&self.data.out_color, [0.1, 0.1, 0.1, 1.0]);

            // Aspect ratio, FOV, znear, zfar
            let proj = Perspective3::new(4.0 / 3.0, ::std::f32::consts::PI / 4.0, 1.0, 100.0);
            let transform = proj.as_matrix() * default_view(5.0).to_homogeneous();
            let transform = transform * Matrix4::from_scaled_axis(Vector3::y() * rotation);

            let locals = Locals {
                transform: transform.into(),
            };
            encoder.update_constant_buffer(&self.data.locals, &locals);
            encoder.clear_depth(&self.data.out_depth, 1.0);

            encoder.draw(&self.slice, &self.pso, &self.data);
            encoder.flush(device);
        }

        graphics::present(ctx);

        Ok(())
    }
}

fn add_cube_vertices(vertices: &mut Vec<Vertex>, offset: Vector3<f32>) {
    let points = [
        [
            [
                Point3::new(0.0, 0.0, 0.0) + offset,
                Point3::new(0.0, 0.0, 1.0) + offset,
            ],
            [
                Point3::new(0.0, 1.0, 0.0) + offset,
                Point3::new(0.0, 1.0, 1.0) + offset,
            ],
        ],
        [
            [
                Point3::new(1.0, 0.0, 0.0) + offset,
                Point3::new(1.0, 0.0, 1.0) + offset,
            ],
            [
                Point3::new(1.0, 1.0, 0.0) + offset,
                Point3::new(1.0, 1.0, 1.0) + offset,
            ],
        ],
    ];

    // front (0, 0, 1)
    add_plane_vertices(vertices,
        points[0][0][1], points[1][0][1], points[0][1][1], points[1][1][1],
    );

    // back (0, 0, -1)
    add_plane_vertices(vertices,
        points[0][1][0], points[1][1][0], points[0][0][0], points[1][0][0],
    );

    // right (1, 0, 0)
    add_plane_vertices(vertices,
        points[1][0][0], points[1][1][0], points[1][0][1], points[1][1][1],
    );

    // left (-1, 0, 0)
    add_plane_vertices(vertices,
        points[0][1][0], points[0][0][0], points[0][1][1], points[0][0][1],
    );

    // top (0, 1, 0)
    add_plane_vertices(vertices,
        points[1][1][0], points[0][1][0], points[1][1][1], points[0][1][1],
    );

    // bottom (0, -1, 0)
    add_plane_vertices(vertices,
        points[0][0][0], points[1][0][0], points[0][0][1], points[1][0][1],
    );
}

fn add_plane_vertices(
    vertices: &mut Vec<Vertex>,
    lb: Point3<f32>, rb: Point3<f32>, lt: Point3<f32>, rt: Point3<f32>
) {
    vertices.push(Vertex { pos: nvtp(lb), tex_coord: [0.0, 0.0] });
    vertices.push(Vertex { pos: nvtp(rb), tex_coord: [1.0, 0.0] });
    vertices.push(Vertex { pos: nvtp(rt), tex_coord: [1.0, 1.0] });

    vertices.push(Vertex { pos: nvtp(lb), tex_coord: [0.0, 0.0] });
    vertices.push(Vertex { pos: nvtp(rt), tex_coord: [1.0, 1.0] });
    vertices.push(Vertex { pos: nvtp(lt), tex_coord: [0.0, 1.0] });
}

fn nvtp(v: Point3<f32>) -> [f32; 4] {
    [v.x, v.y, v.z, 1.0]
}
