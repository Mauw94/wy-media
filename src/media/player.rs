use std::{
    fs::File,
    io::BufReader,
    ops::Add,
    path::Path,
    time::{Duration, Instant},
};

use rodio::{cpal, Decoder, OutputStream, OutputStreamHandle, Sink};
use tui::widgets::ListState;

use super::media::{self, Media};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayStatus {
    Waiting,
    Playing(Instant, Duration),
    Stopped(Duration),
}

pub struct PlayListItem {
    pub name: String,
    pub duration: Duration,
    pub current_pos: Duration,
    pub status: PlayStatus,
    pub path: String,
}

pub struct PlayList {
    pub lists: Vec<PlayListItem>,
}

pub trait Player {
    fn new() -> Self;
    fn add_to_list(&mut self, media: Media, once: bool) -> bool;
    fn play(&mut self) -> bool;
    fn next(&mut self) -> bool;
    fn stop(&mut self) -> bool;
    fn pause(&mut self) -> bool;
    fn resume(&mut self) -> bool;
    fn get_progress(&self) -> (f32, f32);
    fn is_playing(&self) -> bool;
    fn tick(&mut self);
    fn volume(&self) -> f32;
    fn set_volume(&mut self, new_volume: f32) -> bool;
}

pub struct MusicPlayer {
    pub current_time: Duration,
    pub total_time: Duration,
    pub play_list: PlayList,
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Sink,
    initialized: bool,
}

impl Player for MusicPlayer {
    fn new() -> Self {
        for dev in cpal::available_hosts() {
            println!("{:?}", dev);
        }
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Self {
            current_time: Duration::from_secs(0),
            total_time: Duration::from_secs(0),
            play_list: PlayList { lists: vec![] },
            stream,
            stream_handle,
            sink,
            initialized: false,
        }
    }

    fn add_to_list(&mut self, media: Media, once: bool) -> bool {
        match media.src {
            media::Source::Local(path) => {
                return self.play_with_file(path, once);
            }
            // media::Source::M3u8(_path) => false,
        }
    }

    fn play(&mut self) -> bool {
        self.sink.play();
        if let Some(item) = self.play_list.lists.first_mut() {
            let status = &mut item.status;
            match status {
                PlayStatus::Waiting => {
                    *status = PlayStatus::Playing(Instant::now(), Duration::from_nanos(0));
                }
                PlayStatus::Playing(_, _) => {}
                PlayStatus::Stopped(duration) => {
                    *status = PlayStatus::Playing(Instant::now(), *duration);
                }
            }
        }
        true
    }

    fn next(&mut self) -> bool {
        let len = self.play_list.lists.len();
        if len >= 1 {
            self.play_list.lists.remove(0);
            self.stop();
            if !self.play_list.lists.is_empty() {
                let top_music = self.play_list.lists.first().unwrap();
                let f = File::open(top_music.path.as_str()).unwrap();
                let buf_reader = BufReader::new(f);
                let (stream, stream_handle) = OutputStream::try_default().unwrap();
                self.stream = stream;
                self.stream_handle = stream_handle;
                let volume = self.volume();
                self.sink = Sink::try_new(&self.stream_handle).unwrap();
                self.set_volume(volume);
                self.sink.append(Decoder::new(buf_reader).unwrap());
                self.play();
            }
        } else {
            // nothing in playlist
            return false;
        }
        true
    }

    fn stop(&mut self) -> bool {
        self.sink.stop();
        true
    }

    fn pause(&mut self) -> bool {
        self.sink.pause();
        if let Some(item) = self.play_list.lists.first_mut() {
            let status = &mut item.status;
            match status {
                PlayStatus::Waiting => {}
                PlayStatus::Playing(instant, duration) => {
                    *status = PlayStatus::Stopped(duration.add(instant.elapsed()));
                }
                PlayStatus::Stopped(_) => {}
            }
        }
        true
    }

    fn resume(&mut self) -> bool {
        self.sink.play();
        if let Some(item) = self.play_list.lists.first_mut() {
            let status = &mut item.status;
            match status {
                PlayStatus::Waiting => {}
                PlayStatus::Playing(_, _) => {}
                PlayStatus::Stopped(duration) => {
                    *status = PlayStatus::Playing(Instant::now(), *duration);
                }
            }
        }
        true
    }

    fn get_progress(&self) -> (f32, f32) {
        (0.0, 0.0)
    }

    fn is_playing(&self) -> bool {
        self.initialized && !self.sink.is_paused() && !self.play_list.lists.is_empty()
    }

    fn tick(&mut self) {
        let is_playing = self.is_playing();
        if let Some(song) = self.play_list.lists.first_mut() {
            let status = &mut song.status;
            match status {
                PlayStatus::Waiting => {
                    if is_playing {
                        *status = PlayStatus::Playing(Instant::now(), Duration::from_nanos(0));
                    }
                }
                PlayStatus::Playing(instant, duration) => {
                    let now = instant.elapsed().add(duration.clone());
                    if now.ge(&song.duration) {
                        self.next();
                    } else {
                        self.current_time = now;
                        self.total_time = song.duration.clone();
                    }
                }
                PlayStatus::Stopped(duration) => {
                    self.current_time = duration.clone();
                    self.total_time = song.duration.clone();
                }
            }
        } else {
            if self.play_list.lists.is_empty() {
                self.stop();
            }
        }
    }

    fn volume(&self) -> f32 {
        self.sink.volume()
    }

    fn set_volume(&mut self, new_volume: f32) -> bool {
        self.sink.set_volume(new_volume);
        true
    }
}

impl MusicPlayer {
    pub fn playing_song(&self) -> Option<&PlayListItem> {
        self.play_list.lists.first()
    }

    fn play_with_file(&mut self, path: String, once: bool) -> bool {
        let duration: Duration;
        if path.ends_with(".mp3") {
            let dur = mp3_duration::from_path(path.clone());
            match dur {
                Ok(dur) => {
                    duration = dur;
                }
                Err(err) => {
                    duration = err.at_duration;
                    if duration.is_zero() {
                        return false;
                    }
                }
            }
        } else {
            // if let Ok(f) = File::open(path.as_str()) {
            //     let dec = Decoder::new(f);
            //     if let Ok(dec) = dec {
            //         if let Some(dur) = dec.total_duration() {
            //             duration = dur;
            //         } else {
            //             return false;
            //         }
            //     } else {
            //         return false;
            //     }
            // } else {
            //     return false;
            // }
            return true;
        }
        match File::open(path.as_str()) {
            Ok(f) => {
                let path = Path::new(path.as_str());
                let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                if once || self.play_list.lists.is_empty() {
                    self.stop();
                    let buf_reader = BufReader::new(f);
                    let sink = self.stream_handle.play_once(buf_reader).unwrap();
                    self.sink = sink;
                    self.play_list.lists.clear();
                }
                let mut state = ListState::default();
                state.select(Some(0));
                self.play_list.lists.push(PlayListItem {
                    name: file_name,
                    duration,
                    current_pos: Duration::from_secs(0),
                    status: PlayStatus::Waiting,
                    path: path.to_string_lossy().to_string(),
                });
                if !self.initialized {
                    self.initialized = true;
                }
                self.play();
                self.tick();
                return true;
            }
            Err(_) => false,
        }
    }
}

impl Drop for MusicPlayer {
    fn drop(&mut self) {}
}
