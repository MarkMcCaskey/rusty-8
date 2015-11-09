use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
/*use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;*/
use std::collections::HashSet;
use std::ops;
use std::path::Path;
extern crate byteorder;
use byteorder::{BigEndian,ReadBytesExt};

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
    let mut state: state::State = Default::default(); 
    let args: Vec<_> = env::args().collect();

    if( args.len() < 2 )
    {
        panic!("Enter the name of a file to run");
    }
    
    let arg: &String = &args[1];
//    let path = Path::new(arg);
    let mut fin =File::open(arg).unwrap();
//        .ok()
//        .expect("Unable to open file");

    let in_vals = fin.read_i16::<BigEndian>().unwrap();
//    state.initialize([0; 4096]);

    //start loop here
    state.run_opcode();
    //render and other IO stuff here?
    
    println!("Hello, world!");
}
