use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn valid_vcf() {
    // Path to a known test VCF in the repository
    let test_vcf = "test_vcf/HG001_GRCh38_1_22_v4.2.1_benchmark.vcf.gz";

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
