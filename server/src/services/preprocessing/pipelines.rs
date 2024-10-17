use anyhow::Error;

use crate::services::preprocessing::pipeline::PreprocessingPipeline;
use crate::services::preprocessing::pipeline_step::{
    ExtractKeywordsStringStep, HtmlToMarkdownStep, MiniLMEmbeddingStep,
};

pub const DIRECT_MINILM_PIPELINE: &str = "direct-minilm";
pub const KEYWORD_MINILM_PIPELINE: &str = "keyword-minilm";
pub const MARKUPLM_PIPELINE: &str = "markuplm";

pub fn get_all_preprocessing_pipelines() -> Result<Vec<PreprocessingPipeline>, Error> {
    let mut pipelines = Vec::new();

    let embedding_step = Box::new(MiniLMEmbeddingStep::new()?);
    let pipeline = PreprocessingPipeline::new(DIRECT_MINILM_PIPELINE, embedding_step)
        .add_step(HtmlToMarkdownStep);
    pipelines.push(pipeline);

    let embedding_step = Box::new(MiniLMEmbeddingStep::new()?);
    let pipeline = PreprocessingPipeline::new(KEYWORD_MINILM_PIPELINE, embedding_step)
        .add_step(HtmlToMarkdownStep)
        .add_step(ExtractKeywordsStringStep);
    pipelines.push(pipeline);

    Ok(pipelines)
}
