use anyhow::anyhow;
use std::fmt;
use std::io;
use std::str;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Failed(String),
    #[error("authentication was required")]
    AuthenticationRequired,
}

#[derive(Debug)]
enum Status {
    Failed,
    Success,
    Authenticate,
}

impl TryFrom<u8> for Status {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Status::Failed),
            1 => Ok(Status::Success),
            2 => Ok(Status::Authenticate),

            _ => Err(anyhow!("bad Status: {}", value)),
        }
    }
}

pub struct Setup {
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub length: u16,
    pub release_number: u32,
    pub resource_id_base: u32,
    pub resource_id_mask: u32,
    pub motion_buffer_size: u32,
    pub vendor_length: u16,
    pub maximum_request_length: u16,
    pub screen_count: u8,
    pub pixmap_format_count: u8,
    pub image_byte_order: u8,
    pub bitmap_format_bit_order: u8,
    pub bitmap_format_scanline_unit: u8,
    pub bitmap_format_scanline_pad: u8,
    pub min_keycode: u8,
    pub max_keycode: u8,
    pub vendor: String,
    pub pixmap_formats: Vec<PixmapFormat>,
    pub screens: Vec<Screen>,
}

impl fmt::Debug for Setup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Setup")
            .field("major_version", &self.protocol_major_version)
            .field("minor_version", &self.protocol_minor_version)
            .field("vendor", &self.vendor)
            .field("screens", &self.screens.len())
            .finish()
    }
}

impl Setup {
    pub fn new<T: io::Read + io::Write>(stream: &mut T) -> anyhow::Result<Self> {
        Self::write(stream)?;
        Self::read(stream)
    }

    pub fn read_status<T: io::Read>(r: &mut T) -> anyhow::Result<()> {
        use byteorder::ReadBytesExt;

        match r.read_u8()?.try_into()? {
            Status::Success => Ok(()),

            Status::Failed => {
                let length = r.read_u8()?;

                for _ in 0..6 {
                    r.read_u8()?;
                }

                let reason = {
                    let length = usize::try_from(length)?;

                    let mut data = vec![0; usize::try_from(length)?];
                    r.read_exact(&mut data)?;

                    String::from(str::from_utf8(&data)?)
                };

                Err(Error::Failed(reason).into())
            }

            Status::Authenticate => Err(Error::AuthenticationRequired.into()),
        }
    }

    fn write<W: io::Write>(w: &mut W) -> anyhow::Result<()> {
        use byteorder::{BigEndian, WriteBytesExt};

        w.write_u8(0x42)?; // MSB first

        w.write_u8(0)?; // unused

        w.write_u16::<BigEndian>(11)?; // protocol-major-version
        w.write_u16::<BigEndian>(0)?; // protocol-minor-version

        w.write_u16::<BigEndian>(0)?; // length of authorization-protocol-name
        w.write_u16::<BigEndian>(0)?; // length of authorization-protocol-data

        w.write_u16::<BigEndian>(0)?; // unused

        w.flush().map_err(|e| e.into())
    }

    fn read<T: io::Read>(r: &mut T) -> anyhow::Result<Self> {
        use byteorder::{BigEndian, ReadBytesExt};

        Self::read_status(r)?;
        r.read_u8()?; // unused

        let protocol_major_version = r.read_u16::<BigEndian>()?;
        let protocol_minor_version = r.read_u16::<BigEndian>()?;
        let length = r.read_u16::<BigEndian>()?;
        let release_number = r.read_u32::<BigEndian>()?;
        let resource_id_base = r.read_u32::<BigEndian>()?;
        let resource_id_mask = r.read_u32::<BigEndian>()?;
        let motion_buffer_size = r.read_u32::<BigEndian>()?;
        let vendor_length = r.read_u16::<BigEndian>()?;
        let maximum_request_length = r.read_u16::<BigEndian>()?;
        let screen_count = r.read_u8()?;
        let pixmap_format_count = r.read_u8()?;
        let image_byte_order = r.read_u8()?;
        let bitmap_format_bit_order = r.read_u8()?;
        let bitmap_format_scanline_unit = r.read_u8()?;
        let bitmap_format_scanline_pad = r.read_u8()?;
        let min_keycode = r.read_u8()?;
        let max_keycode = r.read_u8()?;

        // unused
        for _ in 0..4 {
            r.read_u8()?;
        }

        let vendor = {
            let length = usize::try_from(vendor_length)?;

            let mut data = vec![0; length];
            r.read_exact(&mut data)?;

            // padding
            for _ in 0..(4 - (length % 4)) % 4 {
                r.read_u8()?;
            }

            String::from(str::from_utf8(&data)?)
        };

        let pixmap_formats = (0..pixmap_format_count)
            .map(|_| PixmapFormat::read(r))
            .collect::<Result<Vec<_>, anyhow::Error>>()?;

        let screens = (0..screen_count)
            .map(|_| Screen::read(r))
            .collect::<Result<Vec<_>, anyhow::Error>>()?;

        Ok(Setup {
            protocol_major_version,
            protocol_minor_version,
            length,
            release_number,
            resource_id_base,
            resource_id_mask,
            motion_buffer_size,
            vendor_length,
            maximum_request_length,
            screen_count,
            pixmap_format_count,
            image_byte_order,
            bitmap_format_bit_order,
            bitmap_format_scanline_unit,
            bitmap_format_scanline_pad,
            min_keycode,
            max_keycode,
            vendor,
            pixmap_formats,
            screens,
        })
    }
}

#[derive(Debug)]
pub struct PixmapFormat {
    pub depth: u8,
    pub bits_per_pixel: u8,
    pub scanline_pad: u8,
}

