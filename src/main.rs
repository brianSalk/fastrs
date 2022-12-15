use rand;
use rand::Rng;
use std::env;

fn main() {
    let mut flags = Flags::new();
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
            flags.min = match min_length_str.parse::<u32>() {
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
            flags.max = match max_length_str.parse::<u32>() {
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
            flags.n = match n_str.parse::<u32>() {
                Ok(num) => num,
                Err(e) => {
                    eprintln!("{0}",e);
                    std::process::exit(1);
                }
            };    
            i += 1;
        }

        if next_arg == "-t" || next_arg == "--title" {
            if i == len - 1 {
                eprintln!("missing mandatory argument to --title");
                std::process::exit(1);
            }
            flags.title = match env::args().nth(i+1) {
                Some(title) => title,
                None => {
                    eprintln!("missing mandatory argument to --title");
                    std::process::exit(1);
                }
            };
            i += 1;
        }
        
        if next_arg == "--dna" {
            flags.is_dna = true;
        }

        if next_arg == "--prot" {
            flags.is_dna = false;
        }
        i += 1;
    } // end while loop to read command line args
    // exit with error if min is greater than max
    if flags.min > flags.max {
        eprintln!("argument to --min must be <= argument to --max");
        std::process::exit(1);
    }

    let seqs = if flags.is_dna {create_dna_seqs(flags.n,flags.min,flags.max)} else {create_prot_seqs(flags.n, flags.min, flags.max)};

    for (i,seq) in seqs.iter().enumerate() {
        println!("> {0}{1}",flags.title,i+1);
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

fn add_rand_prot(bio_string: &mut String) {
    let r = rand::thread_rng().gen_range(1 ..26);
    match r {
        1 => bio_string.push('A'),
        2 => bio_string.push('B'),
        3 => bio_string.push('C'),
        4 => bio_string.push('D'),
        5 => bio_string.push('E'),
        6 => bio_string.push('F'),
        7 => bio_string.push('G'),
        8 => bio_string.push('H'),
        9 => bio_string.push('I'),
        10 => bio_string.push('J'),
        11 => bio_string.push('K'),
        12 => bio_string.push('L'),
        13 => bio_string.push('M'),
        14 => bio_string.push('N'),
        15 => bio_string.push('O'),
        16 => bio_string.push('P'),
        17 => bio_string.push('Q'),
        18 => bio_string.push('R'),
        19 => bio_string.push('S'),
        20 => bio_string.push('T'),
        21 => bio_string.push('U'),
        22 => bio_string.push('V'),
        23 => bio_string.push('W'),
        24 => bio_string.push('Y'),
        25 => bio_string.push('Z'),
        _ => panic!("range out of bounds in add_rand_nucl")
    };
}


fn create_seq(length:u32, add_rand: &mut dyn FnMut(&mut String)) -> String {
    let mut bio_string = String::new();
    for _ in 0..length {
        add_rand(&mut bio_string);
    }
    bio_string
}

fn create_dna_seqs(n:u32, min:u32,max:u32) -> Vec<String> {
    let mut seqs = Vec::new();
    for _ in 0..n {
        let length = rand::thread_rng().gen_range(min..max+1);
        seqs.push(create_seq(length,& mut add_rand_nucl));
    };
    seqs
}

fn create_prot_seqs(n:u32, min:u32,max:u32) -> Vec<String> {
    let mut seqs = Vec::new();
    for _ in 0..n {
        let length = rand::thread_rng().gen_range(min..max+1);
        seqs.push(create_seq(length,& mut add_rand_prot));
    };
    seqs
}

// group all flags into a struct for conveniance and easier maintainability 
struct Flags {
    max: u32,
    min: u32,
    n: u32,
    title: String,
    is_dna: bool,

}
impl Flags {
    fn new() -> Self {
        return Self {
            min: 100,
            max: 200,
            n: 10,
            title: String::from("seq"),
            is_dna: true
        }
    }
}
