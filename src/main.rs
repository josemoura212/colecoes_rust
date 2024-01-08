fn main() {
    let lista: [u8; 5] = [1, 2, 3, 4, 5];

    println!("Valor na posição 2: {}\n", lista[2]);

    let mut numeros: Vec<u8> = Vec::new();

    for i in 1..=5 {
        numeros.push(i);
    }

    println!("Valores do vetor {:?} ", numeros);

    for n in numeros {
        println!("{}", n);
    }
}
