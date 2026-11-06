//! PCI driver model.

#[derive(Debug, Clone)]
pub struct PciDriver {
    pub name: String,
    pub vendor_id: u16,
    pub device_id: u16,
}

#[derive(Debug, Default)]
pub struct DriverManager {
    pub drivers: Vec<PciDriver>,
}

impl DriverManager {
    pub fn new() -> Self { Self::default() }
    pub fn register(&mut self, d: PciDriver) { self.drivers.push(d); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn driver_manager() {
        let mut dm = DriverManager::new();
        dm.register(PciDriver { name: "e1000".into(), vendor_id: 0x8086, device_id: 0x100e });
        assert_eq!(dm.drivers.len(), 1);
    }
}
