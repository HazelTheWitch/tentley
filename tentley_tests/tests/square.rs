use tentley::prelude::*;
use tentley_tests::Error;

#[test]
fn diagonal() {
    let mut m = mat![
        1, 2;
        3, 4
    ];

    assert_eq!(vec![&1, &4], m.diagonal());

    let _ = m.diagonal_mut().into_iter().map(|e| { *e = *e * 2; e }).collect::<Vec<&mut i32>>();

    assert_eq!(vec![&2, &8], m.diagonal());
}

#[test]
fn determinant() -> Error {
    let m = mat![
        1, 2;
        3, 4
    ];

    assert_eq!(-2, m.determinant()?);

    Ok(())
}