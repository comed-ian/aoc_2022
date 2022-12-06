use day5::*;

fn main() {
    let mut state = get_data("./data");
    state.rearrange();
    println!("Rarranged with the CrateMover 9000 to get: {}", state.get_tops());
    let mut state = get_data("./data");
    state.rearrange2();
    println!("Rarranged with the CrateMover 9001 to get: {}", state.get_tops());
} 
