
use crate::color::Color;
use crate::fidelity_consts::{SAMPLE_COUNT};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Illumination {
    pub color: Color,
    pub intensity: f32,
}

impl Illumination {
    pub fn new() -> Self {
        Illumination {
            color: Color(0.0, 0.0, 0.0),
            intensity: 0.0,
        }
    }
}

//pub fn integrate<'a,I: Iterator<Item = &'a Illumination>>(samples: I) -> Illumination {
pub fn integrate(samples: &[Illumination;SAMPLE_COUNT]) -> Illumination {
    let mut lum = Illumination::new();

    // HACK: Do a true weighted-average on colors eventually (scale by brightness)
    let mut samples_with_illum = 0.0;

    for index in 0..SAMPLE_COUNT {
        let sample = samples[index];

        //if sample.intensity > 0.001 {
            lum.color.0 += sample.color.0;
            lum.color.1 += sample.color.1;
            lum.color.2 += sample.color.2;

            samples_with_illum += 1.0;
        //}

        lum.intensity += sample.intensity;
    }

    lum.color.0 /= samples_with_illum;
    lum.color.1 /= samples_with_illum;
    lum.color.2 /= samples_with_illum;

    lum.intensity /= SAMPLE_COUNT as f32;

    return lum;
}