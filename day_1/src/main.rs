use std::error::Error;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let f = std::fs::File::open("input_a.txt")?;
    let r = BufReader::new(f);
    let mut sum = 0;
    for line in r.lines() {
        let line = line?;
        let sign = match line.chars().nth(0) {
            Some('-') => -1, 
            Some('+') => 1,
            _ => 1
        };
        let rest = &line[1..];
        let val = rest.to_string().parse::<i32>()?;
        sum += val * sign;
    }
    println!("Sum is {}", sum);
    Ok(())
}
