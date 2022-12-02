use day1::*;

fn main() {
    // get sorted vec of (elfnum, total calories) in descending order
    let cal_map = get_calories();
    
    // get max calories carried by a single elf
    let (idx, max) = match get_max_calories(&cal_map) {
        Some((i, m)) => (i, m),
        None => panic!("No max calorie carrying elf found!"),
    };
    println!("Elf #{} is carrying the most, {}, calories", idx, max);
    
    // get max calories carried by first three elves
    let n: usize = 3;
    let total = match get_max_n_calories(&cal_map, n) {
        Some(c) => c,
        None => panic!("There aren't {n} elves in the party!!"),
    };
    println!("The first {n} elves are carrying {total} calories");
}
