use anyhow::Result;
use serde::de::DeserializeOwned;
use ureq::Agent;

pub trait Callable {
    fn agent(&self) -> Agent;
    fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T>;
}
