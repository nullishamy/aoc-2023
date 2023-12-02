/*
--- Day 1: Trebuchet?! ---
Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from")
when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").

As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills.
Consequently, the Elves are having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover.
On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.
*/

#[test]
fn part_1() {
    let input = include_str!("day1.input");

    let sum = input
        .lines()
        .map(|line| {
            let numbers_in_line = line
                .chars()
                .map(|c| c as u8)
                .filter(|c| *c >= b'0' && *c <= b'9')
                .map(|c| c as char)
                .collect::<Vec<_>>();

            let first = *numbers_in_line.first().unwrap();
            let last = *numbers_in_line.last().unwrap();

            format!("{}{}", first, last).parse::<u64>().unwrap()
        })
        .sum::<u64>();

    assert_eq!(sum, 54390);
}

/*
Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters:
    one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

Equipped with this new information, you now need to find the real first and last digit on each line.
*/

#[derive(Debug)]
struct Found {
    at: usize,
    as_num: String,
}

#[test]
fn part_2() {
    let input = include_str!("./day1.input");
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut sum = 0;

    for line in input.lines() {
        let literal_numbers = {
            let mut result = vec![];
            for (at, ch) in line.char_indices() {
                if ch as u8 >= b'0' && ch as u8 <= b'9' {
                    result.push(Found {
                        at,
                        as_num: ch.to_string(),
                    })
                }
            }
            result
        };

        let word_numbers = {
            let mut result = vec![];
            for (key, word) in words.iter().enumerate() {
                for (at, _) in line.match_indices(word) {
                    result.push(Found {
                        at,
                        as_num: (key + 1).to_string(),
                    });
                }
            }
            result
        };

        let mut all_numbers = literal_numbers
            .iter()
            .chain(word_numbers.iter())
            .collect::<Vec<_>>();

        all_numbers.sort_by(|a, b| a.at.cmp(&b.at));

        let first = all_numbers.first().unwrap();
        let last = all_numbers.last().unwrap();

        let numeric = format!("{}{}", first.as_num, last.as_num)
            .parse::<u32>()
            .unwrap();

        sum += numeric;
    }

    assert_eq!(sum, 54277);
}
