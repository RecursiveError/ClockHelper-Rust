#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ClockNodes {
    None,
    HSIRC,
    HSIDiv,
    HSEOSC,
    LSIRC,
    LSEOSC,
    CSIRC,
    HSI48,
    I2S_CKIN,
    traceClkSource,
    TraceCLKOutput,
    SysClkSource,
    SysCLKOutput,
    MCO1Mult,
    MCO1Div,
    MCO1Pin,
    MCO2Mult,
    MCO2Div,
    MCO2Pin,
    D1CPRE,
    D1CPREOutput,
    CpuClockOutput,
    CortexPrescaler,
    CortexSysOutput,
    HPRE,
    AHBOutput,
    AXIClockOutput,
    HCLK3Output,
    D1PPRE,
    APB3Output,
    D2PPRE1,
    Tim1Mul,
    Tim1Output,
    AHB12Output,
    APB1Output,
    D2PPRE2,
    APB2Output,
    Tim2Mul,
    Tim2Output,
    AHB4Output,
    D3PPRE,
    APB4Output,
    PLLSource,
    CKPERSource,
    CKPERoutput,
    DIVM1,
    DIVM2,
    DIVM3,
    DIVN1,
    PLLFRACN,
    DIVP1,
    DIVQ1,
    DIVQ1output,
    DIVR1,
    DIVR1output,
    DIVN2,
    PLL2FRACN,
    DIVP2,
    DIVP2output,
    DIVQ2,
    DIVQ2output,
    DIVR2,
    DIVR2output,
    DIVN3,
    DIVP3,
    PLL3FRACN,
    DIVP3output,
    DIVQ3,
    DIVQ3output,
    DIVR3,
    LTDCOutput,
    DIVR3output,
    HSERTCDevisor,
    RTCClkSource,
    RTCOutput,
    IWDGOutput,
    SPI123Mult,
    SPI123output,
    SAI1Mult,
    DFSDMACLKoutput,
    SAI1output,
    SAI4BMult,
    SAI4Boutput,
    SAI4AMult,
    SAI4Aoutput,
    RNGMult,
    RNGoutput,
    I2C1235Mult,
    I2C1235output,
    I2C4Mult,
    I2C4output,
    SPDIFMult,
    SPDIFoutput,
    QSPIMult,
    QSPIoutput,
    FMCMult,
    FMCoutput,
    SWPMult,
    SWPoutput,
    SDMMCMult,
    SDMMCoutput,
    DFSDMMult,
    DFSDMoutput,
    USART16Mult,
    USART16output,
    USART234578Mult,
    USART234578output,
    LPUART1Mult,
    LPUART1output,
    LPTIM1Mult,
    LPTIM1output,
    LPTIM345Mult,
    LPTIM345output,
    LPTIM2Mult,
    LPTIM2output,
    SPI6Mult,
    SPI6output,
    SPI45Mult,
    SPI45output,
    USBMult,
    USBoutput,
    FDCANMult,
    FDCANoutput,
    ADCMult,
    ADCoutput,
    CECCSIDevisor,
    CECMult,
    CECoutput,
    HrtimMult,
    HRTIMoutput,
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
pub enum HSIDivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
}

