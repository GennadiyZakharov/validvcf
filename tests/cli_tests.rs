use assert_cmd::prelude::*;
use std::process::Command;
use validvcf::error_codes::VcfErrorCode;

#[test]
fn valid_vcf() {
    // Path to a known test VCF in the repository
    let test_vcf = "test_vcf/HG001_head.vcf";

    Command::cargo_bin("validvcf")
        .expect("binary should build")
        .arg("--quiet")
        .arg(test_vcf)
        .assert()
        .success();
}

#[test]
fn valid_vcf_bgz() {
    // Path to a known test VCF in the repository
    let test_vcf = "test_vcf/HG001_head.vcf.bgz";

    Command::cargo_bin("validvcf")
        .expect("binary should build")
        .arg("--quiet")
        .arg(test_vcf)
        .assert()
        .success();
}

#[test]
fn non_exist_vcf() {
    // Path to a known test VCF in the repository
    let test_vcf = "test_vcf/non_existing_file.vcf.gz";

    Command::cargo_bin("validvcf")
        .expect("binary should build")
        .arg("--quiet")
        .arg(test_vcf)
        .assert()
        .failure()
        .code(VcfErrorCode::FileNotFound("non_existing_file.vcf.gz".to_string()).error_code());
}

#[test]
fn missed_info_empty() {
    // Path to a known test VCF in the repository
    let test_vcf = "test_vcf/HG001_missed_info_empty.vcf";

    Command::cargo_bin("validvcf")
        .expect("binary should build")
        .arg("--quiet")
        .arg(test_vcf)
        .assert()
        .failure()
        .code(VcfErrorCode::EmptyVcfEntry("".to_string()).error_code());
}

#[test]
fn missed_info_point() {
    // Path to a known test VCF in the repository
    let test_vcf = "test_vcf/HG001_missed_info_point.vcf";

    Command::cargo_bin("validvcf")
        .expect("binary should build")
        .arg("--quiet")
        .arg(test_vcf)
        .assert()
        .success();
}

#[test]
fn corrupted_format() {
    // Path to a known test VCF in the repository
    let test_vcf = "test_vcf/HG001_corrupted_format.vcf.gz";

    Command::cargo_bin("validvcf")
        .expect("binary should build")
        .arg("--quiet")
        .arg(test_vcf)
        .assert()
        .failure()
        .code(VcfErrorCode::EmptyFormatEntry("".to_string()).error_code());
}

#[test]
fn corrupted_sample() {
    // Path to a known test VCF in the repository
    let test_vcf = "test_vcf/HG001_corrupted_sample.vcf.gz";

    Command::cargo_bin("validvcf")
        .expect("binary should build")
        .arg("--quiet")
        .arg(test_vcf)
        .assert()
        .failure()
        .code(VcfErrorCode::EmptySampleEntry("".to_string()).error_code());
}

#[test]
fn corrupted_sample_entries() {
    // Path to a known test VCF in the repository
    let test_vcf = "test_vcf/HG001_corrupted_sample.vcf.gz";

    Command::cargo_bin("validvcf")
        .expect("binary should build")
        .arg("--quiet")
        .arg(test_vcf)
        .assert()
        .failure()
        .code(VcfErrorCode::EmptySampleEntry("".to_string()).error_code());
}

#[test]
fn incorrect_format() {
    // Path to a known test VCF in the repository
    let test_vcf = "test_vcf/HG001_incorrect_info.vcf.gz";

    Command::cargo_bin("validvcf")
        .expect("binary should build")
        .arg("--quiet")
        .arg(test_vcf)
        .assert()
        .failure();
}