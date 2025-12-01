use core::fmt::Debug;

use dtb_reader::DeviceTreeNode;

use crate::DriverManager;

pub trait Driver: Send + Sync + Debug {
    /// Initializes driver and registers its capabilities to driver manager
    /// Returns `true` if it succeeds
    fn try_initialize(node: &DeviceTreeNode, path: &str, manager: &mut DriverManager) -> bool
    where
        Self: Sized;

    /// Returns array of compatibility strings
    fn compatible() -> &'static [&'static str]
    where
        Self: Sized;
}
