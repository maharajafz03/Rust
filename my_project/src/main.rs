trait Racing {
   fn mode(&self); 
}

struct Driver {
    car_name: String,
    fuel_type: String
}

struct Biker {
    bike_name: String,
    color: String
}

impl Racing for Driver {
    
    fn mode(&self) {
        println!("F1_driver")
    }

    fn display(&self) {
     println!("{} {}", self.car_name, self.fuel_type )
    }
}

impl Racing for Biker {    
    fn mode(&self) {
        println!("MOTO_GP")
    }
}        

fn render<T: Racing>(racer: T) {
    racer.mode();
}

fn main() {

    let user = Biker {
        bike_name: String::from("ducati_panigale"),
        color: String::from("red")
    };

    let user2 = Driver{
        car_name: String::from("pagani"),
        fuel_type: String::from("hybrid")
    };

    render(user);
    render(user2);
}