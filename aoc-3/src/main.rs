use std::{fs::read_to_string, collections::HashMap};
use regex::{Regex, Match};

#[derive(Debug)]
struct Row<'a> 
{
    digits: Vec<Match<'a>>,
    specials: Vec<Match<'a>>,
    gears: Vec<Match<'a>>,
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let parts = parse(&input);
    let (p_sum, g_sum) = sum(parts);
    println!("Part sum: {}, {}",p_sum,g_sum);
}

fn parse(input: &str) -> HashMap::<usize,Row> {
    let mut rows = HashMap::<usize,Row>::new();
    for (i, s) in input.split('\n').map(|s| s.trim()).enumerate() {
        // println!("row {}",i);
        let re_num = Regex::new(r"(?P<part>\d+)").unwrap(); //r"Game (?<id>\d)"
        let re_special = Regex::new(r"[^. \n\d]").unwrap(); //r"Game (?<id>\d)"
        let re_gears = Regex::new(r"\*").unwrap(); //r"Game (?<id>\d)"

        // let matches: Vec<_> = re_special.find_iter(input)
        let mut digits  = Vec::<Match>::new();
        let mut specials  = Vec::<Match>::new();
        let mut gears = Vec::<Match>::new();

        for m in re_num.find_iter(s) {
            digits.push(m);
        }

        for m in re_special.find_iter(s) {
            specials.push(m);
        }

        for m in re_gears.find_iter(s) {
            gears.push(m);
        }
        rows.insert(i,Row { digits: digits, specials: specials, gears: gears });
    }

    // println!("{:?}\n",rows);
    rows
}

fn sum(parts: HashMap::<usize,Row>) -> (u32,u32) {
    let mut sum:u32 = 0;
    let mut sum_gear: u32 = 0;

    for &k in parts.keys().into_iter(){
        if k == 0 as usize {
            let r = parts.get(&k);
            let ar = parts.get(&(k+1));
            sum += check_row(None, r, ar);
        } else if k < parts.len() {
            let br = parts.get(&(k-1));
            let r = parts.get(&k);
            let ar = parts.get(&(k+1));
            sum += check_row(br, r, ar);
        } else {
            let br = parts.get(&(k-1));
            let r = parts.get(&k);
            sum += check_row(br, r, None);

        }
    }
    sum_gear += check_gears(parts);

    (sum, sum_gear)
}

fn check_row(br:Option<&Row<'_>>, r:Option<&Row<'_>>, ar:Option<&Row<'_>>) -> u32 {
    let mut sum: u32 = 0;
    if br.is_none() {
        for m in &r.unwrap().digits {
            let r_b = check_special(m, &r.unwrap().specials);
            let ar_b = check_special(m, &ar.unwrap().specials);
            
            if r_b || ar_b {
                sum += m.as_str().parse::<u32>().unwrap(); 
            }
        }
    } else if ar.is_none() {
        for m in &r.unwrap().digits {
            let br_b = check_special(m, &br.unwrap().specials);
            let r_b = check_special(m, &r.unwrap().specials);
           
            if br_b || r_b {
                sum += m.as_str().parse::<u32>().unwrap(); 
            }
        }
    } else {
        for m in &r.unwrap().digits {
            let br_b = check_special(m, &br.unwrap().specials);
            let r_b = check_special(m, &r.unwrap().specials);
            let ar_b = check_special(m, &ar.unwrap().specials);
            
            if br_b || r_b || ar_b {
                sum += m.as_str().parse::<u32>().unwrap(); 
            }
        }
    }
    sum
}

fn check_gears(parts: HashMap::<usize,Row>) -> u32 {
    let mut sum:u32 = 0;
    for key in parts.keys().into_iter() {
        if *key > 0 && *key < parts.len()-1 {
            let r = &parts.get(key).unwrap().digits;
            let rb = &parts.get(&(key-1)).unwrap().digits;
            let ra = &parts.get(&(key+1)).unwrap().digits;
            for g in &parts.get(key).unwrap().gears{
                test_gear(g, &[r,rb,ra]).map(|x| sum += x);
            }     
        }else if *key == 0 {
            let r = &parts.get(key).unwrap().digits;
            let ra = &parts.get(&(key+1)).unwrap().digits;
            for g in &parts.get(key).unwrap().gears{
                test_gear(g, &[r,ra]).map(|x| sum += x);
            }     
        } else {
            let r = &parts.get(key).unwrap().digits;
            let rb = &parts.get(&(key-1)).unwrap().digits;
            for g in &parts.get(key).unwrap().gears{
                test_gear(g, &[r,rb]).map(|x| sum += x);
            }     

        }
    }

    check_gears_2(parts);
    sum
}

fn check_gears_2(parts: HashMap::<usize,Row>)->u32{
    let mut sum = 0;

    let mut keys: Vec<_> = parts.keys().clone().collect();
    keys.sort_unstable();

    
    for w in keys.windows(3) {
        println!("{:?}",w)
    }

    sum
}

fn test_gear(g:&Match, d_vec: &[&Vec<Match<'_>>]) -> Option<u32>{
    let mut res = Vec::<u32>::new();
    for &digits in d_vec.into_iter() {
        for d in digits {
            if d.start() == 0 {
                if d.start() <= g.end() && g.start() <= d.end() {
                    res.push(d.as_str().parse::<u32>().unwrap());
                }                
            }else if d.start() <= g.end() && g.start() <= d.end() {
                println!("else: {:?}{:?}",g,d);
                res.push(d.as_str().parse::<u32>().unwrap());
            }
        }
    }

    if res.len() >= 2 {
        println!("res: {:?}",res);
        return Some(res.iter().fold(1, |sum, x|sum*x))
    }
    
    None
}

fn check_special(m:&Match, specials: &Vec<Match<'_>>) -> bool {
    let mut result = false;
    if specials.is_empty(){
        return false;
    } else {
        for s in specials {
            if m.start() == 0 {
                if m.start() <= s.end() && s.start() <= m.end() {
                    result = true;
                }                
            }else if m.start() <= s.end() && s.start() <= m.end() {
                result = true;
            }
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_parts(){
        let parts = parse(
        "467..114..
               ...*......
               ..35..633.
               ......#...
               617*......
               .....+.58.
               .%592.....
               ......755.
               ...$.*....
               .664.598..");
        assert_eq!(sum(parts),(4361,467835));
    }
}