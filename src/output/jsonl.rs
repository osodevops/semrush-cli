pub fn render(data: &[serde_json::Value]) -> String {
    data.iter()
        .filter_map(|v| serde_json::to_string(v).ok())
        .collect::<Vec<_>>()
        .join("\n")
}
