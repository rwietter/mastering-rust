#![allow(unused_doc_comments)]

// A.R.I.I
// O.B.R.M Ownership Borrowing Resource Management
// Borrow Checker => vai checar as referências e empréstimos
// Tipos copy => tipos que são copiados para a stack em outro endereço (i32, f32, bool, char, etc)

pub fn ownership() {
    // ----------------------------------------------------
    // ---------------------- Stack -----------------------
    // ----------------------------------------------------
    let a = 5; // [a] -> 5
    let b = a; // [b] -> 5
    let c = &b; // [c] -> [b] -> 5 (c referencia b, que referencia 5)
    println!("a: {}, b: {}, c: {}", a, b, *c);

    // ----------------------------------------------------
    // ---------------------- Heap ------------------------
    // ----------------------------------------------------
    let s = String::from("Hello"); // [s] -> [0x01] -> "Hello" | String "Hello" é posse de s
    let mut s1 = s; // [s1] -> [0x01] -> "Hello"

    // let s2 = s; // => s transferiu posse para s1, não pode ser usado pois foi desalocado

    let s2 = &mut s1; // [s2] -> [s] -> [0x01] -> "Hello"
    s2.push_str(", world!");
    println!("s2: {}", *s2); // [s2] -> [s] -> [0x01] -> "Hello, world!"
    println!("s: {}", s1); // [s] -> [0x01] -> "Hello, world!"

    // ----------------------------------------------------
    // ---------------------- Heap ------------------------
    // ----------------------------------------------------
    read_borrowing();
    mut_borrowing();
}

// Borrow significa emprestar a posse de uma variável para outra
pub fn read_borrowing() {
    // ----------------------------------------------------
    // ------------------ Borrow Stack --------------------
    // ----------------------------------------------------
    let bruna = "Bruna"; // alocado na stack
    stack_say_goodbye(bruna);

    // ----------------------------------------------------
    // ------------------ Borrow Heap --------------------
    // ----------------------------------------------------
    let carlos = String::from("Carlos");
    // let larrisa = &carlos; // Carlos emprestou a posse de sua propriedade (*READ*) para Larrisa
    heap_say_goodbye(carlos); // Carlos emprestou a posse de sua propriedade (*READ*) para a função

    // heap_say_good_morning(carlos); // a posse de carlos foi movida para `goodbye` e deixou de ser existida. (value used here after move) => precisa clonar a propriedade ou emprestar a propriedade

    let patricia = String::from("Patricia");
    // Mas podemos resolver isso, em vez de passar a posse da variável, podemos passar uma emprestar a propriedade (READ | MUT)
    heap_borrow_say_good_morning(&patricia) // [patricia] -> [0x83834984] -> "Patricia"
}

// Os argumentos da função devem ter um tamanho conhecido estaticamente, os tipos emprestados sempre têm um tamanho
// conhecido. Eles devem ser copiados utilizando o operador de cópia (&).
fn stack_say_goodbye(name: &str) {
    println!("\nStack Goodbye, {}!\n", name);
}

fn heap_say_goodbye(goodbye: String) {
    println!("Heap Goodbye, {}!\n", goodbye);
}

fn heap_say_good_morning(morning: String) {
    println!("Heap Good Morning, {}!\n", morning);
}

// [borrow_goodbye] -> [0x83834984] -> "Patricia"
fn heap_borrow_say_good_morning(borrow_goodbye: &String) {
    println!("Heap Good Morning, {}!\n", borrow_goodbye);
}

// -------------------------------------------
// ------------ Mut Borrowing ----------------
// -------------------------------------------
fn mut_borrowing() {
    let mut word = String::from("Hello");
    add_prefix(&mut word);
    to_uppercase(&mut word);
    println!("{word}");
}

fn to_uppercase(word: &mut String) -> &mut String {
    *word = word.to_uppercase();
    word
}

fn add_prefix(word: &mut String) -> &mut String {
    *word = format!("{}{}", "Hi, ", word);
    word
}
