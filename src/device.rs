//! PCI device model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PciClass {
    Unclassified,
    MassStorage,
    Network,
    Display,
    Multimedia,
    Memory,
    Bridge,
    Communication,
    GenericSystem,
    InputDevice,
    DockingStation,
    Processor,
    SerialBus,
    Wireless,
    IntelligentController,
    SatelliteCommunication,
    Encryption,
    SignalProcessing,
}

#[derive(Debug, Clone)]
pub struct PciVendor {
    pub id: u16,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct PciDevice {
    pub address: super::PciAddress,
    pub vendor: PciVendor,
    pub device_id: u16,
    pub class: PciClass,
    pub revision: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pci_device() {
        let d = PciDevice {
            address: crate::pci::PciAddress::new(0, 1, 0),
            vendor: PciVendor { id: 0x8086, name: "Intel".into() },
            device_id: 0x1234,
            class: PciClass::Network,
            revision: 1,
        };
        assert_eq!(d.class, PciClass::Network);
    }
}
