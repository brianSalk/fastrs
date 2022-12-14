use rand;
use rand::Rng;
use std::env;

fn main() {
    let mut min_length:u32 = 100;
    let mut max_length:u32 = 200;
    // process command line arguments
    let mut i:usize = 1;
    let len = env::args().len();
    while i < len  {
        let next_arg = env::args().nth(i).unwrap();
        if next_arg == String::from("--min") {
            if i < env::args().len() {
                let min_length_str = match env::args().nth(i+1) {
                    Some(num) => num,
                    None => {
                        eprintln!("missing mandatory argument for --min");
                        std::process::exit(1);
                    }
                };
                min_length = match min_length_str.parse::<u32>() {
                   Ok(num) => num,
                   Err(e) => {
                        eprintln!("{0}",e);
                        std::process::exit(1);
                   }
                };

            } else {
                eprintln!("missing required argument to --min");
                std::process::exit(1);
            }
            i += 1;

        }
        if next_arg == "--max" {
            if i == len -1{
                eprintln!("missing required argument to --max");
                std::process::exit(1);
            }
            let max_length_str = match std::env::args().nth(i+1) {
                Some(num) => num,
                None => {
                    eprintln!("missing mandatory argument for --max");
                    std::process::exit(1);
                }
            };
            max_length = match max_length_str.parse::<u32>() {
                Ok(num) => num,
                Err(e) => {
                    eprintln!("{0}",e);
                    std::process::exit(1);
                }

            };
            i += 1;

        }
        
        i += 1;
    }
    let mut bio_string = String::new();
    for _ in 0..100 {
        add_rand_nucl(&mut bio_string);
    }
    println!("{0}",bio_string);
}
fn add_rand_nucl(bio_string: &mut String) {
    let r = rand::thread_rng().gen_range(1 ..5);

    match r {
        1 => bio_string.push('A'),
        2 => bio_string.push('C'),
        3 => bio_string.push('G'),
        4 => bio_string.push('T'),
        _ => panic!("range out of bounds in add_rand_nucl")
    }

    

}

