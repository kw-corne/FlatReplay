use std::collections::HashMap;
use std::{fs, path::Path};

use anyhow::Result;
use bimap::BiMap;
use serde::Serialize;

use boxcars::{ActorId, ObjectId, Quaternion, Replay, Vector3f};

pub mod rocket_league;
use rocket_league::RLObject;

pub mod replay_header;
use replay_header::ReplayHeader;

#[derive(Serialize, Default, Debug)]
struct ObjectData {
    position: Option<Vector3f>,
    rotation: Option<Quaternion>,
}

#[derive(Serialize, Default, Debug)]
pub struct Frame {
    delta: f32, // ms since last frame
    objects: HashMap<ObjectId, ObjectData>,
}

impl Frame {
    fn new(delta: f32) -> Self {
        Frame {
            delta: delta * 1000.0,
            objects: HashMap::new(),
        }
    }
}

#[derive(Serialize, Default, Debug)]
pub struct FlatReplay {
    pub header: ReplayHeader,
    pub frames: Vec<Frame>,
    pub objects: BiMap<ObjectId, RLObject>,

    is_built: bool,
    this_iter_frames: i32,
}

impl FlatReplay {
    fn build(&mut self, replay: &Replay) {
        if self.is_built {
            panic!("Attempt to build when already built.");
        }

        self.map_objects(&replay);
        let mut object_map = self.get_inital_object_map();
        let mut actor_map: BiMap<ObjectId, ActorId> = BiMap::new();

        let network_frames = replay
            .network_frames
            .as_ref()
            .expect("Replay should contain network frames.");

        for frame in &network_frames.frames {
            self.frames.push(Frame::new(frame.delta));

            self.update_actor_map(&mut actor_map, &frame.new_actors);

            for attr in &frame.updated_actors {}

            // for actor in &frame.deleted_actors {
            //
            // }
        }

        self.is_built = true;
    }

    fn update_actor_map(
        &self,
        actor_map: &mut BiMap<ObjectId, ActorId>,
        new_actors: &Vec<boxcars::NewActor>,
    ) {
        for actor in new_actors {
            if !self.objects.contains_left(&actor.object_id) {
                continue; // its not an object were tracking
            }

            actor_map.insert(actor.object_id, actor.actor_id);
        }
    }

    // Returns a map of every ObjectId (that is tracked) to a default ObjectData struct
    fn get_inital_object_map(&self) -> HashMap<ObjectId, ObjectData> {
        let mut m: HashMap<ObjectId, ObjectData> = HashMap::new();

        for obj_id in self.objects.left_values() {
            m.insert(*obj_id, ObjectData::default());
        }

        m
    }

    // Find the ObjectIds of the objects we care about
    fn map_objects(&mut self, replay: &Replay) {
        for (i, obj_name) in replay.objects.iter().enumerate() {
            if let Ok(obj) = RLObject::try_from(obj_name.as_str()) {
                self.objects.insert(ObjectId(i as i32), obj);
            }
        }
    }
}

impl TryFrom<&Path> for FlatReplay {
    type Error = anyhow::Error;

    fn try_from(path: &Path) -> Result<Self> {
        let data = fs::read(path)?;
        Self::try_from(data.as_slice())
    }
}

impl TryFrom<&[u8]> for FlatReplay {
    type Error = anyhow::Error;

    fn try_from(data: &[u8]) -> Result<Self> {
        let replay = boxcars::ParserBuilder::new(&data)
            .always_check_crc()
            .must_parse_network_data()
            .parse()?;

        Ok(Self::from(&replay))
    }
}

impl From<&Replay> for FlatReplay {
    fn from(replay: &Replay) -> Self {
        let header = ReplayHeader::from(replay);

        let mut fr = FlatReplay {
            header,
            ..Default::default()
        };

        fr.build(replay);

        fr
    }
}
