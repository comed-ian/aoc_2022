use std::fs::File;
use std::io::prelude::*;
use std::fmt::Display;
use std::fmt;
use std::collections::HashSet;

#[derive(Debug, Hash, Clone)]
pub struct Item {
    pub c: char,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.c == other.c 
    }
}

impl Eq for Item {}

impl Item {
    pub fn get_priority(&self) -> u32 {
        if self.c >= 'A' && self.c <= 'Z' {
            return self.c as u32 - 'A' as u32 + 27
        }
        else if self.c >= 'a' && self.c <= 'z' {
            return self.c as u32 - 'a' as u32 + 1
        }
        panic!("Error, invalid item type");
    }
}

#[derive(Debug)]
pub struct Rucksack {
    pub all_items: String,
    pub comp1: String,
    pub comp2: String,
}

impl From<&str> for Rucksack {
    fn from(s: &str) -> Self {
        let len = s.len() / 2;
        Self {
            all_items: String::from(s),
            comp1: String::from(&s[..len]),
            comp2: String::from(&s[len..]),
        }
    }
}

impl Display for Rucksack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        write!(f, "Compartment 1: {},\nCompartment 2: {})", self.comp1, self.comp2)
    }
}

impl Rucksack {
    pub fn find_duplicates(&self) -> HashSet<Item> {
        let mut dups: HashSet<Item> = HashSet::new();
        for c in self.comp1.chars() {
            let _ = match self.comp2.find(c) {
                Some(_) => dups.insert(Item { c }),
                None => false,
            };
        }
        dups
    }
    pub fn find_badge(&self, other: &Self, other2: &Self) -> HashSet<Item> {
        let mut dups: HashSet<Item> = HashSet::new();
        for c in self.all_items.chars() {
            let _ = match other.all_items.find(c) {
                Some(_) => dups.insert(Item { c }),
                None => false,
            };
        }
        let mut badges: HashSet<Item> = HashSet::new();
        for i in dups {
            let _ = match other2.all_items.find(i.c) {
                Some(_) => badges.insert(Item { c: i.c }),
                None => false,
            };
        }
        badges
    }
}

pub fn get_rucksacks(filename: &str) -> Vec<Rucksack> {
    let mut file = File::open(filename).expect("Error finding file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading from file");
    let sacks = contents.trim().split("\n").collect::<Vec<&str>>();
    let mut rucksacks: Vec<Rucksack> = Vec::new();

    for s in sacks {
        let r: Rucksack = Rucksack::from(s);
        rucksacks.push(r);        
    }    
    
    rucksacks
}

pub struct Group {
    elves: Vec<Option<Rucksack>>,
}

impl Group {
    pub fn get_badge(&self) -> HashSet<Item> {
        for e in &self.elves { 
            match e {
                Some(_) => (),
                None => panic!("Error, group has less than 3 elves"),
            }
        }
        // find matches between elf 1 and 2
        let elf1 = self.elves[0].as_ref().unwrap();
        let elf2 = self.elves[1].as_ref().unwrap();
        let elf3 = self.elves[2].as_ref().unwrap();
        elf1.find_badge(elf2, elf3)
    }
}

pub fn get_groups(mut v: Vec<Rucksack>) -> Vec<Group> {
    let mut groups: Vec<Group> = Vec::new();
    while v.len() >= 3 {
        let mut elves: Vec<Option<Rucksack>> = Vec::new();
        for _ in 0..3 { elves.push(v.pop()); }
        groups.push(Group {
            elves,
        })
    }
    groups
}
