pub mod rl_map;

use anyhow::anyhow;
use serde::Serialize;

#[derive(Serialize, Debug, Hash, PartialEq, Eq)]
pub enum RLObject {
    Ball,
    Car,
}

impl RLObject {
    const fn name(&self) -> &str {
        match *self {
            RLObject::Ball => "Archetypes.Ball.Ball_Default",
            RLObject::Car => "Archetypes.Car.Car_Default",
        }
    }
}

impl TryFrom<&str> for RLObject {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Archetypes.Ball.Ball_Default" => Ok(RLObject::Ball),
            "Archetypes.Car.Car_Default" => Ok(RLObject::Car),
            _ => Err(anyhow!("Can't create RLObject from '{s}' (yet).")),
        }
    }
}

#[derive(Serialize, Debug, Hash, PartialEq, Eq)]
pub enum Platform {
    Steam,
    Epic,
    Playstation,
    Xbox,
    Switch,
}
