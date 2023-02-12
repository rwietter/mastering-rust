pub fn is_equals_len(k: &String, c: &String) -> bool {
    return k.len() == c.len();
}

pub fn hamming_distance(k: String, c: String) -> i32 {
    if is_equals_len(&k, &c) == false {
        panic!("Strings must have the same length");
    }

    let mut distance = 0;

    for i in 0..k.len() {
        if k.chars().nth(i).unwrap() != c.chars().nth(i).unwrap() {
            distance += 1;
        }
    }

    return distance;
}

pub fn init() {
    let k = String::from("karolin");
    let c = String::from("kathrin");

    let distance = hamming_distance(k, c);

    println!("Hamming distance: {}", distance);
}
