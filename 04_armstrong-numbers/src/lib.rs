pub fn is_armstrong_number(num: u32) -> bool {
    let pow: u32 = num.to_string().len() as u32;
    let sum: u32 = num.to_string()
        .chars()
        .map(|c| {
            c.to_digit(10).unwrap().pow(pow)
        })
        .sum();

    sum == num
}
