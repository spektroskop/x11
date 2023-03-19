use crate::display;
use crate::proto;
use byteorder::{BigEndian, WriteBytesExt};
use std::io;

#[derive(Debug)]
pub struct CreateWindow {
    depth: u8,
    window: u32,
    parent: u32,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    border_width: u16,
    class: proto::Class,
    visual: proto::Visual,
    value_mask: u32,
    values: Vec<u32>,
}

impl CreateWindow {
    pub fn new(
        depth: u8,
        window: u32,
        parent: u32,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        border_width: u16,
        class: proto::Class,
        visual: proto::Visual,
    ) -> Self {
        Self {
            depth,
            window,
            parent,
            x,
            y,
            width,
            height,
            border_width,
            class,
            visual,
            value_mask: 0,
            values: Vec::new(),
        }
    }

    pub fn write<W: io::Write>(&self, w: &mut W) -> anyhow::Result<()> {
        w.write_u8(proto::OPCODE_CREATE_WINDOW)?;

        w.write_u8(self.depth)?; // depth

        w.write_u16::<BigEndian>((8 + self.values.len()).try_into()?)?; // request length (8+n)

        w.write_u32::<BigEndian>(self.window)?;
        w.write_u32::<BigEndian>(self.parent)?;

        w.write_u16::<BigEndian>(self.x)?;
        w.write_u16::<BigEndian>(self.y)?;
        w.write_u16::<BigEndian>(self.width)?;
        w.write_u16::<BigEndian>(self.height)?;
        w.write_u16::<BigEndian>(self.border_width)?;

        self.class.write(w)?;
        self.visual.write(w)?;

        w.write_u32::<BigEndian>(self.value_mask)?;

        // TODO: order?
        for v in &self.values {
            w.write_u32::<BigEndian>(*v)?; // n
        }

        Ok(())
    }

    pub fn event_mask(&mut self, value: u32) -> &mut Self {
        self.value_mask |= proto::VALUE_MASK_EVENT_MASK;
        self.values.push(value);
        self
    }
}

#[derive(Debug)]
pub struct ChangeWindowAttributes {
    window: u32,
    value_mask: u32,
    values: Vec<u32>,
}

impl ChangeWindowAttributes {
    pub fn new(window: u32) -> Self {
        Self {
            window,
            value_mask: 0,
            values: Vec::new(),
        }
    }

    pub fn write<W: io::Write>(&self, w: &mut W) -> anyhow::Result<()> {
        w.write_u8(proto::OPCODE_CHANGE_WINDOW_ATTRIBUTES)?;
        w.write_u8(0)?; // unused
        w.write_u16::<BigEndian>((3 + self.values.len()).try_into()?)?; // request length (3+n)

        w.write_u32::<BigEndian>(self.window)?;

        w.write_u32::<BigEndian>(self.value_mask)?;

        for v in &self.values {
            w.write_u32::<BigEndian>(*v)?; // n
        }

        Ok(())
    }

    pub fn event_mask(&mut self, value: u32) -> &mut Self {
        self.value_mask |= proto::VALUE_MASK_EVENT_MASK;
        self.values.push(value);
        self
    }
}

#[derive(Debug)]
pub struct GetWindowAttributes {
    window: u32,
}

impl GetWindowAttributes {
    pub fn new(window: u32) -> Self {
        Self { window }
    }

    pub fn write<W: io::Write>(&self, w: &mut W) -> anyhow::Result<()> {
        w.write_u8(proto::OPCODE_GET_WINDOW_ATTRIBUTES)?;
        w.write_u8(0)?; // unused
        w.write_u16::<BigEndian>(2)?; // request length

        w.write_u32::<BigEndian>(self.window)?; // window

        Ok(())
    }
}

#[derive(Debug)]
pub struct MapWindow {
    window: u32,
}

impl MapWindow {
    pub fn new(window: u32) -> Self {
        Self { window }
    }

    pub fn write<W: io::Write>(&self, w: &mut W) -> anyhow::Result<()> {
        w.write_u8(proto::OPCODE_MAP_WINDOW)?;
        w.write_u8(0)?; // unused
        w.write_u16::<BigEndian>(2)?; // request length

        w.write_u32::<BigEndian>(self.window)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct QueryTree {
    window: u32,
}

#[derive(Debug)]
pub struct QueryTreeReply {
    root: proto::Window,
    parent: Option<proto::Window>,
    children: Vec<proto::Window>,
}

impl QueryTree {
    pub fn new(window: u32) -> Self {
        Self { window }
    }

    pub fn write<W: io::Write>(&self, w: &mut W) -> anyhow::Result<()> {
        w.write_u8(proto::OPCODE_QUERY_TREE)?;
        w.write_u8(0)?; // unused
        w.write_u16::<BigEndian>(2)?; // request length

        w.write_u32::<BigEndian>(self.window)?;

        Ok(())
    }
}
