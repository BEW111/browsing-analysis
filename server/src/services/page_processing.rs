use anyhow::{Context, Error};
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use htmd::HtmlToMarkdown;
use keyword_extraction::yake::{Yake, YakeParams};
use pgvector::Vector;

pub fn get_markdown_from_html(page_html: &String) -> Result<String, Error> {
    let converter = HtmlToMarkdown::builder()
        .skip_tags(vec!["script", "style"])
        .build();

    Ok(converter.convert(&page_html)?)
}

pub fn extract_keywords(text: &String, num_keywords: usize) -> Vec<String> {
    let stop_words = stop_words::get(stop_words::LANGUAGE::English);

    let yake = Yake::new(YakeParams::WithDefaults(text, &stop_words));
    yake.get_ranked_keywords(num_keywords)
}

pub fn get_embedding_from_text(text: &String) -> Result<Vector, Error> {
    let embedding_model = TextEmbedding::try_new(
        InitOptions::new(EmbeddingModel::AllMiniLML6V2).with_show_download_progress(true),
    )?;

    println!("Creating page embedding...");
    let pages_to_embed = vec![text];
    let page_embedding_vec = embedding_model
        .embed(pages_to_embed, None)?
        .pop()
        .context("List of embeddings is empty")?;
    println!("Created embedding.");

    Ok(pgvector::Vector::from(page_embedding_vec))
}
