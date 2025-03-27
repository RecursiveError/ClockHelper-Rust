#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ClockNodes {
    None,
    HSIRC,
    HSEOSC,
    LSIRC,
    LSEOSC,
    I2S_CKIN,
    SysClkSource,
    SysCLKOutput,
    PLLSource,
    PLLM,
    PLLSAIM,
    PLLI2SM,
    HSERTCDevisor,
    RTCClkSource,
    RTCOutput,
    IWDGOutput,
    HSIDivCEC,
    CECMult,
    CECOutput,
    FMPI2C1Mult,
    FMPI2C1output,
    USBMult,
    USBoutput,
    SPDIFMult,
    SPDIFoutput,
    SDIOMult,
    SDIOoutput,
    SAIAMult,
    SAIAoutput,
    SAIBMult,
    SAIBoutput,
    I2S1Mult,
    I2S1output,
    I2S2Mult,
    I2S2output,
    MCO1Mult,
    MCO1Div,
    MCO1Pin,
    MCO2Mult,
    MCO2Div,
    MCO2Pin,
    AHBPrescaler,
    PWRCLKoutput,
    AHBOutput,
    HCLKOutput,
    CortexPrescaler,
    CortexSysOutput,
    FCLKCortexOutput,
    APB1Prescaler,
    APB1Output,
    TimPrescalerAPB1,
    TimPrescOut1,
    APB2Prescaler,
    APB2Output,
    TimPrescalerAPB2,
    TimPrescOut2,
    PLLN,
    PLLP,
    PLLQ,
    PLLR,
    PLLSAIN,
    PLLSAIP,
    PLLSAIoutput,
    PLLSAIQ,
    PLLSAIQDiv,
    PLLI2SN,
    PLLI2SP,
    PLLI2Soutput,
    PLLI2SQ,
    PLLI2SQDiv,
    PLLI2SR,
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
pub enum HSEOSCConf {
    Value(u32),
}

impl HSEOSCConf {
    pub const fn min() -> u32 {
        4000000
    }

