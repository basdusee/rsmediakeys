
use std::os::unix::net::UnixStream;
use std::io::prelude::*;

// Create an easy alias for all the error handling stuff
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// The state (stop, pause, play) of MPD is stored in an enum when asked from the daemon
pub enum StateOfPlay {
    Stop,
    Pause,
    Play,
}

/// This struct stores a buffer and a Unix socket to talk to MPD.
///
/// we use a Unix socket to communicate to the Music Player Daemon.
/// It's a cleartext protocol and MPD talks back, which is cosy.
/// The connection struct and implementation of it, are the backbone of this lib.
/// 
/// Use Connect::new("/home/user/.mpd/socket/") to create a new connection.
pub struct Connection {
    stream: UnixStream,
    buffer: [u8; 2048],
}

impl Connection {
    /// Connects to MPD and stores the connection details
    ///
    /// You first have to connect to MPD in order to control it.
    /// Start a new connection wth this "new" function.
    ///
    /// # Arguments
    /// * 'sock' - A string slice containing the path to the MPD Unix socket.
    ///
    /// # Returns
    /// You get a Connection struct back, with connection and buffer in it.
    /// It's inside a Result type, so errors propagate upstream with this one.
    pub fn new(sock: &str) -> Result<Connection> {

        let mut newsock = UnixStream::connect(sock)?;
        let mut newbuffer = [0u8; 2048];

        // read the inital "OK MPD 1.2.3" string from the socket
        match newsock.read(&mut newbuffer) {
            Ok(value) => if !newbuffer.starts_with(b"OK MPD") { 
                             panic!("Could connect but MPD is not okay:\n{}", value);
                         }
            Err(error) => panic!("Could not connect to socket!\n{}", error),
        }

        Ok(Connection {
            stream: newsock,
            buffer: newbuffer, 
        })
    }

    fn _get_status(&mut self) -> Result<MpdStatus> {
        // ask for the current status
        self.stream.write_all(b"status\n")?;
        self.stream.read(&mut self.buffer)?;
        let status = std::str::from_utf8(&self.buffer)?;

        let mut result = MpdStatus::new();
        for statusline in status.split('\n') {
            let mut parsedline = statusline.split(':');
            let varname = parsedline.next().unwrap_or("").trim();
            let value = parsedline.next().unwrap_or("").trim();
            match varname { 
                "volume" => result.volume = value.parse()?, 
                "repeat" => result.repeat = value.parse()?,
                "random" => result.random = value.parse()?,
                "single" => result.single = value.parse()?,
                "consume" => result.consume = value.parse()?,
                "partition" => result.partition = value.to_string(),
                "playlist" => result.playlist = value.parse()?,
                "playlistlength" => result.playlistlength = value.parse()?,
                "mixrampdb" => result.mixrampdb = value.parse()?,
                "state" => match value { 
                               "stop" =>  result.state = StateOfPlay::Stop,
                               "pause" =>  result.state = StateOfPlay::Pause,
                               "play" =>  result.state = StateOfPlay::Play,
                               _ => {}
                           }
                "song" => result.song = value.parse()?,
                "songid" => result.songid = value.parse()?,
                "time" => result.time = value.to_string(),
                "elapsed" => result.elapsed = value.parse()?,
                "bitrate" => result.bitrate = value.parse()?,
                "duration" => result.duration = value.parse()?,
                "audio" => result.audio = value.to_string(),
                "nextsong" => result.nextsong = value.parse()?,
                "nextsongid" => result.nextsongid = value.parse()?,
                _ => {} 
            }
        }
        Ok(result)
    }

    fn _get_song(&mut self) -> Result<Song> {
        // ask for the current song
        self.stream.write_all(b"currentsong\n")?;
        self.stream.read(&mut self.buffer)?;
        let song = std::str::from_utf8(&self.buffer)?;

        let mut result = Song::new();
        for songline in song.split('\n') {
            let mut parsedline = songline.split(':');
            let varname = parsedline.next().unwrap_or("").trim();
            let value = parsedline.next().unwrap_or("").trim();
            match varname { 
                "file" => result.file =  value.to_string(),
                "Last-Modified" => result.last_modified =  value.to_string(),
                "Artist" => result.artist =  value.to_string(),
                "Title" => result.title =  value.to_string(),
                "Album" => result.album =  value.to_string(),
                "Track" => result.track =  value.parse()?,
                "Date" => result.date =  value.parse()?,
                "Genre" => result.genre =  value.to_string(),
                "Time" => result.time =  value.parse()?,
                "duration" => result.duration =  value.parse()?,
                "Pos" => result.pos =  value.parse()?,
                "Id" => result.id =  value.parse()?,
                _ => {} 
            }
        }
        Ok(result)
    }

