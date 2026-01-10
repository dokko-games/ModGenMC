use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Generator {
    pub domain: String,
    pub name: String
}
pub trait Generate {
    fn generate(&self);
}
impl Generate for Generator {
    fn generate(&self) {
        println!("Running Generator {}/{}...", self.name, self.domain);
    }
}