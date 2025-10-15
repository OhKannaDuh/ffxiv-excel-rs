/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct DawnQuestMember {
    pub row_id: u32,
    pub member_id: u32,
    pub member: RowRef<ENpcResident>,
    pub big_image_old_id: u32,
    pub big_image_new_id: u32,
    pub class_id: u32,
    pub class: RowRef<DawnMemberUIParam>,
}

impl Sheet for DawnQuestMember {
    const SHEET_NAME: &'static str = "DawnQuestMember";
}

impl FromExcelRow for DawnQuestMember {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            member_id: single_row.columns.get(2).to_u32(),
            member: RowRef::<ENpcResident>::from(single_row.columns.get(2).to_u32()),
            big_image_old_id: single_row.columns.get(3).to_u32(),
            big_image_new_id: single_row.columns.get(4).to_u32(),
            class_id: single_row.columns.get(5).to_u32(),
            class: RowRef::<DawnMemberUIParam>::from(single_row.columns.get(5).to_u32()),
        })
    }
}

