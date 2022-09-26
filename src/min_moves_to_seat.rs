// https://leetcode.com/problems/minimum-number-of-moves-to-seat-everyone/

pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
    let mut result = 0;

    // To minimize the number of moves, a student needs to the closest seat:
    // Pre-sort both vectors and take abs values of their corresponding difference
    seats.sort();
    students.sort();

    for i in 0..seats.len() {
        result += (students[i] - seats[i]).abs();
    }

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
