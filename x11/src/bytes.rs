use std::io;
use std::str;

pub fn read_string<T: io::Read>(r: &mut T, n: usize) -> anyhow::Result<String> {
    let mut buf = vec![0; n];
    r.read_exact(&mut buf[..])?;
    Ok(String::from(str::from_utf8(&buf[..n])?))
}

pub fn skip<T: io::Read>(r: &mut T, n: usize) -> anyhow::Result<()> {
    let mut buf = vec![0; n];
    r.read_exact(&mut buf[..]).map_err(|e| e.into())
}

pub fn read_u8<T: io::Read>(r: &mut T) -> anyhow::Result<u8> {
    let mut buf = [0; 1];
    r.read_exact(&mut buf[..])?;
    Ok(buf[0])
}

pub fn read_u16<T: io::Read>(r: &mut T) -> anyhow::Result<u16> {
    let mut buf = [0; 2];
    r.read_exact(&mut buf[..])?;
    Ok(u16::from_be_bytes(buf))
}

pub fn read_u32<T: io::Read>(r: &mut T) -> anyhow::Result<u32> {
    let mut buf = [0; 4];
    r.read_exact(&mut buf[..])?;
    Ok(u32::from_be_bytes(buf))
}

pub fn write_u8<T: io::Write>(w: &mut T, v: u8) -> io::Result<()> {
    w.write_all(&[v])
}

pub fn write_u16<T: io::Write>(w: &mut T, v: u16) -> io::Result<()> {
    w.write_all(&(v).to_be_bytes())
}

pub fn write_u32<T: io::Write>(w: &mut T, v: u32) -> io::Result<()> {
    w.write_all(&(v).to_be_bytes())
}

pub fn pad(n: usize) -> usize {
    (4 - (n % 4)) % 4
}
