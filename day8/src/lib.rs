use std::fs;

#[derive(Copy, Clone, Debug)]
pub struct Control {
    top: u8,
    bottom: u8,
    left: u8,
    right: u8,
}   
        
type Grid = Vec<Vec<u8>>;
type GridU32 = Vec<Vec<u32>>;
type ControlGrid = Vec<Vec<Control>>;


pub fn get_scenic_score(g: &Grid) -> GridU32 {
    let size = g.len();
    let mut c: GridU32 = vec![vec![0; size]; size];
    for i in 0..size-1 {
        for j in 0..size-1 {
            if i == 0 || i == size - 1 || j == 0 || j == size - 1 {
                c[i][j] = 0;
                continue;
            }
            let val = g[i][j];
            let mut score = 1u32;
            let mut count = 0u32;
            let mut idx: usize = j - 1;
            // check left
            loop {
                count += 1;
                if g[i][idx] >= val { break; }
                if idx == 0 { break; }
                idx -= 1;
            }
            score *= count;
            // check up 
            count = 0;
            idx = i - 1;
            loop {
                count += 1;
                if idx == 0 { break; } 
                if g[idx][j] >= val { break; }
                idx -= 1
            }
            score *= count;
            // check right 
            count = 0;
            idx = j + 1; 
            loop {
                count += 1;
                if idx == size - 1 { break; }
                if g[i][idx] >= val { break; }
                idx += 1;
            }
            score *= count;
            //check down 
            count = 0; 
            idx = i + 1;
            loop {
                count += 1;
                if idx == size - 1 { break; }
                if g[idx][j] >= val { break; }
                idx += 1;
            }
            score *= count; 
            c[i][j] = score;
        }
    }
    c
}

pub fn get_max_scenic_score(c: &GridU32) -> u32 {
    *c.iter().flatten().max_by(|a, b| a.cmp(b)).unwrap()
}

pub fn check_down(v: &mut Grid, g: &Grid, c: &mut ControlGrid, i_start: usize, i_end: usize, col: usize) {
    for idx in i_start..i_end+1 {
        // check prevailing left value and update 
        if c[idx-1][col].top < g[idx][col] {
            c[idx][col].top = g[idx][col];
            v[idx][col] = 1; 
        } else {
            if v[idx][col] != 1 { v[idx][col] = 0 };
            c[idx][col].top = c[idx-1][col].top;
        }
    }     
}

pub fn check_up(v: &mut Grid, g: &Grid, c: &mut ControlGrid, i_start: usize, i_end: usize, col: usize) {
    for idx in (i_start..i_end+1).rev() {
        // check prevailing left value and update 
        if c[idx+1][col].bottom < g[idx][col] {
            c[idx][col].bottom = g[idx][col];
            v[idx][col] = 1; 
        } else {
            if v[idx][col] != 1 { v[idx][col] = 0 };
            c[idx][col].bottom = c[idx+1][col].bottom;
        }
    }     
}

pub fn check_left(v: &mut Grid, g: &Grid, c: &mut ControlGrid, i_start: usize, i_end: usize, row: usize) {
    for idx in (i_start..i_end+1).rev() {
        // check prevailing left value and update 
        if c[row][idx+1].right < g[row][idx] {
            c[row][idx].right = g[row][idx];
            v[row][idx] = 1; 
        } else {
            if v[row][idx] != 1 { v[row][idx] = 0 };
            c[row][idx].right = c[row][idx+1].right;
        }
    }     
}

pub fn check_right(v: &mut Grid, g: &Grid, c: &mut ControlGrid, i_start: usize, i_end: usize, row: usize) {
    for idx in i_start..i_end+1 {
        // check prevailing left value and update 
        if c[row][idx-1].left < g[row][idx] {
            c[row][idx].left = g[row][idx];
            v[row][idx] = 1; 
        } else {
            if v[row][idx] != 1 { v[row][idx] = 0 };
            c[row][idx].left = c[row][idx-1].left;
        }
    }     
}

pub fn is_visible(g: &Grid) -> Grid {
    let size = g.len();
    let mut visible: Grid = vec![vec![0xffu8; size]; size];

    let mut control: ControlGrid = vec![vec![Control {
        top: 0xff,
        left: 0xff,
        right: 0xff,
        bottom: 0xff,
    }; size]; size];
    
    // init edges 
    for i in 0..size {
        visible[0][i] = 1;
        visible[i][0] = 1;
        visible[i][size-1] = 1;
        visible[size-1][i] = 1;
        // init control, only matters for one direction (looking inward)
        control[0][i].top = g[0][i];
        control[i][0].left = g[i][0];
        control[i][size-1].right = g[i][size-1];
        control[size-1][i].bottom = g[size-1][i]
    }

    // print_control_grid(&control);

    let mut i_start: usize = 1;
    let mut i_end: usize = size - 2;
    while i_start <= i_end {
        for row in i_start..i_end + 1 { 
            check_right(&mut visible, &g, &mut control, i_start, i_end, row);
            check_left(&mut visible, &g, &mut control, i_start, i_end, row);
            check_down(&mut visible, &g, &mut control, i_start, i_end, row);
            check_up(&mut visible, &g, &mut control, i_start, i_end, row);
            // print_control_grid(&control);
        } 
        i_start += 1;
        i_end -= 1;
    }
    // print_grid(&g);
    // println!();
    // print_grid(&visible);
    visible
}

pub fn count_visible(g: &Grid) -> u32 {
    g.iter().flatten().fold(0u32, |sum, &val| sum + val as u32)
}

pub fn print_control_grid(g: &ControlGrid) {
    for i in g {
        for j in i {
            print!("{:?} ", j);
        }
        println!();
    }
}

pub fn print_grid(g: &Grid) {
    for i in g {
        for j in i {
            if *j == 255 { print!("X "); }
            else { print!("{} ", j); }
        }
        println!();
    }
}

pub fn print_grid_u32(g: &GridU32) {
    for i in g {
        for j in i {
            print!("{} ", j);
        }
        println!();
    }
}

pub fn get_grid(path: &str) -> Grid {
    let data = fs::read_to_string(path).expect("could not read file");
    let g: Grid = data.lines()
            .map(
                |x| x.chars()
                    .map(|y| y.to_digit(10)
                                .unwrap() as u8
                    ).collect::<Vec<u8>>()
            ).collect::<Vec<Vec<u8>>>();
    g
}
