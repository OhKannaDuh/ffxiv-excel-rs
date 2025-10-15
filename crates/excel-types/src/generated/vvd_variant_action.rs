/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct VVDVariantAction {
    pub row_id: u32,
    pub action_id: u32,
    pub action: RowRef<Action>,
}

impl Sheet for VVDVariantAction {
    const SHEET_NAME: &'static str = "VVDVariantAction";
}

impl FromExcelRow for VVDVariantAction {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            action_id: single_row.columns.get(0).to_u32(),
            action: RowRef::<Action>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

