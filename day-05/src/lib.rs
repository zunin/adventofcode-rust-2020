use std::ops::Range;
use itertools::Itertools;

#[cfg(test)]
mod tests {
    use super::*;
    use shared::*;
    #[test]
    fn example_data_one() {
        assert_eq!(get_seat_id(&"FBFBBFFRLR".to_string()), 357);
    }

    #[test]
    fn example_data_two() {
        assert_eq!(get_seat_id(&"BFFFBBFRRR".to_string()), 567);
    }

    #[test]
    fn example_data_three() {
        assert_eq!(get_seat_id(&"FFFBBBFRRR".to_string()), 119);
    }

    #[test]
    fn example_data_four() {
        assert_eq!(get_seat_id(&"BBFFBBFRLL".to_string()), 820);
    }

    #[test]
    fn part_one() {
        assert_eq!(get_highest_seat_id(get_input_txt().as_slice()), 818);
    }

    #[test]
    fn part_two() {
        assert_eq!(get_my_seat_id(get_input_txt().as_slice()), 559);
    }
}

#[derive(Debug)]
pub struct SeatRange {
    row: Range<i32>,
    column: Range<i32>
}

#[derive(Debug)]
pub enum PartitionToken {
    Front,
    Back,
    Left,
    Right
}

pub fn tokenize(seat: &String) -> Vec<PartitionToken> {
    seat.chars().map(|c| {
        match c {
            'F' => Some(PartitionToken::Front),
            'B' => Some(PartitionToken::Back),
            'L' => Some(PartitionToken::Left),
            'R' => Some(PartitionToken::Right),
            _ => None
        }.expect("Unknown character {:?} found.")
    }).collect::<Vec<PartitionToken>>()
}

pub fn get_seat_id(input: &String) -> i32 {
    let seat = tokenize(input).into_iter()
        .fold(SeatRange { row: 0..128, column: 0..8 }, |seat, token| {
            match token {
                PartitionToken::Right => SeatRange {
                    row: seat.row,
                    column: (seat.column.start + ((seat.column.end - seat.column.start) / 2))..seat.column.end
                },
                PartitionToken::Left => SeatRange {
                    row: seat.row,
                    column: seat.column.start..(seat.column.end - ((seat.column.end - seat.column.start) / 2))
                },
                PartitionToken::Front => SeatRange {
                    row: seat.row.start..(seat.row.end - ((seat.row.end-seat.row.start) / 2)),
                    column: seat.column
                },
                PartitionToken::Back => SeatRange {
                    row: seat.row.start + ((seat.row.end-seat.row.start) / 2)..seat.row.end,
                    column: seat.column
                },
            }
    });

    (seat.row.start * 8) + seat.column.start
}

pub fn get_highest_seat_id(input :&[String]) -> i32 {
    input.iter().map(get_seat_id).max().unwrap()
}

pub fn get_my_seat_id(input :&[String]) -> i32 {
    let seat_ids = input.iter().map(get_seat_id)
        .sorted()
        .collect_vec();

    println!("{:?}", seat_ids);

    for seat_id in seat_ids.as_slice() {
        let next_seat = seat_id + 1;
        let upper_neighbour = seat_id + 2;
        if seat_ids.contains(&upper_neighbour) && !seat_ids.contains(&next_seat) {
            return *seat_id + 1
        }
    }

    None.expect("No candidates found")
}