use anyhow::{ensure, Context};
use product::Product;
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    ThreadPoolBuilder,
};
use settings::SimilarityType;
use std::sync::{Arc, Mutex};

use crate::settings::Settings;

mod output;
mod product;
mod settings;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::fmt()
        .with_writer(tracing_appender::rolling::minutely("./logs", "similarity"))
        .with_ansi(false)
        .init();

    let settings: Settings = Settings::init().context("Init settings file.")?;
    ensure!(
        !settings.similarities_types.is_empty(),
        "Similarity types settings not found."
    );

    ThreadPoolBuilder::new()
        .num_threads(settings.num_threads)
        .build_global()
        .context("Build rayon thread pool.")?;

    let products = Product::init().context("Init products file.")?;
    ensure!(!products.is_empty(), "Products not found.");

    let calculated_products = init_calculate(settings, products).context("Calculate.")?;
    ensure!(
        !calculated_products.is_empty(),
        "Calculated products empty."
    );

    output::write_output(calculated_products)?;

    Ok(())
}

fn init_calculate(settings: Settings, products: Vec<Product>) -> anyhow::Result<Vec<Product>> {
    let products_without_settings: Vec<&Product> = products
        .iter()
        .filter(|product| product.settings_id.is_empty())
        .collect();

    ensure!(
        !products_without_settings.is_empty(),
        "Products without settings not found."
    );

    let products_with_settings: Vec<&Product> = products
        .iter()
        .filter(|product| !product.settings_id.is_empty())
        .collect();

    ensure!(
        !products_with_settings.is_empty(),
        "Products with settings not found."
    );

    tracing::info!(
        "Products: {} / {}",
        products_with_settings.len(),
        products_without_settings.len(),
    );

    let products = Arc::new(Mutex::new(Vec::<Product>::new()));

    products_without_settings
        .par_iter()
        .for_each(|product_without_settings| {
            products_with_settings
                .par_iter()
                .for_each(|product_with_settings| {
                    if let Some(product) = calculate_similarity(
                        &settings,
                        product_without_settings,
                        product_with_settings,
                    ) {
                        let mut products = products
                            .lock()
                            .expect("Lock is already held by the current thread.");

                        products.push(product);
                    }
                });
        });

    let products = Arc::try_unwrap(products)
        .expect("Error on try_unwrap products.")
        .into_inner()
        .expect("Error on try_unwrap::into_inner products.");

    Ok(products)
}

fn calculate_similarity(
    settings: &Settings,
    product_without_settings: &Product,
    product_with_settings: &Product,
) -> Option<Product> {
    for similarity_type in &settings.similarities_types {
        match *similarity_type {
            SimilarityType::Hamming(min) => {
                match strsim::hamming(&product_without_settings.name, &product_with_settings.name) {
                    Ok(value) => {
                        tracing::info!(
                            "Product [{:?}] -> [{:?}]: {:?} (Result: {:?})",
                            product_without_settings.name,
                            product_with_settings.name,
                            similarity_type,
                            value
                        );

                        if min < value {
                            return Some(Product::new(
                                product_without_settings,
                                product_with_settings,
                            ));
                        }
                    }
                    Err(e) => {
                        tracing::info!(
                            "Product [{:?}] -> [{:?}]: {:?} (Result: {:?})",
                            product_without_settings.name,
                            product_with_settings.name,
                            similarity_type,
                            e
                        );
                    }
                }
            }
            SimilarityType::Levenshtein(min) => {
                let value = strsim::levenshtein(
                    &product_without_settings.name,
                    &product_with_settings.name,
                );

                tracing::info!(
                    "Product [{:?}] -> [{:?}]: {:?} (Result: {:?})",
                    product_without_settings.name,
                    product_with_settings.name,
                    similarity_type,
                    value
                );

                if min < value {
                    return Some(Product::new(
                        product_without_settings,
                        product_with_settings,
                    ));
                }
            }
            SimilarityType::NormalizedLevenshtein(min) => {
                let value = strsim::normalized_levenshtein(
                    &product_without_settings.name,
                    &product_with_settings.name,
                );

                tracing::info!(
                    "Product [{:?}] -> [{:?}]: {:?} (Result: {:?})",
                    product_without_settings.name,
                    product_with_settings.name,
                    similarity_type,
                    value
                );

                if min < value {
                    return Some(Product::new(
                        product_without_settings,
                        product_with_settings,
                    ));
                }
            }
            SimilarityType::OsaDistance(min) => {
                let value = strsim::osa_distance(
                    &product_without_settings.name,
                    &product_with_settings.name,
                );

                tracing::info!(
                    "Product [{:?}] -> [{:?}]: {:?} (Result: {:?})",
                    product_without_settings.name,
                    product_with_settings.name,
                    similarity_type,
                    value
                );

                if min < value {
                    return Some(Product::new(
                        product_without_settings,
                        product_with_settings,
                    ));
                }
            }
            SimilarityType::DamerauLevenshtein(min) => {
                let value = strsim::damerau_levenshtein(
                    &product_without_settings.name,
                    &product_with_settings.name,
                );

                tracing::info!(
                    "Product [{:?}] -> [{:?}]: {:?} (Result: {:?})",
                    product_without_settings.name,
                    product_with_settings.name,
                    similarity_type,
                    value
                );

                if min < value {
                    return Some(Product::new(
                        product_without_settings,
                        product_with_settings,
                    ));
                }
            }
            SimilarityType::NormalizedDamerauLevenshtein(min) => {
                let value = strsim::normalized_damerau_levenshtein(
                    &product_without_settings.name,
                    &product_with_settings.name,
                );

                tracing::info!(
                    "Product [{:?}] -> [{:?}]: {:?} (Result: {:?})",
                    product_without_settings.name,
                    product_with_settings.name,
                    similarity_type,
                    value
                );

                if min < value {
                    return Some(Product::new(
                        product_without_settings,
                        product_with_settings,
                    ));
                }
            }
            SimilarityType::Jaro(min) => {
                let value =
                    strsim::jaro(&product_without_settings.name, &product_with_settings.name);

                tracing::info!(
                    "Product [{:?}] -> [{:?}]: {:?} (Result: {:?})",
                    product_without_settings.name,
                    product_with_settings.name,
                    similarity_type,
                    value
                );

                if min < value {
                    return Some(Product::new(
                        product_without_settings,
                        product_with_settings,
                    ));
                }
            }
            SimilarityType::JaroWinkler(min) => {
                let value = strsim::jaro_winkler(
                    &product_without_settings.name,
                    &product_with_settings.name,
                );

                tracing::info!(
                    "Product [{:?}] -> [{:?}]: {:?} (Result: {:?})",
                    product_without_settings.name,
                    product_with_settings.name,
                    similarity_type,
                    value
                );

                if min < value {
                    return Some(Product::new(
                        product_without_settings,
                        product_with_settings,
                    ));
                }
            }
            SimilarityType::SorensenDice(min) => {
                let value = strsim::sorensen_dice(
                    &product_without_settings.name,
                    &product_with_settings.name,
                );

                tracing::info!(
                    "Product [{:?}] -> [{:?}]: {:?} (Result: {:?})",
                    product_without_settings.name,
                    product_with_settings.name,
                    similarity_type,
                    value
                );

                if min < value {
                    return Some(Product::new(
                        product_without_settings,
                        product_with_settings,
                    ));
                }
            }
        }
    }

    None
}
