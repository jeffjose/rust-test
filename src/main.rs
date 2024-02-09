use rand;
trait Coordinate {
    fn scale(&mut self, scaler: f32);
    fn display(&self);
}
struct RandomPoint {
    x: f32,
    y: f32,
}
impl RandomPoint {
    fn new() -> Self {
        Self {
            x: rand::random(),
            y: rand::random(),
        }
    }
}

impl Coordinate for RandomPoint {
    fn scale(&mut self, scaler: f32) {
        self.x *= scaler;
        self.y *= scaler;
    }

    fn display(&self) {
        println!("{} {}", self.x, self.y)
    }
}

struct StaticPoint {
    x: f32,
    y: f32,
}

impl StaticPoint {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Coordinate for StaticPoint {
    fn scale(&mut self, scaler: f32) {
        self.x *= scaler;
        self.y *= scaler;
    }

    fn display(&self) {
        println!("{} {}", self.x, self.y)
    }
}
fn main() {
    let mut p1 = StaticPoint::new(1.0, 1.0);

    p1.display();

    p1.scale(10.0);
    p1.display();

    let mut r1 = RandomPoint::new();

    r1.display();
    r1.scale(10.0);
    r1.display();
}
