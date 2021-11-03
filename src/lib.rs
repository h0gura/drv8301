//! # Driver for the DRV8301 Brushless DC Motor Driver IC
//!
//! ## Example
//! ```
//! let mut drv8301 = Drv8301::new(&mut spi, ncs, en_gate);
//! drv8301.init().unwrap();
//! 
//! let read_data = drv8301.read::<StatusRegister1>().unwrap();
//! hprintln!("StsReg1: {:b}", read_data.data() & DATA_MASK);
//! let read_data = drv8301.read::<StatusRegister2>().unwrap();
//! hprintln!("StsReg2: {:b}", read_data.data() & DATA_MASK);
//! let read_data = drv8301.read::<ControlRegister1>().unwrap();
//! hprintln!("CtrReg1: {:b}", read_data.data() & DATA_MASK);
//! let read_data = drv8301.read::<ControlRegister2>().unwrap();
//! hprintln!("CtrReg2: {:b}", read_data.data() & DATA_MASK);
//! 
//! drv8301.write(|w: ControlRegister1| {
//!     w.gate_reset(Flag::Enabled)
//!     .gate_current(2u8)
//!     .pwm_mode(Flag::Enabled)
//! }).unwrap();
//! ```

#![no_std]

pub mod drv8301;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
