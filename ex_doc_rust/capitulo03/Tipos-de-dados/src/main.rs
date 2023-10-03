fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    let x = 2.2;
    let y: f32 = 3.3;
    println!("x é igual a {} y é igual a {}", x, y);

    //  Operadores numericos

    let some = 5 + 10;
    let subtrair = 95.5 - 4.3;
    let multiplicar = 4 * 38;

    //divisão
    let quociente = 56.7 / 32.2; 
    let truncado = -5 / 3;
    let resto_div = 43 % 5;

    println!("soma {}, subtração {}, Multiplicação {}, quociente {}, truncado {} e resto da divisão {}", some, subtrair, multiplicar, quociente, truncado, resto_div);

    let t = true;
    let f: bool = false;
    println!("Verdadeiro é {} e Falso é {}", t, f);

    // CRIANDO UMA TUPLA

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // PRIMEIRA FORMA DE PEGAR UM DADO DE UMA TUPLA
    println!("O valor de y é: {y}");

    let tpl: (i32, f64, u8) = (278, 22.1, 7);
    let n1 = tpl.0; // SEGUNDA FORMA DE PEGAR UM DADO DE UMA TUPLA
    println!("{n1}");
}


// Em rust a quatro tipos de pricipais de dados são eles:
// Inteiro, Flutuante, Boleanos e Caracter

// INTEIROS: O tipo Imteiro recebe um comprimento e uma letra que define se q tipo vai ser com ou sem sinal.

// O comprimento pode variar entre 8, 16, 32, 64 e 128 bits. Antes dessa definição uma letra na frente "u" USIZE e "i" ISIZE.

// ESSAS LETRAS INDICAM SE A VARIAVEL PODE OU NÃO RECEBER NUMEROS NEGATIVOS. SENDO "i" PODE RECEBER NÚMEROS NEGATIVOS E "u" NÃO PODE.

// PONTO FLUTUANTES: O rust possui dois tipos primitivoss de para números com ponto flutuantes são eles f32 e f64 sendo este o padão.

// OPERADORES NÚMERICOS: contem quatro pricipais são eles, adisão, subtração, multiplicação e divisão, sendo essa dividida em divisão simples e divisão inteira.

// TIPO BOOLEANO: valor booleano e composto por dois valores true e false.

// TIPOS COMPOSTOS: Os tipos compostos podem agrupar vários valores em um tipo. Rust tem dois tipos de compostos primitivos: tuplas e arrays. 

// TUPLAS: A tupla sem nenhum valor possui um nome especial, unit . Este valor e seu tipo correspondente são escritos ()e representam um valor vazio ou um tipo de retorno vazio. As expressões retornam implicitamente o valor unitário se não retornarem nenhum outro valor.

