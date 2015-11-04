use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
/*use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;*/
use std::collections::HashSet;
use std::ops;

mod state;


/* IO code needs a lot of review, commenting it out for now
fn pressed_scancode_set(e: &sdl2::EventPump) -> HashSet<Scancode> {
B
    e.keyboard_state().pressed_scancodes().collect()
}

fn pressed_keycode_set(e: &sdl2::EventPump) -> HashSet <Keycode> {
    e.keyboard_state().pressed_scancodes()
        .filter_map(Keycode::from_scancode())
        .collect()
}

fn newly_pressed(old: &HashSet<Scancode>, new: &HashSet<Scancode>)
                 -> HashSet<Scancode> {
    new - old
}

fn graphics_loop() {
    let surface = sdl2::Surface::new(64,32, sdl2::Pixels::PixelFormatEnum::Index1LSB);
    let render_context = sdl2::render::from_surface(surface);
    
}
*/

fn main() {
    let mut state: state::State; 

    let arg_vals = env::args();
    if( arg_vals.length() < 2 )
    {
        panic!("Enter the name of a file to run");
    }

    let mut fin = try!(File::open(arg_vals[1]));
    fin.read(&mut state.memory[512]); // not sure about this
    state.program_counter=512;
    //start loop here
    state.run_opcode();
    //render and other IO stuff here?
    
    println!("Hello, world!");
}
