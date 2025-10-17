mod vcf_validate;
mod maybe_gz_reader;
mod error_codes;

use clap::Parser;
use std::io::{BufRead};
use crate::error_codes::VcfErrorCode;

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

fn report_error(e: VcfErrorCode) -> i32 {
    eprintln!("Error: {}", e.error_message());
    e.error_code()
}


fn validate_vcf(vcf_path: &str) -> i32 {
    use std::path::Path;

    let path = Path::new(vcf_path);
    if ! path.exists() {
        return report_error(VcfErrorCode::FileNotFound(vcf_path.to_string()))
    }

    let reader = match maybe_gz_reader::open_maybe_gzipped(path) {
        Ok(f) => f,
        Err(e) => {
            return report_error(VcfErrorCode::FileReadError(e.to_string()));
        }
    };

    let mut is_header = true;
    let mut header: Vec<String> = Vec::new();
    let mut n_samples = 0;

    // `lines()` yields an iterator of `Result<String, io::Error>`.
    for (line_number, line_res) in reader.lines().enumerate() {
        let line = match line_res {
            Ok(line) => {
                // Do whatever you need with each line.
                line
            }
            Err(e) => {
                eprintln!("I/O error on reading line {}", line_number + 1);
                return report_error(VcfErrorCode::FileReadError(e.to_string()));
            }
        };

        if is_header {
            if line.starts_with("##") {
                // Collecting header for further analysis
                header.push(line);
                continue;
            } else {
                println!("Found header at line {}", line_number + 1);
                is_header = false;
                match vcf_validate::validate_vcf_cols_header(&line) {
                    Ok(samples) => n_samples = samples,
                    Err(e) => {
                        eprintln!("Error validating line {}", line_number + 1);
                        eprintln!("    {}", line);
                        return report_error(e);
                    }
                };
                continue;
            }
        };
        match vcf_validate::validate_vcf_line(&line, n_samples) {
            Ok(_) => {continue},
            Err(e) => {
                eprintln!("Error validating line {}", line_number + 1);
                eprintln!("    {}", line);
                return report_error(e);
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
