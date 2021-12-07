use aoclib::fileops;

fn main() {
    let lines = fileops::get_file_lines("input2.txt");
    for line in lines {
        println!("{}", line);
    }
}
