struct State {
    memory:         [i8; 4096],
    registers:      [i8; 16],
    stack:          [i16;16],
    delay_timer:     i8,
    sound_timer:     i8,
    stack_pointer:   i16,
    program_counter: i16,
}

impl State {

    fn dispatch(&self, opcode: i16) {
        let b = ((opcode & 0x0F00) >> 8) as i8;
        let c = ((opcode & 0x00F0) >> 4) as i8; 
        let x = b as usize;
        let y = c as usize;
        let bcd = (opcode & 0xFFF) as i16;
        let cd  = (opcode & 0xFF) as i8;
        
        match( opcode & 0xF000 >> 12 ) {
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
            10 => self.program_counter = bcd,
            11 => self.program_counter = bcd + (self.registers[0] as i16),
            12 => panic!("Opcode 0xCXXX not implemented!"),//random number stuff later
            13 => panic!("Opcode 0xDXXX not implemented!"),
            14 => panic!("Opcode 0xEXXX not implemented!"),
            15 => self.f_dispatcher( opcode ),
            _  => panic!("Opcode {} not recognized", opcode)
        }
    }

    fn f_dispatcher(&self, opcode: i16) {
        let b = (opcode & 0x0F00) >> 8;
        let x = b as usize;

        match( opcode & 0xFF ) {
            0x07 => self.registers[x] = self.delay_timer,
            0x0A => panic!("Opcode FXXA not implemented!"),
            0x15 => self.delay_timer = self.registers[x],
            0x18 => self.sound_timer = self.registers[x],
            0x1E => self.program_counter += (self.registers[x] as i16),
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
        let first_byte = self.memory[
            (self.program_counter-2) as usize] as i16; 
        let second_byte = self.memory[
            (self.program_counter-1) as usize] as i16;
        self.dispatch( second_byte | (first_byte << 8) );
    }

    fn arithmetic_dispatch(&self, opcode: i16 ) {
        let b = (opcode & 0xF00) >> 8; //0x0x00
        let c = (opcode & 0xF0)  >> 4;  //0x00y0 
        let x = b as usize;
        let y = c as usize;
        match (opcode & 0xF) {
            0x0 => self.registers[x]  = self.registers[y], 
            0x1 => self.registers[x] |= self.registers[y],
            0x2 => self.registers[x] &= self.registers[y],
            0x3 => self.registers[x] ^= self.registers[y],
            0x4 => self.arithmetic_four(x, y),
            0x5 => self.arithmetic_five(x, y),
	    0x6 => self.arithmetic_six(x, y),
	    0x7 => self.arithmetic_seven(x, y),
            0xE => self.arithmetic_fourteen(x, y),               _   => panic!("Opcode {} not recognized", opcode),
        }
    }

    fn arithmetic_four(&self, x: usize, y: usize) {
	// Stores Vy + Vx into Vx and sets VF = carry      
        let xl = self.registers[x] as i16;
        let yl = self.registers[y] as i16;
        self.registers[x] += self.registers[y];
        if((xl+yl>>8)>1){
            self.registers[0xF]=1
        }
    }

    
    fn arithmetic_five(&self, x: usize, y: usize) {
        if(self.registers[x] > self.registers[y]) {
            self.registers[0xF]=1;
        }
        self.registers[x] -= self.registers[y];
    }


    //Sets VF as the least sigificant bit of Vx Then Vx is divided by 2  
    fn arithmetic_six(&self, x: usize, y: usize) {
        self.registers[0xF] = self.registers[x] & 0x1;
        self.registers[x] >> 1;
    }


    fn arithmetic_seven(&self, x: usize, y: usize) {
        if(self.registers[x] < self.registers[y]) {
            self.registers[0xF] = 1;
        }
        self.registers[x] = self.registers[y]
            - self.registers[x];
    }

    fn arithmetic_fourteen(&self, x: usize, y: usize) {
        self.registers[0xF] = (self.registers[x] >> 7) & 1;
	self.registers[x] << 1; 
    }

    fn skip_if_xeqy( &self, x: usize, y: usize ) {
        if (self.registers[x] == self.registers[y]) {
            self.increment_pc();
        }
    }

    fn skip_if_xneqy( &self, x: usize, y: usize ) {
        if (self.registers[x] != self.registers[y]) {
            self.increment_pc();
        }
    }

    fn skip_if_eq( &self, x: usize, n: i8 ) {
        if (self.registers[x] == n) {
            self.increment_pc();
        }
    }

    fn skip_if_neq( &self, x: usize, n: i8 ) {
        if (self.registers[x] != n) {
            self.increment_pc();
        }
    }

    fn increment_pc(&self) {
        self.program_counter += 2;
    }

    fn call_op( &self, address: i16 ) {
        self.stack[self.stack_pointer as usize] =
            self.program_counter;

        self.program_counter = address;
    }

    fn return_op(&self) {
        self.program_counter =
            self.stack[self.stack_pointer as usize];
        self.stack_pointer-=1;
    }
}

