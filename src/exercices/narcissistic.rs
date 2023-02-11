pub fn is_narcissistic(number: u32) -> bool {
    let number_base = number.to_string().len() as u32;

    let narcissistic = number
        .to_string()
        .chars()
        .try_fold(Vec::new(), |mut f, c| {
            f.push(c.to_digit(10).unwrap());
            Some(f)
        })
        .unwrap()
        .iter()
        .map(|d| d.pow(number_base))
        .sum::<u32>();

    println!(
        "number {} is narcissistic number {:?} ? {:?}",
        number,
        narcissistic,
        narcissistic == number
    );

    return narcissistic == number;
}

#[test]
fn is_narcissistic_test() {
    assert_eq!(is_narcissistic(1), true);
    assert_eq!(is_narcissistic(2), true);
    assert_eq!(is_narcissistic(3), true);
    assert_eq!(is_narcissistic(4), true);
    assert_eq!(is_narcissistic(5), true);
    assert_eq!(is_narcissistic(153), true);
    assert_eq!(is_narcissistic(154), false);
    assert_eq!(is_narcissistic(370), true);
    assert_eq!(is_narcissistic(371), true);
    assert_eq!(is_narcissistic(407), true);
    assert_eq!(is_narcissistic(408), false);
    assert_eq!(is_narcissistic(9926315), true);
}
