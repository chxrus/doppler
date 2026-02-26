pub fn downmix_to_mono(samples: &[f32], channels: u16) -> Vec<f32> {
    if channels <= 1 {
        return samples.to_vec();
    }

    let channel_count = usize::from(channels);
    let frame_count = samples.len() / channel_count;
    let mut mono_samples = Vec::with_capacity(frame_count);

    for frame in samples.chunks_exact(channel_count) {
        let sum: f32 = frame.iter().copied().sum();
        mono_samples.push(sum / channels as f32);
    }

    mono_samples
}

pub fn resample_to_16k(mono_samples: &[f32], source_sample_rate: u32) -> Vec<f32> {
    const TARGET_SAMPLE_RATE: u32 = 16_000;

    if source_sample_rate == 0 {
        return Vec::new();
    }

    if source_sample_rate == TARGET_SAMPLE_RATE {
        return mono_samples.to_vec();
    }

    if mono_samples.len() < 2 {
        return mono_samples.to_vec();
    }

    let ratio = source_sample_rate as f64 / TARGET_SAMPLE_RATE as f64;
    let output_len = ((mono_samples.len() as f64) / ratio).round() as usize;
    let mut output = Vec::with_capacity(output_len.max(1));

    for index in 0..output_len {
        let source_position = index as f64 * ratio;
        let left_index = source_position.floor() as usize;
        let right_index = (left_index + 1).min(mono_samples.len() - 1);
        let fraction = (source_position - left_index as f64) as f32;

        let left_sample = mono_samples[left_index];
        let right_sample = mono_samples[right_index];
        output.push(left_sample + (right_sample - left_sample) * fraction);
    }

    output
}
