use day6::*;

fn main() {
    let mut packet = receive_packet("./data");
    println!("Found start of packet at index {}", packet.find_packet_start());
    println!("Found start of message at index {}", packet.find_message_start());
}
