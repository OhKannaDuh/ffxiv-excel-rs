/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GoldSaucerArcadeMachine {
    pub row_id: u32,
    pub fail_image_id: u32,
    pub poor_id: u32,
    pub good_id: u32,
    pub great_id: u32,
    pub excellent_id: u32,
}

impl Sheet for GoldSaucerArcadeMachine {
    const SHEET_NAME: &'static str = "GoldSaucerArcadeMachine";
}

impl FromExcelRow for GoldSaucerArcadeMachine {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            fail_image_id: single_row.columns.get(6).to_u32(),
            poor_id: single_row.columns.get(35).to_u32(),
            good_id: single_row.columns.get(36).to_u32(),
            great_id: single_row.columns.get(37).to_u32(),
            excellent_id: single_row.columns.get(38).to_u32(),
        })
    }
}

