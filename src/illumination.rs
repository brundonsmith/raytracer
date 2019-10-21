
use crate::color::Color;

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

pub fn integrate<'a,I: Iterator<Item = &'a Illumination>>(samples: I) -> Illumination {
    let mut count = 0;
    let mut lum = Illumination::new();

    // HACK: Do a true weighted-average eventually
    let mut samples_with_illum = 0.0;

    for &sample in samples {
        if sample.intensity > 0.001 {
            samples_with_illum += 1.0;

        lum.color.0 += sample.color.0;
        lum.color.1 += sample.color.1;
        lum.color.2 += sample.color.2;
        }

        lum.intensity += sample.intensity;

        count += 1;
    }


    let float_count = count as f32;

    lum.color.0 /= samples_with_illum;
    lum.color.1 /= samples_with_illum;
    lum.color.2 /= samples_with_illum;

    lum.intensity /= float_count;

    return lum;
}