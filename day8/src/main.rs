use day8::*;

fn main() {
    let g = get_grid("./data");
    let v = is_visible(&g);
    println!("The number of visible trees from the exterior is {}", 
        count_visible(&v));
    let s = get_scenic_score(&g);
    println!("The max scenic score for all trees is {}", 
        get_max_scenic_score(&s));
}
