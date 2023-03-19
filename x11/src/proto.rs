use std::io;

pub type Window = u32;

pub const RESPONSE_ERROR: u8 = 0;
pub const RESPONSE_REPLY: u8 = 1;

enum EventMask {
    KeyPress,
    KeyRelease,
    ButtonPress,
    ButtonRelease,
    EnterWindow,
    LeaveWindow,
    PointerMotion,
    PointerMotionHint,
    Button1Motion,
    Button2Motion,
    Button3Motion,
    Button4Motion,
    Button5Motion,
    ButtonMotion,
    KeymapState,
    Xposure,
    VisibilityChange,
    StructureNotify,
    ResizeRedirect,
    SubstructureNotify,
    SubstructureRedirect,
    FocusChange,
    PropertyChange,
    ColormapChange,
    OwnerGrabButton,
}

impl EventMask {
    fn write<W: io::Write>(&self, w: &mut W) -> anyhow::Result<()> {
        use byteorder::{BigEndian, WriteBytesExt};

        let value = match self {
            EventMask::KeyPress => 0x00000001,
            EventMask::KeyRelease => 0x00000002,
            EventMask::ButtonPress => 0x00000004,
            EventMask::ButtonRelease => 0x00000008,
            EventMask::EnterWindow => 0x00000010,
            EventMask::LeaveWindow => 0x00000020,
            EventMask::PointerMotion => 0x00000040,
            EventMask::PointerMotionHint => 0x00000080,
            EventMask::Button1Motion => 0x00000100,
            EventMask::Button2Motion => 0x00000200,
            EventMask::Button3Motion => 0x00000400,
            EventMask::Button4Motion => 0x00000800,
            EventMask::Button5Motion => 0x00001000,
            EventMask::ButtonMotion => 0x00002000,
            EventMask::KeymapState => 0x00004000,
            EventMask::Xposure => 0x00008000,
            EventMask::VisibilityChange => 0x00010000,
            EventMask::StructureNotify => 0x00020000,
            EventMask::ResizeRedirect => 0x00040000,
            EventMask::SubstructureNotify => 0x00080000,
            EventMask::SubstructureRedirect => 0x00100000,
            EventMask::FocusChange => 0x00200000,
            EventMask::PropertyChange => 0x00400000,
            EventMask::ColormapChange => 0x00800000,
            EventMask::OwnerGrabButton => 0x01000000,
        };

        w.write_u32::<BigEndian>(value).map_err(|err| err.into())
    }
}

pub const EVENT_MASK_KEY_PRESS: u32 = 0x00000001;
pub const EVENT_MASK_KEY_RELEASE: u32 = 0x00000002;
pub const EVENT_MASK_BUTTON_PRESS: u32 = 0x00000004;
pub const EVENT_MASK_BUTTON_RELEASE: u32 = 0x00000008;
pub const EVENT_MASK_ENTER_WINDOW: u32 = 0x00000010;
pub const EVENT_MASK_LEAVE_WINDOW: u32 = 0x00000020;
pub const EVENT_MASK_POINTER_MOTION: u32 = 0x00000040;
pub const EVENT_MASK_POINTER_MOTION_HINT: u32 = 0x00000080;
pub const EVENT_MASK_BUTTON1_MOTION: u32 = 0x00000100;
pub const EVENT_MASK_BUTTON2_MOTION: u32 = 0x00000200;
pub const EVENT_MASK_BUTTON3_MOTION: u32 = 0x00000400;
pub const EVENT_MASK_BUTTON4_MOTION: u32 = 0x00000800;
pub const EVENT_MASK_BUTTON5_MOTION: u32 = 0x00001000;
pub const EVENT_MASK_BUTTON_MOTION: u32 = 0x00002000;
pub const EVENT_MASK_KEYMAP_STATE: u32 = 0x00004000;
pub const EVENT_MASK_EXPOSURE: u32 = 0x00008000;
pub const EVENT_MASK_VISIBILITY_CHANGE: u32 = 0x00010000;
pub const EVENT_MASK_STRUCTURE_NOTIFY: u32 = 0x00020000;
pub const EVENT_MASK_RESIZE_REDIRECT: u32 = 0x00040000;
pub const EVENT_MASK_SUBSTRUCTURE_NOTIFY: u32 = 0x00080000;
pub const EVENT_MASK_SUBSTRUCTURE_REDIRECT: u32 = 0x00100000;
pub const EVENT_MASK_FOCUS_CHANGE: u32 = 0x00200000;
pub const EVENT_MASK_PROPERTY_CHANGE: u32 = 0x00400000;
pub const EVENT_MASK_COLORMAP_CHANGE: u32 = 0x00800000;
pub const EVENT_MASK_OWNER_GRAB_BUTTON: u32 = 0x01000000;

