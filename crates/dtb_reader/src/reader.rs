use crate::{
    DeviceTreeNode,
    reserve_entry::{MemoryReserveEntry, MemoryReserveEntryIter},
    tree::ParsingError,
};

const DTB_VERSION: u32 = 17;
const MAGIC_VALUE: u32 = 0xd00dfeed;

#[repr(C)]
#[derive(Debug)]
pub struct Header {
    magic: u32,
    totalsize: u32,
    off_dt_struct: u32,
    off_dt_strings: u32,
    off_mem_rsvmap: u32,
    version: u32,
    last_comp_version: u32,
    boot_cpuid_phys: u32,
    size_dt_strings: u32,
    size_dt_struct: u32,
}

#[derive(Debug)]
pub enum DtbInitError {
    UnsupportedDtbVersion,
    InvalidHeader { expected: u32, found: u32 },
    NoCpusNode,
    ParsingError(ParsingError),
}

pub struct DtbReader {
    ptr: *const u32,
    pub fdt_header: Header,
    root_node: DeviceTreeNode,
    cpus_node: DeviceTreeNode,
    aliases_node: Option<DeviceTreeNode>,
}

impl DtbReader {
    /// Initializes a new DtbReader from a raw pointer to a Device Tree Blob (DTB).
    ///
    /// # Errors
    ///
    /// Returns an error on invalid or unsupported DTB.
    ///
    /// # Safety
    ///
    /// **Unsafe**. Caller must ensure `ptr` points to a valid DTB memory region for the lifetime of `DtbReader`.
    ///
    pub unsafe fn new(ptr: *const u32) -> Result<DtbReader, DtbInitError> {
        let be_header = unsafe { &*(ptr as *const Header) };

        // Header is Big-Endian by default so we need to convert it
        let header = Header {
            magic: u32::from_be(be_header.magic),
            totalsize: u32::from_be(be_header.totalsize),
            off_dt_struct: u32::from_be(be_header.off_dt_struct),
            off_dt_strings: u32::from_be(be_header.off_dt_strings),
            off_mem_rsvmap: u32::from_be(be_header.off_mem_rsvmap),
            version: u32::from_be(be_header.version),
            last_comp_version: u32::from_be(be_header.last_comp_version),
            boot_cpuid_phys: u32::from_be(be_header.boot_cpuid_phys),
            size_dt_strings: u32::from_be(be_header.size_dt_strings),
            size_dt_struct: u32::from_be(be_header.size_dt_struct),
        };

        if header.version != DTB_VERSION && header.last_comp_version != DTB_VERSION {
            return Err(DtbInitError::UnsupportedDtbVersion);
        }

        if header.magic != MAGIC_VALUE {
            return Err(DtbInitError::InvalidHeader {
                expected: MAGIC_VALUE,
                found: header.magic,
            });
        }

        let str_block_ptr = unsafe { ptr.byte_offset(header.off_dt_strings as isize) as *const u8 };
        let root_node_ptr = unsafe { ptr.byte_offset(header.off_dt_struct as isize) };
        let root_node = DeviceTreeNode::parse(root_node_ptr, str_block_ptr)
            .map_err(DtbInitError::ParsingError)?;

        let mut cpus_node = Err(DtbInitError::NoCpusNode);
        let mut aliases_node = None;

        for n in root_node.children() {
            match n.name() {
                "cpus" => cpus_node = Ok(n),
                "aliases" => aliases_node = Some(n),
                _ => {}
            }
        }

        Ok(DtbReader {
            ptr,
            fdt_header: header,
            root_node,
            cpus_node: cpus_node?,
            aliases_node,
        })
    }

    pub fn reserve_entry_iter(&self) -> impl Iterator<Item = MemoryReserveEntry> {
        let start_ptr = unsafe {
            self.ptr
                .byte_offset(self.fdt_header.off_mem_rsvmap as isize)
        };
        MemoryReserveEntryIter::new(start_ptr)
    }

    pub fn root_node(&self) -> DeviceTreeNode {
        self.root_node
    }

    pub fn cpus_node(&self) -> DeviceTreeNode {
        self.cpus_node
    }

    pub fn resolve_alias(&self, alias: &str) -> Option<&str> {
        for prop in self.aliases_node?.properties() {
            if prop.name() == alias {
                return prop.value_str();
            }
        }

        None
    }

    pub fn find_node(&self, path: &str) -> Option<DeviceTreeNode> {
        if path == "/" {
            return Some(self.root_node);
        }

        let real_path = if path.starts_with("/") {
            path
        } else {
            self.resolve_alias(path)?
        };

        let mut current = self.root_node;

        for part in real_path[1..].split("/") {
            current = current.get_child(part)?;
        }

        Some(current)
    }
}
