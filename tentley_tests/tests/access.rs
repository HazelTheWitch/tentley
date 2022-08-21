use tentley::prelude::*;

#[test]
fn get() {
    let m = mat![
        1, 2, 3;
        4, 5, 6;
        7, 8, 9
    ];

    assert_eq!(Some(&5), m.get(1, 1));
    assert_eq!(None, m.get(10, 10));
    assert_eq!(&9, unsafe { m.get_unchecked(2, 2) });
}

#[test]
fn get_mut() {
    let mut m = mat![
        1, 2, 3;
        4, 5, 6;
        7, 8, 9
    ];

    assert_eq!(Some(&5), m.get(1, 1));

    *m.get_mut(1, 1).unwrap() = -5;

    assert_eq!(Some(&-5), m.get(1, 1));

    unsafe {
        *m.get_unchecked_mut(1, 1) = 10;
    }

    assert_eq!(Some(&10), m.get(1, 1));
}

#[test]
fn get_row() {
    let mut m = mat![
        1, 2, 3;
        4, 5, 6;
        7, 8, 9
    ];

    assert_eq!(Some(vec![&1, &2, &3]), m.get_row(0));

    for e in m.get_row_mut(0).unwrap() {
        *e = *e * 2;
    }

    assert_eq!(Some(vec![&2, &4, &6]), m.get_row(0));
}

#[test]
fn get_col() {
    let mut m = mat![
        1, 2, 3;
        4, 5, 6;
        7, 8, 9
    ];

    assert_eq!(Some(vec![&1, &4, &7]), m.get_col(0));

    for e in m.get_col_mut(0).unwrap() {
        *e = *e * 2;
    }

    assert_eq!(Some(vec![&2, &8, &14]), m.get_col(0));
}
