use day3::*;

fn main() {
    let rucksacks = get_rucksacks("./data");

    // get priorities for all duplicates
    let mut total_priorities = 0u32; 
    for r in &rucksacks {
        for i in r.find_duplicates() {
            total_priorities += i.get_priority();
        }
    }
    println!("Found duplicates with a total {} priority value", total_priorities); 

    // divide into groups and find badges 
    let groups = get_groups(rucksacks);
    let mut total_priorities = 0u32;
    for g in groups {
        let b = g.get_badge();
        match b.len() {
            1 => {
                for d in b { total_priorities += d.get_priority() }
            },
            _ => panic!("Error, no badge found for group"),
        }
    }
    println!("Found badges with a total {} priority value", total_priorities); 
}
