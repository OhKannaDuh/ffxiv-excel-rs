/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct IKDRouteTable {
    pub row_id: u32,
    pub route_id: u32,
    pub route: RowRef<IKDRoute>,
}

impl Sheet for IKDRouteTable {
    const SHEET_NAME: &'static str = "IKDRouteTable";
}

impl FromExcelRow for IKDRouteTable {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            route_id: single_row.columns.get(0).to_u32(),
            route: RowRef::<IKDRoute>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

