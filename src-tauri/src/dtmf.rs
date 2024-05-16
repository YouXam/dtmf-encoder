use hound::{SampleFormat, WavSpec, WavWriter};
use std::f32::consts::PI;
use std::fs::File;
use std::i16;
use std::io::BufWriter;
use tempfile::NamedTempFile;

const DTMF_FREQS: [((f32, f32), char); 16] = [
    ((697.0, 1209.0), '1'), ((697.0, 1336.0), '2'), ((697.0, 1477.0), '3'), ((697.0, 1633.0), 'A'),
    ((770.0, 1209.0), '4'), ((770.0, 1336.0), '5'), ((770.0, 1477.0), '6'), ((770.0, 1633.0), 'B'),
    ((852.0, 1209.0), '7'), ((852.0, 1336.0), '8'), ((852.0, 1477.0), '9'), ((852.0, 1633.0), 'C'),
    ((941.0, 1209.0), '*'), ((941.0, 1336.0), '0'), ((941.0, 1477.0), '#'), ((941.0, 1633.0), 'D'),
];
const SAMPLE_RATE: u32 = 8000;
const AMPLITUDE: f32 = i16::MAX as f32;

fn find_freqs(c: char) -> anyhow::Result<(f32, f32)> {
    DTMF_FREQS.iter().find(|(_, d)| *d == c)
        .ok_or(anyhow::anyhow!("Invalid character")).map(|x| *x)
        .map(|x| x.0)
}

pub fn tone(c: char) -> anyhow::Result<Vec<u8>> {
    let freqs = find_freqs(c)?;
    let length = (SAMPLE_RATE as f32 * 0.2) as u32;
    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let temp_file = NamedTempFile::new()?;
    let mut writer = WavWriter::create(&temp_file, spec)?;
    write_tone(&mut writer, length, freqs)?;
    writer.finalize()?;
    Ok(std::fs::read(&temp_file)?)
}

pub fn save(path: &str, s: &str) -> anyhow::Result<()> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let mut writer = WavWriter::create(path, spec).unwrap();
    for c in s.chars() {
        let freqs = find_freqs(c)?;
        let length = (SAMPLE_RATE as f32 * 0.05) as u32;
        write_tone(&mut writer, length, freqs)?;
        write_silence(&mut writer, length);
    }
    writer.finalize()?;
    Ok(())
}

fn write_tone(writer: &mut WavWriter<BufWriter<File>>, length: u32, tone: (f32, f32)) -> anyhow::Result<()> {
    for t in 0..length {
        let t = t as f32 / SAMPLE_RATE as f32;
        let x_sin = (t * tone.0 * 2.0 * PI).sin();
        let y_sin = (t * tone.1 * 2.0 * PI).sin();
        let sample = (x_sin + y_sin) / 2.0;
        writer.write_sample((sample * AMPLITUDE) as i16)?;
    }
    Ok(())
}

fn write_silence(writer: &mut WavWriter<BufWriter<File>>, length: u32) {
    (0..length).for_each(|_| writer.write_sample(0 as i16).unwrap());
}