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
            Ok(start) => println!("{} took: {}", self.name, start.as_secs_f64()),
            Err(e) => println!("Timeguard Error {:?}", e),
        }
    }
}

fn main() {
    {
        let _tg = Timeguard::new("dummy".to_string());
        let dummy = primes::dummy(1000);
        println!("Dummy answer: {}", dummy);
    }
    {
        let _tg = Timeguard::new("eratorsten".to_string());
        let eratosten = primes::eratosten(1000, 100);
        println!("Eratorsten answer: {}", eratosten);
    }
}

mod primes {
    pub fn eratosten(mut n: usize, buffer_size: usize) -> usize {
        let mut buffer: Vec<usize> = Vec::with_capacity(buffer_size);
        let mut primes: Vec<usize> = Vec::new();
        let mut last_i = 0;
        for i in 0..buffer_size {
            buffer.push(i + 1);
        }
        let mut last_max = buffer[buffer_size - 1];
        while n > 0 {
            last_i += 1;
            if last_i >= buffer_size {
                last_i = 0;
                let upper_bound = last_max + buffer_size;
                buffer.clear();
                for i in last_max..upper_bound {
                    let i = i + 1;
                    buffer.push(i);
                    for p in primes.clone() {
                        if i % p == 0 {
                            buffer.pop();
                            buffer.push(0);
                        }
                    }
                }
                last_max += buffer_size;
            }
            // 0s on strides
            if buffer[last_i] != 0 {
                primes.push(buffer[last_i]);
                n -= 1;
                let mut i = last_i;
                while (i + buffer[last_i]) < buffer_size {
                    i += buffer[last_i];
                    buffer[i] = 0;
                }
            }
        }
        primes.last().unwrap().clone()
    }

    pub fn dummy(mut n: usize) -> usize {
        let mut cur = 2;
        while n > 0 {
            let mut is_prime = true;
            for i in 2..cur {
                if cur % i == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                n -= 1;
            }
            cur += 1;
        }
        cur - 1
    }
}
