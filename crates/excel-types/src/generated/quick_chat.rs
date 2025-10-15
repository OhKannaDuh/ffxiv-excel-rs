/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct QuickChat {
    pub row_id: u32,
    pub name_action: String,
    pub icon_id: u32,
    pub addon_id: u32,
    pub addon: RowRef<Addon>,
    pub quick_chat_transient_id: u32,
    pub quick_chat_transient: RowRef<QuickChatTransient>,
}

impl Sheet for QuickChat {
    const SHEET_NAME: &'static str = "QuickChat";
}

impl FromExcelRow for QuickChat {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name_action: single_row.columns.get(0).to_owned_string(),
            icon_id: single_row.columns.get(1).to_u32(),
            addon_id: single_row.columns.get(2).to_u32(),
            addon: RowRef::<Addon>::from(single_row.columns.get(2).to_u32()),
            quick_chat_transient_id: single_row.columns.get(3).to_u32(),
            quick_chat_transient: RowRef::<QuickChatTransient>::from(single_row.columns.get(3).to_u32()),
        })
    }
}

