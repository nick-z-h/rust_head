use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

type BoxResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Cli {
    #[clap(help = "Input", default_value = "-")]
    files: Vec<String>,
    #[clap(
        help = "Print the first <LINES> line(s)",
        conflicts_with = "bytes",
        short = 'n',
        long = "lines",
        default_value = "10"
    )]
    lines: usize,
    #[clap(
        help = "Print the first <BYTES> byte(s)",
        conflicts_with = "lines",
        short = 'c',
        long = "bytes"
    )]
    bytes: Option<usize>,
}

pub fn run(cli: Cli) -> BoxResult<()> {
    let num_files = cli.files.len();
    for (file_num, filename) in cli.files.iter().enumerate() {
        match open(&filename) {
            Err(e) => eprintln!("{filename}: {e}"),
            Ok(mut buf_reader) => {
                if num_files > 1 {
                    println!("{}==> {filename} <==", if file_num > 0 { "\n" } else { "" })
                }
                if let Some(num_bytes) = cli.bytes {
                    let bytes_read = buf_reader
                        .bytes()
                        .take(num_bytes)
                        .collect::<Result<Vec<_>, _>>()?;
                    print!("{}", String::from_utf8_lossy(&bytes_read));
                } else {
                    let mut buffer = String::new();
                    for _ in 0..cli.lines {
                        let bytes = buf_reader.read_line(&mut buffer)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{buffer}");
                        buffer.clear();
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> BoxResult<Cli> {
    Ok(Cli::parse())
}

fn open(filename: &str) -> BoxResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
