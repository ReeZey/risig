use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Fish {
    pub(crate) weight: u8,
    pub(crate) length: u8,
    pub(crate) fish_type: FishType 
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum FishType {
    Herring
}

impl Display for FishType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            FishType::Herring => write!(f, "Herring"),
        }
    }
}