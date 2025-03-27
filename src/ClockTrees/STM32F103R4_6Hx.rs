#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ClockNodes {
    None,
    HSIRC,
    FLITFCLKoutput,
    HSIDivPLL,
    LSIRC,
    LSEOSC,
    HSEOSC,
    HSEDivPLL,
    SysClkSource,
    SysCLKOutput,
    HSERTCDevisor,
    RTCClkSource,
    RTCOutput,
    IWDGOutput,
    MCOMultDivisor,
    MCOMult,
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
    APB2Prescaler,
    APB2Output,
    TimPrescalerAPB2,
    TimPrescOut2,
    ADCprescaler,
    ADCoutput,
    USBPrescaler,
    USBoutput,
    PLLSource,
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
pub enum HSEOSCConf {
    Value(u32),
}

impl HSEOSCConf {
    pub const fn min() -> u32 {
        4000000
    }

    pub const fn max() -> u32 {
        16000000
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
pub enum HSEDivPLLConf {
    DIV1,
    DIV2,
}

impl HSEDivPLLConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            HSEDivPLLConf::DIV1 => return Ok(1.0),
            HSEDivPLLConf::DIV2 => return Ok(2.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SysClkSourceConf {
    HSIRC,
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
pub enum MCOMultConf {
    MCOMultDivisor,
    HSIRC,
    HSEOSC,
    SysCLKOutput,
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
pub enum APB2PrescalerConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl APB2PrescalerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            APB2PrescalerConf::DIV1 => return Ok(1.0),
            APB2PrescalerConf::DIV2 => return Ok(2.0),
            APB2PrescalerConf::DIV4 => return Ok(4.0),
            APB2PrescalerConf::DIV8 => return Ok(8.0),
            APB2PrescalerConf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ADCprescalerConf {
    DIV2,
    DIV4,
    DIV6,
    DIV8,
}

impl ADCprescalerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            ADCprescalerConf::DIV2 => return Ok(2.0),
            ADCprescalerConf::DIV4 => return Ok(4.0),
            ADCprescalerConf::DIV6 => return Ok(6.0),
            ADCprescalerConf::DIV8 => return Ok(8.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USBPrescalerConf {
    DIV1,
    DIV1_5,
}

impl USBPrescalerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            USBPrescalerConf::DIV1 => return Ok(1.0),
            USBPrescalerConf::DIV1_5 => return Ok(1.5),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSourceConf {
    HSIDivPLL,
    HSEDivPLL,
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
    pub HSEOSC: HSEOSCConf,
    pub HSEDivPLL: HSEDivPLLConf,
    pub SysClkSource: SysClkSourceConf,
    pub RTCClkSource: RTCClkSourceConf,
    pub MCOMult: MCOMultConf,
    pub AHBPrescaler: AHBPrescalerConf,
    pub TimSysPresc: TimSysPrescConf,
    pub APB1Prescaler: APB1PrescalerConf,
    pub APB2Prescaler: APB2PrescalerConf,
    pub ADCprescaler: ADCprescalerConf,
    pub USBPrescaler: USBPrescalerConf,
    pub PLLSource: PLLSourceConf,
    pub PLLMUL: PLLMULConf,
}

impl Default for ClockTree {
    fn default() -> Self {
        Self {
            LSEOSC: LSEOSCConf::Value(32768),
            HSEOSC: HSEOSCConf::Value(8000000),
            HSEDivPLL: HSEDivPLLConf::DIV1,
            SysClkSource: SysClkSourceConf::HSIRC,
            RTCClkSource: RTCClkSourceConf::LSIRC,
            MCOMult: MCOMultConf::SysCLKOutput,
            AHBPrescaler: AHBPrescalerConf::DIV1,
            TimSysPresc: TimSysPrescConf::DIV1,
            APB1Prescaler: APB1PrescalerConf::DIV1,
            APB2Prescaler: APB2PrescalerConf::DIV1,
            ADCprescaler: ADCprescalerConf::DIV2,
            USBPrescaler: USBPrescalerConf::DIV1,
            PLLSource: PLLSourceConf::HSIDivPLL,
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
    fn HSIDivPLL_get(&self) -> Result<f32, ClockError> {
        let input = self.HSIRC_get()? as f32;
        let value = 2 as f32;
        Ok((input / value) as f32)
    }

    pub fn LSIRC_get(&self) -> Result<f32, ClockError> {
        Ok(40000 as f32)
    }
    pub fn LSEOSC_get(&self) -> Result<f32, ClockError> {
        self.LSEOSC.get()
    }
    pub fn HSEOSC_get(&self) -> Result<f32, ClockError> {
        self.HSEOSC.get()
    }
    fn HSEDivPLL_get(&self) -> Result<f32, ClockError> {
        let input = self.HSEOSC_get()? as f32;
        let value = self.HSEDivPLL.get()? as f32;
        Ok((input / value) as f32)
    }

    fn SysClkSource_get(&self) -> Result<f32, ClockError> {
        match self.SysClkSource {
            SysClkSourceConf::HSIRC => return self.HSIRC_get(),
            SysClkSourceConf::HSEOSC => return self.HSEOSC_get(),
            SysClkSourceConf::PLLMUL => return self.PLLMUL_get(),
        };
    }
    pub fn SysCLKOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SysClkSource_get()?;
        if input > (72000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 72000000),
                from: ClockNodes::SysClkSource,
                to: ClockNodes::SysCLKOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SysClkSource,
                to: ClockNodes::SysCLKOutput,
            });
        }
        Ok(input)
    }
    fn HSERTCDevisor_get(&self) -> Result<f32, ClockError> {
        let input = self.HSEOSC_get()? as f32;
        let value = 128 as f32;
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
        let value = 2 as f32;
        Ok((input / value) as f32)
    }

    fn MCOMult_get(&self) -> Result<f32, ClockError> {
        match self.MCOMult {
            MCOMultConf::MCOMultDivisor => return self.MCOMultDivisor_get(),
            MCOMultConf::HSIRC => return self.HSIRC_get(),
            MCOMultConf::HSEOSC => return self.HSEOSC_get(),
            MCOMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
        };
    }
    pub fn MCOoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.MCOMult_get()?;
        if input > (50000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 50000000),
                from: ClockNodes::MCOMult,
                to: ClockNodes::MCOoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::MCOMult,
                to: ClockNodes::MCOoutput,
            });
        }
        Ok(input)
    }
    fn AHBPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()? as f32;
        let value = self.AHBPrescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn AHBOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBPrescaler_get()?;
        if input > (72000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 72000000),
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
        if input > (36000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 36000000),
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
    fn APB2Prescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.APB2Prescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB2Output_get(&self) -> Result<f32, ClockError> {
        let input = self.APB2Prescaler_get()?;
        if input > (72000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 72000000),
                from: ClockNodes::APB2Prescaler,
                to: ClockNodes::APB2Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::APB2Prescaler,
                to: ClockNodes::APB2Output,
            });
        }
        Ok(input)
    }
    fn TimPrescalerAPB2_get(&self) -> Result<f32, ClockError> {
        let input = self.APB2Prescaler_get()? as f32;
        let value = 2 as f32;
        Ok((input * value) as f32)
    }

    pub fn TimPrescOut2_get(&self) -> Result<f32, ClockError> {
        self.TimPrescalerAPB2_get()
    }
    fn ADCprescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.APB2Prescaler_get()? as f32;
        let value = self.ADCprescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn ADCoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.ADCprescaler_get()?;
        if input > (14000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 14000000),
                from: ClockNodes::ADCprescaler,
                to: ClockNodes::ADCoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::ADCprescaler,
                to: ClockNodes::ADCoutput,
            });
        }
        Ok(input)
    }
    fn USBPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLMUL_get()? as f32;
        let value = self.USBPrescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn USBoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.USBPrescaler_get()?;
        if input > (48120000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48120000),
                from: ClockNodes::USBPrescaler,
                to: ClockNodes::USBoutput,
            });
        } else if input < (47880000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 47880000),
                from: ClockNodes::USBPrescaler,
                to: ClockNodes::USBoutput,
            });
        }
        Ok(input)
    }
    fn PLLSource_get(&self) -> Result<f32, ClockError> {
        match self.PLLSource {
            PLLSourceConf::HSIDivPLL => return self.HSIDivPLL_get(),
            PLLSourceConf::HSEDivPLL => return self.HSEDivPLL_get(),
        };
    }
    pub fn VCO2output_get(&self) -> Result<f32, ClockError> {
        self.PLLSource_get()
    }
    fn PLLMUL_get(&self) -> Result<f32, ClockError> {
        let input = self.VCO2output_get()? as f32;
        let value = self.PLLMUL.get()? as f32;
        Ok((input * value) as f32)
    }
}
