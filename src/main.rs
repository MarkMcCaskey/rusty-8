use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use std::collections::HashSet;

struct State {
    memory:         [i8, 4096],
    registers:      [i8, 16],
    stack:          [i16,16],
    delay_timer:     i16,
    sound_timer:     i16,
    stack_pointer:   i16,
    program_counter: i16,
}

impl State {

    fn dispatch(&self, opcode: i16) {
        let b = (opcode & 0x0F00) >> 8;
        let c = (opcode & 0x00F0) >> 4;
        
        match( opcode & 0xF000 >> 12 ) {
            0 => match opcode {
                0x00E0 => self.clear_screen(),
                0x00EE => self.return_op(),
                _ => panic!("Invalid opcode {}", opcode),
            },
            1  => self.program_counter = (opcode & 0x0FFF),
            2  => self.call_op( opcode & 0x0FFF ),
            3  => self.skip_if_eq(   b, (opcode & 0xFF) ),
            4  => self.skip_if_neq(  b, (opcode & 0xFF) ),
            5  => self.skip_if_xeqy( b, (opcode & 0xF0) ),
            6  => self.registers[b]  =  (opcode & 0xFF),
            7  => self.registers[b] +=  (opcode & 0xFF),
            8  => self.arithmetic_dispatch( opcode ),
            9  => self.skip_if_xneqy(b, c),
            10 => self.program_counter = (opcode & 0xFFF),
            11 => self.program_counter = (opcode & 0xFFF) + self.registers[0],
            12 => panic!("Opcode 0xCXXX not implemented!"),//random number stuff later
            13 => panic!("Opcode 0xDXXX not implemented!"),
            14 => panic!("Opcode 0xEXXX not implemented!"),
            15 => self.f_dispatcher( opcode ),
            _  => panic!("Opcode {} not recognized", opcode)
        }
    }

    fn f_dispatcher(&self, opcode: i16) {
        let b = (opcode & 0x0F00) >> 8;

        match( opcode & 0xFF ) {
            0x07 => self.registers[b] = self.delay_timer,
            0x0A => panic!("Opcode FXXA not implemented!"),
            0x15 => self.delay_timer = self.registers[b],
            0x18 => self.sound_timer = self.registers[b],
            0x1E => self.program_counter += self.registers[b],
            0x29 => panic!("Opcode FX29 not implemented!"),
            0x33 => panic!("Opcode FX33 not implemented!"),
            0x55 => panic!("Opcode FX55 not implemented!"),
            _    => panic!("Opcode {} not recognized", opcode)
             
        }
    }

    fn clear_screen(&self) {
        panic!("Clear screen not implemented!");
    }

    fn run_opcode(&self) {
        self.increment_pc();
        self.dispatch( self.memory[self.program_counter-2] );
    }

    fn arithmetic_dispatch(&self, opcode: i16 ) {
        let x = (opcode & 0xF00) >> 8; //0x0x00
        let y = (opcode & 0xF0)  >> 4;  //0x00y0 
        match (opcode & 0xF) {
            0x0 => self.registers[x]  = self.registers[y], // Stores Vy into Vx
            0x1 => self.registers[x] |= self.registers[y],// Stores Bitwise OR of Vy and Vx into Vx
            0x2 => self.registers[x] &= self.registers[y],// Stores Bitwise AND of Vy and Vx into Vx
            0x3 => self.registers[x] ^= self.registers[y],// Stores Bitwise XOR of Vy and Vx into Vx
            0x4 => self.registers[x] += self.registers[y]; if((x+y>>8)>1){self.registers[0xF]=1},
		// Stores Vy + Vx into Vx and sets VF = carry      
  	    0x5 => if(self.registers[x]>self.registers[y]){self.registers[0xF]=1} self.registers[x] -= self.registers[y] ,
		// If Vx>Vy then VF is 1, Stores Vy - Vx into Vx ans sets VF = NOT carry
	    0x6 => self.registers[f] = self.registers[x] & 0x1; self.registers[x] >> 1,
		 //Sets VF as the least sigificant bit of Vx Then Vx is divided by 2  
	    0x7 => if(self.registers[x]<self.registers[y]){self.registers[0xF]=1} self.registers[x] -= self.registers[y] ,
		// If Vx>Vy then VF is 1, Stores Vy - Vx into Vx ans sets VF = NOT carry   
            0xE =>  self.registers[0xF] =(self.registers[x] >> 3)//Sets the most signigicant bit of Vx to VF
		 self.registers[x] << 1, // Then Vx is multipled by 2
            _  => panic!("Opcode {} not recognized", opcode),

        }
    }

    fn skip_if_xeqy( &self, x: i8, y: i8 ) {
        if (self.registers[x] == self.registers[y])
            self.increment_pc();
    }

    fn skip_if_xneqy( &self, x: i8, y: i8 ) {
        if (self.registers[x] != self.registers[y])
            self.increment_pc();
    }

    fn skip_if_eq( &self, x: i8, n: i8 ) {
        if (self.registers[x] == n)
            self.increment_pc();
    }

    fn skip_if_neq( &self, x: i8, n: i8 ) {
        if (self.registers[x] != n)
            self.increment_pc();
    }

    fn increment_pc(&self) {
        self.program_counter += 2;
    }

    fn call_op( &self, address: i16 ) {
        self.stack[stack_pointer] = self.program_counter;
        self.program_counter = address;
    }

    fn return_op(&self) {
        self.program_counter = self.stack[self.stack_pointer];
        self.stack_pointer--;
    }
}

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


fn main() {
    let mut state: State; 
    let arg_vals = env::args();
    if( arg_vals.length() < 2 )
    {
        panic!("Enter the name of a file to run");
    }

    let mut f = try!(File::open(arg_vals[1]));
    f.read(&mut state.memory[512]); // not sure about this
    state.program_counter=512;
    //start loop here
    state.run_opcode();
    //render and other IO stuff here?
    
    println!("Hello, world!");
}
