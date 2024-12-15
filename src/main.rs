use std::io;

fn main() {
    let mut nome = String::new();
    println!("Digite o seu Nome:");

    io::stdin()
        .read_line(&mut nome)
        .expect("Falha ao ler o nome");

    let mut ano = String::new();
    println!("Digite o ano que você nasceu:");

    io::stdin()
        .read_line(&mut ano)
        .expect("Falha ao ler o ano");

    // Converte o ano para inteiro
    let ano: i32 = ano.trim().parse().expect("Por favor, digite um número válido!");
    let idade = 2024 - ano;

    println!("Olá, bem-vindo, {}! Você tem {} anos.", nome.trim(), idade);
}
