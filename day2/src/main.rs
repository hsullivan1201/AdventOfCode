fn main() {
    let lines = include_str!("input.txt").lines();
    let mut gameSum = 5050;
    let mut isValid = true;
    for line in lines { 
        isValid = true;
        let mut charList = line.chars();
        let gameNum = charList.nth(5).unwrap();
        let mut stringSum:String = "".to_owned();
        let secondDig = charList.nth(0).unwrap();
        let thirdDig = charList.nth(0).unwrap();
        stringSum.push(gameNum);
        if secondDig.is_numeric() {
            //println!("Second digit exits, it is {}, first is {}",secondDig,gameNum);
            stringSum.push(secondDig);
            if thirdDig.eq(&'0'){
                stringSum.push(thirdDig);
            }
        }

        let gameNumInt = stringSum.parse::<i32>().unwrap();
        let mut i = 0;
        let mut marbleNum = 0;
        let mut marbleNumStr:String = "".to_string();
        for oneChar in line.chars() {
            if ((i > 6) & oneChar.is_numeric()) {
                let nextChar = line.chars().nth(i+1).unwrap();
                if nextChar.is_numeric() {
                    
                     marbleNumStr = format!("{}{}",oneChar,nextChar);
                     i += 1;
                }
                else {
                    marbleNumStr =format!("{}",oneChar);
                }
                marbleNum = marbleNumStr.parse::<i32>().unwrap();
                if (marbleNum > 14) & line.chars().nth(i+2).unwrap().eq(&'b') {
                    println!("invalid at {} in game {}.",marbleNum,gameNumInt);
                    isValid = false;
                }
                else if (marbleNum > 13) & line.chars().nth(i+2).unwrap().eq(&'g') {
                    println!("invalid at {} in game {}",marbleNum,gameNumInt);
                    isValid = false
                }
                else if (marbleNum > 12) & line.chars().nth(i+2).unwrap().eq(&'r') {
                    println!("invalid at {} in game {}",marbleNum,gameNumInt);
                    isValid = false;
                }
                
            }
            i += 1;
        }
        if isValid {
            //gameSum += gameNumInt;
            //println!("Game {} is valid, so sum is now {}",gameNumInt,gameSum);
        }
        else {
            gameSum -= gameNumInt;
            println!("INVALID - subtracting {} so total is now {}.",gameNumInt,gameSum);
        }
    }
    println!("Sum is {}",gameSum);
}


