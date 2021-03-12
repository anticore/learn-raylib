#![no_std]

#[macro_use]
extern crate dotenv_codegen;

use raylib::prelude::*;

fn main() {
    let is_prod: bool = dotenv!("PROD") == "true";
    let res_x: i32 = dotenv!("RES_X").parse().unwrap();
    let res_y: i32 = dotenv!("RES_Y").parse().unwrap();

    let resolution: [f32; 2] = [res_x as f32, res_y as f32];

    let (mut rl, thread) = 
        if is_prod {
            raylib::init() 
                .size(res_x, res_y)
                .fullscreen()
                .vsync()
                .title("release")
                .build()
        } else {
            raylib::init()
                .size(res_x, res_y)
                .undecorated()
                .title("debug")
                .build()
        };
    
    rl.set_target_fps(60);

    let shader: &str = include_str!("shaders/main.frag");
    let mut shader: Shader = rl.load_shader_code(&thread, None, Some(shader));

    let resolution_loc = shader.get_shader_location("resolution");
    let time_loc = shader.get_shader_location("time");

    let mut lastframe = rl.load_render_texture(&thread, res_x as u32, res_y  as u32).unwrap();
    let lastframe_loc = shader.get_shader_location("lastframe");

    shader.set_shader_value(resolution_loc, resolution);

    let mut run_time = 0.0;

    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();
        run_time += delta_time;

        shader.set_shader_value(time_loc, run_time);
        
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        {
            let mut d = d.begin_texture_mode(&thread, &mut lastframe);
            { 
               let mut d = d.begin_shader_mode(&shader);
                d.draw_rectangle(0, 0, res_x, res_y, Color::BLACK);
            }
        }

        d.draw_texture_rec(
            lastframe.texture(),
            Rectangle::new(
                0.0,
                0.0,
                lastframe.texture.width as f32,
                -lastframe.texture.height as f32,
            ),
            Vector2::new(
                res_x as f32 - lastframe.texture.width as f32,
                0.0,
            ),
            Color::WHITE,
        );


        shader.set_shader_value_texture(lastframe_loc, lastframe.texture());
    }
}