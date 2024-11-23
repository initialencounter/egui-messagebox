pub async fn load_icon_from_url(url: &str) -> egui::IconData {
    if url.is_empty() {
        return egui::IconData::default();
    }
    if let Ok(response) = reqwest::get(url).await {
        let bytes = response.bytes().await.unwrap();
        return if let Ok(image) = image::load_from_memory(&bytes) {
            let image = image.into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            egui::IconData {
                rgba,
                width,
                height,
            }
        } else {
            egui::IconData::default()
        };
    }
    egui::IconData::default()
}

pub fn load_icon_from_bytes(bytes: &[u8]) -> egui::IconData {
    if let Ok(image) = image::load_from_memory(bytes) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        return egui::IconData {
            rgba,
            width,
            height,
        };
    }
    egui::IconData::default()
}

pub fn load_icon_from_path(path: &str) -> egui::IconData {
    if let Ok(image) = image::open(path) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        return egui::IconData {
            rgba,
            width,
            height,
        };
    }
    egui::IconData::default()
}
