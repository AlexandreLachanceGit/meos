use alloc::{boxed::Box, collections::btree_map::BTreeMap};

use dtb_reader::DeviceTreeNode;
use spin::Once;

use crate::{DriverManager, driver::Driver, drivers::ns16550a::Ns16550a};

type DriverInitFn = Box<dyn Fn(&DeviceTreeNode, &str, &mut DriverManager) -> bool + Send + Sync>;

/// Registry of all available drivers
///
/// **IMPORTANT: If multiple drivers have the same compatibility, the latest driver that is registered
/// will overwrite the previous one(s)**
pub struct DriverRegistry {
    // Maps compatibility string to driver's try_initialize fn
    drivers: BTreeMap<&'static str, DriverInitFn>,
}

impl DriverRegistry {
    fn new() -> Self {
        Self {
            drivers: BTreeMap::new(),
        }
    }

    /// Registers a driver
    fn register_driver<T>(&mut self)
    where
        T: Driver + 'static + Send + Sync,
    {
        for &compatible in T::compatible() {
            self.drivers.insert(compatible, Box::new(T::try_initialize));
        }
    }

    /// Get compatible driver based on compatibility string
    pub fn get_compatible(&self, compatible: &str) -> Option<&DriverInitFn> {
        self.drivers.get(compatible)
    }
}

pub fn get_registry() -> &'static DriverRegistry {
    static REGISTRY: Once<DriverRegistry> = Once::new();

    REGISTRY.call_once(|| {
        let mut registry = DriverRegistry::new();

        registry.register_driver::<Ns16550a>();

        registry
    })
}
