/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct RelicNote {
    pub row_id: u32,
    pub event_item_id: u32,
    pub event_item: RowRef<EventItem>,
}

impl Sheet for RelicNote {
    const SHEET_NAME: &'static str = "RelicNote";
}

impl FromExcelRow for RelicNote {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            event_item_id: single_row.columns.get(0).to_u32(),
            event_item: RowRef::<EventItem>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

