pub mod fileops {
    use std::fs::File;
    use std::path::Path;
    use std::io::{self, BufRead};

    pub fn get_file_lines<P>(path: P)
        -> impl Iterator<Item = String>
    where P: AsRef<Path>
    {
        let file = File::open(path).expect("could open file!");
        println!("test");
        io::BufReader::new(file).lines()
            .map(|x| {
                x.expect("couldn't read line!")
            })
    }
}

