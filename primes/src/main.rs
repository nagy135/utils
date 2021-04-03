use std::time::SystemTime;

struct Timeguard {
    start: SystemTime,
    name: String,
}

impl Timeguard {
    fn new(name: String) -> Timeguard {
        Timeguard {
            name,
            start: SystemTime::now(),
        }
    }
}

impl Drop for Timeguard {
    fn drop(&mut self) {
        match self.start.elapsed() {
            Ok(start) => println!("{}: ,took: {}", self.name, start.as_secs_f64()),
            Err(e) => println!("Timeguard Error {:?}", e),
        }
    }
}

fn main() {
    {
        let _tg = Timeguard::new("dummy".to_string());
        let dummy = primes::dummy(4);
        println!("Dummy answer: {}", dummy);
    }
}

mod primes {
    pub fn dummy(n: usize) -> usize {
        5
    }
}
