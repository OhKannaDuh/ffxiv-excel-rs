/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AetherCurrentCompFlgSet {
    pub row_id: u32,
    pub territory_id: u32,
    pub territory: RowRef<TerritoryType>,
}

impl Sheet for AetherCurrentCompFlgSet {
    const SHEET_NAME: &'static str = "AetherCurrentCompFlgSet";
}

impl FromExcelRow for AetherCurrentCompFlgSet {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            territory_id: single_row.columns.get(0).to_u32(),
            territory: RowRef::<TerritoryType>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

