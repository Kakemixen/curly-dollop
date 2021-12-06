pub mod fileops {
    use std::fs::File;
    use std::path::Path;
    use std::io::{self, BufRead};

    pub fn get_file_lines<P>(path: P)
        -> io::Lines<io::BufReader<File>>
    where P: AsRef<Path>
    {
        let file = File::open(path).expect("could open file!");
        println!("test");
        io::BufReader::new(file).lines()
    }
}

