pub enum GHWSection {
    Null,
    String,
    Hierarchy,
    Type,
    WellKnownType,
    EOH,
    Snapshot,
    Cycle,
    Directory,
    Tailer,
}

impl From<i32> for GHWSection {
    fn from(value: i32) -> Self {
        match value {
            0 => GHWSection::Null,
            1 => GHWSection::String,
            2 => GHWSection::Hierarchy,
            3 => GHWSection::Type,
            4 => GHWSection::WellKnownType,
            5 => GHWSection::EOH,
            6 => GHWSection::Snapshot,
            7 => GHWSection::Cycle,
            8 => GHWSection::Directory,
            9 => GHWSection::Tailer,
            _ => panic!("Cannot convert {} to GHWSection", value),
        }
    }
}
