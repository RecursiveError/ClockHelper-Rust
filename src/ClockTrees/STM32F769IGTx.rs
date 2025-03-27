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
    HSERTCDevisor,
    RTCClkSource,
    RTCOutput,
    IWDGOutput,
    I2C1Mult,
    I2C1output,
    I2C2Mult,
    I2C2output,
    I2C3Mult,
    I2C3output,
    I2C4Mult,
    I2C4output,
    PLL48Mult,
    RNGoutput,
    USBoutput,
    LCDTFTKOutput,
    SPDIFoutput,
    SAI1Mult,
    SAI1output,
    SAI2Mult,
    SAI2output,
    DFSDMAudioMult,
    DFSDMAudiooutput,
    SDMMCMult,
    SDMMCoutput,
    SDMMC2Mult,
    SDMMC2output,
    I2SMult,
    I2Soutput,
    EthernetPtpOutput,
    MCO1Mult,
    MCO1Div,
    MCO1Pin,
    MCO2Mult,
    MCO2Div,
    MCO2Pin,
    AHBPrescaler,
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
    USART1Mult,
    USART1output,
    USART2Mult,
    USART2output,
    USART3Mult,
    USART3output,
    USART6Mult,
    USART6output,
    UART4Mult,
    UART4output,
    UART5Mult,
    UART5output,
    UART7Mult,
    UART7output,
    UART8Mult,
    UART8output,
    LPTIM1Mult,
    LPTIM1Output,
    HSIDivCEC,
    CECMult,
    CECOutput,
    DFSDMMult,
    DFSDMoutput,
    DSIPHYPrescaler,
    DSIMult,
    DSIoutput,
    DSITXPrescaler,
    DSITXCLKEsc,
    PLLDSIIDF,
    PLLDSIMultiplicator,
    PLLDSINDIV,
    VCOoutput,
    PLLDSIDevisor,
    PLLDSIODF,
    PLLDSIoutput,
    PLLN,
    PLLP,
    PLLQ,
    PLLQoutput,
    PLLR,
    PLLRoutput,
    PLLSAIN,
    PLLSAIP,
    PLLSAIoutput,
    PLLSAIQ,
    PLLSAIQDiv,
    PLLSAIR,
    PLLSAIRDiv,
    PLLI2SN,
    PLLI2SP,
    PLLI2SQ,
    PLLI2SQDiv,
    PLLI2SR,
    PLLI2SRoutput,
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
pub enum LSIRCConf {
    Value(u32),
}

impl LSIRCConf {
    pub const fn min() -> u32 {
        17000
    }

