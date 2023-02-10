mod borrowing;
mod exercices;
mod ownership;

fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }

    ownership::ownership();
    borrowing::borrowing();
    exercices::cubes::sum_cubes(3);
}
