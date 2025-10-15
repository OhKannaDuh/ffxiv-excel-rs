/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CustomTalkNestHandlers {
    pub row_id: u32,
    pub nest_handler_id: u32,
    pub nest_handler_gil_shop: RowRef<GilShop>,
    pub nest_handler_special_shop: RowRef<SpecialShop>,
    pub nest_handler_disposal_shop: RowRef<DisposalShop>,
}

impl Sheet for CustomTalkNestHandlers {
    const SHEET_NAME: &'static str = "CustomTalkNestHandlers";
}

impl FromExcelRow for CustomTalkNestHandlers {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            nest_handler_id: single_row.columns.get(0).to_u32(),
            nest_handler_gil_shop: RowRef::<GilShop>::from(single_row.columns.get(0).to_u32()),
            nest_handler_special_shop: RowRef::<SpecialShop>::from(single_row.columns.get(0).to_u32()),
            nest_handler_disposal_shop: RowRef::<DisposalShop>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

