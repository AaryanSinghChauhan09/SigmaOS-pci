//! PCI configuration space model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarType { Memory, Io }

#[derive(Debug, Clone, Copy)]
pub struct Bar {
    pub bar_type: BarType,
    pub base: u64,
    pub size: u32,
}

#[derive(Debug, Clone)]
pub struct ConfigSpace {
    pub vendor_id: u16,
    pub device_id: u16,
    pub bars: [Option<Bar>; 6],
}

impl ConfigSpace {
    pub fn new(vendor_id: u16, device_id: u16) -> Self {
        Self { vendor_id, device_id, bars: [None; 6] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_space() {
        let cs = ConfigSpace::new(0x8086, 0x1234);
        assert_eq!(cs.vendor_id, 0x8086);
    }
}
