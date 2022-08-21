#[cfg(test)]
mod tests {
    use tentley::prelude::*;

    #[test]
    fn access() {
        let m = mat![1, 2; 3, 4];

        assert_eq!(m.get(0, 0), Some(&1));
        assert_eq!(m.get(1, 0), Some(&3));
        assert_eq!(m.get(0, 1), Some(&2));
        assert_eq!(m.get(1, 1), Some(&4));

        assert_eq!(m.get_row(0), Some(vec![&1, &2]));
        assert_eq!(m.get_row(1), Some(vec![&3, &4]));

        assert_eq!(m.get_col(0), Some(vec![&1, &3]));
        assert_eq!(m.get_col(1), Some(vec![&2, &4]));
    }

    #[test]
    fn transpose() {
        let m = mat![
            1, 2, 3;
            4, 5, 6
        ];

        let m_t = mat![
            1, 4;
            2, 5;
            3, 6
        ];

        assert_eq!(m_t, m.transpose());
    }

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
    fn addition() {
        let m0 = mat![
            1, 2;
            3, 4
        ];

        let m1 = mat![
            -1, -2;
            -3, -4
        ];

        let zeros = mat![
            0, 0;
            0, 0
        ];

        assert_eq!(zeros, m0 + m1);
    }

    #[test]
    fn identity() {
        let i = SquareMatrix::<f32, 3>::identity();

        assert_eq!(mat![
            1.0, 0.0, 0.0;
            0.0, 1.0, 0.0;
            0.0, 0.0, 1.0
        ], i);
    }

    #[test]
    fn lu_decomposition() {
        let m = mat![
            1.0, 2.0;
            5.0, 3.0
        ];

        let (l, u) = m.lu_decomposition().unwrap();

        assert_eq!(m, l * u);
    }
}