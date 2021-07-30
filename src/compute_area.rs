pub fn run() {
	let circle = Circle {
		r: 2_f32
	};

	print_area(circle)
}

fn print_area<T: Graph>(graph: T) {
	println!("The area is {}", graph.compute_area());
}

trait Graph {
	fn compute_area(&self) -> f32;
}

struct Circle {
	r: f32
}

struct Rectangle {
	x: f32,
	y: f32
}

impl Graph for Circle {
	fn compute_area(&self) -> f32 {
		self.r * self.r * 3.1415926
	}
}

impl Graph for Rectangle {
	fn compute_area(&self) -> f32 {
		self.x * self.y
	}
}
