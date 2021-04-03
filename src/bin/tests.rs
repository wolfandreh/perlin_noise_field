fn main() {
    // define simple enum
    enum Color {
        Red,
        Green,
        Blue,
        Orange,
        Custom(String),
        Coord{ x: i32, y: i32 }
    }

    let favorite: Color = Color::Green;

    let custom: Color = Color::Custom("pink".to_string());

    if let Color::Green = favorite {
        println!("Favorite color is green. ")
    }

    match favorite {
        Color::Green => println!("favorite color is green"),
        Color::Blue => println!("favorite color is blue"),
        _ => {}
    }

    match custom {
        Color::Custom(color) => println!("Favorite Color is {}", color),
        _ => {}
    }
}