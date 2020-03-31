use std::net::Shutdown;

use byteorder::{ByteOrder, LittleEndian};

use crate::io::{Buf, BufMut, BufStream, MaybeTlsStream};
use crate::mssql::protocol::{Encode, PacketHeader, Prelogin};
use crate::mssql::MsSql;
use crate::mssql::MsSqlError;
use crate::url::Url;

// Size before a packet is split
const MAX_PACKET_SIZE: u32 = 1024;

pub(crate) struct MsSqlStream {
    pub(super) stream: BufStream<MaybeTlsStream>,

    // Is the stream ready to send commands
    // Put another way, are we still expecting an EOF or OK packet to terminate
    pub(super) is_ready: bool,

    pub(super) packet: Vec<u8>,
}

impl MsSqlStream {
    pub(super) async fn new(url: &Url) -> crate::Result<Self> {
        let stream = MaybeTlsStream::connect(&url, 1433).await?;

        Ok(Self {
            stream: BufStream::new(stream),
            is_ready: true,
            packet: Vec::new(),
        })
    }

    pub(super) async fn send<T>(&mut self, packet: T, initial: bool) -> crate::Result<()>
    where
        T: Encode + std::fmt::Debug,
    {
        self.write(packet);
        self.flush().await
    }

    pub(super) async fn flush(&mut self) -> crate::Result<()> {
        Ok(self.stream.flush().await?)
    }

    pub(super) fn write<T>(&mut self, packet: T)
    where
        T: Encode,
    {
        let buf = self.stream.buffer_mut();
        packet.encode(buf);
    }

    pub(super) async fn receive(&mut self) -> crate::Result<&Vec<u8>> {
        self.read().await?;

        Ok(&self.packet)
    }

    pub(super) async fn read(&mut self) -> crate::Result<()> {
        let header = self.stream.peek(8_usize).await?;
        dbg!(&header);

        let header = PacketHeader::read(header.clone())?;
        dbg!(&header);

        let length = header.length;

        self.stream.consume(8);

        let payload = self.stream.peek(18).await?;
        dbg!(payload);

        self.packet = payload.to_vec();

        self.stream.consume(length as usize);

        Ok(())
    }
}
