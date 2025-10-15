/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ModelChara {
    pub row_id: u32,
    pub _type: u8,
    pub model: u16,
    pub base: u8,
    pub variant: u8,
    pub se_pack: u16,
    pub pap_variation: bool,
}

impl Sheet for ModelChara {
    const SHEET_NAME: &'static str = "ModelChara";
}

impl FromExcelRow for ModelChara {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            _type: single_row.columns.get(0).to_u8(),
            model: single_row.columns.get(1).to_u16(),
            base: single_row.columns.get(2).to_u8(),
            variant: single_row.columns.get(3).to_u8(),
            se_pack: single_row.columns.get(4).to_u16(),
            pap_variation: single_row.columns.get(7).to_bit(1),
        })
    }
}

