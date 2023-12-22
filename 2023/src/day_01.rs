use std::io;

use regex::Regex;

fn celebration_value_of<I: std::iter::Iterator<Item = u32>>(digits: I) -> u32 {
    let mut first_digit = 0;
    let mut last_digit = 0;
    for digit in digits {
        last_digit = digit;
        if first_digit == 0 {
            first_digit = digit;
        }
    }
    first_digit * 10 + last_digit
}

fn digits_in<'a>(line: &'a str) -> impl std::iter::Iterator<Item = u32> + 'a {
    line.chars().filter_map(|c| c.to_digit(10))
}

pub fn part_1<B: io::BufRead>(input: io::Lines<B>) -> io::Result<String> {
    Ok(input
        .map(|l| Ok(celebration_value_of(digits_in(&l?))))
        .sum::<io::Result<u32>>()?
        .to_string())
}

fn english_digits_in<'a>(
    line: &'a str,
    re: &'a Regex,
) -> impl std::iter::Iterator<Item = u32> + 'a {
    let mut start = 0;
    std::iter::from_fn(move || {
        let m = re.find(&line[start..])?;
        start += m.range().min().unwrap() + 1;
        Some(match m.as_str() {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            s => s.parse().unwrap(),
        })
    })
}

fn regex() -> Regex {
    Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d").unwrap()
}

pub fn part_2<B: io::BufRead>(input: io::Lines<B>) -> io::Result<String> {
    let re = regex();
    Ok(input
        .map(|l| Ok(celebration_value_of(english_digits_in(&l?, &re))))
        .sum::<io::Result<u32>>()?
        .to_string())
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case("a1b2c3d4e5f" => 15)]
    #[test_case("treb7uchet" => 77)]
    fn it_gets_the_right_number(line: &str) -> u32 {
        super::celebration_value_of(super::digits_in(line))
    }

    #[test_case("threeight" => 38)]
    #[test_case("zoneight234" => 14)]
    fn it_understands_english(line: &str) -> u32 {
        let re = super::regex();
        super::celebration_value_of(super::english_digits_in(line, &re))
    }
}
