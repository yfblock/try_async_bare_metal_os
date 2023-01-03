use alloc::vec;
use core::ptr::NonNull;
use core::sync::atomic::*;
use lazy_static::lazy_static;
use spin::lazy::Lazy;
use spin::Once;
use virtio_drivers::{Hal, MmioTransport, PhysAddr, VirtAddr, VirtIOBlk, VirtIOHeader};
use crate::mutex::Mutex;

extern "C" {
    fn end();
}

lazy_static! {
    static ref DMA_PADDR: AtomicUsize = AtomicUsize::new(end as usize);
}

pub static mut DEVICE: Once<Mutex<VirtIOBlk<HalImpl, MmioTransport>>> = Once::new();

pub struct HalImpl;

impl Hal for HalImpl {
    fn dma_alloc(pages: usize) -> PhysAddr {
        let paddr = DMA_PADDR.fetch_add(0x1000 * pages, Ordering::SeqCst);
        println!("alloc DMA: paddr={:#x}, pages={}", paddr, pages);
        paddr
    }

    fn dma_dealloc(paddr: PhysAddr, pages: usize) -> i32 {
        println!("dealloc DMA: paddr={:#x}, pages={}", paddr, pages);
        0
    }

    fn phys_to_virt(paddr: PhysAddr) -> VirtAddr {
        paddr
    }

    fn virt_to_phys(vaddr: VirtAddr) -> PhysAddr {
        vaddr
    }
}

pub fn init() {
    unsafe {
        DEVICE.call_once(|| {
            let header = NonNull::new(0x10001000 as *mut VirtIOHeader).unwrap();
            let transport = unsafe { MmioTransport::new(header) }.unwrap();
            let device = VirtIOBlk::<HalImpl, MmioTransport>::new(transport)
                .expect("failed to create blk driver");
            Mutex::new(device)
        });
    }
}