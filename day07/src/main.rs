use std::fs::File;
use std::io::{BufRead,BufReader};
use regex::Regex;

fn main() {
    println!("Advent of code 2021, day 7!");

    let filename = "rsc/input2.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(r"\d+").unwrap();
    let mut sum = 0;
    for (_idx, line) in reader.lines().enumerate() {
        println!("{:?}", line);

        for cap in re.captures_iter(&line.unwrap()){
            println!("cap: {:?}", &cap[0]);

            let nbr: i32 = cap[0].parse().unwrap();
            if nbr >= 100000{
                sum += nbr;
            }
        }


    }
    println!{"sum: {}", sum};
}