#![allow(non_camel_case_types)]
#![allow(dead_code)]
#[doc = "Peripheral Access Controller"]
pub const PAC: pac::Pac = unsafe { pac::Pac::from_ptr(0x4000_0000usize as _) };
#[doc = "Non-Volatile Memory Controller"]
pub const NVMCTRL: nvmctrl::Nvmctrl = unsafe { nvmctrl::Nvmctrl::from_ptr(0x4100_4000usize as _) };
pub mod common {
    use core::marker::PhantomData;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct RW;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct R;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct W;
    mod sealed {
        use super::*;
        pub trait Access {}
        impl Access for R {}
        impl Access for W {}
        impl Access for RW {}
    }
    pub trait Access: sealed::Access + Copy {}
    impl Access for R {}
    impl Access for W {}
    impl Access for RW {}
    pub trait Read: Access {}
    impl Read for RW {}
    impl Read for R {}
    pub trait Write: Access {}
    impl Write for RW {}
    impl Write for W {}
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct Reg<T: Copy, A: Access> {
        ptr: *mut u8,
        phantom: PhantomData<*mut (T, A)>,
    }
    unsafe impl<T: Copy, A: Access> Send for Reg<T, A> {}
    unsafe impl<T: Copy, A: Access> Sync for Reg<T, A> {}
    impl<T: Copy, A: Access> Reg<T, A> {
        #[allow(clippy::missing_safety_doc)]
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut T) -> Self {
            Self {
                ptr: ptr as _,
                phantom: PhantomData,
            }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut T {
            self.ptr as _
        }
    }
    impl<T: Copy, A: Read> Reg<T, A> {
        #[inline(always)]
        pub fn read(&self) -> T {
            unsafe { (self.ptr as *mut T).read_volatile() }
        }
    }
    impl<T: Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write_value(&self, val: T) {
            unsafe { (self.ptr as *mut T).write_volatile(val) }
        }
    }
    impl<T: Default + Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            let mut val = Default::default();
            let res = f(&mut val);
            self.write_value(val);
            res
        }
    }
    impl<T: Copy, A: Read + Write> Reg<T, A> {
        #[inline(always)]
        pub fn modify<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            let mut val = self.read();
            let res = f(&mut val);
            self.write_value(val);
            res
        }
    }
}
pub mod pac {
    #[doc = "Peripheral Access Controller"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pac {
        ptr: *mut u8,
    }
    unsafe impl Send for Pac {}
    unsafe impl Sync for Pac {}
    impl Pac {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Write control"]
        #[inline(always)]
        pub const fn wrctrl(self) -> super::common::Reg<regs::Wrctrl, super::common::RW> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Event control"]
        #[inline(always)]
        pub const fn evctrl(self) -> super::common::Reg<regs::Evctrl, super::common::RW> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Interrupt enable clear"]
        #[inline(always)]
        pub const fn intenclr(self) -> super::common::Reg<regs::Intenclr, super::common::RW> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Interrupt enable set"]
        #[inline(always)]
        pub const fn intenset(self) -> super::common::Reg<regs::Intenset, super::common::RW> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x09usize) as _) }
        }
        #[doc = "Bridge interrupt flag status"]
        #[inline(always)]
        pub const fn intflagahb(self) -> super::common::Reg<regs::Intflagahb, super::common::RW> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Peripheral interrupt flag status - Bridge A"]
        #[inline(always)]
        pub const fn intflaga(self) -> super::common::Reg<regs::Intflaga, super::common::RW> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Peripheral interrupt flag status - Bridge B"]
        #[inline(always)]
        pub const fn intflagb(self) -> super::common::Reg<regs::Intflagb, super::common::RW> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "Peripheral interrupt flag status - Bridge C"]
        #[inline(always)]
        pub const fn intflagc(self) -> super::common::Reg<regs::Intflagc, super::common::RW> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "Peripheral write protection status - Bridge A"]
        #[inline(always)]
        pub const fn statusa(self) -> super::common::Reg<regs::Statusa, super::common::R> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
        }
        #[doc = "Peripheral write protection status - Bridge B"]
        #[inline(always)]
        pub const fn statusb(self) -> super::common::Reg<regs::Statusb, super::common::R> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
        }
        #[doc = "Peripheral write protection status - Bridge C"]
        #[inline(always)]
        pub const fn statusc(self) -> super::common::Reg<regs::Statusc, super::common::R> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
        }
        #[doc = "Peripheral non-secure status - Bridge A"]
        #[inline(always)]
        pub const fn nonseca(self) -> super::common::Reg<regs::Nonseca, super::common::R> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
        }
        #[doc = "Peripheral non-secure status - Bridge B"]
        #[inline(always)]
        pub const fn nonsecb(self) -> super::common::Reg<regs::Nonsecb, super::common::R> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
        }
        #[doc = "Peripheral non-secure status - Bridge C"]
        #[inline(always)]
        pub const fn nonsecc(self) -> super::common::Reg<regs::Nonsecc, super::common::R> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
        }
        #[doc = "Peripheral secure status locked - Bridge A"]
        #[inline(always)]
        pub const fn seclocka(self) -> super::common::Reg<regs::Seclocka, super::common::R> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
        }
        #[doc = "Peripheral secure status locked - Bridge B"]
        #[inline(always)]
        pub const fn seclockb(self) -> super::common::Reg<regs::Seclockb, super::common::R> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
        }
        #[doc = "Peripheral secure status locked - Bridge C"]
        #[inline(always)]
        pub const fn seclockc(self) -> super::common::Reg<regs::Seclockc, super::common::R> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Event control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Evctrl(pub u8);
        impl Evctrl {
            #[doc = "Peripheral acess error event output"]
            #[inline(always)]
            pub const fn erreo(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral acess error event output"]
            #[inline(always)]
            pub fn set_erreo(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
        }
        impl Default for Evctrl {
            #[inline(always)]
            fn default() -> Evctrl {
                Evctrl(0)
            }
        }
        #[doc = "Interrupt enable clear"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Intenclr(pub u8);
        impl Intenclr {
            #[doc = "Peripheral access error interrupt disable"]
            #[inline(always)]
            pub const fn err(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral access error interrupt disable"]
            #[inline(always)]
            pub fn set_err(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
        }
        impl Default for Intenclr {
            #[inline(always)]
            fn default() -> Intenclr {
                Intenclr(0)
            }
        }
        #[doc = "Interrupt enable set"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Intenset(pub u8);
        impl Intenset {
            #[doc = "Peripheral access error interrupt enable"]
            #[inline(always)]
            pub const fn err(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Peripheral access error interrupt enable"]
            #[inline(always)]
            pub fn set_err(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
        }
        impl Default for Intenset {
            #[inline(always)]
            fn default() -> Intenset {
                Intenset(0)
            }
        }
        #[doc = "Peripheral interrupt flag status - Bridge A"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Intflaga(pub u32);
        impl Intflaga {
            #[doc = "PAC"]
            #[inline(always)]
            pub const fn pac_(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "PAC"]
            #[inline(always)]
            pub fn set_pac_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "PM"]
            #[inline(always)]
            pub const fn pm_(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "PM"]
            #[inline(always)]
            pub fn set_pm_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "MCLK"]
            #[inline(always)]
            pub const fn mclk_(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "MCLK"]
            #[inline(always)]
            pub fn set_mclk_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "RSTC"]
            #[inline(always)]
            pub const fn rstc_(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "RSTC"]
            #[inline(always)]
            pub fn set_rstc_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "OSCCTRL"]
            #[inline(always)]
            pub const fn oscctrl_(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "OSCCTRL"]
            #[inline(always)]
            pub fn set_oscctrl_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "OSC32KCTRL"]
            #[inline(always)]
            pub const fn osc32kctrl_(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "OSC32KCTRL"]
            #[inline(always)]
            pub fn set_osc32kctrl_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "SUPC"]
            #[inline(always)]
            pub const fn supc_(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "SUPC"]
            #[inline(always)]
            pub fn set_supc_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "GCLK"]
            #[inline(always)]
            pub const fn gclk_(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "GCLK"]
            #[inline(always)]
            pub fn set_gclk_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "WDT"]
            #[inline(always)]
            pub const fn wdt_(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "WDT"]
            #[inline(always)]
            pub fn set_wdt_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "RTC"]
            #[inline(always)]
            pub const fn rtc_(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "RTC"]
            #[inline(always)]
            pub fn set_rtc_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "EIC"]
            #[inline(always)]
            pub const fn eic_(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "EIC"]
            #[inline(always)]
            pub fn set_eic_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "FREQM"]
            #[inline(always)]
            pub const fn freqm_(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "FREQM"]
            #[inline(always)]
            pub fn set_freqm_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "PORT"]
            #[inline(always)]
            pub const fn port_(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "PORT"]
            #[inline(always)]
            pub fn set_port_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "AC"]
            #[inline(always)]
            pub const fn ac_(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "AC"]
            #[inline(always)]
            pub fn set_ac_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
        }
        impl Default for Intflaga {
            #[inline(always)]
            fn default() -> Intflaga {
                Intflaga(0)
            }
        }
        #[doc = "Bridge interrupt flag status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Intflagahb(pub u32);
        impl Intflagahb {
            #[doc = "FLASH"]
            #[inline(always)]
            pub const fn flash_(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "FLASH"]
            #[inline(always)]
            pub fn set_flash_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "HPB0"]
            #[inline(always)]
            pub const fn hpb0_(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "HPB0"]
            #[inline(always)]
            pub fn set_hpb0_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "HPB1"]
            #[inline(always)]
            pub const fn hpb1_(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "HPB1"]
            #[inline(always)]
            pub fn set_hpb1_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "HPB2"]
            #[inline(always)]
            pub const fn hpb2_(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "HPB2"]
            #[inline(always)]
            pub fn set_hpb2_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "HSRAMCPU"]
            #[inline(always)]
            pub const fn hsramcpu_(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "HSRAMCPU"]
            #[inline(always)]
            pub fn set_hsramcpu_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "HSRAMDMAC"]
            #[inline(always)]
            pub const fn hsramdmac_(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "HSRAMDMAC"]
            #[inline(always)]
            pub fn set_hsramdmac_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "HSRAMDSU"]
            #[inline(always)]
            pub const fn hsramdsu_(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "HSRAMDSU"]
            #[inline(always)]
            pub fn set_hsramdsu_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
        }
        impl Default for Intflagahb {
            #[inline(always)]
            fn default() -> Intflagahb {
                Intflagahb(0)
            }
        }
        #[doc = "Peripheral interrupt flag status - Bridge B"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Intflagb(pub u32);
        impl Intflagb {
            #[doc = "IDAU"]
            #[inline(always)]
            pub const fn idau_(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "IDAU"]
            #[inline(always)]
            pub fn set_idau_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "DSU"]
            #[inline(always)]
            pub const fn dsu_(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "DSU"]
            #[inline(always)]
            pub fn set_dsu_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "NVMCTRL"]
            #[inline(always)]
            pub const fn nvmctrl_(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "NVMCTRL"]
            #[inline(always)]
            pub fn set_nvmctrl_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "DMAC"]
            #[inline(always)]
            pub const fn dmac_(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "DMAC"]
            #[inline(always)]
            pub fn set_dmac_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Intflagb {
            #[inline(always)]
            fn default() -> Intflagb {
                Intflagb(0)
            }
        }
        #[doc = "Peripheral interrupt flag status - Bridge C"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Intflagc(pub u32);
        impl Intflagc {
            #[doc = "EVSYS"]
            #[inline(always)]
            pub const fn evsys_(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "EVSYS"]
            #[inline(always)]
            pub fn set_evsys_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "SERCOM0"]
            #[inline(always)]
            pub const fn sercom0_(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "SERCOM0"]
            #[inline(always)]
            pub fn set_sercom0_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "SERCOM1"]
            #[inline(always)]
            pub const fn sercom1_(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "SERCOM1"]
            #[inline(always)]
            pub fn set_sercom1_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "TC0"]
            #[inline(always)]
            pub const fn tc0_(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "TC0"]
            #[inline(always)]
            pub fn set_tc0_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "TC1"]
            #[inline(always)]
            pub const fn tc1_(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "TC1"]
            #[inline(always)]
            pub fn set_tc1_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "TC2"]
            #[inline(always)]
            pub const fn tc2_(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "TC2"]
            #[inline(always)]
            pub fn set_tc2_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "ADC"]
            #[inline(always)]
            pub const fn adc_(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "ADC"]
            #[inline(always)]
            pub fn set_adc_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "DAC"]
            #[inline(always)]
            pub const fn dac_(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "DAC"]
            #[inline(always)]
            pub fn set_dac_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "PTC"]
            #[inline(always)]
            pub const fn ptc_(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "PTC"]
            #[inline(always)]
            pub fn set_ptc_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "TRNG"]
            #[inline(always)]
            pub const fn trng_(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "TRNG"]
            #[inline(always)]
            pub fn set_trng_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "CCL"]
            #[inline(always)]
            pub const fn ccl_(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "CCL"]
            #[inline(always)]
            pub fn set_ccl_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "OPAMP"]
            #[inline(always)]
            pub const fn opamp_(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "OPAMP"]
            #[inline(always)]
            pub fn set_opamp_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "TRAM"]
            #[inline(always)]
            pub const fn tram_(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "TRAM"]
            #[inline(always)]
            pub fn set_tram_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
        }
        impl Default for Intflagc {
            #[inline(always)]
            fn default() -> Intflagc {
                Intflagc(0)
            }
        }
        #[doc = "Peripheral non-secure status - Bridge A"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Nonseca(pub u32);
        impl Nonseca {
            #[doc = "PAC Non-Secure"]
            #[inline(always)]
            pub const fn pac_(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "PAC Non-Secure"]
            #[inline(always)]
            pub fn set_pac_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "PM Non-Secure"]
            #[inline(always)]
            pub const fn pm_(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "PM Non-Secure"]
            #[inline(always)]
            pub fn set_pm_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "MCLK Non-Secure"]
            #[inline(always)]
            pub const fn mclk_(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "MCLK Non-Secure"]
            #[inline(always)]
            pub fn set_mclk_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "RSTC Non-Secure"]
            #[inline(always)]
            pub const fn rstc_(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "RSTC Non-Secure"]
            #[inline(always)]
            pub fn set_rstc_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "OSCCTRL Non-Secure"]
            #[inline(always)]
            pub const fn oscctrl_(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "OSCCTRL Non-Secure"]
            #[inline(always)]
            pub fn set_oscctrl_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "OSC32KCTRL Non-Secure"]
            #[inline(always)]
            pub const fn osc32kctrl_(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "OSC32KCTRL Non-Secure"]
            #[inline(always)]
            pub fn set_osc32kctrl_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "SUPC Non-Secure"]
            #[inline(always)]
            pub const fn supc_(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "SUPC Non-Secure"]
            #[inline(always)]
            pub fn set_supc_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "GCLK Non-Secure"]
            #[inline(always)]
            pub const fn gclk_(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "GCLK Non-Secure"]
            #[inline(always)]
            pub fn set_gclk_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "WDT Non-Secure"]
            #[inline(always)]
            pub const fn wdt_(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "WDT Non-Secure"]
            #[inline(always)]
            pub fn set_wdt_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "RTC Non-Secure"]
            #[inline(always)]
            pub const fn rtc_(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "RTC Non-Secure"]
            #[inline(always)]
            pub fn set_rtc_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "EIC Non-Secure"]
            #[inline(always)]
            pub const fn eic_(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "EIC Non-Secure"]
            #[inline(always)]
            pub fn set_eic_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "FREQM Non-Secure"]
            #[inline(always)]
            pub const fn freqm_(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "FREQM Non-Secure"]
            #[inline(always)]
            pub fn set_freqm_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "PORT Non-Secure"]
            #[inline(always)]
            pub const fn port_(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "PORT Non-Secure"]
            #[inline(always)]
            pub fn set_port_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "AC Non-Secure"]
            #[inline(always)]
            pub const fn ac_(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "AC Non-Secure"]
            #[inline(always)]
            pub fn set_ac_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
        }
        impl Default for Nonseca {
            #[inline(always)]
            fn default() -> Nonseca {
                Nonseca(0)
            }
        }
        #[doc = "Peripheral non-secure status - Bridge B"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Nonsecb(pub u32);
        impl Nonsecb {
            #[doc = "IDAU Non-Secure"]
            #[inline(always)]
            pub const fn idau_(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "IDAU Non-Secure"]
            #[inline(always)]
            pub fn set_idau_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "DSU Non-Secure"]
            #[inline(always)]
            pub const fn dsu_(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "DSU Non-Secure"]
            #[inline(always)]
            pub fn set_dsu_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "NVMCTRL Non-Secure"]
            #[inline(always)]
            pub const fn nvmctrl_(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "NVMCTRL Non-Secure"]
            #[inline(always)]
            pub fn set_nvmctrl_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "DMAC Non-Secure"]
            #[inline(always)]
            pub const fn dmac_(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "DMAC Non-Secure"]
            #[inline(always)]
            pub fn set_dmac_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Nonsecb {
            #[inline(always)]
            fn default() -> Nonsecb {
                Nonsecb(0)
            }
        }
        #[doc = "Peripheral non-secure status - Bridge C"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Nonsecc(pub u32);
        impl Nonsecc {
            #[doc = "EVSYS Non-Secure"]
            #[inline(always)]
            pub const fn evsys_(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "EVSYS Non-Secure"]
            #[inline(always)]
            pub fn set_evsys_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "SERCOM0 Non-Secure"]
            #[inline(always)]
            pub const fn sercom0_(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "SERCOM0 Non-Secure"]
            #[inline(always)]
            pub fn set_sercom0_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "SERCOM1 Non-Secure"]
            #[inline(always)]
            pub const fn sercom1_(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "SERCOM1 Non-Secure"]
            #[inline(always)]
            pub fn set_sercom1_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "TC0 Non-Secure"]
            #[inline(always)]
            pub const fn tc0_(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "TC0 Non-Secure"]
            #[inline(always)]
            pub fn set_tc0_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "TC1 Non-Secure"]
            #[inline(always)]
            pub const fn tc1_(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "TC1 Non-Secure"]
            #[inline(always)]
            pub fn set_tc1_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "TC2 Non-Secure"]
            #[inline(always)]
            pub const fn tc2_(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "TC2 Non-Secure"]
            #[inline(always)]
            pub fn set_tc2_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "ADC Non-Secure"]
            #[inline(always)]
            pub const fn adc_(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "ADC Non-Secure"]
            #[inline(always)]
            pub fn set_adc_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "DAC Non-Secure"]
            #[inline(always)]
            pub const fn dac_(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "DAC Non-Secure"]
            #[inline(always)]
            pub fn set_dac_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "PTC Non-Secure"]
            #[inline(always)]
            pub const fn ptc_(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "PTC Non-Secure"]
            #[inline(always)]
            pub fn set_ptc_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "TRNG Non-Secure"]
            #[inline(always)]
            pub const fn trng_(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "TRNG Non-Secure"]
            #[inline(always)]
            pub fn set_trng_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "CCL Non-Secure"]
            #[inline(always)]
            pub const fn ccl_(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "CCL Non-Secure"]
            #[inline(always)]
            pub fn set_ccl_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "OPAMP Non-Secure"]
            #[inline(always)]
            pub const fn opamp_(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "OPAMP Non-Secure"]
            #[inline(always)]
            pub fn set_opamp_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "TRAM Non-Secure"]
            #[inline(always)]
            pub const fn tram_(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "TRAM Non-Secure"]
            #[inline(always)]
            pub fn set_tram_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
        }
        impl Default for Nonsecc {
            #[inline(always)]
            fn default() -> Nonsecc {
                Nonsecc(0)
            }
        }
        #[doc = "Peripheral secure status locked - Bridge A"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Seclocka(pub u32);
        impl Seclocka {
            #[doc = "PAC Secure Status Locked"]
            #[inline(always)]
            pub const fn pac_(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "PAC Secure Status Locked"]
            #[inline(always)]
            pub fn set_pac_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "PM Secure Status Locked"]
            #[inline(always)]
            pub const fn pm_(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "PM Secure Status Locked"]
            #[inline(always)]
            pub fn set_pm_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "MCLK Secure Status Locked"]
            #[inline(always)]
            pub const fn mclk_(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "MCLK Secure Status Locked"]
            #[inline(always)]
            pub fn set_mclk_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "RSTC Secure Status Locked"]
            #[inline(always)]
            pub const fn rstc_(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "RSTC Secure Status Locked"]
            #[inline(always)]
            pub fn set_rstc_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "OSCCTRL Secure Status Locked"]
            #[inline(always)]
            pub const fn oscctrl_(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "OSCCTRL Secure Status Locked"]
            #[inline(always)]
            pub fn set_oscctrl_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "OSC32KCTRL Secure Status Locked"]
            #[inline(always)]
            pub const fn osc32kctrl_(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "OSC32KCTRL Secure Status Locked"]
            #[inline(always)]
            pub fn set_osc32kctrl_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "SUPC Secure Status Locked"]
            #[inline(always)]
            pub const fn supc_(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "SUPC Secure Status Locked"]
            #[inline(always)]
            pub fn set_supc_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "GCLK Secure Status Locked"]
            #[inline(always)]
            pub const fn gclk_(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "GCLK Secure Status Locked"]
            #[inline(always)]
            pub fn set_gclk_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "WDT Secure Status Locked"]
            #[inline(always)]
            pub const fn wdt_(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "WDT Secure Status Locked"]
            #[inline(always)]
            pub fn set_wdt_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "RTC Secure Status Locked"]
            #[inline(always)]
            pub const fn rtc_(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "RTC Secure Status Locked"]
            #[inline(always)]
            pub fn set_rtc_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "EIC Secure Status Locked"]
            #[inline(always)]
            pub const fn eic_(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "EIC Secure Status Locked"]
            #[inline(always)]
            pub fn set_eic_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "FREQM Secure Status Locked"]
            #[inline(always)]
            pub const fn freqm_(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "FREQM Secure Status Locked"]
            #[inline(always)]
            pub fn set_freqm_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "PORT Secure Status Locked"]
            #[inline(always)]
            pub const fn port_(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "PORT Secure Status Locked"]
            #[inline(always)]
            pub fn set_port_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "AC Secure Status Locked"]
            #[inline(always)]
            pub const fn ac_(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "AC Secure Status Locked"]
            #[inline(always)]
            pub fn set_ac_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
        }
        impl Default for Seclocka {
            #[inline(always)]
            fn default() -> Seclocka {
                Seclocka(0)
            }
        }
        #[doc = "Peripheral secure status locked - Bridge B"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Seclockb(pub u32);
        impl Seclockb {
            #[doc = "IDAU Secure Status Locked"]
            #[inline(always)]
            pub const fn idau_(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "IDAU Secure Status Locked"]
            #[inline(always)]
            pub fn set_idau_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "DSU Secure Status Locked"]
            #[inline(always)]
            pub const fn dsu_(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "DSU Secure Status Locked"]
            #[inline(always)]
            pub fn set_dsu_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "NVMCTRL Secure Status Locked"]
            #[inline(always)]
            pub const fn nvmctrl_(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "NVMCTRL Secure Status Locked"]
            #[inline(always)]
            pub fn set_nvmctrl_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "DMAC Secure Status Locked"]
            #[inline(always)]
            pub const fn dmac_(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "DMAC Secure Status Locked"]
            #[inline(always)]
            pub fn set_dmac_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Seclockb {
            #[inline(always)]
            fn default() -> Seclockb {
                Seclockb(0)
            }
        }
        #[doc = "Peripheral secure status locked - Bridge C"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Seclockc(pub u32);
        impl Seclockc {
            #[doc = "EVSYS Secure Status Locked"]
            #[inline(always)]
            pub const fn evsys_(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "EVSYS Secure Status Locked"]
            #[inline(always)]
            pub fn set_evsys_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "SERCOM0 Secure Status Locked"]
            #[inline(always)]
            pub const fn sercom0_(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "SERCOM0 Secure Status Locked"]
            #[inline(always)]
            pub fn set_sercom0_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "SERCOM1 Secure Status Locked"]
            #[inline(always)]
            pub const fn sercom1_(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "SERCOM1 Secure Status Locked"]
            #[inline(always)]
            pub fn set_sercom1_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "TC0 Secure Status Locked"]
            #[inline(always)]
            pub const fn tc0_(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "TC0 Secure Status Locked"]
            #[inline(always)]
            pub fn set_tc0_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "TC1 Secure Status Locked"]
            #[inline(always)]
            pub const fn tc1_(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "TC1 Secure Status Locked"]
            #[inline(always)]
            pub fn set_tc1_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "TC2 Secure Status Locked"]
            #[inline(always)]
            pub const fn tc2_(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "TC2 Secure Status Locked"]
            #[inline(always)]
            pub fn set_tc2_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "ADC Secure Status Locked"]
            #[inline(always)]
            pub const fn adc_(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "ADC Secure Status Locked"]
            #[inline(always)]
            pub fn set_adc_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "DAC Secure Status Locked"]
            #[inline(always)]
            pub const fn dac_(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "DAC Secure Status Locked"]
            #[inline(always)]
            pub fn set_dac_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "PTC Secure Status Locked"]
            #[inline(always)]
            pub const fn ptc_(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "PTC Secure Status Locked"]
            #[inline(always)]
            pub fn set_ptc_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "TRNG Secure Status Locked"]
            #[inline(always)]
            pub const fn trng_(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "TRNG Secure Status Locked"]
            #[inline(always)]
            pub fn set_trng_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "CCL Secure Status Locked"]
            #[inline(always)]
            pub const fn ccl_(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "CCL Secure Status Locked"]
            #[inline(always)]
            pub fn set_ccl_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "OPAMP Secure Status Locked"]
            #[inline(always)]
            pub const fn opamp_(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "OPAMP Secure Status Locked"]
            #[inline(always)]
            pub fn set_opamp_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "TRAM Secure Status Locked"]
            #[inline(always)]
            pub const fn tram_(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "TRAM Secure Status Locked"]
            #[inline(always)]
            pub fn set_tram_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
        }
        impl Default for Seclockc {
            #[inline(always)]
            fn default() -> Seclockc {
                Seclockc(0)
            }
        }
        #[doc = "Peripheral write protection status - Bridge A"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Statusa(pub u32);
        impl Statusa {
            #[doc = "PAC APB Protect Enable"]
            #[inline(always)]
            pub const fn pac_(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "PAC APB Protect Enable"]
            #[inline(always)]
            pub fn set_pac_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "PM APB Protect Enable"]
            #[inline(always)]
            pub const fn pm_(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "PM APB Protect Enable"]
            #[inline(always)]
            pub fn set_pm_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "MCLK APB Protect Enable"]
            #[inline(always)]
            pub const fn mclk_(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "MCLK APB Protect Enable"]
            #[inline(always)]
            pub fn set_mclk_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "RSTC APB Protect Enable"]
            #[inline(always)]
            pub const fn rstc_(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "RSTC APB Protect Enable"]
            #[inline(always)]
            pub fn set_rstc_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "OSCCTRL APB Protect Enable"]
            #[inline(always)]
            pub const fn oscctrl_(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "OSCCTRL APB Protect Enable"]
            #[inline(always)]
            pub fn set_oscctrl_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "OSC32KCTRL APB Protect Enable"]
            #[inline(always)]
            pub const fn osc32kctrl_(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "OSC32KCTRL APB Protect Enable"]
            #[inline(always)]
            pub fn set_osc32kctrl_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "SUPC APB Protect Enable"]
            #[inline(always)]
            pub const fn supc_(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "SUPC APB Protect Enable"]
            #[inline(always)]
            pub fn set_supc_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "GCLK APB Protect Enable"]
            #[inline(always)]
            pub const fn gclk_(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "GCLK APB Protect Enable"]
            #[inline(always)]
            pub fn set_gclk_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "WDT APB Protect Enable"]
            #[inline(always)]
            pub const fn wdt_(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "WDT APB Protect Enable"]
            #[inline(always)]
            pub fn set_wdt_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "RTC APB Protect Enable"]
            #[inline(always)]
            pub const fn rtc_(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "RTC APB Protect Enable"]
            #[inline(always)]
            pub fn set_rtc_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "EIC APB Protect Enable"]
            #[inline(always)]
            pub const fn eic_(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "EIC APB Protect Enable"]
            #[inline(always)]
            pub fn set_eic_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "FREQM APB Protect Enable"]
            #[inline(always)]
            pub const fn freqm_(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "FREQM APB Protect Enable"]
            #[inline(always)]
            pub fn set_freqm_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "PORT APB Protect Enable"]
            #[inline(always)]
            pub const fn port_(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "PORT APB Protect Enable"]
            #[inline(always)]
            pub fn set_port_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "AC APB Protect Enable"]
            #[inline(always)]
            pub const fn ac_(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "AC APB Protect Enable"]
            #[inline(always)]
            pub fn set_ac_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
        }
        impl Default for Statusa {
            #[inline(always)]
            fn default() -> Statusa {
                Statusa(0)
            }
        }
        #[doc = "Peripheral write protection status - Bridge B"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Statusb(pub u32);
        impl Statusb {
            #[doc = "IDAU APB Protect Enable"]
            #[inline(always)]
            pub const fn idau_(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "IDAU APB Protect Enable"]
            #[inline(always)]
            pub fn set_idau_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "DSU APB Protect Enable"]
            #[inline(always)]
            pub const fn dsu_(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "DSU APB Protect Enable"]
            #[inline(always)]
            pub fn set_dsu_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "NVMCTRL APB Protect Enable"]
            #[inline(always)]
            pub const fn nvmctrl_(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "NVMCTRL APB Protect Enable"]
            #[inline(always)]
            pub fn set_nvmctrl_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "DMAC APB Protect Enable"]
            #[inline(always)]
            pub const fn dmac_(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "DMAC APB Protect Enable"]
            #[inline(always)]
            pub fn set_dmac_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
        }
        impl Default for Statusb {
            #[inline(always)]
            fn default() -> Statusb {
                Statusb(0)
            }
        }
        #[doc = "Peripheral write protection status - Bridge C"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Statusc(pub u32);
        impl Statusc {
            #[doc = "EVSYS APB Protect Enable"]
            #[inline(always)]
            pub const fn evsys_(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "EVSYS APB Protect Enable"]
            #[inline(always)]
            pub fn set_evsys_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "SERCOM0 APB Protect Enable"]
            #[inline(always)]
            pub const fn sercom0_(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "SERCOM0 APB Protect Enable"]
            #[inline(always)]
            pub fn set_sercom0_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
            #[doc = "SERCOM1 APB Protect Enable"]
            #[inline(always)]
            pub const fn sercom1_(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "SERCOM1 APB Protect Enable"]
            #[inline(always)]
            pub fn set_sercom1_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "TC0 APB Protect Enable"]
            #[inline(always)]
            pub const fn tc0_(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "TC0 APB Protect Enable"]
            #[inline(always)]
            pub fn set_tc0_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
            }
            #[doc = "TC1 APB Protect Enable"]
            #[inline(always)]
            pub const fn tc1_(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "TC1 APB Protect Enable"]
            #[inline(always)]
            pub fn set_tc1_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
            }
            #[doc = "TC2 APB Protect Enable"]
            #[inline(always)]
            pub const fn tc2_(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "TC2 APB Protect Enable"]
            #[inline(always)]
            pub fn set_tc2_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "ADC APB Protect Enable"]
            #[inline(always)]
            pub const fn adc_(&self) -> bool {
                let val = (self.0 >> 7usize) & 0x01;
                val != 0
            }
            #[doc = "ADC APB Protect Enable"]
            #[inline(always)]
            pub fn set_adc_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
            }
            #[doc = "DAC APB Protect Enable"]
            #[inline(always)]
            pub const fn dac_(&self) -> bool {
                let val = (self.0 >> 8usize) & 0x01;
                val != 0
            }
            #[doc = "DAC APB Protect Enable"]
            #[inline(always)]
            pub fn set_dac_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
            }
            #[doc = "PTC APB Protect Enable"]
            #[inline(always)]
            pub const fn ptc_(&self) -> bool {
                let val = (self.0 >> 9usize) & 0x01;
                val != 0
            }
            #[doc = "PTC APB Protect Enable"]
            #[inline(always)]
            pub fn set_ptc_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
            }
            #[doc = "TRNG APB Protect Enable"]
            #[inline(always)]
            pub const fn trng_(&self) -> bool {
                let val = (self.0 >> 10usize) & 0x01;
                val != 0
            }
            #[doc = "TRNG APB Protect Enable"]
            #[inline(always)]
            pub fn set_trng_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
            }
            #[doc = "CCL APB Protect Enable"]
            #[inline(always)]
            pub const fn ccl_(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "CCL APB Protect Enable"]
            #[inline(always)]
            pub fn set_ccl_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "OPAMP APB Protect Enable"]
            #[inline(always)]
            pub const fn opamp_(&self) -> bool {
                let val = (self.0 >> 12usize) & 0x01;
                val != 0
            }
            #[doc = "OPAMP APB Protect Enable"]
            #[inline(always)]
            pub fn set_opamp_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
            }
            #[doc = "TRAM APB Protect Enable"]
            #[inline(always)]
            pub const fn tram_(&self) -> bool {
                let val = (self.0 >> 13usize) & 0x01;
                val != 0
            }
            #[doc = "TRAM APB Protect Enable"]
            #[inline(always)]
            pub fn set_tram_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
            }
        }
        impl Default for Statusc {
            #[inline(always)]
            fn default() -> Statusc {
                Statusc(0)
            }
        }
        #[doc = "Write control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Wrctrl(pub u32);
        impl Wrctrl {
            #[doc = "Peripheral identifier"]
            #[inline(always)]
            pub const fn perid(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "Peripheral identifier"]
            #[inline(always)]
            pub fn set_perid(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
            #[doc = "Peripheral access control key"]
            #[inline(always)]
            pub const fn key(&self) -> super::vals::Key {
                let val = (self.0 >> 16usize) & 0xff;
                super::vals::Key::from_bits(val as u8)
            }
            #[doc = "Peripheral access control key"]
            #[inline(always)]
            pub fn set_key(&mut self, val: super::vals::Key) {
                self.0 =
                    (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
            }
        }
        impl Default for Wrctrl {
            #[inline(always)]
            fn default() -> Wrctrl {
                Wrctrl(0)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Key {
            #[doc = "No action"]
            OFF = 0x0,
            #[doc = "Clear protection"]
            CLR = 0x01,
            #[doc = "Set protection"]
            SET = 0x02,
            #[doc = "Set and lock protection"]
            SETLCK = 0x03,
            #[doc = "Set IP secure"]
            SETSEC = 0x04,
            #[doc = "Set IP non-secure"]
            SETNONSEC = 0x05,
            #[doc = "Lock IP security value"]
            SECLOCK = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
            _RESERVED_10 = 0x10,
            _RESERVED_11 = 0x11,
            _RESERVED_12 = 0x12,
            _RESERVED_13 = 0x13,
            _RESERVED_14 = 0x14,
            _RESERVED_15 = 0x15,
            _RESERVED_16 = 0x16,
            _RESERVED_17 = 0x17,
            _RESERVED_18 = 0x18,
            _RESERVED_19 = 0x19,
            _RESERVED_1a = 0x1a,
            _RESERVED_1b = 0x1b,
            _RESERVED_1c = 0x1c,
            _RESERVED_1d = 0x1d,
            _RESERVED_1e = 0x1e,
            _RESERVED_1f = 0x1f,
            _RESERVED_20 = 0x20,
            _RESERVED_21 = 0x21,
            _RESERVED_22 = 0x22,
            _RESERVED_23 = 0x23,
            _RESERVED_24 = 0x24,
            _RESERVED_25 = 0x25,
            _RESERVED_26 = 0x26,
            _RESERVED_27 = 0x27,
            _RESERVED_28 = 0x28,
            _RESERVED_29 = 0x29,
            _RESERVED_2a = 0x2a,
            _RESERVED_2b = 0x2b,
            _RESERVED_2c = 0x2c,
            _RESERVED_2d = 0x2d,
            _RESERVED_2e = 0x2e,
            _RESERVED_2f = 0x2f,
            _RESERVED_30 = 0x30,
            _RESERVED_31 = 0x31,
            _RESERVED_32 = 0x32,
            _RESERVED_33 = 0x33,
            _RESERVED_34 = 0x34,
            _RESERVED_35 = 0x35,
            _RESERVED_36 = 0x36,
            _RESERVED_37 = 0x37,
            _RESERVED_38 = 0x38,
            _RESERVED_39 = 0x39,
            _RESERVED_3a = 0x3a,
            _RESERVED_3b = 0x3b,
            _RESERVED_3c = 0x3c,
            _RESERVED_3d = 0x3d,
            _RESERVED_3e = 0x3e,
            _RESERVED_3f = 0x3f,
            _RESERVED_40 = 0x40,
            _RESERVED_41 = 0x41,
            _RESERVED_42 = 0x42,
            _RESERVED_43 = 0x43,
            _RESERVED_44 = 0x44,
            _RESERVED_45 = 0x45,
            _RESERVED_46 = 0x46,
            _RESERVED_47 = 0x47,
            _RESERVED_48 = 0x48,
            _RESERVED_49 = 0x49,
            _RESERVED_4a = 0x4a,
            _RESERVED_4b = 0x4b,
            _RESERVED_4c = 0x4c,
            _RESERVED_4d = 0x4d,
            _RESERVED_4e = 0x4e,
            _RESERVED_4f = 0x4f,
            _RESERVED_50 = 0x50,
            _RESERVED_51 = 0x51,
            _RESERVED_52 = 0x52,
            _RESERVED_53 = 0x53,
            _RESERVED_54 = 0x54,
            _RESERVED_55 = 0x55,
            _RESERVED_56 = 0x56,
            _RESERVED_57 = 0x57,
            _RESERVED_58 = 0x58,
            _RESERVED_59 = 0x59,
            _RESERVED_5a = 0x5a,
            _RESERVED_5b = 0x5b,
            _RESERVED_5c = 0x5c,
            _RESERVED_5d = 0x5d,
            _RESERVED_5e = 0x5e,
            _RESERVED_5f = 0x5f,
            _RESERVED_60 = 0x60,
            _RESERVED_61 = 0x61,
            _RESERVED_62 = 0x62,
            _RESERVED_63 = 0x63,
            _RESERVED_64 = 0x64,
            _RESERVED_65 = 0x65,
            _RESERVED_66 = 0x66,
            _RESERVED_67 = 0x67,
            _RESERVED_68 = 0x68,
            _RESERVED_69 = 0x69,
            _RESERVED_6a = 0x6a,
            _RESERVED_6b = 0x6b,
            _RESERVED_6c = 0x6c,
            _RESERVED_6d = 0x6d,
            _RESERVED_6e = 0x6e,
            _RESERVED_6f = 0x6f,
            _RESERVED_70 = 0x70,
            _RESERVED_71 = 0x71,
            _RESERVED_72 = 0x72,
            _RESERVED_73 = 0x73,
            _RESERVED_74 = 0x74,
            _RESERVED_75 = 0x75,
            _RESERVED_76 = 0x76,
            _RESERVED_77 = 0x77,
            _RESERVED_78 = 0x78,
            _RESERVED_79 = 0x79,
            _RESERVED_7a = 0x7a,
            _RESERVED_7b = 0x7b,
            _RESERVED_7c = 0x7c,
            _RESERVED_7d = 0x7d,
            _RESERVED_7e = 0x7e,
            _RESERVED_7f = 0x7f,
            _RESERVED_80 = 0x80,
            _RESERVED_81 = 0x81,
            _RESERVED_82 = 0x82,
            _RESERVED_83 = 0x83,
            _RESERVED_84 = 0x84,
            _RESERVED_85 = 0x85,
            _RESERVED_86 = 0x86,
            _RESERVED_87 = 0x87,
            _RESERVED_88 = 0x88,
            _RESERVED_89 = 0x89,
            _RESERVED_8a = 0x8a,
            _RESERVED_8b = 0x8b,
            _RESERVED_8c = 0x8c,
            _RESERVED_8d = 0x8d,
            _RESERVED_8e = 0x8e,
            _RESERVED_8f = 0x8f,
            _RESERVED_90 = 0x90,
            _RESERVED_91 = 0x91,
            _RESERVED_92 = 0x92,
            _RESERVED_93 = 0x93,
            _RESERVED_94 = 0x94,
            _RESERVED_95 = 0x95,
            _RESERVED_96 = 0x96,
            _RESERVED_97 = 0x97,
            _RESERVED_98 = 0x98,
            _RESERVED_99 = 0x99,
            _RESERVED_9a = 0x9a,
            _RESERVED_9b = 0x9b,
            _RESERVED_9c = 0x9c,
            _RESERVED_9d = 0x9d,
            _RESERVED_9e = 0x9e,
            _RESERVED_9f = 0x9f,
            _RESERVED_a0 = 0xa0,
            _RESERVED_a1 = 0xa1,
            _RESERVED_a2 = 0xa2,
            _RESERVED_a3 = 0xa3,
            _RESERVED_a4 = 0xa4,
            _RESERVED_a5 = 0xa5,
            _RESERVED_a6 = 0xa6,
            _RESERVED_a7 = 0xa7,
            _RESERVED_a8 = 0xa8,
            _RESERVED_a9 = 0xa9,
            _RESERVED_aa = 0xaa,
            _RESERVED_ab = 0xab,
            _RESERVED_ac = 0xac,
            _RESERVED_ad = 0xad,
            _RESERVED_ae = 0xae,
            _RESERVED_af = 0xaf,
            _RESERVED_b0 = 0xb0,
            _RESERVED_b1 = 0xb1,
            _RESERVED_b2 = 0xb2,
            _RESERVED_b3 = 0xb3,
            _RESERVED_b4 = 0xb4,
            _RESERVED_b5 = 0xb5,
            _RESERVED_b6 = 0xb6,
            _RESERVED_b7 = 0xb7,
            _RESERVED_b8 = 0xb8,
            _RESERVED_b9 = 0xb9,
            _RESERVED_ba = 0xba,
            _RESERVED_bb = 0xbb,
            _RESERVED_bc = 0xbc,
            _RESERVED_bd = 0xbd,
            _RESERVED_be = 0xbe,
            _RESERVED_bf = 0xbf,
            _RESERVED_c0 = 0xc0,
            _RESERVED_c1 = 0xc1,
            _RESERVED_c2 = 0xc2,
            _RESERVED_c3 = 0xc3,
            _RESERVED_c4 = 0xc4,
            _RESERVED_c5 = 0xc5,
            _RESERVED_c6 = 0xc6,
            _RESERVED_c7 = 0xc7,
            _RESERVED_c8 = 0xc8,
            _RESERVED_c9 = 0xc9,
            _RESERVED_ca = 0xca,
            _RESERVED_cb = 0xcb,
            _RESERVED_cc = 0xcc,
            _RESERVED_cd = 0xcd,
            _RESERVED_ce = 0xce,
            _RESERVED_cf = 0xcf,
            _RESERVED_d0 = 0xd0,
            _RESERVED_d1 = 0xd1,
            _RESERVED_d2 = 0xd2,
            _RESERVED_d3 = 0xd3,
            _RESERVED_d4 = 0xd4,
            _RESERVED_d5 = 0xd5,
            _RESERVED_d6 = 0xd6,
            _RESERVED_d7 = 0xd7,
            _RESERVED_d8 = 0xd8,
            _RESERVED_d9 = 0xd9,
            _RESERVED_da = 0xda,
            _RESERVED_db = 0xdb,
            _RESERVED_dc = 0xdc,
            _RESERVED_dd = 0xdd,
            _RESERVED_de = 0xde,
            _RESERVED_df = 0xdf,
            _RESERVED_e0 = 0xe0,
            _RESERVED_e1 = 0xe1,
            _RESERVED_e2 = 0xe2,
            _RESERVED_e3 = 0xe3,
            _RESERVED_e4 = 0xe4,
            _RESERVED_e5 = 0xe5,
            _RESERVED_e6 = 0xe6,
            _RESERVED_e7 = 0xe7,
            _RESERVED_e8 = 0xe8,
            _RESERVED_e9 = 0xe9,
            _RESERVED_ea = 0xea,
            _RESERVED_eb = 0xeb,
            _RESERVED_ec = 0xec,
            _RESERVED_ed = 0xed,
            _RESERVED_ee = 0xee,
            _RESERVED_ef = 0xef,
            _RESERVED_f0 = 0xf0,
            _RESERVED_f1 = 0xf1,
            _RESERVED_f2 = 0xf2,
            _RESERVED_f3 = 0xf3,
            _RESERVED_f4 = 0xf4,
            _RESERVED_f5 = 0xf5,
            _RESERVED_f6 = 0xf6,
            _RESERVED_f7 = 0xf7,
            _RESERVED_f8 = 0xf8,
            _RESERVED_f9 = 0xf9,
            _RESERVED_fa = 0xfa,
            _RESERVED_fb = 0xfb,
            _RESERVED_fc = 0xfc,
            _RESERVED_fd = 0xfd,
            _RESERVED_fe = 0xfe,
            _RESERVED_ff = 0xff,
        }
        impl Key {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Key {
                unsafe { core::mem::transmute(val & 0xff) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Key {
            #[inline(always)]
            fn from(val: u8) -> Key {
                Key::from_bits(val)
            }
        }
        impl From<Key> for u8 {
            #[inline(always)]
            fn from(val: Key) -> u8 {
                Key::to_bits(val)
            }
        }
    }
}
pub mod nvmctrl {
    #[doc = "Non-Volatile Memory Controller"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nvmctrl {
        ptr: *mut u8,
    }
    unsafe impl Send for Nvmctrl {}
    unsafe impl Sync for Nvmctrl {}
    impl Nvmctrl {
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self { ptr: ptr as _ }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as _
        }
        #[doc = "Control A"]
        #[inline(always)]
        pub const fn ctrla(self) -> super::common::Reg<regs::Ctrla, super::common::W> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
        }
        #[doc = "Control B"]
        #[inline(always)]
        pub const fn ctrlb(self) -> super::common::Reg<regs::Ctrlb, super::common::RW> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }
        #[doc = "Control C"]
        #[inline(always)]
        pub const fn ctrlc(self) -> super::common::Reg<regs::Ctrlc, super::common::RW> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }
        #[doc = "Event Control"]
        #[inline(always)]
        pub const fn evctrl(self) -> super::common::Reg<regs::Evctrl, super::common::RW> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
        }
        #[doc = "Interrupt Enable Clear"]
        #[inline(always)]
        pub const fn intenclr(self) -> super::common::Reg<regs::Intenclr, super::common::RW> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
        }
        #[doc = "Interrupt Enable Set"]
        #[inline(always)]
        pub const fn intenset(self) -> super::common::Reg<regs::Intenset, super::common::RW> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }
        #[doc = "Interrupt Flag Status and Clear"]
        #[inline(always)]
        pub const fn intflag(self) -> super::common::Reg<regs::Intflag, super::common::RW> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }
        #[doc = "Status"]
        #[inline(always)]
        pub const fn status(self) -> super::common::Reg<regs::Status, super::common::R> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }
        #[doc = "Address"]
        #[inline(always)]
        pub const fn addr(self) -> super::common::Reg<regs::Addr, super::common::RW> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
        }
        #[doc = "Secure Unlock Register"]
        #[inline(always)]
        pub const fn sulck(self) -> super::common::Reg<regs::Sulck, super::common::RW> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }
        #[doc = "Non-Secure Unlock Register"]
        #[inline(always)]
        pub const fn nsulck(self) -> super::common::Reg<regs::Nsulck, super::common::RW> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x22usize) as _) }
        }
        #[doc = "NVM Parameter"]
        #[inline(always)]
        pub const fn param(self) -> super::common::Reg<regs::Param, super::common::RW> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
        }
        #[doc = "Data Scramble Configuration"]
        #[inline(always)]
        pub const fn dscc(self) -> super::common::Reg<regs::Dscc, super::common::W> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
        }
        #[doc = "Security Control"]
        #[inline(always)]
        pub const fn secctrl(self) -> super::common::Reg<regs::Secctrl, super::common::RW> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
        }
        #[doc = "Secure Boot Configuration"]
        #[inline(always)]
        pub const fn scfgb(self) -> super::common::Reg<regs::Scfgb, super::common::RW> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
        }
        #[doc = "Secure Application and Data Configuration"]
        #[inline(always)]
        pub const fn scfgad(self) -> super::common::Reg<regs::Scfgad, super::common::RW> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
        }
        #[doc = "Non-secure Write Enable"]
        #[inline(always)]
        pub const fn nonsec(self) -> super::common::Reg<regs::Nonsec, super::common::RW> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
        }
        #[doc = "Non-secure Write Reference Value"]
        #[inline(always)]
        pub const fn nschk(self) -> super::common::Reg<regs::Nschk, super::common::RW> {
            unsafe { super::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
        }
    }
    pub mod regs {
        #[doc = "Address"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Addr(pub u32);
        impl Addr {
            #[doc = "NVM Address Offset In The Selected Array"]
            #[inline(always)]
            pub const fn aoffset(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "NVM Address Offset In The Selected Array"]
            #[inline(always)]
            pub fn set_aoffset(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
            #[doc = "Array Select"]
            #[inline(always)]
            pub const fn array(&self) -> super::vals::Array {
                let val = (self.0 >> 22usize) & 0x03;
                super::vals::Array::from_bits(val as u8)
            }
            #[doc = "Array Select"]
            #[inline(always)]
            pub fn set_array(&mut self, val: super::vals::Array) {
                self.0 =
                    (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
            }
        }
        impl Default for Addr {
            #[inline(always)]
            fn default() -> Addr {
                Addr(0)
            }
        }
        #[doc = "Control A"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctrla(pub u16);
        impl Ctrla {
            #[doc = "Command"]
            #[inline(always)]
            pub const fn cmd(&self) -> super::vals::Cmd {
                let val = (self.0 >> 0usize) & 0x7f;
                super::vals::Cmd::from_bits(val as u8)
            }
            #[doc = "Command"]
            #[inline(always)]
            pub fn set_cmd(&mut self, val: super::vals::Cmd) {
                self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u16) & 0x7f) << 0usize);
            }
            #[doc = "Command Execution"]
            #[inline(always)]
            pub const fn cmdex(&self) -> super::vals::Cmdex {
                let val = (self.0 >> 8usize) & 0xff;
                super::vals::Cmdex::from_bits(val as u8)
            }
            #[doc = "Command Execution"]
            #[inline(always)]
            pub fn set_cmdex(&mut self, val: super::vals::Cmdex) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u16) & 0xff) << 8usize);
            }
        }
        impl Default for Ctrla {
            #[inline(always)]
            fn default() -> Ctrla {
                Ctrla(0)
            }
        }
        #[doc = "Control B"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctrlb(pub u32);
        impl Ctrlb {
            #[doc = "NVM Read Wait States"]
            #[inline(always)]
            pub const fn rws(&self) -> u8 {
                let val = (self.0 >> 1usize) & 0x0f;
                val as u8
            }
            #[doc = "NVM Read Wait States"]
            #[inline(always)]
            pub fn set_rws(&mut self, val: u8) {
                self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
            }
            #[doc = "Power Reduction Mode during Sleep"]
            #[inline(always)]
            pub const fn sleepprm(&self) -> super::vals::Sleepprm {
                let val = (self.0 >> 8usize) & 0x03;
                super::vals::Sleepprm::from_bits(val as u8)
            }
            #[doc = "Power Reduction Mode during Sleep"]
            #[inline(always)]
            pub fn set_sleepprm(&mut self, val: super::vals::Sleepprm) {
                self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
            }
            #[doc = "fast wake-up"]
            #[inline(always)]
            pub const fn fwup(&self) -> bool {
                let val = (self.0 >> 11usize) & 0x01;
                val != 0
            }
            #[doc = "fast wake-up"]
            #[inline(always)]
            pub fn set_fwup(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
            }
            #[doc = "NVMCTRL Read Mode"]
            #[inline(always)]
            pub const fn readmode(&self) -> super::vals::Readmode {
                let val = (self.0 >> 16usize) & 0x03;
                super::vals::Readmode::from_bits(val as u8)
            }
            #[doc = "NVMCTRL Read Mode"]
            #[inline(always)]
            pub fn set_readmode(&mut self, val: super::vals::Readmode) {
                self.0 =
                    (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
            }
            #[doc = "Cache Disable"]
            #[inline(always)]
            pub const fn cachedis(&self) -> bool {
                let val = (self.0 >> 18usize) & 0x01;
                val != 0
            }
            #[doc = "Cache Disable"]
            #[inline(always)]
            pub fn set_cachedis(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
            }
            #[doc = "Quick Write Enable"]
            #[inline(always)]
            pub const fn qwen(&self) -> bool {
                let val = (self.0 >> 19usize) & 0x01;
                val != 0
            }
            #[doc = "Quick Write Enable"]
            #[inline(always)]
            pub fn set_qwen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
            }
        }
        impl Default for Ctrlb {
            #[inline(always)]
            fn default() -> Ctrlb {
                Ctrlb(0)
            }
        }
        #[doc = "Control C"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ctrlc(pub u8);
        impl Ctrlc {
            #[doc = "Manual Write"]
            #[inline(always)]
            pub const fn manw(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Manual Write"]
            #[inline(always)]
            pub fn set_manw(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
        }
        impl Default for Ctrlc {
            #[inline(always)]
            fn default() -> Ctrlc {
                Ctrlc(0)
            }
        }
        #[doc = "Data Scramble Configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Dscc(pub u32);
        impl Dscc {
            #[doc = "Data Scramble Key"]
            #[inline(always)]
            pub const fn dsckey(&self) -> u32 {
                let val = (self.0 >> 0usize) & 0x3fff_ffff;
                val as u32
            }
            #[doc = "Data Scramble Key"]
            #[inline(always)]
            pub fn set_dsckey(&mut self, val: u32) {
                self.0 =
                    (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
            }
        }
        impl Default for Dscc {
            #[inline(always)]
            fn default() -> Dscc {
                Dscc(0)
            }
        }
        #[doc = "Event Control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Evctrl(pub u8);
        impl Evctrl {
            #[doc = "Auto Write Event Enable"]
            #[inline(always)]
            pub const fn autowei(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Auto Write Event Enable"]
            #[inline(always)]
            pub fn set_autowei(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "Auto Write Event Polarity Inverted"]
            #[inline(always)]
            pub const fn autowinv(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Auto Write Event Polarity Inverted"]
            #[inline(always)]
            pub fn set_autowinv(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
            }
        }
        impl Default for Evctrl {
            #[inline(always)]
            fn default() -> Evctrl {
                Evctrl(0)
            }
        }
        #[doc = "Interrupt Enable Clear"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Intenclr(pub u8);
        impl Intenclr {
            #[doc = "NVM Done Interrupt Clear"]
            #[inline(always)]
            pub const fn done(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "NVM Done Interrupt Clear"]
            #[inline(always)]
            pub fn set_done(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "Programming Error Status Interrupt Clear"]
            #[inline(always)]
            pub const fn proge(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Programming Error Status Interrupt Clear"]
            #[inline(always)]
            pub fn set_proge(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
            }
            #[doc = "Lock Error Status Interrupt Clear"]
            #[inline(always)]
            pub const fn locke(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Lock Error Status Interrupt Clear"]
            #[inline(always)]
            pub fn set_locke(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
            }
            #[doc = "NVM Error Interrupt Clear"]
            #[inline(always)]
            pub const fn nvme(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "NVM Error Interrupt Clear"]
            #[inline(always)]
            pub fn set_nvme(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
            }
            #[doc = "Key Write Error Interrupt Clear"]
            #[inline(always)]
            pub const fn keye(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Key Write Error Interrupt Clear"]
            #[inline(always)]
            pub fn set_keye(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
            }
            #[doc = "NS configuration change detected Interrupt Clear"]
            #[inline(always)]
            pub const fn nschk(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "NS configuration change detected Interrupt Clear"]
            #[inline(always)]
            pub fn set_nschk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
            }
        }
        impl Default for Intenclr {
            #[inline(always)]
            fn default() -> Intenclr {
                Intenclr(0)
            }
        }
        #[doc = "Interrupt Enable Set"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Intenset(pub u8);
        impl Intenset {
            #[doc = "NVM Done Interrupt Enable"]
            #[inline(always)]
            pub const fn done(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "NVM Done Interrupt Enable"]
            #[inline(always)]
            pub fn set_done(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "Programming Error Status Interrupt Enable"]
            #[inline(always)]
            pub const fn proge(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Programming Error Status Interrupt Enable"]
            #[inline(always)]
            pub fn set_proge(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
            }
            #[doc = "Lock Error Status Interrupt Enable"]
            #[inline(always)]
            pub const fn locke(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Lock Error Status Interrupt Enable"]
            #[inline(always)]
            pub fn set_locke(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
            }
            #[doc = "NVM Error Interrupt Enable"]
            #[inline(always)]
            pub const fn nvme(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "NVM Error Interrupt Enable"]
            #[inline(always)]
            pub fn set_nvme(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
            }
            #[doc = "Key Write Error Interrupt Enable"]
            #[inline(always)]
            pub const fn keye(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "Key Write Error Interrupt Enable"]
            #[inline(always)]
            pub fn set_keye(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
            }
            #[doc = "NS configuration change detected Interrupt Enable"]
            #[inline(always)]
            pub const fn nschk(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "NS configuration change detected Interrupt Enable"]
            #[inline(always)]
            pub fn set_nschk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
            }
        }
        impl Default for Intenset {
            #[inline(always)]
            fn default() -> Intenset {
                Intenset(0)
            }
        }
        #[doc = "Interrupt Flag Status and Clear"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Intflag(pub u8);
        impl Intflag {
            #[doc = "NVM Done"]
            #[inline(always)]
            pub const fn done(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "NVM Done"]
            #[inline(always)]
            pub fn set_done(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
            }
            #[doc = "Programming Error Status"]
            #[inline(always)]
            pub const fn proge(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Programming Error Status"]
            #[inline(always)]
            pub fn set_proge(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
            }
            #[doc = "Lock Error Status"]
            #[inline(always)]
            pub const fn locke(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Lock Error Status"]
            #[inline(always)]
            pub fn set_locke(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
            }
            #[doc = "NVM Error"]
            #[inline(always)]
            pub const fn nvme(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "NVM Error"]
            #[inline(always)]
            pub fn set_nvme(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
            }
            #[doc = "KEY Write Error"]
            #[inline(always)]
            pub const fn keye(&self) -> bool {
                let val = (self.0 >> 4usize) & 0x01;
                val != 0
            }
            #[doc = "KEY Write Error"]
            #[inline(always)]
            pub fn set_keye(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
            }
            #[doc = "NS configuration change detected"]
            #[inline(always)]
            pub const fn nschk(&self) -> bool {
                let val = (self.0 >> 5usize) & 0x01;
                val != 0
            }
            #[doc = "NS configuration change detected"]
            #[inline(always)]
            pub fn set_nschk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
            }
        }
        impl Default for Intflag {
            #[inline(always)]
            fn default() -> Intflag {
                Intflag(0)
            }
        }
        #[doc = "Non-secure Write Enable"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Nonsec(pub u32);
        impl Nonsec {
            #[doc = "Non-secure APB alias write enable, non-secure AHB writes to non-secure regions enable"]
            #[inline(always)]
            pub const fn write(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Non-secure APB alias write enable, non-secure AHB writes to non-secure regions enable"]
            #[inline(always)]
            pub fn set_write(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Nonsec {
            #[inline(always)]
            fn default() -> Nonsec {
                Nonsec(0)
            }
        }
        #[doc = "Non-secure Write Reference Value"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Nschk(pub u32);
        impl Nschk {
            #[doc = "Reference value to be checked against NONSEC.WRITE"]
            #[inline(always)]
            pub const fn write(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Reference value to be checked against NONSEC.WRITE"]
            #[inline(always)]
            pub fn set_write(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Nschk {
            #[inline(always)]
            fn default() -> Nschk {
                Nschk(0)
            }
        }
        #[doc = "Non-Secure Unlock Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Nsulck(pub u16);
        impl Nsulck {
            #[doc = "Non-Secure Boot Region"]
            #[inline(always)]
            pub const fn bns(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Non-Secure Boot Region"]
            #[inline(always)]
            pub fn set_bns(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
            }
            #[doc = "Non-Secure Application Region"]
            #[inline(always)]
            pub const fn ans(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Non-Secure Application Region"]
            #[inline(always)]
            pub fn set_ans(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
            }
            #[doc = "Non-Secure Data Region"]
            #[inline(always)]
            pub const fn dns(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Non-Secure Data Region"]
            #[inline(always)]
            pub fn set_dns(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
            }
            #[doc = "Write Key"]
            #[inline(always)]
            pub const fn nslkey(&self) -> super::vals::Nslkey {
                let val = (self.0 >> 8usize) & 0xff;
                super::vals::Nslkey::from_bits(val as u8)
            }
            #[doc = "Write Key"]
            #[inline(always)]
            pub fn set_nslkey(&mut self, val: super::vals::Nslkey) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u16) & 0xff) << 8usize);
            }
        }
        impl Default for Nsulck {
            #[inline(always)]
            fn default() -> Nsulck {
                Nsulck(0)
            }
        }
        #[doc = "NVM Parameter"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Param(pub u32);
        impl Param {
            #[doc = "FLASH Pages"]
            #[inline(always)]
            pub const fn flashp(&self) -> u16 {
                let val = (self.0 >> 0usize) & 0xffff;
                val as u16
            }
            #[doc = "FLASH Pages"]
            #[inline(always)]
            pub fn set_flashp(&mut self, val: u16) {
                self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
            }
            #[doc = "Page Size"]
            #[inline(always)]
            pub const fn psz(&self) -> super::vals::Psz {
                let val = (self.0 >> 16usize) & 0x07;
                super::vals::Psz::from_bits(val as u8)
            }
            #[doc = "Page Size"]
            #[inline(always)]
            pub fn set_psz(&mut self, val: super::vals::Psz) {
                self.0 =
                    (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
            }
            #[doc = "DATAFLASH Pages"]
            #[inline(always)]
            pub const fn dflashp(&self) -> u16 {
                let val = (self.0 >> 20usize) & 0x0fff;
                val as u16
            }
            #[doc = "DATAFLASH Pages"]
            #[inline(always)]
            pub fn set_dflashp(&mut self, val: u16) {
                self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
            }
        }
        impl Default for Param {
            #[inline(always)]
            fn default() -> Param {
                Param(0)
            }
        }
        #[doc = "Secure Application and Data Configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Scfgad(pub u32);
        impl Scfgad {
            #[doc = "User Row Write Enable"]
            #[inline(always)]
            pub const fn urwen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "User Row Write Enable"]
            #[inline(always)]
            pub fn set_urwen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
        }
        impl Default for Scfgad {
            #[inline(always)]
            fn default() -> Scfgad {
                Scfgad(0)
            }
        }
        #[doc = "Secure Boot Configuration"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Scfgb(pub u32);
        impl Scfgb {
            #[doc = "Boot Configuration Row Read Enable"]
            #[inline(always)]
            pub const fn bcren(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Boot Configuration Row Read Enable"]
            #[inline(always)]
            pub fn set_bcren(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Boot Configuration Row Write Enable"]
            #[inline(always)]
            pub const fn bcwen(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Boot Configuration Row Write Enable"]
            #[inline(always)]
            pub fn set_bcwen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
            }
        }
        impl Default for Scfgb {
            #[inline(always)]
            fn default() -> Scfgb {
                Scfgb(0)
            }
        }
        #[doc = "Security Control"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Secctrl(pub u32);
        impl Secctrl {
            #[doc = "Tamper Erase Enable"]
            #[inline(always)]
            pub const fn tampeen(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Tamper Erase Enable"]
            #[inline(always)]
            pub fn set_tampeen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
            }
            #[doc = "Silent Access"]
            #[inline(always)]
            pub const fn silacc(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Silent Access"]
            #[inline(always)]
            pub fn set_silacc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
            }
            #[doc = "Data Scramble Enable"]
            #[inline(always)]
            pub const fn dscen(&self) -> bool {
                let val = (self.0 >> 3usize) & 0x01;
                val != 0
            }
            #[doc = "Data Scramble Enable"]
            #[inline(always)]
            pub fn set_dscen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
            }
            #[doc = "Data Flash is eXecute Never"]
            #[inline(always)]
            pub const fn dxn(&self) -> bool {
                let val = (self.0 >> 6usize) & 0x01;
                val != 0
            }
            #[doc = "Data Flash is eXecute Never"]
            #[inline(always)]
            pub fn set_dxn(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
            }
            #[doc = "Tamper Rease Row"]
            #[inline(always)]
            pub const fn terow(&self) -> u8 {
                let val = (self.0 >> 8usize) & 0x07;
                val as u8
            }
            #[doc = "Tamper Rease Row"]
            #[inline(always)]
            pub fn set_terow(&mut self, val: u8) {
                self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
            }
            #[doc = "Write Key"]
            #[inline(always)]
            pub const fn key(&self) -> super::vals::Key {
                let val = (self.0 >> 24usize) & 0xff;
                super::vals::Key::from_bits(val as u8)
            }
            #[doc = "Write Key"]
            #[inline(always)]
            pub fn set_key(&mut self, val: super::vals::Key) {
                self.0 =
                    (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
            }
        }
        impl Default for Secctrl {
            #[inline(always)]
            fn default() -> Secctrl {
                Secctrl(0)
            }
        }
        #[doc = "Status"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Status(pub u16);
        impl Status {
            #[doc = "Power Reduction Mode"]
            #[inline(always)]
            pub const fn prm(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Power Reduction Mode"]
            #[inline(always)]
            pub fn set_prm(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
            }
            #[doc = "NVM Page Buffer Active Loading"]
            #[inline(always)]
            pub const fn load(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "NVM Page Buffer Active Loading"]
            #[inline(always)]
            pub fn set_load(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
            }
            #[doc = "NVM Ready"]
            #[inline(always)]
            pub const fn ready(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "NVM Ready"]
            #[inline(always)]
            pub fn set_ready(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
            }
            #[doc = "Debug Access Level Fuse"]
            #[inline(always)]
            pub const fn dalfuse(&self) -> u8 {
                let val = (self.0 >> 3usize) & 0x03;
                val as u8
            }
            #[doc = "Debug Access Level Fuse"]
            #[inline(always)]
            pub fn set_dalfuse(&mut self, val: u8) {
                self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u16) & 0x03) << 3usize);
            }
        }
        impl Default for Status {
            #[inline(always)]
            fn default() -> Status {
                Status(0)
            }
        }
        #[doc = "Secure Unlock Register"]
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Sulck(pub u16);
        impl Sulck {
            #[doc = "Secure Boot Region"]
            #[inline(always)]
            pub const fn bs(&self) -> bool {
                let val = (self.0 >> 0usize) & 0x01;
                val != 0
            }
            #[doc = "Secure Boot Region"]
            #[inline(always)]
            pub fn set_bs(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
            }
            #[doc = "Secure Application Region"]
            #[inline(always)]
            pub const fn as_(&self) -> bool {
                let val = (self.0 >> 1usize) & 0x01;
                val != 0
            }
            #[doc = "Secure Application Region"]
            #[inline(always)]
            pub fn set_as_(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
            }
            #[doc = "Data Secure Region"]
            #[inline(always)]
            pub const fn ds(&self) -> bool {
                let val = (self.0 >> 2usize) & 0x01;
                val != 0
            }
            #[doc = "Data Secure Region"]
            #[inline(always)]
            pub fn set_ds(&mut self, val: bool) {
                self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
            }
            #[doc = "Write Key"]
            #[inline(always)]
            pub const fn slkey(&self) -> super::vals::Slkey {
                let val = (self.0 >> 8usize) & 0xff;
                super::vals::Slkey::from_bits(val as u8)
            }
            #[doc = "Write Key"]
            #[inline(always)]
            pub fn set_slkey(&mut self, val: super::vals::Slkey) {
                self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u16) & 0xff) << 8usize);
            }
        }
        impl Default for Sulck {
            #[inline(always)]
            fn default() -> Sulck {
                Sulck(0)
            }
        }
    }
    pub mod vals {
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Array {
            #[doc = "FLASH Array"]
            FLASH = 0x0,
            #[doc = "DATA FLASH Array"]
            DATAFLASH = 0x01,
            #[doc = "Auxilliary Space"]
            AUX = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Array {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Array {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Array {
            #[inline(always)]
            fn from(val: u8) -> Array {
                Array::from_bits(val)
            }
        }
        impl From<Array> for u8 {
            #[inline(always)]
            fn from(val: Array) -> u8 {
                Array::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Cmd {
            _RESERVED_0 = 0x0,
            _RESERVED_1 = 0x01,
            #[doc = "Erase Row - Erases the row addressed by the ADDR register."]
            ER = 0x02,
            _RESERVED_3 = 0x03,
            #[doc = "Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register."]
            WP = 0x04,
            _RESERVED_5 = 0x05,
            _RESERVED_6 = 0x06,
            _RESERVED_7 = 0x07,
            _RESERVED_8 = 0x08,
            _RESERVED_9 = 0x09,
            _RESERVED_a = 0x0a,
            _RESERVED_b = 0x0b,
            _RESERVED_c = 0x0c,
            _RESERVED_d = 0x0d,
            _RESERVED_e = 0x0e,
            _RESERVED_f = 0x0f,
            _RESERVED_10 = 0x10,
            _RESERVED_11 = 0x11,
            _RESERVED_12 = 0x12,
            _RESERVED_13 = 0x13,
            _RESERVED_14 = 0x14,
            _RESERVED_15 = 0x15,
            _RESERVED_16 = 0x16,
            _RESERVED_17 = 0x17,
            _RESERVED_18 = 0x18,
            _RESERVED_19 = 0x19,
            _RESERVED_1a = 0x1a,
            _RESERVED_1b = 0x1b,
            _RESERVED_1c = 0x1c,
            _RESERVED_1d = 0x1d,
            _RESERVED_1e = 0x1e,
            _RESERVED_1f = 0x1f,
            _RESERVED_20 = 0x20,
            _RESERVED_21 = 0x21,
            _RESERVED_22 = 0x22,
            _RESERVED_23 = 0x23,
            _RESERVED_24 = 0x24,
            _RESERVED_25 = 0x25,
            _RESERVED_26 = 0x26,
            _RESERVED_27 = 0x27,
            _RESERVED_28 = 0x28,
            _RESERVED_29 = 0x29,
            _RESERVED_2a = 0x2a,
            _RESERVED_2b = 0x2b,
            _RESERVED_2c = 0x2c,
            _RESERVED_2d = 0x2d,
            _RESERVED_2e = 0x2e,
            _RESERVED_2f = 0x2f,
            _RESERVED_30 = 0x30,
            _RESERVED_31 = 0x31,
            _RESERVED_32 = 0x32,
            _RESERVED_33 = 0x33,
            _RESERVED_34 = 0x34,
            _RESERVED_35 = 0x35,
            _RESERVED_36 = 0x36,
            _RESERVED_37 = 0x37,
            _RESERVED_38 = 0x38,
            _RESERVED_39 = 0x39,
            _RESERVED_3a = 0x3a,
            _RESERVED_3b = 0x3b,
            _RESERVED_3c = 0x3c,
            _RESERVED_3d = 0x3d,
            _RESERVED_3e = 0x3e,
            _RESERVED_3f = 0x3f,
            _RESERVED_40 = 0x40,
            _RESERVED_41 = 0x41,
            #[doc = "Sets the power reduction mode."]
            SPRM = 0x42,
            #[doc = "Clears the power reduction mode."]
            CPRM = 0x43,
            #[doc = "Page Buffer Clear - Clears the page buffer."]
            PBC = 0x44,
            _RESERVED_45 = 0x45,
            #[doc = "Invalidate all cache lines."]
            INVALL = 0x46,
            _RESERVED_47 = 0x47,
            _RESERVED_48 = 0x48,
            _RESERVED_49 = 0x49,
            _RESERVED_4a = 0x4a,
            #[doc = "Set DAL=0"]
            SDAL0 = 0x4b,
            #[doc = "Set DAL=1"]
            SDAL1 = 0x4c,
            _RESERVED_4d = 0x4d,
            _RESERVED_4e = 0x4e,
            _RESERVED_4f = 0x4f,
            _RESERVED_50 = 0x50,
            _RESERVED_51 = 0x51,
            _RESERVED_52 = 0x52,
            _RESERVED_53 = 0x53,
            _RESERVED_54 = 0x54,
            _RESERVED_55 = 0x55,
            _RESERVED_56 = 0x56,
            _RESERVED_57 = 0x57,
            _RESERVED_58 = 0x58,
            _RESERVED_59 = 0x59,
            _RESERVED_5a = 0x5a,
            _RESERVED_5b = 0x5b,
            _RESERVED_5c = 0x5c,
            _RESERVED_5d = 0x5d,
            _RESERVED_5e = 0x5e,
            _RESERVED_5f = 0x5f,
            _RESERVED_60 = 0x60,
            _RESERVED_61 = 0x61,
            _RESERVED_62 = 0x62,
            _RESERVED_63 = 0x63,
            _RESERVED_64 = 0x64,
            _RESERVED_65 = 0x65,
            _RESERVED_66 = 0x66,
            _RESERVED_67 = 0x67,
            _RESERVED_68 = 0x68,
            _RESERVED_69 = 0x69,
            _RESERVED_6a = 0x6a,
            _RESERVED_6b = 0x6b,
            _RESERVED_6c = 0x6c,
            _RESERVED_6d = 0x6d,
            _RESERVED_6e = 0x6e,
            _RESERVED_6f = 0x6f,
            _RESERVED_70 = 0x70,
            _RESERVED_71 = 0x71,
            _RESERVED_72 = 0x72,
            _RESERVED_73 = 0x73,
            _RESERVED_74 = 0x74,
            _RESERVED_75 = 0x75,
            _RESERVED_76 = 0x76,
            _RESERVED_77 = 0x77,
            _RESERVED_78 = 0x78,
            _RESERVED_79 = 0x79,
            _RESERVED_7a = 0x7a,
            _RESERVED_7b = 0x7b,
            _RESERVED_7c = 0x7c,
            _RESERVED_7d = 0x7d,
            _RESERVED_7e = 0x7e,
            _RESERVED_7f = 0x7f,
        }
        impl Cmd {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Cmd {
                unsafe { core::mem::transmute(val & 0x7f) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Cmd {
            #[inline(always)]
            fn from(val: u8) -> Cmd {
                Cmd::from_bits(val)
            }
        }
        impl From<Cmd> for u8 {
            #[inline(always)]
            fn from(val: Cmd) -> u8 {
                Cmd::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Cmdex(pub u8);
        impl Cmdex {
            #[doc = "Execution Key"]
            pub const KEY: Self = Self(0xa5);
        }
        impl Cmdex {
            pub const fn from_bits(val: u8) -> Cmdex {
                Self(val & 0xff)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl From<u8> for Cmdex {
            #[inline(always)]
            fn from(val: u8) -> Cmdex {
                Cmdex::from_bits(val)
            }
        }
        impl From<Cmdex> for u8 {
            #[inline(always)]
            fn from(val: Cmdex) -> u8 {
                Cmdex::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Key(pub u8);
        impl Key {
            #[doc = "Write Key"]
            pub const KEY: Self = Self(0xa5);
        }
        impl Key {
            pub const fn from_bits(val: u8) -> Key {
                Self(val & 0xff)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl From<u8> for Key {
            #[inline(always)]
            fn from(val: u8) -> Key {
                Key::from_bits(val)
            }
        }
        impl From<Key> for u8 {
            #[inline(always)]
            fn from(val: Key) -> u8 {
                Key::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Nslkey(pub u8);
        impl Nslkey {
            #[doc = "Write Key"]
            pub const KEY: Self = Self(0xa5);
        }
        impl Nslkey {
            pub const fn from_bits(val: u8) -> Nslkey {
                Self(val & 0xff)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl From<u8> for Nslkey {
            #[inline(always)]
            fn from(val: u8) -> Nslkey {
                Nslkey::from_bits(val)
            }
        }
        impl From<Nslkey> for u8 {
            #[inline(always)]
            fn from(val: Nslkey) -> u8 {
                Nslkey::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Psz {
            #[doc = "8 bytes"]
            _8 = 0x0,
            #[doc = "16 bytes"]
            _16 = 0x01,
            #[doc = "32 bytes"]
            _32 = 0x02,
            #[doc = "64 bytes"]
            _64 = 0x03,
            #[doc = "128 bytes"]
            _128 = 0x04,
            #[doc = "256 bytes"]
            _256 = 0x05,
            #[doc = "512 bytes"]
            _512 = 0x06,
            #[doc = "1024 bytes"]
            _1024 = 0x07,
        }
        impl Psz {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Psz {
                unsafe { core::mem::transmute(val & 0x07) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Psz {
            #[inline(always)]
            fn from(val: u8) -> Psz {
                Psz::from_bits(val)
            }
        }
        impl From<Psz> for u8 {
            #[inline(always)]
            fn from(val: Psz) -> u8 {
                Psz::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Readmode {
            #[doc = "The NVM Controller (cache system) does not insert wait states on a cache miss. Gives the best system performance."]
            NO_MISS_PENALTY = 0x0,
            #[doc = "Reduces power consumption of the cache system, but inserts a wait state each time there is a cache miss. This mode may not be relevant if CPU performance is required, as the application will be stalled and may lead to increase run time."]
            LOW_POWER = 0x01,
            #[doc = "The cache system ensures that a cache hit or miss takes the same amount of time, determined by the number of programmed flash wait states. This mode can be used for real-time applications that require deterministic execution timings."]
            DETERMINISTIC = 0x02,
            _RESERVED_3 = 0x03,
        }
        impl Readmode {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Readmode {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Readmode {
            #[inline(always)]
            fn from(val: u8) -> Readmode {
                Readmode::from_bits(val)
            }
        }
        impl From<Readmode> for u8 {
            #[inline(always)]
            fn from(val: Readmode) -> u8 {
                Readmode::to_bits(val)
            }
        }
        #[repr(u8)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Sleepprm {
            #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode upon first access."]
            WAKEONACCESS = 0x0,
            #[doc = "NVM block enters low-power mode when entering sleep.NVM block exits low-power mode when exiting sleep."]
            WAKEUPINSTANT = 0x01,
            _RESERVED_2 = 0x02,
            #[doc = "Auto power reduction disabled."]
            DISABLED = 0x03,
        }
        impl Sleepprm {
            #[inline(always)]
            pub const fn from_bits(val: u8) -> Sleepprm {
                unsafe { core::mem::transmute(val & 0x03) }
            }
            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                unsafe { core::mem::transmute(self) }
            }
        }
        impl From<u8> for Sleepprm {
            #[inline(always)]
            fn from(val: u8) -> Sleepprm {
                Sleepprm::from_bits(val)
            }
        }
        impl From<Sleepprm> for u8 {
            #[inline(always)]
            fn from(val: Sleepprm) -> u8 {
                Sleepprm::to_bits(val)
            }
        }
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Slkey(pub u8);
        impl Slkey {
            #[doc = "Write Key"]
            pub const KEY: Self = Self(0xa5);
        }
        impl Slkey {
            pub const fn from_bits(val: u8) -> Slkey {
                Self(val & 0xff)
            }
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }
        impl From<u8> for Slkey {
            #[inline(always)]
            fn from(val: u8) -> Slkey {
                Slkey::from_bits(val)
            }
        }
        impl From<Slkey> for u8 {
            #[inline(always)]
            fn from(val: Slkey) -> u8 {
                Slkey::to_bits(val)
            }
        }
    }
}
