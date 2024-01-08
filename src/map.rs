use std::collections::HashMap;

pub fn map() {
    let mut mapa: HashMap<&str, &str> = HashMap::new();
    mapa.insert("nome", "José Augusto");
    mapa.insert("email", "josemoura212@gmail.com");
    mapa.insert("url", "https://mangatrix.b4a.app");

    let valores = HashMap::from([("nome", "José Augsuto")]);

    println!("{:?}", mapa);
    println!("{:?}", valores);

    for (chave, valor) in &mapa {
        println!("chave: {}, valor: {}", chave, valor);
    }

    match mapa.get(&"nome") {
        Some(valor) => println!("{}", valor),
        None => println!("Valor nao encontrado"),
    }
}
