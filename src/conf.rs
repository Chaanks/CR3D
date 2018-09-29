pub struct Conf {
    pub vsync: bool,
    pub title: String,
    pub width: f64,
    pub height: f64,
}

// Config is used to specify window setup
impl Conf {
    pub fn new(title: String, height: f64, width: f64, vsync: bool) -> Self {
        Self {
            title,
            width,
            height,
            vsync,
        }
    }
}