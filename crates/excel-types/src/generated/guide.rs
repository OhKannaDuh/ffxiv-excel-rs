/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Guide {
    pub row_id: u32,
    pub guide_title_id: u32,
    pub guide_title: RowRef<GuideTitle>,
    pub guide_page_id: u32,
    pub guide_page: RowRef<GuidePage>,
}

impl Sheet for Guide {
    const SHEET_NAME: &'static str = "Guide";
}

impl FromExcelRow for Guide {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            guide_title_id: single_row.columns.get(0).to_u32(),
            guide_title: RowRef::<GuideTitle>::from(single_row.columns.get(0).to_u32()),
            guide_page_id: single_row.columns.get(1).to_u32(),
            guide_page: RowRef::<GuidePage>::from(single_row.columns.get(1).to_u32()),
        })
    }
}

