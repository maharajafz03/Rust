#[derive(Debug)]
struct Beh {
    name: String,
    age: i32,
}

fn main() {
    // Initialize a vector of `Beh`
    let mut karni: Vec<Beh> = Vec::new();

    // Push a new `Beh` struct into the vector
    karni.push(Beh {
        name: String::from("mgaraja"),
        age: 29,
    });

    // Print the vector using `{:?}` for debugging
    println!("{:?}", karni);
}
