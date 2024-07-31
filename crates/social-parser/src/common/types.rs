use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Source {
    Meta,
    Discord,
}

pub struct Archive {
    source: Source,
    // connections: Option<Connections>,
}

// TODO: Complete
