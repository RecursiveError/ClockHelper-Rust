#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ClockNodes {
    None,
    HSIRC,
    FLITFCLKoutput,
    HSICECDiv,
    HSIRC48,
    HSIRC14,
    ADCoutput,
    LSIRC,
    LSEOSC,
    CECMult,
    CECOutput,
    HSEOSC,
    USBMult,
    USBoutput,
    SysClkSource,
    SysCLKOutput,
    I2SOutput,
    HSERTCDevisor,
    RTCClkSource,
    RTCOutput,
    IWDGOutput,
    MCOMultDivisor,
    MCOMult,
    MCODivider,
    MCOoutput,
    AHBPrescaler,
    AHBOutput,
    HCLKOutput,
    FCLKCortexOutput,
    TimSysPresc,
    TimSysOutput,
    APB1Prescaler,
    APB1Output,
    TimPrescalerAPB1,
    TimPrescOut1,
    I2C1Mult,
    I2C1Output,
    USART1Mult,
    USART1Output,
    USART2Mult,
    USART2Output,
    PLLSource,
    PLLDiv,
    VCO2output,
    PLLMUL,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ClockErrorType {
    Underflow(u32, u32),
    Overflow(u32, u32),
}
#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ClockError {
    err_type: ClockErrorType,
    from: ClockNodes,
    to: ClockNodes,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LSEOSCConf {
    Value(u32),
}

impl LSEOSCConf {
    pub const fn min() -> u32 {
        0
    }

