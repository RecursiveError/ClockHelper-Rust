#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ClockNodes {
    None,
    HSIRC,
    FLITFCLKoutput,
    HSIRCDiv,
    HSICECDiv,
    LSIRC,
    LSEOSC,
    CECMult,
    CECOutput,
    HSEOSC,
    HSEPLLsourceDevisor,
    PRESCALERUSB,
    USBoutput,
    SysClkSource,
    SysCLKOutput,
    SDADCPresc,
    SDADCoutput,
    PWROutput,
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
    ADCPresc,
    ADCoutput,
    APB2Output,
    TimPrescalerAPB2,
    TimPrescOut2,
    I2C1Mult,
    I2C1Output,
    I2C2Mult,
    I2C2Output,
    USART1Mult,
    USART1Output,
    USART2Mult,
    USART2Output,
    USART3Mult,
    USART3Output,
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
pub enum HSEPLLsourceDevisorConf {
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

impl HSEPLLsourceDevisorConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            HSEPLLsourceDevisorConf::DIV1 => return Ok(1.0),
            HSEPLLsourceDevisorConf::DIV2 => return Ok(2.0),
            HSEPLLsourceDevisorConf::DIV3 => return Ok(3.0),
            HSEPLLsourceDevisorConf::DIV4 => return Ok(4.0),
            HSEPLLsourceDevisorConf::DIV5 => return Ok(5.0),
            HSEPLLsourceDevisorConf::DIV6 => return Ok(6.0),
            HSEPLLsourceDevisorConf::DIV7 => return Ok(7.0),
            HSEPLLsourceDevisorConf::DIV8 => return Ok(8.0),
            HSEPLLsourceDevisorConf::DIV9 => return Ok(9.0),
            HSEPLLsourceDevisorConf::DIV10 => return Ok(10.0),
            HSEPLLsourceDevisorConf::DIV11 => return Ok(11.0),
            HSEPLLsourceDevisorConf::DIV12 => return Ok(12.0),
            HSEPLLsourceDevisorConf::DIV13 => return Ok(13.0),
            HSEPLLsourceDevisorConf::DIV14 => return Ok(14.0),
            HSEPLLsourceDevisorConf::DIV15 => return Ok(15.0),
            HSEPLLsourceDevisorConf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PRESCALERUSBConf {
    DIV1,
    DIV1_5,
}

impl PRESCALERUSBConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PRESCALERUSBConf::DIV1 => return Ok(1.0),
            PRESCALERUSBConf::DIV1_5 => return Ok(1.5),
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
pub enum SDADCPrescConf {
    DIV2,
    DIV4,
    DIV6,
    DIV8,
    DIV10,
    DIV12,
    DIV14,
    DIV16,
    DIV20,
    DIV24,
    DIV28,
    DIV32,
    DIV36,
    DIV40,
    DIV44,
    DIV48,
}

impl SDADCPrescConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            SDADCPrescConf::DIV2 => return Ok(2.0),
            SDADCPrescConf::DIV4 => return Ok(4.0),
            SDADCPrescConf::DIV6 => return Ok(6.0),
            SDADCPrescConf::DIV8 => return Ok(8.0),
            SDADCPrescConf::DIV10 => return Ok(10.0),
            SDADCPrescConf::DIV12 => return Ok(12.0),
            SDADCPrescConf::DIV14 => return Ok(14.0),
            SDADCPrescConf::DIV16 => return Ok(16.0),
            SDADCPrescConf::DIV20 => return Ok(20.0),
            SDADCPrescConf::DIV24 => return Ok(24.0),
            SDADCPrescConf::DIV28 => return Ok(28.0),
            SDADCPrescConf::DIV32 => return Ok(32.0),
            SDADCPrescConf::DIV36 => return Ok(36.0),
            SDADCPrescConf::DIV40 => return Ok(40.0),
            SDADCPrescConf::DIV44 => return Ok(44.0),
            SDADCPrescConf::DIV48 => return Ok(48.0),
        }
    }
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
    SysCLKOutput,
    HSIRC,
    HSEOSC,
    LSIRC,
    LSEOSC,
    MCOMultDivisor,
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
pub enum ADCPrescConf {
    DIV2,
    DIV4,
    DIV6,
    DIV8,
}

