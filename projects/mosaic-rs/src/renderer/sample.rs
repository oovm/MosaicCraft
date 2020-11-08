use super::*;
use crate::ColorSampler;

impl Mosaic {
    pub fn sample(&self) {
        match self.sampler {
            ColorSampler::CenterPixel => self.sample_center_pixel(),
            ColorSampler::Average => self.sample_global_average(),
            ColorSampler::Dominant => self.sample_dominant_color(),
            ColorSampler::Vibrant => self.sample_vibrant_color(),
        }
    }

    fn sample_center_pixel(&self) {
        unimplemented!()
    }

    fn sample_global_average(&self) {
        unimplemented!()
    }

    fn sample_dominant_color(&self) {
        unimplemented!()
    }

    fn sample_vibrant_color(&self) {
        unimplemented!()
    }
}
