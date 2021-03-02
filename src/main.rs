use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread::spawn;
use std::sync::mpsc::*;

struct Pixel {
    x: u32,
    y: u32,
    color: sdl2::pixels::Color
}

enum Command{
    Quit,
    Noop,
    Resolution(u32, u32)
}

enum DrawOp{
    Noop,
    Draw(Vec<Pixel>)
}

fn main() {
    // Main doing dispatching only, all works happens in threads.
    let (main_command_sender, main_command_queue): (Sender<Command>, Receiver<Command>) = channel();
    let main_command_sender_for_view = main_command_sender.clone();
    
    let (draw_sender, draw_reciever): (Sender<DrawOp>, Receiver<DrawOp>) = channel();
    let (model_sender, model_reciever): (Sender<Command>, Receiver<Command>) = channel();
    
    let model = spawn(move ||{model(main_command_sender, model_reciever)});
    let view = spawn(move ||{view(main_command_sender_for_view, draw_reciever)});
    
    for cmd in main_command_queue.iter() {
        match cmd {
            Command::Quit => {
                println!("Quit command");
                break
            },
            Command::Noop => {},
            Command::Resolution(x, y) =>{
                println!("Resolution: {}x{}", x, y);
                model_sender.send(cmd);
            }
        }
    }
}

fn model(main_cmd: Sender<Command>, model_cmd: Receiver<Command>){
    for cmd in model_cmd.iter(){
        match cmd {
            Command::Resolution(x, y) =>{
                println!("Model resolution: {}x{}", x, y);
            }
            _ => {},
        }
    }
    
}
fn view(main_cmd: Sender<Command>, draw_cmd: Receiver<DrawOp>) {
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
    main_cmd.send(Command::Resolution(X, Y)).unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(sdl2::pixels::PixelFormatEnum::ABGR8888, X as u32, Y/cpus as u32)
        .unwrap();
    texture.with_lock(None, |array, _| {for b in array.iter_mut(){*b = 0;}}).unwrap();
    let mut start = std::time::Instant::now();
    let  mut last_printed:u64 = 0;
    loop {
        frames +=1;
        match draw_cmd.try_recv(){
            Ok(DrawOp::Draw(pixels)) =>{
                texture.with_lock(
                    None,
                    |bytearray, pitch_size|{
                        for pixel in  pixels.iter(){
                            bytearray[pixel.x as usize * pixel_bytes + pixel.y as usize*pitch_size] = pixel.color.r;
                            bytearray[pixel.x as usize * pixel_bytes + pixel.y as usize*pitch_size + 1] = pixel.color.g;
                            bytearray[pixel.x as usize * pixel_bytes + pixel.y as usize*pitch_size + 1] = pixel.color.b;
                        }
                    }
                ).unwrap();
            }
            _ => {}
        }    
        
        // texture1.update(
        //     None,
        //     &pixels,
        //     X as usize,
        // ).unwrap();

        canvas.copy(&texture, None, None).unwrap();
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    main_cmd.send(Command::Quit);
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