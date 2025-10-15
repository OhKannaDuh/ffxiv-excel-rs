/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct World {
    pub row_id: u32,
    pub internal_name: String,
    pub name: String,
    pub region: u8,
    pub user_type: u8,
    pub data_center_id: u32,
    pub data_center: RowRef<WorldDCGroupType>,
    pub is_public: bool,
}

impl Sheet for World {
    const SHEET_NAME: &'static str = "World";
}

impl FromExcelRow for World {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            internal_name: single_row.columns.get(0).to_owned_string(),
            name: single_row.columns.get(1).to_owned_string(),
            region: single_row.columns.get(2).to_u8(),
            user_type: single_row.columns.get(3).to_u8(),
            data_center_id: single_row.columns.get(4).to_u32(),
            data_center: RowRef::<WorldDCGroupType>::from(single_row.columns.get(4).to_u32()),
            is_public: single_row.columns.get(5).to_bit(0),
        })
    }
}

