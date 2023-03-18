use sdl2;
use nalgebra as ng;

mod triangle;
mod rasterizer;

use rasterizer::Rasterizer;

fn get_view_matrix(eye: &ng::Vector3<f64>) -> ng::Matrix4<f64> {
    let translate = ng::Matrix4::new(
        1.0, 0.0, 0.0, -eye.x,
        0.0, 1.0, 0.0, -eye.y,
        0.0, 0.0, 1.0, -eye.z,
        0.0, 0.0, 0.0, 1.0
    );
    translate * ng::Matrix4::identity()
}


fn get_model_matrix(rotation_angle: f64) -> ng::Matrix4<f64> {
    let (sin, cos) = (rotation_angle * std::f64::consts::PI / 180.0).sin_cos();
    ng::Matrix4::new(
        cos, sin, 0.0, 0.0,
        -sin, cos, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )        
}

fn get_projection_matrix(eye_fov: f64, aspect_radio: f64, z_near: f64, z_far: f64) -> ng::Matrix4<f64> {
    let PI = std::f64::consts::PI;

    let m1 = ng::Matrix4::new(
        z_near, 0.0, 0.0, 0.0,
        0.0, z_near, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, -1.0, 0.0
    );

    let delta_x = z_near * (eye_fov * PI / 2.0 / 180.0).tan();
    let delta_y = delta_x * aspect_radio;
    let delta_z = (z_near - z_far) / 2.0;
    let center_z = (z_near + delta_z) / 2.0;

    let m2 = ng::Matrix4::new(
        1.0 / delta_x, 0.0, 0.0, 0.0,
        0.0, 1.0 / delta_y, 0.0, 0.0,
        0.0, 0.0, 1.0 / delta_z, 0.0,
        0.0, 0.0, center_z / delta_z, 1.0
    ).transpose();

    m2 * m1
}



fn main() {
    let sdl2_context = sdl2::init().unwrap();
    let video = sdl2_context.video().unwrap();
    let (width, height) = (700, 700);
    let window = video
        .window("Arcade Shooter", width, height)
        .position_centered()
        .vulkan()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().accelerated().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGB24, width, height)
        .unwrap();

    let mut rst = Rasterizer::new(width, height);

    let mut eye_pos = ng::Vector3::new(0.0, 0.0, 5.0);
    let mut angle = 0;

    let vert = vec![
        nalgebra::Vector3::new(2.0, 0.0, -2.0),
        nalgebra::Vector3::new(0.0, 2.0, -2.0),
        nalgebra::Vector3::new(-2.0, 0.0, -2.0),
    ];
    let ind = vec![nalgebra::Vector3::new(0, 1, 2)];
    let vert_id = rst.load_vertices(&vert);
    let ind_id = rst.load_indices(&ind);
    
    'running: loop {
        rst.clear(rasterizer::Buffers::Color | rasterizer::Buffers::Depth);
        rst.set_model(get_model_matrix(angle as f64));
        rst.set_view(get_view_matrix(&eye_pos));
        rst.set_projection(get_projection_matrix(45.0, 1.0, 0.1, 50.0));
        rst.draw(&vert_id, &ind_id, rasterizer::Primitive::Triangle);


        texture.update(None, rst.as_raw_data(), (width * 3) as usize).unwrap();
        canvas.copy(&texture, None, None).unwrap();

        for event in sdl2_context.event_pump().unwrap().poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                sdl2::event::Event::KeyDown { timestamp, window_id, keycode, scancode, keymod, repeat } => {
                    match keycode {
                        Some(sdl2::keyboard::Keycode::Escape) => break 'running,
                        Some(sdl2::keyboard::Keycode::A) => angle = angle + 10,
                        Some(sdl2::keyboard::Keycode::D) => angle = angle - 10,
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        canvas.present();
    }
}