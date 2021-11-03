#![allow(non_camel_case_types)]

pub mod command;
pub mod register;

use core::fmt::Debug;

use embedded_hal::digital::v2::StatefulOutputPin;
use embedded_hal::spi::FullDuplex;
use nb::block;

use self::command::SpiCommand;
use self::register::*;
use core::convert::Infallible;


pub struct Drv8301<'a, SPI, NSCS, EN_GATE>
where
    SPI: FullDuplex<u16>,
    NSCS: StatefulOutputPin<Error = Infallible>,
    EN_GATE: StatefulOutputPin<Error = Infallible>,
{
    spi: &'a mut SPI,
    nscs: NSCS,
    en_gate: EN_GATE,
}

impl<'a, SPI, NSCS, EN_GATE> Drv8301<'a, SPI, NSCS, EN_GATE>
where
    SPI: FullDuplex<u16>,
    NSCS: StatefulOutputPin<Error = Infallible>,
    EN_GATE: StatefulOutputPin<Error = Infallible>,
{
    pub fn new(spi: &mut SPI, nscs: NSCS, en_gate: EN_GATE) -> Drv8301<SPI, NSCS, EN_GATE> {
        Drv8301 { spi, nscs, en_gate }
    }

    pub fn read<REG>(&mut self) -> Result<REG, SPI::Error>
    where
        REG: Register,
    {
        self.exec::<REG>(SpiCommand::read())?;
        let data = self.exec::<REG>(SpiCommand::read())?;

        Ok(REG::parse(data))
    }

    pub fn write<REG, F>(&mut self, f: F) -> Result<REG, SPI::Error>
    where
        REG: Register,
        F: Fn(REG) -> REG,
    {
        let val: REG = self.read()?;
        let update_reg = f(val);
        
        let ret = self.exec::<REG>(SpiCommand::write(update_reg.data()))?;

        Ok(REG::parse(ret))
    }

    fn exec<REG>(&mut self, cmd: SpiCommand<REG>) -> Result<u16, SPI::Error>
    where
        REG: Register,
    {
        let data: u16 = cmd.into();

        self.nscs.set_low().unwrap();
        // Give the drv at least 50ns to prepare
        cortex_m::asm::delay(8u32);

        block!(self.spi.send(data))?;
        let ret = block!(self.spi.read());

        // Make sure scs is high for at least 500ns between frames
        self.nscs.set_high().unwrap();
        cortex_m::asm::delay(32u32);

        ret
    }

    pub fn init(&mut self) -> Result<(), ()>
    where
    <SPI as FullDuplex<u16>>::Error: Debug,
    {
        // reset EN_GATE
        self.reset_en_gate();
        cortex_m::asm::delay(8u32);

        // set NSCS
        self.nscs.set_low().unwrap();
        cortex_m::asm::delay(1000000u32);
        
        // set initial value for ControlRegister1
        // gate_current: 1.7A -> 0
        // gete_reset: Normal -> 0
        // pwm_mode: 6 PWM -> PwmMode::Six
        // ocp_mode: Current limit -> 0
        // oc_adj_set: 0.358 -> 15
        self.write(|w: ControlRegister1| {
            w
            .gate_current(0u8)
            .gate_reset(Flag::Disable)
            .pwm_mode(PwmMode::Six)
            .ocp_mode(0u8)
            .oc_adj_set(15u8)
        }).unwrap();

        // set initial value for ControlRegister2
        // octw_mode: Report both OT and OC -> 0
        // shunt_gain: 80 V/V -> 3
        // dc_dal_ch1: 1 -> Flag::Enable
        // dc_cal_ch2: 1 -> Flag::Enable
        // oc_toff: Cycle by cycle -> 0
        self.write(|w: ControlRegister2| {
            w
            .octw_mode(0u8)
            .shunt_gain(3u8)
            .dc_cal_ch1(Flag::Disable)
            .dc_cal_ch2(Flag::Disable)
            .oc_toff(Flag::Disable)
        }).unwrap();

        // wait
        cortex_m::asm::delay(8u32);
        
        Ok(())
    }

    fn reset_en_gate(&mut self) {
        self.en_gate.set_high().unwrap();
        cortex_m::asm::delay(8u32);

        self.en_gate.set_low().unwrap();
        cortex_m::asm::delay(8u32);

        self.en_gate.set_high().unwrap();
        cortex_m::asm::delay(8u32);
    }

}
