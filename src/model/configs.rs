#[derive(Debug)]
pub struct Configs {
    pub focus : bool
}

impl Default for Configs {
    fn default() -> Self {
        Configs { focus: true }
    }
}
