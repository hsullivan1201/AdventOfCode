use std::{collections::HashMap, iter::Map};

fn main2(){
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

fn main () {
    let mut numsList = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9")]);
    let lines = include_str!("input").lines();
    let lines2 = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four
        4nineeightseven2\nzoneight234\n7pqrstsixteen".lines();
    let mut sum = 0;
    for mut line in lines {
        //replac all matches with digits
       let line = line.replace("one", "one1one");
       let line =line.replace("two", "two2two");
       let line =line.replace("three", "three3three");
       let line =line.replace("four", "four4four");
       let line =line.replace("five", "five5five");
       let line =line.replace("six", "six6six");
       let line =line.replace("seven", "seven7seven");
       let line = line.replace("eight", "eight8eight");
       let line = line.replace("nine", "nine9nine");
       println!("{}",line);
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
       println!("stringsum = {}",stringSum);
       sum += stringSum.parse::<i32>().unwrap();
    }
    println!("Sum is {}", sum);
    

}
