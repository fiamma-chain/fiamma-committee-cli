#[derive(Debug)]
pub struct Signer {}

impl Default for Signer {
    fn default() -> Self {
        Self::new()
    }
}

impl Signer {
    pub fn new() -> Self {
        Signer {}
    }
}
