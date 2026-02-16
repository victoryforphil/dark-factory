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

fn render_pretty(command: &Command, body: &Value) -> Result<String, anyhow::Error> {
    match command {
        Command::Info { .. } => render_info_summary(body),
        _ => render_pretty_value(body),
    }
}

fn render_info_summary(body: &Value) -> Result<String, anyhow::Error> {
    let Some(data) = body.get("data").and_then(Value::as_object) else {
        return render_pretty_value(body);
    };

    let mut context = BTreeMap::new();
    context.insert(
        "Directory".to_string(),
        data.get("directory")
            .map(to_cell)
            .unwrap_or_else(|| "-".to_string()),
    );
    context.insert(
        "Locator".to_string(),
        data.get("locator")
            .map(to_cell)
            .unwrap_or_else(|| "-".to_string()),
    );

    let products = data
        .get("products")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let variants = data
        .get("variants")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    context.insert("Products".to_string(), products.len().to_string());
    context.insert("Variants".to_string(), variants.len().to_string());

    let mut sections = vec![
        "Directory Context".to_string(),
        render_key_value_table(&context),
        "Products".to_string(),
        render_info_products_table(&products),
        "Variants".to_string(),
        render_info_variants_table(&variants),
    ];

    if products.is_empty() {
        sections.push("Hint: run `dark_cli init` in this directory first.".to_string());
    }

    Ok(sections.join("\n\n"))
}

fn render_info_products_table(rows: &[Value]) -> String {
    if rows.is_empty() {
        return "No products for this locator.".to_string();
    }

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Product ID"),
        Cell::new("Display Name"),
        Cell::new("Locator"),
        Cell::new("Git Repo"),
        Cell::new("Branch"),
        Cell::new("Worktrees"),
        Cell::new("Updated"),
    ]));

    for row in rows {
        table.add_row(Row::new(vec![
            Cell::new(&id_field(row, "id")),
            Cell::new(&field(row, "displayName")),
            Cell::new(&field(row, "locator")),
            Cell::new(&deep_field(row, &["gitInfo", "repoName"])),
            Cell::new(&deep_field(row, &["gitInfo", "branch"])),
            Cell::new(&deep_field(row, &["gitInfo", "worktreeCount"])),
            Cell::new(&field(row, "updatedAt")),
        ]));
    }

    table.to_string()
}

fn render_info_variants_table(rows: &[Value]) -> String {
    if rows.is_empty() {
        return "No variants for this locator.".to_string();
    }

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Variant ID"),
        Cell::new("Product ID"),
        Cell::new("Name"),
        Cell::new("Branch"),
        Cell::new("Dirty"),
        Cell::new("Ahead/Behind"),
        Cell::new("Worktree"),
        Cell::new("Last Polled"),
    ]));

    for row in rows {
        table.add_row(Row::new(vec![
            Cell::new(&id_field(row, "id")),
            Cell::new(&id_field(row, "productId")),
            Cell::new(&field(row, "name")),
            Cell::new(&deep_field(row, &["gitInfo", "branch"])),
            Cell::new(&dirty_cell(row)),
            Cell::new(&ahead_behind_cell(row)),
            Cell::new(&worktree_cell(row)),
            Cell::new(&field(row, "gitInfoLastPolledAt")),
        ]));
    }

    table.to_string()
}

fn ahead_behind_cell(row: &Value) -> String {
    let ahead = row
        .get("gitInfo")
        .and_then(|value| value.get("status"))
        .and_then(|value| value.get("ahead"))
        .map(to_cell);
    let behind = row
        .get("gitInfo")
        .and_then(|value| value.get("status"))
        .and_then(|value| value.get("behind"))
        .map(to_cell);

    match (ahead, behind) {
        (Some(ahead), Some(behind)) => format!("{ahead}/{behind}"),
        _ => "-".to_string(),
    }
}

fn field(row: &Value, key: &str) -> String {
    row.get(key).map(to_cell).unwrap_or_else(|| "-".to_string())
}

fn id_field(row: &Value, key: &str) -> String {
    row.get(key)
        .and_then(Value::as_str)
        .map(compact_id)
        .unwrap_or_else(|| field(row, key))
}

fn compact_id(value: &str) -> String {
    if let Some(hash) = value.strip_prefix("prd_") {
        return format!("prd_{}", shorten(hash, 12));
    }

    shorten(value, 12)
}

fn shorten(value: &str, take: usize) -> String {
    if value.len() <= take {
        return value.to_string();
    }

    format!("{}...", &value[..take])
}

fn deep_field(row: &Value, path: &[&str]) -> String {
    let mut current = row;

    for key in path {
        let Some(next) = current.get(*key) else {
            return "-".to_string();
        };

        current = next;
    }

    to_cell(current)
}

fn dirty_cell(row: &Value) -> String {
    let clean = row
        .get("gitInfo")
        .and_then(|value| value.get("status"))
        .and_then(|value| value.get("clean"));

    match clean {
        Some(Value::Bool(true)) => "no".to_string(),
        Some(Value::Bool(false)) => "yes".to_string(),
        _ => "-".to_string(),
    }
}

fn worktree_cell(row: &Value) -> String {
    let is_linked = row
        .get("gitInfo")
        .and_then(|value| value.get("isLinkedWorktree"))
        .and_then(Value::as_bool);

    match is_linked {
        Some(true) => "linked".to_string(),
        Some(false) => "main".to_string(),
        None => "-".to_string(),
    }
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
