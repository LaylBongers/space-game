use {
    ggez::{
        Context, GameResult,
        graphics,
    },
    gfx::{
        traits::{FactoryExt},
        texture,
        self, PipelineState, Slice, Factory,
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
        rotation: [[f32; 4]; 4] = "u_Rotation",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::Global<[[f32; 4]; 4]> = "u_Transform",
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        color: gfx::TextureSampler<[f32; 4]> = "t_Color",
        out_color: gfx::RenderTarget<ColorFormat> = "Target0",
        out_depth: gfx::DepthTarget<DepthFormat> =
            gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

impl Vertex {
    fn new(p: [i8; 3], t: [i8; 2]) -> Vertex {
        Vertex {
            pos: [f32::from(p[0]), f32::from(p[1]), f32::from(p[2]), 1.0],
            tex_coord: [f32::from(t[0]), f32::from(t[1])],
        }
    }
}

fn default_view() -> Isometry3<f32> {
    // Eye location, target location, up-vector
    Isometry3::look_at_rh(
        &Point3::new(1.5f32, -5.0, 3.0),
        &Point3::new(0f32, 0.0, 0.0),
        &Vector3::z_axis(),
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

        // Shaders.
        let vs =
br#"#version 150 core

in vec4 a_Pos;
in vec2 a_TexCoord;
out vec2 v_TexCoord;

uniform Locals {
    mat4 u_Transform;
    mat4 u_Rotation;
};

void main() {
    v_TexCoord = a_TexCoord;
    gl_Position = u_Transform * u_Rotation * a_Pos ;
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

        // Cube geometry
        let vertex_data = [
            // top (0, 0, 1)
            Vertex::new([-1, -1, 1], [0, 0]),
            Vertex::new([1, -1, 1], [1, 0]),
            Vertex::new([1, 1, 1], [1, 1]),
            Vertex::new([-1, 1, 1], [0, 1]),
            // bottom (0, 0, -1)
            Vertex::new([-1, 1, -1], [1, 0]),
            Vertex::new([1, 1, -1], [0, 0]),
            Vertex::new([1, -1, -1], [0, 1]),
            Vertex::new([-1, -1, -1], [1, 1]),
            // right (1, 0, 0)
            Vertex::new([1, -1, -1], [0, 0]),
            Vertex::new([1, 1, -1], [1, 0]),
            Vertex::new([1, 1, 1], [1, 1]),
            Vertex::new([1, -1, 1], [0, 1]),
            // left (-1, 0, 0)
            Vertex::new([-1, -1, 1], [1, 0]),
            Vertex::new([-1, 1, 1], [0, 0]),
            Vertex::new([-1, 1, -1], [0, 1]),
            Vertex::new([-1, -1, -1], [1, 1]),
            // front (0, 1, 0)
            Vertex::new([1, 1, -1], [1, 0]),
            Vertex::new([-1, 1, -1], [0, 0]),
            Vertex::new([-1, 1, 1], [0, 1]),
            Vertex::new([1, 1, 1], [1, 1]),
            // back (0, -1, 0)
            Vertex::new([1, -1, 1], [0, 0]),
            Vertex::new([-1, -1, 1], [1, 0]),
            Vertex::new([-1, -1, -1], [1, 1]),
            Vertex::new([1, -1, -1], [0, 1]),
        ];

        #[cfg_attr(rustfmt, rustfmt_skip)]
        let index_data: &[u16] = &[
             0,  1,  2,  2,  3,  0, // top
             4,  5,  6,  6,  7,  4, // bottom
             8,  9, 10, 10, 11,  8, // right
            12, 13, 14, 14, 15, 12, // left
            16, 17, 18, 18, 19, 16, // front
            20, 21, 22, 22, 23, 20, // back
        ];

        // Create vertex buffer
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, index_data);

        // Create 1-pixel blue texture.
        let texels = [[0x20, 0xA0, 0xC0, 0x00]];
        let (_, texture_view) = factory
            .create_texture_immutable::<gfx::format::Rgba8>(
                texture::Kind::D2(1, 1, texture::AaMode::Single),
                texture::Mipmap::Provided,
                &[&texels],
            )
            .unwrap();

        let sinfo =
            texture::SamplerInfo::new(texture::FilterMethod::Bilinear, texture::WrapMode::Clamp);

        // Create pipeline state object
        let pso = factory.create_pipeline_simple(vs, fs, pipe::new()).unwrap();

        // Aspect ratio, FOV, znear, zfar
        let proj = Perspective3::new(4.0 / 3.0, ::std::f32::consts::PI / 4.0, 1.0, 10.0);
        let transform = proj.as_matrix() * default_view().to_homogeneous();

        // Bundle all the data together.
        let data = pipe::Data {
            vbuf,
            transform: transform.into(),
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

            let rotation = Matrix4::from_scaled_axis(Vector3::z() * rotation);

            let locals = Locals {
                transform: self.data.transform,
                rotation: rotation.into(),
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
