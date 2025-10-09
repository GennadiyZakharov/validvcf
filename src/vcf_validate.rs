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


pub fn validate_vcf_cols_header(line: &str) -> Result<usize, String> {
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
            Ok(samples.len())
        }
        None => {
            Err(format!("Incorrect VCF column header line\n{}", line))
        },
    }
}
pub fn validate_vcf_line(line: &str) -> Result<usize, String> {
    match vcf_line.captures(&line) {
        Some(caps) => {Ok(0)}
        None => {
            Err(format!("Incorrect VCF line\n{}", line))
        }
    }
}
