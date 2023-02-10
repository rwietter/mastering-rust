pub fn borrowing() {
    // A sintaxe &s1 nos permite criar uma referência que se refere ao valor s1, mas não o possui. Como ela não o possui, o valor a que ela aponta não será destruído quando a referência sair de escopo.
    let s1 = String::from("texto");

    let tamanho = calcula_tamanho(&s1);

    println!("O tamanho de '{}' é {}.", s1, tamanho);
}

// [s] -> [s1] -> [0x01] -> "texto"
fn calcula_tamanho(s: &String) -> usize {
    s.len()
}
