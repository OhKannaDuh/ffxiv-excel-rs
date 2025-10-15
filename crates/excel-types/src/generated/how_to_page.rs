/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HowToPage {
    pub row_id: u32,
    pub _type: u8,
    pub icon_type: u8,
    pub image_id: u32,
    pub text_type: u8,
}

impl Sheet for HowToPage {
    const SHEET_NAME: &'static str = "HowToPage";
}

impl FromExcelRow for HowToPage {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            _type: single_row.columns.get(0).to_u8(),
            icon_type: single_row.columns.get(1).to_u8(),
            image_id: single_row.columns.get(2).to_u32(),
            text_type: single_row.columns.get(3).to_u8(),
        })
    }
}

