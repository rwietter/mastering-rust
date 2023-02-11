mod borrowing;
mod exercices;
mod ownership;

fn to_uppercase(s: &mut String) -> &mut String {
    s.make_ascii_uppercase();
    s
}

fn main() {
    // for number in (1..4).rev() {
    //     println!("{number}!");
    // }

    let mut s = String::from("texto");

    // let r1 = &s; // sem problema
    // let r2 = &s; // sem problema
    let r3 = &mut s; // PROBLEMA GRANDE
    println!("{:?}", to_uppercase(r3));
    println!("{:?}", s);

    ownership::ownership();
    borrowing::borrowing();
    exercices::cubes::sum_cubes(3);
    exercices::narcissistic::is_narcissistic(153);
    exercices::narcissistic::is_narcissistic(407);
    exercices::narcissistic::is_narcissistic(408);
}
