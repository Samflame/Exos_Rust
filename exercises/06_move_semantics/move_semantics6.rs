// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let data = "Rust is great!".to_string();

    let last_char = get_char(&data);
    println!("Dernier caractère : {}", last_char);

    string_uppercase(data);
}

fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

fn string_uppercase(data: String) {
    let data_uppercase = data.to_uppercase();
    println!("{}", data_uppercase);
}

//J’ai ajouté une variable last_char pour stocker le résultat de get_char et l’afficher
// La fonction get_char prend maintenant une référence à une String au lieu de prendre possession de la String ce qui permet à data d’être utilisé à nouveau après l’appel à get_char.
//Quant à la fonction string_uppercase, elle prend maintenant possession de la String passée en paramètre. Elle crée une nouvelle String en majuscules et l’affiche
