use std::ops::Index;

#[cfg(test)]
mod tests {
    use super::*;
    use shared::get_input_txt;

    #[test]
    fn example_data_part_one() {
        assert_eq!(
            count_tree_in_toggoban_path(vec![
                "..##.......",
                "#...#...#..",
                ".#....#..#.",
                "..#.#...#.#",
                ".#...##..#.",
                "..#.##.....",
                ".#.#.#....#",
                ".#........#",
                "#.##...#...",
                "#...##....#",
                ".#..#...#.#"
            ].into_iter().map(String::from).collect::<Vec<_>>(), 1, 3),
        7);
    }

    #[test]
    fn part_one() {
        assert_eq!(
            count_tree_in_toggoban_path(get_input_txt(), 1, 3),
            169);
    }

    #[test]
    fn example_data_part_two_partial_1() {
        assert_eq!(
            count_tree_in_toggoban_path(vec![
                "..##.......",
                "#...#...#..",
                ".#....#..#.",
                "..#.#...#.#",
                ".#...##..#.",
                "..#.##.....",
                ".#.#.#....#",
                ".#........#",
                "#.##...#...",
                "#...##....#",
                ".#..#...#.#"
            ].into_iter().map(String::from).collect::<Vec<_>>(), 1, 1),
            2);
    }

    #[test]
    fn example_data_part_two_partial_2() {
        assert_eq!(
            count_tree_in_toggoban_path(vec![
                "..##.......",
                "#...#...#..",
                ".#....#..#.",
                "..#.#...#.#",
                ".#...##..#.",
                "..#.##.....",
                ".#.#.#....#",
                ".#........#",
                "#.##...#...",
                "#...##....#",
                ".#..#...#.#"
            ].into_iter().map(String::from).collect::<Vec<_>>(), 1, 5),
            3);
    }

    #[test]
    fn example_data_part_two_partial_3() {
        assert_eq!(
            count_tree_in_toggoban_path(vec![
                "..##.......",
                "#...#...#..",
                ".#....#..#.",
                "..#.#...#.#",
                ".#...##..#.",
                "..#.##.....",
                ".#.#.#....#",
                ".#........#",
                "#.##...#...",
                "#...##....#",
                ".#..#...#.#"
            ].into_iter().map(String::from).collect::<Vec<_>>(), 1, 7),
            4);
    }

    #[test]
    fn example_data_part_two_partial_4() {
        assert_eq!(
            count_tree_in_toggoban_path(vec![
                "..##.......",
                "#...#...#..",
                ".#....#..#.",
                "..#.#...#.#",
                ".#...##..#.",
                "..#.##.....",
                ".#.#.#....#",
                ".#........#",
                "#.##...#...",
                "#...##....#",
                ".#..#...#.#"
            ].into_iter().map(String::from).collect::<Vec<_>>(), 2, 1),
            2);
    }

    #[test]
    fn example_data_part_two() {
        assert_eq!(
            get_multiplied_number_of_trees_in_slopes(vec![
                "..##.......",
                "#...#...#..",
                ".#....#..#.",
                "..#.#...#.#",
                ".#...##..#.",
                "..#.##.....",
                ".#.#.#....#",
                ".#........#",
                "#.##...#...",
                "#...##....#",
                ".#..#...#.#"
            ].into_iter().map(String::from).collect::<Vec<_>>()),
            336);
    }

    #[test]
    fn part_two() {
        assert_eq!(
            get_multiplied_number_of_trees_in_slopes(get_input_txt()), 7_560_370_818);
    }
}

pub fn get_multiplied_number_of_trees_in_slopes<T>(input: T) -> usize
    where T : IntoIterator<Item = String>
{
    let string_box = input.into_iter().collect::<Vec<String>>().into_boxed_slice();

    count_tree_in_toggoban_path(string_box.clone().into_vec(), 1, 1) *
    count_tree_in_toggoban_path(string_box.clone().into_vec(), 1, 3) *
    count_tree_in_toggoban_path(string_box.clone().into_vec(), 1, 5) *
    count_tree_in_toggoban_path(string_box.clone().into_vec(), 1, 7) *
    count_tree_in_toggoban_path(string_box.into_vec(), 2, 1)

}


pub fn count_tree_in_toggoban_path<T>(input: T, line_incr: usize, row_incr: usize) -> usize
    where T : IntoIterator<Item = String>
{
    let map = parse_map(input);

    let mut tree_count: usize = 0;
    let mut row_index: usize = 0;
    let mut line_index: usize = 0;

    while line_index < map.lines.len() {
        if let MapSquare::Tree = map[line_index][row_index] {
            tree_count += 1;
        }

        row_index += row_incr;
        line_index += line_incr;
    }

    tree_count
}

#[derive(Debug)]
pub enum MapSquare {
    Tree,
    Open
}

#[derive(Debug)]
pub struct Map {
    lines: Vec<MapLine>
}

#[derive(Debug)]
pub struct MapLine {
    rows: Vec<MapSquare>
}

impl Index<usize> for Map {
    type Output = MapLine;

    fn index(&self, index: usize) -> &Self::Output {
        &self.lines[index]
    }
}

impl Index<usize> for MapLine {
    type Output = MapSquare;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index % self.rows.len()]
    }
}



pub fn parse_map<T>(input: T) -> Map
    where T : IntoIterator<Item = String>
{
    Map {
        lines : input.into_iter().map(|line: String| {
            MapLine {
                rows: line.chars().map(|token| {
                    match token {
                        '.' => MapSquare::Open,
                        _ => MapSquare::Tree
                    }
                }).collect::<Vec<MapSquare>>()
            }
        }).collect::<Vec<MapLine>>()
    }
}
