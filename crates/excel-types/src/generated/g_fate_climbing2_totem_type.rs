/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GFateClimbing2TotemType {
    pub row_id: u32,
    pub public_content_text_data_id: u32,
    pub public_content_text_data: RowRef<PublicContentTextData>,
}

impl Sheet for GFateClimbing2TotemType {
    const SHEET_NAME: &'static str = "GFateClimbing2TotemType";
}

impl FromExcelRow for GFateClimbing2TotemType {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            public_content_text_data_id: single_row.columns.get(0).to_u32(),
            public_content_text_data: RowRef::<PublicContentTextData>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

