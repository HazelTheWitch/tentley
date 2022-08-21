use tentley::prelude::*;

#[test]
fn new() {
    let m = Matrix::new([[1, 2], [3, 4], [5, 6]]);

    assert_eq!((3, 2), m.shape());

    for row in 0..3 {
        for col in 0..2 {
            assert_eq!(Some(&(col + row * 2 + 1)), m.get(row, col));
        }
    }

    assert_eq!(None, m.get(0, 10));
    assert_eq!(None, m.get(10, 0));
    assert_eq!(None, m.get(10, 10));
}

#[test]
fn full() {
    let m = Matrix::<_, 2, 1>::full(1.5);

    assert_eq!((2, 1), m.shape());

    for i in 0..2 {
        assert_eq!(Some(&1.5), m.get(i, 0));
    }
}

#[test]
fn ones() {
    let m = Matrix::<i32, 2, 1>::ones();

    assert_eq!((2, 1), m.shape());

    for i in 0..2 {
        assert_eq!(Some(&1), m.get(i, 0));
    }
}

#[test]
fn zeros() {
    let m = Matrix::<i32, 2, 1>::zeros();

    assert_eq!((2, 1), m.shape());

    for i in 0..2 {
        assert_eq!(Some(&0), m.get(i, 0));
    }
}

#[test]
fn from_fn() {
    let m = Matrix::<_, 4, 4>::from_fn(|r, c| r as i32 + c as i32);

    assert_eq!((4, 4), m.shape());

    for r in 0..4 {
        for c in 0..4 {
            assert_eq!(Some(&(r as i32 + c as i32)), m.get(r, c));
        }
    }
}

#[test]
fn identity() {
    let m = Matrix::<_, 4, 4>::identity();

    assert_eq!((4, 4), m.shape());

    for r in 0..4 {
        for c in 0..4 {
            assert_eq!(Some(&{ if r == c { 1 } else { 0 }}), m.get(r, c));
        }
    }
}