use std::io;

fn celebration_value_of(line: &str) -> u32 {
    let mut first_digit = 0;
    let mut last_digit = 0;
    for ch in line.chars() {
        if let Some(digit) = ch.to_digit(10) {
            last_digit = digit;
            if first_digit == 0 {
                first_digit = digit;
            }
        }
    }
    first_digit * 10 + last_digit
}

pub fn part_1<B: io::BufRead>(input: io::Lines<B>) -> io::Result<String> {
    Ok(input
        .map(|l| Ok(celebration_value_of(&l?)))
        .sum::<io::Result<u32>>()?
        .to_string())
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case("a1b2c3d4e5f" => 15)]
    #[test_case("treb7uchet" => 77)]
    fn it_gets_the_right_number(line: &str) -> u32 {
        super::celebration_value_of(line)
    }
}
