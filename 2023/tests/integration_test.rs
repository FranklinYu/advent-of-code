use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

#[derive(Copy, Clone)]
struct Day(i32);

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn read_test_input(day: Day, index: usize) -> io::Lines<BufReader<File>> {
    let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_data.push(format!("test-data/day-{day}/input-{index}.txt"));
    let file = File::open(&test_data).unwrap();
    BufReader::new(file).lines()
}

fn read_expected_outputs(day: Day) -> io::Lines<BufReader<File>> {
    let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_data.push(format!("test-data/day-{day}/answers.txt"));
    let file = File::open(&test_data).unwrap();
    BufReader::new(file).lines()
}

#[test]
fn it_passes_for_day_1() {
    let day = Day(1);
    let results = read_expected_outputs(day)
        .collect::<io::Result<Vec<_>>>()
        .unwrap();
    // demo for part 1
    assert_eq!(
        aoc_2023::day_01::part_1(read_test_input(day, 0)).unwrap(),
        results[0]
    );

    // verification for part 1
    assert_eq!(
        aoc_2023::day_01::part_1(read_test_input(day, 1)).unwrap(),
        results[1]
    );

    // demo for part 2
    assert_eq!(
        aoc_2023::day_01::part_2(read_test_input(day, 2)).unwrap(),
        results[2]
    );

    // verification for part 2 (data was reused)
    assert_eq!(
        aoc_2023::day_01::part_2(read_test_input(day, 1)).unwrap(),
        results[3]
    );
}

#[test]
fn it_passes_for_day_2() {
    let day = Day(2);
    let results = read_expected_outputs(day)
        .collect::<io::Result<Vec<_>>>()
        .unwrap();

    // demo for part 1
    assert_eq!(
        aoc_2023::day_02::part_1(read_test_input(day, 0)).unwrap(),
        results[0]
    );

    // verification for part 1
    assert_eq!(
        aoc_2023::day_02::part_1(read_test_input(day, 1)).unwrap(),
        results[1]
    );

    // demo for part 2 (data was reused)
    assert_eq!(
        aoc_2023::day_02::part_2(read_test_input(day, 0)).unwrap(),
        results[2]
    );

    // verification for part 2 (data was reused)
    assert_eq!(
        aoc_2023::day_02::part_2(read_test_input(day, 1)).unwrap(),
        results[3]
    );
}
