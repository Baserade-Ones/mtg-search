use eframe::egui;
use egui::{
    load::{BytesPoll, ImageLoadResult, ImageLoader, ImagePoll, LoadError, SizeHint},
    mutex::Mutex,
    ColorImage,
};
use std::{collections::HashMap, mem::size_of, sync::Arc};

pub fn load_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    let fallback = fonts.families.get(&egui::FontFamily::Proportional).unwrap()[0].clone();

    fonts.font_data.insert(
        "Mana".to_owned(),
        egui::FontData::from_static(include_bytes!("../mana/fonts/mana.ttf")),
    );
    fonts.families.insert(
        egui::FontFamily::Name("Mana".into()),
        vec!["Mana".to_owned(), fallback.clone()],
    );

    ctx.set_fonts(fonts);
}

/// Simplified rewrite if egui_extras::ImageCrateLoader which refuses to work on WASM for unclear
/// reasons
#[derive(Default)]
pub struct Image {
    cache: Mutex<HashMap<String, Arc<ColorImage>>>,
}

impl Image {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ImageLoader for Image {
    fn id(&self) -> &str {
        "KMS"
    }

    fn load(&self, ctx: &egui::Context, uri: &str, _: SizeHint) -> ImageLoadResult {
        let mut cache = self.cache.lock();
        if let Some(image) = cache.get(uri).cloned() {
            Ok(ImagePoll::Ready { image })
        } else {
            match ctx.try_load_bytes(uri) {
                Ok(BytesPoll::Ready { bytes, .. }) => {
                    let image = image::load_from_memory(&bytes)
                        .map_err(|err| LoadError::Loading(err.to_string()))?;
                    let size = [image.width() as _, image.height() as _];
                    let image_buffer = image.to_rgba8();
                    let pixels = image_buffer.as_flat_samples();
                    let image = Arc::new(egui::ColorImage::from_rgba_unmultiplied(
                        size,
                        pixels.as_slice(),
                    ));

                    cache.insert(uri.into(), image.clone());
                    Ok(ImagePoll::Ready { image })
                }
                Ok(BytesPoll::Pending { size }) => Ok(ImagePoll::Pending { size }),
                Err(err) => Err(err),
            }
        }
    }

    fn forget(&self, uri: &str) {
        let _ = self.cache.lock().remove(uri);
    }

    fn forget_all(&self) {
        self.cache.lock().clear();
    }

    fn byte_size(&self) -> usize {
        self.cache
            .lock()
            .values()
            .map(|image| image.pixels.len() * size_of::<egui::Color32>())
            .sum()
    }
}
