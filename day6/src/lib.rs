use std::fs;

pub struct Packet {
    raw: String,
    packet_start: usize,
    message_start: usize,
}

impl Packet {
    pub fn find_packet_start(&mut self) -> usize {
        let start = self.find_start(4);
        self.packet_start = start;
        start
    }
    pub fn find_message_start(&mut self) -> usize {
        let start = self.find_start(14);
        self.message_start = start;
        start
    }
    pub fn find_start(&mut self, len: usize) -> usize {
        let mut start: usize = 0;
        for i in 0..self.raw.len()-len-1 { 
            let mut slice = String::from(&self.raw[i..i+len])
                            .chars()
                            .collect::<Vec<char>>();
            slice.sort_by(|a, b| a.cmp(b));
            slice.dedup();
            if slice.len() == len {
                start = i + len;
                break
            }
        }
        start        
    }   
}

pub fn receive_packet(filename: &str) -> Packet {
    let data = fs::read_to_string(filename).expect("failed to read from file");
    let raw = String::from(data.trim());
    Packet {
        raw,
        packet_start: 0,
        message_start: 0,
    }
} 
