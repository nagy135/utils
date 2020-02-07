use std::env;

fn main(){
    if env::args().len() < 2 {
        panic!("Please, specify argument...one string value");
    }
    let args: Vec<String> = env::args().collect();
    print!("{}", str_to_int(&args[1]));
}

fn str_to_int(data: &str) -> u64 {
    let byte_arr = data.to_string().into_bytes();
    let mut result = 0;
    let alpha_offset: u64 = 26;
    for (o,i) in byte_arr.iter().rev().enumerate() {
        let order_i: u64 = *i as u64 - 97;
        if o > 0 {
            result += alpha_offset.pow(o as u32) + order_i;
        } else {
            result += order_i;
        }
    }
    result
}
