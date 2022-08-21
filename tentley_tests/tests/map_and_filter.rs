use tentley::prelude::*;

#[test]
fn map() {
    let m = mat![1, 2; 3, 4];

    let m2 = m.map(|e| e * e);

    assert_eq!(mat![1, 4; 9, 16], m2);
}

#[test]
fn filter() {
    let m = mat![1, 2; 3, 4];

    let f = m.filter(|x| x % 2 == 0);

    assert_eq!(mat![Option<i32>;
        None, Some(2);
        None, Some(4)    
    ], f);
}