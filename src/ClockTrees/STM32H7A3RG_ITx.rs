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
    RC48,
    I2S_CKIN,
    SysClkSource,
    SysCLKOutput,
    MCO1Mult,
    MCO1Div,
    MCO1Pin,
    MCO2Mult,
    MCO2Div,
    MCO2Pin,
    traceClkSource,
    TraceCLKOutput,
    CDCPRE,
    CDCPREOutput,
    CpuClockOutput,
    CortexPrescaler,
    CortexSysOutput,
    HPRE,
    AHBOutput,
    AXIClockOutput,
    HCLK3Output,
    CDPPRE,
    APB3Output,
    CDPPRE1,
    Tim1Mul,
    Tim1Output,
    AHB12Output,
    APB1Output,
    CDPPRE2,
    APB2Output,
    Tim2Mul,
    Tim2Output,
    AHB4Output,
    SRDPPRE,
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
    PLL3FRACN,
    DIVP3,
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
    DAC1Output,
    DAC2Output,
    SPI123Mult,
    SPI123output,
    SAI1Mult,
    DFSDMACLKoutput,
    SAI1output,
    SAI2BMult,
    SAI2Boutput,
    SAI2AMult,
    SAI2Aoutput,
    RNGMult,
    RNGoutput,
    I2C123Mult,
    I2C123output,
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
    DFSDM2Mult,
    DFSDM2output,
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
    DFSDM2ACLKoutput,
    SPI45Mult,
    SPI45output,
    USBMult,
    USBoutput,
    FDCANMult,
    FDCANoutput,
    ADCMult,
    ADCoutput,
    CECMult,
    CECoutput,
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
    RC48,
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
pub enum traceClkSourceConf {
    HSIDiv,
    CSIRC,
    HSEOSC,
    DIVR1,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CDCPREConf {
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

impl CDCPREConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            CDCPREConf::DIV1 => return Ok(1.0),
            CDCPREConf::DIV2 => return Ok(2.0),
            CDCPREConf::DIV4 => return Ok(4.0),
            CDCPREConf::DIV8 => return Ok(8.0),
            CDCPREConf::DIV16 => return Ok(16.0),
            CDCPREConf::DIV64 => return Ok(64.0),
            CDCPREConf::DIV128 => return Ok(128.0),
            CDCPREConf::DIV256 => return Ok(256.0),
            CDCPREConf::DIV512 => return Ok(512.0),
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
pub enum CDPPREConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl CDPPREConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            CDPPREConf::DIV1 => return Ok(1.0),
            CDPPREConf::DIV2 => return Ok(2.0),
            CDPPREConf::DIV4 => return Ok(4.0),
            CDPPREConf::DIV8 => return Ok(8.0),
            CDPPREConf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CDPPRE1Conf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl CDPPRE1Conf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            CDPPRE1Conf::DIV1 => return Ok(1.0),
            CDPPRE1Conf::DIV2 => return Ok(2.0),
            CDPPRE1Conf::DIV4 => return Ok(4.0),
            CDPPRE1Conf::DIV8 => return Ok(8.0),
            CDPPRE1Conf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CDPPRE2Conf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl CDPPRE2Conf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            CDPPRE2Conf::DIV1 => return Ok(1.0),
            CDPPRE2Conf::DIV2 => return Ok(2.0),
            CDPPRE2Conf::DIV4 => return Ok(4.0),
            CDPPRE2Conf::DIV8 => return Ok(8.0),
            CDPPRE2Conf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SRDPPREConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl SRDPPREConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            SRDPPREConf::DIV1 => return Ok(1.0),
            SRDPPREConf::DIV2 => return Ok(2.0),
            SRDPPREConf::DIV4 => return Ok(4.0),
            SRDPPREConf::DIV8 => return Ok(8.0),
            SRDPPREConf::DIV16 => return Ok(16.0),
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
        8
    }

