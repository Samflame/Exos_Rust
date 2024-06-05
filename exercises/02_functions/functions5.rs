// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}

// la fonction square ne retourne pas de valeur
// Il faut utiliser l’instruction return ou écrire l’expression de retour sans point-virgule.
