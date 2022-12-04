use std::{fs::File, io::BufReader, env};

pub fn read_file(name: &str, day: i32) -> BufReader<File> {
    let input_dir = format!( "{}{}", env::current_dir().unwrap().to_str().unwrap(), format!(r#"\inputs\{}{}.txt"#, name, day));
    let file = File::open(input_dir).unwrap();
    BufReader::new(file)
}