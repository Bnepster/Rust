fn main() {
    println!("Hello, world!");

    func_teste(); // Chamando a função
    func_num(5); // Chamando função com parametro 
    num_mais(2, 'B'); // Chamando uma função com dois parametros
}

// Criação de nova função
fn func_teste() {
    println!("Função teste");
}

// Criando função com patametro
fn func_num(x: i32) {
    println!("O parametro é {x}");
}

// Criando função com mais de um parametro
fn num_mais(primeiro: i32, segundo: char){
    println!("Primeiro parametro {primeiro}, segundo parametro {segundo}");
}

// Os valores dos parametros são são delinidos na chamada da funçãa, ja na função em si são definidos os nomes e os tipos dos parametros, como podemos ver acima.

