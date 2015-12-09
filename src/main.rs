use std::env;
use std::io::prelude::*;
use std::fs::File;

extern crate sdl2;
extern crate rand;

mod state;
mod graphics;

fn main() {
    let mut state: state::State;
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 
    {
        println!("Enter the name of a file to run");
        std::process::exit(1);
    }
    
    let arg: &String = &args[1];
    let fin = File::open(arg)
        .ok()
        .expect("File not found");

    state = Default::default(); 

    let mut i = 0x200;
    for x in fin.bytes() {
        state.memory[i] = x.unwrap();
        i+=1;
    }

    state.load_font();
    state.initialize_graphics();

    let sleep_time = 2;
    i = 0;
    while state.next_opcode() != 0 {
        state.run_opcode();
        std::thread::sleep_ms(sleep_time);
        if i == 0
        { state.advance_timer(); i += (60/sleep_time) as usize; }
        else { i-=1; }
    }
    std::process::exit(0);
}