pub const VALUE_MASK_BACKGROUND_PIXMAP: u32 = 0x00000001;
pub const VALUE_MASK_BACKGROUND_PIXEL: u32 = 0x00000002;
pub const VALUE_MASK_BORDER_PIXMAP: u32 = 0x00000004;
pub const VALUE_MASK_BORDER_PIXEL: u32 = 0x00000008;
pub const VALUE_MASK_BIT_GRAVITY: u32 = 0x00000010;
pub const VALUE_MASK_WIN_GRAVITY: u32 = 0x00000020;
pub const VALUE_MASK_BACKING_STORE: u32 = 0x00000040;
pub const VALUE_MASK_BACKING_PLANES: u32 = 0x00000080;
pub const VALUE_MASK_BACKING_PIXEL: u32 = 0x00000100;
pub const VALUE_MASK_OVERRIDE_REDIRECT: u32 = 0x00000200;
pub const VALUE_MASK_SAVE_UNDER: u32 = 0x00000400;
pub const VALUE_MASK_EVENT_MASK: u32 = 0x00000800;
pub const VALUE_MASK_DO_NOT_PROPAGATE_MASK: u32 = 0x00001000;
pub const VALUE_MASK_COLORMAP: u32 = 0x00002000;
pub const VALUE_MASK_CURSOR: u32 = 0x00004000;

pub const OPCODE_CREATE_WINDOW: u8 = 1;
pub const OPCODE_CHANGE_WINDOW_ATTRIBUTES: u8 = 2;
pub const OPCODE_GET_WINDOW_ATTRIBUTES: u8 = 3;
pub const OPCODE_MAP_WINDOW: u8 = 8;
pub const OPCODE_QUERY_TREE: u8 = 15;

#[derive(Debug)]
pub enum Class {
    CopyFromParent,
    InputOutput,
    InputOnly,
}

impl Class {
    pub fn write<W: io::Write>(&self, w: &mut W) -> anyhow::Result<()> {
        use byteorder::{BigEndian, WriteBytesExt};

        match self {
            Class::CopyFromParent => w.write_u16::<BigEndian>(0),
            Class::InputOutput => w.write_u16::<BigEndian>(1),
            Class::InputOnly => w.write_u16::<BigEndian>(2),
        }
        .map_err(|err| err.into())
    }
}

#[derive(Debug)]
pub enum Visual {
    Id(u32),
    CopyFromParent,
}

impl Visual {
    pub fn write<W: io::Write>(&self, w: &mut W) -> anyhow::Result<()> {
        use byteorder::{BigEndian, WriteBytesExt};

        match self {
            Visual::Id(id) => w.write_u32::<BigEndian>(*id),
            Visual::CopyFromParent => w.write_u32::<BigEndian>(0),
        }
        .map_err(|err| err.into())
    }
}

#[derive(Debug)]
pub enum StackMode {
    Above,
    Below,
    TopIf,
    BottomIf,
    Opposite,
}

#[derive(Debug)]
pub enum BackgroundPixmap {
    Pixmap(u32),
    None,
    ParentRelative,
}

