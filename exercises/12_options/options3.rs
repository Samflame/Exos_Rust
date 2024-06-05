// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    // Utilisez une référence à `y` pour éviter de prendre possession de la valeur.
    match &y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    // Maintenant, `y` peut être utilisé après le `match` car sa valeur n'a pas été déplacée.
    y; // Cette ligne est correcte et ne causera pas d'erreur.
}

//J'ai ajouté une référence non mutable à y avant le match
