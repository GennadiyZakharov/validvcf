use regex::Regex;
use lazy_static::lazy_static;
use crate::error_codes::VcfErrorCode;

const N_FIXED_FIELDS: usize = 9;

lazy_static! {
    static ref header_columns: Regex = Regex::new(
        r"^#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO\tFORMAT(\t\w+)+$"
    )
    .expect("invalid regex");
}

pub fn validate_vcf_cols_header(line: &str) -> Result<usize, VcfErrorCode> {
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
            Ok(samples.len())
        }
        None => {
            Err(VcfErrorCode::IncorrectHeader(line.to_string()))
        },
    }
}

fn validate_vcf_info(info_str: &str) -> Result<usize, VcfErrorCode> {
    // Check that FORMAT is not empty
    if info_str.is_empty() {
        return Err(VcfErrorCode::EmptyVcfEntry(info_str.to_string()));
    };
    // Splitting FORMAT by ':'
    let fields: Vec<&str> = info_str.split(';').collect();

    // Checking for empty fields
    if fields.iter().any(|&field| field.is_empty()) {
        return Err(VcfErrorCode::EmptyInfoEntry(info_str.to_string()));
    };
    for field in fields {
        let _code = validate_vcf_format_entry(field)?;
    };
    Ok(0)
}

fn validate_vcf_format_entry(entry_str: &str) -> Result<usize, VcfErrorCode> {
    let fields: Vec<&str> = entry_str.split('=').collect();
    if fields.len() != 2 {
        return Err(VcfErrorCode::IncorrectInfoEntry(entry_str.to_string()));
    };
    Ok(0)
}

fn validate_vcf_format(format_str: &str) -> Result<usize, VcfErrorCode> {
    // Check that FORMAT is not empty
    if format_str.is_empty() {
        return Err(VcfErrorCode::EmptyVcfEntry(format_str.to_string()));
    }
    // Splitting FORMAT by ':'
    let fields: Vec<&str> = format_str.split(':').collect();

    // Checking for empty fields
    if fields.iter().any(|&field| field.is_empty()) {
        return Err(VcfErrorCode::EmptyFormatEntry(format_str.to_string()));
    }
    Ok(fields.len())
}

fn validate_vcf_sample(sample_str: &str, n_format_entries:usize) -> Result<usize, VcfErrorCode> {
    // Check that FORMAT is not empty
    if sample_str.is_empty() {
        return Err(VcfErrorCode::EmptyVcfEntry(sample_str.to_string()));
    }
    // Splitting entries by ':'
    let fields: Vec<&str> = sample_str.split(':').collect();

    // Checking for empty fields
    if fields.iter().any(|&field| field.is_empty()) {
        return Err(VcfErrorCode::EmptySampleEntry(sample_str.to_string()));
    }
    if fields.len() != n_format_entries {
        return Err(VcfErrorCode::IncorrectSampleEntriesNumber(sample_str.to_string()));
    }
    Ok(0)
}


pub fn validate_vcf_line(line: &str, n_samples:usize) -> Result<usize, VcfErrorCode> {
    let fields: Vec<&str> = line.split("\t").collect();
    if fields.len() != N_FIXED_FIELDS + n_samples {
        return Err(VcfErrorCode::IncorrectEntriesNumber(line.to_string()));
    };
    let info = fields[7];
    let format = fields[8];
    let _code = validate_vcf_info(&info)?;
    let n_format_entries = validate_vcf_format(&format)?;
    for sample in fields[9..].iter() {
        let _code = validate_vcf_sample(&sample, n_format_entries)?;
    }
    Ok(0)

}