impl HSIDivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            HSIDivConf::DIV1 => return Ok(1.0),
            HSIDivConf::DIV2 => return Ok(2.0),
            HSIDivConf::DIV4 => return Ok(4.0),
            HSIDivConf::DIV8 => return Ok(8.0),
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
        50000000
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
        31400
    }

    pub const fn max() -> u32 {
        32600
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
pub enum traceClkSourceConf {
    HSIDiv,
    CSIRC,
    HSEOSC,
    DIVR1,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SysClkSourceConf {
    HSIDiv,
    CSIRC,
    HSEOSC,
    DIVP1,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCO1MultConf {
    LSEOSC,
    HSEOSC,
    HSIDiv,
    HSI48,
    DIVQ1,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCO1DivConf {
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
}

impl MCO1DivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MCO1DivConf::DIV1 => return Ok(1.0),
            MCO1DivConf::DIV2 => return Ok(2.0),
            MCO1DivConf::DIV3 => return Ok(3.0),
            MCO1DivConf::DIV4 => return Ok(4.0),
            MCO1DivConf::DIV5 => return Ok(5.0),
            MCO1DivConf::DIV6 => return Ok(6.0),
            MCO1DivConf::DIV7 => return Ok(7.0),
            MCO1DivConf::DIV8 => return Ok(8.0),
            MCO1DivConf::DIV9 => return Ok(9.0),
            MCO1DivConf::DIV10 => return Ok(10.0),
            MCO1DivConf::DIV11 => return Ok(11.0),
            MCO1DivConf::DIV12 => return Ok(12.0),
            MCO1DivConf::DIV13 => return Ok(13.0),
            MCO1DivConf::DIV14 => return Ok(14.0),
            MCO1DivConf::DIV15 => return Ok(15.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCO2MultConf {
    SysCLKOutput,
    DIVP2,
    HSEOSC,
    DIVP1,
    CSIRC,
    LSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCO2DivConf {
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
}

impl MCO2DivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MCO2DivConf::DIV1 => return Ok(1.0),
            MCO2DivConf::DIV2 => return Ok(2.0),
            MCO2DivConf::DIV3 => return Ok(3.0),
            MCO2DivConf::DIV4 => return Ok(4.0),
            MCO2DivConf::DIV5 => return Ok(5.0),
            MCO2DivConf::DIV6 => return Ok(6.0),
            MCO2DivConf::DIV7 => return Ok(7.0),
            MCO2DivConf::DIV8 => return Ok(8.0),
            MCO2DivConf::DIV9 => return Ok(9.0),
            MCO2DivConf::DIV10 => return Ok(10.0),
            MCO2DivConf::DIV11 => return Ok(11.0),
            MCO2DivConf::DIV12 => return Ok(12.0),
            MCO2DivConf::DIV13 => return Ok(13.0),
            MCO2DivConf::DIV14 => return Ok(14.0),
            MCO2DivConf::DIV15 => return Ok(15.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum D1CPREConf {
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

impl D1CPREConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            D1CPREConf::DIV1 => return Ok(1.0),
            D1CPREConf::DIV2 => return Ok(2.0),
            D1CPREConf::DIV4 => return Ok(4.0),
            D1CPREConf::DIV8 => return Ok(8.0),
            D1CPREConf::DIV16 => return Ok(16.0),
            D1CPREConf::DIV64 => return Ok(64.0),
            D1CPREConf::DIV128 => return Ok(128.0),
            D1CPREConf::DIV256 => return Ok(256.0),
            D1CPREConf::DIV512 => return Ok(512.0),
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
pub enum HPREConf {
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

impl HPREConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            HPREConf::DIV1 => return Ok(1.0),
            HPREConf::DIV2 => return Ok(2.0),
            HPREConf::DIV4 => return Ok(4.0),
            HPREConf::DIV8 => return Ok(8.0),
            HPREConf::DIV16 => return Ok(16.0),
            HPREConf::DIV64 => return Ok(64.0),
            HPREConf::DIV128 => return Ok(128.0),
            HPREConf::DIV256 => return Ok(256.0),
            HPREConf::DIV512 => return Ok(512.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum D1PPREConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl D1PPREConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            D1PPREConf::DIV1 => return Ok(1.0),
            D1PPREConf::DIV2 => return Ok(2.0),
            D1PPREConf::DIV4 => return Ok(4.0),
            D1PPREConf::DIV8 => return Ok(8.0),
            D1PPREConf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum D2PPRE1Conf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl D2PPRE1Conf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            D2PPRE1Conf::DIV1 => return Ok(1.0),
            D2PPRE1Conf::DIV2 => return Ok(2.0),
            D2PPRE1Conf::DIV4 => return Ok(4.0),
            D2PPRE1Conf::DIV8 => return Ok(8.0),
            D2PPRE1Conf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum D2PPRE2Conf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl D2PPRE2Conf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            D2PPRE2Conf::DIV1 => return Ok(1.0),
            D2PPRE2Conf::DIV2 => return Ok(2.0),
            D2PPRE2Conf::DIV4 => return Ok(4.0),
            D2PPRE2Conf::DIV8 => return Ok(8.0),
            D2PPRE2Conf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum D3PPREConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl D3PPREConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            D3PPREConf::DIV1 => return Ok(1.0),
            D3PPREConf::DIV2 => return Ok(2.0),
            D3PPREConf::DIV4 => return Ok(4.0),
            D3PPREConf::DIV8 => return Ok(8.0),
            D3PPREConf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSourceConf {
    HSIDiv,
    CSIRC,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CKPERSourceConf {
    HSIDiv,
    CSIRC,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DIVM1Conf {
    Value(u32),
}

impl DIVM1Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        63
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVM1Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVM1,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVM1,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DIVM2Conf {
    Value(u32),
}

impl DIVM2Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        63
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVM2Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVM2,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVM2,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DIVM3Conf {
    Value(u32),
}

impl DIVM3Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        63
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVM3Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVM3,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVM3,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DIVN1Conf {
    Value(u32),
}

impl DIVN1Conf {
    pub const fn min() -> u32 {
        4
    }

    pub const fn max() -> u32 {
        512
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVN1Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVN1,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVN1,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLFRACNConf {
    Value(u32),
}

impl PLLFRACNConf {
    pub const fn min() -> u32 {
        0
    }

    pub const fn max() -> u32 {
        8191
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLFRACNConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLFRACN,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLFRACN,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DIVP1Conf {
    Value(u32),
}

impl DIVP1Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        128
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVP1Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVP1,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVP1,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DIVQ1Conf {
    Value(u32),
}

impl DIVQ1Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        128
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVQ1Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVQ1,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVQ1,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DIVR1Conf {
    Value(u32),
}

impl DIVR1Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        128
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVR1Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVR1,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVR1,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DIVN2Conf {
    Value(u32),
}

impl DIVN2Conf {
    pub const fn min() -> u32 {
        4
    }

    pub const fn max() -> u32 {
        512
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVN2Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVN2,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVN2,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL2FRACNConf {
    Value(u32),
}

impl PLL2FRACNConf {
    pub const fn min() -> u32 {
        0
    }

    pub const fn max() -> u32 {
        8191
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL2FRACNConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL2FRACN,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL2FRACN,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DIVP2Conf {
    Value(u32),
}

impl DIVP2Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        128
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVP2Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVP2,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVP2,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DIVQ2Conf {
    Value(u32),
}

impl DIVQ2Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        128
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVQ2Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVQ2,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVQ2,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DIVR2Conf {
    Value(u32),
}

impl DIVR2Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        128
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVR2Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVR2,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVR2,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DIVN3Conf {
    Value(u32),
}

impl DIVN3Conf {
    pub const fn min() -> u32 {
        4
    }

    pub const fn max() -> u32 {
        512
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVN3Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVN3,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVN3,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DIVP3Conf {
    Value(u32),
}

impl DIVP3Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        128
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVP3Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVP3,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVP3,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL3FRACNConf {
    Value(u32),
}

impl PLL3FRACNConf {
    pub const fn min() -> u32 {
        0
    }

    pub const fn max() -> u32 {
        8191
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL3FRACNConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL3FRACN,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL3FRACN,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DIVQ3Conf {
    Value(u32),
}

impl DIVQ3Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        128
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVQ3Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVQ3,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVQ3,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DIVR3Conf {
    Value(u32),
}

impl DIVR3Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        128
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVR3Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVR3,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVR3,
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
    DIV32,
    DIV33,
    DIV34,
    DIV35,
    DIV36,
    DIV37,
    DIV38,
    DIV39,
    DIV40,
    DIV41,
    DIV42,
    DIV43,
    DIV44,
    DIV45,
    DIV46,
    DIV47,
    DIV48,
    DIV49,
    DIV50,
    DIV51,
    DIV52,
    DIV53,
    DIV54,
    DIV55,
    DIV56,
    DIV57,
    DIV58,
    DIV59,
    DIV60,
    DIV61,
    DIV62,
    DIV63,
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
            HSERTCDevisorConf::DIV32 => return Ok(32.0),
            HSERTCDevisorConf::DIV33 => return Ok(33.0),
            HSERTCDevisorConf::DIV34 => return Ok(34.0),
            HSERTCDevisorConf::DIV35 => return Ok(35.0),
            HSERTCDevisorConf::DIV36 => return Ok(36.0),
            HSERTCDevisorConf::DIV37 => return Ok(37.0),
            HSERTCDevisorConf::DIV38 => return Ok(38.0),
            HSERTCDevisorConf::DIV39 => return Ok(39.0),
            HSERTCDevisorConf::DIV40 => return Ok(40.0),
            HSERTCDevisorConf::DIV41 => return Ok(41.0),
            HSERTCDevisorConf::DIV42 => return Ok(42.0),
            HSERTCDevisorConf::DIV43 => return Ok(43.0),
            HSERTCDevisorConf::DIV44 => return Ok(44.0),
            HSERTCDevisorConf::DIV45 => return Ok(45.0),
            HSERTCDevisorConf::DIV46 => return Ok(46.0),
            HSERTCDevisorConf::DIV47 => return Ok(47.0),
            HSERTCDevisorConf::DIV48 => return Ok(48.0),
            HSERTCDevisorConf::DIV49 => return Ok(49.0),
            HSERTCDevisorConf::DIV50 => return Ok(50.0),
            HSERTCDevisorConf::DIV51 => return Ok(51.0),
            HSERTCDevisorConf::DIV52 => return Ok(52.0),
            HSERTCDevisorConf::DIV53 => return Ok(53.0),
            HSERTCDevisorConf::DIV54 => return Ok(54.0),
            HSERTCDevisorConf::DIV55 => return Ok(55.0),
            HSERTCDevisorConf::DIV56 => return Ok(56.0),
            HSERTCDevisorConf::DIV57 => return Ok(57.0),
            HSERTCDevisorConf::DIV58 => return Ok(58.0),
            HSERTCDevisorConf::DIV59 => return Ok(59.0),
            HSERTCDevisorConf::DIV60 => return Ok(60.0),
            HSERTCDevisorConf::DIV61 => return Ok(61.0),
            HSERTCDevisorConf::DIV62 => return Ok(62.0),
            HSERTCDevisorConf::DIV63 => return Ok(63.0),
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
pub enum SPI123MultConf {
    DIVQ1,
    DIVP2,
    DIVP3,
    I2S_CKIN,
    CKPERSource,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SAI1MultConf {
    DIVQ1,
    DIVP2,
    DIVP3,
    I2S_CKIN,
    CKPERSource,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SAI4BMultConf {
    DIVQ1,
    DIVP2,
    DIVP3,
    I2S_CKIN,
    CKPERSource,
    SPDIFMult,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SAI4AMultConf {
    DIVQ1,
    DIVP2,
    DIVP3,
    I2S_CKIN,
    CKPERSource,
    SPDIFMult,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum RNGMultConf {
    HSI48,
    DIVQ1,
    LSEOSC,
    LSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C1235MultConf {
    D2PPRE1,
    DIVR3,
    HSIDiv,
    CSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C4MultConf {
    D3PPRE,
    DIVR3,
    HSIDiv,
    CSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPDIFMultConf {
    DIVQ1,
    DIVR2,
    DIVR3,
    HSIDiv,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum QSPIMultConf {
    HCLK3Output,
    DIVQ1,
    DIVR2,
    CKPERSource,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FMCMultConf {
    HCLK3Output,
    DIVQ1,
    DIVR2,
    CKPERSource,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SWPMultConf {
    D2PPRE1,
    HSIDiv,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SDMMCMultConf {
    DIVQ1,
    DIVR2,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DFSDMMultConf {
    D2PPRE2,
    SysCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART16MultConf {
    D2PPRE2,
    DIVQ2,
    DIVQ3,
    HSIDiv,
    CSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART234578MultConf {
    D2PPRE1,
    DIVQ2,
    DIVQ3,
    HSIDiv,
    CSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPUART1MultConf {
    D1PPRE,
    DIVQ2,
    DIVQ3,
    HSIDiv,
    CSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM1MultConf {
    D2PPRE1,
    DIVP2,
    DIVR3,
    LSEOSC,
    LSIRC,
    CKPERSource,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM345MultConf {
    D3PPRE,
    DIVP2,
    DIVR3,
    LSEOSC,
    LSIRC,
    CKPERSource,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM2MultConf {
    D3PPRE,
    DIVP2,
    DIVR3,
    LSEOSC,
    LSIRC,
    CKPERSource,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI6MultConf {
    D3PPRE,
    DIVQ2,
    DIVQ3,
    HSIDiv,
    CSIRC,
    HSEOSC,
    I2S_CKIN,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI45MultConf {
    D2PPRE2,
    DIVQ2,
    DIVQ3,
    HSIDiv,
    CSIRC,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USBMultConf {
    DIVQ1,
    DIVQ3,
    HSI48,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FDCANMultConf {
    HSEOSC,
    DIVQ1,
    DIVQ2,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ADCMultConf {
    DIVP2,
    DIVR3,
    CKPERSource,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CECMultConf {
    LSEOSC,
    LSIRC,
    CECCSIDevisor,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum HrtimMultConf {
    Tim2Output,
    D1CPRE,
}
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ClockTree {
    pub HSIDiv: HSIDivConf,
    pub HSEOSC: HSEOSCConf,
    pub LSIRC: LSIRCConf,
    pub LSEOSC: LSEOSCConf,
    pub traceClkSource: traceClkSourceConf,
    pub SysClkSource: SysClkSourceConf,
    pub MCO1Mult: MCO1MultConf,
    pub MCO1Div: MCO1DivConf,
    pub MCO2Mult: MCO2MultConf,
    pub MCO2Div: MCO2DivConf,
    pub D1CPRE: D1CPREConf,
    pub CortexPrescaler: CortexPrescalerConf,
    pub HPRE: HPREConf,
    pub D1PPRE: D1PPREConf,
    pub D2PPRE1: D2PPRE1Conf,
    pub D2PPRE2: D2PPRE2Conf,
    pub D3PPRE: D3PPREConf,
    pub PLLSource: PLLSourceConf,
    pub CKPERSource: CKPERSourceConf,
    pub DIVM1: DIVM1Conf,
    pub DIVM2: DIVM2Conf,
    pub DIVM3: DIVM3Conf,
    pub DIVN1: DIVN1Conf,
    pub PLLFRACN: PLLFRACNConf,
    pub DIVP1: DIVP1Conf,
    pub DIVQ1: DIVQ1Conf,
    pub DIVR1: DIVR1Conf,
    pub DIVN2: DIVN2Conf,
    pub PLL2FRACN: PLL2FRACNConf,
    pub DIVP2: DIVP2Conf,
    pub DIVQ2: DIVQ2Conf,
    pub DIVR2: DIVR2Conf,
    pub DIVN3: DIVN3Conf,
    pub DIVP3: DIVP3Conf,
    pub PLL3FRACN: PLL3FRACNConf,
    pub DIVQ3: DIVQ3Conf,
    pub DIVR3: DIVR3Conf,
    pub HSERTCDevisor: HSERTCDevisorConf,
    pub RTCClkSource: RTCClkSourceConf,
    pub SPI123Mult: SPI123MultConf,
    pub SAI1Mult: SAI1MultConf,
    pub SAI4BMult: SAI4BMultConf,
    pub SAI4AMult: SAI4AMultConf,
    pub RNGMult: RNGMultConf,
    pub I2C1235Mult: I2C1235MultConf,
    pub I2C4Mult: I2C4MultConf,
    pub SPDIFMult: SPDIFMultConf,
    pub QSPIMult: QSPIMultConf,
    pub FMCMult: FMCMultConf,
    pub SWPMult: SWPMultConf,
    pub SDMMCMult: SDMMCMultConf,
    pub DFSDMMult: DFSDMMultConf,
    pub USART16Mult: USART16MultConf,
    pub USART234578Mult: USART234578MultConf,
    pub LPUART1Mult: LPUART1MultConf,
    pub LPTIM1Mult: LPTIM1MultConf,
    pub LPTIM345Mult: LPTIM345MultConf,
    pub LPTIM2Mult: LPTIM2MultConf,
    pub SPI6Mult: SPI6MultConf,
    pub SPI45Mult: SPI45MultConf,
    pub USBMult: USBMultConf,
    pub FDCANMult: FDCANMultConf,
    pub ADCMult: ADCMultConf,
    pub CECMult: CECMultConf,
    pub HrtimMult: HrtimMultConf,
}

impl Default for ClockTree {
    fn default() -> Self {
        Self {
            HSIDiv: HSIDivConf::DIV1,
            HSEOSC: HSEOSCConf::Value(25000000),
            LSIRC: LSIRCConf::Value(32000),
            LSEOSC: LSEOSCConf::Value(32768),
            traceClkSource: traceClkSourceConf::HSIDiv,
            SysClkSource: SysClkSourceConf::HSIDiv,
            MCO1Mult: MCO1MultConf::HSIDiv,
            MCO1Div: MCO1DivConf::DIV1,
            MCO2Mult: MCO2MultConf::SysCLKOutput,
            MCO2Div: MCO2DivConf::DIV1,
            D1CPRE: D1CPREConf::DIV1,
            CortexPrescaler: CortexPrescalerConf::DIV1,
            HPRE: HPREConf::DIV1,
            D1PPRE: D1PPREConf::DIV1,
            D2PPRE1: D2PPRE1Conf::DIV1,
            D2PPRE2: D2PPRE2Conf::DIV1,
            D3PPRE: D3PPREConf::DIV1,
            PLLSource: PLLSourceConf::HSIDiv,
            CKPERSource: CKPERSourceConf::HSIDiv,
            DIVM1: DIVM1Conf::Value(32),
            DIVM2: DIVM2Conf::Value(32),
            DIVM3: DIVM3Conf::Value(32),
            DIVN1: DIVN1Conf::Value(129),
            PLLFRACN: PLLFRACNConf::Value(0),
            DIVP1: DIVP1Conf::Value(2),
            DIVQ1: DIVQ1Conf::Value(2),
            DIVR1: DIVR1Conf::Value(2),
            DIVN2: DIVN2Conf::Value(129),
            PLL2FRACN: PLL2FRACNConf::Value(0),
            DIVP2: DIVP2Conf::Value(2),
            DIVQ2: DIVQ2Conf::Value(2),
            DIVR2: DIVR2Conf::Value(2),
            DIVN3: DIVN3Conf::Value(129),
            DIVP3: DIVP3Conf::Value(2),
            PLL3FRACN: PLL3FRACNConf::Value(0),
            DIVQ3: DIVQ3Conf::Value(2),
            DIVR3: DIVR3Conf::Value(2),
            HSERTCDevisor: HSERTCDevisorConf::DIV2,
            RTCClkSource: RTCClkSourceConf::LSIRC,
            SPI123Mult: SPI123MultConf::DIVQ1,
            SAI1Mult: SAI1MultConf::DIVQ1,
            SAI4BMult: SAI4BMultConf::DIVQ1,
            SAI4AMult: SAI4AMultConf::DIVQ1,
            RNGMult: RNGMultConf::HSI48,
            I2C1235Mult: I2C1235MultConf::D2PPRE1,
            I2C4Mult: I2C4MultConf::D3PPRE,
            SPDIFMult: SPDIFMultConf::DIVQ1,
            QSPIMult: QSPIMultConf::HCLK3Output,
            FMCMult: FMCMultConf::HCLK3Output,
            SWPMult: SWPMultConf::D2PPRE1,
            SDMMCMult: SDMMCMultConf::DIVQ1,
            DFSDMMult: DFSDMMultConf::D2PPRE2,
            USART16Mult: USART16MultConf::D2PPRE2,
            USART234578Mult: USART234578MultConf::D2PPRE1,
            LPUART1Mult: LPUART1MultConf::D1PPRE,
            LPTIM1Mult: LPTIM1MultConf::D2PPRE1,
            LPTIM345Mult: LPTIM345MultConf::D3PPRE,
            LPTIM2Mult: LPTIM2MultConf::D3PPRE,
            SPI6Mult: SPI6MultConf::D3PPRE,
            SPI45Mult: SPI45MultConf::D2PPRE2,
            USBMult: USBMultConf::DIVQ1,
            FDCANMult: FDCANMultConf::DIVQ1,
            ADCMult: ADCMultConf::DIVP2,
            CECMult: CECMultConf::LSIRC,
            HrtimMult: HrtimMultConf::Tim2Output,
        }
    }
}
impl ClockTree {
    pub fn HSIRC_get(&self) -> Result<f32, ClockError> {
        Ok(64000000 as f32)
    }
    fn HSIDiv_get(&self) -> Result<f32, ClockError> {
        let input = self.HSIRC_get()? as f32;
        let value = self.HSIDiv.get()? as f32;
        Ok((input / value) as f32)
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
    pub fn CSIRC_get(&self) -> Result<f32, ClockError> {
        Ok(4000000 as f32)
    }
    pub fn HSI48_get(&self) -> Result<f32, ClockError> {
        Ok(48000000 as f32)
    }
    pub fn I2S_CKIN_get(&self) -> Result<f32, ClockError> {
        Ok(12288000 as f32)
    }
    fn traceClkSource_get(&self) -> Result<f32, ClockError> {
        match self.traceClkSource {
            traceClkSourceConf::HSIDiv => return self.HSIDiv_get(),
            traceClkSourceConf::CSIRC => return self.CSIRC_get(),
            traceClkSourceConf::HSEOSC => return self.HSEOSC_get(),
            traceClkSourceConf::DIVR1 => return self.DIVR1_get(),
        };
    }
    pub fn TraceCLKOutput_get(&self) -> Result<f32, ClockError> {
        self.traceClkSource_get()
    }
    fn SysClkSource_get(&self) -> Result<f32, ClockError> {
        match self.SysClkSource {
            SysClkSourceConf::HSIDiv => return self.HSIDiv_get(),
            SysClkSourceConf::CSIRC => return self.CSIRC_get(),
            SysClkSourceConf::HSEOSC => return self.HSEOSC_get(),
            SysClkSourceConf::DIVP1 => return self.DIVP1_get(),
        };
    }
    pub fn SysCLKOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SysClkSource_get()?;
        if input > (550000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 550000000),
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
    fn MCO1Mult_get(&self) -> Result<f32, ClockError> {
        match self.MCO1Mult {
            MCO1MultConf::LSEOSC => return self.LSEOSC_get(),
            MCO1MultConf::HSEOSC => return self.HSEOSC_get(),
            MCO1MultConf::HSIDiv => return self.HSIDiv_get(),
            MCO1MultConf::HSI48 => return self.HSI48_get(),
            MCO1MultConf::DIVQ1 => return self.DIVQ1_get(),
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
            MCO2MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            MCO2MultConf::DIVP2 => return self.DIVP2_get(),
            MCO2MultConf::HSEOSC => return self.HSEOSC_get(),
            MCO2MultConf::DIVP1 => return self.DIVP1_get(),
            MCO2MultConf::CSIRC => return self.CSIRC_get(),
            MCO2MultConf::LSIRC => return self.LSIRC_get(),
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
    fn D1CPRE_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()? as f32;
        let value = self.D1CPRE.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn D1CPREOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.D1CPRE_get()?;
        if input > (550000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 550000000),
                from: ClockNodes::D1CPRE,
                to: ClockNodes::D1CPREOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::D1CPRE,
                to: ClockNodes::D1CPREOutput,
            });
        }
        Ok(input)
    }
    pub fn CpuClockOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.D1CPRE_get()?;
        if input > (550000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 550000000),
                from: ClockNodes::D1CPRE,
                to: ClockNodes::CpuClockOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::D1CPRE,
                to: ClockNodes::CpuClockOutput,
            });
        }
        Ok(input)
    }
    fn CortexPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.D1CPRE_get()? as f32;
        let value = self.CortexPrescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn CortexSysOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.CortexPrescaler_get()?;
        if input > (550000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 550000000),
                from: ClockNodes::CortexPrescaler,
                to: ClockNodes::CortexSysOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::CortexPrescaler,
                to: ClockNodes::CortexSysOutput,
            });
        }
        Ok(input)
    }
    fn HPRE_get(&self) -> Result<f32, ClockError> {
        let input = self.D1CPRE_get()? as f32;
        let value = self.HPRE.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn AHBOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.HPRE_get()?;
        if input > (275000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 275000000),
                from: ClockNodes::HPRE,
                to: ClockNodes::AHBOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::HPRE,
                to: ClockNodes::AHBOutput,
            });
        }
        Ok(input)
    }
    pub fn AXIClockOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()?;
        if input > (275000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 275000000),
                from: ClockNodes::AHBOutput,
                to: ClockNodes::AXIClockOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::AHBOutput,
                to: ClockNodes::AXIClockOutput,
            });
        }
        Ok(input)
    }
    pub fn HCLK3Output_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()?;
        if input > (275000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 275000000),
                from: ClockNodes::AHBOutput,
                to: ClockNodes::HCLK3Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::AHBOutput,
                to: ClockNodes::HCLK3Output,
            });
        }
        Ok(input)
    }
    fn D1PPRE_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.D1PPRE.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB3Output_get(&self) -> Result<f32, ClockError> {
        let input = self.D1PPRE_get()?;
        if input > (137500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 137500000),
                from: ClockNodes::D1PPRE,
                to: ClockNodes::APB3Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::D1PPRE,
                to: ClockNodes::APB3Output,
            });
        }
        Ok(input)
    }
    fn D2PPRE1_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.D2PPRE1.get()? as f32;
        Ok((input / value) as f32)
    }

    fn Tim1Mul_get(&self) -> Result<f32, ClockError> {
        let input = self.D2PPRE1_get()? as f32;
        let value = 2 as f32;
        Ok((input * value) as f32)
    }

    pub fn Tim1Output_get(&self) -> Result<f32, ClockError> {
        self.Tim1Mul_get()
    }
    pub fn AHB12Output_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()?;
        if input > (275000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 275000000),
                from: ClockNodes::AHBOutput,
                to: ClockNodes::AHB12Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::AHBOutput,
                to: ClockNodes::AHB12Output,
            });
        }
        Ok(input)
    }
    pub fn APB1Output_get(&self) -> Result<f32, ClockError> {
        let input = self.D2PPRE1_get()?;
        if input > (137500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 137500000),
                from: ClockNodes::D2PPRE1,
                to: ClockNodes::APB1Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::D2PPRE1,
                to: ClockNodes::APB1Output,
            });
        }
        Ok(input)
    }
    fn D2PPRE2_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.D2PPRE2.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB2Output_get(&self) -> Result<f32, ClockError> {
        let input = self.D2PPRE2_get()?;
        if input > (137500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 137500000),
                from: ClockNodes::D2PPRE2,
                to: ClockNodes::APB2Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::D2PPRE2,
                to: ClockNodes::APB2Output,
            });
        }
        Ok(input)
    }
    fn Tim2Mul_get(&self) -> Result<f32, ClockError> {
        let input = self.D2PPRE2_get()? as f32;
        let value = 2 as f32;
        Ok((input * value) as f32)
    }

    pub fn Tim2Output_get(&self) -> Result<f32, ClockError> {
        self.Tim2Mul_get()
    }
    pub fn AHB4Output_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()?;
        if input > (275000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 275000000),
                from: ClockNodes::AHBOutput,
                to: ClockNodes::AHB4Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::AHBOutput,
                to: ClockNodes::AHB4Output,
            });
        }
        Ok(input)
    }
    fn D3PPRE_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.D3PPRE.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB4Output_get(&self) -> Result<f32, ClockError> {
        let input = self.D3PPRE_get()?;
        if input > (137500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 137500000),
                from: ClockNodes::D3PPRE,
                to: ClockNodes::APB4Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::D3PPRE,
                to: ClockNodes::APB4Output,
            });
        }
        Ok(input)
    }
    fn PLLSource_get(&self) -> Result<f32, ClockError> {
        match self.PLLSource {
            PLLSourceConf::HSIDiv => return self.HSIDiv_get(),
            PLLSourceConf::CSIRC => return self.CSIRC_get(),
            PLLSourceConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    fn CKPERSource_get(&self) -> Result<f32, ClockError> {
        match self.CKPERSource {
            CKPERSourceConf::HSIDiv => return self.HSIDiv_get(),
            CKPERSourceConf::CSIRC => return self.CSIRC_get(),
            CKPERSourceConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    pub fn CKPERoutput_get(&self) -> Result<f32, ClockError> {
        self.CKPERSource_get()
    }
    fn DIVM1_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLSource_get()? as f32;
        let value = self.DIVM1.get()? as f32;
        Ok((input / value) as f32)
    }

    fn DIVM2_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLSource_get()? as f32;
        let value = self.DIVM2.get()? as f32;
        Ok((input / value) as f32)
    }

    fn DIVM3_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLSource_get()? as f32;
        let value = self.DIVM3.get()? as f32;
        Ok((input / value) as f32)
    }

    fn DIVN1_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVM1_get()? as f32;
        let frac = self.PLLFRACN_get()? as f32;
        let frac_max = PLLFRACNConf::max() as f32;
        let value = self.DIVN1.get()? as f32;
        let ret = (input * (value + (frac / frac_max)));
        Ok(ret as f32)
    }

    pub fn PLLFRACN_get(&self) -> Result<f32, ClockError> {
        self.PLLFRACN.get()
    }
    fn DIVP1_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN1_get()? as f32;
        let value = self.DIVP1.get()? as f32;
        Ok((input / value) as f32)
    }

    fn DIVQ1_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN1_get()? as f32;
        let value = self.DIVQ1.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn DIVQ1output_get(&self) -> Result<f32, ClockError> {
        self.DIVQ1_get()
    }
    fn DIVR1_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN1_get()? as f32;
        let value = self.DIVR1.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn DIVR1output_get(&self) -> Result<f32, ClockError> {
        self.DIVR1_get()
    }
    fn DIVN2_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVM2_get()? as f32;
        let frac = self.PLL2FRACN_get()? as f32;
        let frac_max = PLL2FRACNConf::max() as f32;
        let value = self.DIVN2.get()? as f32;
        let ret = (input * (value + (frac / frac_max)));
        Ok(ret as f32)
    }

    pub fn PLL2FRACN_get(&self) -> Result<f32, ClockError> {
        self.PLL2FRACN.get()
    }
    fn DIVP2_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN2_get()? as f32;
        let value = self.DIVP2.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn DIVP2output_get(&self) -> Result<f32, ClockError> {
        self.DIVP2_get()
    }
    fn DIVQ2_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN2_get()? as f32;
        let value = self.DIVQ2.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn DIVQ2output_get(&self) -> Result<f32, ClockError> {
        self.DIVQ2_get()
    }
    fn DIVR2_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN2_get()? as f32;
        let value = self.DIVR2.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn DIVR2output_get(&self) -> Result<f32, ClockError> {
        self.DIVR2_get()
    }
    fn DIVN3_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVM3_get()? as f32;
        let frac = self.PLL3FRACN_get()? as f32;
        let frac_max = PLL3FRACNConf::max() as f32;
        let value = self.DIVN3.get()? as f32;
        let ret = (input * (value + (frac / frac_max)));
        Ok(ret as f32)
    }

    fn DIVP3_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN3_get()? as f32;
        let value = self.DIVP3.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn PLL3FRACN_get(&self) -> Result<f32, ClockError> {
        self.PLL3FRACN.get()
    }
    pub fn DIVP3output_get(&self) -> Result<f32, ClockError> {
        self.DIVP3_get()
    }
    fn DIVQ3_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN3_get()? as f32;
        let value = self.DIVQ3.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn DIVQ3output_get(&self) -> Result<f32, ClockError> {
        self.DIVQ3_get()
    }
    fn DIVR3_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN3_get()? as f32;
        let value = self.DIVR3.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn LTDCOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVR3_get()?;
        if input > (166000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 166000000),
                from: ClockNodes::DIVR3,
                to: ClockNodes::LTDCOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::DIVR3,
                to: ClockNodes::LTDCOutput,
            });
        }
        Ok(input)
    }
    pub fn DIVR3output_get(&self) -> Result<f32, ClockError> {
        self.DIVR3_get()
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
    fn SPI123Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI123Mult {
            SPI123MultConf::DIVQ1 => return self.DIVQ1_get(),
            SPI123MultConf::DIVP2 => return self.DIVP2_get(),
            SPI123MultConf::DIVP3 => return self.DIVP3_get(),
            SPI123MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            SPI123MultConf::CKPERSource => return self.CKPERSource_get(),
        };
    }
    pub fn SPI123output_get(&self) -> Result<f32, ClockError> {
        let input = self.SPI123Mult_get()?;
        if input > (250000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 250000000),
                from: ClockNodes::SPI123Mult,
                to: ClockNodes::SPI123output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SPI123Mult,
                to: ClockNodes::SPI123output,
            });
        }
        Ok(input)
    }
    fn SAI1Mult_get(&self) -> Result<f32, ClockError> {
        match self.SAI1Mult {
            SAI1MultConf::DIVQ1 => return self.DIVQ1_get(),
            SAI1MultConf::DIVP2 => return self.DIVP2_get(),
            SAI1MultConf::DIVP3 => return self.DIVP3_get(),
            SAI1MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            SAI1MultConf::CKPERSource => return self.CKPERSource_get(),
        };
    }
    pub fn DFSDMACLKoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SAI1Mult_get()?;
        if input > (250000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 250000000),
                from: ClockNodes::SAI1Mult,
                to: ClockNodes::DFSDMACLKoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SAI1Mult,
                to: ClockNodes::DFSDMACLKoutput,
            });
        }
        Ok(input)
    }
    pub fn SAI1output_get(&self) -> Result<f32, ClockError> {
        let input = self.SAI1Mult_get()?;
        if input > (166000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 166000000),
                from: ClockNodes::SAI1Mult,
                to: ClockNodes::SAI1output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SAI1Mult,
                to: ClockNodes::SAI1output,
            });
        }
        Ok(input)
    }
    fn SAI4BMult_get(&self) -> Result<f32, ClockError> {
        match self.SAI4BMult {
            SAI4BMultConf::DIVQ1 => return self.DIVQ1_get(),
            SAI4BMultConf::DIVP2 => return self.DIVP2_get(),
            SAI4BMultConf::DIVP3 => return self.DIVP3_get(),
            SAI4BMultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            SAI4BMultConf::CKPERSource => return self.CKPERSource_get(),
            SAI4BMultConf::SPDIFMult => return self.SPDIFMult_get(),
        };
    }
    pub fn SAI4Boutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SAI4BMult_get()?;
        if input > (166000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 166000000),
                from: ClockNodes::SAI4BMult,
                to: ClockNodes::SAI4Boutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SAI4BMult,
                to: ClockNodes::SAI4Boutput,
            });
        }
        Ok(input)
    }
    fn SAI4AMult_get(&self) -> Result<f32, ClockError> {
        match self.SAI4AMult {
            SAI4AMultConf::DIVQ1 => return self.DIVQ1_get(),
            SAI4AMultConf::DIVP2 => return self.DIVP2_get(),
            SAI4AMultConf::DIVP3 => return self.DIVP3_get(),
            SAI4AMultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            SAI4AMultConf::CKPERSource => return self.CKPERSource_get(),
            SAI4AMultConf::SPDIFMult => return self.SPDIFMult_get(),
        };
    }
    pub fn SAI4Aoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SAI4AMult_get()?;
        if input > (166000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 166000000),
                from: ClockNodes::SAI4AMult,
                to: ClockNodes::SAI4Aoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SAI4AMult,
                to: ClockNodes::SAI4Aoutput,
            });
        }
        Ok(input)
    }
    fn RNGMult_get(&self) -> Result<f32, ClockError> {
        match self.RNGMult {
            RNGMultConf::HSI48 => return self.HSI48_get(),
            RNGMultConf::DIVQ1 => return self.DIVQ1_get(),
            RNGMultConf::LSEOSC => return self.LSEOSC_get(),
            RNGMultConf::LSIRC => return self.LSIRC_get(),
        };
    }
    pub fn RNGoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.RNGMult_get()?;
        if input > (250000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 250000000),
                from: ClockNodes::RNGMult,
                to: ClockNodes::RNGoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::RNGMult,
                to: ClockNodes::RNGoutput,
            });
        }
        Ok(input)
    }
    fn I2C1235Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C1235Mult {
            I2C1235MultConf::D2PPRE1 => return self.D2PPRE1_get(),
            I2C1235MultConf::DIVR3 => return self.DIVR3_get(),
            I2C1235MultConf::HSIDiv => return self.HSIDiv_get(),
            I2C1235MultConf::CSIRC => return self.CSIRC_get(),
        };
    }
    pub fn I2C1235output_get(&self) -> Result<f32, ClockError> {
        let input = self.I2C1235Mult_get()?;
        if input > (137500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 137500000),
                from: ClockNodes::I2C1235Mult,
                to: ClockNodes::I2C1235output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::I2C1235Mult,
                to: ClockNodes::I2C1235output,
            });
        }
        Ok(input)
    }
    fn I2C4Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C4Mult {
            I2C4MultConf::D3PPRE => return self.D3PPRE_get(),
            I2C4MultConf::DIVR3 => return self.DIVR3_get(),
            I2C4MultConf::HSIDiv => return self.HSIDiv_get(),
            I2C4MultConf::CSIRC => return self.CSIRC_get(),
        };
    }
    pub fn I2C4output_get(&self) -> Result<f32, ClockError> {
        let input = self.I2C4Mult_get()?;
        if input > (137500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 137500000),
                from: ClockNodes::I2C4Mult,
                to: ClockNodes::I2C4output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::I2C4Mult,
                to: ClockNodes::I2C4output,
            });
        }
        Ok(input)
    }
    fn SPDIFMult_get(&self) -> Result<f32, ClockError> {
        match self.SPDIFMult {
            SPDIFMultConf::DIVQ1 => return self.DIVQ1_get(),
            SPDIFMultConf::DIVR2 => return self.DIVR2_get(),
            SPDIFMultConf::DIVR3 => return self.DIVR3_get(),
            SPDIFMultConf::HSIDiv => return self.HSIDiv_get(),
        };
    }
    pub fn SPDIFoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SPDIFMult_get()?;
        if input > (250000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 250000000),
                from: ClockNodes::SPDIFMult,
                to: ClockNodes::SPDIFoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SPDIFMult,
                to: ClockNodes::SPDIFoutput,
            });
        }
        Ok(input)
    }
    fn QSPIMult_get(&self) -> Result<f32, ClockError> {
        match self.QSPIMult {
            QSPIMultConf::HCLK3Output => return self.HCLK3Output_get(),
            QSPIMultConf::DIVQ1 => return self.DIVQ1_get(),
            QSPIMultConf::DIVR2 => return self.DIVR2_get(),
            QSPIMultConf::CKPERSource => return self.CKPERSource_get(),
        };
    }
    pub fn QSPIoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.QSPIMult_get()?;
        if input > (333000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 333000000),
                from: ClockNodes::QSPIMult,
                to: ClockNodes::QSPIoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::QSPIMult,
                to: ClockNodes::QSPIoutput,
            });
        }
        Ok(input)
    }
    fn FMCMult_get(&self) -> Result<f32, ClockError> {
        match self.FMCMult {
            FMCMultConf::HCLK3Output => return self.HCLK3Output_get(),
            FMCMultConf::DIVQ1 => return self.DIVQ1_get(),
            FMCMultConf::DIVR2 => return self.DIVR2_get(),
            FMCMultConf::CKPERSource => return self.CKPERSource_get(),
        };
    }
    pub fn FMCoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.FMCMult_get()?;
        if input > (275000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 275000000),
                from: ClockNodes::FMCMult,
                to: ClockNodes::FMCoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::FMCMult,
                to: ClockNodes::FMCoutput,
            });
        }
        Ok(input)
    }
    fn SWPMult_get(&self) -> Result<f32, ClockError> {
        match self.SWPMult {
            SWPMultConf::D2PPRE1 => return self.D2PPRE1_get(),
            SWPMultConf::HSIDiv => return self.HSIDiv_get(),
        };
    }
    pub fn SWPoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SWPMult_get()?;
        if input > (137500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 137500000),
                from: ClockNodes::SWPMult,
                to: ClockNodes::SWPoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SWPMult,
                to: ClockNodes::SWPoutput,
            });
        }
        Ok(input)
    }
    fn SDMMCMult_get(&self) -> Result<f32, ClockError> {
        match self.SDMMCMult {
            SDMMCMultConf::DIVQ1 => return self.DIVQ1_get(),
            SDMMCMultConf::DIVR2 => return self.DIVR2_get(),
        };
    }
    pub fn SDMMCoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SDMMCMult_get()?;
        if input > (250000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 250000000),
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
    fn DFSDMMult_get(&self) -> Result<f32, ClockError> {
        match self.DFSDMMult {
            DFSDMMultConf::D2PPRE2 => return self.D2PPRE2_get(),
            DFSDMMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
        };
    }
    pub fn DFSDMoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.DFSDMMult_get()?;
        if input > (275000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 275000000),
                from: ClockNodes::DFSDMMult,
                to: ClockNodes::DFSDMoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::DFSDMMult,
                to: ClockNodes::DFSDMoutput,
            });
        }
        Ok(input)
    }
    fn USART16Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART16Mult {
            USART16MultConf::D2PPRE2 => return self.D2PPRE2_get(),
            USART16MultConf::DIVQ2 => return self.DIVQ2_get(),
            USART16MultConf::DIVQ3 => return self.DIVQ3_get(),
            USART16MultConf::HSIDiv => return self.HSIDiv_get(),
            USART16MultConf::CSIRC => return self.CSIRC_get(),
            USART16MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn USART16output_get(&self) -> Result<f32, ClockError> {
        let input = self.USART16Mult_get()?;
        if input > (137500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 137500000),
                from: ClockNodes::USART16Mult,
                to: ClockNodes::USART16output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::USART16Mult,
                to: ClockNodes::USART16output,
            });
        }
        Ok(input)
    }
    fn USART234578Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART234578Mult {
            USART234578MultConf::D2PPRE1 => return self.D2PPRE1_get(),
            USART234578MultConf::DIVQ2 => return self.DIVQ2_get(),
            USART234578MultConf::DIVQ3 => return self.DIVQ3_get(),
            USART234578MultConf::HSIDiv => return self.HSIDiv_get(),
            USART234578MultConf::CSIRC => return self.CSIRC_get(),
            USART234578MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn USART234578output_get(&self) -> Result<f32, ClockError> {
        let input = self.USART234578Mult_get()?;
        if input > (137500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 137500000),
                from: ClockNodes::USART234578Mult,
                to: ClockNodes::USART234578output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::USART234578Mult,
                to: ClockNodes::USART234578output,
            });
        }
        Ok(input)
    }
    fn LPUART1Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPUART1Mult {
            LPUART1MultConf::D1PPRE => return self.D1PPRE_get(),
            LPUART1MultConf::DIVQ2 => return self.DIVQ2_get(),
            LPUART1MultConf::DIVQ3 => return self.DIVQ3_get(),
            LPUART1MultConf::HSIDiv => return self.HSIDiv_get(),
            LPUART1MultConf::CSIRC => return self.CSIRC_get(),
            LPUART1MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn LPUART1output_get(&self) -> Result<f32, ClockError> {
        let input = self.LPUART1Mult_get()?;
        if input > (137500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 137500000),
                from: ClockNodes::LPUART1Mult,
                to: ClockNodes::LPUART1output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::LPUART1Mult,
                to: ClockNodes::LPUART1output,
            });
        }
        Ok(input)
    }
    fn LPTIM1Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM1Mult {
            LPTIM1MultConf::D2PPRE1 => return self.D2PPRE1_get(),
            LPTIM1MultConf::DIVP2 => return self.DIVP2_get(),
            LPTIM1MultConf::DIVR3 => return self.DIVR3_get(),
            LPTIM1MultConf::LSEOSC => return self.LSEOSC_get(),
            LPTIM1MultConf::LSIRC => return self.LSIRC_get(),
            LPTIM1MultConf::CKPERSource => return self.CKPERSource_get(),
        };
    }
    pub fn LPTIM1output_get(&self) -> Result<f32, ClockError> {
        let input = self.LPTIM1Mult_get()?;
        if input > (137500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 137500000),
                from: ClockNodes::LPTIM1Mult,
                to: ClockNodes::LPTIM1output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::LPTIM1Mult,
                to: ClockNodes::LPTIM1output,
            });
        }
        Ok(input)
    }
    fn LPTIM345Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM345Mult {
            LPTIM345MultConf::D3PPRE => return self.D3PPRE_get(),
            LPTIM345MultConf::DIVP2 => return self.DIVP2_get(),
            LPTIM345MultConf::DIVR3 => return self.DIVR3_get(),
            LPTIM345MultConf::LSEOSC => return self.LSEOSC_get(),
            LPTIM345MultConf::LSIRC => return self.LSIRC_get(),
            LPTIM345MultConf::CKPERSource => return self.CKPERSource_get(),
        };
    }
    pub fn LPTIM345output_get(&self) -> Result<f32, ClockError> {
        let input = self.LPTIM345Mult_get()?;
        if input > (137500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 137500000),
                from: ClockNodes::LPTIM345Mult,
                to: ClockNodes::LPTIM345output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::LPTIM345Mult,
                to: ClockNodes::LPTIM345output,
            });
        }
        Ok(input)
    }
    fn LPTIM2Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM2Mult {
            LPTIM2MultConf::D3PPRE => return self.D3PPRE_get(),
            LPTIM2MultConf::DIVP2 => return self.DIVP2_get(),
            LPTIM2MultConf::DIVR3 => return self.DIVR3_get(),
            LPTIM2MultConf::LSEOSC => return self.LSEOSC_get(),
            LPTIM2MultConf::LSIRC => return self.LSIRC_get(),
            LPTIM2MultConf::CKPERSource => return self.CKPERSource_get(),
        };
    }
    pub fn LPTIM2output_get(&self) -> Result<f32, ClockError> {
        let input = self.LPTIM2Mult_get()?;
        if input > (137500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 137500000),
                from: ClockNodes::LPTIM2Mult,
                to: ClockNodes::LPTIM2output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::LPTIM2Mult,
                to: ClockNodes::LPTIM2output,
            });
        }
        Ok(input)
    }
    fn SPI6Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI6Mult {
            SPI6MultConf::D3PPRE => return self.D3PPRE_get(),
            SPI6MultConf::DIVQ2 => return self.DIVQ2_get(),
            SPI6MultConf::DIVQ3 => return self.DIVQ3_get(),
            SPI6MultConf::HSIDiv => return self.HSIDiv_get(),
            SPI6MultConf::CSIRC => return self.CSIRC_get(),
            SPI6MultConf::HSEOSC => return self.HSEOSC_get(),
            SPI6MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    pub fn SPI6output_get(&self) -> Result<f32, ClockError> {
        let input = self.SPI6Mult_get()?;
        if input > (250000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 250000000),
                from: ClockNodes::SPI6Mult,
                to: ClockNodes::SPI6output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SPI6Mult,
                to: ClockNodes::SPI6output,
            });
        }
        Ok(input)
    }
    fn SPI45Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI45Mult {
            SPI45MultConf::D2PPRE2 => return self.D2PPRE2_get(),
            SPI45MultConf::DIVQ2 => return self.DIVQ2_get(),
            SPI45MultConf::DIVQ3 => return self.DIVQ3_get(),
            SPI45MultConf::HSIDiv => return self.HSIDiv_get(),
            SPI45MultConf::CSIRC => return self.CSIRC_get(),
            SPI45MultConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    pub fn SPI45output_get(&self) -> Result<f32, ClockError> {
        let input = self.SPI45Mult_get()?;
        if input > (250000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 250000000),
                from: ClockNodes::SPI45Mult,
                to: ClockNodes::SPI45output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SPI45Mult,
                to: ClockNodes::SPI45output,
            });
        }
        Ok(input)
    }
    fn USBMult_get(&self) -> Result<f32, ClockError> {
        match self.USBMult {
            USBMultConf::DIVQ1 => return self.DIVQ1_get(),
            USBMultConf::DIVQ3 => return self.DIVQ3_get(),
            USBMultConf::HSI48 => return self.HSI48_get(),
        };
    }
    pub fn USBoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.USBMult_get()?;
        if input > (125000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 125000000),
                from: ClockNodes::USBMult,
                to: ClockNodes::USBoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::USBMult,
                to: ClockNodes::USBoutput,
            });
        }
        Ok(input)
    }
    fn FDCANMult_get(&self) -> Result<f32, ClockError> {
        match self.FDCANMult {
            FDCANMultConf::HSEOSC => return self.HSEOSC_get(),
            FDCANMultConf::DIVQ1 => return self.DIVQ1_get(),
            FDCANMultConf::DIVQ2 => return self.DIVQ2_get(),
        };
    }
    pub fn FDCANoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.FDCANMult_get()?;
        if input > (137500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 137500000),
                from: ClockNodes::FDCANMult,
                to: ClockNodes::FDCANoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::FDCANMult,
                to: ClockNodes::FDCANoutput,
            });
        }
        Ok(input)
    }
    fn ADCMult_get(&self) -> Result<f32, ClockError> {
        match self.ADCMult {
            ADCMultConf::DIVP2 => return self.DIVP2_get(),
            ADCMultConf::DIVR3 => return self.DIVR3_get(),
            ADCMultConf::CKPERSource => return self.CKPERSource_get(),
        };
    }
    pub fn ADCoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.ADCMult_get()?;
        if input > (160000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 160000000),
                from: ClockNodes::ADCMult,
                to: ClockNodes::ADCoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::ADCMult,
                to: ClockNodes::ADCoutput,
            });
        }
        Ok(input)
    }
    fn CECCSIDevisor_get(&self) -> Result<f32, ClockError> {
        let input = self.CSIRC_get()? as f32;
        let value = 122 as f32;
        Ok((input / value) as f32)
    }

    fn CECMult_get(&self) -> Result<f32, ClockError> {
        match self.CECMult {
            CECMultConf::LSEOSC => return self.LSEOSC_get(),
            CECMultConf::LSIRC => return self.LSIRC_get(),
            CECMultConf::CECCSIDevisor => return self.CECCSIDevisor_get(),
        };
    }
    pub fn CECoutput_get(&self) -> Result<f32, ClockError> {
        self.CECMult_get()
    }
    fn HrtimMult_get(&self) -> Result<f32, ClockError> {
        match self.HrtimMult {
            HrtimMultConf::Tim2Output => return self.Tim2Output_get(),
            HrtimMultConf::D1CPRE => return self.D1CPRE_get(),
        };
    }
    pub fn HRTIMoutput_get(&self) -> Result<f32, ClockError> {
        self.HrtimMult_get()
    }
}
