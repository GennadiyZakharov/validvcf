use assert_cmd::prelude::*;
use std::process::Command;

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
        .failure();
}

#[test]
fn missed_info() {
    // Path to a known test VCF in the repository
    let test_vcf = "test_vcf/HG001_missed_info.vcf";

    Command::cargo_bin("validvcf")
        .expect("binary should build")
        .arg("--quiet")
        .arg(test_vcf)
        .assert()
        .failure();
}

#[test]
fn corrupted_format() {
    // Path to a known test VCF in the repository
    let test_vcf = "test_vcf/HG001_corrupted_format.vcf";

    Command::cargo_bin("validvcf")
        .expect("binary should build")
        .arg("--quiet")
        .arg(test_vcf)
        .assert()
        .success();
}

