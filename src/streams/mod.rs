use ffmpeg_next::Error;

pub struct Streams {
    duratiion_s: i32,
    stream_url: String,
}

pub trait Streamable {
    fn transcode(&self, bitrate: i32) -> Result<i32, Error>;
    fn total_byte_len(&self, bitrate: i32) -> i64;
}

impl Streams {
    pub fn new(duratiion_s: i32, stream_url: String) -> Self { Self { duratiion_s, stream_url } }

    pub fn duratiion_s(&self) -> i32 {
        self.duratiion_s
    }

    pub fn stream_url(&self) -> &str {
        self.stream_url.as_ref()
    }
}

impl Streamable for Streams {
    fn transcode(&self, bitrate: i32) -> Result<i32, Error> {
        todo!()
    }

    fn total_byte_len(&self, bitrate: i32) -> i64 {
        todo!()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    # [test]
    fn check_correct_total_byte_len() {
        let duration = 60;
        let stream_url = "http://url.mp3".to_owned();
        let stream = Streams::new(duration, stream_url);
        let bitarate = 100;
        let stream_byte_len = stream.total_byte_len(bitarate);

        assert_eq!(stream_byte_len, i64::from(bitarate * duration))
    }
}