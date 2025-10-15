/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HWDInfoBoardArticle {
    pub row_id: u32,
    pub _type_id: u32,
    pub _type: RowRef<HWDInfoBoardArticleType>,
    pub text: String,
}

impl Sheet for HWDInfoBoardArticle {
    const SHEET_NAME: &'static str = "HWDInfoBoardArticle";
}

impl FromExcelRow for HWDInfoBoardArticle {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            _type_id: single_row.columns.get(0).to_u32(),
            _type: RowRef::<HWDInfoBoardArticleType>::from(single_row.columns.get(0).to_u32()),
            text: single_row.columns.get(4).to_owned_string(),
        })
    }
}

