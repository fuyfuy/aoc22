use std::{
    fs::File,
    io::{BufRead, BufReader}, env,
};
pub fn run() {
    let input_dir = format!("{}{}", env::current_dir().unwrap().to_str().unwrap(), r"\inputs\inp1.txt");
    let file = File::open(input_dir).unwrap();
    let reader = BufReader::new(file);
    let mut top1 = 0;
    let mut top2 = 0;
    let mut top3 = 0;
    let mut current_sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if !line.is_empty() {
            let num = line.parse::<u32>().expect("somehow not an int");
            current_sum += num;
        } else {
            if current_sum > top1 {
                top3 = top2;
                top2 = top1;
                top1 = current_sum;
            } else if current_sum > top2 {
                top3 = top2;
                top2 = current_sum;
            } else if current_sum > top3 {
                top3 = current_sum;
            }
            current_sum = 0;
        }
    }
    println!("top 3 sum is {}", top1 + top2 + top3);
    println!("top 1 {}", top1);
}
