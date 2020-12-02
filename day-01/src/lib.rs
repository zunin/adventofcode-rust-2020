#[cfg(test)]
mod tests {
    use shared::get_input_txt;
    use super::*;
    #[test]
    fn example_data_part_one() {
        assert_eq!(
            solve_part_one(
                vec![
                    "1721",
                    "979",
                    "366",
                    "299",
                    "675",
                    "1456"
                ].into_iter().map(String::from).collect::<Vec<_>>()
            ),
            514_579
        );
    }

    #[test]
    fn part_one() {
        assert_eq!(
            solve_part_one(get_input_txt()),
            658_899
        );
    }

    #[test]
    fn example_data_part_two() {
        assert_eq!(
            solve_part_two(
               vec![
                    "1721",
                    "979",
                    "366",
                    "299",
                    "675",
                    "1456"
                ].into_iter().map(String::from).collect::<Vec<_>>()
            ),
            241_861_950
        );
    }

    #[test]
    fn part_two() {
        assert_eq!(
            solve_part_two(get_input_txt()),
            155_806_250
        );
    }
}
use itertools::Itertools;


pub fn solve_part_one<T>(input: T) -> i32
where T : IntoIterator<Item = String>
{
    let numbers = input.into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .sorted()
        .collect::<Vec<i32>>();

    for (outer_index, outer_number) in numbers.iter().enumerate() {
        for inner_number in &numbers[outer_index..] {
            match outer_number + inner_number {
                sum if sum == 2020 => return outer_number * inner_number,
                sum if sum > 2020 => break,
                _ => {}
            }
        }
    }
    0
}

pub fn solve_part_two<T>(input: T) -> i32
where T : IntoIterator<Item = String>
{
    let numbers = input.into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .sorted()
        .collect::<Vec<i32>>();

    for (outer_index, outer_number) in numbers.iter().enumerate() {
        for (mid_index, mid_number) in numbers[outer_index..].iter().enumerate() {
            for inner_number in numbers[mid_index..].iter() {
                match outer_number + mid_number + inner_number {
                    sum if sum == 2020 => return outer_number * mid_number * inner_number,
                    sum if sum > 2020 => break,
                    _ => {}
                }
            }
            
        }
    }

    0
}
