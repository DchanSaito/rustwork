use std::io;
use std::io::Write;

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Triangle {
    fn area(&self) -> f64 {
        let s = (self.a + self.b + self.c) / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }
}

struct TriangleBuilder {
    a: f64,
    b: f64,
    c: f64,
}

impl TriangleBuilder {
    fn new() -> TriangleBuilder {
        TriangleBuilder { a: 3.0, b: 4.0, c: 5.0, }
    }

    fn a(&mut self, arg: f64) -> &mut TriangleBuilder {
        self.a = arg;
        self
    }

    fn b(&mut self, arg: f64) -> &mut TriangleBuilder {
        self.b = arg;
        self
    }

    fn c(&mut self, arg: f64) -> &mut TriangleBuilder {
        self.c = arg;
        self
    }

    fn finalize(&self) -> Triangle {
        Triangle { a: self.a, b: self.b, c: self.c }
    }
}

fn input_data() -> f64 {
    let mut line = String::new();

    io::stdin().read_line(&mut line)
        .expect("Failed to read line");

    let line: f64 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => 1.0,
    };

    line
}
    

fn main() {
    print!("input a: ");
    io::stdout().flush().unwrap();
    let a = input_data();

    print!("input b: ");
    io::stdout().flush().unwrap();
    let b = input_data();

    print!("input c: ");
    io::stdout().flush().unwrap();
    let c = input_data();
    
    let tri = TriangleBuilder::new()
        .a(a).b(b).c(c)
        .finalize();

    println!("this triangle's area: {}", tri.area());
}

////////////////////////////////////////////////////////////////////

#[test]
fn triangle_test() {
    let tri = Triangle { a: 3.0, b: 4.0, c: 5.0 };
    assert_eq!(6.0, tri.area());
}
