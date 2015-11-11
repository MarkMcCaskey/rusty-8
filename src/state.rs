use std::mem;
use rand::Rng;
use graphics;

extern crate sdl2;
extern crate rand;

pub struct State {
    pub memory:     [u8; 4096],
    registers:      [u8; 16],
    stack:          [u16;16],
    delay_timer:     u8,
    sound_timer:     u8,
    stack_pointer:   u16,
    program_counter: u16,
    I:               u16,
    screen:         [[bool; 64]; 32],
    graphics:        graphics::Graphics
}

impl Default for State {
    #[inline]
    fn default() -> State {
        State {
            memory:          unsafe { mem::zeroed() },
            registers:       unsafe { mem::zeroed() },
            stack:           unsafe { mem::zeroed() },
            delay_timer:     0,
            sound_timer:     0,
            stack_pointer:   0,
            program_counter: 0x200,
            I:               0,
            screen:          unsafe { mem::zeroed() },
            graphics:        Default::default()
        }
    }
}

impl State {
    fn dispatch(&mut self, opcode: u16) {
        let b = ((opcode & 0x0F00) >> 8) as u8;
        let c = ((opcode & 0x00F0) >> 4) as u8; 
        let x = b                        as usize;
        let y = c                        as usize;
        let bcd = (opcode & 0xFFF)       as u16;
        let cd  = (opcode & 0xFF)        as u8;
        
        match (opcode & 0xF000) >> 12  {
            0 => match opcode {
                0x00E0 => self.clear_screen(),
                0x00EE => self.return_op(),
                _ => panic!("Invalid opcode {}", opcode),
            },
            1  => self.program_counter = bcd,
            2  => self.call_op( bcd ),
            3  => self.skip_if_eq( x, cd ),
            4  => self.skip_if_neq( x, cd ),
            5  => self.skip_if_xeqy( x, y ),
            6  => self.registers[x]  =  cd,
            7  => self.registers[x] +=  cd,
            8  => self.arithmetic_dispatch( opcode ),
            9  => self.skip_if_xneqy( x, y),
            10 => self.I = bcd,
            11 => self.program_counter = bcd + (self.registers[0] as u16),
            12 => self.random( opcode ),
            13 => self.display_sprite( opcode ),
            14 => panic!("Opcode 0xEXXX not implemented!"),
            15 => self.f_dispatcher( opcode ),
            _  => panic!("Opcode {} not recognized", opcode)
        }
    }

    fn f_dispatcher(&mut self, opcode: u16) {
        let b = (opcode & 0x0F00) >> 8;
        let x = b as usize;

        match opcode & 0xFF {
            0x07 => self.registers[x] = self.delay_timer,
            0x0A => panic!("Opcode FXXA not implemented!"),
            0x15 => self.delay_timer = self.registers[x],
            0x18 => self.sound_timer = self.registers[x],
            0x1E => self.I += self.registers[x] as u16,
            0x29 => self.I = (self.registers[x] as u16)*4,
            0x33 => self.store_bcd(x),
            0x55 => self.store_registers(),
            0x65 => self.load_registers(),
            _    => panic!("Opcode {} not recognized", opcode)
             
        }
    }

    fn random(&mut self, opcode: u16)
    {
        let x    = ((opcode >> 8) & 0xF) as usize;
        let mask = (opcode & 0xFF) as u8;
        let mut rng = rand::thread_rng();

        self.registers[x] = rng.gen::<u8>() & mask;
    }

    fn store_bcd(&mut self, x: usize)
    {
        let one = self.registers[x] / 100;
        let two = (self.registers[x] % 100) / 10;
        let three = self.registers[x] % 10;
        
        self.memory[self.I as usize] = one;
        self.memory[(self.I+1) as usize] = two;
        self.memory[(self.I+2) as usize] = three;
    }
    
    fn store_registers(&mut self)
    {
        for i in 0..15 {
            self.memory[(self.I + i) as usize]
                = self.registers[i as usize];
        }
    }

    fn load_registers(&mut self)
    {
        for i in 0..15 {
            self.registers[i as usize] =
                self.memory[(self.I + i) as usize];
        }
    }

    fn clear_screen(&mut self) {
        for i in self.screen.iter_mut()  {
            for j in i.iter_mut() {
                *j = false;
            }
        }
    }

    fn display_sprite(&mut self, opcode: u16)
    {
        let x     = ((opcode >> 8) & 0xF) as usize;
        let y     = ((opcode >> 4) & 0xF) as usize;
        let n     = ( opcode       & 0xF) as usize;
        let start = self.I                as usize;

        let mut xdraw = self.registers[x] as usize;
        let mut ydraw = self.registers[y] as usize;

        for i in start..(start + n) {
            //draw pixel
            match self.memory[i] {
                0 => {/* do nothing*/},
                _ => {
                    if self.screen[x][y] {
                        self.registers[0xF] = 1;
                    }
                    self.screen[x][y] = !self.screen[x][y]
                }
            }

            //increment coordinates
            if xdraw >= 63 { xdraw  = 0; }
            else           { xdraw += 1; }
            if ydraw >= 31 { ydraw  = 0; }
            else           { ydraw += 1; }
        }
    }
    
