use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Copy, Clone)]
struct Day(i32);

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Day {
    fn test_file(self, name: &str) -> File {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push(format!("test-data/day-{self}/{name}"));
        File::open(&test_data).unwrap()
    }
}

#[derive(Deserialize)]
struct TestCase {
    file: String,
    answer: String,
}

struct TestSuite {
    day: Day,
    configuration: HashMap<String, Vec<TestCase>>,
}

enum Part {
    Part1,
    Part2,
}

impl Part {
    fn to_str(&self) -> &str {
        match self {
            Part::Part1 => "part-1",
            Part::Part2 => "part-2",
        }
    }
}

type Lines = io::Lines<BufReader<File>>;

impl TestSuite {
    fn test_cases(&self, part: Part) -> impl Iterator<Item = (Lines, &str)> {
        self.configuration[part.to_str()]
            .iter()
            .map(|tc| -> (Lines, &str) {
                let br = BufReader::new(self.day.test_file(&tc.file));
                (br.lines(), &tc.answer)
            })
    }
}

impl Day {
    fn load_test_suite(self) -> TestSuite {
        TestSuite {
            day: self,
            configuration: serde_yaml::from_reader(self.test_file("answers.yaml")).unwrap(),
        }
    }
}

#[test]
fn it_passes_for_day_1() {
    let test_suite = Day(1).load_test_suite();

    for (lines, answer) in test_suite.test_cases(Part::Part1) {
        assert_eq!(aoc_2023::day_01::part_1(lines).unwrap(), answer);
    }

    for (lines, answer) in test_suite.test_cases(Part::Part2) {
        assert_eq!(aoc_2023::day_01::part_2(lines).unwrap(), answer);
    }
}

#[test]
fn it_passes_for_day_2() {
    let test_suite = Day(2).load_test_suite();

    for (lines, answer) in test_suite.test_cases(Part::Part1) {
        assert_eq!(aoc_2023::day_02::part_1(lines).unwrap(), answer);
    }

    for (lines, answer) in test_suite.test_cases(Part::Part2) {
        assert_eq!(aoc_2023::day_02::part_2(lines).unwrap(), answer);
    }
}

#[test]
fn it_passes_for_day_7() {
    let test_suite = Day(7).load_test_suite();

    for (lines, answer) in test_suite.test_cases(Part::Part1) {
        assert_eq!(aoc_2023::day_07::part_1(lines).unwrap(), answer);
    }

    for (lines, answer) in test_suite.test_cases(Part::Part2) {
        assert_eq!(aoc_2023::day_07::part_2(lines).unwrap(), answer);
    }
}
