use enum_primitive_derive_nostd::Primitive;
use num_traits::{FromPrimitive, ToPrimitive};

#[cfg(feature = "use-defmt")]
use defmt::Format;

pub trait Register {
    fn addr() -> u8;
    fn parse(reg: u16) -> Self;
    fn data(&self) -> u16;
}

#[derive(Copy, Clone, Primitive, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "use-defmt", derive(Format))]
pub enum Flag {
    Enable = 1,
    Disable = 0,
}

impl Flag {
    pub fn enabled(&self) -> bool {
        matches!(self, Flag::Enable)
    }
}

#[derive(Copy, Clone, Primitive, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "use-defmt", derive(Format))]
pub enum PwmMode {
    Six = 0,
    Three = 1,
}

#[derive(Copy, Clone, Primitive, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "use-defmt", derive(Format))]
pub enum OcpMode {
    CurrentLimit = 0,
    OcLatchShutdown = 1,
    ReportOnly = 2,
    OcDisabled = 3,
}

macro_rules! register {
    (struct $name: ident [$addr: expr] { $($var: ident: $kind: ty [$size: expr, $offset: expr]),+ }) => {
        #[derive(Copy, Clone, Debug)]
        #[cfg_attr(feature = "use-defmt", derive(Format))]
        pub struct $name {
            pub bits: u16,
            $(pub $var: $kind,)*
        }

        impl $name {
        $(
            pub fn $var(mut self, $var: $kind) -> Self {
                self.$var = $var;
                self
            }
        )*
        }

        impl Register for $name {
            fn addr() -> u8 {
                return $addr;
            }

            fn parse(reg: u16) -> Self {
                $name {
                    bits: reg,
                    $($var: <$kind>::from_u16((reg & ($size << $offset)) >> $offset).unwrap(),)*
                }
            }

            fn data(&self) -> u16 {
                let mut data = self.bits;

                $(data = (data & !($size << $offset)) | (self.$var.to_u16().unwrap() << $offset);)*

                data
            }
        }
    };
}

register!(struct StatusRegister1 [0x0]{
    fault: Flag [0b1, 10],
    gvdd_uv: Flag [0b1, 9],
    pvdd_uv: Flag [0b1, 8],
    otsd: Flag [0b1, 7],
    otw: Flag [0b1, 6],
    fetha_oc: Flag [0b1, 5],
    fetla_oc: Flag [0b1, 4],
    fethb_oc: Flag [0b1, 3],
    fetlb_oc: Flag [0b1, 2],
    fethc_oc: Flag [0b1, 1],
    fetlc_oc: Flag [0b1, 0]
});

register!(struct StatusRegister2 [0x1]{
    temp1: Flag [0b1, 10],
    temp2: Flag [0b1, 9],
    temp3: Flag [0b1, 8],
    gvdd_ov: Flag [0b1, 7],
    device_id: u8 [0b1111, 0]
});

register!(struct ControlRegister1 [0x2]{
    oc_adj_set: u8 [0b11111, 6],
    ocp_mode: u8 [0b11, 4],
    pwm_mode: PwmMode [0b1, 3],
    gate_reset: Flag [0b1, 2],
    gate_current: u8 [0b11, 0]
});

register!(struct ControlRegister2 [0x3]{
    oc_toff: Flag [0b1, 6],
    dc_cal_ch2: Flag [0b1, 5],
    dc_cal_ch1: Flag [0b1, 4],
    shunt_gain:  u8 [0b11, 2],
    octw_mode: u8 [0b11, 0]
});

