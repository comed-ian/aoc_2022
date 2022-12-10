use day7::*;

fn main() {
    const TOTAL_SPACE: usize = 70000000;
    const NEEDED_SPACE: usize = 30000000;
    let mut fs = injest_data("./data");
    let alloc = fs.get_size();
    println!("Directories with less than {} bytes have total of {} bytes", 
          100000, fs.sum_less_than(100000));
    let clear_size = alloc - (TOTAL_SPACE - NEEDED_SPACE);
    println!("Total size {}, need to clear size {}", alloc, clear_size);
    println!("Smallest directory with greater than {} bytes has {} bytes", 
           clear_size, fs.find_dir_to_delete(clear_size)); 
}
