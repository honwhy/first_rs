use super::{Engine, SpecTransform};
use crate::pb::*;
use anyhow::Result;
use bytes::Bytes;
use image::ImageOutputFormat;
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
    core::{self, Point_, Rect_, Size, Vector},
    imgcodecs, imgproc, intensity_transform,
    prelude::*,
    xphoto, Result as OpencvResult,
};

pub struct Opencv(Mat);

impl TryFrom<Bytes> for Opencv {
    type Error = anyhow::Error;

    fn try_from(data: Bytes) -> Result<Self, Self::Error> {
        let src = Mat::from_slice::<u8>(data.as_ref())?;
        let dest = imgcodecs::imdecode(&src, imgcodecs::IMREAD_UNCHANGED)?;
        Ok(Self(dest))
    }
}

impl Engine for Opencv {
    fn apply(&mut self, specs: &[Spec]) {
        for spec in specs.iter() {
            match spec.data {
                Some(spec::Data::Crop(ref v)) => self.transform(v),
                Some(spec::Data::Contrast(ref v)) => self.transform(v),
                // Some(spec::Data::Filter(ref v)) => self.transform(v),
                Some(spec::Data::Fliph(ref v)) => self.transform(v),
                Some(spec::Data::Flipv(ref v)) => self.transform(v),
                Some(spec::Data::Resize(ref v)) => self.transform(v),
                // Some(spec::Data::Watermark(ref v)) => self.transform(v),
                Some(spec::Data::Oil(ref v)) => self.transform(v),
                _ => {}
            }
        }
    }

    fn generate(self, format: ImageOutputFormat) -> Vec<u8> {
        let mut buf: Vector<u8> = Vector::new();
        let flags: Vector<i32> = Vector::new();
        let ext = match format {
            ImageOutputFormat::Png => ".png",
            ImageOutputFormat::Jpeg(_v) => ".jpeg",
            ImageOutputFormat::Pnm(_v) => ".pnm",
            ImageOutputFormat::Gif => ".gif",
            ImageOutputFormat::Ico => ".ico",
            ImageOutputFormat::Bmp => ".bmp",
            _ => ".jpg",
        };
        let _ = imgcodecs::imencode(&ext, &self.0, &mut buf, &flags);
        buf.to_vec()
    }
}

impl SpecTransform<&Crop> for Opencv {
    fn transform(&mut self, op: &Crop) {
        let rect = Rect_::from_points(
            Point_::<i32>::new(op.x1 as i32, op.y1 as i32),
            Point_::<i32>::new(op.x2 as i32, op.y2 as i32),
        );
        let dest = Mat::roi(&self.0, rect).unwrap();
        self.0 = dest;
    }
}
impl SpecTransform<&Contrast> for Opencv {
    fn transform(&mut self, _op: &Contrast) {
        let ssize = self.0.size().unwrap();
        let src = self.0.clone();
        let mut dest = Mat::default();
        let _ = intensity_transform::contrast_stretching(
            src,
            &mut dest,
            0,
            0,
            ssize.width,
            ssize.height,
        );
        self.0 = dest;
    }
}

impl SpecTransform<&Fliph> for Opencv {
    fn transform(&mut self, _op: &Fliph) {
        let mut dest = Mat::default();
        let _ = core::flip(&self.0, &mut dest, 1);
        self.0 = dest;
    }
}

impl SpecTransform<&Flipv> for Opencv {
    fn transform(&mut self, _op: &Flipv) {
        let mut dest = Mat::default();
        let _ = core::flip(&self.0, &mut dest, 0);
        self.0 = dest;
    }
}

impl SpecTransform<&Resize> for Opencv {
    fn transform(&mut self, op: &Resize) {
        let dsize = Size::new(0, 0);
        let ssize = self.0.size().unwrap();
        let ws = ssize.width as f64 / op.width as f64;
        let hs = ssize.height as f64 / op.height as f64;
        let mut dest = Mat::default();
        match resize::ResizeType::from_i32(op.rtype).unwrap() {
            resize::ResizeType::Normal | resize::ResizeType::SeamCarve => {
                let _ = imgproc::resize(&self.0, &mut dest, dsize, ws, hs, imgproc::INTER_LINEAR);
            }
        };
        self.0 = dest;
    }
}

impl SpecTransform<&Oil> for Opencv {
    fn transform(&mut self, op: &Oil) {
        let mut dest = Mat::default();
        let _ = xphoto::oil_painting(
            &self.0,
            &mut dest,
            op.intensity as i32,
            op.radius,
            imgproc::ColorConversionCodes::COLOR_BGR2BGRA as i32,
        );
        self.0 = dest;
    }
}