    pub const fn max() -> u32 {
        1000000
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            LSEOSCConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::LSEOSC,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::LSEOSC,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CECMultConf {
    HSICECDiv,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum HSEOSCConf {
    Value(u32),
}

impl HSEOSCConf {
    pub const fn min() -> u32 {
        4000000
    }

    pub const fn max() -> u32 {
        32000000
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            HSEOSCConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::HSEOSC,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::HSEOSC,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USBMultConf {
    PLLMUL,
    HSIRC48,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SysClkSourceConf {
    HSIRC,
    HSIRC48,
    HSEOSC,
    PLLMUL,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum RTCClkSourceConf {
    HSERTCDevisor,
    LSEOSC,
    LSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCOMultDivisorConf {
    DIV1,
    DIV2,
}

impl MCOMultDivisorConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MCOMultDivisorConf::DIV1 => return Ok(1.0),
            MCOMultDivisorConf::DIV2 => return Ok(2.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCOMultConf {
    MCOMultDivisor,
    HSIRC,
    HSIRC48,
    HSIRC14,
    HSEOSC,
    LSIRC,
    LSEOSC,
    SysCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCODividerConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
    DIV32,
    DIV64,
    DIV128,
}

impl MCODividerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MCODividerConf::DIV1 => return Ok(1.0),
            MCODividerConf::DIV2 => return Ok(2.0),
            MCODividerConf::DIV4 => return Ok(4.0),
            MCODividerConf::DIV8 => return Ok(8.0),
            MCODividerConf::DIV16 => return Ok(16.0),
            MCODividerConf::DIV32 => return Ok(32.0),
            MCODividerConf::DIV64 => return Ok(64.0),
            MCODividerConf::DIV128 => return Ok(128.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum AHBPrescalerConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
    DIV64,
    DIV128,
    DIV256,
    DIV512,
}

impl AHBPrescalerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            AHBPrescalerConf::DIV1 => return Ok(1.0),
            AHBPrescalerConf::DIV2 => return Ok(2.0),
            AHBPrescalerConf::DIV4 => return Ok(4.0),
            AHBPrescalerConf::DIV8 => return Ok(8.0),
            AHBPrescalerConf::DIV16 => return Ok(16.0),
            AHBPrescalerConf::DIV64 => return Ok(64.0),
            AHBPrescalerConf::DIV128 => return Ok(128.0),
            AHBPrescalerConf::DIV256 => return Ok(256.0),
            AHBPrescalerConf::DIV512 => return Ok(512.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum TimSysPrescConf {
    DIV1,
    DIV8,
}

impl TimSysPrescConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            TimSysPrescConf::DIV1 => return Ok(1.0),
            TimSysPrescConf::DIV8 => return Ok(8.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum APB1PrescalerConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl APB1PrescalerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            APB1PrescalerConf::DIV1 => return Ok(1.0),
            APB1PrescalerConf::DIV2 => return Ok(2.0),
            APB1PrescalerConf::DIV4 => return Ok(4.0),
            APB1PrescalerConf::DIV8 => return Ok(8.0),
            APB1PrescalerConf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C1MultConf {
    HSIRC,
    SysCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART1MultConf {
    SysCLKOutput,
    HSIRC,
    LSEOSC,
    APB1Prescaler,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART2MultConf {
    SysCLKOutput,
    HSIRC,
    LSEOSC,
    APB1Prescaler,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSourceConf {
    HSIRC,
    HSIRC48,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLDivConf {
    DIV1,
    DIV2,
    DIV3,
    DIV4,
    DIV5,
    DIV6,
    DIV7,
    DIV8,
    DIV9,
    DIV10,
    DIV11,
    DIV12,
    DIV13,
    DIV14,
    DIV15,
    DIV16,
}

impl PLLDivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLDivConf::DIV1 => return Ok(1.0),
            PLLDivConf::DIV2 => return Ok(2.0),
            PLLDivConf::DIV3 => return Ok(3.0),
            PLLDivConf::DIV4 => return Ok(4.0),
            PLLDivConf::DIV5 => return Ok(5.0),
            PLLDivConf::DIV6 => return Ok(6.0),
            PLLDivConf::DIV7 => return Ok(7.0),
            PLLDivConf::DIV8 => return Ok(8.0),
            PLLDivConf::DIV9 => return Ok(9.0),
            PLLDivConf::DIV10 => return Ok(10.0),
            PLLDivConf::DIV11 => return Ok(11.0),
            PLLDivConf::DIV12 => return Ok(12.0),
            PLLDivConf::DIV13 => return Ok(13.0),
            PLLDivConf::DIV14 => return Ok(14.0),
            PLLDivConf::DIV15 => return Ok(15.0),
            PLLDivConf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLMULConf {
    MUL2,
    MUL3,
    MUL4,
    MUL5,
    MUL6,
    MUL7,
    MUL8,
    MUL9,
    MUL10,
    MUL11,
    MUL12,
    MUL13,
    MUL14,
    MUL15,
    MUL16,
}

impl PLLMULConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLMULConf::MUL2 => return Ok(2.0),
            PLLMULConf::MUL3 => return Ok(3.0),
            PLLMULConf::MUL4 => return Ok(4.0),
            PLLMULConf::MUL5 => return Ok(5.0),
            PLLMULConf::MUL6 => return Ok(6.0),
            PLLMULConf::MUL7 => return Ok(7.0),
            PLLMULConf::MUL8 => return Ok(8.0),
            PLLMULConf::MUL9 => return Ok(9.0),
            PLLMULConf::MUL10 => return Ok(10.0),
            PLLMULConf::MUL11 => return Ok(11.0),
            PLLMULConf::MUL12 => return Ok(12.0),
            PLLMULConf::MUL13 => return Ok(13.0),
            PLLMULConf::MUL14 => return Ok(14.0),
            PLLMULConf::MUL15 => return Ok(15.0),
            PLLMULConf::MUL16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ClockTree {
    pub LSEOSC: LSEOSCConf,
    pub CECMult: CECMultConf,
    pub HSEOSC: HSEOSCConf,
    pub USBMult: USBMultConf,
    pub SysClkSource: SysClkSourceConf,
    pub RTCClkSource: RTCClkSourceConf,
    pub MCOMultDivisor: MCOMultDivisorConf,
    pub MCOMult: MCOMultConf,
    pub MCODivider: MCODividerConf,
    pub AHBPrescaler: AHBPrescalerConf,
    pub TimSysPresc: TimSysPrescConf,
    pub APB1Prescaler: APB1PrescalerConf,
    pub I2C1Mult: I2C1MultConf,
    pub USART1Mult: USART1MultConf,
    pub USART2Mult: USART2MultConf,
    pub PLLSource: PLLSourceConf,
    pub PLLDiv: PLLDivConf,
    pub PLLMUL: PLLMULConf,
}

impl Default for ClockTree {
    fn default() -> Self {
        Self {
            LSEOSC: LSEOSCConf::Value(32768),
            CECMult: CECMultConf::HSICECDiv,
            HSEOSC: HSEOSCConf::Value(8000000),
            USBMult: USBMultConf::HSIRC48,
            SysClkSource: SysClkSourceConf::HSIRC,
            RTCClkSource: RTCClkSourceConf::LSIRC,
            MCOMultDivisor: MCOMultDivisorConf::DIV1,
            MCOMult: MCOMultConf::SysCLKOutput,
            MCODivider: MCODividerConf::DIV1,
            AHBPrescaler: AHBPrescalerConf::DIV1,
            TimSysPresc: TimSysPrescConf::DIV1,
            APB1Prescaler: APB1PrescalerConf::DIV1,
            I2C1Mult: I2C1MultConf::HSIRC,
            USART1Mult: USART1MultConf::APB1Prescaler,
            USART2Mult: USART2MultConf::APB1Prescaler,
            PLLSource: PLLSourceConf::HSIRC,
            PLLDiv: PLLDivConf::DIV1,
            PLLMUL: PLLMULConf::MUL2,
        }
    }
}
impl ClockTree {
    pub fn HSIRC_get(&self) -> Result<f32, ClockError> {
        Ok(8000000 as f32)
    }
    pub fn FLITFCLKoutput_get(&self) -> Result<f32, ClockError> {
        self.HSIRC_get()
    }
    fn HSICECDiv_get(&self) -> Result<f32, ClockError> {
        let input = self.HSIRC_get()? as f32;
        let value = 244 as f32;
        Ok((input / value) as f32)
    }

    pub fn HSIRC48_get(&self) -> Result<f32, ClockError> {
        Ok(48000000 as f32)
    }
    pub fn HSIRC14_get(&self) -> Result<f32, ClockError> {
        Ok(14000000 as f32)
    }
    pub fn ADCoutput_get(&self) -> Result<f32, ClockError> {
        self.HSIRC14_get()
    }
    pub fn LSIRC_get(&self) -> Result<f32, ClockError> {
        Ok(40000 as f32)
    }
    pub fn LSEOSC_get(&self) -> Result<f32, ClockError> {
        self.LSEOSC.get()
    }
    fn CECMult_get(&self) -> Result<f32, ClockError> {
        match self.CECMult {
            CECMultConf::HSICECDiv => return self.HSICECDiv_get(),
            CECMultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn CECOutput_get(&self) -> Result<f32, ClockError> {
        self.CECMult_get()
    }
    pub fn HSEOSC_get(&self) -> Result<f32, ClockError> {
        self.HSEOSC.get()
    }
    fn USBMult_get(&self) -> Result<f32, ClockError> {
        match self.USBMult {
            USBMultConf::PLLMUL => return self.PLLMUL_get(),
            USBMultConf::HSIRC48 => return self.HSIRC48_get(),
        };
    }
    pub fn USBoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.USBMult_get()?;
        if input > (48120000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48120000),
                from: ClockNodes::USBMult,
                to: ClockNodes::USBoutput,
            });
        } else if input < (47880000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 47880000),
                from: ClockNodes::USBMult,
                to: ClockNodes::USBoutput,
            });
        }
        Ok(input)
    }
    fn SysClkSource_get(&self) -> Result<f32, ClockError> {
        match self.SysClkSource {
            SysClkSourceConf::HSIRC => return self.HSIRC_get(),
            SysClkSourceConf::HSIRC48 => return self.HSIRC48_get(),
            SysClkSourceConf::HSEOSC => return self.HSEOSC_get(),
            SysClkSourceConf::PLLMUL => return self.PLLMUL_get(),
        };
    }
    pub fn SysCLKOutput_get(&self) -> Result<f32, ClockError> {
        self.SysClkSource_get()
    }
    pub fn I2SOutput_get(&self) -> Result<f32, ClockError> {
        self.SysCLKOutput_get()
    }
    fn HSERTCDevisor_get(&self) -> Result<f32, ClockError> {
        let input = self.HSEOSC_get()? as f32;
        let value = 32 as f32;
        Ok((input / value) as f32)
    }

    fn RTCClkSource_get(&self) -> Result<f32, ClockError> {
        match self.RTCClkSource {
            RTCClkSourceConf::HSERTCDevisor => return self.HSERTCDevisor_get(),
            RTCClkSourceConf::LSEOSC => return self.LSEOSC_get(),
            RTCClkSourceConf::LSIRC => return self.LSIRC_get(),
        };
    }
    pub fn RTCOutput_get(&self) -> Result<f32, ClockError> {
        self.RTCClkSource_get()
    }
    pub fn IWDGOutput_get(&self) -> Result<f32, ClockError> {
        self.LSIRC_get()
    }
    fn MCOMultDivisor_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLMUL_get()? as f32;
        let value = self.MCOMultDivisor.get()? as f32;
        Ok((input / value) as f32)
    }

    fn MCOMult_get(&self) -> Result<f32, ClockError> {
        match self.MCOMult {
            MCOMultConf::MCOMultDivisor => return self.MCOMultDivisor_get(),
            MCOMultConf::HSIRC => return self.HSIRC_get(),
            MCOMultConf::HSIRC48 => return self.HSIRC48_get(),
            MCOMultConf::HSIRC14 => return self.HSIRC14_get(),
            MCOMultConf::HSEOSC => return self.HSEOSC_get(),
            MCOMultConf::LSIRC => return self.LSIRC_get(),
            MCOMultConf::LSEOSC => return self.LSEOSC_get(),
            MCOMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
        };
    }
    fn MCODivider_get(&self) -> Result<f32, ClockError> {
        let input = self.MCOMult_get()? as f32;
        let value = self.MCODivider.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn MCOoutput_get(&self) -> Result<f32, ClockError> {
        self.MCODivider_get()
    }
    fn AHBPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()? as f32;
        let value = self.AHBPrescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn AHBOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBPrescaler_get()?;
        if input > (48000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48000000),
                from: ClockNodes::AHBPrescaler,
                to: ClockNodes::AHBOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::AHBPrescaler,
                to: ClockNodes::AHBOutput,
            });
        }
        Ok(input)
    }
    pub fn HCLKOutput_get(&self) -> Result<f32, ClockError> {
        self.AHBOutput_get()
    }
    pub fn FCLKCortexOutput_get(&self) -> Result<f32, ClockError> {
        self.AHBOutput_get()
    }
    fn TimSysPresc_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.TimSysPresc.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn TimSysOutput_get(&self) -> Result<f32, ClockError> {
        self.TimSysPresc_get()
    }
    fn APB1Prescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.APB1Prescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB1Output_get(&self) -> Result<f32, ClockError> {
        let input = self.APB1Prescaler_get()?;
        if input > (48000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48000000),
                from: ClockNodes::APB1Prescaler,
                to: ClockNodes::APB1Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::APB1Prescaler,
                to: ClockNodes::APB1Output,
            });
        }
        Ok(input)
    }
    fn TimPrescalerAPB1_get(&self) -> Result<f32, ClockError> {
        let input = self.APB1Prescaler_get()? as f32;
        let value = 2 as f32;
        Ok((input * value) as f32)
    }

    pub fn TimPrescOut1_get(&self) -> Result<f32, ClockError> {
        self.TimPrescalerAPB1_get()
    }
    fn I2C1Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C1Mult {
            I2C1MultConf::HSIRC => return self.HSIRC_get(),
            I2C1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
        };
    }
    pub fn I2C1Output_get(&self) -> Result<f32, ClockError> {
        self.I2C1Mult_get()
    }
    fn USART1Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART1Mult {
            USART1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            USART1MultConf::HSIRC => return self.HSIRC_get(),
            USART1MultConf::LSEOSC => return self.LSEOSC_get(),
            USART1MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
        };
    }
    pub fn USART1Output_get(&self) -> Result<f32, ClockError> {
        self.USART1Mult_get()
    }
    fn USART2Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART2Mult {
            USART2MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            USART2MultConf::HSIRC => return self.HSIRC_get(),
            USART2MultConf::LSEOSC => return self.LSEOSC_get(),
            USART2MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
        };
    }
    pub fn USART2Output_get(&self) -> Result<f32, ClockError> {
        self.USART2Mult_get()
    }
    fn PLLSource_get(&self) -> Result<f32, ClockError> {
        match self.PLLSource {
            PLLSourceConf::HSIRC => return self.HSIRC_get(),
            PLLSourceConf::HSIRC48 => return self.HSIRC48_get(),
            PLLSourceConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    fn PLLDiv_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLSource_get()? as f32;
        let value = self.PLLDiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn VCO2output_get(&self) -> Result<f32, ClockError> {
        self.PLLDiv_get()
    }
    fn PLLMUL_get(&self) -> Result<f32, ClockError> {
        let input = self.VCO2output_get()? as f32;
        let value = self.PLLMUL.get()? as f32;
        Ok((input * value) as f32)
    }
}
