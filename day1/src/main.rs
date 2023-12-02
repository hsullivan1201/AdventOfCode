use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

fn main() -> std::io::Result<()>{
    let lines = include_str!("input").lines();
    let lines2 = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet".lines();
    let mut result: String = "".to_owned();
    let mut sum = 0;
    for line in lines {
        let mut firstChar = '#';
        let mut lastChar = '#';
        for char in line.chars() {
            if char.is_numeric() & (firstChar == '#') {
                firstChar = char;
            }
            else if char.is_numeric(){
                lastChar = char;
            }
        }
        if lastChar == '#' {
            lastChar = firstChar;
        }

        let stringSum = "{firstChar}{lastChar}";

        sum += stringSum.parse::<i32>().unwrap();
        println!("{firstChar}{lastChar}");
        result.push(firstChar);
        result.push(lastChar);
        result.push_str("\n")
    }
    result.push_str("Sum = {sum}");
    let mut file = File::create("result.txt")?;
    write!(file, "{}", result);
    Ok(())

}
