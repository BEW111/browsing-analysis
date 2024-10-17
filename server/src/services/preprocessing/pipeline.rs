use crate::services::preprocessing::pipeline_step::{EmbeddingStep, PreprocessingStep};

use anyhow::Error;

pub struct PreprocessingPipeline {
    pub name: &'static str,
    pub steps: Vec<Box<dyn PreprocessingStep>>,
    pub embedding_step: Box<dyn EmbeddingStep>,
}

impl PreprocessingPipeline {
    pub fn new(name: &'static str, embedding_step: Box<dyn EmbeddingStep>) -> Self {
        PreprocessingPipeline {
            name,
            steps: vec![],
            embedding_step,
        }
    }

    pub fn add_step<T: PreprocessingStep + 'static>(mut self, step: T) -> Self {
        self.steps.push(Box::new(step));
        self
    }

    pub fn run(&self, page_content: &str) -> Result<pgvector::Vector, Error> {
        let mut intermediate_result = page_content.to_string();

        for step in &self.steps {
            intermediate_result = step.process(&intermediate_result)?;
        }

        return self.embedding_step.embed(&intermediate_result);
    }
}
