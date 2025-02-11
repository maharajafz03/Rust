
mod server;
mod latest;


#[derive(Debug,Clone)]
enum Maga {
   right,
    left
}

fn main() {
    let s = Maga::left;
    let b = s.clone();
    println!("{:?}", s);
    println!("{:?}", b);
    server::server();
    latest::latest(); 
}