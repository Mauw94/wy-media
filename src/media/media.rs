pub enum Source {
    // M3u8(RadioConfig),
    Local(String),
}

pub struct Media {
    pub src: Source,
}
