use anyhow::{Context, Error};
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use pgvector::Vector;

pub fn generate_pgvector_embedding(page_content: &String) -> Result<Vector, Error> {
    let embedding_model = TextEmbedding::try_new(
        InitOptions::new(EmbeddingModel::AllMiniLML6V2).with_show_download_progress(true),
    )?;

    println!("Creating page embedding...");
    let pages_to_embed = vec![page_content];
    let page_embedding_vec = embedding_model
        .embed(pages_to_embed, None)?
        .pop()
        .context("List of embeddings is empty")?;
    println!("Created embedding.");

    Ok(pgvector::Vector::from(page_embedding_vec))
}
