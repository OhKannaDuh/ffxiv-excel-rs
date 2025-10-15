/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CompanyCraftDraft {
    pub row_id: u32,
    pub name: String,
    pub company_craft_draft_category_id: u32,
    pub company_craft_draft_category: RowRef<CompanyCraftDraftCategory>,
    pub order: u32,
}

impl Sheet for CompanyCraftDraft {
    const SHEET_NAME: &'static str = "CompanyCraftDraft";
}

impl FromExcelRow for CompanyCraftDraft {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            company_craft_draft_category_id: single_row.columns.get(1).to_u32(),
            company_craft_draft_category: RowRef::<CompanyCraftDraftCategory>::from(single_row.columns.get(1).to_u32()),
            order: single_row.columns.get(8).to_u32(),
        })
    }
}

