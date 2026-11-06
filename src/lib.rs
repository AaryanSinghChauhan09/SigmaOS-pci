//! SigmaOS-pci — PCI bus model.
//!
//! Models PCI devices, BDF (bus/device/function), configuration space,
//! BARs (Base Address Registers), and PCI driver management.
//! Deterministic and std-only (no actual hardware access).

pub mod pci;
pub mod device;
pub mod cfg;
pub mod driver;

pub use pci::{PciBus, PciAddress};
pub use device::{PciDevice, PciClass, PciVendor};
pub use cfg::{ConfigSpace, Bar};
pub use driver::{PciDriver, DriverManager};
