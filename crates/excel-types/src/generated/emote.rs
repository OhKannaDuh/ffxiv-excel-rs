/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Emote {
    pub row_id: u32,
    pub name: String,
    pub emote_category_id: u32,
    pub emote_category: RowRef<EmoteCategory>,
    pub emote_mode_id: u32,
    pub emote_mode: RowRef<EmoteMode>,
    pub has_cancel_emote: bool,
    pub draws_weapon: bool,
    pub order: u16,
    pub text_command_id: u32,
    pub text_command: RowRef<TextCommand>,
    pub icon_id: u32,
    pub log_message_targeted_id: u32,
    pub log_message_targeted: RowRef<LogMessage>,
    pub log_message_untargeted_id: u32,
    pub log_message_untargeted: RowRef<LogMessage>,
    pub unlock_link: u32,
}

impl Sheet for Emote {
    const SHEET_NAME: &'static str = "Emote";
}

impl FromExcelRow for Emote {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            emote_category_id: single_row.columns.get(11).to_u32(),
            emote_category: RowRef::<EmoteCategory>::from(single_row.columns.get(11).to_u32()),
            emote_mode_id: single_row.columns.get(12).to_u32(),
            emote_mode: RowRef::<EmoteMode>::from(single_row.columns.get(12).to_u32()),
            has_cancel_emote: single_row.columns.get(15).to_bit(5),
            draws_weapon: single_row.columns.get(16).to_bit(6),
            order: single_row.columns.get(18).to_u16(),
            text_command_id: single_row.columns.get(19).to_u32(),
            text_command: RowRef::<TextCommand>::from(single_row.columns.get(19).to_u32()),
            icon_id: single_row.columns.get(20).to_u32(),
            log_message_targeted_id: single_row.columns.get(21).to_u32(),
            log_message_targeted: RowRef::<LogMessage>::from(single_row.columns.get(21).to_u32()),
            log_message_untargeted_id: single_row.columns.get(22).to_u32(),
            log_message_untargeted: RowRef::<LogMessage>::from(single_row.columns.get(22).to_u32()),
            unlock_link: single_row.columns.get(23).to_u32(),
        })
    }
}

