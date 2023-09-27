fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("O valor de X é {}" ,x);
    }
    println!("Esse é o valor de X {}" ,x);

    {
        let nome = "Bruno";
        println!("{}", nome);
        let nome = "sousa";
        println!("{}", nome);
    }

    // Uma constante deve ser declarada usando o metodo const.
    // Pode estar presente até mesmo no escopo global.
    // Seu nome deve ser escrito com LETRAS_MAIUSCILAS (e um sublinhado entre elas)

    const NOME_COMPLETO: &str = "Bruno de Sousa da Silva";
    println!("{}", NOME_COMPLETO);
}
