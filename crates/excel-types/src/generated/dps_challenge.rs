/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct DpsChallenge {
    pub row_id: u32,
    pub player_level: u16,
    pub place_name_id: u32,
    pub place_name: RowRef<PlaceName>,
    pub icon_id: u32,
    pub order: u16,
    pub name: String,
    pub description: String,
}

impl Sheet for DpsChallenge {
    const SHEET_NAME: &'static str = "DpsChallenge";
}

impl FromExcelRow for DpsChallenge {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            player_level: single_row.columns.get(0).to_u16(),
            place_name_id: single_row.columns.get(3).to_u32(),
            place_name: RowRef::<PlaceName>::from(single_row.columns.get(3).to_u32()),
            icon_id: single_row.columns.get(4).to_u32(),
            order: single_row.columns.get(5).to_u16(),
            name: single_row.columns.get(6).to_owned_string(),
            description: single_row.columns.get(7).to_owned_string(),
        })
    }
}

