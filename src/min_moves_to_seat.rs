// https://leetcode.com/problems/minimum-number-of-moves-to-seat-everyone/

pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
    -1
}

#[test]
fn test_min_moves_to_seat() {
    let expected = 4;
    let input_seats = [3, 1, 5].into();
    let input_students = [2, 7, 4].into();
    let output = min_moves_to_seat(input_seats, input_students);
    assert_eq!(expected, output);
}
