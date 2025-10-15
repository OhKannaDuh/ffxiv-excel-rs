/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct DescriptionSection {
    pub row_id: u32,
    pub string_id: u32,
    pub string: RowRef<DescriptionString>,
    pub page_id: u32,
    pub page: RowRef<DescriptionPage>,
}

impl Sheet for DescriptionSection {
    const SHEET_NAME: &'static str = "DescriptionSection";
}

impl FromExcelRow for DescriptionSection {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            string_id: single_row.columns.get(0).to_u32(),
            string: RowRef::<DescriptionString>::from(single_row.columns.get(0).to_u32()),
            page_id: single_row.columns.get(1).to_u32(),
            page: RowRef::<DescriptionPage>::from(single_row.columns.get(1).to_u32()),
        })
    }
}

