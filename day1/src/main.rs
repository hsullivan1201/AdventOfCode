use std::collections::HashMap;

fn main(){
    let lines = include_str!("input").lines();
    let lines2 = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet".lines();
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
        let mut stringSum: String = "".to_owned();
        stringSum.push(firstChar);
        stringSum.push(lastChar);
        sum += stringSum.parse::<i32>().unwrap();
    }
    println!("Sum is {}", sum);
}

fn part2 () {
    let mut numsList = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)]);
    let lines = include_str!("input").lines();
    let lines2 = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four
        \n4nineeightseven2\nzoneight234\n7pqrstsixteen".lines();
    let mut sum = 0;
    for line in lines {
        let keys:Vec<&str> = numsList.keys().collect();
        for key in keys {
            let matches: Vec<&str> = line.rmatches(key).collect();
        }
    }
}
