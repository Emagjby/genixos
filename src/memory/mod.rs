use crate::{BootInfo, debug, info};

pub fn inspect(info: &'static BootInfo) {
    info!("Memory discovery started\n");

    let mut total_described_bytes: u64 = 0;
    let mut usable_bytes: u64 = 0;
    let mut usable_regions: u16 = 0;

    for (i, region) in info.memory_map.iter().enumerate() {
        let start_addr = region.range.start_addr();
        let end_addr = region.range.end_addr();
        let size = end_addr - start_addr;

        if region.region_type == bootloader::bootinfo::MemoryRegionType::Usable {
            usable_bytes += size;
            usable_regions += 1;
        }

        total_described_bytes += size;

        debug!("Region {}: {:#010x}..{:#010x}", i, start_addr, end_addr);
        debug!("Type: {:?}", region.region_type);
        debug!("Size: {} bytes\n", size);
    }

    info!("Memory discovery completed\n");

    debug!(
        "Total memory described by map: {} bytes",
        total_described_bytes
    );
    debug!("Total memory classified as usable: {} bytes", usable_bytes);
    debug!("Number of usable regions: {}\n", usable_regions);
}
