struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }

    fn circumference(&self) -> f64 {
        std::f64::consts::PI * self.radius
    }
}

fn main() {
    let c = Circle::new(0.0, 0.0, 2.0);
    println!("{}", c.circumference());
}
