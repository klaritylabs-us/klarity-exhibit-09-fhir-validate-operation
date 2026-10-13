use std::{env, fs, process::ExitCode};
use fhir_validate_op::validate_operation_json;

fn main() -> ExitCode {
    let path = env::args().nth(1).expect("usage: validate-operation <file>");
    let json = fs::read_to_string(path).unwrap();
    let report = validate_operation_json(&json).unwrap();
    println!("{}", serde_json::to_string_pretty(&report.issues).unwrap());
    ExitCode::from(if report.is_valid() { 0 } else { 1 })
}
