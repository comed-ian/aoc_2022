use std::fs;

#[derive(Debug)]
pub enum Command {
    Cd { dest: String  }, 
    Ls { res: Vec<String> },
}

impl From<String> for Command {
    fn from(s: String) -> Self {
        match &s[..2] {
            "ls" => Command::Ls { 
                res: s.split("\n")
                        .map(|x| x.to_string())
                        .filter(|y| y != "" && y != "ls")
                        .collect::<Vec<String>>() 
            },
            "cd" => Command::Cd { 
                dest: s.split(" ")
                        .last()
                        .unwrap()
                        .trim()
                        .to_string()
            },
            _ => panic!("Error matching command"),
        }
    }   
}

#[derive(Debug)]
pub enum ElfFile {
    Directory {
        name: String,
        size: usize,
        children: Vec<ElfFile>,
    },
    File {
        name: String,
        size: usize,
    }
}

impl From<String> for ElfFile {
    fn from(s: String) -> Self {
        match s.split(" ").next().unwrap() {
            "dir" => ElfFile::Directory {
                size: 0,
                name: s.split(" ").last().unwrap().to_owned(),
                children: Vec::new(),
            },
            size => ElfFile::File {
                name: s.split(" ").last().unwrap().to_owned(),
                size: size.parse::<usize>().unwrap(),
            }
        }
    }
}

impl ElfFile {
    pub fn name(&self) -> &String {
        match self {
            ElfFile::File { name, .. } => &name,
            ElfFile::Directory { name, .. } => &name,
        } 
    }
    pub fn add(&mut self, f: ElfFile) {
        match self {
            ElfFile::Directory { children, .. } => children.push(f),
            _ => panic!("Error, cannot add child to file"),
        } 
    }
    pub fn get_node(&mut self, path: &[impl AsRef<str>]) -> Option<&mut Self> {
        if path.len() == 0 { return Some(self) }

        let children = match self {
            ElfFile::Directory { children, .. } => children,
            ElfFile::File { .. } => panic!("Error, cannot cd within file"),
        };

        let to = &path[0].as_ref();
        for c in children {
            if c.name() == to { return c.get_node(&path[1..]) } 
        }

        None
       
        // ElfFile::Directory { children, name, .. } => {
        //     if path.len() == 0 { return Some(self) }
        //     let to = &path[0].as_ref();
        //     for c in children {
        //         if c.name() == to { return c.get_node(&path[1..]) } 
        //     }
        //     panic!("No child {} found for node {}", to, name);
        // },
    }
    pub fn dir_size(&self) -> usize {
        match self {
            ElfFile::File { .. } => 0,
            ElfFile::Directory { size, .. } =>  *size
        }
    }
    pub fn get_size(&mut self) -> usize {
        match self {
            ElfFile::File { size, .. } => *size,
            ElfFile::Directory { children, size, .. } => { 
                *size = children.into_iter().map(Self::get_size).sum();
                *size
            }
        }
    }
    pub fn find_smallest_dir_larger_than(&self, limit: usize, current: usize) 
        -> usize {
        match self {
            ElfFile::File { .. } => 0xffffffff,
            ElfFile::Directory { size, children, .. } => {
                if *size < limit { return current }
                let smallest = children
                    .iter()
                    .map(|x| x.find_smallest_dir_larger_than(limit, current))
                    .filter(|&x| x!=0 && x >= limit)
                    .min().unwrap();
                if smallest == current { *size }
                else { smallest }
            }
        } 
    }
    pub fn sum_dir_smaller_than(&self, limit: usize) -> usize {
        match self {
            ElfFile::File { .. } => 0,
            ElfFile::Directory { size, children, .. } => {
                let mut total = 0usize; 
                if *size <= limit { total += *size }
                total += children
                    .iter()
                    .map(|x| x.sum_dir_smaller_than(limit))
                    .sum::<usize>();
                total
            }
        }
    } 
}

#[derive(Debug)]
pub struct FileSystem {
    root: ElfFile,
    curr_path: Vec<String>,
}

impl FileSystem {
    pub fn handle(&mut self, c: Command) {
        match c {
            Command::Cd { dest } => {
                match &dest[..] {
                    "/" => self.curr_path.clear(),
                    ".." => drop(self.curr_path.pop()),
                    dir => self.curr_path.push(dir.to_owned()), 
                }
            },
            Command::Ls { res } => {
                let curr_node = self.root.get_node(&self.curr_path).unwrap();
                for r in res {
                    curr_node.add(ElfFile::from(r));
                }
            }
        }
    }
    pub fn get_size(&mut self) -> usize {
        self.root.get_size()
    }
    pub fn sum_less_than(&self, size: usize) -> usize {
        self.root.sum_dir_smaller_than(size)
    }
    pub fn find_dir_to_delete(&self, size: usize) -> usize {
        self.root.find_smallest_dir_larger_than(size, 0xffffffff)
    }
}

pub fn injest_data(filename: &str) -> FileSystem {
    let root = ElfFile::Directory {
        name: "/".to_owned(),
        size: 0,
        children: Vec::new(),
    };

    let mut fs = FileSystem {
        root,
        curr_path: Vec::new(),
    };

    let data = fs::read_to_string(filename).expect("failed to read from file");
    for i in data.trim().split("$ ") {
        if i == "" { continue } ;
        fs.handle(Command::from(i.to_owned()));
    }  
    fs  
}
