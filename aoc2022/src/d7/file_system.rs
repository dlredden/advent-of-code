use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Dir {
    pub name: String,
    pub parent: Option<HashMap<String, Dir>>,
    dirs: HashMap<String, Dir>,
    files: HashMap<String, i32>,
    size: i32,
}

impl Dir {
    pub fn new(name: String, parent: Option<Dir>) -> Dir {
        if parent.is_none() {
            return Dir {
                name,
                parent: None,
                dirs: HashMap::new(),
                files: HashMap::new(),
                size: 0,
            };
        }

        let d = Dir {
            name,
            parent: Some(HashMap::new()),
            dirs: HashMap::new(),
            files: HashMap::new(),
            size: 0,
        };

        d.dirs.insert(d.name.to_string(), parent.unwrap());
        d
    }

    pub fn change_dir(&mut self, name: String) -> &mut Dir {
        let to_dir = name;
        if to_dir == ".." {
            if self.parent.is_none() {
                return self;
            }
            return &mut self.parent.unwrap().values().next().unwrap();
        }

        fn find_dir<'a>(dirs: &'a mut HashMap<String, Dir>, name: &String) -> Option<&'a mut Dir> {
            if dirs.is_empty() {
                return None;
            }

            for (n, d) in dirs {
                if n == name {
                    return Some(d);
                } else {
                    find_dir(&mut d.dirs, name);
                }
            }
            None
        }

        let x = find_dir(&mut self.dirs, &to_dir);
        if x.is_some() {
            return x.unwrap();
        }
        self
    }

    pub fn add_dir(&mut self, name: String) {
        let dir = Dir::new(name.to_string(), Some(self));
        self.dirs.insert(name.to_string(), dir);
    }

    pub fn add_file(&mut self, name: String, size: i32) {
        self.files.insert(name, size);
        self.size += size;
    }

    pub fn get_size(&self) -> i32 {
        if self.dirs.is_empty() {
            return self.size;
        }

        let mut total_size = self.size;
        for (name, dir) in &self.dirs {
            total_size += dir.get_size();
        }

        total_size
    }

    pub fn ls(&self) {
        println!("{}", self.name);
        for (name, size) in &self.files {
            println!("  {}", name);
        }
        for (name, dir) in &self.dirs {
            dir.ls();
        }
    }

    pub fn get_sizes(&self) -> Vec<i32> {
        let mut sizes = Vec::new();
        sizes.push(self.get_size());

        for (name, dir) in &self.dirs {
            for size in dir.get_sizes() {
                sizes.push(size);
            }
        }

        sizes
    }
}
