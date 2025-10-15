/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MainCommand {
    pub row_id: u32,
    pub icon_id: u32,
    pub category: u8,
    pub main_command_category_id: u32,
    pub main_command_category: RowRef<MainCommandCategory>,
    pub sort_id: i8,
    pub name: String,
    pub description: String,
}

impl Sheet for MainCommand {
    const SHEET_NAME: &'static str = "MainCommand";
}

impl FromExcelRow for MainCommand {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            icon_id: single_row.columns.get(0).to_u32(),
            category: single_row.columns.get(1).to_u8(),
            main_command_category_id: single_row.columns.get(2).to_u32(),
            main_command_category: RowRef::<MainCommandCategory>::from(single_row.columns.get(2).to_u32()),
            sort_id: single_row.columns.get(3).to_i8(),
            name: single_row.columns.get(5).to_owned_string(),
            description: single_row.columns.get(6).to_owned_string(),
        })
    }
}

