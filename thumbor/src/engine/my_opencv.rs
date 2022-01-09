use super::{Engine, SpecTransform};
use crate::pb::*;
//use anyhow::Result;
use bytes::Bytes;
use image::{DynamicImage, ImageBuffer, ImageOutputFormat};
use lazy_static::lazy_static;
use std::convert::TryFrom;

// lazy_static! {
//     static ref WATERMARK: PhotonImage = {
//         let data = include_bytes!("../../rust-logo.png");
//         let watermark = open_image_from_bytes(data).unwrap();
//         transform::resize(&watermark, 64, 64, transform::SamplingFilter::Nearest)
//     };
// }
use opencv::{
	core::{self, UMat, UMatUsageFlags, Vector,Stream},
	imgproc,
	prelude::*,
	Result,
	types,
    imgcodecs,
};

pub struct Opencv(Mat);

impl TryFrom<Bytes> for Opencv {
    type Error = anyhow::Error;

    fn try_from(data: Bytes) -> Result<Self, Self::Error> {
        let src = Mat::from_slice::<u8>(data.as_ref())?;
		let dest = imgcodecs::imdecode(&src, imgcodecs::IMREAD_COLOR)?;
        Ok(Self(dest))
    }
    
}

impl Engine for Opencv {
    fn apply(&mut self, specs: &[Spec]) {
        for spec in specs.iter() {
            match spec.data {
                // Some(spec::Data::Crop(ref v)) => self.transform(v),
                // Some(spec::Data::Contrast(ref v)) => self.transform(v),
                // Some(spec::Data::Filter(ref v)) => self.transform(v),
                Some(spec::Data::Fliph(ref v)) => self.transform(v),
                // Some(spec::Data::Flipv(ref v)) => self.transform(v),
                // Some(spec::Data::Resize(ref v)) => self.transform(v),
                // Some(spec::Data::Watermark(ref v)) => self.transform(v),
                // Some(spec::Data::Oil(ref v)) => self.transform(v),
                _ => {}
            }
        }
    }

    fn generate(self, format: ImageOutputFormat) -> Vec<u8> {
        let mut buf: Vector<u8> = Vector::new();
        let mut flags: Vector<i32> = Vector::new();
        flags.push(imgcodecs::ImwriteFlags::IMWRITE_JPEG_QUALITY as i32);
        imgcodecs::imencode("jpeg", &self.0, &mut buf, &flags);
        println!("after opencv imencode");
        buf.to_vec()
    }
    
}

impl SpecTransform<&Fliph> for Opencv {
    fn transform(&mut self, _op: &Fliph) {
        let mut dest = Mat::default();
        println!("Transform opencv fliph");
        core::flip(&self.0, &mut dest, 1);
        self.0 = dest;
    }
}

