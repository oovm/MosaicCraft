mod sample;
mod shape;
use crate::MosaicRenderer;


impl MosaicRenderer {
    // https://mathematica.stackexchange.com/questions/106165/reproduce-image-effect-in-mathematica
    pub fn render(&self) {
        if self.grayscale {
            unimplemented!()
        }

        match self.background {
            None => (),
            Some(_) => unimplemented!(),
        }

        if self.smooth {
            unimplemented!()
        }
        else {
            unimplemented!()
        }

        unimplemented!()
    }
}
