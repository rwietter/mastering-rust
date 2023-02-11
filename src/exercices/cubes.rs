// Write a function that takes a positive integer n, sums all the cubed values from 1 to n (inclusive), and returns that sum.

// Assume that the input n will always be a positive integer.

// Examples: (Input --> output)
// 2 --> 9 (sum of the cubes of 1 and 2 is 1 + 8)
// 3 --> 36 (sum of the cubes of 1, 2, and 3 is 1 + 8 + 27)

/*
  let res: RangeInclusive<u32> = 1..=n; // 1..=n => 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
*/

/*
  O operador |i| é uma forma curta de escrever uma função anônima (também conhecida como closure) em Rust. Em outras
  palavras, ele define uma função que não tem um nome e pode ser passada como argumento para outras funções, como a
  função map.
*/

/*
  Fully annotated closure: let variable = |i: u32| -> u32 { ... }
*/

use std::ops::RangeInclusive;

pub fn sum_cubes(natural: u32) -> u32 {
    let range = |n: u32| -> RangeInclusive<u32> { 1..=n };
    let pow = |idx: u32| -> u32 { idx.pow(3) };

    return range(natural).map(pow).sum();
}

/*
  ------------ Versão com Código Imperativo ------------
  pub fn sum_cubes(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i.pow(3);
    }
    sum;
  }
*/
