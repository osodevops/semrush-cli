use crate::api::columns;
use crate::error::AppError;

/// Parse a Semrush CSV response into a Vec of JSON objects with human-readable keys.
///
/// Semrush v3 responses are semicolon-delimited CSV with a header row using
/// column codes (Ph, Nq, Cp, etc.). This parser converts them to JSON objects
/// with human-readable field names and appropriate types.
pub fn parse_csv_response(body: &str) -> Result<Vec<serde_json::Value>, AppError> {
    let body = body.trim();
    if body.is_empty() {
        return Ok(vec![]);
    }

    // Check for API error responses (they come as plain text)
    if body.starts_with("ERROR") {
        return Err(parse_api_error(body));
    }

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .flexible(true)
        .from_reader(body.as_bytes());

    let headers: Vec<String> = reader
        .headers()
        .map_err(|e| AppError::ParseError {
            message: format!("Failed to read CSV headers: {e}"),
        })?
        .iter()
        .map(|h| columns::to_human(h.trim()).to_string())
        .collect();

    let mut rows = Vec::new();
    for result in reader.records() {
        let record = result?;
        let mut row = serde_json::Map::new();
        for (i, field) in record.iter().enumerate() {
            if let Some(header) = headers.get(i) {
                row.insert(header.clone(), coerce_value(field.trim()));
            }
        }
        rows.push(serde_json::Value::Object(row));
    }

    Ok(rows)
}

/// Try to coerce a string value into an appropriate JSON type.
fn coerce_value(s: &str) -> serde_json::Value {
    if s.is_empty() || s == "--" {
        return serde_json::Value::Null;
    }

    // Try integer
    if let Ok(n) = s.parse::<i64>() {
        return serde_json::Value::Number(serde_json::Number::from(n));
    }

    // Try float
    if let Ok(f) = s.parse::<f64>() {
        if let Some(n) = serde_json::Number::from_f64(f) {
            return serde_json::Value::Number(n);
        }
    }

    // Trends data (comma-separated floats)
    if s.contains(',') && s.split(',').all(|p| p.trim().parse::<f64>().is_ok()) {
        let values: Vec<serde_json::Value> = s
            .split(',')
            .filter_map(|p| {
                p.trim()
                    .parse::<f64>()
                    .ok()
                    .and_then(serde_json::Number::from_f64)
                    .map(serde_json::Value::Number)
            })
            .collect();
        return serde_json::Value::Array(values);
    }

    serde_json::Value::String(s.to_string())
}

fn parse_api_error(body: &str) -> AppError {
    let msg = body
        .strip_prefix("ERROR ")
        .unwrap_or(body)
        .trim()
        .to_string();

    if msg.contains("NOTHING FOUND") {
        return AppError::ApiError {
            status_code: 200,
            message: "No data found for the given parameters".to_string(),
        };
    }

    let code_end = msg.find(" :: ").unwrap_or(msg.len());
    let error_code = &msg[..code_end];

    match error_code {
        "50" | "100" => AppError::AuthFailed { message: msg },
        "120" => AppError::InsufficientUnits { message: msg },
        "130" | "140" => AppError::InvalidParams { message: msg },
        _ => AppError::ApiError {
            status_code: 200,
            message: msg,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv_basic() {
        let csv = "Ph;Nq;Cp\nrust programming;12100;2.45\nrust language;8100;1.80";
        let rows = parse_csv_response(csv).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0]["keyword"], "rust programming");
        assert_eq!(rows[0]["search_volume"], 12100);
        assert_eq!(rows[0]["cpc"], 2.45);
    }

    #[test]
    fn test_parse_csv_empty() {
        let rows = parse_csv_response("").unwrap();
        assert!(rows.is_empty());
    }

    #[test]
    fn test_parse_csv_error() {
        let result = parse_csv_response("ERROR 50 :: INVALID API KEY");
        assert!(result.is_err());
    }

    #[test]
    fn test_coerce_trends() {
        let val = coerce_value("0.82,0.91,1.0,0.95");
        assert!(val.is_array());
        assert_eq!(val.as_array().unwrap().len(), 4);
    }

    #[test]
    fn test_coerce_null() {
        assert_eq!(coerce_value("--"), serde_json::Value::Null);
        assert_eq!(coerce_value(""), serde_json::Value::Null);
    }
}
