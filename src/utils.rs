use winit::window::Icon;

pub fn load_icon(path: &str) -> Icon {
    let img = image::open(path).unwrap().to_rgba8();
    let (width, height) = img.dimensions();
    let rgba = img.into_raw();
    Icon::from_rgba(rgba, width, height).unwrap()
}
