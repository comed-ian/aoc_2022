use std::fs;

#[derive(Clone, Debug, Copy)]
pub struct Instruction {
    pub from: usize,
    pub to: usize,
    pub qty: usize,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let line = s.split(" ").collect::<Vec<&str>>();
        assert_eq!(line.len(), 6); 
        // subtract 1 from indices to account for columns starting at 1
        Self {
            qty: line[1].parse().unwrap(),
            from: line[3].parse::<usize>().unwrap() - 1,
            to: line[5].parse::<usize>().unwrap() - 1,
        }
    }
}

#[derive(Debug)]
pub struct ShipState {
    pub stacks: Vec<Stack>,
    pub instructions: Vec<Instruction>,
    pub num_stacks: usize,
}

impl ShipState {
    pub fn rearrange(&mut self) {
        for i in &self.instructions {
            for _ in 0..i.qty {
                let s = self.stacks[i.from].pop().unwrap();
                self.stacks[i.to].push(s);
            }
        }   
    }
    pub fn rearrange2(&mut self) {
        for i in &self.instructions {
            let top = self.stacks[i.from].len();
            let mut new_stack: Stack = self.stacks[i.from]
                .drain(top - i.qty..top).collect();
            self.stacks[i.to].append(&mut new_stack);
        }   
    }
 
    pub fn get_tops(&self) -> String {
        self.stacks.iter().map(|x| x.iter().last().unwrap()).collect::<String>()
    }
}
pub type Stack = Vec::<char>;

pub fn get_data(filename: &str) -> ShipState { 
    let data = fs::read_to_string(filename).expect("failed to read from file");
    let mut split = data.split("\n\n").collect::<Vec<&str>>();
    assert_eq!(split.len(), 2);

    // read instructions
    let mut instructions: Vec<Instruction> = Vec::new();
    let ins = split.pop().unwrap().trim();
    for i in ins.split("\n") {
         instructions.push(Instruction::from(i));
    }

    // read current ship state 
    let state = split.pop().unwrap();
    let (crates, last) = state.rsplit_once('\n').unwrap();
    let num_stacks: usize = last.trim().rsplit_once(' ').unwrap().1.parse().unwrap();
    let mut stacks: Vec<Stack> = vec![Vec::new(); num_stacks];
    // iterate through rows from bottom up 
    for l in crates.split("\n").collect::<Vec<&str>>().into_iter().rev() {
        for i in 0..num_stacks as usize {
            let slice = &l[i*4..(i*4+3)];   
            if slice != "   " {
                stacks[i].push(slice.chars().nth(1).unwrap());
            } 
        }
    }
    ShipState {
        instructions,
        stacks,
        num_stacks,
    }
}
