use tentley::prelude::*;

#[test]
fn matrix_macro() {
    let m = mat![0, 1; 2, 3];

    assert_eq!(Matrix::new([[0, 1], [2, 3]]), m);
}

#[test]
fn vector_macro() {
    let v = vector![1, 2, 3];

    assert_eq!(Matrix::new([[1], [2], [3]]), v);

    let v = row_vector![1, 2, 3];

    assert_eq!(Matrix::new([[1, 2, 3]]), v);
}

#[test]
fn typed_macros() {
    let m = mat![f64;
        1, 2, 3;
        4, 5, 6;
        7, 8, 9
    ];

    let v = vector![f64; 1, 0.5, 0.25];

    assert_eq!(vector![1.0, 0.5, 0.25], v);
    assert_eq!(mat![
        1.0, 2.0, 3.0;
        4.0, 5.0, 6.0;
        7.0, 8.0, 9.0
    ], m);
}