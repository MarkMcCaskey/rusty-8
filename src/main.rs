struct State {
    memory: [i8,4096],
    registers: [i8,16],
    stack: [i16, 16],
    delay_timer: i16,
    sound_timer: i16,
    stack_pointer: i16,
    program_counter: i16,
}

impl State {

    fn dispatch(&self, opcode: i16) {
        let b = (opcode & 0x0F00) >> 8;
        let c = (opcode & 0x00F0) >> 4;
        
        match( opcode & 0xF000 >> 12 ) {
            0 => match opcode {
                00E0 => self.clear_screen(),
                00EE => self.return_op(),
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
            12 => ,//random number stuff later
            13 => ,
            14 => ,
            15 => self.f_dispatcher( opcode )
        }
    }

    fn f_dispatcher(&self, opcode: i16) {
        let b = (opcode & 0x0F00) >> 8;

        match( opcode & 0xFF ) {
            0x07 => self.registers[b] = self.delay_timer,
            0x0A => ,
            0x15 => self.delay_timer = self.registers[b],
            0x18 => self.sound_timer = self.registers[b],
            0x1E => self.program_counter += self.registers[b],
            0x29 => ,
            0x33 => ,
            0x55 => 
        }
    }

    fn clear_screen(&self) {
    }

    fn arithmetic_dispatch(&self, opcode: i16 ) {
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


fn main() {
    let mut state: State; 
    println!("Hello, world!");
}