    pub const fn max() -> u32 {
        26000000
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
pub enum SysClkSourceConf {
    HSIRC,
    HSEOSC,
    PLLP,
    PLLR,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSourceConf {
    HSIRC,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLMConf {
    Value(u32),
}

impl PLLMConf {
    pub const fn min() -> u32 {
        2
    }

    pub const fn max() -> u32 {
        63
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLMConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLM,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLM,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSAIMConf {
    Value(u32),
}

impl PLLSAIMConf {
    pub const fn min() -> u32 {
        2
    }

    pub const fn max() -> u32 {
        63
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLSAIMConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLSAIM,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLSAIM,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLI2SMConf {
    Value(u32),
}

impl PLLI2SMConf {
    pub const fn min() -> u32 {
        2
    }

    pub const fn max() -> u32 {
        63
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLI2SMConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLI2SM,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLI2SM,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum HSERTCDevisorConf {
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
    DIV17,
    DIV18,
    DIV19,
    DIV20,
    DIV21,
    DIV22,
    DIV23,
    DIV24,
    DIV25,
    DIV26,
    DIV27,
    DIV28,
    DIV29,
    DIV30,
    DIV31,
}

impl HSERTCDevisorConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            HSERTCDevisorConf::DIV2 => return Ok(2.0),
            HSERTCDevisorConf::DIV3 => return Ok(3.0),
            HSERTCDevisorConf::DIV4 => return Ok(4.0),
            HSERTCDevisorConf::DIV5 => return Ok(5.0),
            HSERTCDevisorConf::DIV6 => return Ok(6.0),
            HSERTCDevisorConf::DIV7 => return Ok(7.0),
            HSERTCDevisorConf::DIV8 => return Ok(8.0),
            HSERTCDevisorConf::DIV9 => return Ok(9.0),
            HSERTCDevisorConf::DIV10 => return Ok(10.0),
            HSERTCDevisorConf::DIV11 => return Ok(11.0),
            HSERTCDevisorConf::DIV12 => return Ok(12.0),
            HSERTCDevisorConf::DIV13 => return Ok(13.0),
            HSERTCDevisorConf::DIV14 => return Ok(14.0),
            HSERTCDevisorConf::DIV15 => return Ok(15.0),
            HSERTCDevisorConf::DIV16 => return Ok(16.0),
            HSERTCDevisorConf::DIV17 => return Ok(17.0),
            HSERTCDevisorConf::DIV18 => return Ok(18.0),
            HSERTCDevisorConf::DIV19 => return Ok(19.0),
            HSERTCDevisorConf::DIV20 => return Ok(20.0),
            HSERTCDevisorConf::DIV21 => return Ok(21.0),
            HSERTCDevisorConf::DIV22 => return Ok(22.0),
            HSERTCDevisorConf::DIV23 => return Ok(23.0),
            HSERTCDevisorConf::DIV24 => return Ok(24.0),
            HSERTCDevisorConf::DIV25 => return Ok(25.0),
            HSERTCDevisorConf::DIV26 => return Ok(26.0),
            HSERTCDevisorConf::DIV27 => return Ok(27.0),
            HSERTCDevisorConf::DIV28 => return Ok(28.0),
            HSERTCDevisorConf::DIV29 => return Ok(29.0),
            HSERTCDevisorConf::DIV30 => return Ok(30.0),
            HSERTCDevisorConf::DIV31 => return Ok(31.0),
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
pub enum CECMultConf {
    HSIDivCEC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FMPI2C1MultConf {
    HSIRC,
    APB1Prescaler,
    SysCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USBMultConf {
    PLLQ,
    PLLSAIP,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPDIFMultConf {
    PLLR,
    PLLI2SP,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SDIOMultConf {
    SysCLKOutput,
    USBMult,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SAIAMultConf {
    I2S_CKIN,
    PLLR,
    PLLI2SQDiv,
    PLLSAIQDiv,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SAIBMultConf {
    PLLSource,
    PLLR,
    PLLI2SQDiv,
    PLLSAIQDiv,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2S1MultConf {
    PLLSource,
    PLLR,
    I2S_CKIN,
    PLLI2SR,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2S2MultConf {
    PLLSource,
    PLLR,
    I2S_CKIN,
    PLLI2SR,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCO1MultConf {
    LSEOSC,
    HSEOSC,
    HSIRC,
    PLLP,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCO1DivConf {
    DIV1,
    DIV2,
    DIV3,
    DIV4,
    DIV5,
}

impl MCO1DivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MCO1DivConf::DIV1 => return Ok(1.0),
            MCO1DivConf::DIV2 => return Ok(2.0),
            MCO1DivConf::DIV3 => return Ok(3.0),
            MCO1DivConf::DIV4 => return Ok(4.0),
            MCO1DivConf::DIV5 => return Ok(5.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCO2MultConf {
    SysClkSource,
    PLLI2SR,
    HSEOSC,
    PLLP,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCO2DivConf {
    DIV1,
    DIV2,
    DIV3,
    DIV4,
    DIV5,
}

impl MCO2DivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MCO2DivConf::DIV1 => return Ok(1.0),
            MCO2DivConf::DIV2 => return Ok(2.0),
            MCO2DivConf::DIV3 => return Ok(3.0),
            MCO2DivConf::DIV4 => return Ok(4.0),
            MCO2DivConf::DIV5 => return Ok(5.0),
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
pub enum CortexPrescalerConf {
    DIV1,
    DIV8,
}

impl CortexPrescalerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            CortexPrescalerConf::DIV1 => return Ok(1.0),
            CortexPrescalerConf::DIV8 => return Ok(8.0),
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
pub enum PLLNConf {
    Value(u32),
}

impl PLLNConf {
    pub const fn min() -> u32 {
        50
    }

    pub const fn max() -> u32 {
        432
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLNConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLN,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLN,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLPConf {
    DIV2,
    DIV4,
    DIV6,
    DIV8,
}

impl PLLPConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLPConf::DIV2 => return Ok(2.0),
            PLLPConf::DIV4 => return Ok(4.0),
            PLLPConf::DIV6 => return Ok(6.0),
            PLLPConf::DIV8 => return Ok(8.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLQConf {
    Value(u32),
}

impl PLLQConf {
    pub const fn min() -> u32 {
        2
    }

    pub const fn max() -> u32 {
        15
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLQConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLQ,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLQ,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLRConf {
    Value(u32),
}

impl PLLRConf {
    pub const fn min() -> u32 {
        2
    }

    pub const fn max() -> u32 {
        7
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLRConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLR,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLR,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSAINConf {
    Value(u32),
}

impl PLLSAINConf {
    pub const fn min() -> u32 {
        50
    }

    pub const fn max() -> u32 {
        432
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLSAINConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLSAIN,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLSAIN,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSAIPConf {
    DIV2,
    DIV4,
    DIV6,
    DIV8,
}

impl PLLSAIPConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLSAIPConf::DIV2 => return Ok(2.0),
            PLLSAIPConf::DIV4 => return Ok(4.0),
            PLLSAIPConf::DIV6 => return Ok(6.0),
            PLLSAIPConf::DIV8 => return Ok(8.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSAIQConf {
    Value(u32),
}

impl PLLSAIQConf {
    pub const fn min() -> u32 {
        2
    }

    pub const fn max() -> u32 {
        15
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLSAIQConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLSAIQ,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLSAIQ,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSAIQDivConf {
    Value(u32),
}

impl PLLSAIQDivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        32
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLSAIQDivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLSAIQDiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLSAIQDiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLI2SNConf {
    Value(u32),
}

impl PLLI2SNConf {
    pub const fn min() -> u32 {
        50
    }

    pub const fn max() -> u32 {
        432
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLI2SNConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLI2SN,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLI2SN,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLI2SPConf {
    DIV2,
    DIV4,
    DIV6,
    DIV8,
}

impl PLLI2SPConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLI2SPConf::DIV2 => return Ok(2.0),
            PLLI2SPConf::DIV4 => return Ok(4.0),
            PLLI2SPConf::DIV6 => return Ok(6.0),
            PLLI2SPConf::DIV8 => return Ok(8.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLI2SQConf {
    Value(u32),
}

impl PLLI2SQConf {
    pub const fn min() -> u32 {
        2
    }

    pub const fn max() -> u32 {
        15
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLI2SQConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLI2SQ,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLI2SQ,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLI2SQDivConf {
    Value(u32),
}

impl PLLI2SQDivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        32
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLI2SQDivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLI2SQDiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLI2SQDiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLI2SRConf {
    Value(u32),
}

impl PLLI2SRConf {
    pub const fn min() -> u32 {
        2
    }

    pub const fn max() -> u32 {
        7
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLI2SRConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLI2SR,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLI2SR,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ClockTree {
    pub HSEOSC: HSEOSCConf,
    pub LSEOSC: LSEOSCConf,
    pub SysClkSource: SysClkSourceConf,
    pub PLLSource: PLLSourceConf,
    pub PLLM: PLLMConf,
    pub PLLSAIM: PLLSAIMConf,
    pub PLLI2SM: PLLI2SMConf,
    pub HSERTCDevisor: HSERTCDevisorConf,
    pub RTCClkSource: RTCClkSourceConf,
    pub CECMult: CECMultConf,
    pub FMPI2C1Mult: FMPI2C1MultConf,
    pub USBMult: USBMultConf,
    pub SPDIFMult: SPDIFMultConf,
    pub SDIOMult: SDIOMultConf,
    pub SAIAMult: SAIAMultConf,
    pub SAIBMult: SAIBMultConf,
    pub I2S1Mult: I2S1MultConf,
    pub I2S2Mult: I2S2MultConf,
    pub MCO1Mult: MCO1MultConf,
    pub MCO1Div: MCO1DivConf,
    pub MCO2Mult: MCO2MultConf,
    pub MCO2Div: MCO2DivConf,
    pub AHBPrescaler: AHBPrescalerConf,
    pub CortexPrescaler: CortexPrescalerConf,
    pub APB1Prescaler: APB1PrescalerConf,
    pub APB2Prescaler: APB2PrescalerConf,
    pub PLLN: PLLNConf,
    pub PLLP: PLLPConf,
    pub PLLQ: PLLQConf,
    pub PLLR: PLLRConf,
    pub PLLSAIN: PLLSAINConf,
    pub PLLSAIP: PLLSAIPConf,
    pub PLLSAIQ: PLLSAIQConf,
    pub PLLSAIQDiv: PLLSAIQDivConf,
    pub PLLI2SN: PLLI2SNConf,
    pub PLLI2SP: PLLI2SPConf,
    pub PLLI2SQ: PLLI2SQConf,
    pub PLLI2SQDiv: PLLI2SQDivConf,
    pub PLLI2SR: PLLI2SRConf,
}

impl Default for ClockTree {
    fn default() -> Self {
        Self {
            HSEOSC: HSEOSCConf::Value(25000000),
            LSEOSC: LSEOSCConf::Value(32768),
            SysClkSource: SysClkSourceConf::HSIRC,
            PLLSource: PLLSourceConf::HSIRC,
            PLLM: PLLMConf::Value(16),
            PLLSAIM: PLLSAIMConf::Value(16),
            PLLI2SM: PLLI2SMConf::Value(16),
            HSERTCDevisor: HSERTCDevisorConf::DIV2,
            RTCClkSource: RTCClkSourceConf::LSIRC,
            CECMult: CECMultConf::HSIDivCEC,
            FMPI2C1Mult: FMPI2C1MultConf::APB1Prescaler,
            USBMult: USBMultConf::PLLQ,
            SPDIFMult: SPDIFMultConf::PLLR,
            SDIOMult: SDIOMultConf::USBMult,
            SAIAMult: SAIAMultConf::PLLSAIQDiv,
            SAIBMult: SAIBMultConf::PLLSAIQDiv,
            I2S1Mult: I2S1MultConf::PLLI2SR,
            I2S2Mult: I2S2MultConf::PLLI2SR,
            MCO1Mult: MCO1MultConf::HSIRC,
            MCO1Div: MCO1DivConf::DIV1,
            MCO2Mult: MCO2MultConf::SysClkSource,
            MCO2Div: MCO2DivConf::DIV1,
            AHBPrescaler: AHBPrescalerConf::DIV1,
            CortexPrescaler: CortexPrescalerConf::DIV1,
            APB1Prescaler: APB1PrescalerConf::DIV1,
            APB2Prescaler: APB2PrescalerConf::DIV1,
            PLLN: PLLNConf::Value(192),
            PLLP: PLLPConf::DIV2,
            PLLQ: PLLQConf::Value(2),
            PLLR: PLLRConf::Value(2),
            PLLSAIN: PLLSAINConf::Value(192),
            PLLSAIP: PLLSAIPConf::DIV2,
            PLLSAIQ: PLLSAIQConf::Value(2),
            PLLSAIQDiv: PLLSAIQDivConf::Value(1),
            PLLI2SN: PLLI2SNConf::Value(192),
            PLLI2SP: PLLI2SPConf::DIV2,
            PLLI2SQ: PLLI2SQConf::Value(2),
            PLLI2SQDiv: PLLI2SQDivConf::Value(1),
            PLLI2SR: PLLI2SRConf::Value(2),
        }
    }
}
impl ClockTree {
    pub fn HSIRC_get(&self) -> Result<f32, ClockError> {
        Ok(16000000 as f32)
    }
    pub fn HSEOSC_get(&self) -> Result<f32, ClockError> {
        self.HSEOSC.get()
    }
    pub fn LSIRC_get(&self) -> Result<f32, ClockError> {
        Ok(32000 as f32)
    }
    pub fn LSEOSC_get(&self) -> Result<f32, ClockError> {
        self.LSEOSC.get()
    }
    pub fn I2S_CKIN_get(&self) -> Result<f32, ClockError> {
        Ok(12288000 as f32)
    }
    fn SysClkSource_get(&self) -> Result<f32, ClockError> {
        match self.SysClkSource {
            SysClkSourceConf::HSIRC => return self.HSIRC_get(),
            SysClkSourceConf::HSEOSC => return self.HSEOSC_get(),
            SysClkSourceConf::PLLP => return self.PLLP_get(),
            SysClkSourceConf::PLLR => return self.PLLR_get(),
        };
    }
    pub fn SysCLKOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SysClkSource_get()?;
        if input > (180000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 180000000),
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
    fn PLLSource_get(&self) -> Result<f32, ClockError> {
        match self.PLLSource {
            PLLSourceConf::HSIRC => return self.HSIRC_get(),
            PLLSourceConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    fn PLLM_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLSource_get()? as f32;
        let value = self.PLLM.get()? as f32;
        Ok((input / value) as f32)
    }

    fn PLLSAIM_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLSource_get()? as f32;
        let value = self.PLLSAIM.get()? as f32;
        Ok((input / value) as f32)
    }

    fn PLLI2SM_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLSource_get()? as f32;
        let value = self.PLLI2SM.get()? as f32;
        Ok((input / value) as f32)
    }

    fn HSERTCDevisor_get(&self) -> Result<f32, ClockError> {
        let input = self.HSEOSC_get()? as f32;
        let value = self.HSERTCDevisor.get()? as f32;
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
    fn HSIDivCEC_get(&self) -> Result<f32, ClockError> {
        let input = self.HSIRC_get()? as f32;
        let value = 488 as f32;
        Ok((input / value) as f32)
    }

    fn CECMult_get(&self) -> Result<f32, ClockError> {
        match self.CECMult {
            CECMultConf::HSIDivCEC => return self.HSIDivCEC_get(),
            CECMultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn CECOutput_get(&self) -> Result<f32, ClockError> {
        self.CECMult_get()
    }
    fn FMPI2C1Mult_get(&self) -> Result<f32, ClockError> {
        match self.FMPI2C1Mult {
            FMPI2C1MultConf::HSIRC => return self.HSIRC_get(),
            FMPI2C1MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            FMPI2C1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
        };
    }
    pub fn FMPI2C1output_get(&self) -> Result<f32, ClockError> {
        self.FMPI2C1Mult_get()
    }
    fn USBMult_get(&self) -> Result<f32, ClockError> {
        match self.USBMult {
            USBMultConf::PLLQ => return self.PLLQ_get(),
            USBMultConf::PLLSAIP => return self.PLLSAIP_get(),
        };
    }
    pub fn USBoutput_get(&self) -> Result<f32, ClockError> {
        self.USBMult_get()
    }
    fn SPDIFMult_get(&self) -> Result<f32, ClockError> {
        match self.SPDIFMult {
            SPDIFMultConf::PLLR => return self.PLLR_get(),
            SPDIFMultConf::PLLI2SP => return self.PLLI2SP_get(),
        };
    }
    pub fn SPDIFoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SPDIFMult_get()?;
        if input > (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 0),
                from: ClockNodes::SPDIFMult,
                to: ClockNodes::SPDIFoutput,
            });
        } else if input < (5632000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 5632000),
                from: ClockNodes::SPDIFMult,
                to: ClockNodes::SPDIFoutput,
            });
        }
        Ok(input)
    }
    fn SDIOMult_get(&self) -> Result<f32, ClockError> {
        match self.SDIOMult {
            SDIOMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            SDIOMultConf::USBMult => return self.USBMult_get(),
        };
    }
    pub fn SDIOoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SDIOMult_get()?;
        if input > (48000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48000000),
                from: ClockNodes::SDIOMult,
                to: ClockNodes::SDIOoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SDIOMult,
                to: ClockNodes::SDIOoutput,
            });
        }
        Ok(input)
    }
    fn SAIAMult_get(&self) -> Result<f32, ClockError> {
        match self.SAIAMult {
            SAIAMultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            SAIAMultConf::PLLR => return self.PLLR_get(),
            SAIAMultConf::PLLI2SQDiv => return self.PLLI2SQDiv_get(),
            SAIAMultConf::PLLSAIQDiv => return self.PLLSAIQDiv_get(),
        };
    }
    pub fn SAIAoutput_get(&self) -> Result<f32, ClockError> {
        self.SAIAMult_get()
    }
    fn SAIBMult_get(&self) -> Result<f32, ClockError> {
        match self.SAIBMult {
            SAIBMultConf::PLLSource => return self.PLLSource_get(),
            SAIBMultConf::PLLR => return self.PLLR_get(),
            SAIBMultConf::PLLI2SQDiv => return self.PLLI2SQDiv_get(),
            SAIBMultConf::PLLSAIQDiv => return self.PLLSAIQDiv_get(),
        };
    }
    pub fn SAIBoutput_get(&self) -> Result<f32, ClockError> {
        self.SAIBMult_get()
    }
    fn I2S1Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2S1Mult {
            I2S1MultConf::PLLSource => return self.PLLSource_get(),
            I2S1MultConf::PLLR => return self.PLLR_get(),
            I2S1MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            I2S1MultConf::PLLI2SR => return self.PLLI2SR_get(),
        };
    }
    pub fn I2S1output_get(&self) -> Result<f32, ClockError> {
        let input = self.I2S1Mult_get()?;
        if input > (192000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 192000000),
                from: ClockNodes::I2S1Mult,
                to: ClockNodes::I2S1output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::I2S1Mult,
                to: ClockNodes::I2S1output,
            });
        }
        Ok(input)
    }
    fn I2S2Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2S2Mult {
            I2S2MultConf::PLLSource => return self.PLLSource_get(),
            I2S2MultConf::PLLR => return self.PLLR_get(),
            I2S2MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            I2S2MultConf::PLLI2SR => return self.PLLI2SR_get(),
        };
    }
    pub fn I2S2output_get(&self) -> Result<f32, ClockError> {
        let input = self.I2S2Mult_get()?;
        if input > (192000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 192000000),
                from: ClockNodes::I2S2Mult,
                to: ClockNodes::I2S2output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::I2S2Mult,
                to: ClockNodes::I2S2output,
            });
        }
        Ok(input)
    }
    fn MCO1Mult_get(&self) -> Result<f32, ClockError> {
        match self.MCO1Mult {
            MCO1MultConf::LSEOSC => return self.LSEOSC_get(),
            MCO1MultConf::HSEOSC => return self.HSEOSC_get(),
            MCO1MultConf::HSIRC => return self.HSIRC_get(),
            MCO1MultConf::PLLP => return self.PLLP_get(),
        };
    }
    fn MCO1Div_get(&self) -> Result<f32, ClockError> {
        let input = self.MCO1Mult_get()? as f32;
        let value = self.MCO1Div.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn MCO1Pin_get(&self) -> Result<f32, ClockError> {
        let input = self.MCO1Div_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::MCO1Div,
                to: ClockNodes::MCO1Pin,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::MCO1Div,
                to: ClockNodes::MCO1Pin,
            });
        }
        Ok(input)
    }
    fn MCO2Mult_get(&self) -> Result<f32, ClockError> {
        match self.MCO2Mult {
            MCO2MultConf::SysClkSource => return self.SysClkSource_get(),
            MCO2MultConf::PLLI2SR => return self.PLLI2SR_get(),
            MCO2MultConf::HSEOSC => return self.HSEOSC_get(),
            MCO2MultConf::PLLP => return self.PLLP_get(),
        };
    }
    fn MCO2Div_get(&self) -> Result<f32, ClockError> {
        let input = self.MCO2Mult_get()? as f32;
        let value = self.MCO2Div.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn MCO2Pin_get(&self) -> Result<f32, ClockError> {
        let input = self.MCO2Div_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::MCO2Div,
                to: ClockNodes::MCO2Pin,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::MCO2Div,
                to: ClockNodes::MCO2Pin,
            });
        }
        Ok(input)
    }
    fn AHBPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()? as f32;
        let value = self.AHBPrescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn PWRCLKoutput_get(&self) -> Result<f32, ClockError> {
        self.SysCLKOutput_get()
    }
    pub fn AHBOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBPrescaler_get()?;
        if input > (180000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 180000000),
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
    fn CortexPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.CortexPrescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn CortexSysOutput_get(&self) -> Result<f32, ClockError> {
        self.CortexPrescaler_get()
    }
    pub fn FCLKCortexOutput_get(&self) -> Result<f32, ClockError> {
        self.AHBOutput_get()
    }
    fn APB1Prescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.APB1Prescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB1Output_get(&self) -> Result<f32, ClockError> {
        let input = self.APB1Prescaler_get()?;
        if input > (45000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 45000000),
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
        if input > (90000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 90000000),
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
    fn PLLN_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLM_get()? as f32;
        let value = self.PLLN.get()? as f32;
        Ok((input * value) as f32)
    }

    fn PLLP_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLN_get()? as f32;
        let value = self.PLLP.get()? as f32;
        Ok((input / value) as f32)
    }

    fn PLLQ_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLN_get()? as f32;
        let value = self.PLLQ.get()? as f32;
        Ok((input / value) as f32)
    }

    fn PLLR_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLN_get()? as f32;
        let value = self.PLLR.get()? as f32;
        Ok((input / value) as f32)
    }

    fn PLLSAIN_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLSAIM_get()? as f32;
        let value = self.PLLSAIN.get()? as f32;
        Ok((input * value) as f32)
    }

    fn PLLSAIP_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLSAIN_get()? as f32;
        let value = self.PLLSAIP.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn PLLSAIoutput_get(&self) -> Result<f32, ClockError> {
        self.PLLSAIP_get()
    }
    fn PLLSAIQ_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLSAIN_get()? as f32;
        let value = self.PLLSAIQ.get()? as f32;
        Ok((input / value) as f32)
    }

    fn PLLSAIQDiv_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLSAIQ_get()? as f32;
        let value = self.PLLSAIQDiv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn PLLI2SN_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLI2SM_get()? as f32;
        let value = self.PLLI2SN.get()? as f32;
        Ok((input * value) as f32)
    }

    fn PLLI2SP_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLI2SN_get()? as f32;
        let value = self.PLLI2SP.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn PLLI2Soutput_get(&self) -> Result<f32, ClockError> {
        self.PLLI2SP_get()
    }
    fn PLLI2SQ_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLI2SN_get()? as f32;
        let value = self.PLLI2SQ.get()? as f32;
        Ok((input / value) as f32)
    }

    fn PLLI2SQDiv_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLI2SQ_get()? as f32;
        let value = self.PLLI2SQDiv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn PLLI2SR_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLI2SN_get()? as f32;
        let value = self.PLLI2SR.get()? as f32;
        Ok((input / value) as f32)
    }
}
