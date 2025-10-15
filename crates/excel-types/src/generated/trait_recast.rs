/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct TraitRecast {
    pub row_id: u32,
    pub _trait_id: u32,
    pub _trait: RowRef<Trait>,
    pub action_id: u32,
    pub action: RowRef<Action>,
    pub time_ds: u16,
}

impl Sheet for TraitRecast {
    const SHEET_NAME: &'static str = "TraitRecast";
}

impl FromExcelRow for TraitRecast {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            _trait_id: single_row.columns.get(0).to_u32(),
            _trait: RowRef::<Trait>::from(single_row.columns.get(0).to_u32()),
            action_id: single_row.columns.get(1).to_u32(),
            action: RowRef::<Action>::from(single_row.columns.get(1).to_u32()),
            time_ds: single_row.columns.get(2).to_u16(),
        })
    }
}