    pub const fn max() -> u32 {
        420
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
    DIV2,
    DIV4,
    DIV6,
    DIV8,
    DIV10,
    DIV12,
    DIV14,
    DIV16,
    DIV18,
    DIV20,
    DIV22,
    DIV24,
    DIV26,
    DIV28,
    DIV30,
    DIV32,
    DIV34,
    DIV36,
    DIV38,
    DIV40,
    DIV42,
    DIV44,
    DIV46,
    DIV48,
    DIV50,
    DIV52,
    DIV54,
    DIV56,
    DIV58,
    DIV60,
    DIV62,
    DIV64,
    DIV66,
    DIV68,
    DIV70,
    DIV72,
    DIV74,
    DIV76,
    DIV78,
    DIV80,
    DIV82,
    DIV84,
    DIV86,
    DIV88,
    DIV90,
    DIV92,
    DIV94,
    DIV96,
    DIV98,
    DIV100,
    DIV102,
    DIV104,
    DIV106,
    DIV108,
    DIV110,
    DIV112,
    DIV114,
    DIV116,
    DIV118,
    DIV120,
    DIV122,
    DIV124,
    DIV126,
    DIV128,
}

impl DIVP1Conf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVP1Conf::DIV2 => return Ok(2.0),
            DIVP1Conf::DIV4 => return Ok(4.0),
            DIVP1Conf::DIV6 => return Ok(6.0),
            DIVP1Conf::DIV8 => return Ok(8.0),
            DIVP1Conf::DIV10 => return Ok(10.0),
            DIVP1Conf::DIV12 => return Ok(12.0),
            DIVP1Conf::DIV14 => return Ok(14.0),
            DIVP1Conf::DIV16 => return Ok(16.0),
            DIVP1Conf::DIV18 => return Ok(18.0),
            DIVP1Conf::DIV20 => return Ok(20.0),
            DIVP1Conf::DIV22 => return Ok(22.0),
            DIVP1Conf::DIV24 => return Ok(24.0),
            DIVP1Conf::DIV26 => return Ok(26.0),
            DIVP1Conf::DIV28 => return Ok(28.0),
            DIVP1Conf::DIV30 => return Ok(30.0),
            DIVP1Conf::DIV32 => return Ok(32.0),
            DIVP1Conf::DIV34 => return Ok(34.0),
            DIVP1Conf::DIV36 => return Ok(36.0),
            DIVP1Conf::DIV38 => return Ok(38.0),
            DIVP1Conf::DIV40 => return Ok(40.0),
            DIVP1Conf::DIV42 => return Ok(42.0),
            DIVP1Conf::DIV44 => return Ok(44.0),
            DIVP1Conf::DIV46 => return Ok(46.0),
            DIVP1Conf::DIV48 => return Ok(48.0),
            DIVP1Conf::DIV50 => return Ok(50.0),
            DIVP1Conf::DIV52 => return Ok(52.0),
            DIVP1Conf::DIV54 => return Ok(54.0),
            DIVP1Conf::DIV56 => return Ok(56.0),
            DIVP1Conf::DIV58 => return Ok(58.0),
            DIVP1Conf::DIV60 => return Ok(60.0),
            DIVP1Conf::DIV62 => return Ok(62.0),
            DIVP1Conf::DIV64 => return Ok(64.0),
            DIVP1Conf::DIV66 => return Ok(66.0),
            DIVP1Conf::DIV68 => return Ok(68.0),
            DIVP1Conf::DIV70 => return Ok(70.0),
            DIVP1Conf::DIV72 => return Ok(72.0),
            DIVP1Conf::DIV74 => return Ok(74.0),
            DIVP1Conf::DIV76 => return Ok(76.0),
            DIVP1Conf::DIV78 => return Ok(78.0),
            DIVP1Conf::DIV80 => return Ok(80.0),
            DIVP1Conf::DIV82 => return Ok(82.0),
            DIVP1Conf::DIV84 => return Ok(84.0),
            DIVP1Conf::DIV86 => return Ok(86.0),
            DIVP1Conf::DIV88 => return Ok(88.0),
            DIVP1Conf::DIV90 => return Ok(90.0),
            DIVP1Conf::DIV92 => return Ok(92.0),
            DIVP1Conf::DIV94 => return Ok(94.0),
            DIVP1Conf::DIV96 => return Ok(96.0),
            DIVP1Conf::DIV98 => return Ok(98.0),
            DIVP1Conf::DIV100 => return Ok(100.0),
            DIVP1Conf::DIV102 => return Ok(102.0),
            DIVP1Conf::DIV104 => return Ok(104.0),
            DIVP1Conf::DIV106 => return Ok(106.0),
            DIVP1Conf::DIV108 => return Ok(108.0),
            DIVP1Conf::DIV110 => return Ok(110.0),
            DIVP1Conf::DIV112 => return Ok(112.0),
            DIVP1Conf::DIV114 => return Ok(114.0),
            DIVP1Conf::DIV116 => return Ok(116.0),
            DIVP1Conf::DIV118 => return Ok(118.0),
            DIVP1Conf::DIV120 => return Ok(120.0),
            DIVP1Conf::DIV122 => return Ok(122.0),
            DIVP1Conf::DIV124 => return Ok(124.0),
            DIVP1Conf::DIV126 => return Ok(126.0),
            DIVP1Conf::DIV128 => return Ok(128.0),
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
        8
    }

    pub const fn max() -> u32 {
        420
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
        8
    }

    pub const fn max() -> u32 {
        420
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
pub enum SAI2BMultConf {
    DIVQ1,
    DIVP2,
    DIVP3,
    I2S_CKIN,
    CKPERSource,
    SPDIFMult,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SAI2AMultConf {
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
    RC48,
    DIVQ1,
    LSEOSC,
    LSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C123MultConf {
    CDPPRE1,
    DIVR3,
    HSIDiv,
    CSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C4MultConf {
    SRDPPRE,
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
    CDPPRE1,
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
    CDPPRE2,
    SysCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DFSDM2MultConf {
    SRDPPRE,
    SysCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART16MultConf {
    CDPPRE2,
    DIVQ2,
    DIVQ3,
    HSIDiv,
    CSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART234578MultConf {
    CDPPRE1,
    DIVQ2,
    DIVQ3,
    HSIDiv,
    CSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPUART1MultConf {
    CDPPRE,
    DIVQ2,
    DIVQ3,
    HSIDiv,
    CSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM1MultConf {
    CDPPRE1,
    DIVP2,
    DIVR3,
    LSEOSC,
    LSIRC,
    CKPERSource,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM345MultConf {
    SRDPPRE,
    DIVP2,
    DIVR3,
    LSEOSC,
    LSIRC,
    CKPERSource,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM2MultConf {
    SRDPPRE,
    DIVP2,
    DIVR3,
    LSEOSC,
    LSIRC,
    CKPERSource,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI6MultConf {
    SRDPPRE,
    DIVQ2,
    DIVQ3,
    HSIDiv,
    CSIRC,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI45MultConf {
    CDPPRE1,
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
    RC48,
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
    CSIRC,
}
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ClockTree {
    pub HSIDiv: HSIDivConf,
    pub HSEOSC: HSEOSCConf,
    pub LSIRC: LSIRCConf,
    pub LSEOSC: LSEOSCConf,
    pub SysClkSource: SysClkSourceConf,
    pub MCO1Mult: MCO1MultConf,
    pub MCO1Div: MCO1DivConf,
    pub MCO2Mult: MCO2MultConf,
    pub MCO2Div: MCO2DivConf,
    pub traceClkSource: traceClkSourceConf,
    pub CDCPRE: CDCPREConf,
    pub CortexPrescaler: CortexPrescalerConf,
    pub HPRE: HPREConf,
    pub CDPPRE: CDPPREConf,
    pub CDPPRE1: CDPPRE1Conf,
    pub CDPPRE2: CDPPRE2Conf,
    pub SRDPPRE: SRDPPREConf,
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
    pub PLL3FRACN: PLL3FRACNConf,
    pub DIVP3: DIVP3Conf,
    pub DIVQ3: DIVQ3Conf,
    pub DIVR3: DIVR3Conf,
    pub HSERTCDevisor: HSERTCDevisorConf,
    pub RTCClkSource: RTCClkSourceConf,
    pub SPI123Mult: SPI123MultConf,
    pub SAI1Mult: SAI1MultConf,
    pub SAI2BMult: SAI2BMultConf,
    pub SAI2AMult: SAI2AMultConf,
    pub RNGMult: RNGMultConf,
    pub I2C123Mult: I2C123MultConf,
    pub I2C4Mult: I2C4MultConf,
    pub SPDIFMult: SPDIFMultConf,
    pub QSPIMult: QSPIMultConf,
    pub FMCMult: FMCMultConf,
    pub SWPMult: SWPMultConf,
    pub SDMMCMult: SDMMCMultConf,
    pub DFSDMMult: DFSDMMultConf,
    pub DFSDM2Mult: DFSDM2MultConf,
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
}

impl Default for ClockTree {
    fn default() -> Self {
        Self {
            HSIDiv: HSIDivConf::DIV1,
            HSEOSC: HSEOSCConf::Value(24000000),
            LSIRC: LSIRCConf::Value(32000),
            LSEOSC: LSEOSCConf::Value(32768),
            SysClkSource: SysClkSourceConf::HSIDiv,
            MCO1Mult: MCO1MultConf::HSIDiv,
            MCO1Div: MCO1DivConf::DIV1,
            MCO2Mult: MCO2MultConf::SysCLKOutput,
            MCO2Div: MCO2DivConf::DIV1,
            traceClkSource: traceClkSourceConf::HSIDiv,
            CDCPRE: CDCPREConf::DIV1,
            CortexPrescaler: CortexPrescalerConf::DIV1,
            HPRE: HPREConf::DIV1,
            CDPPRE: CDPPREConf::DIV1,
            CDPPRE1: CDPPRE1Conf::DIV1,
            CDPPRE2: CDPPRE2Conf::DIV1,
            SRDPPRE: SRDPPREConf::DIV1,
            PLLSource: PLLSourceConf::HSIDiv,
            CKPERSource: CKPERSourceConf::HSIDiv,
            DIVM1: DIVM1Conf::Value(32),
            DIVM2: DIVM2Conf::Value(32),
            DIVM3: DIVM3Conf::Value(32),
            DIVN1: DIVN1Conf::Value(129),
            PLLFRACN: PLLFRACNConf::Value(0),
            DIVP1: DIVP1Conf::DIV2,
            DIVQ1: DIVQ1Conf::Value(2),
            DIVR1: DIVR1Conf::Value(2),
            DIVN2: DIVN2Conf::Value(129),
            PLL2FRACN: PLL2FRACNConf::Value(0),
            DIVP2: DIVP2Conf::Value(2),
            DIVQ2: DIVQ2Conf::Value(2),
            DIVR2: DIVR2Conf::Value(2),
            DIVN3: DIVN3Conf::Value(129),
            PLL3FRACN: PLL3FRACNConf::Value(0),
            DIVP3: DIVP3Conf::Value(2),
            DIVQ3: DIVQ3Conf::Value(2),
            DIVR3: DIVR3Conf::Value(2),
            HSERTCDevisor: HSERTCDevisorConf::DIV2,
            RTCClkSource: RTCClkSourceConf::LSIRC,
            SPI123Mult: SPI123MultConf::DIVQ1,
            SAI1Mult: SAI1MultConf::DIVQ1,
            SAI2BMult: SAI2BMultConf::DIVQ1,
            SAI2AMult: SAI2AMultConf::DIVQ1,
            RNGMult: RNGMultConf::RC48,
            I2C123Mult: I2C123MultConf::CDPPRE1,
            I2C4Mult: I2C4MultConf::SRDPPRE,
            SPDIFMult: SPDIFMultConf::DIVQ1,
            QSPIMult: QSPIMultConf::HCLK3Output,
            FMCMult: FMCMultConf::HCLK3Output,
            SWPMult: SWPMultConf::CDPPRE1,
            SDMMCMult: SDMMCMultConf::DIVQ1,
            DFSDMMult: DFSDMMultConf::CDPPRE2,
            DFSDM2Mult: DFSDM2MultConf::SRDPPRE,
            USART16Mult: USART16MultConf::CDPPRE2,
            USART234578Mult: USART234578MultConf::CDPPRE1,
            LPUART1Mult: LPUART1MultConf::CDPPRE,
            LPTIM1Mult: LPTIM1MultConf::CDPPRE1,
            LPTIM345Mult: LPTIM345MultConf::SRDPPRE,
            LPTIM2Mult: LPTIM2MultConf::SRDPPRE,
            SPI6Mult: SPI6MultConf::SRDPPRE,
            SPI45Mult: SPI45MultConf::CDPPRE1,
            USBMult: USBMultConf::DIVQ1,
            FDCANMult: FDCANMultConf::DIVQ1,
            ADCMult: ADCMultConf::DIVP2,
            CECMult: CECMultConf::LSIRC,
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
    pub fn RC48_get(&self) -> Result<f32, ClockError> {
        Ok(48000000 as f32)
    }
    pub fn I2S_CKIN_get(&self) -> Result<f32, ClockError> {
        Ok(12288000 as f32)
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
        if input > (280000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 280000000),
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
            MCO1MultConf::RC48 => return self.RC48_get(),
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
    fn CDCPRE_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()? as f32;
        let value = self.CDCPRE.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn CDCPREOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.CDCPRE_get()?;
        if input > (280000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 280000000),
                from: ClockNodes::CDCPRE,
                to: ClockNodes::CDCPREOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::CDCPRE,
                to: ClockNodes::CDCPREOutput,
            });
        }
        Ok(input)
    }
    pub fn CpuClockOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.CDCPREOutput_get()?;
        if input > (280000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 280000000),
                from: ClockNodes::CDCPREOutput,
                to: ClockNodes::CpuClockOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::CDCPREOutput,
                to: ClockNodes::CpuClockOutput,
            });
        }
        Ok(input)
    }
    fn CortexPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.CDCPREOutput_get()? as f32;
        let value = self.CortexPrescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn CortexSysOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.CortexPrescaler_get()?;
        if input > (280000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 280000000),
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
        let input = self.CDCPREOutput_get()? as f32;
        let value = self.HPRE.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn AHBOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.HPRE_get()?;
        if input > (280000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 280000000),
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
        if input > (280000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 280000000),
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
        if input > (280000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 280000000),
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
    fn CDPPRE_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.CDPPRE.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB3Output_get(&self) -> Result<f32, ClockError> {
        let input = self.CDPPRE_get()?;
        if input > (140000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 140000000),
                from: ClockNodes::CDPPRE,
                to: ClockNodes::APB3Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::CDPPRE,
                to: ClockNodes::APB3Output,
            });
        }
        Ok(input)
    }
    fn CDPPRE1_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.CDPPRE1.get()? as f32;
        Ok((input / value) as f32)
    }

    fn Tim1Mul_get(&self) -> Result<f32, ClockError> {
        let input = self.CDPPRE1_get()? as f32;
        let value = 2 as f32;
        Ok((input * value) as f32)
    }

    pub fn Tim1Output_get(&self) -> Result<f32, ClockError> {
        self.Tim1Mul_get()
    }
    pub fn AHB12Output_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()?;
        if input > (280000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 280000000),
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
        let input = self.CDPPRE1_get()?;
        if input > (140000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 140000000),
                from: ClockNodes::CDPPRE1,
                to: ClockNodes::APB1Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::CDPPRE1,
                to: ClockNodes::APB1Output,
            });
        }
        Ok(input)
    }
    fn CDPPRE2_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.CDPPRE2.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB2Output_get(&self) -> Result<f32, ClockError> {
        let input = self.CDPPRE2_get()?;
        if input > (140000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 140000000),
                from: ClockNodes::CDPPRE2,
                to: ClockNodes::APB2Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::CDPPRE2,
                to: ClockNodes::APB2Output,
            });
        }
        Ok(input)
    }
    fn Tim2Mul_get(&self) -> Result<f32, ClockError> {
        let input = self.CDPPRE2_get()? as f32;
        let value = 2 as f32;
        Ok((input * value) as f32)
    }

    pub fn Tim2Output_get(&self) -> Result<f32, ClockError> {
        self.Tim2Mul_get()
    }
    pub fn AHB4Output_get(&self) -> Result<f32, ClockError> {
        self.AHBOutput_get()
    }
    fn SRDPPRE_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.SRDPPRE.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB4Output_get(&self) -> Result<f32, ClockError> {
        let input = self.SRDPPRE_get()?;
        if input > (140000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 140000000),
                from: ClockNodes::SRDPPRE,
                to: ClockNodes::APB4Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SRDPPRE,
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

    pub fn PLL3FRACN_get(&self) -> Result<f32, ClockError> {
        self.PLL3FRACN.get()
    }
    fn DIVP3_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN3_get()? as f32;
        let value = self.DIVP3.get()? as f32;
        Ok((input / value) as f32)
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
        self.DIVR3_get()
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
    pub fn DAC1Output_get(&self) -> Result<f32, ClockError> {
        self.LSIRC_get()
    }
    pub fn DAC2Output_get(&self) -> Result<f32, ClockError> {
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
        self.SPI123Mult_get()
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
        self.SAI1Mult_get()
    }
    pub fn SAI1output_get(&self) -> Result<f32, ClockError> {
        self.SAI1Mult_get()
    }
    fn SAI2BMult_get(&self) -> Result<f32, ClockError> {
        match self.SAI2BMult {
            SAI2BMultConf::DIVQ1 => return self.DIVQ1_get(),
            SAI2BMultConf::DIVP2 => return self.DIVP2_get(),
            SAI2BMultConf::DIVP3 => return self.DIVP3_get(),
            SAI2BMultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            SAI2BMultConf::CKPERSource => return self.CKPERSource_get(),
            SAI2BMultConf::SPDIFMult => return self.SPDIFMult_get(),
        };
    }
    pub fn SAI2Boutput_get(&self) -> Result<f32, ClockError> {
        self.SAI2BMult_get()
    }
    fn SAI2AMult_get(&self) -> Result<f32, ClockError> {
        match self.SAI2AMult {
            SAI2AMultConf::DIVQ1 => return self.DIVQ1_get(),
            SAI2AMultConf::DIVP2 => return self.DIVP2_get(),
            SAI2AMultConf::DIVP3 => return self.DIVP3_get(),
            SAI2AMultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            SAI2AMultConf::CKPERSource => return self.CKPERSource_get(),
            SAI2AMultConf::SPDIFMult => return self.SPDIFMult_get(),
        };
    }
    pub fn SAI2Aoutput_get(&self) -> Result<f32, ClockError> {
        self.SAI2AMult_get()
    }
    fn RNGMult_get(&self) -> Result<f32, ClockError> {
        match self.RNGMult {
            RNGMultConf::RC48 => return self.RC48_get(),
            RNGMultConf::DIVQ1 => return self.DIVQ1_get(),
            RNGMultConf::LSEOSC => return self.LSEOSC_get(),
            RNGMultConf::LSIRC => return self.LSIRC_get(),
        };
    }
    pub fn RNGoutput_get(&self) -> Result<f32, ClockError> {
        self.RNGMult_get()
    }
    fn I2C123Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C123Mult {
            I2C123MultConf::CDPPRE1 => return self.CDPPRE1_get(),
            I2C123MultConf::DIVR3 => return self.DIVR3_get(),
            I2C123MultConf::HSIDiv => return self.HSIDiv_get(),
            I2C123MultConf::CSIRC => return self.CSIRC_get(),
        };
    }
    pub fn I2C123output_get(&self) -> Result<f32, ClockError> {
        self.I2C123Mult_get()
    }
    fn I2C4Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C4Mult {
            I2C4MultConf::SRDPPRE => return self.SRDPPRE_get(),
            I2C4MultConf::DIVR3 => return self.DIVR3_get(),
            I2C4MultConf::HSIDiv => return self.HSIDiv_get(),
            I2C4MultConf::CSIRC => return self.CSIRC_get(),
        };
    }
    pub fn I2C4output_get(&self) -> Result<f32, ClockError> {
        self.I2C4Mult_get()
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
        self.SPDIFMult_get()
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
        self.QSPIMult_get()
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
        self.FMCMult_get()
    }
    fn SWPMult_get(&self) -> Result<f32, ClockError> {
        match self.SWPMult {
            SWPMultConf::CDPPRE1 => return self.CDPPRE1_get(),
            SWPMultConf::HSIDiv => return self.HSIDiv_get(),
        };
    }
    pub fn SWPoutput_get(&self) -> Result<f32, ClockError> {
        self.SWPMult_get()
    }
    fn SDMMCMult_get(&self) -> Result<f32, ClockError> {
        match self.SDMMCMult {
            SDMMCMultConf::DIVQ1 => return self.DIVQ1_get(),
            SDMMCMultConf::DIVR2 => return self.DIVR2_get(),
        };
    }
    pub fn SDMMCoutput_get(&self) -> Result<f32, ClockError> {
        self.SDMMCMult_get()
    }
    fn DFSDMMult_get(&self) -> Result<f32, ClockError> {
        match self.DFSDMMult {
            DFSDMMultConf::CDPPRE2 => return self.CDPPRE2_get(),
            DFSDMMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
        };
    }
    pub fn DFSDMoutput_get(&self) -> Result<f32, ClockError> {
        self.DFSDMMult_get()
    }
    fn DFSDM2Mult_get(&self) -> Result<f32, ClockError> {
        match self.DFSDM2Mult {
            DFSDM2MultConf::SRDPPRE => return self.SRDPPRE_get(),
            DFSDM2MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
        };
    }
    pub fn DFSDM2output_get(&self) -> Result<f32, ClockError> {
        self.DFSDM2Mult_get()
    }
    fn USART16Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART16Mult {
            USART16MultConf::CDPPRE2 => return self.CDPPRE2_get(),
            USART16MultConf::DIVQ2 => return self.DIVQ2_get(),
            USART16MultConf::DIVQ3 => return self.DIVQ3_get(),
            USART16MultConf::HSIDiv => return self.HSIDiv_get(),
            USART16MultConf::CSIRC => return self.CSIRC_get(),
            USART16MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn USART16output_get(&self) -> Result<f32, ClockError> {
        self.USART16Mult_get()
    }
    fn USART234578Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART234578Mult {
            USART234578MultConf::CDPPRE1 => return self.CDPPRE1_get(),
            USART234578MultConf::DIVQ2 => return self.DIVQ2_get(),
            USART234578MultConf::DIVQ3 => return self.DIVQ3_get(),
            USART234578MultConf::HSIDiv => return self.HSIDiv_get(),
            USART234578MultConf::CSIRC => return self.CSIRC_get(),
            USART234578MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn USART234578output_get(&self) -> Result<f32, ClockError> {
        self.USART234578Mult_get()
    }
    fn LPUART1Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPUART1Mult {
            LPUART1MultConf::CDPPRE => return self.CDPPRE_get(),
            LPUART1MultConf::DIVQ2 => return self.DIVQ2_get(),
            LPUART1MultConf::DIVQ3 => return self.DIVQ3_get(),
            LPUART1MultConf::HSIDiv => return self.HSIDiv_get(),
            LPUART1MultConf::CSIRC => return self.CSIRC_get(),
            LPUART1MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn LPUART1output_get(&self) -> Result<f32, ClockError> {
        self.LPUART1Mult_get()
    }
    fn LPTIM1Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM1Mult {
            LPTIM1MultConf::CDPPRE1 => return self.CDPPRE1_get(),
            LPTIM1MultConf::DIVP2 => return self.DIVP2_get(),
            LPTIM1MultConf::DIVR3 => return self.DIVR3_get(),
            LPTIM1MultConf::LSEOSC => return self.LSEOSC_get(),
            LPTIM1MultConf::LSIRC => return self.LSIRC_get(),
            LPTIM1MultConf::CKPERSource => return self.CKPERSource_get(),
        };
    }
    pub fn LPTIM1output_get(&self) -> Result<f32, ClockError> {
        self.LPTIM1Mult_get()
    }
    fn LPTIM345Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM345Mult {
            LPTIM345MultConf::SRDPPRE => return self.SRDPPRE_get(),
            LPTIM345MultConf::DIVP2 => return self.DIVP2_get(),
            LPTIM345MultConf::DIVR3 => return self.DIVR3_get(),
            LPTIM345MultConf::LSEOSC => return self.LSEOSC_get(),
            LPTIM345MultConf::LSIRC => return self.LSIRC_get(),
            LPTIM345MultConf::CKPERSource => return self.CKPERSource_get(),
        };
    }
    pub fn LPTIM345output_get(&self) -> Result<f32, ClockError> {
        self.LPTIM345Mult_get()
    }
    fn LPTIM2Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM2Mult {
            LPTIM2MultConf::SRDPPRE => return self.SRDPPRE_get(),
            LPTIM2MultConf::DIVP2 => return self.DIVP2_get(),
            LPTIM2MultConf::DIVR3 => return self.DIVR3_get(),
            LPTIM2MultConf::LSEOSC => return self.LSEOSC_get(),
            LPTIM2MultConf::LSIRC => return self.LSIRC_get(),
            LPTIM2MultConf::CKPERSource => return self.CKPERSource_get(),
        };
    }
    pub fn LPTIM2output_get(&self) -> Result<f32, ClockError> {
        self.LPTIM2Mult_get()
    }
    fn SPI6Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI6Mult {
            SPI6MultConf::SRDPPRE => return self.SRDPPRE_get(),
            SPI6MultConf::DIVQ2 => return self.DIVQ2_get(),
            SPI6MultConf::DIVQ3 => return self.DIVQ3_get(),
            SPI6MultConf::HSIDiv => return self.HSIDiv_get(),
            SPI6MultConf::CSIRC => return self.CSIRC_get(),
            SPI6MultConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    pub fn SPI6output_get(&self) -> Result<f32, ClockError> {
        self.SPI6Mult_get()
    }
    pub fn DFSDM2ACLKoutput_get(&self) -> Result<f32, ClockError> {
        self.SPI6Mult_get()
    }
    fn SPI45Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI45Mult {
            SPI45MultConf::CDPPRE1 => return self.CDPPRE1_get(),
            SPI45MultConf::DIVQ2 => return self.DIVQ2_get(),
            SPI45MultConf::DIVQ3 => return self.DIVQ3_get(),
            SPI45MultConf::HSIDiv => return self.HSIDiv_get(),
            SPI45MultConf::CSIRC => return self.CSIRC_get(),
            SPI45MultConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    pub fn SPI45output_get(&self) -> Result<f32, ClockError> {
        self.SPI45Mult_get()
    }
    fn USBMult_get(&self) -> Result<f32, ClockError> {
        match self.USBMult {
            USBMultConf::DIVQ1 => return self.DIVQ1_get(),
            USBMultConf::DIVQ3 => return self.DIVQ3_get(),
            USBMultConf::RC48 => return self.RC48_get(),
        };
    }
    pub fn USBoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.USBMult_get()?;
        if input > (48000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48000000),
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
        self.FDCANMult_get()
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
        if input > (50000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 50000000),
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
    fn CECMult_get(&self) -> Result<f32, ClockError> {
        match self.CECMult {
            CECMultConf::LSEOSC => return self.LSEOSC_get(),
            CECMultConf::LSIRC => return self.LSIRC_get(),
            CECMultConf::CSIRC => return self.CSIRC_get(),
        };
    }
    pub fn CECoutput_get(&self) -> Result<f32, ClockError> {
        self.CECMult_get()
    }
}
