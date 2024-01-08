pub fn stringes() {
    let mut texto = String::from("José Augusto");

    texto.push_str(" Soares");

    println!("{}", texto);
    println!("capacity: {}", texto.capacity());
}
