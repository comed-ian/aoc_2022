use std::fs::File;
use std::io::prelude::*;

pub fn get_calories() -> Vec<(u64, u64)> {
    let mut file = File::open("./data").expect("Error finding file data.txt");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading to file");
    let elves = contents.trim().split("\n\n").collect::<Vec<&str>>();
   
    let mut cal_vec: Vec<(u64, u64)> = Vec::new(); 
    for (i, e) in elves.iter().enumerate() {
        let foods = e.split("\n").collect::<Vec<&str>>();
        let mut total_cals: u64 = 0;
        for f in foods {
            total_cals += f.parse::<u64>().unwrap();
        }
        cal_vec.push((i as u64, total_cals));
    }

    // sort vector by calorie count 
    cal_vec.sort_by_key(|&(_k, v)| !v);  
    cal_vec
}

pub fn get_max_calories(v: &Vec<(u64, u64)>) -> Option<(u64, u64)> {
    if v.len() > 0 {
        return Some((v.first().unwrap().0, v.first().unwrap().1))
    }
    None
}

pub fn get_max_n_calories(v: &Vec<(u64, u64)>, n: usize) -> Option<u64> {
    if v.len() >= n {
        let mut total = 0 as u64;
        for i in 0..n {
            total += v[i].1;
        }
        return Some(total)
    }
    None
}

