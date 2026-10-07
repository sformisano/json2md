//! Renders a cumulative review summary from caller-owned schema and template files.

use json2md::View;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema: Value = serde_json::from_str(include_str!("gitlab-review/summary.schema.json"))?;
    let input: Value = serde_json::from_str(include_str!("gitlab-review/follow-up.json"))?;
    let view = View::new(&schema, include_str!("gitlab-review/summary.j2"))?;

    print!("{}", view.render(&input)?);
    Ok(())
}
