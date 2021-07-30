pub fn run() {
	let yellow_light = TrafficLight::Yellow;
	let red_light = TrafficLight::Red;
	let green_light = TrafficLight::Green;
	println!("The time of yellow light is: {}", yellow_light.time());
	println!("The time of red light is: {}", red_light.time());
	println!("The time of green light is: {}", green_light.time());
}

enum TrafficLight {
	Red,
	Green,
	Yellow
}

impl TrafficLight {
	fn time(&self) -> u8 {
			match self {
					TrafficLight::Red => 60,
					TrafficLight::Green => 30,
					TrafficLight::Yellow => 3
			}
	}
}