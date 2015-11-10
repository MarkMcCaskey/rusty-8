use std::env;
use std::io::prelude::*;
use std::fs::File;

extern crate sdl2;
extern crate rand;

mod state;

fn main() {
    let mut state: state::State = Default::default(); 
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 
    {
        print!("Enter the name of a file to run");
        std::process::exit(1);
    }
    
    let arg: &String = &args[1];
    let fin = File::open(arg)
        .ok()
        .expect("File not found");

    let mut i = 0x200;
    for x in fin.bytes() {
        state.memory[i] = x.unwrap();
        //DEBUG:
        println!( "{:X} at {}", state.memory[i], i );
        i+=1;
    }

    state.load_font();

    while state.next_opcode() != 0 {
        state.run_opcode();
        //render and other IO stuff here?
        state.graphics();
    }
    
    println!("Hello, world!");
    std::process::exit(0);
}
