use prost::Message;
use prost_reflect::{DynamicMessage, MethodDescriptor, ReflectMessage};
use tonic::{
    codec::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder},
    Status,
};

#[derive(Debug, Clone)]
pub struct DynamicCodec(MethodDescriptor);

impl DynamicCodec {
    pub fn new(method_descriptor: MethodDescriptor) -> DynamicCodec {
        DynamicCodec(method_descriptor)
    }
}

impl Codec for DynamicCodec {
    type Encode = DynamicMessage;
    type Decode = DynamicMessage;

    type Encoder = DynamicCodec;
    type Decoder = DynamicCodec;

    fn encoder(&mut self) -> Self::Encoder {
        self.clone()
    }

    fn decoder(&mut self) -> Self::Decoder {
        self.clone()
    }
}

impl Encoder for DynamicCodec {
    type Item = DynamicMessage;
    type Error = Status;

    fn encode(&mut self, message: Self::Item, buf: &mut EncodeBuf) -> Result<(), Self::Error> {
        debug_assert_eq!(message.descriptor(), self.0.input());
        message
            .encode(buf)
            .map_err(|err| Status::internal(err.to_string()))?;
        Ok(())
    }
}

impl Decoder for DynamicCodec {
    type Item = DynamicMessage;
    type Error = Status;

    fn decode(&mut self, buf: &mut DecodeBuf) -> Result<Option<Self::Item>, Self::Error> {
        let mut message = DynamicMessage::new(self.0.output());
        message
            .merge(buf)
            .map_err(|err| Status::internal(err.to_string()))?;
        Ok(Some(message))
    }
}