    pub fn run_opcode(&mut self) {
        self.increment_pc();
        let first_byte = self.memory[
            (self.program_counter-2) as usize] as u16; 
        let second_byte = self.memory[
            (self.program_counter-1) as usize] as u16;
        //DEBUG:
        println!( "Running opcode: {:X}", (second_byte | (first_byte << 8)));
        self.dispatch( second_byte | (first_byte << 8) );
    }

    pub fn next_opcode(&self) -> u16 {
        let fb = self.memory[(self.program_counter+1)
                             as usize] as u16;
        let sb = self.memory[self.program_counter
                              as usize] as u16;
        fb | (sb << 8)
    }

    fn arithmetic_dispatch(&mut self, opcode: u16 ) {
        let b = (opcode & 0xF00) >> 8; //0x0x00
        let c = (opcode & 0xF0)  >> 4;  //0x00y0 
        let x = b as usize;
        let y = c as usize;
        match opcode & 0xF {
            0x0 => self.registers[x]  = self.registers[y], 
            0x1 => self.registers[x] |= self.registers[y],
            0x2 => self.registers[x] &= self.registers[y],
            0x3 => self.registers[x] ^= self.registers[y],
            0x4 => self.arithmetic_four(x, y),
            0x5 => self.arithmetic_five(x, y),
	    0x6 => self.arithmetic_six(x),
	    0x7 => self.arithmetic_seven(x, y),
            0xE => self.arithmetic_fourteen(x),
            _   => panic!("Opcode {} not recognized", opcode),
        }
    }

    fn arithmetic_four(&mut self, x: usize, y: usize) {
	// Stores Vy + Vx into Vx and sets VF = carry  
        let xl = self.registers[x] as u16;
        let yl = self.registers[y] as u16;
        self.registers[x] += self.registers[y];
        if (xl+yl>>8) > 1 {
            self.registers[0xF]=1
        }
    }
    
    fn arithmetic_five(&mut self, x: usize, y: usize) {
        if self.registers[x] > self.registers[y] {
            self.registers[0xF]=1;
        }
        self.registers[x] -= self.registers[y];
    }

    //Sets VF as the least sigificant bit of Vx Then Vx is divided by 2  
    fn arithmetic_six(&mut self, x: usize) {
        self.registers[0xF] = self.registers[x] & 0x1;
        self.registers[x] >> 1;
    }

    fn arithmetic_seven(&mut self, x: usize, y: usize) {
        if self.registers[x] < self.registers[y] {
            self.registers[0xF] = 1;
        }
        self.registers[x] = self.registers[y]
            - self.registers[x];
    }

    fn arithmetic_fourteen(&mut self, x: usize) {
        self.registers[0xF] = (self.registers[x] >> 7) & 1;
	self.registers[x] << 1; 
    }

    fn skip_if_xeqy( &mut self, x: usize, y: usize ) {
        if self.registers[x] == self.registers[y] {
            self.increment_pc();
        }
    }

    fn skip_if_xneqy( &mut self, x: usize, y: usize ) {
        if self.registers[x] != self.registers[y] {
            self.increment_pc();
        }
    }

    fn skip_if_eq( &mut self, x: usize, n: u8 ) {
        if self.registers[x] == n {
            self.increment_pc();
        }
    }

    fn skip_if_neq( &mut self, x: usize, n: u8 ) {
        if self.registers[x] != n {
            self.increment_pc();
        }
    }

    fn increment_pc(&mut self) {
        self.program_counter += 2;
    }

    fn call_op( &mut self, address: u16 ) {
        self.stack[self.stack_pointer as usize] =
            self.program_counter;

        self.program_counter = address;
    }

    fn return_op(&mut self) {
        self.program_counter =
            self.stack[self.stack_pointer as usize];
        self.stack_pointer-=1;
    }

    pub fn load_font(&mut self)
    {
        let zero  = [0xF0, 0x90, 0x90, 0x90, 0xF0];
        let one   = [0x20, 0x60, 0x20, 0x20, 0x70];
        let two   = [0xF0, 0x10, 0xF0, 0x80, 0xF0];
        let three = [0xF0, 0x10, 0xF0, 0x10, 0xF0];
        let four  = [0x90, 0x90, 0xF0, 0x10, 0x10];
        let five  = [0xF0, 0x80, 0xF0, 0x10, 0xF0];
        let six   = [0xF0, 0x80, 0xF0, 0x90, 0xF0];
        let seven = [0xF0, 0x10, 0x20, 0x40, 0x40];
        let eight = [0xF0, 0x90, 0xF0, 0x90, 0xF0];
        let nine  = [0xF0, 0x90, 0xF0, 0x10, 0xF0];
        let a     = [0xF0, 0x90, 0xF0, 0x90, 0x90];
        let b     = [0xE0, 0x90, 0xE0, 0x90, 0xE0];
        let c     = [0xF0, 0x80, 0x80, 0x80, 0xF0];
        let d     = [0xE0, 0x90, 0x90, 0x90, 0xE0];
        let e     = [0xF0, 0x80, 0xF0, 0x80, 0xF0];
        let f     = [0xF0, 0x80, 0xF0, 0x80, 0x80];

        let font_list = [zero, one, two, three, four,
                         five, six, seven, eight, nine,
                         a, b, c, d, e, f];

        let mut i = 0;
        for &x in font_list.iter() {
            for &n in x.iter() {
                self.memory[i] = n;
                i += 1;
            }
        }
    }
    
}
