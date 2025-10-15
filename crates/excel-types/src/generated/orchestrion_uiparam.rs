/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct OrchestrionUiparam {
    pub row_id: u32,
    pub orchestrion_category_id: u32,
    pub orchestrion_category: RowRef<OrchestrionCategory>,
    pub order: u16,
}

impl Sheet for OrchestrionUiparam {
    const SHEET_NAME: &'static str = "OrchestrionUiparam";
}

impl FromExcelRow for OrchestrionUiparam {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            orchestrion_category_id: single_row.columns.get(0).to_u32(),
            orchestrion_category: RowRef::<OrchestrionCategory>::from(single_row.columns.get(0).to_u32()),
            order: single_row.columns.get(1).to_u16(),
        })
    }
}

