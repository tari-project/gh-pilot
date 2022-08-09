use comfy_table::presets::UTF8_BORDERS_ONLY;
use comfy_table::{Cell, Color, ContentArrangement, Row, Table};
use gh_pilot::ghp_api::models::Label;
use hex;

pub fn pretty_table(header_label: &str, header_value: &str) -> Table {
    let mut table = Table::new();
    table
        .load_preset(UTF8_BORDERS_ONLY)
        .set_content_arrangement(ContentArrangement::Dynamic);
    let mut name_row = Row::new();
    name_row
        .add_cell(cc(Color::Green, header_label))
        .add_cell(cc(Color::Green, header_value));
    table
}

pub fn add_labels(table: &mut Table, labels: &[Label]) {
    table.add_row(&["Labels"]);
    labels.iter().for_each(|label| {
        let mut row = Row::new();
        let color = color_from_hex(label.color.as_str());
        let desc = label
            .description
            .as_ref()
            .map(|d| d.as_str())
            .unwrap_or_default();
        row.add_cell(cc(color, label.name.as_str()))
            .add_cell(Cell::new(desc));
        table.add_row(row);
    })
}

pub fn cc(color: Color, val: &str) -> Cell {
    Cell::new(val).fg(color)
}

pub fn color_from_hex(hex_code: &str) -> Color {
    if hex_code.len() < 6 {
        return Color::White;
    }
    hex::decode(hex_code).map(|v| {
        Color::from((v[0], v[1], v[2]))
    }).unwrap_or_else(|_| Color::White)
}