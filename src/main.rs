use rand;
use rand::Rng;
use std::env;

fn main() {
    let mut flags = Flags::new();
    // process command line arguments
    let mut i:usize = 1;
    let len = env::args().len();
    // if -h or --help show up in the arguments, display help and exit successfully
    {
        let mut prev_arg = String::new();
        for arg in env::args() {
            // allow user to user --help as argument for --title.
            if (prev_arg != "--title" && prev_arg != "-t") && arg == "-h" || arg == "--help" {
                Flags::help();
                std::process::exit(0);
            }
            prev_arg = arg;
        }
    }
    while i < len  {
        let next_arg = env::args().nth(i).unwrap();
        if next_arg == String::from("--min") {
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
        // FIX ME: change this to seq instead of title.
        if next_arg == "-t" || next_arg == "--title" {
            flags.seq = match env::args().nth(i+1) {
                Some(title) => title,
                None => {
                    eprintln!("missing mandatory argument to --title");
                    std::process::exit(1);
                }
            };
            i += 1;
        }
        
        if next_arg == "--type" {
            flags.bio_type = match std::env::args().nth(i+1) {
                Some(s) => {
                    match s.as_str() {
                        "dna"=> BioType::Dna,
                        "rna" => BioType::Rna,
                        "prot" => BioType::Prot,
                        &_ => {
                            eprintln!("invalid argument to --type: {0}",std::env::args().nth(i+1).unwrap());
                            std::process::exit(1);
                        }
                    }
                },
                None => {
                    eprintln!("missing mandatory argument to --type");
                    std::process::exit(1);
                }
            };
        }
        if next_arg == "--custom_charset" {
            flags.custom_charset = match std::env::args().nth(i+1) {
                Some(s) => s,
                None => {
                    println!("missing mandatory argument to --custom_charset");
                    std::process::exit(1);
                }
            };
            i+=1;
        }
        if next_arg == "--length" || next_arg == "-l" {
            flags.length = match std::env::args().nth(i+1) {
                Some(n) => {
                    match n.parse::<u32>() {
                        Ok(n) => n, 
                        Err(e) => {
                            println!("{0}",e);
                            std::process::exit(1);
                        }
                    }
                }
                None => {
                    println!("missing mandatory argument to --length");
                    std::process::exit(1);
                }
            }
        }

        i += 1;
    } // end while loop to read command line args
    // exit with error if min is greater than max
    if flags.min > flags.max {
        eprintln!("argument to --min must be <= argument to --max");
        std::process::exit(1);
    }
    if flags.length > 0 {
        flags.min = flags.length;
        flags.max = flags.length;
    }
    let seqs;
    if flags.custom_charset == "" {
        seqs = match flags.bio_type {
            BioType::Dna => create_seqs(flags.n, flags.min, flags.max, vec!['A','C','G','T']),
            BioType::Rna => create_seqs(flags.n, flags.min, flags.max, vec!['A','C','G','U']),
            BioType::Prot => {
                let mut prot_vec: Vec<char> = ('A'..'W').into_iter().collect();
                prot_vec.push('X');
                prot_vec.push('Z');
                create_seqs(flags.n, flags.min, flags.max, prot_vec)
            }
        };
    } else {
        seqs = create_seqs(flags.n, flags.min, flags.max, flags.custom_charset.chars().collect() );
    }

    for (i,seq) in seqs.iter().enumerate() {
        println!("> {0}{1}",flags.seq,i+1);
        println!("{0}", seq);
    }
} // end main



fn create_each_seq(length:u32, letters: &Vec<char>) -> String {
    let mut bio_string = String::new();
    for _ in 0..length {
        let r = rand::thread_rng().gen_range(0..letters.len());
        bio_string.push(letters[r]);
    }
    bio_string
}

fn create_seqs(n:u32, min:u32,max:u32, letters: Vec<char>) -> Vec<String> {
    let mut seqs = Vec::new();
    for _ in 0..n {
        let length = rand::thread_rng().gen_range(min..max+1);
        seqs.push(create_each_seq(length,&letters));
    };
    seqs
}


// group all flags into a struct for conveniance and easier maintainability 
struct Flags {
    max: u32,
    min: u32,
    n: u32,
    length: u32,
    seq: String,
    bio_type: BioType,
    custom_charset: String

}
impl Flags {
    fn new() -> Self {
        return Self {
            min: 100,
            max: 200,
            n: 10,
            length: 0,
            seq: String::from("seq"),
            bio_type: BioType::Dna,
            custom_charset: String::new()
        }
    }
    fn help() {
        println!("create_fasta is a command line tool that creates randomly generated fasta files");
        println!("==========================================================================\n");
        println!("--length, -l: specify length of each sequence in fasta. Overrides --min and --max");
        println!("--min: minimum length of a randomly generated sequence.(ignored if -l or --length is also used)");
        println!("--max: maximum length of a randomly generated sequence.(ignored if -l or --length is also used)");
        println!("--num_seqs, -n: specify number of sequences to generate.");
        println!("--out, -o: outfile to print fasta file to.  (print to stdout if nothing is specified)");
        println!("--seq: base name of the sequence, if multiple sequences are generated, ascending integers starting at 1 will be appended to the base name");
        println!("--type: specify the type of fasta file to create, (valid options are 'dna', 'rna', and 'prot')");
        println!("--cust_charset: specify your own character set to use.  Overrides value of --type");

    }
}
enum BioType {
    Dna,Rna,Prot
}
