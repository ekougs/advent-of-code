use crate::utils::lines;
use lazy_static::lazy_static;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn dirs_size(dirs_commands_filename: &str, max_size: usize) -> usize {
    if let Ok(lines) = lines(dirs_commands_filename) {
        let mut root_dir: Option<Rc<RefCell<Dir>>> = None;
        let mut current_dir: Option<Rc<RefCell<Dir>>> = None;

        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                match line {
                    line if CD_PARENT_LINE_REGEX.is_match(&line) => {
                        current_dir = match current_dir {
                            Some(dir) => Dir::parent(dir),
                            None => panic!("current dir should not be empty when moving to parent"),
                        };
                    }
                    line if CD_LINE_REGEX.is_match(&line) => {
                        let dirname = match Dir::dirname(&line) {
                            Ok(dirname) => dirname,
                            Err(_) => panic!("could not extract directory from {}", line),
                        };
                        current_dir = Dir::move_to(current_dir, &dirname);
                        if root_dir.is_none() {
                            root_dir = Some(current_dir.to_owned().unwrap());
                        }
                    }
                    line if FILE_LINE_REGEX.is_match(&line) => {
                        let file_result = File::try_create(&line);
                        let file = match file_result {
                            Ok(file) => file,
                            Err(_) => panic!("could not create file from `{}`", line),
                        };
                        match &current_dir {
                            Some(dir) => Dir::add_file(dir, file),
                            None => panic!("cannot add a file to a None dir"),
                        };
                    }
                    _ => continue,
                };
            }
        }
        return Dir::size_under(root_dir, max_size)
    };
    panic!("should have found file {}", dirs_commands_filename)
}

lazy_static! {
    static ref CD_PARENT_LINE_REGEX: Regex = Regex::new(r"^\$ cd \.\.$").unwrap();
    static ref CD_LINE_REGEX: Regex = Regex::new(r"^\$ cd (.+)$").unwrap();
    static ref DIR_NAME_LINE_REGEX: Regex = Regex::new(r"^dir (.+)$").unwrap();
    static ref FILE_LINE_REGEX: Regex = Regex::new(r"^(\d+) (.+)$").unwrap();
    static ref LS_LINE_REGEX: Regex = Regex::new(r"^\$ ls$").unwrap();
}

#[derive(Clone,Debug)]
struct Dir {
    name: Option<String>,
    parent: Option<Rc<RefCell<Dir>>>,
    dirs: HashMap<String, Rc<RefCell<Dir>>>,
    files: Vec<File>,
}

impl Dir {
    fn get_or_insert(dir_ref: Rc<RefCell<Dir>>, subdirname: &str) -> Option<Rc<RefCell<Dir>>> {
        let dir = match dir_ref.try_borrow_mut() {
            Ok(mut dir) => {
                if !dir.dirs.contains_key(subdirname) {
                    dir.dirs.insert(
                        String::from(subdirname),
                        Rc::new(RefCell::new(Dir {
                            name: Some(String::from(subdirname)),
                            parent: Some(Rc::clone(&dir_ref)),
                            dirs: HashMap::new(),
                            files: vec![],
                        })),
                    );
                }
                dir
            }
            Err(_) => panic!("Could not borrow a dir to add subdir {}", subdirname),
        };

        match dir.dirs.get(subdirname) {
            Some(subdir) => Some(Rc::clone(subdir)),
            None => panic!(
                "Could not find subdir {} into dir {}",
                subdirname,
                dir.to_owned().get_name()
            ),
        }
    }

    fn get_name(self) -> String {
        match self.name {
            Some(dir_name) => String::from(dir_name),
            None => String::from("/"),
        }
    }

    fn dirname(line: &str) -> Result<String, ()> {
        let mut cd_line_matches = CD_LINE_REGEX.captures_iter(line);
        match cd_line_matches.next() {
            Some(captures) => Ok(captures[1].to_string()),
            _ => Err(()),
        }
    }

    fn parent(dir: Rc<RefCell<Dir>>)-> Option<Rc<RefCell<Dir>>> {
        match dir.try_borrow() {
            Ok(dir) => match dir.to_owned().parent {
                Some(parent) => Some(Rc::clone(&parent)),
                None => panic!("cannot move to parent"),
            },
            Err(_) => panic!("current dir could not be borrowed"),
        }
    }

    fn move_to(current_dir: Option<Rc<RefCell<Dir>>>, dirname: &str) -> Option<Rc<RefCell<Dir>>> {
        match current_dir {
            None => Some(Rc::new(RefCell::new(Dir {
                name: None,
                parent: None,
                dirs: HashMap::new(),
                files: vec![],
            }))),
            Some(dir) => Dir::get_or_insert(dir, dirname),
        }
    }

    fn add_file(current_dir: &Rc<RefCell<Dir>>, file: File) {
        match current_dir.try_borrow_mut() {
            Ok(mut mut_current_dir) => mut_current_dir.files.push(file),
            Err(_) => panic!("could not borrow as mutable the current dir"),
        };
    }

    fn size(current_dir: &Rc<RefCell<Dir>>) -> usize {
        match current_dir.try_borrow() {
            Ok(current_dir) => {
                let mut size: usize = current_dir.files.iter().map(|f| f.size).sum();
                for dir in current_dir.dirs.values() {
                    size += Dir::size(&dir);
                }
                size
            },
            Err(_) => panic!("could not borrow as mutable the current dir"),
        }
    }

    fn size_under(root_dir: Option<Rc<RefCell<Dir>>>, max_dir_size: usize) -> usize {
        let mut total_size_under_max = 0;
        let mut dirs: Vec<Option<Rc<RefCell<Dir>>>> = vec![root_dir];
        while !dirs.is_empty() {
            let dir = match dirs.pop() {
                Some(Some(dir)) => dir,
                _ => continue,
            };
            let size = Dir::size(&dir);
            if size < max_dir_size {
                total_size_under_max += size;
            }
            match dir.try_borrow() {
                Ok(dir) => {
                    for dir in dir.dirs.values() {
                        dirs.push(Some(Rc::clone(dir)));
                    }
                }
                Err(_) => panic!("could not borrow dir for size_under"),
            };
        }
        total_size_under_max
    }
}

#[derive(Clone,Debug)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn try_create(line: &str) -> Result<Self, ()> {
        let mut file_line_matches = FILE_LINE_REGEX.captures_iter(line);
        match file_line_matches.next() {
            Some(captures) => {
                Ok(File{name: captures[2].to_string(), size: usize::from_str_radix(&captures[1], 10).unwrap()})
            },
            _ => Err(()),
        }
    }
}

enum ParsingResult {
    File(File),
    Dir(Dir),
}
