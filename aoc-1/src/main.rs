use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input").unwrap();
    //let row = input.split('\n').map(|s| s.trim());

    // let mut sum:i32 = 0;
    // for (_, s) in row.enumerate() {
    //     match first_and_last_2(s) {
    //         Some(x) => sum += x,
    //         None => ()
    //     }
    // }
    let sum = sum(input);

    println!("{}", sum);
}

fn sum(data: String) -> i32 {
    let row = data.split('\n').map(|s| s.trim());
    let mut sum:i32 = 0;

    for (_, s) in row.enumerate() {
        match first_and_last(s) {
            Some(x) => sum += x,
            None => ()
        }
    }
    sum
}


fn first_and_last(data: &str) -> Option<i32>{

    let a1 = vec![
        ("one",1),
        ("1",1),
        ("two",2),
        ("2",2),
        ("three",3),
        ("3",3),
        ("four",4),
        ("4",4),
        ("five",5),
        ("5",5),
        ("six",6),
        ("6",6),
        ("seven",7),
        ("7",7),
        ("eight",8),
        ("8",8),
        ("nine",9),
        ("9",9)];

    // println!("{}",data);

    let mut first:(i32,i32) = (0,-1);
    let mut last:(i32,i32) = (0,-1);

    for num in &a1 {
        let test = data.find(num.0);
        match test {
            Some(i) => {
                // println!("found {:?} at pos: {:?}",num.1,i);
                if (i as i32) < first.1 || first.1 == -1{
                    // println!("setting first{},{}, i{}",first.0, first.1, (i as i32));
                    first.0 = num.1;
                    first.1 = i as i32;
                }

                if (i as i32) > last.1 {
                    // println!("setting last{},{}, i{}",last.0, last.1, (i as i32));
                    last.0 = num.1;
                    last.1 = i as i32;
                }

            },
            None => (),
        }
    }

    for num in &a1 {
        let test = data.rfind(num.0);
        match test {
            Some(i) => {
                if (i as i32) > last.1 {
                    // println!("setting last{},{}, i{}",last.0, last.1, (i as i32));
                    last.0 = num.1;
                    last.1 = i as i32;
                }

            },
            None => (),
        }
    }


    if first.1 == -1 {
        return None
    }


    //println!("{},{}",data,first.0*10+last.0);

    Some(first.0*10+last.0)

    // let digits: Vec<&str> = data.matches(char::is_numeric).collect();
    // //let first = digits.first().unwrap()+ digits.pop().unwrap();
    // if digits.len() == 0 {
    //     return None
    // }
    // Some(format!("{}{}",digits[0],digits[digits.len()-1]).parse::<u32>().unwrap())
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_first_and_last_digit() {
        println!("1abc2");
        let result = first_and_last("1abc2");
        assert_eq!(result,Some(12));
        
        println!("pqr3stu8vwx");
        let result = first_and_last("pqr3stu8vwx");
        assert_eq!(result,Some(38));
        
        println!("a1b2c3d4e5f");
        let result = first_and_last("a1b2c3d4e5f");
        assert_eq!(result,Some(15));
        
        println!("treb7uchet");
        let result = first_and_last("treb7uchet");
        assert_eq!(result,Some(77));
        
        println!("trebuchet");
        let result = first_and_last("trebuchet");
        assert_eq!(result,None);

        println!("two1nine");
        let result = first_and_last("two1nine");
        assert_eq!(result,Some(29));

        println!("eightwothree");
        let result = first_and_last("eightwothree");
        assert_eq!(result,Some(83));

        println!("abcone2threexyz");
        let result = first_and_last("abcone2threexyz");
        assert_eq!(result,Some(13));

        println!("xtwone3four");
        let result = first_and_last("xtwone3four");
        assert_eq!(result,Some(24));

        println!("4nineeightseven2");
        let result = first_and_last("4nineeightseven2");
        assert_eq!(result,Some(42));

        println!("zoneight234");
        let result = first_and_last("zoneight234");
        assert_eq!(result,Some(14));
        
        println!("7pqrstsixteen");
        let result = first_and_last("7pqrstsixteen");
        assert_eq!(result,Some(76));
    }
    
    #[test]
    fn checksum_1() {
        let sum = sum(String::from("1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"));
        assert_eq!(sum, 142);
    }

    #[test]
    fn checksum_2() {
        let sum = sum(String::from("two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"));
        assert_eq!(sum, 281);
    }

    #[test]
    fn checksum_3() {
        let sum = sum(String::from("two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        trebuchet
        75stwofivefvnjtktztwo"));
        assert_eq!(sum, 281+142+72);
    }
}

//55877