use rand;
use rand::Rng;
use std::env;

fn main() {
    let mut min_length:u32 = 100;
    let mut max_length:u32 = 200;
    let mut n = 1;
    let mut seq_name = "seq";
    // process command line arguments
    let mut i:usize = 1;
    let len = env::args().len();
    while i < len  {
        let next_arg = env::args().nth(i).unwrap();
        if next_arg == String::from("--min") {
            if i == len-1 {
                eprintln!("missing mandatory argument to --min");
                std::process::exit(1);
            }
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

        if next_arg == "-n" || next_arg == "--num_seqs" {
            if i == len -1 {
                eprintln!("missing mandatory argument for --num_seqs");
                std::process::exit(1);
            }
            let n_str = match env::args().nth(i+1) {
                Some(num) => num,
                None => {
                    eprintln!("missing mandatory argument for --num_seqs");
                    std::process::exit(1);
                }
            };
            n = match n_str.parse::<u32>() {
                Ok(num) => num,
                Err(e) => {
                    eprintln!("{0}",e);
                    std::process::exit(1);
                }
            };    
            i += 1;
        }
        
        i += 1;
    } // end while loop to read command line args
    let seqs = create_dna_seqs(n,min_length,max_length);
    for (i,seq) in seqs.iter().enumerate() {
        println!("> {0}{1}",seq_name,i+1);
        println!("{0}", seq);
    }
} // end main
fn add_rand_nucl(bio_string: &mut String) {
    let r = rand::thread_rng().gen_range(1 ..5);
    match r {
        1 => bio_string.push('A'),
        2 => bio_string.push('C'),
        3 => bio_string.push('G'),
        4 => bio_string.push('T'),
        _ => panic!("range out of bounds in add_rand_nucl")
    };
}

fn create_dna_seq(length:u32) -> String {
    let mut bio_string = String::new();
    for _ in 0..length {
        add_rand_nucl(&mut bio_string);
    }
    bio_string
}

fn create_dna_seqs(n:u32, min:u32,max:u32) -> Vec<String> {
    let mut seqs = Vec::new();
    for _ in 0..n {
        let length = rand::thread_rng().gen_range(min..max+1);
        seqs.push(create_dna_seq(length));
    };
    seqs
}
