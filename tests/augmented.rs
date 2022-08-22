use tentley::prelude::*;

#[test]
fn augmented() -> Result<(), TentleyError> {
    let mut a = AugmentedMatrix::<f32, 2, 2, 2>::new(mat![1f32, 2f32; 3f32, 4f32], mat![1f32, 0f32; 0f32, 1f32]);

    a.add_rows(0, 1, -3.0)?;
    a.add_rows(1, 0, 1.0)?;
    a.multiply_row(1, -1.0 / 2.0)?;

    let (left, right) = a.extract();

    assert_eq!(Matrix::identity(), left);
    assert_eq!(mat![
        -2.0, 1.0;
        3.0 / 2.0, -1.0 / 2.0
    ], right);

    Ok(())
}