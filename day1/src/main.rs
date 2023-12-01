use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let reader = read_to_string("./inputs/day1.txt").unwrap();

    part_1(&reader);
    part_2(&reader);
}

fn part_1(reader: &String) {
    let val = reader.lines().fold(0, |acc: u32, line| {
        let digits = line.chars().filter_map(|x| x.to_digit(10));

        let first = digits.clone().next().unwrap();
        let last = digits.last().unwrap();

        let num = first * 10 + last;

        acc + num
    });

    println!("Part 1: {}", val);
}

fn part_2(reader: &String) {
    let val = reader.lines().fold(0, |acc: u32, line| {
        let digits = get_nums(line);

        let first = digits.first().unwrap();
        let last = digits.last().unwrap();

        let num = first * 10 + last;
        // println!("| {}", num);

        acc + num
    });
    println!("Part 2: {}", val);
}

fn get_nums(line: &str) -> Vec<u32> {
    let pattern = r"(^(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9))";
    let re = Regex::new(pattern).unwrap();

    match_all_substrings(line, &re)
        .iter()
        .map(|mat| {
            // println!("{:?}", mat);
            match *mat {
                "one" | "1" => 1,
                "two" | "2" => 2,
                "three" | "3" => 3,
                "four" | "4" => 4,
                "five" | "5" => 5,
                "six" | "6" => 6,
                "seven" | "7" => 7,
                "eight" | "8" => 8,
                "nine" | "9" => 9,
                _ => {
                    eprintln!("Error");
                    0
                }
            }
        })
        .collect()
}

fn match_substring<'a>(substr: &'a str, re: &Regex) -> Option<&'a str> {
    re.find(substr).map(|mat| &substr[mat.start()..mat.end()])
}

fn match_all_substrings<'a>(line: &'a str, re: &Regex) -> Vec<&'a str> {
    // print!("{} ||", line);
    (0..line.len())
        .map(|i| match_substring(&line[i..], &re))
        .filter(|x| x.is_some())
        .map(|x| {
            // print!("{} ", x.unwrap());
            x.unwrap()
        })
        .collect()
}

#[test]
fn test_match_substring() {
    let pattern = r"(^(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9))";
    let re = Regex::new(pattern).unwrap();
    let substr = "9f";
    let mat = match_substring(substr, &re);
    assert_eq!(mat, Some("9"));
    println!("{} ", &substr[0..1]);
}
