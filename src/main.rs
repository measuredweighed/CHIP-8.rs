extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::env;
use std::path::Path;
use std::time::{Duration, Instant};

mod hardware;
mod cpu;
mod resources;

use hardware::{Display, Keyboard};
use cpu::{CPU};

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("Please provide a filename"); }

    let sdl_context = sdl2::init().unwrap();
    let mut events = sdl_context.event_pump().unwrap();
    
    let mut display = Display::new(&sdl_context, 20);
    let mut keyboard = Keyboard::new();
    let mut cpu = CPU::new();

    // Load our game
    let path = Path::new(&args[1]);
    cpu.load(path);

    let mut last_cycle_ts = Instant::now();
    let mut last_timer_update_ts = Instant::now();
    let mut last_display_ts = Instant::now();

    'main: loop {

        for event in events.poll_iter() {
            match event {
                Event::Quit {..} => break 'main,

                Event::KeyDown {keycode: Some(keycode), ..} => {
                    if keycode == Keycode::Escape {
                        break 'main
                    } else {
                        keyboard.set_key_status(keycode, true);
                    }
                },
                Event::KeyUp {keycode: Some(keycode), ..} => {
                    keyboard.set_key_status(keycode, false);
                },
                _ => {}
            }
        }

        // TODO: clean this up, possibly have CPU instantiate and manage
        // an instance of the Keyboard struct, rather than keeping it separated
        cpu.key_state = keyboard.keys;

        // CPU ticks at a rate of 500HZ
        if Instant::now() - last_cycle_ts > Duration::from_millis(1000/500) {
            cpu.cycle();
            last_cycle_ts = Instant::now();
        }

        // Timers tick at a rate of 60HZ
        if Instant::now() - last_timer_update_ts > Duration::from_millis(1000/60) {
            cpu.update_timers();
            last_timer_update_ts = Instant::now();
        }

        if Instant::now() - last_display_ts > Duration::from_millis(10) {
            display.draw(cpu.screen);
            last_display_ts = Instant::now();
        }
    }

}