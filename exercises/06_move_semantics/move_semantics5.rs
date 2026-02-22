#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}



// Data et défini 

// Get_car l'utilise mais sans prendre l'emprunt donc il devient propriétaire de la String
// Cela passe dans la fonction est la sortie est '!'

// string_uppercase avec en paramètres data, mais data n'existe plus parceque 