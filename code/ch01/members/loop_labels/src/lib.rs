pub fn silly_sub(a: u32, b: u32) -> i64 {
    if a < b {
        return 0; // Если a меньше b, результат будет 0
    }
    let mut result: i64 = 0;
    'increment: loop {
        if result == a as i64 {
            let mut dec = b;
            loop {
                if dec == 0 {
                    // breaks directly out of 'increment loop
                    break 'increment;
                } else {
                    result -= 1;
                    dec -= 1;
                }
            }
        } else {
            result += 1;
        }
    }
    result
}

#[test]
fn tests() {
    assert_eq!(silly_sub(5, 2), 3);
    assert_eq!(silly_sub(10, 5), 5);
    assert_eq!(silly_sub(0, 0), 0);
    assert_eq!(silly_sub(100, 7), 93);
    assert_eq!(silly_sub(20, 20), 0);
    assert_eq!(silly_sub(15, 8), 7);

    assert_eq!(silly_sub(50, 50), 0);
    assert_eq!(silly_sub(12, 3), 9);
    // assert_eq!(silly_sub(u32::MAX, u32::MAX), u32::MAX as i64); // Еще один необычный случай для проверки отрицательных значений
}
