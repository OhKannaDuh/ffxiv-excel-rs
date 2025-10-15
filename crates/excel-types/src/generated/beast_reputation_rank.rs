/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BeastReputationRank {
    pub row_id: u32,
    pub required_reputation: u16,
    pub name: String,
    pub allied_names: String,
    pub color_id: u32,
    pub color: RowRef<UIColor>,
}

impl Sheet for BeastReputationRank {
    const SHEET_NAME: &'static str = "BeastReputationRank";
}

impl FromExcelRow for BeastReputationRank {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            required_reputation: single_row.columns.get(0).to_u16(),
            name: single_row.columns.get(1).to_owned_string(),
            allied_names: single_row.columns.get(2).to_owned_string(),
            color_id: single_row.columns.get(3).to_u32(),
            color: RowRef::<UIColor>::from(single_row.columns.get(3).to_u32()),
        })
    }
}