    pub const fn max() -> u32 {
        47000
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            LSIRCConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::LSIRC,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::LSIRC,
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
pub enum I2C1MultConf {
    APB1Prescaler,
    SysCLKOutput,
    HSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C2MultConf {
    APB1Prescaler,
    SysCLKOutput,
    HSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C3MultConf {
    APB1Prescaler,
    SysCLKOutput,
    HSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C4MultConf {
    APB1Prescaler,
    SysCLKOutput,
    HSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL48MultConf {
    PLLQ,
    PLLSAIP,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SAI1MultConf {
    PLLSAIQDiv,
    PLLI2SQDiv,
    I2S_CKIN,
    PLLSource,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SAI2MultConf {
    PLLSAIQDiv,
    PLLI2SQDiv,
    I2S_CKIN,
    PLLSource,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DFSDMAudioMultConf {
    SAI1Mult,
    SAI2Mult,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SDMMCMultConf {
    PLL48Mult,
    SysCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SDMMC2MultConf {
    PLL48Mult,
    SysCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2SMultConf {
    PLLI2SR,
    I2S_CKIN,
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
pub enum USART1MultConf {
    APB2Prescaler,
    SysCLKOutput,
    HSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART2MultConf {
    APB1Prescaler,
    SysCLKOutput,
    HSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART3MultConf {
    APB1Prescaler,
    SysCLKOutput,
    HSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART6MultConf {
    APB2Prescaler,
    SysCLKOutput,
    HSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum UART4MultConf {
    APB1Prescaler,
    SysCLKOutput,
    HSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum UART5MultConf {
    APB1Prescaler,
    SysCLKOutput,
    HSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum UART7MultConf {
    APB1Prescaler,
    SysCLKOutput,
    HSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum UART8MultConf {
    APB1Prescaler,
    SysCLKOutput,
    HSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM1MultConf {
    APB1Prescaler,
    LSIRC,
    HSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CECMultConf {
    HSIDivCEC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DFSDMMultConf {
    APB2Prescaler,
    SysCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DSIMultConf {
    PLLRoutput,
    DSIPHYPrescaler,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DSITXPrescalerConf {
    Value(u32),
}

impl DSITXPrescalerConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        32
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DSITXPrescalerConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DSITXPrescaler,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DSITXPrescaler,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLDSIIDFConf {
    DIV1,
    DIV2,
    DIV3,
    DIV4,
    DIV5,
    DIV6,
    DIV7,
}

impl PLLDSIIDFConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLDSIIDFConf::DIV1 => return Ok(1.0),
            PLLDSIIDFConf::DIV2 => return Ok(2.0),
            PLLDSIIDFConf::DIV3 => return Ok(3.0),
            PLLDSIIDFConf::DIV4 => return Ok(4.0),
            PLLDSIIDFConf::DIV5 => return Ok(5.0),
            PLLDSIIDFConf::DIV6 => return Ok(6.0),
            PLLDSIIDFConf::DIV7 => return Ok(7.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLDSINDIVConf {
    Value(u32),
}

impl PLLDSINDIVConf {
    pub const fn min() -> u32 {
        10
    }

    pub const fn max() -> u32 {
        125
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLDSINDIVConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLDSINDIV,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLDSINDIV,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLDSIODFConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
}

impl PLLDSIODFConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLDSIODFConf::DIV1 => return Ok(1.0),
            PLLDSIODFConf::DIV2 => return Ok(2.0),
            PLLDSIODFConf::DIV4 => return Ok(4.0),
            PLLDSIODFConf::DIV8 => return Ok(8.0),
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
pub enum PLLSAIRConf {
    Value(u32),
}

impl PLLSAIRConf {
    pub const fn min() -> u32 {
        2
    }

    pub const fn max() -> u32 {
        7
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLSAIRConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLSAIR,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLSAIR,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSAIRDivConf {
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl PLLSAIRDivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLSAIRDivConf::DIV2 => return Ok(2.0),
            PLLSAIRDivConf::DIV4 => return Ok(4.0),
            PLLSAIRDivConf::DIV8 => return Ok(8.0),
            PLLSAIRDivConf::DIV16 => return Ok(16.0),
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
    pub LSIRC: LSIRCConf,
    pub LSEOSC: LSEOSCConf,
    pub SysClkSource: SysClkSourceConf,
    pub PLLSource: PLLSourceConf,
    pub PLLM: PLLMConf,
    pub HSERTCDevisor: HSERTCDevisorConf,
    pub RTCClkSource: RTCClkSourceConf,
    pub I2C1Mult: I2C1MultConf,
    pub I2C2Mult: I2C2MultConf,
    pub I2C3Mult: I2C3MultConf,
    pub I2C4Mult: I2C4MultConf,
    pub PLL48Mult: PLL48MultConf,
    pub SAI1Mult: SAI1MultConf,
    pub SAI2Mult: SAI2MultConf,
    pub DFSDMAudioMult: DFSDMAudioMultConf,
    pub SDMMCMult: SDMMCMultConf,
    pub SDMMC2Mult: SDMMC2MultConf,
    pub I2SMult: I2SMultConf,
    pub MCO1Mult: MCO1MultConf,
    pub MCO1Div: MCO1DivConf,
    pub MCO2Mult: MCO2MultConf,
    pub MCO2Div: MCO2DivConf,
    pub AHBPrescaler: AHBPrescalerConf,
    pub CortexPrescaler: CortexPrescalerConf,
    pub APB1Prescaler: APB1PrescalerConf,
    pub APB2Prescaler: APB2PrescalerConf,
    pub USART1Mult: USART1MultConf,
    pub USART2Mult: USART2MultConf,
    pub USART3Mult: USART3MultConf,
    pub USART6Mult: USART6MultConf,
    pub UART4Mult: UART4MultConf,
    pub UART5Mult: UART5MultConf,
    pub UART7Mult: UART7MultConf,
    pub UART8Mult: UART8MultConf,
    pub LPTIM1Mult: LPTIM1MultConf,
    pub CECMult: CECMultConf,
    pub DFSDMMult: DFSDMMultConf,
    pub DSIMult: DSIMultConf,
    pub DSITXPrescaler: DSITXPrescalerConf,
    pub PLLDSIIDF: PLLDSIIDFConf,
    pub PLLDSINDIV: PLLDSINDIVConf,
    pub PLLDSIODF: PLLDSIODFConf,
    pub PLLN: PLLNConf,
    pub PLLP: PLLPConf,
    pub PLLQ: PLLQConf,
    pub PLLR: PLLRConf,
    pub PLLSAIN: PLLSAINConf,
    pub PLLSAIP: PLLSAIPConf,
    pub PLLSAIQ: PLLSAIQConf,
    pub PLLSAIQDiv: PLLSAIQDivConf,
    pub PLLSAIR: PLLSAIRConf,
    pub PLLSAIRDiv: PLLSAIRDivConf,
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
            LSIRC: LSIRCConf::Value(32000),
            LSEOSC: LSEOSCConf::Value(32768),
            SysClkSource: SysClkSourceConf::HSIRC,
            PLLSource: PLLSourceConf::HSIRC,
            PLLM: PLLMConf::Value(16),
            HSERTCDevisor: HSERTCDevisorConf::DIV2,
            RTCClkSource: RTCClkSourceConf::LSIRC,
            I2C1Mult: I2C1MultConf::APB1Prescaler,
            I2C2Mult: I2C2MultConf::APB1Prescaler,
            I2C3Mult: I2C3MultConf::APB1Prescaler,
            I2C4Mult: I2C4MultConf::APB1Prescaler,
            PLL48Mult: PLL48MultConf::PLLQ,
            SAI1Mult: SAI1MultConf::PLLSAIQDiv,
            SAI2Mult: SAI2MultConf::PLLSAIQDiv,
            DFSDMAudioMult: DFSDMAudioMultConf::SAI1Mult,
            SDMMCMult: SDMMCMultConf::SysCLKOutput,
            SDMMC2Mult: SDMMC2MultConf::SysCLKOutput,
            I2SMult: I2SMultConf::PLLI2SR,
            MCO1Mult: MCO1MultConf::HSIRC,
            MCO1Div: MCO1DivConf::DIV1,
            MCO2Mult: MCO2MultConf::SysClkSource,
            MCO2Div: MCO2DivConf::DIV1,
            AHBPrescaler: AHBPrescalerConf::DIV1,
            CortexPrescaler: CortexPrescalerConf::DIV1,
            APB1Prescaler: APB1PrescalerConf::DIV1,
            APB2Prescaler: APB2PrescalerConf::DIV1,
            USART1Mult: USART1MultConf::APB2Prescaler,
            USART2Mult: USART2MultConf::APB1Prescaler,
            USART3Mult: USART3MultConf::APB1Prescaler,
            USART6Mult: USART6MultConf::APB2Prescaler,
            UART4Mult: UART4MultConf::APB1Prescaler,
            UART5Mult: UART5MultConf::APB1Prescaler,
            UART7Mult: UART7MultConf::APB1Prescaler,
            UART8Mult: UART8MultConf::APB1Prescaler,
            LPTIM1Mult: LPTIM1MultConf::APB1Prescaler,
            CECMult: CECMultConf::HSIDivCEC,
            DFSDMMult: DFSDMMultConf::APB2Prescaler,
            DSIMult: DSIMultConf::DSIPHYPrescaler,
            DSITXPrescaler: DSITXPrescalerConf::Value(4),
            PLLDSIIDF: PLLDSIIDFConf::DIV1,
            PLLDSINDIV: PLLDSINDIVConf::Value(20),
            PLLDSIODF: PLLDSIODFConf::DIV1,
            PLLN: PLLNConf::Value(192),
            PLLP: PLLPConf::DIV2,
            PLLQ: PLLQConf::Value(2),
            PLLR: PLLRConf::Value(2),
            PLLSAIN: PLLSAINConf::Value(192),
            PLLSAIP: PLLSAIPConf::DIV2,
            PLLSAIQ: PLLSAIQConf::Value(2),
            PLLSAIQDiv: PLLSAIQDivConf::Value(1),
            PLLSAIR: PLLSAIRConf::Value(2),
            PLLSAIRDiv: PLLSAIRDivConf::DIV2,
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
        self.LSIRC.get()
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
        };
    }
    pub fn SysCLKOutput_get(&self) -> Result<f32, ClockError> {
        self.SysClkSource_get()
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
    fn I2C1Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C1Mult {
            I2C1MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            I2C1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            I2C1MultConf::HSIRC => return self.HSIRC_get(),
        };
    }
    pub fn I2C1output_get(&self) -> Result<f32, ClockError> {
        self.I2C1Mult_get()
    }
    fn I2C2Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C2Mult {
            I2C2MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            I2C2MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            I2C2MultConf::HSIRC => return self.HSIRC_get(),
        };
    }
    pub fn I2C2output_get(&self) -> Result<f32, ClockError> {
        self.I2C2Mult_get()
    }
    fn I2C3Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C3Mult {
            I2C3MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            I2C3MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            I2C3MultConf::HSIRC => return self.HSIRC_get(),
        };
    }
    pub fn I2C3output_get(&self) -> Result<f32, ClockError> {
        self.I2C3Mult_get()
    }
    fn I2C4Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C4Mult {
            I2C4MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            I2C4MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            I2C4MultConf::HSIRC => return self.HSIRC_get(),
        };
    }
    pub fn I2C4output_get(&self) -> Result<f32, ClockError> {
        self.I2C4Mult_get()
    }
    fn PLL48Mult_get(&self) -> Result<f32, ClockError> {
        match self.PLL48Mult {
            PLL48MultConf::PLLQ => return self.PLLQ_get(),
            PLL48MultConf::PLLSAIP => return self.PLLSAIP_get(),
        };
    }
    pub fn RNGoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL48Mult_get()?;
        if input > (48000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48000000),
                from: ClockNodes::PLL48Mult,
                to: ClockNodes::RNGoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::PLL48Mult,
                to: ClockNodes::RNGoutput,
            });
        }
        Ok(input)
    }
    pub fn USBoutput_get(&self) -> Result<f32, ClockError> {
        self.PLL48Mult_get()
    }
    pub fn LCDTFTKOutput_get(&self) -> Result<f32, ClockError> {
        self.PLLSAIRDiv_get()
    }
    pub fn SPDIFoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLI2SP_get()?;
        if input > (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 0),
                from: ClockNodes::PLLI2SP,
                to: ClockNodes::SPDIFoutput,
            });
        } else if input < (5632000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 5632000),
                from: ClockNodes::PLLI2SP,
                to: ClockNodes::SPDIFoutput,
            });
        }
        Ok(input)
    }
    fn SAI1Mult_get(&self) -> Result<f32, ClockError> {
        match self.SAI1Mult {
            SAI1MultConf::PLLSAIQDiv => return self.PLLSAIQDiv_get(),
            SAI1MultConf::PLLI2SQDiv => return self.PLLI2SQDiv_get(),
            SAI1MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            SAI1MultConf::PLLSource => return self.PLLSource_get(),
        };
    }
    pub fn SAI1output_get(&self) -> Result<f32, ClockError> {
        self.SAI1Mult_get()
    }
    fn SAI2Mult_get(&self) -> Result<f32, ClockError> {
        match self.SAI2Mult {
            SAI2MultConf::PLLSAIQDiv => return self.PLLSAIQDiv_get(),
            SAI2MultConf::PLLI2SQDiv => return self.PLLI2SQDiv_get(),
            SAI2MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            SAI2MultConf::PLLSource => return self.PLLSource_get(),
        };
    }
    pub fn SAI2output_get(&self) -> Result<f32, ClockError> {
        self.SAI2Mult_get()
    }
    fn DFSDMAudioMult_get(&self) -> Result<f32, ClockError> {
        match self.DFSDMAudioMult {
            DFSDMAudioMultConf::SAI1Mult => return self.SAI1Mult_get(),
            DFSDMAudioMultConf::SAI2Mult => return self.SAI2Mult_get(),
        };
    }
    pub fn DFSDMAudiooutput_get(&self) -> Result<f32, ClockError> {
        self.DFSDMAudioMult_get()
    }
    fn SDMMCMult_get(&self) -> Result<f32, ClockError> {
        match self.SDMMCMult {
            SDMMCMultConf::PLL48Mult => return self.PLL48Mult_get(),
            SDMMCMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
        };
    }
    pub fn SDMMCoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SDMMCMult_get()?;
        if input > (50000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 50000000),
                from: ClockNodes::SDMMCMult,
                to: ClockNodes::SDMMCoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SDMMCMult,
                to: ClockNodes::SDMMCoutput,
            });
        }
        Ok(input)
    }
    fn SDMMC2Mult_get(&self) -> Result<f32, ClockError> {
        match self.SDMMC2Mult {
            SDMMC2MultConf::PLL48Mult => return self.PLL48Mult_get(),
            SDMMC2MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
        };
    }
    pub fn SDMMC2output_get(&self) -> Result<f32, ClockError> {
        let input = self.SDMMC2Mult_get()?;
        if input > (50000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 50000000),
                from: ClockNodes::SDMMC2Mult,
                to: ClockNodes::SDMMC2output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SDMMC2Mult,
                to: ClockNodes::SDMMC2output,
            });
        }
        Ok(input)
    }
    fn I2SMult_get(&self) -> Result<f32, ClockError> {
        match self.I2SMult {
            I2SMultConf::PLLI2SR => return self.PLLI2SR_get(),
            I2SMultConf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    pub fn I2Soutput_get(&self) -> Result<f32, ClockError> {
        self.I2SMult_get()
    }
    pub fn EthernetPtpOutput_get(&self) -> Result<f32, ClockError> {
        self.AHBPrescaler_get()
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
        self.MCO1Div_get()
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
        self.MCO2Div_get()
    }
    fn AHBPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()? as f32;
        let value = self.AHBPrescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn AHBOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBPrescaler_get()?;
        if input > (216000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 216000000),
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
        if input > (54000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 54000000),
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
        if input > (108000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 108000000),
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
    fn USART1Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART1Mult {
            USART1MultConf::APB2Prescaler => return self.APB2Prescaler_get(),
            USART1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            USART1MultConf::HSIRC => return self.HSIRC_get(),
            USART1MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn USART1output_get(&self) -> Result<f32, ClockError> {
        self.USART1Mult_get()
    }
    fn USART2Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART2Mult {
            USART2MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            USART2MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            USART2MultConf::HSIRC => return self.HSIRC_get(),
            USART2MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn USART2output_get(&self) -> Result<f32, ClockError> {
        self.USART2Mult_get()
    }
    fn USART3Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART3Mult {
            USART3MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            USART3MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            USART3MultConf::HSIRC => return self.HSIRC_get(),
            USART3MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn USART3output_get(&self) -> Result<f32, ClockError> {
        self.USART3Mult_get()
    }
    fn USART6Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART6Mult {
            USART6MultConf::APB2Prescaler => return self.APB2Prescaler_get(),
            USART6MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            USART6MultConf::HSIRC => return self.HSIRC_get(),
            USART6MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn USART6output_get(&self) -> Result<f32, ClockError> {
        self.USART6Mult_get()
    }
    fn UART4Mult_get(&self) -> Result<f32, ClockError> {
        match self.UART4Mult {
            UART4MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            UART4MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            UART4MultConf::HSIRC => return self.HSIRC_get(),
            UART4MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn UART4output_get(&self) -> Result<f32, ClockError> {
        self.UART4Mult_get()
    }
    fn UART5Mult_get(&self) -> Result<f32, ClockError> {
        match self.UART5Mult {
            UART5MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            UART5MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            UART5MultConf::HSIRC => return self.HSIRC_get(),
            UART5MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn UART5output_get(&self) -> Result<f32, ClockError> {
        self.UART5Mult_get()
    }
    fn UART7Mult_get(&self) -> Result<f32, ClockError> {
        match self.UART7Mult {
            UART7MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            UART7MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            UART7MultConf::HSIRC => return self.HSIRC_get(),
            UART7MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn UART7output_get(&self) -> Result<f32, ClockError> {
        self.UART7Mult_get()
    }
    fn UART8Mult_get(&self) -> Result<f32, ClockError> {
        match self.UART8Mult {
            UART8MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            UART8MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            UART8MultConf::HSIRC => return self.HSIRC_get(),
            UART8MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn UART8output_get(&self) -> Result<f32, ClockError> {
        self.UART8Mult_get()
    }
    fn LPTIM1Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM1Mult {
            LPTIM1MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            LPTIM1MultConf::LSIRC => return self.LSIRC_get(),
            LPTIM1MultConf::HSIRC => return self.HSIRC_get(),
            LPTIM1MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn LPTIM1Output_get(&self) -> Result<f32, ClockError> {
        self.LPTIM1Mult_get()
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
    fn DFSDMMult_get(&self) -> Result<f32, ClockError> {
        match self.DFSDMMult {
            DFSDMMultConf::APB2Prescaler => return self.APB2Prescaler_get(),
            DFSDMMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
        };
    }
    pub fn DFSDMoutput_get(&self) -> Result<f32, ClockError> {
        self.DFSDMMult_get()
    }
    fn DSIPHYPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLDSIODF_get()? as f32;
        let value = 8 as f32;
        Ok((input / value) as f32)
    }

    fn DSIMult_get(&self) -> Result<f32, ClockError> {
        match self.DSIMult {
            DSIMultConf::PLLRoutput => return self.PLLRoutput_get(),
            DSIMultConf::DSIPHYPrescaler => return self.DSIPHYPrescaler_get(),
        };
    }
    pub fn DSIoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.DSIMult_get()?;
        if input > (62500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 62500000),
                from: ClockNodes::DSIMult,
                to: ClockNodes::DSIoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::DSIMult,
                to: ClockNodes::DSIoutput,
            });
        }
        Ok(input)
    }
    fn DSITXPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.DSIMult_get()? as f32;
        let value = self.DSITXPrescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn DSITXCLKEsc_get(&self) -> Result<f32, ClockError> {
        let input = self.DSITXPrescaler_get()?;
        if input > (20000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 20000000),
                from: ClockNodes::DSITXPrescaler,
                to: ClockNodes::DSITXCLKEsc,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::DSITXPrescaler,
                to: ClockNodes::DSITXCLKEsc,
            });
        }
        Ok(input)
    }
    fn PLLDSIIDF_get(&self) -> Result<f32, ClockError> {
        let input = self.HSEOSC_get()? as f32;
        let value = self.PLLDSIIDF.get()? as f32;
        Ok((input / value) as f32)
    }

    fn PLLDSIMultiplicator_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLDSIIDF_get()? as f32;
        let value = 2 as f32;
        Ok((input * value) as f32)
    }

    fn PLLDSINDIV_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLDSIMultiplicator_get()? as f32;
        let value = self.PLLDSINDIV.get()? as f32;
        Ok((input * value) as f32)
    }

    pub fn VCOoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLDSINDIV_get()?;
        if input > (1000000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1000000000),
                from: ClockNodes::PLLDSINDIV,
                to: ClockNodes::VCOoutput,
            });
        } else if input < (500000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 500000000),
                from: ClockNodes::PLLDSINDIV,
                to: ClockNodes::VCOoutput,
            });
        }
        Ok(input)
    }
    fn PLLDSIDevisor_get(&self) -> Result<f32, ClockError> {
        let input = self.VCOoutput_get()? as f32;
        let value = 2 as f32;
        Ok((input / value) as f32)
    }

    fn PLLDSIODF_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLDSIDevisor_get()? as f32;
        let value = self.PLLDSIODF.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn PLLDSIoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLDSIODF_get()?;
        if input > (500000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 500000000),
                from: ClockNodes::PLLDSIODF,
                to: ClockNodes::PLLDSIoutput,
            });
        } else if input < (80000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 80000000),
                from: ClockNodes::PLLDSIODF,
                to: ClockNodes::PLLDSIoutput,
            });
        }
        Ok(input)
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

    pub fn PLLQoutput_get(&self) -> Result<f32, ClockError> {
        self.PLLQ_get()
    }
    fn PLLR_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLN_get()? as f32;
        let value = self.PLLR.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn PLLRoutput_get(&self) -> Result<f32, ClockError> {
        self.PLLR_get()
    }
    fn PLLSAIN_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLM_get()? as f32;
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

    fn PLLSAIR_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLSAIN_get()? as f32;
        let value = self.PLLSAIR.get()? as f32;
        Ok((input / value) as f32)
    }

    fn PLLSAIRDiv_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLSAIR_get()? as f32;
        let value = self.PLLSAIRDiv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn PLLI2SN_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLM_get()? as f32;
        let value = self.PLLI2SN.get()? as f32;
        Ok((input * value) as f32)
    }

    fn PLLI2SP_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLI2SN_get()? as f32;
        let value = self.PLLI2SP.get()? as f32;
        Ok((input / value) as f32)
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

    pub fn PLLI2SRoutput_get(&self) -> Result<f32, ClockError> {
        self.PLLI2SR_get()
    }
}
