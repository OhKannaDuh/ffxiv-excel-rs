/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct InstanceContentGuide {
    pub row_id: u32,
    pub instance_id: u32,
    pub instance: RowRef<InstanceContent>,
}

impl Sheet for InstanceContentGuide {
    const SHEET_NAME: &'static str = "InstanceContentGuide";
}

impl FromExcelRow for InstanceContentGuide {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            instance_id: single_row.columns.get(0).to_u32(),
            instance: RowRef::<InstanceContent>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

