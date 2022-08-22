use tentley::prelude::*;

fn main() -> Result<(), TentleyError> {
    let m = mat![f64;
        1, 2, 3;
        4, 5, 6;
        7, 8, 9
    ];

    let mut a = AugmentedMatrix::new(Matrix::identity(), m);

    a.swap_rows(0, 1)?;

    let (p, m_prime) = a.extract();

    println!("{:?}\n{:?}\n{:?}", p * m, p, m_prime);

    Ok(())
}