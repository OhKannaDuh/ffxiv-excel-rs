/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GrandCompanyRank {
    pub row_id: u32,
    pub tier: u8,
    pub order: u8,
    pub max_seals: u32,
    pub required_seals: u32,
    pub icon_maelstrom_id: u32,
    pub icon_serpents_id: u32,
    pub icon_flames_id: u32,
    pub quest_maelstrom_id: u32,
    pub quest_maelstrom: RowRef<Quest>,
    pub quest_serpents_id: u32,
    pub quest_serpents: RowRef<Quest>,
    pub quest_flames_id: u32,
    pub quest_flames: RowRef<Quest>,
}

impl Sheet for GrandCompanyRank {
    const SHEET_NAME: &'static str = "GrandCompanyRank";
}

impl FromExcelRow for GrandCompanyRank {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            tier: single_row.columns.get(0).to_u8(),
            order: single_row.columns.get(1).to_u8(),
            max_seals: single_row.columns.get(2).to_u32(),
            required_seals: single_row.columns.get(3).to_u32(),
            icon_maelstrom_id: single_row.columns.get(4).to_u32(),
            icon_serpents_id: single_row.columns.get(5).to_u32(),
            icon_flames_id: single_row.columns.get(6).to_u32(),
            quest_maelstrom_id: single_row.columns.get(7).to_u32(),
            quest_maelstrom: RowRef::<Quest>::from(single_row.columns.get(7).to_u32()),
            quest_serpents_id: single_row.columns.get(8).to_u32(),
            quest_serpents: RowRef::<Quest>::from(single_row.columns.get(8).to_u32()),
            quest_flames_id: single_row.columns.get(9).to_u32(),
            quest_flames: RowRef::<Quest>::from(single_row.columns.get(9).to_u32()),
        })
    }
}

