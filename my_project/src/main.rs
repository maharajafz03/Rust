
// mod server;

// trait Racing {
//     fn mode(&self);
    
//     // Default implementation of display
//     fn display(&self) {
//         println!("This is a racer.");
//     }
// }

// struct Driver {
//     car_name: String,
//     fuel_type: String,
// }

// struct Biker {
//     bike_name: String,
//     color: String,
// }

// impl Racing for Driver {
//     fn mode(&self) {
//         println!("F1_driver");
//     }

//     // Overriding the default display method
//     fn display(&self) {
//         println!("{} {}", self.car_name, self.fuel_type);
//     }
// }

// impl Racing for Biker {
//     fn mode(&self) {
//         println!("MOTO_GP");
//     }

//     // Overriding the default display method
//     fn display(&self) {
//         println!("{} {}", self.bike_name, self.color);
//     }
// }

// fn render<T: Racing>(racer: &T) {
//     racer.mode();
//     racer.display();  // Display now works for both Driver and Biker
// }

// fn main() {
//     let user = Biker {
//         bike_name: String::from("ducati_panigale"),
//         color: String::from("red"),
//     };

//     let user2 = Driver {
//         car_name: String::from("pagani"),
//         fuel_type: String::from("hybrid"),
//     };

//     render(&user);
//     render(&user2);

//     server::server(); 
//     println!("{}",sum!(7, 8));
//
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
}