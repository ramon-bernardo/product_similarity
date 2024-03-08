use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub num_threads: usize,
    pub similarities_types: Vec<SimilarityType>,
}

impl Settings {
    pub const FILE: &'static str = "settings.json";

    pub fn init() -> anyhow::Result<Self> {
        Self::create()?;
        Self::load()
    }

    fn create() -> anyhow::Result<()> {
        let path = Path::new(Self::FILE);
        if !Path::exists(path) {
            let settings = Self::default();
            let serialized_settings =
                serde_json::to_string_pretty(&settings).context("Serialize settings.")?;

            let mut file = File::create(path).context("Create settings file.")?;
            file.write_all(serialized_settings.as_bytes())
                .context("Write settings file.")?;
        }

        Ok(())
    }

    fn load() -> anyhow::Result<Self> {
        let mut file = File::open(Self::FILE).context("Open settings file.")?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .context("Read settings file.")?;

        Ok(serde_json::from_str(&contents).context("Serialize settings file.")?)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            num_threads: 2,
            similarities_types: vec![
                SimilarityType::Hamming(100),
                SimilarityType::Levenshtein(5),
                SimilarityType::NormalizedLevenshtein(0.9),
                SimilarityType::OsaDistance(100),
                SimilarityType::DamerauLevenshtein(100),
                SimilarityType::NormalizedDamerauLevenshtein(0.9),
                SimilarityType::Jaro(0.9),
                SimilarityType::JaroWinkler(0.9),
                SimilarityType::SorensenDice(0.9),
            ],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SimilarityType {
    Hamming(usize),
    Levenshtein(usize),
    NormalizedLevenshtein(f64),
    OsaDistance(usize),
    DamerauLevenshtein(usize),
    NormalizedDamerauLevenshtein(f64),
    Jaro(f64),
    JaroWinkler(f64),
    SorensenDice(f64),
}
