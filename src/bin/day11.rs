use aoc::runner::run;

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}
impl Operation {
    pub fn apply(&self, lhs: u64) -> u64 {
        match self {
            Operation::Add(rhs) => lhs + rhs,
            Operation::Mul(rhs) => lhs * rhs,
            Operation::Square => lhs * lhs,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    targets: [usize; 2],
}

fn parse_input(input: &str) -> Vec<Monkey> {
    return input
        .trim()
        .split("\n\n")
        .map(|block| {
            let mut lines = block.trim().split('\n').map(str::trim);
            assert!(lines.next().unwrap().starts_with("Monkey"));

            let mut parts = lines.next().unwrap().splitn(2, ':');
            assert_eq!(parts.next().unwrap(), "Starting items");
            let items = parts
                .next()
                .unwrap()
                .split(',')
                .map(str::trim)
                .map(str::parse)
                .map(Result::unwrap)
                .collect();

            let mut parts = lines.next().unwrap().splitn(2, '=');
            assert_eq!(parts.next().unwrap(), "Operation: new ");
            let parts: [&str; 3] = parts
                .next()
                .unwrap()
                .trim()
                .splitn(3, ' ')
                .collect::<Vec<&str>>()
                .try_into()
                .unwrap();
            let operation = match parts {
                ["old", "+", rhs] => Operation::Add(rhs.parse().unwrap()),
                ["old", "*", "old"] => Operation::Square,
                ["old", "*", rhs] => Operation::Mul(rhs.parse().unwrap()),
                _ => panic!("Invalid operation {parts:?}"),
            };

            let line = lines.next().unwrap().trim();
            assert!(line.starts_with("Test: divisible by "));
            let test = line[19..].parse().unwrap();

            let line = lines.next().unwrap().trim();
            assert!(line.starts_with("If true: throw to monkey "));
            let target_true = line[25..].parse().unwrap();

            let line = lines.next().unwrap().trim();
            assert!(line.starts_with("If false: throw to monkey "));
            let target_false = line[26..].parse().unwrap();

            assert!(lines.next().is_none());

            Monkey {
                items,
                operation,
                test,
                targets: [target_true, target_false],
            }
        })
        .collect();
}

fn do_round(monkeys: &mut Vec<Monkey>, counter: &mut [u64], therapy: &impl Fn(u64) -> u64) {
    let mut new_items: Vec<Vec<u64>> = (0..monkeys.len()).map(|_| Vec::new()).collect();
    for (i, monkey) in monkeys.iter_mut().enumerate() {
        new_items.push(Vec::new());
        let new = new_items.swap_remove(i);

        for item in monkey.items.iter().chain(new.iter()) {
            let item = therapy(monkey.operation.apply(*item));
            let test = item % monkey.test == 0;
            let target = monkey.targets[usize::from(!test)];
            new_items[target].push(item);
            counter[i] += 1;
        }
    }
    for (i, new) in new_items.into_iter().enumerate() {
        monkeys[i].items = new;
    }
}

fn monkey_business(monkeys: &mut Vec<Monkey>, rounds: usize, therapy: impl Fn(u64) -> u64) -> u64 {
    let mut counter = vec![0; monkeys.len()];
    for _ in 0..rounds {
        do_round(monkeys, &mut counter, &therapy);
    }
    counter.sort_unstable();
    counter.pop().unwrap() * counter.pop().unwrap()
}

pub fn part1(input: &str) -> u64 {
    let mut monkeys = parse_input(input);
    monkey_business(&mut monkeys, 20, |worry| worry / 3)
}

pub fn part2(input: &str) -> u64 {
    let mut monkeys = parse_input(input);
    let modulo = monkeys.iter().map(|m| m.test).reduce(|l, r| l * r).unwrap();
    monkey_business(&mut monkeys, 10_000, |worry| worry % modulo)
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &str = "
        Monkey 0:
          Starting items: 79, 98
          Operation: new = old * 19
          Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3

        Monkey 1:
          Starting items: 54, 65, 75, 74
          Operation: new = old + 6
          Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0

        Monkey 2:
          Starting items: 79, 60, 97
          Operation: new = old * old
          Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3

        Monkey 3:
          Starting items: 74
          Operation: new = old + 3
          Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT);
        let expected = vec![
            Monkey {
                items: vec![79, 98],
                operation: Operation::Mul(19),
                test: 23,
                targets: [2, 3],
            },
            Monkey {
                items: vec![54, 65, 75, 74],
                operation: Operation::Add(6),
                test: 19,
                targets: [2, 0],
            },
            Monkey {
                items: vec![79, 60, 97],
                operation: Operation::Square,
                test: 13,
                targets: [1, 3],
            },
            Monkey {
                items: vec![74],
                operation: Operation::Add(3),
                test: 17,
                targets: [0, 1],
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_do_round() {
        let mut monkeys = parse_input(EXAMPLE_INPUT);
        let mut counter = vec![0, 0, 0, 0];
        let therapy = |worry| worry / 3;

        do_round(&mut monkeys, &mut counter, &therapy);
        assert_eq!(monkeys[0].items, vec![20, 23, 27, 26]);
        assert_eq!(monkeys[1].items, vec![2080, 25, 167, 207, 401, 1046]);
        assert_eq!(monkeys[2].items, vec![]);
        assert_eq!(monkeys[3].items, vec![]);
        assert_eq!(counter, vec![2, 4, 3, 5]);

        do_round(&mut monkeys, &mut counter, &therapy);
        assert_eq!(monkeys[0].items, vec![695, 10, 71, 135, 350]);
        assert_eq!(monkeys[1].items, vec![43, 49, 58, 55, 362]);
        assert_eq!(monkeys[2].items, vec![]);
        assert_eq!(monkeys[3].items, vec![]);

        do_round(&mut monkeys, &mut counter, &therapy);
        assert_eq!(monkeys[0].items, vec![16, 18, 21, 20, 122]);
        assert_eq!(monkeys[1].items, vec![1468, 22, 150, 286, 739]);
        assert_eq!(monkeys[2].items, vec![]);
        assert_eq!(monkeys[3].items, vec![]);

        do_round(&mut monkeys, &mut counter, &therapy);
        assert_eq!(monkeys[0].items, vec![491, 9, 52, 97, 248, 34]);
        assert_eq!(monkeys[1].items, vec![39, 45, 43, 258]);
        assert_eq!(monkeys[2].items, vec![]);
        assert_eq!(monkeys[3].items, vec![]);

        do_round(&mut monkeys, &mut counter, &therapy);
        assert_eq!(monkeys[0].items, vec![15, 17, 16, 88, 1037]);
        assert_eq!(monkeys[1].items, vec![20, 110, 205, 524, 72]);
        assert_eq!(monkeys[2].items, vec![]);
        assert_eq!(monkeys[3].items, vec![]);

        do_round(&mut monkeys, &mut counter, &therapy);
        assert_eq!(monkeys[0].items, vec![8, 70, 176, 26, 34]);
        assert_eq!(monkeys[1].items, vec![481, 32, 36, 186, 2190]);
        assert_eq!(monkeys[2].items, vec![]);
        assert_eq!(monkeys[3].items, vec![]);

        do_round(&mut monkeys, &mut counter, &therapy);
        assert_eq!(monkeys[0].items, vec![162, 12, 14, 64, 732, 17]);
        assert_eq!(monkeys[1].items, vec![148, 372, 55, 72]);
        assert_eq!(monkeys[2].items, vec![]);
        assert_eq!(monkeys[3].items, vec![]);

        do_round(&mut monkeys, &mut counter, &therapy);
        assert_eq!(monkeys[0].items, vec![51, 126, 20, 26, 136]);
        assert_eq!(monkeys[1].items, vec![343, 26, 30, 1546, 36]);
        assert_eq!(monkeys[2].items, vec![]);
        assert_eq!(monkeys[3].items, vec![]);

        do_round(&mut monkeys, &mut counter, &therapy);
        assert_eq!(monkeys[0].items, vec![116, 10, 12, 517, 14]);
        assert_eq!(monkeys[1].items, vec![108, 267, 43, 55, 288]);
        assert_eq!(monkeys[2].items, vec![]);
        assert_eq!(monkeys[3].items, vec![]);

        do_round(&mut monkeys, &mut counter, &therapy);
        assert_eq!(monkeys[0].items, vec![91, 16, 20, 98]);
        assert_eq!(monkeys[1].items, vec![481, 245, 22, 26, 1092, 30]);
        assert_eq!(monkeys[2].items, vec![]);
        assert_eq!(monkeys[3].items, vec![]);

        for _ in 0..5 {
            do_round(&mut monkeys, &mut counter, &therapy);
        }
        assert_eq!(monkeys[0].items, vec![83, 44, 8, 184, 9, 20, 26, 102]);
        assert_eq!(monkeys[1].items, vec![110, 36]);
        assert_eq!(monkeys[2].items, vec![]);
        assert_eq!(monkeys[3].items, vec![]);

        for _ in 0..5 {
            do_round(&mut monkeys, &mut counter, &therapy);
        }
        assert_eq!(monkeys[0].items, vec![10, 12, 14, 26, 34]);
        assert_eq!(monkeys[1].items, vec![245, 93, 53, 199, 115]);
        assert_eq!(monkeys[2].items, vec![]);
        assert_eq!(monkeys[3].items, vec![]);
        assert_eq!(counter, vec![101, 95, 7, 105]);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 10_605);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 2_713_310_158);
    }
}