#[derive(Debug)]
pub enum BorderPixmap {
    Pixmap(u32),
    CopyFromParent,
}

#[derive(Debug)]
pub enum BitGravity {
    Forget = 0,
    NorthWest,
    North,
    NorthEast,
    West,
    Center,
    East,
    SouthWest,
    South,
    SouthEast,
    Static,
}

impl TryFrom<u8> for BitGravity {
    type Error = anyhow::Error;

    fn try_from(v: u8) -> anyhow::Result<Self> {
        match v {
            0 => Ok(BitGravity::Forget),
            1 => Ok(BitGravity::NorthWest),
            2 => Ok(BitGravity::North),
            3 => Ok(BitGravity::NorthEast),
            4 => Ok(BitGravity::West),
            5 => Ok(BitGravity::Center),
            6 => Ok(BitGravity::East),
            7 => Ok(BitGravity::SouthWest),
            8 => Ok(BitGravity::South),
            9 => Ok(BitGravity::SouthEast),
            10 => Ok(BitGravity::Static),

            _ => anyhow::bail!("bad BitGravity"),
        }
    }
}

impl Into<u8> for BitGravity {
    fn into(self) -> u8 {
        match self {
            BitGravity::Forget => 0,
            BitGravity::NorthWest => 1,
            BitGravity::North => 2,
            BitGravity::NorthEast => 3,
            BitGravity::West => 4,
            BitGravity::Center => 5,
            BitGravity::East => 6,
            BitGravity::SouthWest => 7,
            BitGravity::South => 8,
            BitGravity::SouthEast => 9,
            BitGravity::Static => 10,
        }
    }
}

#[derive(Debug)]
pub enum WinGravity {
    Unmap,
    Static,
    NorthWest,
    North,
    NorthEast,
    West,
    Center,
    East,
    SouthWest,
    South,
    SouthEast,
}

impl TryFrom<u8> for WinGravity {
    type Error = anyhow::Error;

    fn try_from(v: u8) -> anyhow::Result<Self> {
        match v {
            0 => Ok(WinGravity::Unmap),
            1 => Ok(WinGravity::NorthWest),
            2 => Ok(WinGravity::North),
            3 => Ok(WinGravity::NorthEast),
            4 => Ok(WinGravity::West),
            5 => Ok(WinGravity::Center),
            6 => Ok(WinGravity::East),
            7 => Ok(WinGravity::SouthWest),
            8 => Ok(WinGravity::South),
            9 => Ok(WinGravity::SouthEast),
            10 => Ok(WinGravity::Static),

            _ => anyhow::bail!("bad WinGravity"),
        }
    }
}

impl Into<u8> for WinGravity {
    fn into(self) -> u8 {
        match self {
            WinGravity::Unmap => 0,
            WinGravity::NorthWest => 1,
            WinGravity::North => 2,
            WinGravity::NorthEast => 3,
            WinGravity::West => 4,
            WinGravity::Center => 5,
            WinGravity::East => 6,
            WinGravity::SouthWest => 7,
            WinGravity::South => 8,
            WinGravity::SouthEast => 9,
            WinGravity::Static => 10,
        }
    }
}

#[derive(Debug)]
pub enum BackingStore {
    NotUseful,
    WhenMapped,
    Always,
}

#[derive(Debug)]
pub enum Colormap {
    Colormap(u32),
    CopyFromParent,
}

#[derive(Debug)]
pub enum WindowAttribute {
    BackgroundPixmap(BackgroundPixmap),
    BackgroundPixel(u32),
    BorderPixmap(BorderPixmap),
    BorderPixel(u32),
    BitGravity(BitGravity),
    WinGravity(WinGravity),
    BackingStore(BackingStore),
    BackingPlanes(u32),
    BackingPixel(u32),
    SaveUnder(bool),
    // EventMask  SETofEVENT,
    // DoNotPropagateMask   SETofDEVICEEVENT,
    OverrideRedirect(bool),
    Colormap(Colormap),
    Cursor(Option<u32>),
}
