#[derive(Debug)]
pub struct QueryTree {
    sequence_number: u16,
    root: u32,
    parent: Option<u32>,
    children: Vec<u32>,
}

impl QueryTree {
    fn read<T: io::Read>(r: &mut T) -> anyhow::Result<Self> {
        todo()
    }
}
