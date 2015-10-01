let mut memory: [i8,4096];
let mut registers: [i8,16];
let mut stack: [i16, 16];
let mut delay_timer: i16;
let mut sound_timer: i16;
let mut stack_pointer: i16;
let mut program_counter: i16;


fn dispatch(opcode: i16) {
    match( opcode & 0xF000 ) {
        0 => match opcode {
            00E0 => clear_screen(),
            00EE => return_op(),
            _ => panic!("Invalid opcode {}", opcode),
        },
        1 => program_counter = (opcode & 0x0FFF),
        2 => call_op( opcode & 0x0FFF ),
        3 => skip_if_eq((opcode & 0xF00),(opcode & 0xFF)),
        4 => skip_if_neq((opcode & 0xF00),(opcode & 0xFF)),
        5 => skip_if_xeqy((opcode & 0xF00), (opcode & 0xF0)),
        6 => registers[opcode & 0xF00] = (opcode & 0xFF),
        7 => registers[opcode & 0xF00] += (opcode & 0xFF),
        8 => arithmetic_dispatch(opcode),
        9 => skip_if_xneqy((opcode & 0xF00), (opcode & 0xF0)),
        10 => program_counter = (opcode & 0xFFF),
        11 => program_counter = (opcode & 0xFFF) + registers[0];
        12 => ,//random number stuff later
        13 => ,
        
    }
}

fn clear_screen() {
}

fn arithmetic_dispatch(opcode: i16 ) {
}

fn skip_if_xeqy( x: i8, y: i8 ) {
    if (registers[x] == registers[y])
        increment_pc();
}

fn skip_if_xneqy( x: i8, y: i8 ) {
    if (registers[x] != registers[y])
        increment_pc();
}

fn skip_if_eq( x: i8, n: i8 ) {
    if (registers[x] == n)
        increment_pc();
}

fn skip_if_neq( x: i8, n: i8 ) {
    if (registers[x] != n)
        increment_pc();
}

fn increment_pc() {
    program_counter += 2;
}

fn call_op( address: i16 ) {
    stack[stack_pointer] = program_counter;
    program_counter = address;
}

fn return_op() {
    program_counter = stack[stack_pointer];
    stack_pointer--;
}
  

fn main() {
    println!("Hello, world!");
}
