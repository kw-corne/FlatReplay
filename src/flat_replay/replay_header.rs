use super::rocket_league::rl_map::RLMap;
use boxcars::Replay;
use chrono::NaiveDateTime;
use serde::Serialize;
use std::str::FromStr;

#[derive(Serialize, Default, Debug)]
pub struct ReplayHeader {
    pub date: Option<NaiveDateTime>,
    pub game_type: Option<String>, // todo: enums of game types (same as RLMap)
    pub map: Option<RLMap>,
    pub name: Option<String>,
}

impl From<&Replay> for ReplayHeader {
    fn from(replay: &Replay) -> Self {
        let mut header = ReplayHeader {
            game_type: Some(replay.game_type.clone()),
            ..Default::default()
        };

        for (name, prop) in &replay.properties {
            match name.as_str() {
                "Date" => {
                    if let Some(date) = prop.as_string() {
                        if let Ok(d) = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H-%M-%S") {
                            header.date = Some(d);
                        }
                    }
                }
                "MapName" => {
                    if let Some(map) = prop.as_string() {
                        if let Ok(game_map) = RLMap::from_str(map) {
                            header.map = Some(game_map);
                        }
                    }
                }
                "ReplayName" => {
                    if let Some(name) = prop.as_string() {
                        header.name = Some(String::from(name));
                    }
                }
                _ => (),
            }
        }

        header
    }
}
