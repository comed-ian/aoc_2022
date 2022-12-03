use std::fs::File;
use std::io::prelude::*;
use std::cmp::Ordering;
   
#[derive(Debug, Clone)] 
pub enum Choice {
    Rock,
    Paper,
    Scissors
}

impl From<&str> for Choice {
    fn from(c: &str) -> Self {
        match c {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissors,
             _ => panic!("Error trying to parse choice {}", c),
        }
    }
}

impl PartialEq for Choice {
    fn eq(&self, other: &Self) -> bool {
        self.get_val() == other.get_val()
    }
}

impl Eq for Choice {}

impl Ord for Choice {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Rock, Self::Scissors) => Ordering::Greater,
            (Self::Scissors, Self::Rock) => Ordering::Less,
            _ => self.get_val().cmp(&other.get_val())
        }
    }
}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    } 
}

impl Choice {
    fn get_val(&self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }           
    }
    fn get_win(&self) -> Choice {
        match self {
            Self::Rock => Choice::Paper,
            Self::Scissors => Choice::Rock,
            Self::Paper => Choice::Scissors,
        }
    }
    fn get_loss(&self) -> Choice {
        match self {
            Self::Rock => Choice::Scissors,
            Self::Scissors => Choice::Paper,
            Self::Paper => Choice::Rock,
        }
    }
}

pub fn calc_outcome(t: Choice, y: Choice) -> u64 {
    let mut pts: u64 = 0;
   
    // get points for choice and outcome
    pts += y.get_val();
    match y.cmp(&t) {
        Ordering::Greater => pts += 6,
        Ordering::Equal => pts += 3,
        _ => (),
    }
    pts
}

pub fn get_choice(t: Choice, outcome: &str) -> Choice {
    match outcome {
        "X" => t.get_loss(),
        "Y" => t.clone(),
        "Z" => t.get_win(),
        _ => panic!("Error parsing outcome {outcome}"),
    }
}

pub fn play_rps() -> u64 {
    let mut file = File::open("./data").expect("Error finding file data.txt");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading to file");
    let rounds = contents.trim().split("\n").collect::<Vec<&str>>();
    let mut points: u64 = 0;

    for r in rounds {
        let actions  = r.split(" ").collect::<Vec<&str>>();
        let t = Choice::from(actions[0]);
        let y = Choice::from(actions[1]);
 
        points += calc_outcome(t, y);        
    }

    points
}

pub fn play_rps2() -> u64 {
    let mut file = File::open("./data").expect("Error finding file data.txt");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading to file");
    let rounds = contents.trim().split("\n").collect::<Vec<&str>>();
    let mut points: u64 = 0;

    for r in rounds {
        let actions  = r.split(" ").collect::<Vec<&str>>();
        let t = Choice::from(actions[0]);
        let y = get_choice(t.clone(), actions[1]); 
        points += calc_outcome(t, y);        
    }

    points
}
