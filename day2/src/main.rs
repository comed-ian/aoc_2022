use day2::*;

fn main() {
    let points = play_rps();
    println!("Played RPS and recieved {points} points");
    let points = play_rps2();
    println!("Played RPS v2 and recieved {points} points");
}
