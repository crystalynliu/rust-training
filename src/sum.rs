pub fn run() {
	let result = sum(&[u32::max_value(), 2, 3, 4]);
	println!("Sum result is {}", match result {
		Some(v) => v,
		None => 0
	});
}


fn sum(nums: &[u32]) -> Option<u32> {
	let mut result: Option<u32> = Some(0_u32);

	for num in nums.iter() {
		result = match result {
			Some(v) => v.checked_add(*num),
			None => None
		}
	}
	result
}

