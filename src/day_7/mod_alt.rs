use crate::read_txt_file;

struct File {
    name: String,
    size: u32,
}

struct Directory {
    name: String,
    files: Vec<File>,
    directories: Vec<Directory>,
}

impl Directory {
    fn add_file(&mut self, file: File) {
        self.files.push(file);
        // self.files.insert(file.name.clone(), file);
    }

    fn add_directory(&mut self, dir: Directory) {
        self.directories.push(dir);
        // self.directories.insert(dir.name.clone(), dir);
    }

    fn get_file_size(&self) -> u32 {
        self.files.iter().map(|f| f.size).sum()
    }

    fn get_size(&self) -> u32 {
        let mut size = self.get_file_size();

        for dir in self.directories.iter() {
            size += dir.get_size();
        }

        size
    }
}

// fn generate_tree() {
//     let input = read_txt_file(7, crate::TextEnum::Input);

//     let cur_dir = &mut root;
//     // let prev_dir = &root;

//     for line in input.lines().skip(2) {
//         if line.starts_with("$") {
//             let cd = line.split_whitespace().last().unwrap();
//             if cd != "ls" {
//                 // on cd xxxx
//                 if cd != ".." {
//                     let cd = String::from(cd);
//                     cur_dir.add_directory(Directory {
//                         name: cd.clone(),
//                         files: Vec::new(),
//                         directories: Vec::new(),
//                     });

//                     // prev_dir = &cur_dir;
//                     cur_dir = &mut cur_dir.directories.last().unwrap();
//                 // on cd ..
//                 } else {
//                     // cur_dir = &mut prev_dir;
//                     // prev_dir = &&mut *cur_dir.directories.last().unwrap();
//                 }
//             }
//         } else {
//             let mut words = line.split_whitespace();
//             let first = words.next();

//             if first != Some("dir") {
//                 let size = first.unwrap().parse::<u32>().expect("Expected to have integer!");
//                 let name = words.next().unwrap();

//                 cur_dir.add_file(File {
//                     name: String::from(name),
//                     size,
//                 });
//                 // let dir = dir_map.get_mut(dir_stack.last().unwrap()).unwrap();
//                 // let file = File {
//                 //     name: String::from(name),
//                 //     size,
//                 // };

//                 // dir.files.insert(String::from(name), file);
//             }
//         }
//     }

//     println!("{:?}", root.get_size());

//     // dir_map
// }

pub fn solve() {
    println!("DAY 7");

    // let input = read_txt_file(7, crate::TextEnum::Input);

    let mut root = Directory {
        name: String::from("/"),
        files: Vec::new(),
        directories: Vec::new(),
    };

    let mut pointer = Vec::new();
    pointer.push(&mut root);

    // for line in input.lines() {
    //     if line.starts_with("$") {
    //         let cd = line.split_whitespace().last().unwrap();
    //         if cd != "ls" {
    //             // on cd xxxxx
    //             if cd != ".." {
    //                 let new_dir: Directory<'static> = Directory {
    //                     name: String::from(cd),
    //                     files: Vec::new(),
    //                     directories: Vec::new(),
    //                 };

    //                 let prev = pointer.last().unwrap();
    //                 prev.add_directory(new_dir);

    //                 pointer.push(&mut new_dir);
    //             // on cd ..
    //             } else {
    //                 pointer.pop();
    //             }
    //         }
    //     } else {
    //         let mut words = line.split_whitespace();
    //         let first = words.next();

    //         // when line starts with size
    //         if first != Some("dir") {
    //             let size = first
    //                 .unwrap()
    //                 .parse::<u32>()
    //                 .expect("Expected to have integer!");
    //             let name = String::from(words.next().unwrap());
    //             let file: &'static File = &File { size, name };

    //             let dir = pointer.last().unwrap();
    //             dir.add_file(*file);
    //         }
    //     }
    // }

    // // at this point all directory sizes are calculate
    // let total: u32 = dir_sizes.values().filter(|v| v <= &&(100000 as u32)).sum();

    // println!("Sum of directories: {}", total);
    assert_eq!(root.get_size(), 1118405);

    // let ununsed_space = 70_000_000 - dir_sizes.get("/").unwrap();
    // let required_space = 30_000_000 - &ununsed_space;

    // let least = dir_sizes.values().filter(|v| v > &&required_space).min();

    // println!("Least size directory: {}", least.unwrap());
    // for size in dir_sizes.values() {
    //     if size > &required_space {
    //         println!("{}", size);
    //     }
    // }
    // println!("Unused space: {}, Required space: {}", ununsed_space, required_space);
    // assert_eq!(&least, 12545514);
}
