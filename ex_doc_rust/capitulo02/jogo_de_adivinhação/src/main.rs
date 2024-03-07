use std::io; // Chamada para biblioteca de entrada e saida 

fn main() {
    println!("Adivinhe o número!");
    println!("Digite o seu palpite.");

    let mut palpite = String::new();

    io::stdin().read_line(&mut palpite)       
        .expect("Falha ao ler entrada");

    println!("Vôce disse: {}", palpite);
}
