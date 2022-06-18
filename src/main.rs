use rand::prelude::*;
use three_d::*;

const TOTAL_STARS: u16 = 432;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    run();
}

struct StartMaterial {
    pub color: Vec3,
}

impl Material for StartMaterial {
    fn fragment_shader_source(&self, _use_vertex_colors: bool, _lights: &[&dyn Light]) -> String {
        include_str!("star.frag").to_string()
    }

    fn use_uniforms(
        &self,
        program: &Program,
        _camera: &Camera,
        _lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        program.use_uniform(
            "uIResolution",
            vec2(1920.0, 1080.0), // TODO get viewport
        ).unwrap();

        program.use_uniform(
            "uColor",
            vec3(
                self.color.x,
                self.color.y,
                self.color.z,
            ),
        ).unwrap();
        Ok(())
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            write_mask: WriteMask::COLOR_AND_DEPTH,
            depth_test: DepthTest::Always,
            blend: Blend::Enabled {
                source_rgb_multiplier: BlendMultiplierType::SrcColor,
                source_alpha_multiplier: BlendMultiplierType::Zero,
                destination_rgb_multiplier: BlendMultiplierType::Zero,
                destination_alpha_multiplier: BlendMultiplierType::One,
                rgb_equation: BlendEquationType::Add,
                alpha_equation: BlendEquationType::Add,
            },
            cull: Cull::Back,
        }
    }

    fn is_transparent(&self) -> bool {
        true
    }
}

pub fn run() {
    let window = Window::new(WindowSettings {
        title: "Rustshad".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    }).unwrap();

    let context = window.gl().unwrap();

    let camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(0.0, 30.0, 150.0),
        vec3(0.0, 30.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    ).unwrap();

    let mut star_material = StartMaterial {
        color: vec3(0.0, 0.0, 1.0),
    };

    let mut particles_data = Vec::new();
    for _ in 0..TOTAL_STARS {
        let start_position = get_rng_vec3(true);
        let start_velocity = get_rng_vec3(true);
        particles_data.push(ParticleData {
            start_position: vec3(
                100.0 * start_position.x,
                100.0 * start_position.y,
                100.0 * start_position.z,
            ),
            start_velocity,
        });
    }

    let mut cube = CpuMesh::cube();
    cube.transform(&Mat4::from_scale(0.4)).unwrap();

    cube.transform(&Mat4::from_angle_x(degrees(26.0))).unwrap();
    cube.transform(&Mat4::from_angle_y(degrees(12.0))).unwrap();
    cube.transform(&Mat4::from_angle_z(degrees(34.0))).unwrap();

    let mut particles = Particles::new(&context, &cube).unwrap();
    particles.acceleration = vec3(0.0, 0.0, 0.0);

    window
        .render_loop(move |mut frame_input| {;
            let elapsed_time = (frame_input.elapsed_time * 0.001) as f32;
            particles.time += elapsed_time;

            // create stars
            particles.update(&particles_data).unwrap();

            frame_input
                .screen()// 0.019, 0.003, 0.113
                .clear(ClearState::color(0.019, 0.003, 0.113, 1.0))
                .unwrap()
                .write(|| {
                    star_material.color = vec3(0.0, 0.2, 1.0);
                    particles.render_with_material(&star_material, &camera, &[])?;
                    particles.set_transformation(Mat4::from_angle_x(radians(elapsed_time.cos())));
                    Ok(())
                })
                .unwrap();

            FrameOutput::default()
        })
        .unwrap();
}

fn get_rng_vec3(is_signed: bool) -> Vec3 {
    let mut rng = rand::thread_rng();

    let rng_x = rng.gen::<f32>();
    let rng_y = rng.gen::<f32>();
    let rng_z = rng.gen::<f32>();

    if !is_signed {
        return vec3(rng_x, rng_y, rng_z);
    }

    let signed_x: f32 = if rng.gen::<f32>() > 0.5 { 1.0 } else { -1.0 };
    let signed_y: f32 = if rng.gen::<f32>() > 0.5 { 1.0 } else { -1.0 };
    let signed_z: f32 = if rng.gen::<f32>() > 0.5 { 1.0 } else { -1.0 };
    vec3(
        rng_x * signed_x,
        rng_y * signed_y,
        rng_z * signed_z,
    )
}