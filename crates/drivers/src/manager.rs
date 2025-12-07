use core::any::{Any, TypeId};

use alloc::{
    collections::{btree_map::BTreeMap, vec_deque::VecDeque},
    format,
    string::{String, ToString},
    sync::Arc,
};
use dtb_reader::DeviceTreeNode;
use log::info;
use spin::Mutex;

use crate::registry::get_registry;

#[derive(Default)]
pub struct DriverManager {
    drivers: BTreeMap<(String, TypeId), Arc<dyn Any + Send + Sync>>,
}

impl DriverManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Load drivers based on Device Tree
    pub fn load_drivers(&mut self, dtb_root: &DeviceTreeNode) {
        let mut queue: VecDeque<(DeviceTreeNode, String)> = VecDeque::new();

        for child in dtb_root.children() {
            queue.push_back((child, String::new()));
        }

        while let Some((node, parent_path)) = queue.pop_front() {
            let node_name = node.full_name();
            let current_path = if parent_path.is_empty() {
                format!("/{}", node_name)
            } else {
                format!("{}/{}", parent_path, node_name)
            };

            if self.try_init_driver(&node, &current_path) {
                info!("Loaded driver for '{current_path}'");
            }

            for child in node.children() {
                queue.push_back((child, current_path.clone()));
            }
        }
    }

    /// Tries to initialize a compatible driver, returns `true` when it succeeds
    fn try_init_driver(&mut self, node: &DeviceTreeNode, path: &str) -> bool {
        let registry = get_registry();
        let prop = match node.get_property("compatible") {
            Some(p) => p,
            None => return false,
        };

        let compatibles = prop
            .raw_value()
            .split(|&b| b == 0)
            .filter_map(|bytes| core::str::from_utf8(bytes).ok())
            .filter(|s| !s.is_empty());

        for compatible in compatibles {
            if let Some(init_fn) = registry.get_compatible(compatible)
                && init_fn(node, path, self)
            {
                return true;
            }
        }
        false
    }

    /// Registers a specific capability (trait) for a path.
    pub(crate) fn register_capability<T: ?Sized + 'static>(
        &mut self,
        path: &str,
        implementation: Arc<Mutex<T>>,
    ) where
        Arc<Mutex<T>>: Send + Sync,
    {
        let type_id = TypeId::of::<T>();
        let stored: Arc<dyn Any + Send + Sync> = Arc::new(implementation);

        self.drivers.insert((path.to_string(), type_id), stored);
    }

    /// Gets a driver by path, cast to a specific trait.
    ///
    /// Example:
    ///
    /// ```rust
    /// let manager: DriverManager = ...;
    /// let uart: Arc<Mutex<dyn UartDriver>> = manager.get_by_path::<dyn UartDriver>("/soc/serial").unwrap();
    /// ```
    pub fn get_by_path<T: ?Sized + 'static>(&self, path: &str) -> Option<Arc<Mutex<T>>>
    where
        Arc<Mutex<T>>: Send + Sync,
    {
        let type_id = TypeId::of::<T>();
        let any_entry = self.drivers.get(&(path.to_string(), type_id))?;
        let wrapper = any_entry.downcast_ref::<Arc<Mutex<T>>>()?;

        Some(wrapper.clone())
    }
}
