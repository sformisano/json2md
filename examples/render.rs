//! Render a release note from committed JSON and template assets.

use json2md::View;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema: Value = serde_json::from_str(include_str!("assets/note.schema.json"))?;
    let input: Value = serde_json::from_str(include_str!("assets/note.json"))?;
    let view = View::new(&schema, include_str!("assets/note.j2"))?;

    print!("{}", view.render(&input)?);
    Ok(())
}
