use tentley::prelude::*;

#[test]
fn matrix_multiplication() {
    let m0 = mat![
        1, 2, 3;
        4, 5, 6
    ];

    let m1 = mat![
        1, 2;
        2, 3;
        0, -1
    ];

    let p = mat![
        5, 5;
        14, 17
    ];

    assert_eq!(p, m0 * m1);
}

#[test]
fn matrix_addition() {
    let m0 = mat![
        1, 2, 3;
        4, 5, 6
    ];

    let m1 = mat![
        -1, -2, -3;
        -4, -5, -6
    ];

    assert_eq!(Matrix::<i32, 2, 3>::zeros(), m0 + m1);
}

#[test]
fn transpose() {
    let m = mat![
        1, 2, 3;
        4, 5, 6
    ];

    assert_eq!(mat![1, 4; 2, 5; 3, 6], m.transpose());
}