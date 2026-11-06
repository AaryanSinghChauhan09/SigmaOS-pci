//! PCI bus and addressing model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PciAddress {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
}

impl PciAddress {
    pub fn new(bus: u8, device: u8, function: u8) -> Self {
        Self { bus, device, function }
    }
}

#[derive(Debug, Default)]
pub struct PciBus {
    pub devices: Vec<PciAddress>,
}

impl PciBus {
    pub fn new() -> Self { Self::default() }
    pub fn scan(&mut self) { /* model enumeration */ }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pci_address() {
        let a = PciAddress::new(0, 1, 0);
        assert_eq!(a.bus, 0);
    }
}
