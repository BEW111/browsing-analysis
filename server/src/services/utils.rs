use anyhow::Error;
use htmd::HtmlToMarkdown;
use keyword_extraction::yake::{Yake, YakeParams};

pub fn html_to_markdown(html: &str) -> Result<String, Error> {
    let converter = HtmlToMarkdown::builder()
        .skip_tags(vec!["script", "style"])
        .build();

    Ok(converter.convert(&html)?)
}

pub fn extract_keywords(text: &str, num_keywords: usize) -> Vec<String> {
    let stop_words = stop_words::get(stop_words::LANGUAGE::English);
    let yake = Yake::new(YakeParams::WithDefaults(text, &stop_words));
    yake.get_ranked_keywords(num_keywords)
}
