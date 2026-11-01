// ----- Serde Imports -----
use serde::Deserialize;
use serde_json::Result;

// ----- Library Imports -----
use std::fs;

// --------------------------------------------
// ---------- Lore Snippets Importer ----------
// --------------------------------------------

#[derive(Deserialize)]
pub struct LoreSnippets {
    lore_snippets: Vec<String>,
}

/// Turns the lore snippet JSON file into a Vec<String> before returning it
pub async fn read_lore_snippet_json(lore_snippet_json_path: &'static str) -> Result<Vec<String>> {
    let json_string = &fs::read_to_string(lore_snippet_json_path).unwrap();
    let list_of_lore_snippets: LoreSnippets = serde_json::from_str(json_string).unwrap();

    Ok(list_of_lore_snippets.lore_snippets)
}