impl ADCPrescConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            ADCPrescConf::DIV2 => return Ok(2.0),
            ADCPrescConf::DIV4 => return Ok(4.0),
            ADCPrescConf::DIV6 => return Ok(6.0),
            ADCPrescConf::DIV8 => return Ok(8.0),
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
pub enum I2C2MultConf {
    HSIRC,
    SysCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART1MultConf {
    SysCLKOutput,
    HSIRC,
    LSEOSC,
    APB2Prescaler,
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
pub enum USART3MultConf {
    SysCLKOutput,
    HSIRC,
    LSEOSC,
    APB1Prescaler,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSourceConf {
    HSIRCDiv,
    HSEPLLsourceDevisor,
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
    pub HSEPLLsourceDevisor: HSEPLLsourceDevisorConf,
    pub PRESCALERUSB: PRESCALERUSBConf,
    pub SysClkSource: SysClkSourceConf,
    pub SDADCPresc: SDADCPrescConf,
    pub RTCClkSource: RTCClkSourceConf,
    pub MCOMult: MCOMultConf,
    pub AHBPrescaler: AHBPrescalerConf,
    pub TimSysPresc: TimSysPrescConf,
    pub APB1Prescaler: APB1PrescalerConf,
    pub APB2Prescaler: APB2PrescalerConf,
    pub ADCPresc: ADCPrescConf,
    pub I2C1Mult: I2C1MultConf,
    pub I2C2Mult: I2C2MultConf,
    pub USART1Mult: USART1MultConf,
    pub USART2Mult: USART2MultConf,
    pub USART3Mult: USART3MultConf,
    pub PLLSource: PLLSourceConf,
    pub PLLMUL: PLLMULConf,
}

impl Default for ClockTree {
    fn default() -> Self {
        Self {
            LSEOSC: LSEOSCConf::Value(32768),
            CECMult: CECMultConf::HSICECDiv,
            HSEOSC: HSEOSCConf::Value(8000000),
            HSEPLLsourceDevisor: HSEPLLsourceDevisorConf::DIV1,
            PRESCALERUSB: PRESCALERUSBConf::DIV1,
            SysClkSource: SysClkSourceConf::HSIRC,
            SDADCPresc: SDADCPrescConf::DIV2,
            RTCClkSource: RTCClkSourceConf::LSIRC,
            MCOMult: MCOMultConf::SysCLKOutput,
            AHBPrescaler: AHBPrescalerConf::DIV1,
            TimSysPresc: TimSysPrescConf::DIV1,
            APB1Prescaler: APB1PrescalerConf::DIV1,
            APB2Prescaler: APB2PrescalerConf::DIV1,
            ADCPresc: ADCPrescConf::DIV2,
            I2C1Mult: I2C1MultConf::HSIRC,
            I2C2Mult: I2C2MultConf::HSIRC,
            USART1Mult: USART1MultConf::APB2Prescaler,
            USART2Mult: USART2MultConf::APB1Prescaler,
            USART3Mult: USART3MultConf::APB1Prescaler,
            PLLSource: PLLSourceConf::HSIRCDiv,
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
    fn HSIRCDiv_get(&self) -> Result<f32, ClockError> {
        let input = self.HSIRC_get()? as f32;
        let value = 2 as f32;
        Ok((input / value) as f32)
    }

    fn HSICECDiv_get(&self) -> Result<f32, ClockError> {
        let input = self.HSIRC_get()? as f32;
        let value = 244 as f32;
        Ok((input / value) as f32)
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
    fn HSEPLLsourceDevisor_get(&self) -> Result<f32, ClockError> {
        let input = self.HSEOSC_get()? as f32;
        let value = self.HSEPLLsourceDevisor.get()? as f32;
        Ok((input / value) as f32)
    }

    fn PRESCALERUSB_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLMUL_get()? as f32;
        let value = self.PRESCALERUSB.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn USBoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.PRESCALERUSB_get()?;
        if input > (48120000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48120000),
                from: ClockNodes::PRESCALERUSB,
                to: ClockNodes::USBoutput,
            });
        } else if input < (47880000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 47880000),
                from: ClockNodes::PRESCALERUSB,
                to: ClockNodes::USBoutput,
            });
        }
        Ok(input)
    }
    fn SysClkSource_get(&self) -> Result<f32, ClockError> {
        match self.SysClkSource {
            SysClkSourceConf::HSIRC => return self.HSIRC_get(),
            SysClkSourceConf::HSEOSC => return self.HSEOSC_get(),
            SysClkSourceConf::PLLMUL => return self.PLLMUL_get(),
        };
    }
    pub fn SysCLKOutput_get(&self) -> Result<f32, ClockError> {
        self.SysClkSource_get()
    }
    fn SDADCPresc_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()? as f32;
        let value = self.SDADCPresc.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn SDADCoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SDADCPresc_get()?;
        if input > (6000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 6000000),
                from: ClockNodes::SDADCPresc,
                to: ClockNodes::SDADCoutput,
            });
        } else if input < (500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 500000),
                from: ClockNodes::SDADCPresc,
                to: ClockNodes::SDADCoutput,
            });
        }
        Ok(input)
    }
    pub fn PWROutput_get(&self) -> Result<f32, ClockError> {
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
        let value = 2 as f32;
        Ok((input / value) as f32)
    }

    fn MCOMult_get(&self) -> Result<f32, ClockError> {
        match self.MCOMult {
            MCOMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            MCOMultConf::HSIRC => return self.HSIRC_get(),
            MCOMultConf::HSEOSC => return self.HSEOSC_get(),
            MCOMultConf::LSIRC => return self.LSIRC_get(),
            MCOMultConf::LSEOSC => return self.LSEOSC_get(),
            MCOMultConf::MCOMultDivisor => return self.MCOMultDivisor_get(),
        };
    }
    pub fn MCOoutput_get(&self) -> Result<f32, ClockError> {
        self.MCOMult_get()
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

    fn ADCPresc_get(&self) -> Result<f32, ClockError> {
        let input = self.APB2Prescaler_get()? as f32;
        let value = self.ADCPresc.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn ADCoutput_get(&self) -> Result<f32, ClockError> {
        self.ADCPresc_get()
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
    fn I2C1Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C1Mult {
            I2C1MultConf::HSIRC => return self.HSIRC_get(),
            I2C1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
        };
    }
    pub fn I2C1Output_get(&self) -> Result<f32, ClockError> {
        self.I2C1Mult_get()
    }
    fn I2C2Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C2Mult {
            I2C2MultConf::HSIRC => return self.HSIRC_get(),
            I2C2MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
        };
    }
    pub fn I2C2Output_get(&self) -> Result<f32, ClockError> {
        self.I2C2Mult_get()
    }
    fn USART1Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART1Mult {
            USART1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            USART1MultConf::HSIRC => return self.HSIRC_get(),
            USART1MultConf::LSEOSC => return self.LSEOSC_get(),
            USART1MultConf::APB2Prescaler => return self.APB2Prescaler_get(),
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
    fn USART3Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART3Mult {
            USART3MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            USART3MultConf::HSIRC => return self.HSIRC_get(),
            USART3MultConf::LSEOSC => return self.LSEOSC_get(),
            USART3MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
        };
    }
    pub fn USART3Output_get(&self) -> Result<f32, ClockError> {
        self.USART3Mult_get()
    }
    fn PLLSource_get(&self) -> Result<f32, ClockError> {
        match self.PLLSource {
            PLLSourceConf::HSIRCDiv => return self.HSIRCDiv_get(),
            PLLSourceConf::HSEPLLsourceDevisor => return self.HSEPLLsourceDevisor_get(),
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
