mod file_dto;
use file_dto::*;

mod generator;
use generator::*;

mod config;
pub(crate) use config::*;

use std::path::PathBuf;

use physis::{common::Platform, gamedata::GameData};

fn main() -> anyhow::Result<()> {
    let definition_directory = PathBuf::from("../../SaintCoinach/SaintCoinach/Definitions");

    let mut game_data = GameData::from_existing(
        Platform::Win32,
        "C:/Program Files (x86)/Steam/steamapps/common/FINAL FANTASY XIV Online/game",
    );

    let types_root = PathBuf::from("../../crates/excel-types/src/generated");
    std::fs::create_dir_all(&types_root)?;

    let mut modules: Vec<(String, String)> = Vec::new();

    let entries = std::fs::read_dir(&definition_directory)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }

        let file_content = std::fs::read_to_string(&path)?;
        let def_file: DefinitionFile = serde_json::from_str(&file_content).unwrap_or_else(|e| {
            panic!("Failed to parse file {}:\n{e}", path.display());
        });

        if def_file.definitions.is_empty() {
            println!("Skipping empty definition file: {}", def_file.sheet);
            continue;
        }

        let Some(header) = game_data.read_excel_sheet_header(&def_file.sheet) else {
            println!("Sheet {} not found", def_file.sheet);
            continue;
        };

        let mut sheet_name = def_file.sheet.clone();
        let mut mod_name = to_field_name(sheet_name.clone());

        if RESERVED_KEYWORDS.contains(&mod_name.as_str()) {
            mod_name = format!("{mod_name}_sheet");
            sheet_name = format!("{sheet_name}Sheet");
            println!(
                "Module name {} is a reserved keyword, renamed to {}",
                def_file.sheet, mod_name
            );
        }

        modules.push((mod_name.clone(), sheet_name.clone()));

        let code = generate_struct(&def_file, &header);

        let out_path = types_root.join(format!("{mod_name}.rs"));
        std::fs::write(&out_path, code)?;
    }

    let mut mod_rs = String::new();

    for (l, u) in modules {
        mod_rs.push_str(&format!("mod {l};\npub use {l}::{u};\n"));
    }

    std::fs::write(types_root.join("mod.rs"), mod_rs)?;

    Ok(())
}