impl PixmapFormat {
    pub fn read<T: io::Read>(r: &mut T) -> anyhow::Result<Self> {
        use byteorder::ReadBytesExt;

        let depth = r.read_u8()?;
        let bits_per_pixel = r.read_u8()?;
        let scanline_pad = r.read_u8()?;

        // unused
        for _ in 0..5 {
            r.read_u8()?;
        }

        Ok(PixmapFormat {
            depth,
            bits_per_pixel,
            scanline_pad,
        })
    }
}

#[derive(Debug)]
pub enum BackingStores {
    Never,
    WhenMapped,
    Always,
}

impl TryFrom<u8> for BackingStores {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BackingStores::Never),
            1 => Ok(BackingStores::WhenMapped),
            2 => Ok(BackingStores::Always),
            _ => Err(anyhow!("bad BackingStores: {}", value)),
        }
    }
}

#[derive(Debug)]
pub struct Screen {
    pub root_window: u32,
    pub colormap: u32,
    pub white_pixel: u32,
    pub black_pixel: u32,
    pub current_input_masks: u32,
    pub width_in_pixels: u16,
    pub height_in_pixels: u16,
    pub width_in_millimeters: u16,
    pub height_in_millimeters: u16,
    pub min_installed_maps: u16,
    pub max_installed_maps: u16,
    pub root_visual: u32,
    pub backing_stores: BackingStores,
    pub save_unders: u8,
    pub root_depth: u8,
    pub allowed_depth_count: u8,
    pub depths: Vec<Depth>,
}

impl Screen {
    pub fn read<T: io::Read>(r: &mut T) -> anyhow::Result<Self> {
        use byteorder::{BigEndian, ReadBytesExt};

        let root_window = r.read_u32::<BigEndian>()?;
        let colormap = r.read_u32::<BigEndian>()?;
        let white_pixel = r.read_u32::<BigEndian>()?;
        let black_pixel = r.read_u32::<BigEndian>()?;
        let current_input_masks = r.read_u32::<BigEndian>()?;
        let width_in_pixels = r.read_u16::<BigEndian>()?;
        let height_in_pixels = r.read_u16::<BigEndian>()?;
        let width_in_millimeters = r.read_u16::<BigEndian>()?;
        let height_in_millimeters = r.read_u16::<BigEndian>()?;
        let min_installed_maps = r.read_u16::<BigEndian>()?;
        let max_installed_maps = r.read_u16::<BigEndian>()?;
        let root_visual = r.read_u32::<BigEndian>()?;
        let backing_stores = r.read_u8()?.try_into()?;
        let save_unders = r.read_u8()?;
        let root_depth = r.read_u8()?;
        let allowed_depth_count = r.read_u8()?;

        let depths = (0..allowed_depth_count)
            .map(|_| Depth::read(r))
            .collect::<Result<Vec<_>, anyhow::Error>>()?;

        Ok(Screen {
            root_window,
            colormap,
            white_pixel,
            black_pixel,
            current_input_masks,
            width_in_pixels,
            height_in_pixels,
            width_in_millimeters,
            height_in_millimeters,
            min_installed_maps,
            max_installed_maps,
            root_visual,
            backing_stores,
            save_unders,
            root_depth,
            allowed_depth_count,
            depths,
        })
    }
}

#[derive(Debug)]
pub struct Depth {
    pub depth: u8,
    pub visual_type_count: u16,
    pub visual_types: Vec<VisualType>,
}

impl Depth {
    pub fn read<T: io::Read>(r: &mut T) -> anyhow::Result<Self> {
        use byteorder::{BigEndian, ReadBytesExt};

        let depth = r.read_u8()?;
        r.read_u8()?; // unused
        let visual_type_count = r.read_u16::<BigEndian>()?;

        // unused
        for _ in 0..4 {
            r.read_u8()?;
        }

        let visual_types = (0..visual_type_count)
            .map(|_| VisualType::read(r))
            .collect::<Result<Vec<_>, anyhow::Error>>()?;

        Ok(Depth {
            depth,
            visual_type_count,
            visual_types,
        })
    }
}

#[derive(Debug)]
pub enum VisualTypeClass {
    StaticGray,
    GrayScale,
    StaticColor,
    PseudoColor,
    TrueColor,
    DirectColor,
}

impl TryFrom<u8> for VisualTypeClass {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(VisualTypeClass::StaticGray),
            1 => Ok(VisualTypeClass::GrayScale),
            2 => Ok(VisualTypeClass::StaticColor),
            3 => Ok(VisualTypeClass::PseudoColor),
            4 => Ok(VisualTypeClass::TrueColor),
            5 => Ok(VisualTypeClass::DirectColor),

            _ => Err(anyhow!("bad VisualTypeClass: {}", value)),
        }
    }
}

#[derive(Debug)]
pub struct VisualType {
    pub visual_id: u32,
    pub class: VisualTypeClass,
    pub bits_per_rgb_value: u8,
    pub colormap_entries: u16,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
}

impl VisualType {
    pub fn read<T: io::Read>(r: &mut T) -> anyhow::Result<Self> {
        use byteorder::{BigEndian, ReadBytesExt};

        let visual_id = r.read_u32::<BigEndian>()?;
        let class = r.read_u8()?.try_into()?;
        let bits_per_rgb_value = r.read_u8()?;
        let colormap_entries = r.read_u16::<BigEndian>()?;
        let red_mask = r.read_u32::<BigEndian>()?;
        let green_mask = r.read_u32::<BigEndian>()?;
        let blue_mask = r.read_u32::<BigEndian>()?;

        // unused
        for _ in 0..4 {
            r.read_u8()?;
        }

        Ok(VisualType {
            visual_id,
            class,
            bits_per_rgb_value,
            colormap_entries,
            red_mask,
            green_mask,
            blue_mask,
        })
    }
}
