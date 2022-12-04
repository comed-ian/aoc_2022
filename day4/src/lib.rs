use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Section {
    start: u32,
    end: u32,
}

type SectionPair = (Section, Section);

fn get_section_pair(s: &str) -> SectionPair {
    let sections = s.split(",").collect::<Vec<&str>>();
    assert_eq!(sections.len(), 2);
    let mut v: Vec<Section> = Vec::new();

    for s in sections {
        v.push(
            Section::from(
                s.split("-")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u32>>()
            )
        )
    }
    assert_eq!(v.len(), 2);
    (v[0], v[1])
}

impl From<Vec<u32>> for Section {
    fn from(v: Vec<u32>) -> Self {
        assert_eq!(v.len(), 2);
        Self {
            start: v[0],
            end: v[1],
        }
    }
}

impl Section {
    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end 
    }
    fn overlap(&self, other: &Self) -> bool {
        (self.start >= other.start && self.start <= other.end) ||
        (other.start >= self.start && other.start <= self.end)
    }
}

pub fn get_sections(filename: &str) -> Vec<(Section, Section)> {
    let mut file = File::open(filename).expect("Error finding file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading from file");
    let line = contents.trim().split("\n").collect::<Vec<&str>>();
    let mut sections: Vec<SectionPair> = Vec::new();
    for l in line {
       sections.push(get_section_pair(l));
    }
    sections
}

pub fn get_contains(v: &Vec<SectionPair>) -> u32 {
    let mut cnt = 0u32;
    for sp in v {
        if sp.0.contains(&sp.1) { cnt += 1; }
        else if sp.1.contains(&sp.0) { cnt += 1; }
    }   
    cnt
}

pub fn get_overlaps(v: &Vec<SectionPair>) -> u32 {
    let mut cnt = 0u32;
    for sp in v {
        if sp.0.overlap(&sp.1) { cnt += 1; }
    }   
    cnt
}
