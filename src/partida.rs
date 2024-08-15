use std::io::{self, Write};

pub fn nueva_partida(palabra: &String) {
    println!("\n\nBienvenido al ahorcado de FIUBA!");
    let mut intentos = 6;
    let mut palabra_usuario = generar_palabra_usuario(palabra.len());
    let mut letras_usuario: Vec<String> = Vec::new();
    while (*palabra != palabra_usuario) && (intentos > 0) {
        imprimir_palabra_usuario(&mut palabra_usuario);
        imprimir_letras_usuario(&mut letras_usuario);
        println!("Te quedan {} intentos.", intentos);
        let resultado_turno = nuevo_turno(&palabra, &mut letras_usuario, &mut palabra_usuario);
        if !resultado_turno {
            intentos -= 1;
        }
    }
    if *palabra == palabra_usuario {
        ganar(palabra_usuario, letras_usuario, intentos);
    }
    else {
        println!("\nPerdiste la partida ya que te quedaste sin intentos.");
    }
}

fn ganar(mut palabra_usuario: String, mut letras_usuario: Vec<String>, intentos: usize) {
    imprimir_palabra_usuario(&mut palabra_usuario);
    imprimir_letras_usuario(&mut letras_usuario);
    println!("\n¡FELICIDADES! Ganaste la partida. Te quedaron {} intentos.", intentos);
}

fn imprimir_letras_usuario(letras_usuario: &mut Vec<String>) {
    let str = letras_usuario.iter().map(|letra| format!("{}", letra)).collect::<Vec<String>>().join(" ");
    println!("Adivinaste las siguientes letras: {}", str);
}

fn imprimir_palabra_usuario(palabra_usuario: &mut String) {
    let palabra_usuario_split: Vec<&str> = palabra_usuario.split("").collect();
    let palabra_usuario_join = palabra_usuario_split.join(" ");
    println!("\nLa palabra hasta el momento es: {}", palabra_usuario_join.trim());
}

fn nuevo_turno(palabra: &String, letras_usuario: &mut Vec<String>, palabra_usuario: &mut String) -> bool {
    let letra = pedir_letra();
    if palabra.contains(&letra) {
        if !letras_usuario.contains(&letra) {
            letras_usuario.push(letra.clone());
            actualizar_palabra_usuario(&palabra, &letra, palabra_usuario);
        }
        return true;
    }
    false
}

fn actualizar_palabra_usuario(palabra: &String, letra: &String, palabra_usuario: &mut String) {
    for (i, caracter_palabra) in palabra.chars().enumerate() {
        if *letra == caracter_palabra.to_string() {
            palabra_usuario.replace_range(i..i+1, letra);
        }
    }
}

fn pedir_letra() -> String {
    let mut letra = String::new();
    print!("Ingresa una letra: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut letra).expect("Error reading from STDIN");
    let trimmed_letra = letra.trim().to_string();
    trimmed_letra
}

fn generar_palabra_usuario(longitud: usize) -> String {
    let mut palabra_usuario = String::new();
    for _i in 0..longitud {
        palabra_usuario += "_";
    }
    palabra_usuario
}