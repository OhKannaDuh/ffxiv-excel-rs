/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct DpsChallengeTransient {
    pub row_id: u32,
    pub instance_content_id: u32,
    pub instance_content: RowRef<InstanceContent>,
}

impl Sheet for DpsChallengeTransient {
    const SHEET_NAME: &'static str = "DpsChallengeTransient";
}

impl FromExcelRow for DpsChallengeTransient {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            instance_content_id: single_row.columns.get(0).to_u32(),
            instance_content: RowRef::<InstanceContent>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

