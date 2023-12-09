pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 9: Mirage Maintenance";

mod error;
mod input;
mod oasis;

pub fn solve_a() {
	let mut reports = input::get_input().unwrap();

	let mut sum = 0;
	for report in reports.iter_mut() {
		if let Err(err) = report.interpolate() {
			println!("Interpolation error: {:?}", err);
			report.print();
			continue;
		}
		match report.extrapolate() {
			Ok(new) => { sum += new; },
			Err(err) => {
				println!("Extrapolation error: {:?}", err);
				report.print();
			}
		}
	}
	println!("Extrapolated value: {}", sum);
}

pub fn solve_b() {
	let mut reports = input::get_input().unwrap();

	let mut sum = 0;
	for report in reports.iter_mut() {
		if let Err(err) = report.interpolate() {
			println!("Interpolation error: {:?}", err);
			report.print();
			continue;
		}
		match report.extrapolate_back() {
			Ok(new) => { sum += new; },
			Err(err) => {
				println!("Extrapolation error: {:?}", err);
				report.print();
			}
		}
	}
	println!("Back Extrapolated value: {}", sum);

}
