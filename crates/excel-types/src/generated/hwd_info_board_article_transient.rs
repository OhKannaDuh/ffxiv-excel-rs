/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HWDInfoBoardArticleTransient {
    pub row_id: u32,
    pub image_id: u32,
    pub text: String,
    pub npc_name: String,
}

impl Sheet for HWDInfoBoardArticleTransient {
    const SHEET_NAME: &'static str = "HWDInfoBoardArticleTransient";
}

impl FromExcelRow for HWDInfoBoardArticleTransient {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            image_id: single_row.columns.get(0).to_u32(),
            text: single_row.columns.get(1).to_owned_string(),
            npc_name: single_row.columns.get(2).to_owned_string(),
        })
    }
}

