#[test]
fn test_day03() {
    let car = Car { speed: 100 };
    println!("car speed: {}", car.get_speed());
    assert_eq!(car.get_speed(), 100);
    let truck = Truck { speed: 200 };
    assert_eq!(truck.get_speed(), 200);
    println!("truck speed: {}", truck.get_speed());
}

trait Speed {
    fn get_speed(&self) -> i32;
}

struct Car {
    speed: i32,
}

impl Speed for Car {
    fn get_speed(&self) -> i32 {
        self.speed
    }
}

struct Truck {
    speed: i32,
}

impl Speed for Truck {
    fn get_speed(&self) -> i32 {
        self.speed
    }
}
