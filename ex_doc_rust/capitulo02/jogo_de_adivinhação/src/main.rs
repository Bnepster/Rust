// Jogo de adivinhação:
extern crate rand;

use std::io; // Chamada para biblioteca de entrada e saida 
use rand::Rng;

fn main() {
    println!("Advinhe o número!");

    let numero_secreto = rand::thread_rng().gen_range(1, 101);

    println!("O número secreto é: {}", numero_secreto);

    println!("Digite o seu palpite.");

    let mut palpite = String::new(); // Essa é uma vairável mutavel que possui uma string vazia

    io::stdin().read_line(&mut palpite) // Recebe o que o usuário digita na entrada padrão e coloca isso numa string.
        .expect("Falha ao ler entrada");  // Essa linha é responsavel por tratar possiveis erros, isso irá parar o programa.

    println!("Você disse: {}", palpite); // Essa linha imprime a string que salvamos os dados inseridos pelo usuário.
}
