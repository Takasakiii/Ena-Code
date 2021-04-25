use serde::Deserialize;

#[derive(Deserialize)]
pub struct Enarc {
    pub ena_home_path: String
}

impl Enarc {
    pub fn from(yml: String) -> Result<Self, serde_yaml::Error> {
        let yml = &yml[..];
        serde_yaml::from_str::<Self>(yml)
    }
}
