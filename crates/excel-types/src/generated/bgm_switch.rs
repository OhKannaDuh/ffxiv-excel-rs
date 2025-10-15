/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BGMSwitch {
    pub row_id: u32,
    pub bgm_system_define_id: u32,
    pub bgm_system_define: RowRef<BGMSystemDefine>,
    pub quest_id: u32,
    pub quest: RowRef<Quest>,
    pub bgm_id: u32,
    pub bgm: RowRef<BGM>,
    pub bgm_situation: RowRef<BGMSituation>,
}

impl Sheet for BGMSwitch {
    const SHEET_NAME: &'static str = "BGMSwitch";
}

impl FromExcelRow for BGMSwitch {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            bgm_system_define_id: single_row.columns.get(0).to_u32(),
            bgm_system_define: RowRef::<BGMSystemDefine>::from(single_row.columns.get(0).to_u32()),
            quest_id: single_row.columns.get(1).to_u32(),
            quest: RowRef::<Quest>::from(single_row.columns.get(1).to_u32()),
            bgm_id: single_row.columns.get(3).to_u32(),
            bgm: RowRef::<BGM>::from(single_row.columns.get(3).to_u32()),
            bgm_situation: RowRef::<BGMSituation>::from(single_row.columns.get(3).to_u32()),
        })
    }
}

