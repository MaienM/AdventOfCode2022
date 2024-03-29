use aoc::runner::run;

fn to_snafu(mut num: u64) -> String {
    let mut chars = Vec::new();
    while num > 0 {
        let (chr, diff): (char, i64) = match num % 5 {
            0 => ('0', 0),
            1 => ('1', 1),
            2 => ('2', 2),
            3 => ('=', -2),
            4 => ('-', -1),
            _ => panic!(),
        };
        chars.push(chr);
        num = ((num as i64 - diff) / 5) as u64;
    }
    chars.into_iter().rev().collect()
}

fn from_snafu(num: &str) -> u64 {
    let mut result = 0;
    for c in num.chars() {
        let value: i64 = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '-' => -1,
            '=' => -2,
            _ => panic!(),
        };
        result = result * 5 + value;
    }
    result as u64
}

pub fn part1(input: &str) -> String {
    let lines = input.trim().split('\n').map(str::trim);
    let numbers = lines.map(from_snafu);
    to_snafu(numbers.sum())
}

pub fn part2(_input: &str) -> &'static str {
    "I did it!"
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &str = "
        1=-0-2
        12111
        2=0=
        21
        2=01
        111
        20012
        112
        1=-1=
        1-12
        12
        1=
        122
    ";

    const EXAMPLES: [(u64, &str); 15] = [
        (1, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
        (6, "11"),
        (7, "12"),
        (8, "2="),
        (9, "2-"),
        (10, "20"),
        (15, "1=0"),
        (20, "1-0"),
        (2022, "1=11-2"),
        (12345, "1-0---0"),
        (314_159_265, "1121-1110-1=0"),
    ];

    #[test]
    fn example_from_snafu() {
        for (num, snafu) in EXAMPLES {
            assert_eq!(from_snafu(snafu), num);
        }
    }

    #[test]
    fn example_to_snafu() {
        for (num, snafu) in EXAMPLES {
            assert_eq!(to_snafu(num), snafu);
        }
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), "2=-1=0");
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), "I did it!");
    }
}
