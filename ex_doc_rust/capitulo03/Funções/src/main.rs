fn main() {
    println!("Hello, world!");

    func_teste(); // Chamando a função.
    func_num(5); // Chamando função com parametro .
    num_mais(2, 'B'); // Chamando uma função com dois parametros.
}

// Criação de nova função.
fn func_teste() {
    println!("Função teste");
}

// Criando função com patametro.
fn func_num(x: i32) {
    println!("O parametro é {x}");
}

// Criando função com mais de um parametro.
fn num_mais(primeiro: i32, segundo: char){
    println!("Primeiro parametro {primeiro}, segundo parametro {segundo}");
}

fn declaracao() {
    let j = 6; // Isso é uma declaração.
}

fn exprecao() {
    let a = 5;

    let b = {
        let a = 3;
        a + 1 // Essa é uma expreção, ou seja retorna resultados.
    };

    println!("O valor de b é {}", b);
}

fn oito() -> i32 {
    8
}

fn func_retorno() {
    let x = oito();

    println!("O valor de x é {}", x);
}

// Os valores dos parametros são são delinidos na chamada da funçãa, ja na função em si são definidos os nomes e os tipos dos parametros, como podemos ver acima.

// Definições de função também são definições; todo o exemplo é uma declaração em si.
// Declarações não retornam resultados.
// Expreções retornam resultados e não terminam com ponto e virgula.

