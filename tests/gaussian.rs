use tentley::prelude::*;

#[test]
fn row_operations() -> Result<(), TentleyError> {
    let mut m = mat![
        1, 2;
        3, 4
    ];

    m.swap_rows(0, 1)?;

    assert_eq!(mat![
        3, 4;
        1, 2
    ], m);

    m.multiply_row(0, 2)?;

    assert_eq!(mat![
        6, 8;
        1, 2
    ], m);

    m.add_rows(1, 0, 2)?;

    assert_eq!(mat![
        8, 12;
        1, 2
    ], m);

    Ok(())
}