#[test]
fn test_day02() {
    // slice
    let slice = [1, 2, 3];
    println!("slice: {:?}", slice[0]);

    let point = Point::new(1, 2);
    println!("point: {:?}", point);
    let s = String::from("hello");
    println!("s: {:?}", s);

    let some_number = Some(5);
    let some_char = Some('e');
    println!("{:?}", some_number);
    println!("{:?}", some_char);
    // 集合
    let mut v: Vec<i32> = Vec::new();
    println!("v: {:?}", v);
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v: {:?}", v[1]);
    println!("v: {:?}", &v[2]);

    for v in &mut v {
        *v += 1;
        println!("{}", v);
        println!("{}", *v);
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
        }
    }
}
