use leetcode::solution::two_sum::two_sum;

#[test]
fn two_sum_test() {
    let input1 = vec![3, 3];
    assert_eq!(vec![0, 1], two_sum(input1, 6));

    let input2 = vec![3, 2, 4];
    assert_eq!(vec![1, 2], two_sum(input2, 6));

    let input3 = vec![2, 7, 11, 15];
    assert_eq!(vec![0, 1], two_sum(input3, 9));
}
