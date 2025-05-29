use crate::app::App;
use crate::collection::get_collections;
use anyhow::{anyhow, Result};

pub async fn start_web(document: &web_sys::Document) -> Result<()> {
    let data = get_collections().await?;

    let canvas = document
        .get_element_by_id("the_canvas_id")
        .ok_or_else(|| anyhow!("Could not find canvas"))?;

    let web_options = eframe::WebOptions::default();
    let start_result = eframe::WebRunner::new()
        .start(
            web_sys::HtmlCanvasElement::from(wasm_bindgen::JsValue::from(canvas)),
            web_options,
            App::creator(data),
        )
        .await
        .map_err(|e| anyhow!("{:?}", e))?;

    Ok(())
}
