#[derive(Debug, Clone)]
pub struct CommonArgs {
    pub console: bool,
    pub gfx_api: Option<String>,
    // pub level: Option<String>,
    // pub vehicle: Option<String>,
}

impl CommonArgs {
    pub fn to_args(&self) -> Vec<String> {
        let mut a: Vec<String> = vec![];

        if self.console {
            a.push("-console".to_string())
        };
        if let Some(gfx_api) = &self.gfx_api {
            a.push("-gfx".to_string());
            a.push(gfx_api.to_string())
        };

        a
    }
}
