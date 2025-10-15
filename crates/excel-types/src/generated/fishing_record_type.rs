/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct FishingRecordType {
    pub row_id: u32,
    pub addon_id: u32,
    pub addon: RowRef<Addon>,
    pub rank_b_requirement: u16,
    pub rank_a_requirement: u16,
    pub rank_aa_requirement: u16,
    pub rank_aaa_requirement: u16,
    pub rank_s_requirement: u16,
    pub is_spearfishing: u8,
}

impl Sheet for FishingRecordType {
    const SHEET_NAME: &'static str = "FishingRecordType";
}

impl FromExcelRow for FishingRecordType {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            addon_id: single_row.columns.get(0).to_u32(),
            addon: RowRef::<Addon>::from(single_row.columns.get(0).to_u32()),
            rank_b_requirement: single_row.columns.get(1).to_u16(),
            rank_a_requirement: single_row.columns.get(2).to_u16(),
            rank_aa_requirement: single_row.columns.get(3).to_u16(),
            rank_aaa_requirement: single_row.columns.get(4).to_u16(),
            rank_s_requirement: single_row.columns.get(5).to_u16(),
            is_spearfishing: single_row.columns.get(6).to_u8(),
        })
    }
}

