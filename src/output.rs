use anyhow::Context;
use std::path::Path;

use crate::product::Product;

const OUTPUT_FILE: &'static str = "output.json";

pub(crate) fn write_output(products: Vec<Product>) -> anyhow::Result<()> {
    let serialized_settings =
        serde_json::to_string_pretty(&products).context("Serialize output file.")?;

    let path = Path::new(OUTPUT_FILE);
    std::fs::write(path, serialized_settings.as_bytes()).context("Write output file.")?;

    Ok(())
}
