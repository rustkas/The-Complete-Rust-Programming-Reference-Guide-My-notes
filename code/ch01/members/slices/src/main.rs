fn main() {
    let mut numbers: [u8; 4] = [1, 2, 3, 4];
    {
        let all = &numbers[..];
        println!("All of them: {:?}", all);
    }

    {
        let first_two: &mut [u8] = &mut numbers[0..2];
        first_two[0] = 100;
        first_two[1] = 99;

        // Создаем вектор из данных среза first_two
        let vec_from_slice: Vec<u8> = first_two.to_owned();
        numbers[0..2].copy_from_slice(&vec_from_slice);
    }

    println!("Look ma! I can modify through slices: {:?}", numbers);
}
