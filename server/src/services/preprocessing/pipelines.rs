use anyhow::Error;

use crate::services::preprocessing::pipeline::PreprocessingPipeline;
use crate::services::preprocessing::pipeline_step::{
    ExtractKeywordsStringStep, HtmlToMarkdownStep, MiniLMEmbeddingStep,
};

pub const DIRECT_MINILM_PIPELINE: &str = "direct-minilm";

pub fn get_default_pipeline() -> Result<PreprocessingPipeline, Error> {
    let embedding_step = Box::new(MiniLMEmbeddingStep::new()?);

    let pipeline = PreprocessingPipeline::new(DIRECT_MINILM_PIPELINE, embedding_step)
        .add_step(HtmlToMarkdownStep)
        .add_step(ExtractKeywordsStringStep);

    Ok(pipeline)
}
