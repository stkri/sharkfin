#![no_std]

pub struct GIC<const BASE: usize>;

impl<const BASE: usize> GIC<BASE> {
    const DISTRIBUTOR_BASE: usize = BASE + 0x1000;
    const CPU_INTERFACE_BASE: usize = BASE + 0x2000;

    const GICD_CTLR: usize = Self::DISTRIBUTOR_BASE + 0x000;
    const GICD_ISENABLER: usize = Self::DISTRIBUTOR_BASE + 0x100;
    const GICD_IPRIORITYR: usize = Self::DISTRIBUTOR_BASE + 0x400;
    const GICD_ITARGETSR: usize = Self::DISTRIBUTOR_BASE + 0x800;

    const GICC_CTLR: usize = Self::CPU_INTERFACE_BASE + 0x00;
    const GICC_PMR: usize = Self::CPU_INTERFACE_BASE + 0x04;
    const GICC_IAR: usize = Self::CPU_INTERFACE_BASE + 0x0C;
    const GICC_EOIR: usize = Self::CPU_INTERFACE_BASE + 0x10;

    pub fn init() {
        unsafe {
            let gicd_ctlr = Self::GICD_CTLR as *mut u32;
            gicd_ctlr.write_volatile(0);

            let gicd_isenabler = (Self::GICD_ISENABLER + 4) as *mut u32;
            gicd_isenabler.write_volatile(1 << 25);

            let gicd_itargetsr = (Self::GICD_ITARGETSR + 57) as *mut u8;
            gicd_itargetsr.write_volatile(0x01); // CPU 0

            let gicd_ipriorityr = (Self::GICD_IPRIORITYR + 57) as *mut u8;
            gicd_ipriorityr.write_volatile(0xA0);

            let gicd_ctlr = Self::GICD_CTLR as *mut u32;
            gicd_ctlr.write_volatile(1);

            let gicc_ctlr = Self::GICC_CTLR as *mut u32;
            gicc_ctlr.write_volatile(1);

            let gicc_pmr = Self::GICC_PMR as *mut u32;
            gicc_pmr.write_volatile(0xF0);
        }
    }

    pub fn read_interrupt_id() -> u32 {
        unsafe {
            let gicc_iar = Self::GICC_IAR as *const u32;
            gicc_iar.read_volatile()
        }
    }

    pub fn end_interrupt(interrupt_id: u32) {
        unsafe {
            let gicc_eoir = Self::GICC_EOIR as *mut u32;
            gicc_eoir.write_volatile(interrupt_id);
        }
    }
}
