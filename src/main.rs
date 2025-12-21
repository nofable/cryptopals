use clap::Parser;
use cryptopals::vigenere;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_file: String,

    #[arg(short, long, default_value_t = 40)]
    max_hamming: usize,

    #[arg(short = 'a', long, default_value_t = 4)]
    hamming_sample_size: usize,

    #[arg(short, long, default_value_t = 3)]
    no_of_results: usize,
}

fn main() {
    let args = Args::parse();
    match vigenere(
        &args.input_file,
        args.max_hamming,
        args.hamming_sample_size,
        args.no_of_results,
    ) {
        Ok(results) => {
            for result in results {
                println!("{}", result.key);
                println!("{}", result.plaintext);
                println!()
            }
        }

        Err(err) => {
            println!("{:?}", err);
        }
    }
}
