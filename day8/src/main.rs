use day8::*;

fn main() {
    let g = get_grid("./data");
    let v = is_visible(&g);
    println!("{}", count_visible(&v));
}
