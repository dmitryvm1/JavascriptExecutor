use serde_derive::{Deserialize, Serialize};

const DEFAULT_FILE_PATH: &'static str = "ui_state.json";

#[derive(Serialize, Deserialize, Default)]
pub struct PersistentState<S> {
    inner: S
}

impl<'de, S: serde::Serialize + Default + serde::de::DeserializeOwned> PersistentState<S> {
    pub fn save(&self, path: Option<&std::path::Path>) -> Result<(), std::io::Error> {
        let contents = serde_json::to_vec(&self.inner)?;
        let path = path.unwrap_or(std::path::Path::new(DEFAULT_FILE_PATH));
        std::fs::write(path, &contents)
    }

    pub fn load(path: Option<&std::path::Path>) -> Result<Self, std::io::Error> {
        let path = path.unwrap_or(std::path::Path::new(DEFAULT_FILE_PATH));
        if path.exists() {
            let contents = std::fs::read_to_string(path)?;
            let inner = serde_json::from_str::<S>(&contents).unwrap_or_default();
            return Ok(PersistentState { inner })
        } else {
            let state = Self::default();
            state.save(None)?;
            Ok(state)
        }
    }

    pub fn inner_mut(&mut self) -> &mut S {
        &mut self.inner
    }

    pub fn inner(&self) -> &S {
        &self.inner
    }
}