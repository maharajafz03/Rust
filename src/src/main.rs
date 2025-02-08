struct Car{
    brand: String,
    model: String
}

impl Car{

    fn display(&self) {
        println!("{} {}", self.brand, self.model);
    }

    fn update(&mut self, brand: String){
        self.brand = brand;
    }
}

fn main() {

    let mut car = Car{
        brand: String::from("tesla"),
        model: String::from("model_3")
    };

    car.display();
    car.update(String::from("lamborghini"));
    car.display();
}