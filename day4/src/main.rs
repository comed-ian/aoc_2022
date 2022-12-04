use day4::*;

fn main() {
    let s = get_sections("./data");
    println!("Found {} section pairs which contain each other", get_contains(&s));
    println!("Found {} section pairs with overlap", get_overlaps(&s));
}
