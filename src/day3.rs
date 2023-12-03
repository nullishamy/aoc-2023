mod aoc;

/*
--- Day 3: Gear Ratios ---
You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you.
You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone!
The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one.
If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine.
There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum.
(Periods (.) do not count as a symbol.)
*/

#[derive(Debug)]
struct EnginePart {
    range: (usize, usize),
    line: usize,
    number: u64,
    is_gear_ratio: bool,
}

impl EnginePart {
    fn in_line<'a>(&self, line: &'a str) -> &'a str {
        let range = (self.range.0)..(self.range.1);
        &line[range]
    }
}

fn get_part_numbers(all_parts: Vec<EnginePart>, lines: Vec<&str>) -> Vec<EnginePart> {
    // An empty line just has no values, but the length should be consistent with the input
    let empty_line = ".".repeat(lines[0].len());
    let mut valid_parts = vec![];
    for part in all_parts {
        let line = lines.get(part.line).unwrap();

        let line_above = {
            if part.line == 0 {
                empty_line.as_str()
            } else {
                lines.get(part.line - 1).unwrap()
            }
        };

        let line_below = {
            if part.line == lines.len() - 1 {
                empty_line.as_str()
            } else {
                lines.get(part.line + 1).unwrap()
            }
        };

        // Check for adjacent symbols now

        // Start with symbols directly above:
        let has_upper_adjacents = {
            let substr_in_upper = part.in_line(line_above);
            str_contains_symbol(substr_in_upper)
        };

        // Symbols directly below:
        let has_lower_adjacents = {
            let substr_in_lower = part.in_line(line_below);
            str_contains_symbol(substr_in_lower)
        };

        // Symbols on the corners
        let has_corner_adjacents = {
            // Use first char if it would underflow
            let top_left = line_above.get(part.range.0.saturating_sub(1)..(part.range.0));
            let bottom_left = line_below.get(part.range.0.saturating_sub(1)..(part.range.0));

            let top_right = line_above.get((part.range.1)..(part.range.1 + 1));
            let bottom_right = line_below.get((part.range.1)..(part.range.1 + 1));

            [top_left, bottom_left, top_right, bottom_right]
                .iter()
                .any(|st| {
                    if let Some(ch) = st.and_then(|st| st.chars().next()) {
                        char_is_symbol(ch)
                    } else {
                        false
                    }
                })
        };

        // Symbols on each side of the part
        let has_side_adjacents = {
            let left = line.get(part.range.0.saturating_sub(1)..(part.range.0));
            let right = line.get((part.range.1)..(part.range.1 + 1));

            let left_has = if let Some(ch) = left.and_then(|st| st.chars().next()) {
                char_is_symbol(ch)
            } else {
                false
            };

            let right_has = if let Some(ch) = right.and_then(|st| st.chars().next()) {
                char_is_symbol(ch)
            } else {
                false
            };

            left_has || right_has
        };

        if has_upper_adjacents || has_lower_adjacents || has_corner_adjacents || has_side_adjacents
        {
            valid_parts.push(part);
        }
    }
    valid_parts
}

fn char_is_symbol(ch: char) -> bool {
    // (Periods (.) do not count as a symbol.)
    ch.is_ascii_punctuation() && ch != '.'
}

fn str_contains_symbol(line: &str) -> bool {
    line.chars().any(char_is_symbol)
}

fn parse_parts_for_line(line: &str, line_idx: usize) -> Vec<EnginePart> {
    let mut parts = vec![];
    // Find contiguous number sets
    let mut number_str = String::new();
    let mut number_start = 0;

    for (ch_idx, ch) in line.char_indices() {
        // We have just started building a number, setup the start range
        if ch.is_ascii_digit() && number_str.is_empty() {
            number_start = ch_idx;
        }

        // We are building a number
        if ch.is_ascii_digit() {
            number_str.push(ch);
        }

        // We hit EOL. If this character is a number, interpret as as a part
        if ch_idx == line.len() - 1 && ch.is_ascii_digit() {
            parts.push(EnginePart {
                range: (number_start, ch_idx),
                number: number_str.parse().unwrap(),
                line: line_idx,
                is_gear_ratio: false,
            });

            // Break out so we don't try and parse garbage below
            break;
        }

        // We hit a non ascii digit, and have collected a number
        if !ch.is_ascii_digit() && !number_str.is_empty() {
            parts.push(EnginePart {
                range: (number_start, ch_idx),
                number: number_str.parse().unwrap(),
                line: line_idx,
                is_gear_ratio: false,
            });

            number_str.clear();
        }
    }

    parts
}

#[test]
fn part_1() {
    let input = input!(
        ~ "day3.input",
        r#"467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."#
    );

    let lines = input.lines().map(|l| l.trim()).collect::<Vec<_>>();

    let mut parts = vec![];
    for (line_idx, line) in lines.iter().enumerate() {
        parts.extend(parse_parts_for_line(line, line_idx));
    }

    let valid_parts = get_part_numbers(parts, lines);
    let sum = valid_parts.iter().map(|p| p.number).sum::<u64>();

    assert_eq!(sum, 537832);
}

/*
--- Part Two ---
The engineer finds the missing part and installs it in the engine!
As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.

You don't seem to be going very fast, though. Maybe something is still wrong?
Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.

Before you can explain the situation, she suggests that you look out the window.
There stands the engineer, holding a phone in one hand and waving with the other.
You're going so slowly that you haven't even left the station. You exit the gondola.

The missing part wasn't the only issue - one of the gears in the engine is wrong.
A gear is any * symbol that is adjacent to exactly two part numbers.
Its gear ratio is the result of multiplying those two numbers together.

This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.
*/
#[test]
fn part_2() {
    let input = input!(
        ~ "day3.input",
        r#"467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."#
    );

    let lines = input.lines().map(|l| l.trim()).collect::<Vec<_>>();

    let mut parts = vec![];
    for (line_idx, line) in lines.iter().enumerate() {
        parts.extend(parse_parts_for_line(line, line_idx));
    }

    let valid_parts = get_part_numbers(parts, lines);
    panic!()
}
