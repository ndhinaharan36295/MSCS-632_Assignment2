use std::rc::Rc;

struct TemperatureConverter {
    count: Rc<u32>, // Reference-counted memory
}

impl TemperatureConverter {
    fn new() -> Self {
        TemperatureConverter { count: Rc::new(0) }
    }

    fn convert(&self, temp: f64, to_celsius: bool) -> (f64, u32) {
        let mut count = Rc::get_mut(&mut self.count.clone()).unwrap();
        *count += 1;

        let result = if to_celsius {
            (temp - 32.0) * 5.0 / 9.0
        } else {
            (temp * 9.0 / 5.0) + 32.0
        };

        (result, *count)
    }
}

fn main() {
    let converter = TemperatureConverter::new();
    let (converted1, calls1) = converter.convert(100.0, true);
    println!("Converted: {:.2}, Calls: {}", converted1, calls1);

    let (converted2, calls2) = converter.convert(0.0, false);
    println!("Converted: {:.2}, Calls: {}", converted2, calls2);
}
