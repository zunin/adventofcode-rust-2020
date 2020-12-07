use std::ops::Range;
use itertools::Itertools;

#[cfg(test)]
mod tests {
    use super::*;
    use shared::*;

    #[test]
    fn example_data_one() {
        assert_eq!(get_group_yes_questions_count(vec![
            "abc",
            "",
            "a",
            "b",
            "c",
            "",
            "ab",
            "ac",
            "",
            "a",
            "a",
            "a",
            "a",
            "",
            "b"
        ].into_iter().map(String::from).collect_vec().as_slice()), 11);
    }

    #[test]
    fn part_one() {
        assert_eq!(get_group_yes_questions_count(get_input_txt().as_slice()), 7283);
    }

    #[test]
    fn example_data_two() {
        assert_eq!(get_group_common_yes_questions_count(vec![
            "abc",
            "",
            "a",
            "b",
            "c",
            "",
            "ab",
            "ac",
            "",
            "a",
            "a",
            "a",
            "a",
            "",
            "b"
        ].into_iter().map(String::from).collect_vec().as_slice()), 6);
    }

    #[test]
    fn part_two() {
        assert_eq!(get_group_common_yes_questions_count(get_input_txt().as_slice()), 3520);
    }
}

pub fn get_group_yes_questions_count(input: &[String]) -> usize {
    let groups = parse_groups(input);

    groups.into_iter().map(|group| group.get_unique_answers().len()).sum()
}

pub fn get_group_common_yes_questions_count(input: &[String]) -> usize {
    let groups = parse_groups(input);

    groups.into_iter().map(|group| group.get_common_answers().len()).sum()
}

pub fn parse_groups(input: &[String]) -> Vec<Group> {
    input.split(String::is_empty)
        .map(|group_string| Group { answers: group_string.to_vec() })
        .collect_vec()
}

pub struct Group {
    answers: Vec<String>
}

impl Group {
    fn get_answers(&self) -> Vec<Vec<char>> {
        self.answers.iter()
            .map(|s| s.chars().collect_vec())
            .collect_vec()
    }

    fn get_unique_answers(&self) -> Vec<char> {
        self.get_answers().into_iter().flatten().unique().collect_vec()
    }

    fn get_common_answers(&self) -> Vec<char> {
        let common_answers = self.get_answers().into_iter()
            .flat_map(|person_answers| {
                person_answers.into_iter()
                    .filter(|answer| {
                        self.get_answers()
                            .into_iter()
                            .all(|line_answer| line_answer.contains(answer))
                    })
            })
            .unique()
            .collect_vec();

        common_answers
    }
}
