use clap::Parser;
use color_eyre::eyre::Result;

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
/// Returns code `0` if the file exists, `1` otherwise.
/// Replace the body with real VCF validation later.
fn validate_vcf(vcf_path: &str) -> i32 {
    if ! std::path::Path::new(vcf_path).exists() {
        eprintln!("Error: Input file '{}' does not exist", vcf_path);
        return 1;
    };
    
    0
}


fn main() -> Result<()> {
    color_eyre::install()?;
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
