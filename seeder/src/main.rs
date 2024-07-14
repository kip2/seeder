mod json_utils;

use json_utils::*;

fn main() {
    let (table_columns, table_row) = read_json_file("data.json");

    let tf = validate_row_column_length(&table_columns, &table_row);
}
