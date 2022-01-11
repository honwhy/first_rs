use crate::pb::Spec;
use image::ImageOutputFormat;

mod photon;
pub use photon::Photon;

mod my_opencv;
pub use my_opencv::Opencv;

pub trait Engine {
    fn apply(&mut self, specs: &[Spec]);

    fn generate(self, format: ImageOutputFormat) -> Vec<u8>;
}

pub trait SpecTransform<T> {
    fn transform(&mut self, op: T);
}
