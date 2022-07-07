use rand::prelude::*;
use three_d::*;

const TOTAL_STARS: u16 = 1111;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    run();
}

#[derive(Clone, Copy)]
struct StarMaterial {
    time: f32,
}

impl Material for StarMaterial {
    fn fragment_shader_source(&self, _use_vertex_colors: bool, lights: &[&dyn Light]) -> String {
        // let mut shader = lights_shader_source(lights, LightingModel::Blinn);
        let mut shader = String::new();
        shader.push_str(include_str!("star.frag"));

        shader
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Transparent
    }

    fn use_uniforms(&self, program: &Program, camera: &Camera, lights: &[&dyn Light]) {
        program.use_uniform_if_required("cameraPosition", camera.position());
        program.use_uniform("uTime", self.time);
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            write_mask: WriteMask::COLOR_AND_DEPTH,
            depth_test: DepthTest::Always,
            blend: Blend::Enabled {
                source_rgb_multiplier: BlendMultiplierType::SrcColor,
                source_alpha_multiplier: BlendMultiplierType::Zero,
                destination_rgb_multiplier: BlendMultiplierType::One,
                destination_alpha_multiplier: BlendMultiplierType::One,
                rgb_equation: BlendEquationType::Add,
                alpha_equation: BlendEquationType::Add,
            },
            cull: Cull::Back,
        }
    }
}

pub fn run() {
    let window = Window::new(WindowSettings {
        title: "Rustshad".to_string(),
        max_size: Some((1920, 1080)),
        ..Default::default()
    }).unwrap();

    let context = window.gl();

    let camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 30.0, 150.0),
        vec3(0.0, 30.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );

    let mut start_positions = Vec::new();
    let mut start_velocities = Vec::new();
    let mut start_rotations = Vec::new();
    let mut start_scales = Vec::new();
    let mut part_colors = Vec::new();
    for i in 0..TOTAL_STARS {
        let start_position = get_rng_vec3(true);
        let colors = get_rng_vec3(false);
        let rotations = get_rng_vec3(false);
        let scale = get_rng_vec3(false);
        start_positions.push(vec3(
            100.0 * start_position.x,
            100.0 * start_position.y,
            100.0 * start_position.z,
        ));
        start_velocities.push(get_rng_vec3(true));
        let c = Color::new(
            (255.0 * colors.x) as u8,
            (100.0 * colors.y) as u8,
            80,
            255,
        );
        start_rotations.push(Quaternion::from_axis_angle(start_positions[i as usize].normalize(), degrees(rotations.x * 360.0)));
        start_scales.push(vec3(scale.x, scale.x, scale.x));
        part_colors.push(c);
    }

    let cube = CpuMesh::cube();

    let mut star_material = StarMaterial {
        time: 0.0,
    };
    let mut stars = StarsParticle::new(start_positions, start_rotations,  start_scales, part_colors, &context, &cube);

    let point_light = PointLight::new(
        &context,
        0.8,
        Color::WHITE,
        &vec3(0.0, 0.0, 0.0),
        Attenuation { constant: 1.0, linear: 1.0, quadratic: 1.0 },
    );

    let ambient_light = AmbientLight::new(
        &context,
        1.0,
        Color::WHITE,
    );

    window
        .render_loop(move |frame_input| {
            let elapsed_time = (frame_input.elapsed_time * 0.001) as f32;
            stars.time += elapsed_time;
            star_material.time = stars.time;

            // stars.set_transformation(Mat4::from_scale((t / 2.0).sin() / 2.0));

            // stars.set_particles(&particles);

            stars.update();

            frame_input
                .screen()
                .clear(ClearState::color(0.019, 0.003, 0.113, 1.0))
                .write(|| {
                    stars.instance.render_with_material(&star_material, &camera, &[&ambient_light])
                });

            FrameOutput::default()
        });
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

pub struct StarsParticle {
    pub acceleration: f32,
    pub instance: InstancedMesh,
    pub time: f32,
    pub instances: Instances,
}

impl StarsParticle {
    pub fn new(start_positions: Vec<Vec3>, start_rotations: Vec<Quaternion<f32>>, start_scales: Vec<Vec3>, colors: Vec<Color>, context: &Context, cpu_mesh: &CpuMesh) -> StarsParticle {
        let instances = Instances {
            translations: start_positions,
            rotations: Some(start_rotations),
            colors: Some(colors),
            scales: Some(start_scales),
            ..Default::default()
        };

        let instance = InstancedMesh::new(context, &instances, cpu_mesh);

        StarsParticle {
            acceleration: 0.1,
            time: 0.0,
            instances,
            instance,
        }
    }

    pub fn update(& mut self) {
        for i in 0..TOTAL_STARS {
            let rotations =  Some(self.instances.rotations.as_mut().unwrap()).unwrap();
            rotations[i as usize] = Quaternion::from_axis_angle(
                self.instances.translations[i as usize].normalize(), degrees(self.time * 100.0)
            );

            let scales = Some(self.instances.scales.as_mut().unwrap()).unwrap();
            scales[i as usize].x += (self.time * 2.0).cos() / 333.0;
            scales[i as usize].y += (self.time * 2.0).cos() / 333.0;
            scales[i as usize].z += (self.time * 2.0).cos() / 333.0;
        }

        let scaling = Mat4::from_scale((self.time / 5.0).cos() + 2.0);
        let rotating_y = Mat4::from_angle_y(degrees(3.0 * self.time));

        self.instance.set_instances(&self.instances);
        self.instance.set_transformation(scaling.concat(&rotating_y));
    }
}