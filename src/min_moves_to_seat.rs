// https://leetcode.com/problems/minimum-number-of-moves-to-seat-everyone/

use std::iter::zip;

pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
    let mut result = 0;

    // To minimize the number of moves, a student needs to the closest seat:
    // Pre-sort both vectors and take abs values of their corresponding difference
    seats.sort();
    students.sort();

    students.iter().zip(seats).for_each(|(student, seat)| {
        result += (student- seat).abs();
    });

    result
}

#[test]
fn test_min_moves_to_seat() {
    let expected = 4;
    let input_seats = [3, 1, 5].into();
    let input_students = [2, 7, 4].into();
    let output = min_moves_to_seat(input_seats, input_students);
    assert_eq!(expected, output);
}
