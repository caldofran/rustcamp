#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Point(i32, i32);

#[derive(Debug, Clone)]
struct Polyline(Vec<Point>);

impl Polyline {
    fn new(point: Point) -> Polyline {
        Polyline(vec![point])
    }

    fn add(&mut self, point: Point) {
        if !self.0.contains(&point) {
            self.0.push(point);
        }
    }

    fn remove(&mut self) {
        if self.0.len() > 1 {
            self.0.pop();
        }
    }

    fn get(&self, index: usize) -> Option<&Point> {
        self.0.get(index)
    }
}

fn main() {
    println!("Implement me!");
    let mut line = Polyline::new(Point::default());
    println!("Line {line:#?}");
    line.add(Point(1, 2));
    println!("Line {line:#?}");
    line.remove();
    println!("Line {line:#?}");
    let point1 = line.get(1);
    let point0 = line.get(0);
    println!("Point 1: {point1:#?}");
    println!("Point 0: {point0:#?}");
}
