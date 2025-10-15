/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct PartyContentCutscene {
    pub row_id: u32,
    pub cutscene_id: u32,
    pub cutscene: RowRef<Cutscene>,
}

impl Sheet for PartyContentCutscene {
    const SHEET_NAME: &'static str = "PartyContentCutscene";
}

impl FromExcelRow for PartyContentCutscene {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            cutscene_id: single_row.columns.get(0).to_u32(),
            cutscene: RowRef::<Cutscene>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

