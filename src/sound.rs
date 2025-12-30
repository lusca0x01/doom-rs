use rodio::{OutputStream, OutputStreamBuilder, Sink};
use std::cell::UnsafeCell;
use std::time::Duration;

const DOOM_SAMPLE_RATE: u32 = 11025;
const NUM_CHANNELS: usize = 8;

struct SoundSystem {
    stream_handle: OutputStream,
    channels: Vec<Option<Sink>>,
}

struct UnsafeSoundSystem(UnsafeCell<Option<SoundSystem>>);
unsafe impl Sync for UnsafeSoundSystem {}

static SOUND_SYSTEM: UnsafeSoundSystem = UnsafeSoundSystem(UnsafeCell::new(None));

fn get_sound_system_mut() -> Option<&'static mut SoundSystem> {
    unsafe { (&mut *SOUND_SYSTEM.0.get()).as_mut() }
}

fn get_sound_system_ref() -> Option<&'static SoundSystem> {
    unsafe { (&*SOUND_SYSTEM.0.get()).as_ref() }
}

fn convert_doom_sound(data: &[u8]) -> Option<Vec<f32>> {
    if data.len() < 8 {
        return None;
    }
    let format = u16::from_le_bytes([data[0], data[1]]);
    if format != 3 {
        return None;
    }
    let _sample_rate = u16::from_le_bytes([data[2], data[3]]);
    let num_samples = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;
    if data.len() < 8 + num_samples {
        return None;
    }
    let samples: Vec<f32> = data[8..8 + num_samples]
        .iter()
        .map(|&s| ((s as f32 - 128.0) / 128.0).clamp(-1.0, 1.0))
        .collect();
    Some(samples)
}

struct DoomSound {
    samples: Vec<f32>,
    sample_rate: u32,
    position: usize,
}

impl DoomSound {
    fn new(samples: Vec<f32>, sample_rate: u32) -> Self {
        Self {
            samples,
            sample_rate,
            position: 0,
        }
    }
}

impl Iterator for DoomSound {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.samples.len() {
            let sample = self.samples[self.position];
            self.position += 1;
            Some(sample)
        } else {
            None
        }
    }
}

impl rodio::Source for DoomSound {
    fn current_span_len(&self) -> Option<usize> {
        Some(self.samples.len() - self.position)
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f64(
            (self.samples.len() as f64) / (self.sample_rate as f64),
        ))
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_sound_init() -> i32 {
    match OutputStreamBuilder::open_default_stream() {
        Ok(stream_handle) => {
            let mut channels = Vec::with_capacity(NUM_CHANNELS);
            for _ in 0..NUM_CHANNELS {
                channels.push(None);
            }
            let system = SoundSystem {
                stream_handle,
                channels,
            };

            unsafe {
                *SOUND_SYSTEM.0.get() = Some(system);
            }
            1
        }
        Err(e) => {
            eprintln!("[doom-rs] Failed to initialize sound: {}", e);
            0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_sound_shutdown() {
    unsafe {
        *SOUND_SYSTEM.0.get() = None;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_sound_start(
    data: *const u8,
    len: u32,
    channel: i32,
    vol: i32,
    _sep: i32,
) -> i32 {
    if data.is_null() || len == 0 {
        return -1;
    }
    let channel = channel as usize;
    if channel >= NUM_CHANNELS {
        return -1;
    }
    let slice = unsafe { std::slice::from_raw_parts(data, len as usize) };
    let samples = match convert_doom_sound(slice) {
        Some(s) => s,
        None => return -1,
    };
    let system = match get_sound_system_mut() {
        Some(s) => s,
        None => return -1,
    };
    let sink = Sink::connect_new(&system.stream_handle.mixer());
    let volume = (vol as f32 / 127.0).clamp(0.0, 1.0);
    sink.set_volume(volume);
    let source = DoomSound::new(samples, DOOM_SAMPLE_RATE);
    sink.append(source);
    system.channels[channel] = Some(sink);
    channel as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_sound_stop(channel: i32) {
    let channel = channel as usize;
    if channel >= NUM_CHANNELS {
        return;
    }
    if let Some(system) = get_sound_system_mut() {
        if let Some(sink) = system.channels[channel].take() {
            sink.stop();
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_sound_is_playing(channel: i32) -> i32 {
    let channel = channel as usize;
    if channel >= NUM_CHANNELS {
        return 0;
    }
    if let Some(system) = get_sound_system_ref() {
        if let Some(sink) = &system.channels[channel] {
            return if sink.empty() { 0 } else { 1 };
        }
    }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_sound_update_params(channel: i32, vol: i32, _sep: i32) {
    let channel = channel as usize;
    if channel >= NUM_CHANNELS {
        return;
    }
    if let Some(system) = get_sound_system_ref() {
        if let Some(sink) = &system.channels[channel] {
            let volume = (vol as f32 / 127.0).clamp(0.0, 1.0);
            sink.set_volume(volume);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_sound_update() {
    if let Some(system) = get_sound_system_mut() {
        for channel in &mut system.channels {
            if let Some(sink) = channel {
                if sink.empty() {
                    *channel = None;
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_music_init() -> i32 {
    1
}
#[unsafe(no_mangle)]
pub extern "C" fn rust_music_shutdown() {}
#[unsafe(no_mangle)]
pub extern "C" fn rust_music_set_volume(_volume: i32) {}
#[unsafe(no_mangle)]
pub extern "C" fn rust_music_pause() {}
#[unsafe(no_mangle)]
pub extern "C" fn rust_music_resume() {}
#[unsafe(no_mangle)]
pub extern "C" fn rust_music_stop() {}
#[unsafe(no_mangle)]
pub extern "C" fn rust_music_is_playing() -> i32 {
    0
}
