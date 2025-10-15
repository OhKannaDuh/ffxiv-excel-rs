/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct IKDFishParam {
    pub row_id: u32,
    pub fish_id: u32,
    pub fish: RowRef<FishParameter>,
    pub ikd_content_bonus_id: u32,
    pub ikd_content_bonus: RowRef<IKDContentBonus>,
}

impl Sheet for IKDFishParam {
    const SHEET_NAME: &'static str = "IKDFishParam";
}

impl FromExcelRow for IKDFishParam {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            fish_id: single_row.columns.get(0).to_u32(),
            fish: RowRef::<FishParameter>::from(single_row.columns.get(0).to_u32()),
            ikd_content_bonus_id: single_row.columns.get(1).to_u32(),
            ikd_content_bonus: RowRef::<IKDContentBonus>::from(single_row.columns.get(1).to_u32()),
        })
    }
}

