/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CompanyCraftSequence {
    pub row_id: u32,
    pub result_item_id: u32,
    pub result_item: RowRef<Item>,
    pub category: i32,
    pub company_craft_draft_category_id: u32,
    pub company_craft_draft_category: RowRef<CompanyCraftDraftCategory>,
    pub company_craft_type_id: u32,
    pub company_craft_type: RowRef<CompanyCraftType>,
    pub company_craft_draft_id: u32,
    pub company_craft_draft: RowRef<CompanyCraftDraft>,
    pub order: u32,
}

impl Sheet for CompanyCraftSequence {
    const SHEET_NAME: &'static str = "CompanyCraftSequence";
}

impl FromExcelRow for CompanyCraftSequence {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            result_item_id: single_row.columns.get(0).to_u32(),
            result_item: RowRef::<Item>::from(single_row.columns.get(0).to_u32()),
            category: single_row.columns.get(1).to_i32(),
            company_craft_draft_category_id: single_row.columns.get(2).to_u32(),
            company_craft_draft_category: RowRef::<CompanyCraftDraftCategory>::from(single_row.columns.get(2).to_u32()),
            company_craft_type_id: single_row.columns.get(3).to_u32(),
            company_craft_type: RowRef::<CompanyCraftType>::from(single_row.columns.get(3).to_u32()),
            company_craft_draft_id: single_row.columns.get(4).to_u32(),
            company_craft_draft: RowRef::<CompanyCraftDraft>::from(single_row.columns.get(4).to_u32()),
            order: single_row.columns.get(13).to_u32(),
        })
    }
}

