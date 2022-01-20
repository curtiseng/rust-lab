use bytes;
use bytes::BytesMut;
use tokio_util::codec:: {Decoder, Encoder};

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub struct Frame {
    meta : Vec<String>,
    data : String,
}

impl Decoder for Frame {
    type Item = Frame;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        todo!()
    }
}

impl Encoder<String> for Frame {
    type Error = ();

    fn encode(&mut self, item: String, dst: &mut BytesMut) -> Result<(), Self::Error> {
        todo!()
    }
}