    fn _command(&mut self, command: &str) -> Result<(String, StateOfPlay)> {
        self.stream.write_all(format!("{}\n", command).as_bytes())?;
        match self.stream.read(&mut self.buffer) {
            Ok(_value) => if self.buffer.starts_with(b"ACK") { 
                              let index: usize = self.buffer.iter().position(|&x| x == b'}').unwrap();
                              let end: usize = self.buffer.iter().position(|&x| x == b'\n').unwrap();
                              let message = std::str::from_utf8(&self.buffer[index+2..end])?;
                              Ok((message.to_string(), self._get_status()?.state))
                          } 
                          else if self.buffer.starts_with(b"OK") { 
                              let nowsong = self._get_song()?;
                              let track = format!("{} - {}", nowsong.artist, nowsong.title);
                              Ok((track, self._get_status()?.state))
                          } else {
                              panic!("got a response from MPD which I do not understand");
                          } 
            Err(error) => panic!("Could not connect to socket!\n{}", error),
        }

    }

    /// This function asks MPD to skip to the next song in the playlist
    ///
    /// # Returns
    /// A tuple (artist-title, state) with:
    /// * the artist-title of the new song (the next song, so to speak).
    /// * the state of MPD (play/pause/stopped).
    /// or it returns an "error" message instead of artist-title if MPD is mad at you.
    /// You can't "next" for example if MPD is not playing anything at all.
    pub fn next(&mut self) -> Result<(String, StateOfPlay)> {
        self._command("next")
    }

    /// This function asks MPD to skip back to the  previous song in the playlist
    ///
    /// # Returns
    /// A tuple (artist-title, state) with:
    /// * the artist-title of the new song (the previous song, so to speak).
    /// * the state of MPD (play/pause/stopped).
    /// or it returns an "error" message instead of artist-title if MPD is mad at you.
    /// You can't "previous" for example if MPD is not playing anything at all.
    pub fn previous(&mut self) -> Result<(String, StateOfPlay)> {
        self._command("previous")
    }

    /// Asks MPD to stop playing music 
    ///
    /// # Returns
    /// A tuple (artist-title, state) with:
    /// * the artist-title of the song where it stopped.
    /// * the state of MPD (which should be StateOfPlay::Stop).
    pub fn stop(&mut self) -> Result<(String, StateOfPlay)> {
        self._command("stop")
    }

    /// If music is playing, it pauses it. else it starts playing music
    ///
    /// this function is specially designed for keyboards with a combined
    /// play/pause mediakey. Look for a key with both a play triangle and the
    /// two stripes pause logo. This function does what should happen if you press that button.
    /// It starts playing if it isn't and it pauses playing if it is.
    ///
    /// # Returns
    /// A tuple (artist-title, state) with:
    /// * the artist-title of the new song (the previous song, so to speak).
    /// * the state of MPD (should be Play or Pause from the StateOfPlay enum).
    /// or it returns an "error" message instead of artist-title if MPD is mad at you.
    pub fn toggle(&mut self) -> Result<(String, StateOfPlay)> {
        let status = self._get_status()?;
        // below looks weird, but the "pause" command actually toggles play/pause
        // in mpd. "play" only starts playing if stopped.
        match status.state {
            StateOfPlay::Stop => self._command("play"),
            StateOfPlay::Pause => self._command("pause"),
            StateOfPlay::Play => self._command("pause"),
        }
    }
}

struct MpdStatus {
    volume: usize,
    repeat: usize,
    random: usize,
    single: usize,
    consume: usize,
    partition: String,
    playlist: usize,
    playlistlength: usize,
    mixrampdb: f32,
    state: StateOfPlay,
    song: usize,
    songid: usize,
    time: String,
    elapsed: f32,
    bitrate: usize,
    duration: f32,
    audio: String,
    nextsong: usize,
    nextsongid: usize,
}

impl MpdStatus { 
    fn new() -> MpdStatus {
            MpdStatus {
                volume: 100, 
                repeat: 0,
                random: 0,
                single: 0,
                consume: 0,
                partition: String::new(),
                playlist: 0,
                playlistlength: 0,
                mixrampdb: 0.0f32,
                state: StateOfPlay::Stop,
                song: 0,
                songid: 0,
                time: String::new(),
                elapsed: 0.0f32,
                bitrate: 0,
                duration: 0.0f32,
                audio: String::new(),
                nextsong: 0,
                nextsongid: 0,
        }
    }
}

struct Song {
    file: String,
    last_modified: String,
    artist: String,
    title: String,
    album: String,
    track: usize,
    date: usize,
    genre: String,
    time: usize,
    duration: f32,
    pos: usize,
    id: usize,
}

impl Song { 
    fn new() -> Song {
            Song {
                file: String::new(),
                last_modified: String::new(),
                artist: String::new(),
                title: String::new(),
                album: String::new(),
                track: 0,
                date: 0,
                genre: String::new(),
                time: 0,
                duration: 0.0f32,
                pos: 0,
                id: 0,
            }
    }
}
