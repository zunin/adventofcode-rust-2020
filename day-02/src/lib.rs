#[cfg(test)]
mod tests {
    use super::*;
    use shared::get_input_txt;
    #[test]
    fn example_data_part_one() {
        assert_eq!(get_number_of_valid_password(vec![
            "1-3 a: abcde",
            "1-3 b: cdefg",
            "2-9 c: ccccccccc"
        ].into_iter().map(String::from).collect::<Vec<_>>()), 2);
    }

    #[test]
    fn part_one() {
        assert_eq!(get_number_of_valid_password(get_input_txt()), 638);
    }

    #[test]
    fn example_data_part_two() {
        assert_eq!(get_number_of_valid_password_by_position(vec![
            "1-3 a: abcde",
            "1-3 b: cdefg",
            "2-9 c: ccccccccc"
        ].into_iter().map(String::from).collect::<Vec<_>>()), 1);
    }
    #[test]
    fn part_two() {
        assert_eq!(get_number_of_valid_password_by_position(get_input_txt()), 699);
    }

    #[test]
    fn part_two_hint() {
        // 151 too low
        assert!(get_number_of_valid_password_by_position(get_input_txt()) > 151);
    }

}
use itertools::Itertools;

#[derive(Debug)]
pub struct PasswordInputToken {
    must_contain: char,
    at_least: usize,
    at_most: usize,
    password: String
}

pub fn get_number_of_valid_password<T>(input: T) -> usize
    where T : IntoIterator<Item = String>
{
    tokenize(input).into_iter()
        .filter(is_password_valid)
        .count()
}

pub fn get_number_of_valid_password_by_position<T>(input: T) -> usize
    where T : IntoIterator<Item = String>
{
    tokenize(input).into_iter()
        .filter(is_password_valid_by_position)
        .count()
}

pub fn is_password_valid_by_position(token: &PasswordInputToken) -> bool {
    let chars = token.password.chars().into_iter().collect::<Vec<_>>();

    (chars[token.at_least-1] == token.must_contain || chars[token.at_most-1] == token.must_contain)
        && !(chars[token.at_least-1] == token.must_contain && chars[token.at_most-1] == token.must_contain)
}


pub fn is_password_valid(token: &PasswordInputToken) -> bool {
    let chars_in_password = token.password.chars()
        .filter(|c| c == &token.must_contain)
        .count();

    let is_at_most_ok = token.at_most >= chars_in_password;
    let is_at_least_ok = chars_in_password >= token.at_least;

    is_at_most_ok && is_at_least_ok
}

pub fn tokenize<T>(input: T) -> Vec<PasswordInputToken>
    where T : IntoIterator<Item = String>
{
    input.into_iter()
        .map(|line: String| {
            let policy_split = line.split(':').collect::<Vec<_>>();
            let password = policy_split[1].trim().to_string();
            let policy = policy_split[0];
            let range_split = policy.split_whitespace().collect::<Vec<_>>();
            let range = range_split[0];
            let must_contain = range_split[1].parse::<char>().unwrap();
            let max_min_split = range.split('-').collect::<Vec<_>>();
            let at_least = max_min_split[0].parse::<usize>().unwrap();
            let at_most = max_min_split[1].parse::<usize>().unwrap();

            PasswordInputToken {
                must_contain,
                at_least,
                at_most,
                password
            }
        }).collect::<Vec<_>>()
}