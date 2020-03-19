fn celsius_to_fahrenheit(temp: f64) -> f64 {
    temp * 1.8 + 32.0
}

fn main() {
    let mut celsius_degree : f64 = -20.0;

    println!("Celcius(°C)  Fahrenheit(°F)");
    println!("===========================");

    while celsius_degree <= 20.0 {
        println!("{:11.3} {:15.3}", celsius_degree, celsius_to_fahrenheit(celsius_degree));
        celsius_degree += 2.0;
    }
}