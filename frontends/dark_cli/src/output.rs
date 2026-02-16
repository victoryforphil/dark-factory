use std::collections::{BTreeMap, BTreeSet};

use prettytable::{Cell, Row, Table};
use serde_json::Value;

use crate::cli::{Command, OutputFormat};

pub fn render(
    format: OutputFormat,
    command: &Command,
    body: &Value,
) -> Result<String, anyhow::Error> {
    match format {
        OutputFormat::Json => Ok(serde_json::to_string_pretty(body)?),
        OutputFormat::Toml => Ok(toml::to_string_pretty(body)?),
        OutputFormat::Pretty => render_pretty(command, body),
    }
}

fn render_pretty(_command: &Command, body: &Value) -> Result<String, anyhow::Error> {
    render_pretty_value(body)
}

fn render_pretty_value(value: &Value) -> Result<String, anyhow::Error> {
    match value {
        Value::Array(rows) => render_array_table(rows),
        Value::Object(object) => {
            if let Some(rows) = object.get("data").and_then(Value::as_array) {
                let mut sections: Vec<String> = Vec::new();

                let metadata = collect_metadata_rows(object);
                if !metadata.is_empty() {
                    sections.push(render_key_value_table(&metadata));
                }

                sections.push(render_array_table(rows)?);
                return Ok(sections.join("\n\n"));
            }

            let flattened = flatten_value_map(value);
            Ok(render_key_value_table(&flattened))
        }
        _ => Ok(to_cell(value)),
    }
}

fn render_array_table(rows: &[Value]) -> Result<String, anyhow::Error> {
    if rows.is_empty() {
        return Ok("No rows.".to_string());
    }

    if rows.iter().all(Value::is_object) {
        return Ok(render_object_array_table(rows));
    }

    Ok(render_scalar_array_table(rows))
}

fn render_object_array_table(rows: &[Value]) -> String {
    let mut flattened_rows: Vec<BTreeMap<String, String>> = Vec::with_capacity(rows.len());
    let mut headers = BTreeSet::new();

    for row in rows {
        let flattened = flatten_value_map(row);

        for key in flattened.keys() {
            headers.insert(key.clone());
        }

        flattened_rows.push(flattened);
    }

    let ordered_headers: Vec<String> = headers.into_iter().collect();

    let mut table = Table::new();
    table.add_row(Row::new(
        ordered_headers
            .iter()
            .map(|header| Cell::new(header))
            .collect(),
    ));

    for row in flattened_rows {
        let cells: Vec<Cell> = ordered_headers
            .iter()
            .map(|header| {
                let value = row.get(header).map(String::as_str).unwrap_or("-");
                Cell::new(value)
            })
            .collect();
        table.add_row(Row::new(cells));
    }

    table.to_string()
}

fn render_scalar_array_table(rows: &[Value]) -> String {
    let mut table = Table::new();
    table.add_row(Row::new(vec![Cell::new("Value")]));

    for row in rows {
        table.add_row(Row::new(vec![Cell::new(&to_cell(row))]));
    }

    table.to_string()
}

fn collect_metadata_rows(object: &serde_json::Map<String, Value>) -> BTreeMap<String, String> {
    let mut result = BTreeMap::new();

    for (key, value) in object {
        if key == "data" {
            continue;
        }

        flatten_value(key, value, &mut result);
    }

    result
}

fn flatten_value_map(value: &Value) -> BTreeMap<String, String> {
    let mut result = BTreeMap::new();
    flatten_value("", value, &mut result);
    result
}

fn flatten_value(prefix: &str, value: &Value, output: &mut BTreeMap<String, String>) {
    match value {
        Value::Object(object) => {
            if object.is_empty() {
                let key = if prefix.is_empty() {
                    "value".to_string()
                } else {
                    prefix.to_string()
                };
                output.insert(key, "{}".to_string());
                return;
            }

            for (key, nested_value) in object {
                let next_prefix = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{prefix}.{key}")
                };

                flatten_value(&next_prefix, nested_value, output);
            }
        }
        _ => {
            let key = if prefix.is_empty() {
                "value".to_string()
            } else {
                prefix.to_string()
            };

            output.insert(key, to_cell(value));
        }
    }
}

fn render_key_value_table(rows: &BTreeMap<String, String>) -> String {
    let mut table = Table::new();
    table.add_row(Row::new(vec![Cell::new("Field"), Cell::new("Value")]));

    for (field, value) in rows {
        table.add_row(Row::new(vec![Cell::new(field), Cell::new(value)]));
    }

    table.to_string()
}

fn to_cell(value: &Value) -> String {
    match value {
        Value::Null => "-".to_string(),
        Value::String(value) => value.clone(),
        Value::Number(value) => value.to_string(),
        Value::Bool(value) => value.to_string(),
        _ => value.to_string(),
    }
}
