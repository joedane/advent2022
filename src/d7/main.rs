use anyhow::{anyhow, Result};
use std::cell::RefCell;
use std::collections::{hash_map::Entry, HashMap};
use std::rc::{Rc, Weak};

struct File {
    name: String,
    size: u64,
}

impl File {
    fn new(name: &str, size: u64 /*, dir: Rc<RefCell<Dir>> */) -> Self {
        Self {
            name: name.to_owned(),
            size,
        }
    }

    fn dump(&self, level: u8) {
        for _ in 0..level {
            print!("    ");
        }
        println!("file `{}' with size {}", self.name, self.size);
    }
}
struct Dir {
    name: String,
    dirs: HashMap<String, Rc<RefCell<Dir>>>,
    files: Vec<File>,
    parent: Option<Weak<RefCell<Dir>>>,
}

impl Dir {
    fn root() -> Self {
        Dir {
            name: "ROOT".to_string(),
            dirs: HashMap::new(),
            files: vec![],
            parent: None,
        }
    }
    fn new(name: String, parent: Rc<RefCell<Dir>>) -> Dir {
        Dir {
            name,
            dirs: HashMap::new(),
            files: vec![],
            parent: Some(Rc::downgrade(&parent)),
        }
    }

    fn get_parent(&self) -> Option<Rc<RefCell<Dir>>> {
        self.parent.as_ref().and_then(|p| p.upgrade())
    }

    fn add_dir(&mut self, dir: Rc<RefCell<Dir>>) {
        //        println!("adding dir `{}' to `{}'", dir.borrow().name, self.name);
        let name = dir.borrow().name.clone();
        match self.dirs.entry(name) {
            Entry::Vacant(e) => {
                e.insert(dir);
            }
            Entry::Occupied(e) => {
                panic!("inserted directory {} twice", e.key())
            }
        }
    }

    fn dir_exists(&self, name: &str) -> bool {
        self.dirs.contains_key(name)
    }
    fn get_dir(&self, name: &str) -> Option<Rc<RefCell<Dir>>> {
        self.dirs.get(name).cloned()
    }
    fn add_file(&mut self, file: File) {
        self.files.push(file)
    }

    fn check_sizes(&self, results: &mut Vec<(String, u64)>, threshold: u64) -> u64 {
        let mut my_size: u64 = 0;
        for d in self.dirs.values() {
            my_size += d.borrow().check_sizes(results, threshold);
        }
        for f in &self.files {
            my_size += f.size;
        }
        if my_size <= threshold {
            results.push((self.name.to_owned(), my_size));
        }
        my_size
    }
    fn dump(&self, level: u8) {
        for _ in 0..level {
            print!("    ");
        }
        println!("directory {}", self.name);
        for (_, dir) in self.dirs.iter() {
            dir.borrow().dump(level + 1);
        }
        for f in &self.files {
            f.dump(level + 1);
        }
    }
}

fn build<'a>(lines: Vec<&'a str>) -> Result<Rc<RefCell<Dir>>> {
    let root = Rc::new(RefCell::new(Dir::root()));
    let mut current_dir = root.clone();
    let mut i = 0;

    while i < lines.len() {
        println!("LINE: {}", lines[i]);
        if lines[i].starts_with('$') {
            let cmd = &lines[i][2..];
            if cmd.starts_with("cd") {
                let arg = &lines[i][5..];
                if arg == "/" {
                    current_dir = root.clone();
                } else if arg == ".." {
                    let p = current_dir.clone();
                    if let Some(p) = p.borrow().get_parent() {
                        current_dir = p;
                    };
                } else {
                    if current_dir.borrow().dir_exists(arg) {
                        let child = current_dir.borrow().get_dir(arg).unwrap();
                        current_dir = child;
                    } else {
                        let this_ptr = current_dir.clone();
                        let new_dir =
                            Rc::new(RefCell::new(Dir::new(arg.to_owned(), this_ptr.clone())));
                        this_ptr.borrow_mut().add_dir(new_dir.clone());
                        current_dir = new_dir;
                    }
                }
                i += 1;
            } else if cmd.starts_with("ls") {
                i += 1;
                while i < lines.len() && !lines[i].contains('$') {
                    let entry = lines[i];
                    if entry.starts_with("dir") {
                        let dir = Dir::new(entry[4..].to_string(), current_dir.clone());
                        let mut cd = current_dir.borrow_mut();
                        cd.add_dir(Rc::new(RefCell::new(dir)));
                    } else {
                        let (sz, name) = entry
                            .split_once(' ')
                            .ok_or(anyhow!("bad entry: {}", entry))?;
                        let file = File::new(name, sz.parse()? /* current_dir.clone() */);
                        let mut cd = current_dir.borrow_mut();
                        cd.add_file(file);
                    }
                    i += 1;
                }
            } else {
                return Err(anyhow!("unknown command: {}", cmd));
            }
        }
    }
    Ok(root)
}
fn main() -> Result<()> {
    let lines: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|s| s.trim())
        .collect();
    let root = build(lines)?;
    root.borrow().dump(0);

    let mut dirs: Vec<(String, u64)> = vec![];
    let root_size = root.borrow().check_sizes(&mut dirs, std::u64::MAX);

    //println!("total: {}", big_dirs.iter().fold(0, |acc, e| acc + e.1));

    // part 2
    let free_space = 70_000_000 - root_size;
    let needed_space = 30_000_000 - free_space;
    if needed_space == 0 {
        println!("alreay have enough space");
    } else {
        let min = dirs
            .iter()
            .filter(|v| v.1 >= needed_space)
            .min_by(|x, y| x.1.cmp(&y.1))
            .ok_or("failed to find a minimum?");
        println!("best: {:?}", min)
    }

    Ok(())
}

#[cfg(test)]
mod test {

    #[test]
    fn test1() {
        let data = r"$ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k";

        let lines: Vec<_> = data.lines().map(|s| s.trim()).collect();
        let root = crate::build(lines).unwrap();
        //root.borrow().dump(0);

        //        assert_eq!(root.borrow().size(), 48381165);
        let mut dirs: Vec<(String, u64)> = vec![];

        root.borrow().check_sizes(&mut dirs, 100_000);
        for (name, size) in dirs.iter() {
            println!("Dir `{}' has size:\t{}", name, size);
        }
    }
}
