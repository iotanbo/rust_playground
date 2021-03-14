



// Default vehicle control interface implementation
pub trait VehicleControl {

    fn forward(&mut self, _speed: i32) {
        println!(" * vehicle is moving forward");
    }

    fn stop(&mut self) {
        println!(" * vehicle stopped");
    }

    fn turn_left(&mut self) {
        println!(" * vehicle turned left");
    }
    
    fn turn_right(&mut self) {
        println!(" * vehicle turned right");
    }
    
}

#[derive(Default)]
pub struct Bicycle {
    speed: i32,
}


impl VehicleControl for Bicycle {

    fn forward(&mut self, speed: i32) {
        self.speed = speed;
        // Limit speed to 30 km/h
        if speed > 30 {self.speed = 30;}

        // Bicycle normally can't go backwards
        if speed < 0 {
            self.speed = 0;
            println!(" * bicycle can't move backwards, stopped");
            return
        }

        println!(" * bicycle is moving forward with speed {} km/h", self.speed);
    }

    fn stop(&mut self) {
        self.speed = 0;
        println!(" * bicycle stopped");
    }

    // fn turn_left(&mut self) {
    //     println!(" * bicycle turned left");
    // }
    
    // fn turn_right(&mut self) {
    //     println!(" * bicycle turned right");
    // }
}


#[derive(Default)]
pub struct Car {
    speed: i32,
}


impl VehicleControl for Car {

    fn forward(&mut self, speed: i32) {
        self.speed = speed;
        // Limit car speed
        if speed > 200 {self.speed = 200;}

        if speed < 0 {
            println!(" * car is moving BACKWARDS, with speed {} km/h", self.speed * -1);
            return
        }

        println!(" * car is moving forward with speed {} km/h", self.speed);
    }

    fn stop(&mut self) {
        self.speed = 0;
        println!(" * car stopped");
    }

    fn turn_left(&mut self) {
        println!(" * car turned left");
    }
    
    fn turn_right(&mut self) {
        println!(" * car turned right");
    }
}


fn polymorphism_demo() {

    // Vector of references to structs that implement 'VehicleControl' trait.
    let mut vehicles: Vec<Box<dyn VehicleControl>> = vec![];

    vehicles.push(Box::new(Bicycle{..Default::default()}));
    vehicles.push(Box::new(Car{..Default::default()}));


    // v is a mutable reference to the object
    for v in &mut vehicles {
        // * Rust does not know exactly what type is v,
        //   but it knows that 'v' supports 'VehicleControl' methods
        v.forward(12);
        v.forward(60);
        v.forward(1000);
        v.forward(-30);
        v.turn_left();
        v.turn_right();
        v.stop();
    }
}



pub fn demo() {

    println!("==  polymorphism demo begin ==");

    polymorphism_demo();

    println!("== polymorphism demo end ==");
    println!();
}


