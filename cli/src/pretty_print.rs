use comfy_table::presets::UTF8_BORDERS_ONLY;
use comfy_table::{Cell, Color, ContentArrangement, Row, Table};

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

pub fn cc(color: Color, val: &str) -> Cell {
    Cell::new(val).fg(color)
}
