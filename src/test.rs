#[test]
fn it_works() {
}

#[test]
fn addition() {
    let mut state: State;

    state.dispatch(state,0x6000); //a = 0
    state.dispatch(state,0x6100); //b = 0
    state.dispatch(state,0x7110); //b+=16
    state.dispatch(state,0x700A); //a+=10
    state.dispatch(state,0x7004); //a+=4
    state.dispatch(state,0x8014); //a+=b (30)
    state.dispatch(state,0x8104); //b+=a (46)

    assert_eq!(state.registers[1],46);
}

#[test]
fn subtraction() {
    let mut state: State;
    
    state.dispatch(state,0x60F0); //a = 240
    state.dispatch(state,0x61FF); //b = 255
    state.dispatch(state,0x7001); //a+=1 (241)
    state.dispatch(state,0x8105); //b-=a (14)

    assert_eq!(state.registers[1],0xE);
}
