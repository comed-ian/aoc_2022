use std::fs;

#[derive(Debug)]
pub struct Point {
    x: u32,
    y: u32,
}

impl Point {
    pub fn compare(&mut self, other: &Self) {
        if self.x > other.x && self.x - other.x >= 2 { 
            self.x = other.x + 1;
            self.y = other.y;
        }
        else if self.x < other.x && other.x - self.x >= 2 { 
            self.x = other.x - 1; 
            self.y = other.y; 
        }
        else if self.y > other.y && self.y - other.y >= 2 { 
            self.y = other.y + 1; 
            self.x = other.x;
        }
        else if self.y < other.y && other.y - self.y >= 2 { 
            self.y = other.y - 1; 
            self.x = other.x;
        }
    }
}

pub struct Op {
    dir: String,
    len: i32,
}

pub fn run(b: &mut Vec<Vec<bool>>, ops: &Vec<Op>, head: &mut Point, 
    tail: &mut Point) -> usize {
    for op in ops {
        for i in 0..op.len as usize {
            match op.dir {
                _ if op.dir =="L" => head.x -= 1, 
                _ if op.dir =="R" => head.x += 1,
                _ if op.dir =="U" => head.y += 1, 
                _ if op.dir =="D" => head.y -= 1,
                _ => panic!("Invalid move direction"),
            }
            
            tail.compare(head);
            println!("{:?} {:?}", head, tail);
            b[tail.x as usize][tail.y as usize] = true;
        }
    }

    b.iter().flatten().filter(|b| **b).count()
}

pub fn chase_tail(filename: &str) -> usize {
    let v = read_operations(filename);
    let (left, right, up, down) = get_board_size(&v);
    let width: usize = (right - left + 1).try_into().unwrap();
    let height: usize = (up - down + 1).try_into().unwrap();
    println!("{} {}", width, height);
    let mut b: Vec<Vec<bool>> = vec![vec![false; height]; width];

    let mut head = Point {
        x: (0 - left).try_into().unwrap(),
        y: down.abs() as u32,
    };
    let mut tail = Point {
        x: (0 - left).try_into().unwrap(),
        y: down.abs() as u32,
    };
    
    b[tail.x as usize][tail.y as usize] = true;
    run(&mut b, &v, &mut head, &mut tail)
}  


pub fn read_operations(filename: &str) -> Vec<Op> {
    let mut v: Vec<Op> = Vec::new();
    let data = fs::read_to_string(filename).expect("unable to read from file");
    for mut l in data.lines().map(|x| x.split(" ")) {
        let dir = l.next().unwrap().to_owned();
        let len = l.next().unwrap().parse::<i32>().unwrap();
        v.push( Op {dir, len })
     }
     v
}

pub fn get_board_size(ops: &Vec<Op>) -> (i32, i32, i32, i32) {
    let mut left: i32 = 0;
    let mut right: i32 = 0;
    let mut up: i32 = 0;
    let mut down: i32 = 0;
    let mut width: i32 = 0;
    let mut height: i32 = 0;

    for op in ops {
        match op.dir {
            _ if op.dir =="L" => width -= op.len,
            _ if op.dir =="R" => width += op.len,
            _ if op.dir =="U" => height += op.len,
            _ if op.dir =="D" => height -= op.len,
            _ => panic!("Invalid move direction"),
        }
        if width < left { left = width; }
        if width > right { right = width; }
        if height > up { up = height; }
        if height < down { down = height };
    }
    println!("{} {} {} {}", left, right, up, down);
    (left, right, up, down)
}
