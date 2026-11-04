// ----- Serde Imports -----
use serde::{Deserialize, Serialize};
use serde_json::Result;

// ----- Library Imports -----
use std::fs;

// --------------------------------------------
// ---------- Lore Snippets Importer ----------
// --------------------------------------------

#[derive(Deserialize, Serialize)]
pub struct LiveData {
    current_date: String,
}

pub async fn import_live_data(live_data_json_path: &'static str) -> Result<LiveData> {
    let json_string = &fs::read_to_string(live_data_json_path).unwrap();
    let live_data: LiveData = serde_json::from_str(json_string).unwrap();

    Ok(live_data)
}

pub async fn export_live_data(live_data_json_path: &'static str, updated_live_data: LiveData) {
    let json_string = serde_json::to_string(&updated_live_data).unwrap();
    let _ = &fs::write(live_data_json_path, json_string);
}
