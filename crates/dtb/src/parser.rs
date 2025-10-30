const DTB_VERSION: u32 = 17;
const MAGIC_VALUE: u32 = 0xd00dfeed;

#[repr(C)]
#[derive(Debug)]
pub struct FdtHeader {
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
}

pub struct Dtb {
    ptr: *const u32,
    pub fdt_header: FdtHeader,
}

impl Dtb {
    pub fn new(ptr: *const u32) -> Result<Dtb, DtbInitError> {
        let be_header = unsafe { &*(ptr as *const FdtHeader) };

        // Header is Big-Endian by default so we need to convert it
        let header = FdtHeader {
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

        Ok(Dtb {
            ptr,
            fdt_header: header,
        })
    }
}
