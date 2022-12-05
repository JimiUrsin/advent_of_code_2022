mod day01;
mod common;

pub fn main() {
    run_day(1);
}

fn run_day(day: u8) {
    match day {
        1 => day01::solve(),
        2..=25 => println!("Not implemented yet."),
        _ => println!("Why")
    }
}
