use clap::Parser;
//use color_eyre::eyre::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
        static ref header_columns: Regex = Regex::new(
            r"^#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO\tFORMAT(\t\w+)+$"
        )
        .expect("invalid regex");
        static ref vcf_line: Regex = Regex::new(
            r"^\w+\t\d+\t.+\t.+\t.+\t.+\t(.+)\t(.+)\t(.+)(\t.+)+$"
        )
        .expect("invalid regex");

    }



/// validvcf: A fast and simple VCF validator.
///
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Silences all output
    #[arg(short, long, global = true)]
    quiet: bool,

    /// Path to the VCF file
    #[arg(value_name = "VCF", required = true)]
    input: String,
}

/// Placeholder VCF validator.
///
/// - Ensures the file exists.
/// - If the file ends with .gz or .bgz, attempts to open and decompress it as a gzip stream
///   and read at least one line of text. Returns 1 if gzip fails to open/decompress.
/// - If the file is uncompressed, attempts to read at least one line of text.


fn open_maybe_gzipped<P>(path: P) -> io::Result<BufReader<Box<dyn Read>>>
where
    P: AsRef<Path>,
{
    // 1️⃣ Determine whether we think the file is gzipped.
    let is_gzip = path
        .as_ref()
        .extension()
        .and_then(|e| e.to_str())
        .map(|ext| matches!(ext.to_ascii_lowercase().as_str(), "gz" | "bgz"))
        .unwrap_or(false);

    // 2️⃣ Open the file once.
    let file = File::open(path)?;

    // 3️⃣ Choose the appropriate reader.
    let inner: Box<dyn Read> = if is_gzip {
        // MultiGzDecoder handles normal gzip streams as well as concatenated
        // members (the format used by BGZF).  It also transparently passes
        // through uncompressed data if the file isn’t actually gzipped,
        // but we keep the explicit check for clarity.
        Box::new(flate2::read::MultiGzDecoder::new(file))
    } else {
        Box::new(file)
    };

    // 4️⃣ Wrap everything in a BufReader for efficient buffered reads.
    Ok(BufReader::new(inner))
}

fn validate_vcf_cols_header(line: &str) -> Result<usize, String> {
    match header_columns.captures(&line) {
        Some(caps) => {
            // `caps.get(0)` is the whole match; groups start at index 1.
            // Here we have a single capturing group that itself contains
            // several tab‑separated tokens.  We'll split that group on tabs
            // to obtain each individual token.
            let group = caps.get(1).unwrap().as_str();

            // Split on '\t' and filter out the trailing empty element
            // caused by the final tab in the source string.
            let samples: Vec<&str> = group
                .split('\t')
                .filter(|s| !s.is_empty())
                .collect();

            println!("  ✅ matches");
            //println!("  whole match:   {:?}", caps.get(0).unwrap().as_str());
            //println!("  captured group: {:?}", group);
            //println!("  sub‑tokens:     {:?}", samples);
            Result::Ok(samples.len())
        }
        None => {
            Result::Err(format!("Incorrect VCF column header line\n{}", line))
        },
    }
}
fn validate_vcf_line(line: &str) -> Result<usize, String> {
    match vcf_line.captures(&line) {
        Some(caps) => {Ok(0)}
        None => {
            Err(format!("Incorrect VCF line\n{}", line))
        }
    }
}


fn validate_vcf(vcf_path: &str) -> i32 {
    use std::path::Path;

    let path = Path::new(vcf_path);
    if ! path.exists() {
        eprintln!("Error: Input file '{}' does not exist", vcf_path);
        return 1;
    }

    let reader = match open_maybe_gzipped(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: Failed to open file '{}': {}", vcf_path, e);
            return 1;
        }
    };

    let mut is_header = true;
    let mut header: Vec<String> = Vec::new();
    let mut n_samples = 0;

    // `lines()` yields an iterator of `Result<String, io::Error>`.
    for (idx, line_res) in reader.lines().enumerate() {

        let line = match line_res {
            Ok(line) => {
                // Do whatever you need with each line.
                line
            }
            Err(e) => {
                eprintln!("Error reading line {}: {}", idx + 1, e);
                return 1;
            }
        };

        if is_header {
            if line.starts_with("##") {
                // Collecting header for futher analysis
                header.push(line);
                continue;
            } else {
                println!("Found header {}", line);
                is_header = false;
                n_samples = validate_vcf_cols_header(&line).unwrap();
                continue;
            }
        };
        match validate_vcf_line(&line) {
            Ok(_) => {continue},
            Err(e) => {
                eprintln!("Error reading line {}: {}", idx + 1, e);
                return 1;
            }
        };

    }

    0

}


fn main() {
    //color_eyre::install()?;
    let cli = Cli::parse();

    // Placeholder logic: just report the provided path
    if !cli.quiet {
        println!("Validating VCF: {}", cli.input);
    }
    let valid_status = validate_vcf(&cli.input);

    if !cli.quiet {
        if valid_status == 0 {
            println!("Validating VCF: {} → OK", cli.input);
        } else {
            println!("Validating VCF: {} → INVALID", cli.input);
        }
    }

    std::process::exit(valid_status);
}
