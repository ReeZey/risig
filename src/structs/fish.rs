use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use serde::Deserialize;
use serde::Serialize;
use strum_macros::EnumIter;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Fish {
    pub(crate) weight: u8,
    pub(crate) length: u8,
    pub(crate) fish_type: FishType 
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumIter)]
pub(crate) enum FishType {
    Herring,
    Salmon,
    Tuna,
    Trout,
    Cod,
    Bass,
    Halibut,
    Snapper,
    Grouper,
    Flounder,
    Mackerel,
    Sardine,
    Pike,
    Catfish,
    Swordfish,
    Tilapia,
    Perch,
    Carp,
    Sole,
    Anchovy,
    RedSnapper,
    RainbowTrout,
    Bluefish,
    Marlin,
    Whitefish,
    Wahoo,
    Rockfish,
    Barracuda,
    Amberjack,
    Pompano,
    Monkfish,
    Haddock,
    Cobia,
    Tilefish,
    Lingcod,
    Pollock,
    Kingfish,
    Sheepshead,
    Triggerfish,
    Butterfish,
    Wrasse,
    Drum,
    Sturgeon,
}

impl Display for FishType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            FishType::Herring => write!(f, "Herring"),
            FishType::Salmon => write!(f, "Salmon"),
            FishType::Tuna => write!(f, "Tuna"),
            FishType::Trout => write!(f, "Trout"),
            FishType::Cod => write!(f, "Cod"),
            FishType::Bass => write!(f, "Bass"),
            FishType::Halibut => write!(f, "Halibut"),
            FishType::Snapper => write!(f, "Snapper"),
            FishType::Grouper => write!(f, "Grouper"),
            FishType::Flounder => write!(f, "Flounder"),
            FishType::Mackerel => write!(f, "Mackerel"),
            FishType::Sardine => write!(f, "Sardine"),
            FishType::Pike => write!(f, "Pike"),
            FishType::Catfish => write!(f, "Catfish"),
            FishType::Swordfish => write!(f, "Swordfish"),
            FishType::Tilapia => write!(f, "Tilapia"),
            FishType::Perch => write!(f, "Perch"),
            FishType::Carp => write!(f, "Carp"),
            FishType::Sole => write!(f, "Sole"),
            FishType::Anchovy => write!(f, "Anchovy"),
            FishType::RedSnapper => write!(f, "Red Snapper"),
            FishType::RainbowTrout => write!(f, "Rainbow Trout"),
            FishType::Bluefish => write!(f, "Bluefish"),
            FishType::Marlin => write!(f, "Marlin"),
            FishType::Whitefish => write!(f, "Whitefish"),
            FishType::Wahoo => write!(f, "Wahoo"),
            FishType::Rockfish => write!(f, "Rockfish"),
            FishType::Barracuda => write!(f, "Barracuda"),
            FishType::Amberjack => write!(f, "Amberjack"),
            FishType::Pompano => write!(f, "Pompano"),
            FishType::Monkfish => write!(f, "Monkfish"),
            FishType::Haddock => write!(f, "Haddock"),
            FishType::Cobia => write!(f, "Cobia"),
            FishType::Tilefish => write!(f, "Tilefish"),
            FishType::Lingcod => write!(f, "Lingcod"),
            FishType::Pollock => write!(f, "Pollock"),
            FishType::Kingfish => write!(f, "Kingfish"),
            FishType::Sheepshead => write!(f, "Sheepshead"),
            FishType::Triggerfish => write!(f, "Triggerfish"),
            FishType::Butterfish => write!(f, "Butterfish"),
            FishType::Wrasse => write!(f, "Wrasse"),
            FishType::Drum => write!(f, "Drum"),
            FishType::Sturgeon => write!(f, "Sturgeon"),
        }
    }
}