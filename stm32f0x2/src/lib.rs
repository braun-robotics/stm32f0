#![cfg_attr(feature = "rt", feature(global_asm))]
#![cfg_attr(feature = "rt", feature(macro_reexport))]
#![cfg_attr(feature = "rt", feature(used))]
#![doc = "Peripheral access API for STM32F0X2 microcontrollers (generated using svd2rust v0.11.4)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.11.4/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![feature(const_fn)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[macro_reexport(default_handler, exception)]
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::ops::Deref;
use bare_metal::Peripheral;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
pub use interrupt::Interrupt;
#[doc(hidden)]
pub mod interrupt;
pub use cortex_m::peripheral::CPUID;
pub use cortex_m::peripheral::DCB;
pub use cortex_m::peripheral::DWT;
pub use cortex_m::peripheral::FPB;
pub use cortex_m::peripheral::FPU;
pub use cortex_m::peripheral::ITM;
pub use cortex_m::peripheral::MPU;
pub use cortex_m::peripheral::NVIC;
pub use cortex_m::peripheral::SCB;
pub use cortex_m::peripheral::SYST;
pub use cortex_m::peripheral::TPIU;
#[doc = "cyclic redundancy check calculation unit"]
pub const CRC: Peripheral<CRC> = unsafe { Peripheral::new(1073885184) };
#[doc = "cyclic redundancy check calculation unit"]
pub mod crc;
#[doc = "cyclic redundancy check calculation unit"]
pub struct CRC {
    register_block: crc::RegisterBlock,
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    fn deref(&self) -> &crc::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General-purpose I/Os"]
pub const GPIOF: Peripheral<GPIOF> = unsafe { Peripheral::new(1207964672) };
#[doc = "General-purpose I/Os"]
pub mod gpiof;
#[doc = "General-purpose I/Os"]
pub struct GPIOF {
    register_block: gpiof::RegisterBlock,
}
impl Deref for GPIOF {
    type Target = gpiof::RegisterBlock;
    fn deref(&self) -> &gpiof::RegisterBlock {
        &self.register_block
    }
}
#[doc = "GPIOD"]
pub const GPIOD: Peripheral<GPIOD> = unsafe { Peripheral::new(1207962624) };
#[doc = r" Register block"]
pub struct GPIOD {
    register_block: gpiof::RegisterBlock,
}
impl Deref for GPIOD {
    type Target = gpiof::RegisterBlock;
    fn deref(&self) -> &gpiof::RegisterBlock {
        &self.register_block
    }
}
#[doc = "GPIOC"]
pub const GPIOC: Peripheral<GPIOC> = unsafe { Peripheral::new(1207961600) };
#[doc = r" Register block"]
pub struct GPIOC {
    register_block: gpiof::RegisterBlock,
}
impl Deref for GPIOC {
    type Target = gpiof::RegisterBlock;
    fn deref(&self) -> &gpiof::RegisterBlock {
        &self.register_block
    }
}
#[doc = "GPIOB"]
pub const GPIOB: Peripheral<GPIOB> = unsafe { Peripheral::new(1207960576) };
#[doc = r" Register block"]
pub struct GPIOB {
    register_block: gpiof::RegisterBlock,
}
impl Deref for GPIOB {
    type Target = gpiof::RegisterBlock;
    fn deref(&self) -> &gpiof::RegisterBlock {
        &self.register_block
    }
}
#[doc = "GPIOE"]
pub const GPIOE: Peripheral<GPIOE> = unsafe { Peripheral::new(1207963648) };
#[doc = r" Register block"]
pub struct GPIOE {
    register_block: gpiof::RegisterBlock,
}
impl Deref for GPIOE {
    type Target = gpiof::RegisterBlock;
    fn deref(&self) -> &gpiof::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General-purpose I/Os"]
pub const GPIOA: Peripheral<GPIOA> = unsafe { Peripheral::new(1207959552) };
#[doc = "General-purpose I/Os"]
pub mod gpioa;
#[doc = "General-purpose I/Os"]
pub struct GPIOA {
    register_block: gpioa::RegisterBlock,
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Serial peripheral interface"]
pub const SPI1: Peripheral<SPI1> = unsafe { Peripheral::new(1073819648) };
#[doc = "Serial peripheral interface"]
pub mod spi1;
#[doc = "Serial peripheral interface"]
pub struct SPI1 {
    register_block: spi1::RegisterBlock,
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "SPI2"]
pub const SPI2: Peripheral<SPI2> = unsafe { Peripheral::new(1073756160) };
#[doc = r" Register block"]
pub struct SPI2 {
    register_block: spi1::RegisterBlock,
}
impl Deref for SPI2 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Digital-to-analog converter"]
pub const DAC: Peripheral<DAC> = unsafe { Peripheral::new(1073771520) };
#[doc = "Digital-to-analog converter"]
pub mod dac;
#[doc = "Digital-to-analog converter"]
pub struct DAC {
    register_block: dac::RegisterBlock,
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    fn deref(&self) -> &dac::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Power control"]
pub const PWR: Peripheral<PWR> = unsafe { Peripheral::new(1073770496) };
#[doc = "Power control"]
pub mod pwr;
#[doc = "Power control"]
pub struct PWR {
    register_block: pwr::RegisterBlock,
}
impl Deref for PWR {
    type Target = pwr::RegisterBlock;
    fn deref(&self) -> &pwr::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Inter-integrated circuit"]
pub const I2C1: Peripheral<I2C1> = unsafe { Peripheral::new(1073763328) };
#[doc = "Inter-integrated circuit"]
pub mod i2c1;
#[doc = "Inter-integrated circuit"]
pub struct I2C1 {
    register_block: i2c1::RegisterBlock,
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "I2C2"]
pub const I2C2: Peripheral<I2C2> = unsafe { Peripheral::new(1073764352) };
#[doc = r" Register block"]
pub struct I2C2 {
    register_block: i2c1::RegisterBlock,
}
impl Deref for I2C2 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Independent watchdog"]
pub const IWDG: Peripheral<IWDG> = unsafe { Peripheral::new(1073754112) };
#[doc = "Independent watchdog"]
pub mod iwdg;
#[doc = "Independent watchdog"]
pub struct IWDG {
    register_block: iwdg::RegisterBlock,
}
impl Deref for IWDG {
    type Target = iwdg::RegisterBlock;
    fn deref(&self) -> &iwdg::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Window watchdog"]
pub const WWDG: Peripheral<WWDG> = unsafe { Peripheral::new(1073753088) };
#[doc = "Window watchdog"]
pub mod wwdg;
#[doc = "Window watchdog"]
pub struct WWDG {
    register_block: wwdg::RegisterBlock,
}
impl Deref for WWDG {
    type Target = wwdg::RegisterBlock;
    fn deref(&self) -> &wwdg::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Advanced-timers"]
pub const TIM1: Peripheral<TIM1> = unsafe { Peripheral::new(1073818624) };
#[doc = "Advanced-timers"]
pub mod tim1;
#[doc = "Advanced-timers"]
pub struct TIM1 {
    register_block: tim1::RegisterBlock,
}
impl Deref for TIM1 {
    type Target = tim1::RegisterBlock;
    fn deref(&self) -> &tim1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General-purpose-timers"]
pub const TIM2: Peripheral<TIM2> = unsafe { Peripheral::new(1073741824) };
#[doc = "General-purpose-timers"]
pub mod tim2;
#[doc = "General-purpose-timers"]
pub struct TIM2 {
    register_block: tim2::RegisterBlock,
}
impl Deref for TIM2 {
    type Target = tim2::RegisterBlock;
    fn deref(&self) -> &tim2::RegisterBlock {
        &self.register_block
    }
}
#[doc = "TIM3"]
pub const TIM3: Peripheral<TIM3> = unsafe { Peripheral::new(1073742848) };
#[doc = r" Register block"]
pub struct TIM3 {
    register_block: tim2::RegisterBlock,
}
impl Deref for TIM3 {
    type Target = tim2::RegisterBlock;
    fn deref(&self) -> &tim2::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General-purpose-timers"]
pub const TIM14: Peripheral<TIM14> = unsafe { Peripheral::new(1073750016) };
#[doc = "General-purpose-timers"]
pub mod tim14;
#[doc = "General-purpose-timers"]
pub struct TIM14 {
    register_block: tim14::RegisterBlock,
}
impl Deref for TIM14 {
    type Target = tim14::RegisterBlock;
    fn deref(&self) -> &tim14::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Basic-timers"]
pub const TIM6: Peripheral<TIM6> = unsafe { Peripheral::new(1073745920) };
#[doc = "Basic-timers"]
pub mod tim6;
#[doc = "Basic-timers"]
pub struct TIM6 {
    register_block: tim6::RegisterBlock,
}
impl Deref for TIM6 {
    type Target = tim6::RegisterBlock;
    fn deref(&self) -> &tim6::RegisterBlock {
        &self.register_block
    }
}
#[doc = "TIM7"]
pub const TIM7: Peripheral<TIM7> = unsafe { Peripheral::new(1073746944) };
#[doc = r" Register block"]
pub struct TIM7 {
    register_block: tim6::RegisterBlock,
}
impl Deref for TIM7 {
    type Target = tim6::RegisterBlock;
    fn deref(&self) -> &tim6::RegisterBlock {
        &self.register_block
    }
}
#[doc = "External interrupt/event controller"]
pub const EXTI: Peripheral<EXTI> = unsafe { Peripheral::new(1073808384) };
#[doc = "External interrupt/event controller"]
pub mod exti;
#[doc = "External interrupt/event controller"]
pub struct EXTI {
    register_block: exti::RegisterBlock,
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    fn deref(&self) -> &exti::RegisterBlock {
        &self.register_block
    }
}
#[doc = "DMA controller"]
pub const DMA1: Peripheral<DMA1> = unsafe { Peripheral::new(1073872896) };
#[doc = "DMA controller"]
pub mod dma1;
#[doc = "DMA controller"]
pub struct DMA1 {
    register_block: dma1::RegisterBlock,
}
impl Deref for DMA1 {
    type Target = dma1::RegisterBlock;
    fn deref(&self) -> &dma1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Reset and clock control"]
pub const RCC: Peripheral<RCC> = unsafe { Peripheral::new(1073876992) };
#[doc = "Reset and clock control"]
pub mod rcc;
#[doc = "Reset and clock control"]
pub struct RCC {
    register_block: rcc::RegisterBlock,
}
impl Deref for RCC {
    type Target = rcc::RegisterBlock;
    fn deref(&self) -> &rcc::RegisterBlock {
        &self.register_block
    }
}
#[doc = "System configuration controller"]
pub const SYSCFG_COMP: Peripheral<SYSCFG_COMP> = unsafe { Peripheral::new(1073807360) };
#[doc = "System configuration controller"]
pub mod syscfg_comp;
#[doc = "System configuration controller"]
pub struct SYSCFG_COMP {
    register_block: syscfg_comp::RegisterBlock,
}
impl Deref for SYSCFG_COMP {
    type Target = syscfg_comp::RegisterBlock;
    fn deref(&self) -> &syscfg_comp::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Analog-to-digital converter"]
pub const ADC: Peripheral<ADC> = unsafe { Peripheral::new(1073816576) };
#[doc = "Analog-to-digital converter"]
pub mod adc;
#[doc = "Analog-to-digital converter"]
pub struct ADC {
    register_block: adc::RegisterBlock,
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    fn deref(&self) -> &adc::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub const USART1: Peripheral<USART1> = unsafe { Peripheral::new(1073821696) };
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod usart1;
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART1 {
    register_block: usart1::RegisterBlock,
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "USART2"]
pub const USART2: Peripheral<USART2> = unsafe { Peripheral::new(1073759232) };
#[doc = r" Register block"]
pub struct USART2 {
    register_block: usart1::RegisterBlock,
}
impl Deref for USART2 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "USART3"]
pub const USART3: Peripheral<USART3> = unsafe { Peripheral::new(1073760256) };
#[doc = r" Register block"]
pub struct USART3 {
    register_block: usart1::RegisterBlock,
}
impl Deref for USART3 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "USART4"]
pub const USART4: Peripheral<USART4> = unsafe { Peripheral::new(1073761280) };
#[doc = r" Register block"]
pub struct USART4 {
    register_block: usart1::RegisterBlock,
}
impl Deref for USART4 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Real-time clock"]
pub const RTC: Peripheral<RTC> = unsafe { Peripheral::new(1073752064) };
#[doc = "Real-time clock"]
pub mod rtc;
#[doc = "Real-time clock"]
pub struct RTC {
    register_block: rtc::RegisterBlock,
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General-purpose-timers"]
pub const TIM15: Peripheral<TIM15> = unsafe { Peripheral::new(1073823744) };
#[doc = "General-purpose-timers"]
pub mod tim15;
#[doc = "General-purpose-timers"]
pub struct TIM15 {
    register_block: tim15::RegisterBlock,
}
impl Deref for TIM15 {
    type Target = tim15::RegisterBlock;
    fn deref(&self) -> &tim15::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General-purpose-timers"]
pub const TIM16: Peripheral<TIM16> = unsafe { Peripheral::new(1073824768) };
#[doc = "General-purpose-timers"]
pub mod tim16;
#[doc = "General-purpose-timers"]
pub struct TIM16 {
    register_block: tim16::RegisterBlock,
}
impl Deref for TIM16 {
    type Target = tim16::RegisterBlock;
    fn deref(&self) -> &tim16::RegisterBlock {
        &self.register_block
    }
}
#[doc = "TIM17"]
pub const TIM17: Peripheral<TIM17> = unsafe { Peripheral::new(1073825792) };
#[doc = r" Register block"]
pub struct TIM17 {
    register_block: tim16::RegisterBlock,
}
impl Deref for TIM17 {
    type Target = tim16::RegisterBlock;
    fn deref(&self) -> &tim16::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Touch sensing controller"]
pub const TSC: Peripheral<TSC> = unsafe { Peripheral::new(1073889280) };
#[doc = "Touch sensing controller"]
pub mod tsc;
#[doc = "Touch sensing controller"]
pub struct TSC {
    register_block: tsc::RegisterBlock,
}
impl Deref for TSC {
    type Target = tsc::RegisterBlock;
    fn deref(&self) -> &tsc::RegisterBlock {
        &self.register_block
    }
}
#[doc = "HDMI-CEC controller"]
pub const CEC: Peripheral<CEC> = unsafe { Peripheral::new(1073772544) };
#[doc = "HDMI-CEC controller"]
pub mod cec;
#[doc = "HDMI-CEC controller"]
pub struct CEC {
    register_block: cec::RegisterBlock,
}
impl Deref for CEC {
    type Target = cec::RegisterBlock;
    fn deref(&self) -> &cec::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Flash"]
pub const FLASH: Peripheral<FLASH> = unsafe { Peripheral::new(1073881088) };
#[doc = "Flash"]
pub mod flash;
#[doc = "Flash"]
pub struct FLASH {
    register_block: flash::RegisterBlock,
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    fn deref(&self) -> &flash::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Debug support"]
pub const DBGMCU: Peripheral<DBGMCU> = unsafe { Peripheral::new(1073829888) };
#[doc = "Debug support"]
pub mod dbgmcu;
#[doc = "Debug support"]
pub struct DBGMCU {
    register_block: dbgmcu::RegisterBlock,
}
impl Deref for DBGMCU {
    type Target = dbgmcu::RegisterBlock;
    fn deref(&self) -> &dbgmcu::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal serial bus full-speed device interface"]
pub const USB: Peripheral<USB> = unsafe { Peripheral::new(1073765376) };
#[doc = "Universal serial bus full-speed device interface"]
pub mod usb;
#[doc = "Universal serial bus full-speed device interface"]
pub struct USB {
    register_block: usb::RegisterBlock,
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    fn deref(&self) -> &usb::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Clock recovery system"]
pub const CRS: Peripheral<CRS> = unsafe { Peripheral::new(1073769472) };
#[doc = "Clock recovery system"]
pub mod crs;
#[doc = "Clock recovery system"]
pub struct CRS {
    register_block: crs::RegisterBlock,
}
impl Deref for CRS {
    type Target = crs::RegisterBlock;
    fn deref(&self) -> &crs::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Controller area network"]
pub const CAN: Peripheral<CAN> = unsafe { Peripheral::new(1073767424) };
#[doc = "Controller area network"]
pub mod can;
#[doc = "Controller area network"]
pub struct CAN {
    register_block: can::RegisterBlock,
}
impl Deref for CAN {
    type Target = can::RegisterBlock;
    fn deref(&self) -> &can::RegisterBlock {
        &self.register_block
    }
}
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals<'a> {
    #[doc = "CPUID"] pub CPUID: &'a CPUID,
    #[doc = "DCB"] pub DCB: &'a DCB,
    #[doc = "DWT"] pub DWT: &'a DWT,
    #[doc = "FPB"] pub FPB: &'a FPB,
    #[doc = "FPU"] pub FPU: &'a FPU,
    #[doc = "ITM"] pub ITM: &'a ITM,
    #[doc = "MPU"] pub MPU: &'a MPU,
    #[doc = "NVIC"] pub NVIC: &'a NVIC,
    #[doc = "SCB"] pub SCB: &'a SCB,
    #[doc = "SYST"] pub SYST: &'a SYST,
    #[doc = "TPIU"] pub TPIU: &'a TPIU,
    #[doc = "CRC"] pub CRC: &'a CRC,
    #[doc = "GPIOF"] pub GPIOF: &'a GPIOF,
    #[doc = "GPIOD"] pub GPIOD: &'a GPIOD,
    #[doc = "GPIOC"] pub GPIOC: &'a GPIOC,
    #[doc = "GPIOB"] pub GPIOB: &'a GPIOB,
    #[doc = "GPIOE"] pub GPIOE: &'a GPIOE,
    #[doc = "GPIOA"] pub GPIOA: &'a GPIOA,
    #[doc = "SPI1"] pub SPI1: &'a SPI1,
    #[doc = "SPI2"] pub SPI2: &'a SPI2,
    #[doc = "DAC"] pub DAC: &'a DAC,
    #[doc = "PWR"] pub PWR: &'a PWR,
    #[doc = "I2C1"] pub I2C1: &'a I2C1,
    #[doc = "I2C2"] pub I2C2: &'a I2C2,
    #[doc = "IWDG"] pub IWDG: &'a IWDG,
    #[doc = "WWDG"] pub WWDG: &'a WWDG,
    #[doc = "TIM1"] pub TIM1: &'a TIM1,
    #[doc = "TIM2"] pub TIM2: &'a TIM2,
    #[doc = "TIM3"] pub TIM3: &'a TIM3,
    #[doc = "TIM14"] pub TIM14: &'a TIM14,
    #[doc = "TIM6"] pub TIM6: &'a TIM6,
    #[doc = "TIM7"] pub TIM7: &'a TIM7,
    #[doc = "EXTI"] pub EXTI: &'a EXTI,
    #[doc = "DMA1"] pub DMA1: &'a DMA1,
    #[doc = "RCC"] pub RCC: &'a RCC,
    #[doc = "SYSCFG_COMP"] pub SYSCFG_COMP: &'a SYSCFG_COMP,
    #[doc = "ADC"] pub ADC: &'a ADC,
    #[doc = "USART1"] pub USART1: &'a USART1,
    #[doc = "USART2"] pub USART2: &'a USART2,
    #[doc = "USART3"] pub USART3: &'a USART3,
    #[doc = "USART4"] pub USART4: &'a USART4,
    #[doc = "RTC"] pub RTC: &'a RTC,
    #[doc = "TIM15"] pub TIM15: &'a TIM15,
    #[doc = "TIM16"] pub TIM16: &'a TIM16,
    #[doc = "TIM17"] pub TIM17: &'a TIM17,
    #[doc = "TSC"] pub TSC: &'a TSC,
    #[doc = "CEC"] pub CEC: &'a CEC,
    #[doc = "FLASH"] pub FLASH: &'a FLASH,
    #[doc = "DBGMCU"] pub DBGMCU: &'a DBGMCU,
    #[doc = "USB"] pub USB: &'a USB,
    #[doc = "CRS"] pub CRS: &'a CRS,
    #[doc = "CAN"] pub CAN: &'a CAN,
}
impl<'a> Peripherals<'a> {
    #[doc = r" Grants access to all the peripherals"]
    pub unsafe fn all() -> Self {
        Peripherals {
            CPUID: &*CPUID.get(),
            DCB: &*DCB.get(),
            DWT: &*DWT.get(),
            FPB: &*FPB.get(),
            FPU: &*FPU.get(),
            ITM: &*ITM.get(),
            MPU: &*MPU.get(),
            NVIC: &*NVIC.get(),
            SCB: &*SCB.get(),
            SYST: &*SYST.get(),
            TPIU: &*TPIU.get(),
            CRC: &*CRC.get(),
            GPIOF: &*GPIOF.get(),
            GPIOD: &*GPIOD.get(),
            GPIOC: &*GPIOC.get(),
            GPIOB: &*GPIOB.get(),
            GPIOE: &*GPIOE.get(),
            GPIOA: &*GPIOA.get(),
            SPI1: &*SPI1.get(),
            SPI2: &*SPI2.get(),
            DAC: &*DAC.get(),
            PWR: &*PWR.get(),
            I2C1: &*I2C1.get(),
            I2C2: &*I2C2.get(),
            IWDG: &*IWDG.get(),
            WWDG: &*WWDG.get(),
            TIM1: &*TIM1.get(),
            TIM2: &*TIM2.get(),
            TIM3: &*TIM3.get(),
            TIM14: &*TIM14.get(),
            TIM6: &*TIM6.get(),
            TIM7: &*TIM7.get(),
            EXTI: &*EXTI.get(),
            DMA1: &*DMA1.get(),
            RCC: &*RCC.get(),
            SYSCFG_COMP: &*SYSCFG_COMP.get(),
            ADC: &*ADC.get(),
            USART1: &*USART1.get(),
            USART2: &*USART2.get(),
            USART3: &*USART3.get(),
            USART4: &*USART4.get(),
            RTC: &*RTC.get(),
            TIM15: &*TIM15.get(),
            TIM16: &*TIM16.get(),
            TIM17: &*TIM17.get(),
            TSC: &*TSC.get(),
            CEC: &*CEC.get(),
            FLASH: &*FLASH.get(),
            DBGMCU: &*DBGMCU.get(),
            USB: &*USB.get(),
            CRS: &*CRS.get(),
            CAN: &*CAN.get(),
        }
    }
}
