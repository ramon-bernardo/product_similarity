use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    hash::Hash,
    io::{Read, Write},
    path::Path,
};

use crate::settings::SimilarityType;

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub settings_id: Vec<String>,
}

impl Product {
    pub const FILE: &'static str = "products.json";

    pub fn new_with_usize_similarity(
        product_without_settings: &Product,
        product_with_settings: &Product,
        similarity_type: &SimilarityType,
        similarity: usize,
    ) -> Self {
        tracing::info!(
            "Product [{:?}] -> [{:?}]: {:?} (Result: {:?})",
            product_without_settings.name,
            product_with_settings.name,
            similarity_type,
            similarity
        );

        Self {
            id: product_without_settings.id.clone(),
            name: product_without_settings.name.clone(),
            settings_id: product_with_settings.settings_id.clone(),
        }
    }

    pub fn new_with_f64_similarity(
        product_without_settings: &Product,
        product_with_settings: &Product,
        similarity_type: &SimilarityType,
        similarity: f64,
    ) -> Self {
        tracing::info!(
            "Product [{:?}] -> [{:?}]: {:?} (Result: {:?})",
            product_without_settings.name,
            product_with_settings.name,
            similarity_type,
            similarity
        );

        Self {
            id: product_without_settings.id.clone(),
            name: product_without_settings.name.clone(),
            settings_id: product_with_settings.settings_id.clone(),
        }
    }

    pub fn init() -> anyhow::Result<Vec<Product>> {
        Self::create()?;
        Self::load()
    }

    fn create() -> anyhow::Result<()> {
        let path = Path::new(Self::FILE);
        if !Path::exists(path) {
            let settings: Vec<Product> = vec![];
            let serialized_settings =
                serde_json::to_string_pretty(&settings).context("Serialize products file.")?;

            let mut file = File::create(path).context("Create products file.")?;
            file.write_all(serialized_settings.as_bytes())
                .context("Write products file.")?;
        }

        Ok(())
    }

    fn load() -> anyhow::Result<Vec<Self>> {
        let mut file = File::open(Self::FILE).context("Open products file.")?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .context("Read products file.")?;

        Ok(serde_json::from_str(&contents).context("Serialize products file.")?)
    }
}

impl PartialEq for Product {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Product {}

impl Hash for Product {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
