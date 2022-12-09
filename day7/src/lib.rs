use std::fs;

#[derive(Debug)]
pub enum FileType {
    Directory {
        files: Vec<usize>,
    },
    File,
}

impl FileType {
    pub fn add_child(&mut self, n: usize) {
        match self {
            FileType::Directory { files } => files.push(n),
            _ => ()
        }
    }
}

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
pub struct FileSystem {
    files: Vec<ElfFile>,
    current_dir: Option<usize>,
}

impl FileSystem {
    fn add(&mut self, name: String, size: usize) -> usize {
        let n: usize = self.files.len();
        let parent: Option<usize> = self.current_dir;
        let filetype = match size {
            0 => FileType::Directory { files: Vec::new() },
            _ => FileType::File,
        };
        // add file to main FileSystem list
        self.files.push(
            ElfFile {
                idx: n,
                name,
                filetype,
                size,
                parent
            }
        );
        // add file index to current directory's children 
        match self.current_dir {
            Some(idx) => {
                self.files[idx].filetype.add_child(n);
            }
            _ => ()
        }
        // propagate size updates 
        // let mut cd = self.current_dir;
        // loop {
        //     match cd { 
        //         Some(idx) => {
        //             self.files[idx].size += size;
        //             cd = self.files[idx].parent;
        //         },
        //         None => break,
        //     }
        // }
        n
    }

    fn handle_cmd(&mut self, cmd: Command) {
        match cmd {
            Command::Cd { dest } => {
                match self.files.iter().filter(|x| x.name == dest).next() {
                    None => {
                        // if changing to directory that doesn't exist yet
                        // (e.g., '/'), add it 
                        if dest != ".." { 
                            let n = self.add(dest, 0); 
                            self.current_dir = Some(n);
                        }
                        else {
                        // move up a directory
                            match self.current_dir {
                                Some(n) => { 
                                    match self.files[n].parent { 
                                        Some(p) => self.current_dir = Some(p),
                                        None => (), // do nothing
                                    }
                                },
                                None => panic!("moving up from non-existent dir"), 
                            }
                        } 
                    },
                    // directory match, change current dir
                    Some(file) => self.current_dir = Some(file.idx),
                }
                println!("changing current dir to: {}", self.files[self.current_dir.unwrap()].name); 
            },
            Command::Ls { res } => {
                match self.current_dir  {
                    Some(idx) => {
                        for i in res.iter() {
                            _ = match i.split(" ").next().unwrap() {
                                "dir" => self.add(i.split(" ").last().unwrap().to_owned(), 0),
                                size => self.add(i.split(" ").last().unwrap().to_owned(), size.parse::<usize>().unwrap()),
                            };
                        }
                    },
                    None => panic!("ls printed for no current dir"),
                }       
            }
        }
    }

    pub fn filter_less_than(&self, size: usize) -> usize {
        let mut total: usize = 0;
        for f in self.files 
            .iter()
            .filter(|y| y.is_dir())
            .filter(|x| x.size < size) {
                println!("Directory {} has size {}", f.name, f.size);
                total += f.size
        }
        total
    }

    pub fn update_sizes(&mut self) {
        // propagate file sizes to parents
        for mut f in self.files
            .iter()
            .filter(|y| !y.is_dir()) {
                match f.parent {
                    Some(idx) => self.files[idx].size += f.size,
                    None => panic!("abandoned child!"),
                }
        }
        // propagate directory sizes upward 
        // for f in self.files
        //    .iter   
    }
}

#[derive(Debug)]
pub struct ElfFile {
    idx: usize,
    name: String,
    size: usize,
    filetype: FileType,
    parent: Option<usize>,
}

impl ElfFile {
    pub fn is_dir(&self) -> bool {
        match self.filetype {
            FileType::Directory { files: _ } => true,
            _ => false,
        }
    }
}

pub fn enumerate(filename: &str) -> FileSystem {
    let mut filesystem = FileSystem {
        files: Vec::new(),
        current_dir: None,
    };

    let data = fs::read_to_string(filename).expect("failed to read from file");
    for mut i in data.trim().split("$ ") {
        if i == "" { continue } ;
        filesystem.handle_cmd(Command::from(i.to_owned()));
    }    
    filesystem
}
