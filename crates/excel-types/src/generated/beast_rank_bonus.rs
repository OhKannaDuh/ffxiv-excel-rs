/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BeastRankBonus {
    pub row_id: u32,
    pub neutral: u16,
    pub recognized: u16,
    pub friendly: u16,
    pub trusted: u16,
    pub respected: u16,
    pub honored: u16,
    pub sworn: u16,
    pub allied_bloodsworn: u16,
    pub item_id: u32,
    pub item: RowRef<Item>,
}

impl Sheet for BeastRankBonus {
    const SHEET_NAME: &'static str = "BeastRankBonus";
}

impl FromExcelRow for BeastRankBonus {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            neutral: single_row.columns.get(0).to_u16(),
            recognized: single_row.columns.get(1).to_u16(),
            friendly: single_row.columns.get(2).to_u16(),
            trusted: single_row.columns.get(3).to_u16(),
            respected: single_row.columns.get(4).to_u16(),
            honored: single_row.columns.get(5).to_u16(),
            sworn: single_row.columns.get(6).to_u16(),
            allied_bloodsworn: single_row.columns.get(7).to_u16(),
            item_id: single_row.columns.get(8).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(8).to_u32()),
        })
    }
}

