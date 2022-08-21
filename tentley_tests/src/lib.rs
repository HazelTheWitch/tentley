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

        assert_eq!(
            mat![
                1.0, 0.0, 0.0;
                0.0, 1.0, 0.0;
                0.0, 0.0, 1.0
            ],
            i
        );
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

    #[test]
    fn vector() {
        let col_v = vector![1, 2, 3];
        let row_v = row_vector![1, 2, 3];

        assert_eq!((3, 1), col_v.shape());
        assert_eq!((1, 3), row_v.shape());
    }

    #[test]
    fn determinant() {
        let m = mat![
            1, 2;
            3, 4
        ];

        assert_eq!(-2, m.determinant().unwrap());
    }

    #[test]
    fn map() {
        let m = mat![
            1, 2;
            3, 4
        ];

        let m2 = m.map(|x| x * x);

        assert_eq!(mat![1, 4; 9, 16], m2);
    }

    #[test]
    fn filter() {
        let m = mat![
            1, 2;
            3, 4
        ];

        let filtered = m.filter(|e| e % 2 == 0);

        assert_eq!(mat![None, Some(2); None, Some(4)], filtered);
    }
}
