use std::env;
use chip8_core::*;
use std::fs::File;
use std::io::Read;

use sdl2::pixels::Color;
use sdl2::rect::{self, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const TICKS_PER_FRAME: usize = 10;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run /path/to/game");
        return;
    }

    // SDL Setup 
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    //create screen according to size and position in center of monitor
    let window = video_subsystem
        .window("Chip-8 Emulator", (DISPLAY_WIDTH * SCALE).try_into().unwrap(), (DISPLAY_HEIGHT * SCALE).try_into().unwrap())
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    
    let mut chip8 = Chip8::new();
    let mut memory = File::open(&args[1]).expect("Unable to open file");
    let mut buffer = Vec::new();

    memory.read_to_end(&mut buffer).unwrap();
    chip8.load(&buffer);


    let mut event_pump = sdl_context.event_pump().unwrap();
    //setup game loop
    'gameloop: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit{..} => {
                    break 'gameloop;
                },
                _ => ()
            }
        }

        chip8.cycle();
        draw_screen(&chip8, &mut canvas);
    }


}


fn draw_screen(chip8: &Chip8, canvas: &mut Canvas<Window>) {
    // Clear canvas to black
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buffer = chip8.get_display();
    canvas.set_draw_color(Color::RGB(255,255,255));   // Srtting to white

    // let rect = Rect::new(150, 150, 10, 10);
    // canvas.fill_rect(rect).unwrap();

    for (i, pixel) in screen_buffer.iter().enumerate() {
        if *pixel {
            let x = (i % DISPLAY_WIDTH) as u32;
            let y = (i / DISPLAY_HEIGHT) as u32;
            let rect = Rect::new((x as usize * SCALE) as i32, (y as usize * SCALE) as i32, SCALE.try_into().unwrap(), SCALE.try_into().unwrap());
            canvas.fill_rect(rect).unwrap();


        }
    }

    canvas.present();

}

fn key2btn(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 =>    Some(0x1),
        Keycode::Num2 =>    Some(0x2),
        Keycode::Num3 =>    Some(0x3),
        Keycode::Num4 =>    Some(0xC),
        Keycode::Q =>       Some(0x4),
        Keycode::W =>       Some(0x5),
        Keycode::E =>       Some(0x6),
        Keycode::R =>       Some(0xD),
        Keycode::A =>       Some(0x7),
        Keycode::S =>       Some(0x8),
        Keycode::D =>       Some(0x9),
        Keycode::F =>       Some(0xE),
        Keycode::Z =>       Some(0xA),
        Keycode::X =>       Some(0x0),
        Keycode::C =>       Some(0xB),
        Keycode::V =>       Some(0xF),
        _ =>                None,
    }
}