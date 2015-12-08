use std::mem;
use rand::Rng;
use graphics;

extern crate sdl2;
extern crate rand;

pub struct State {
    pub memory:     [u8; 4096],
    pub registers:      [u8; 16], // change later
    stack:          [u16;16],
    delay_timer:     u8,
    sound_timer:     u8,
    stack_pointer:   u16,
    program_counter: u16,
    I:               u16,
    screen:         [[bool; 64]; 32],
    graphics:        Option<graphics::Graphics>
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
            graphics:        None
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
                _     => return, //ignore
            },
            1  => self.program_counter = bcd,
            2  => self.call_op( bcd ),
            3  => self.skip_if_eq( x, cd ),
            4  => self.skip_if_neq( x, cd ),
            5  => self.skip_if_xeqy( x, y ),
            6  => self.registers[x] = cd,
            7  => self.registers[x] =
                (self.registers[x] as u16 + cd as u16) as u8,
            8  => self.arithmetic_dispatch( opcode ),
            9  => self.skip_if_xneqy( x, y),
            10 => self.I = bcd,
            11 => self.program_counter = bcd + (self.registers[0] as u16),
            12 => self.random( opcode ),
            13 => self.display_sprite( opcode ),
            14 => self.input_disp( opcode ),
            15 => self.f_dispatcher( opcode ),
            _  => panic!("Opcode {} not recognized", opcode)
        }
    }

    fn f_dispatcher(&mut self, opcode: u16) {
        let b = (opcode & 0x0F00) >> 8;
        let x = b as usize;

        match opcode & 0xFF {
            0x07 => self.registers[x] = self.delay_timer,
            0x0A => self.wait_for_key_press(x),
            0x15 => self.delay_timer = self.registers[x],
            0x18 => self.sound_timer = self.registers[x],
            0x1E => self.I += self.registers[x] as u16,
            0x29 => self.I  = (self.registers[x] as u16)*5,
            0x33 => self.store_bcd(x),
            0x55 => self.store_registers(),
            0x65 => self.load_registers(),
            _    => panic!("Opcode {:X} not recognized", opcode)
             
        }
    }

    fn wait_for_key_press(&mut self, x: usize )
    {
        let mut found = false;
        let mut s = 0;
        if let Some(ref mut graphics) = self.graphics {
            while ! found 
            {
                //I really need to learn how to use Rust
                //properly
                if graphics.is_pressed(0)
                { s = 0; found = true; }
                else if graphics.is_pressed(1)
                { s = 1; found = true; }
                else if graphics.is_pressed(2)
                { s = 2; found = true; }
                else if graphics.is_pressed(3)
                { s = 3; found = true; }
                else if graphics.is_pressed(4)
                { s = 4; found = true; }
                else if graphics.is_pressed(5)
                { s = 5; found = true; }
                else if graphics.is_pressed(6)
                { s = 6; found = true; }
                else if graphics.is_pressed(7)
                { s = 7; found = true; }
                else if graphics.is_pressed(8)
                { s = 8; found = true; }
                else if graphics.is_pressed(9)
                { s = 9; found = true; }
                else if graphics.is_pressed(10)
                { s = 10; found = true; }
                else if graphics.is_pressed(11)
                { s = 11; found = true; }
                else if graphics.is_pressed(12)
                { s = 12; found = true; }
                else if graphics.is_pressed(13)
                { s = 13; found = true; }
                else if graphics.is_pressed(14)
                { s = 14; found = true; }
                else if graphics.is_pressed(15)
                { s = 15; found = true }

            }
        } else {
            panic!("Graphics not initialized!\n");
        }

        self.registers[x] = s;

    }

    fn random(&mut self, opcode: u16)
    {
        let x       = ((opcode >> 8) & 0xF)  as usize;
        let mask    =  (opcode       & 0xFF) as u8;
        let mut rng = rand::thread_rng();

        self.registers[x] = rng.gen::<u8>() & mask;
    }

    fn store_bcd(&mut self, x: usize)
    {
        let one   =  self.registers[x] / 100;
        let two   = (self.registers[x] % 100) / 10;
        let three =  self.registers[x] % 10;
        
        self.memory[ self.I    as usize] = one;
        self.memory[(self.I+1) as usize] = two;
        self.memory[(self.I+2) as usize] = three;
    }
    
    fn store_registers(&mut self)
    {
        for i in 0..16 {
            self.memory[(self.I + i) as usize]
                = self.registers[i as usize];
        }
    }

    fn load_registers(&mut self)
    {
        for i in 0..16 {
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

        if let Some(ref mut graphics) = self.graphics {
            graphics.clear_screen();
        } else {
            panic!("Graphics have not been initialized");
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

        self.registers[0xF] = 0;
        
        for i in start..(start + n) {
            //draw pixels

            for j in (0..8).rev() {
                match (self.memory[i] >> j) & 1 {
                    0 => { /* do nothing */ },
                    _ => {
                        if self.screen[ydraw % 32][xdraw % 64] {
                            self.registers[0xF] = 1;
                        }
                        self.screen[ydraw % 32][xdraw % 64]
                            = !self.screen[ydraw % 32][xdraw % 64];
                        
                        if let Some(ref mut graphics)
                            = self.graphics
                        {
                            graphics
                                .draw_point(xdraw%64 as usize,
                                            ydraw%32 as usize,
                                            self.screen[ydraw % 32]
                                            [xdraw % 64]);
                        } else {
                            panic!("Graphics were not initialized!\n");
                        }
                    }
                }
                if xdraw >= 63 { xdraw  = 0; }
                else           { xdraw += 1; }

            }
                
            //increment coordinates
            if ydraw >= 31 { ydraw  = 0; }
            else           { ydraw += 1; }
            xdraw = self.registers[x] as usize;
        }

        if let Some(ref mut graphics) = self.graphics {
            graphics.draw_screen();
        } else {
            panic!("Graphics not initalized!");
        }
    }
    
    pub fn run_opcode(&mut self) {
        self.increment_pc();
        let first_byte = self.memory[
            (self.program_counter-2) as usize] as u16; 
        let second_byte = self.memory[
            (self.program_counter-1) as usize] as u16;
        //DEBUG:
        println!( "registers:" );
        for j in self.registers.iter() {
            print!( "{:X} ", j );
        }
        print!("\n");
        println!( "Running opcode: {:X} at {:X}", (second_byte | (first_byte << 8)), self.program_counter - 2);
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
        let mut xl = self.registers[x] as u16;
        let     yl = self.registers[y] as u16;
        self.registers[0xF] = 0;

        if ((xl+yl)>>8) > 0 {
            self.registers[0xF] = 1;
        }
        xl += yl;
        self.registers[x] = xl as u8;
    }
    
    fn arithmetic_five(&mut self, x: usize, y: usize) {
        let mut xl = self.registers[x] as i16;
        let     yl = self.registers[y] as i16;
        self.registers[0xF] = 0;

        if self.registers[x] >/*=*/ self.registers[y] {
            self.registers[0xF] = 1;
        }
        xl -= yl;
        //simulate unsigned overflow
        if xl < 0 {
            xl+=256;
        }
        self.registers[x] = xl as u8;
    }

    //Sets VF as the least sigificant bit of Vx Then Vx is divided by 2  
    fn arithmetic_six(&mut self, x: usize) {
        self.registers[0xF] = self.registers[x] & 0x1;
        self.registers[x] >>= 1;
    }

    fn arithmetic_seven(&mut self, x: usize, y: usize) {
        let mut xl = self.registers[x] as i16;
        let     yl = self.registers[y] as i16;
        self.registers[0xF] = 0;
        if self.registers[x] < self.registers[y] {
            self.registers[0xF] = 1;
        }
        xl = yl - xl;
        println!("XL: {}, X:, {} Y: {}\n", xl, self.registers[x], self.registers[y] );
        //simulate unsigned overflow
        if xl < 0 {
            xl+=256;
        }
        
        self.registers[x] = xl as u8;
    }

    fn arithmetic_fourteen(&mut self, x: usize) {
        self.registers[0xF] = (self.registers[x] >> 7) & 1;
	self.registers[x] <<= 1; 
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
        self.stack_pointer+=1;
        self.program_counter = address;
    }

    fn return_op(&mut self) {
        self.stack_pointer-=1;
        self.program_counter =
            self.stack[self.stack_pointer as usize];
    }

    fn input_disp(&mut self, opcode: u16) {
        let x  = ((opcode >> 8) & 0xF) as usize;
        let cd =  opcode & 0xFF;

        match cd {
            0x9E => self.skip_if_pressed(x),
            0xA1 => self.skip_ifn_pressed(x),
            _    => panic!("Unrecognized opcode")
        }
    }

    fn skip_if_pressed(&mut self, x: usize )
    {
        let mut i = 0;
        if let Some(ref mut graphics) = self.graphics {
            if graphics.is_pressed(self.registers[x]) { i = 1; }
        } else { panic!("Graphics not initialized!"); }

        //Rust won't allow for multilpe borrows
        //and I don't know how else to do this
        //This is bad code, I don't know how to avoid
        //it though.
        if i == 1 
        {
            self.increment_pc();
        }
    }

    fn skip_ifn_pressed(&mut self, x: usize )
    {
        let mut i = 0;
        if let Some(ref mut graphics) = self.graphics {
            if !graphics.is_pressed(self.registers[x]) {
                i = 1; }
        } else {
            panic!("Graphics not initialized!");
        }
        if i == 1 {
            self.increment_pc();
        }
    }

    pub fn advance_timer(&mut self)
    {
        if self.sound_timer > 0
            {self.sound_timer-=1;}
        if self.delay_timer > 0
            {self.delay_timer-=1;}
    }

    pub fn initialize_graphics(&mut self)
    {
        self.graphics = Some(Default::default());
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

#[test]
fn it_works() {
}

#[test]
fn addition() {
    let mut state: State;
    state = Default::default();

    state.dispatch(0x6000); //a = 0
    state.dispatch(0x6100); //b = 0
    state.dispatch(0x7110); //b+=16
    state.dispatch(0x700A); //a+=10
    state.dispatch(0x7004); //a+=4
    state.dispatch(0x8014); //a+=b (30)
    state.dispatch(0x8104); //b+=a (46)

    assert_eq!(state.registers[1],46);
}

#[test]
fn subtraction() {
    let mut state: State;
    state = Default::default();
    
    state.dispatch(0x60F0); //a = 240
    state.dispatch(0x61FF); //b = 255
    state.dispatch(0x7001); //a+=1 (241)
    state.dispatch(0x8105); //b-=a (14)

    assert_eq!(state.registers[1],0xE);
}

#[test]
fn calling_and_returning() {
    let mut state: State;
    state = Default::default();

    state.registers[1] = 8;
}
