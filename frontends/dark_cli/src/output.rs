use prettytable::{Cell, Row, Table};
use serde_json::Value;

use crate::cli::{
    Command, OpencodeAction, OpencodeSessionsAction, OutputFormat, ProductsAction, VariantsAction,
};

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
        Command::Products(products) => match products.action {
            ProductsAction::List { .. } => render_products_table(body),
            _ => Ok(serde_json::to_string_pretty(body)?),
        },
        Command::Variants(variants) => match variants.action {
            VariantsAction::List { .. } => render_variants_table(body),
            _ => Ok(serde_json::to_string_pretty(body)?),
        },
        Command::Opencode(opencode) => match &opencode.action {
            OpencodeAction::Sessions(session_command) => match session_command.action {
                OpencodeSessionsAction::List { .. } => render_sessions_table(body),
                _ => Ok(serde_json::to_string_pretty(body)?),
            },
            _ => Ok(serde_json::to_string_pretty(body)?),
        },
        _ => Ok(serde_json::to_string_pretty(body)?),
    }
}

fn render_products_table(body: &Value) -> Result<String, anyhow::Error> {
    let Some(rows) = body.get("data").and_then(Value::as_array) else {
        return Ok(serde_json::to_string_pretty(body)?);
    };

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("ID"),
        Cell::new("Display Name"),
        Cell::new("Locator"),
        Cell::new("Created"),
    ]));

    for row in rows {
        table.add_row(Row::new(vec![
            Cell::new(&field(row, "id")),
            Cell::new(&field(row, "displayName")),
            Cell::new(&field(row, "locator")),
            Cell::new(&field(row, "createdAt")),
        ]));
    }

    Ok(table.to_string())
}

fn render_sessions_table(body: &Value) -> Result<String, anyhow::Error> {
    let Some(rows) = body.get("data").and_then(Value::as_array) else {
        return Ok(serde_json::to_string_pretty(body)?);
    };

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("ID"),
        Cell::new("Title"),
        Cell::new("Slug"),
        Cell::new("Directory"),
        Cell::new("Updated"),
    ]));

    for row in rows {
        table.add_row(Row::new(vec![
            Cell::new(&field(row, "id")),
            Cell::new(&field(row, "title")),
            Cell::new(&field(row, "slug")),
            Cell::new(&field(row, "directory")),
            Cell::new(&nested_field(row, "time", "updated")),
        ]));
    }

    Ok(table.to_string())
}

fn render_variants_table(body: &Value) -> Result<String, anyhow::Error> {
    let Some(rows) = body.get("data").and_then(Value::as_array) else {
        return Ok(serde_json::to_string_pretty(body)?);
    };

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("ID"),
        Cell::new("Product ID"),
        Cell::new("Name"),
        Cell::new("Locator"),
        Cell::new("Created"),
    ]));

    for row in rows {
        table.add_row(Row::new(vec![
            Cell::new(&field(row, "id")),
            Cell::new(&field(row, "productId")),
            Cell::new(&field(row, "name")),
            Cell::new(&field(row, "locator")),
            Cell::new(&field(row, "createdAt")),
        ]));
    }

    Ok(table.to_string())
}

fn field(row: &Value, key: &str) -> String {
    row.get(key).map(to_cell).unwrap_or_else(|| "-".to_string())
}

fn nested_field(row: &Value, parent: &str, key: &str) -> String {
    row.get(parent)
        .and_then(Value::as_object)
        .and_then(|object| object.get(key))
        .map(to_cell)
        .unwrap_or_else(|| "-".to_string())
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
