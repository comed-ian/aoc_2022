use day7::*;

fn main() {
    let fs = enumerate("./data2");
    println!("{:#?}", fs);
    println!("Directories with less than {} bytes have total of {} bytes", 
         100000, fs.filter_less_than(100000));
}
