use std::string;

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


    let mut vinayaga = vec![1,2,3,4];
    vinayaga.push(76);

    let mut siva =Vec::new();
    siva.push(78);

    println!("this is vinayaga {:?} and this is siva {:?}",vinayaga, siva);

    // let mut mani = ["maga", "karan", "yamuna"];
    // mani.push_str("sino");
    // println!("{}", mani);
    let mut string = String::from("hewllo blow");
    string = String::from("magaraja");
    string.push_str("yamuna");
    println!("{}", string);

    let gabesh = '!';
    println!("{}", gabesh);

}

