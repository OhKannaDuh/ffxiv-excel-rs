/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct SpearfishingRecordPage {
    pub row_id: u32,
    pub place_name_id: u32,
    pub place_name: RowRef<PlaceName>,
    pub image_id: u32,
}

impl Sheet for SpearfishingRecordPage {
    const SHEET_NAME: &'static str = "SpearfishingRecordPage";
}

impl FromExcelRow for SpearfishingRecordPage {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            place_name_id: single_row.columns.get(3).to_u32(),
            place_name: RowRef::<PlaceName>::from(single_row.columns.get(3).to_u32()),
            image_id: single_row.columns.get(4).to_u32(),
        })
    }
}

