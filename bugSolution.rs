fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    // Safe way to access elements, handles out-of-bounds gracefully:
    match vec.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("Index out of bounds!"),
    };

    //Another safe method, using the if let syntax
    if let Some(third) = vec.get(2) {
        println!("The third element is {}", third);
    } else {
        println!("Index out of bounds!");
    }
} 