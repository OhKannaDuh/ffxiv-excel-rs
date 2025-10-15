/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct DawnGrowMember {
    pub row_id: u32,
    pub class_id: u32,
    pub class: RowRef<DawnMemberUIParam>,
}

impl Sheet for DawnGrowMember {
    const SHEET_NAME: &'static str = "DawnGrowMember";
}

impl FromExcelRow for DawnGrowMember {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            class_id: single_row.columns.get(8).to_u32(),
            class: RowRef::<DawnMemberUIParam>::from(single_row.columns.get(8).to_u32()),
        })
    }
}

