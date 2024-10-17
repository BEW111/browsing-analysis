use crate::services::utils::{extract_keywords, html_to_markdown};

use anyhow::{Context, Error};
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use pgvector::Vector;

pub trait PreprocessingStep: Send {
    fn process(&self, input: &String) -> Result<String, Error>;
}

pub trait EmbeddingStep: Send {
    fn embed(&self, input: &String) -> Result<Vector, Error>;
}

pub struct HtmlToMarkdownStep;

impl PreprocessingStep for HtmlToMarkdownStep {
    fn process(&self, page_html: &String) -> Result<String, Error> {
        html_to_markdown(page_html)
    }
}

pub struct ExtractKeywordsStringStep;

impl PreprocessingStep for ExtractKeywordsStringStep {
    fn process(&self, text: &String) -> Result<String, Error> {
        let keywords = extract_keywords(text, 15);
        Ok(keywords.join(" "))
    }
}

pub struct MiniLMEmbeddingStep {
    embedding_model: TextEmbedding,
}

impl MiniLMEmbeddingStep {
    pub fn new() -> Result<Self, Error> {
        let embedding_model = TextEmbedding::try_new(
            InitOptions::new(EmbeddingModel::AllMiniLML6V2).with_show_download_progress(true),
        )?;

        Ok(MiniLMEmbeddingStep { embedding_model })
    }
}

impl EmbeddingStep for MiniLMEmbeddingStep {
    fn embed(&self, text: &String) -> Result<Vector, Error> {
        println!("Creating page embedding...");
        let pages_to_embed = vec![text];
        let page_embedding_vec = self
            .embedding_model
            .embed(pages_to_embed, None)?
            .pop()
            .context("List of embeddings is empty")?;
        println!("Created embedding.");

        Ok(pgvector::Vector::from(page_embedding_vec))
    }
}
