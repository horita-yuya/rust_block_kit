use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ViewsOpenResponse {
    pub ok: bool,
    pub view: Option<String>,
    pub error: Option<String>,
    pub response_metadata: Option<ViewsOpenResponseMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct ViewsOpenResponseMetadata {
    pub messages: Vec<String>,
}