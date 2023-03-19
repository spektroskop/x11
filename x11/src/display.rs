use crate::{error, event, proto, request, setup};
use std::env;
use std::io::{self, Read, Seek};
use std::os::unix::net::UnixStream;
use std::time::Duration;
use tokio::task;

#[derive(Debug)]
pub enum Message {
    Error(error::Error),
    Reply(u16),
    Event(event::Event),
}

pub struct Display {
    conn: UnixStream,
    pub setup: setup::Setup,
}

impl io::Read for Display {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.conn.read(buf)
    }
}

impl io::Write for Display {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.conn.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.conn.flush()
    }
}

impl Display {
    pub fn open() -> anyhow::Result<Self> {
        let addr = env::var("DISPLAY")?;
        let mut conn = UnixStream::connect(&addr)?;

        {
            let timeout = Duration::from_secs(5);
            conn.set_write_timeout(Some(timeout))?;
            conn.set_read_timeout(Some(timeout))?;
        }

        let setup = setup::Setup::new(&mut conn)?;
        println!("{:?}", setup);

        Ok(Self { conn, setup })
    }

    pub fn screen(&self) -> Option<&setup::Screen> {
        self.setup.screens.first()
    }

    pub fn root(&self) -> Option<proto::Window> {
        self.screen().map(|screen| screen.root_window)
    }

    pub fn next_message(&mut self) -> anyhow::Result<Message> {
        self.conn.set_read_timeout(None)?;

        let mut buf: [u8; 32] = [0; 32];
        self.conn.read_exact(&mut buf)?;

        Ok(match buf[0] {
            proto::RESPONSE_ERROR => {
                let mut cursor = io::Cursor::new(&buf[1..]);
                let error = error::Error::read(&mut cursor)?;
                Message::Error(error)
            }

            proto::RESPONSE_REPLY => {
                use byteorder::{BigEndian, ReadBytesExt};

                let mut cursor = io::Cursor::new(&buf[1..]);
                let _data = cursor.read_u8()?;
                let seq = cursor.read_u16::<BigEndian>()?;
                cursor.seek(io::SeekFrom::Start(0))?;
                Message::Reply(seq)
            }

            _code => {
                let mut cursor = io::Cursor::new(&buf);
                let event = event::Event::read(&mut cursor)?;
                Message::Event(event)
            }
        })
    }

    pub async fn query_tree(
        &mut self,
        window: proto::Window,
    ) -> anyhow::Result<Vec<proto::Window>> {
        request::QueryTree::new(window).write(self);
        anyhow::bail!("nop");
    }
}
