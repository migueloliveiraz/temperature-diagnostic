mod generate;

use generate::generate_progress_bar;
use systemstat::{System, Platform};

fn main() {
    let system = System::new();
    let temperature = system.cpu_temp().map(|v| v.round() as u64).unwrap();

    let max_progress_bar = 100;

    generate_progress_bar(temperature, max_progress_bar);

    match temperature {
        temp if temp > 89 => println!("Your temperature is greater than 89. Caution!"),
        _ => println!("Keep calm! Temperature is regular."),
    }
}