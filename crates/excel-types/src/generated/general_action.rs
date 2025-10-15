/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GeneralAction {
    pub row_id: u32,
    pub name: String,
    pub description: String,
    pub action_id: u32,
    pub action: RowRef<Action>,
    pub unlock_link: u16,
    pub recast: u8,
    pub ui_priority: u8,
    pub icon_id: u32,
}

impl Sheet for GeneralAction {
    const SHEET_NAME: &'static str = "GeneralAction";
}

impl FromExcelRow for GeneralAction {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            description: single_row.columns.get(1).to_owned_string(),
            action_id: single_row.columns.get(3).to_u32(),
            action: RowRef::<Action>::from(single_row.columns.get(3).to_u32()),
            unlock_link: single_row.columns.get(4).to_u16(),
            recast: single_row.columns.get(5).to_u8(),
            ui_priority: single_row.columns.get(6).to_u8(),
            icon_id: single_row.columns.get(7).to_u32(),
        })
    }
}

