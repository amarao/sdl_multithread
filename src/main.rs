// extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

// const X:usize = 2560;
// const Y:usize = 1440;



pub fn main() {
    // let cpus = num_cpus::get();
    let texture_format = sdl2::pixels::PixelFormatEnum::ABGR8888;
    let pixel_bytes = texture_format.byte_size_per_pixel();
    let cpus = 1;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 multithread demo",0, 0)
        .fullscreen_desktop()
        .borderless()
        .build()
        .unwrap();
    let mut canvas = window
        .into_canvas()
        // .present_vsync()
        .accelerated()
        .build()
        .unwrap();
    sdl_context.mouse().show_cursor(false);
    let mut frames:u64 = 0;
    let (X, Y) = canvas.output_size().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture1 = texture_creator
        .create_texture_streaming(sdl2::pixels::PixelFormatEnum::ABGR8888, X as u32, Y/cpus as u32)
        .unwrap();
    let mut pixels:Vec<u8> = Vec::new();
    pixels.resize((X*Y*4) as usize, 0);
    let mut start = std::time::Instant::now();
    let  mut last_printed:u64 = 0;
    let mut factor:u32 = 0xFEFABABE + frames as u32;
    'running: loop {
        frames +=1;
        texture1.with_lock(
            None,
            |pixels, _pitch_size|{
                if frames % 120 == 0 {
                    for y in 0..Y as usize{
                        for x in 0..X as usize{
                            let base = {
        
                                if frames & 0x100 == 0{
                                    frames  as u8
                                }
                                else{
                                    0 - frames as u8
                                }
                            };
                            pixels[(x+y*X as usize) *pixel_bytes] = base;
                            pixels[(x+y*X as usize) *pixel_bytes+1] = base;
                            pixels[(x+y*X as usize) *pixel_bytes+2] = base;
                            pixels[(x+y*X as usize) *pixel_bytes+3] = 255;
                        }
                    }
                }
            }
        ).unwrap();
            
        
        // texture1.update(
        //     None,
        //     &pixels,
        //     X as usize,
        // ).unwrap();

        canvas.copy(&texture1, None, None).unwrap();
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.present();
        if start.elapsed() > std::time::Duration::new(1,0){
            let dt = start.elapsed().as_secs_f64();
            let fc = frames - last_printed;
            last_printed = frames;
            start = std::time::Instant::now();
            println!("{:.1}", fc as f64/dt);
        }

    }
}