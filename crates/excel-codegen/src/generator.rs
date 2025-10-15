use physis::exh::{ColumnDataType, EXH};

use crate::{file_dto::*, SKIP_LINKED_TYPES};

struct Fields(pub Vec<String>);
impl Fields {
    pub fn push(&mut self, field: String) {
        self.0.push(format!("    {}", field));
    }
}

struct Assigns(pub Vec<String>);
impl Assigns {
    pub fn push(&mut self, assign: String) {
        self.0.push(format!("            {}", assign));
    }
}

pub fn generate_struct(def: &DefinitionFile, header: &EXH) -> String {
    let type_name = &def.sheet;

    let mut fields = Fields(vec![]);
    fields.push("pub row_id: u32,".to_string());

    let mut assigns = Assigns(vec![]);
    assigns.push("row_id: row.row_id,".to_string());

    for d in &def.definitions {
        let field = d.to_field_name();
        let index = d.index;

        if d.data_type != DefinitionType::None {
            // skip repeat types for now
            continue;
        }

        if let Some(converter) = &d.converter {
            fields.push(format!("pub {field}_id: u32,"));
            assigns.push(format!(
                "{field}_id: single_row.columns.get({index}).to_u32(),"
            ));

            match converter.converter_type {
                ConverterType::Link => {
                    let mut linked_type = "Unknown".to_string();
                    if let Some(t) = &converter.target {
                        linked_type = t.clone();
                    }

                    if SKIP_LINKED_TYPES.contains(&linked_type.as_str()) {
                        continue;
                    }

                    fields.push(format!("pub {field}: RowRef<{linked_type}>,"));
                    assigns.push(format!(
                        "{field}: RowRef::<{linked_type}>::from(single_row.columns.get({index}).to_u32()),"
                    ));
                }
                ConverterType::MultiRef => {
                    if let Some(targets) = &converter.targets {
                        for target in targets {
                            let mut tf = to_field_name(target.clone());
                            // if tf does not start with field
                            if !tf.starts_with(&field) {
                                tf = format!("{}_{}", field, tf);
                            }

                            if SKIP_LINKED_TYPES.contains(&target.as_str()) {
                                continue;
                            }

                            fields.push(format!("pub {tf}: RowRef<{target}>,"));
                            assigns.push(format!(
                                "{tf}: RowRef::<{target}>::from(single_row.columns.get({index}).to_u32()),"
                            ));
                        }
                    }
                }
                // ConverterType::Icon => {
                //     fields.push(format!("pub {field}: RowRef<Image>,"));
                //     assigns.push(format!(
                //         "{field}: RowRef::<Image>::from(single_row.columns.get({index}).to_u32()),"
                //     ));
                // }
                _ => {}
            }
        } else {
            let ty = header
                .column_definitions
                .get(d.index as usize)
                .map(|c| rust_type_for(&c.data_type))
                .unwrap_or_else(|| "u32".into()); // safe fallback

            fields.push(format!("pub {field}: {ty},"));
            assigns.push(format!(
                "{field}: {},",
                emit_extract_expr(
                    d.index,
                    &header.column_definitions[d.index as usize].data_type,
                )
            ));
        }
    }
    format!(
        r#"/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct {type_name} {{
{fields}
}}

impl Sheet for {type_name} {{
    const SHEET_NAME: &'static str = "{type_name}";
}}

impl FromExcelRow for {type_name} {{
    fn from_row(row: &ExcelRow) -> Option<Self> {{
        let single_row = match &row.kind {{
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        }};

        Some(Self {{
{assigns}
        }})
    }}
}}

"#,
        fields = fields.0.join("\n"),
        assigns = assigns.0.join("\n"),
    )
}

fn rust_type_for(col_ty: &ColumnDataType) -> String {
    use ColumnDataType::*;

    match col_ty {
        String => "String".into(),
        Bool => "bool".into(),
        Int8 => "i8".into(),
        UInt8 | PackedBool0 | PackedBool1 | PackedBool2 | PackedBool3 | PackedBool4
        | PackedBool5 | PackedBool6 | PackedBool7 => {
            if matches!(col_ty, UInt8) {
                "u8".into()
            } else {
                "bool".into()
            }
        }
        Int16 => "i16".into(),
        UInt16 => "u16".into(),
        Int32 => "i32".into(),
        UInt32 => "u32".into(),
        Int64 => "i64".into(),
        UInt64 => "u64".into(),
        Float32 => "f32".into(),
    }
}

fn emit_extract_expr(idx: u32, col_ty: &ColumnDataType) -> String {
    use ColumnDataType::*;
    match col_ty {
        String => format!("single_row.columns.get({idx}).to_owned_string()"),
        Bool => format!("single_row.columns.get({idx}).to_bool()"),
        Int8 => format!("single_row.columns.get({idx}).to_i8()"),
        UInt8 => format!("single_row.columns.get({idx}).to_u8()"),
        Int16 => format!("single_row.columns.get({idx}).to_i16()"),
        UInt16 => format!("single_row.columns.get({idx}).to_u16()"),
        Int32 => format!("single_row.columns.get({idx}).to_i32()"),
        UInt32 => format!("single_row.columns.get({idx}).to_u32()"),
        Float32 => format!("single_row.columns.get({idx}).to_f32()"),
        Int64 => format!("single_row.columns.get({idx}).to_i64()"),
        UInt64 => format!("single_row.columns.get({idx}).to_u64()"),
        PackedBool0 => format!("single_row.columns.get({idx}).to_bit(0)"),
        PackedBool1 => format!("single_row.columns.get({idx}).to_bit(1)"),
        PackedBool2 => format!("single_row.columns.get({idx}).to_bit(2)"),
        PackedBool3 => format!("single_row.columns.get({idx}).to_bit(3)"),
        PackedBool4 => format!("single_row.columns.get({idx}).to_bit(4)"),
        PackedBool5 => format!("single_row.columns.get({idx}).to_bit(5)"),
        PackedBool6 => format!("single_row.columns.get({idx}).to_bit(6)"),
        PackedBool7 => format!("single_row.columns.get({idx}).to_bit(7)"),
    }
}
