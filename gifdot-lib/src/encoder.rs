use gdnative::prelude::*;
use gif::{Frame, Encoder, Repeat};

struct FrameData (Vec<u8>, u16);

#[derive(NativeClass)]
#[inherit(Reference)]
#[no_constructor]
pub struct GifdotEncoder {
    width: u16,
    height: u16,
    frame_data: Vec<FrameData>
}

#[methods]
impl GifdotEncoder {
    pub fn new(width: u16, height: u16) -> GifdotEncoder {
        GifdotEncoder { width, height, frame_data: vec![] }
    }

    #[export]
    fn add_frame(&mut self, _owner: &Reference, image_data: PoolArray<u8>, delay: u16) {
        self.frame_data.push(FrameData(image_data.to_vec(), delay))
    }

    #[export]
    fn get_file_data(&mut self, _owner: &Reference) -> PoolArray<u8> {
        let mut buffer = Vec::new();
        let mut encoder = Encoder::new(&mut buffer, self.width, self.height, &[]).unwrap();
        encoder.set_repeat(Repeat::Infinite).unwrap();
        for data in &mut self.frame_data {
            let mut frame = Frame::from_rgba(self.width, self.height, &mut data.0);
            frame.delay = data.1;
            encoder.write_frame(&frame).expect("Error while writing frame")
        }drop(encoder);
        PoolArray::from_vec(buffer)
    }
}