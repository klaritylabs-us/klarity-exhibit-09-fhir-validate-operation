use serde_json::Value;
use crate::outcome::{ValidationIssue, ValidationReport};

pub fn validate_operation_json(json: &str) -> Result<ValidationReport, String> {
    let v: Value = serde_json::from_str(json).map_err(|e| format!("JSON: {e}"))?;
    let mut r = ValidationReport::ok();
    let rt = v.get("resourceType").and_then(|x| x.as_str());
    if rt != Some("Parameters") && rt != Some("Patient") && rt != Some("Observation") {
        r.push(ValidationIssue::error("resourceType", "expected Parameters wrapper or resource body for $validate", "resource-type"));
    }
    if rt == Some("Parameters") {
        let has_resource = v.get("parameter").and_then(|p| p.as_array()).is_some_and(|a| {
            a.iter().any(|p| p.get("name").and_then(|n| n.as_str()) == Some("resource"))
        });
        if !has_resource {
            r.push(ValidationIssue::error("Parameters.parameter", "validate call needs parameter name resource", "validate-resource-param"));
        }
    }
    Ok(r)
}
