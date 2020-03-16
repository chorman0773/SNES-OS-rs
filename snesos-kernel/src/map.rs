#![allow(non_upper_case_globals)]

use snesdev::volatile::AtomicCell;
use crate::file::Fd;
use std::ops::Index;

#[repr(u8)]
pub enum DevNo{
    Rom = 0,
    WRam = 1,
    PRam = 2,
    Xreg = 4,
    Zeros = 5,
    SwapRom = 6,
    Ptrom = 7,
    ExptReg = 8,
    Extdma = 9,
    MemtrapMirror = 14,
    Memtrap = 15
}

#[repr(u8,align(32))]
pub enum Page{
    PgUnmapped([u8;29],u16) = 0,
    PgReg([u8;29],u16) = 1,
    PgDev{dev: DevNo,properties: u16,dev_pg: u32,mirror_offset: u32,swap_ref: u8,__reserved: [u8;15],checksum: u16} = 2
}

#[repr(C)]
#[derive(Copy,Clone)]
struct PageTable{
    page_size: u8,
    __reserved: [u8;31],
    pages: [Page;4096]
}

impl AtomicCell<PageTable>{
    pub fn get_pages(&self) -> &AtomicCell<[Page;4096]>{
        unsafe { self.map(|ptr| { (ptr as *mut u8)+32 as *mut [Page;4096]}) }
    }
}

const PROPS_READ: u16 = 0x0001;
const PROPS_WRITE: u16 = 0x0002;
const PROPS_VOLATILE: u16 = 0x0008;
const PROPS_MASK: u16 = 0x000b;
const MEMTRAP_TYPE_FILE: u32 = 0x00010000;
const MEMTRAP_TYPE_SWAP: u32 = 0x00020000;


extern"C"{
    #[no_mangle]
    static _CurrProcPtBase: AtomicCell<PageTable>;
    #[no_mangle]
    static _KernelPtBase: AtomicCell<PageTable>;

    #[no_mangle]
    static _PageUpdate: AtomicCell<u8>;
}

pub fn get_current_process_pages() -> &AtomicCell<[Page;4096]>{
    return unsafe{ _CurrProcPtBase.get_pages()}
}

pub fn get_kernel_pages() -> &AtomicCell<[Page;4096]>{
    return unsafe{ _KernelPtBase.get_pages()}
}

pub fn compute_checksum(pg: &Page) -> u16{
    let bytes = pg as *Page as *[u16;16];
    let mut checksum = 0u32;
    for val in unsafe { *bytes } {
        checksum += val;
    }
    while checksum > 0xFFFF {
        checksum = checksum & 0xFFFF + checksum >>16;
    }
    !(checksum as u16)
}

pub fn map_file_page(pg: u16,fd: Fd,offset: u32, properties: u16){
    let page = get_current_process_pages().index(pg as iszie);
    let properties = properties&PROPS_MASK | 0x0e00;
    let dev_pg = (fd.0 as u32)|MEMTRAP_TYPE_FILE;
    let mut store_page = Page::PgDev {dev: DevNo::Memtrap, properties, dev_pg, mirror_offset: offset, swap_ref: 0, __reserved: [0;15], checksum: 0 };
    store_page.checksum = compute_checksum(&store_page);
    page.store(store_page);
}

