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
    Dig_CKIN,
    SysClkSource,
    SysCLKOutput,
    MCO1Mult,
    MCO1Div,
    MCO1Pin,
    MCO2Mult,
    MCO2Div,
    MCO2Pin,
    CPRE,
    CPREOutput,
    TPIUPrescaler,
    TPIUOutput,
    CpuClockOutput,
    CortexPrescaler,
    CortexSysOutput,
    BMPRE,
    AHBOutput,
    AXIClockOutput,
    AHB5Output,
    PPRE5,
    APB5Output,
    AHB1234Output,
    PPRE1,
    APB1Output,
    Tim1Mul,
    Tim1Output,
    PPRE2,
    APB2Output,
    Tim2Mul,
    Tim2Output,
    PPRE4,
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
    DIVS1,
    DIVS1output,
    DIVT1,
    DIVT1output,
    DIVN2,
    PLL2FRACN,
    DIVP2,
    DIVP2output,
    DIVQ2,
    DIVQ2output,
    DIVR2,
    DIVR2output,
    DIVS2,
    DIVS2output,
    DIVT2,
    DIVT2output,
    DIVN3,
    PLL3FRACN,
    DIVP3,
    DIVP3output,
    DIVQ3,
    DIVQ3output,
    DIVR3,
    DIVR3output,
    DIVS3,
    DIVS3output,
    DIVT3,
    DIVT3output,
    HSERTCDevisor,
    RTCClkSource,
    RTCOutput,
    IWDGOutput,
    UCPDoutput,
    HSI_DIV,
    SPI1Mult,
    SPI1output,
    SPI23Mult,
    SPI23output,
    SAI1Mult,
    SAI1output,
    SAI2Mult,
    SAI2output,
    I2C1Mult,
    I2C1output,
    I2C23Mult,
    I2C23output,
    SPDIFMult,
    SPDIFoutput,
    LTDCOutput,
    FMCMult,
    FMCoutput,
    SDMMCMult,
    SDMMCoutput,
    USART1Mult,
    USART1output,
    ADFMult,
    ADFoutput,
    USART234578Mult,
    USART234578output,
    LPUART1Mult,
    LPUART1output,
    LPTIM1Mult,
    LPTIM1output,
    LPTIM23Mult,
    LPTIM23output,
    LPTIM45Mult,
    LPTIM45output,
    SPI6Mult,
    SPI6output,
    SPI45Mult,
    SPI45output,
    HSEUSBPHYDevisor,
    USBPHYCLKMux,
    USBPHYCLKOutput,
    USBPHYRC,
    USBPHYRC60,
    USBOCLKMux,
    USBOFSCLKOutput,
    RNGOutput,
    DTSOutput,
    FDCANMult,
    FDCANoutput,
    XSPI1Mult,
    XSPI1output,
    PSSIMult,
    PSSIoutput,
    XSPI2Mult,
    XSPI2output,
    ETHPHYMult,
    ETHPHYoutput,
    ETH1Mult,
    ETH1output,
    ADCMult,
    ADCoutput,
    CECMult,
    CECoutput,
    CSICECDevisor,
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
    HSIDiv,
    HSEOSC,
    LSEOSC,
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
pub enum CPREConf {
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

impl CPREConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            CPREConf::DIV1 => return Ok(1.0),
            CPREConf::DIV2 => return Ok(2.0),
            CPREConf::DIV4 => return Ok(4.0),
            CPREConf::DIV8 => return Ok(8.0),
            CPREConf::DIV16 => return Ok(16.0),
            CPREConf::DIV64 => return Ok(64.0),
            CPREConf::DIV128 => return Ok(128.0),
            CPREConf::DIV256 => return Ok(256.0),
            CPREConf::DIV512 => return Ok(512.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum TPIUPrescalerConf {
    DIV3,
}

impl TPIUPrescalerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            TPIUPrescalerConf::DIV3 => return Ok(3.0),
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
pub enum BMPREConf {
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

impl BMPREConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            BMPREConf::DIV1 => return Ok(1.0),
            BMPREConf::DIV2 => return Ok(2.0),
            BMPREConf::DIV4 => return Ok(4.0),
            BMPREConf::DIV8 => return Ok(8.0),
            BMPREConf::DIV16 => return Ok(16.0),
            BMPREConf::DIV64 => return Ok(64.0),
            BMPREConf::DIV128 => return Ok(128.0),
            BMPREConf::DIV256 => return Ok(256.0),
            BMPREConf::DIV512 => return Ok(512.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PPRE5Conf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl PPRE5Conf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PPRE5Conf::DIV1 => return Ok(1.0),
            PPRE5Conf::DIV2 => return Ok(2.0),
            PPRE5Conf::DIV4 => return Ok(4.0),
            PPRE5Conf::DIV8 => return Ok(8.0),
            PPRE5Conf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PPRE1Conf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl PPRE1Conf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PPRE1Conf::DIV1 => return Ok(1.0),
            PPRE1Conf::DIV2 => return Ok(2.0),
            PPRE1Conf::DIV4 => return Ok(4.0),
            PPRE1Conf::DIV8 => return Ok(8.0),
            PPRE1Conf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PPRE2Conf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl PPRE2Conf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PPRE2Conf::DIV1 => return Ok(1.0),
            PPRE2Conf::DIV2 => return Ok(2.0),
            PPRE2Conf::DIV4 => return Ok(4.0),
            PPRE2Conf::DIV8 => return Ok(8.0),
            PPRE2Conf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PPRE4Conf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl PPRE4Conf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PPRE4Conf::DIV1 => return Ok(1.0),
            PPRE4Conf::DIV2 => return Ok(2.0),
            PPRE4Conf::DIV4 => return Ok(4.0),
            PPRE4Conf::DIV8 => return Ok(8.0),
            PPRE4Conf::DIV16 => return Ok(16.0),
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
    DIV1,
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
            DIVP1Conf::DIV1 => return Ok(1.0),
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
pub enum DIVS1Conf {
    Value(u32),
}

impl DIVS1Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        8
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVS1Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVS1,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVS1,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DIVT1Conf {
    Value(u32),
}

impl DIVT1Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        8
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVT1Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVT1,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVT1,
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
pub enum DIVS2Conf {
    Value(u32),
}

impl DIVS2Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        8
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVS2Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVS2,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVS2,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DIVT2Conf {
    Value(u32),
}

impl DIVT2Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        8
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVT2Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVT2,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVT2,
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
        12
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
pub enum DIVS3Conf {
    Value(u32),
}

impl DIVS3Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        8
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVS3Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVS3,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVS3,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DIVT3Conf {
    Value(u32),
}

impl DIVT3Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        8
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVT3Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVT3,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVT3,
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
pub enum HSI_DIVConf {
    DIV4,
}

impl HSI_DIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            HSI_DIVConf::DIV4 => return Ok(4.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI1MultConf {
    DIVQ1,
    DIVP2,
    DIVP3,
    I2S_CKIN,
    CKPERSource,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI23MultConf {
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
pub enum SAI2MultConf {
    DIVQ1,
    DIVP2,
    DIVP3,
    I2S_CKIN,
    CKPERSource,
    SPDIFMult,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C1MultConf {
    PPRE1,
    DIVR3,
    HSIDiv,
    CSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C23MultConf {
    PPRE1,
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
pub enum FMCMultConf {
    AHB5Output,
    DIVQ1,
    DIVR2,
    HSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SDMMCMultConf {
    DIVS2,
    DIVT2,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART1MultConf {
    PPRE2,
    DIVQ2,
    DIVQ3,
    HSIDiv,
    CSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ADFMultConf {
    AHBOutput,
    DIVP2,
    DIVP3,
    I2S_CKIN,
    CSIRC,
    HSIDiv,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART234578MultConf {
    PPRE1,
    DIVQ2,
    DIVQ3,
    HSIDiv,
    CSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPUART1MultConf {
    PPRE4,
    DIVQ2,
    DIVQ3,
    HSIDiv,
    CSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM1MultConf {
    PPRE1,
    DIVP2,
    DIVR3,
    LSEOSC,
    LSIRC,
    CKPERSource,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM23MultConf {
    PPRE4,
    DIVP2,
    DIVR3,
    LSEOSC,
    LSIRC,
    CKPERSource,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM45MultConf {
    PPRE4,
    DIVP2,
    DIVR3,
    LSEOSC,
    LSIRC,
    CKPERSource,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI6MultConf {
    PPRE4,
    DIVQ2,
    DIVQ3,
    HSIDiv,
    CSIRC,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI45MultConf {
    PPRE2,
    DIVQ2,
    DIVQ3,
    HSIDiv,
    CSIRC,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum HSEUSBPHYDevisorConf {
    DIV2,
}

impl HSEUSBPHYDevisorConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            HSEUSBPHYDevisorConf::DIV2 => return Ok(2.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USBPHYCLKMuxConf {
    HSEOSC,
    HSEUSBPHYDevisor,
    DIVQ3,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USBOCLKMuxConf {
    RC48,
    DIVQ3,
    HSEOSC,
    USBPHYRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FDCANMultConf {
    HSEOSC,
    DIVQ1,
    DIVP2,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XSPI1MultConf {
    AHB5Output,
    DIVS2,
    DIVT2,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PSSIMultConf {
    DIVR3,
    CKPERSource,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XSPI2MultConf {
    AHB5Output,
    DIVS2,
    DIVT2,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ETHPHYMultConf {
    HSEOSC,
    DIVS3,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ETH1MultConf {
    Dig_CKIN,
    HSEOSC,
    ETHPHYMult,
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
    CSICECDevisor,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CSICECDevisorConf {
    DIV122,
}

impl CSICECDevisorConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            CSICECDevisorConf::DIV122 => return Ok(122.0),
        }
    }
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
    pub CPRE: CPREConf,
    pub TPIUPrescaler: TPIUPrescalerConf,
    pub CortexPrescaler: CortexPrescalerConf,
    pub BMPRE: BMPREConf,
    pub PPRE5: PPRE5Conf,
    pub PPRE1: PPRE1Conf,
    pub PPRE2: PPRE2Conf,
    pub PPRE4: PPRE4Conf,
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
    pub DIVS1: DIVS1Conf,
    pub DIVT1: DIVT1Conf,
    pub DIVN2: DIVN2Conf,
    pub PLL2FRACN: PLL2FRACNConf,
    pub DIVP2: DIVP2Conf,
    pub DIVQ2: DIVQ2Conf,
    pub DIVR2: DIVR2Conf,
    pub DIVS2: DIVS2Conf,
    pub DIVT2: DIVT2Conf,
    pub DIVN3: DIVN3Conf,
    pub PLL3FRACN: PLL3FRACNConf,
    pub DIVP3: DIVP3Conf,
    pub DIVQ3: DIVQ3Conf,
    pub DIVR3: DIVR3Conf,
    pub DIVS3: DIVS3Conf,
    pub DIVT3: DIVT3Conf,
    pub HSERTCDevisor: HSERTCDevisorConf,
    pub RTCClkSource: RTCClkSourceConf,
    pub HSI_DIV: HSI_DIVConf,
    pub SPI1Mult: SPI1MultConf,
    pub SPI23Mult: SPI23MultConf,
    pub SAI1Mult: SAI1MultConf,
    pub SAI2Mult: SAI2MultConf,
    pub I2C1Mult: I2C1MultConf,
    pub I2C23Mult: I2C23MultConf,
    pub SPDIFMult: SPDIFMultConf,
    pub FMCMult: FMCMultConf,
    pub SDMMCMult: SDMMCMultConf,
    pub USART1Mult: USART1MultConf,
    pub ADFMult: ADFMultConf,
    pub USART234578Mult: USART234578MultConf,
    pub LPUART1Mult: LPUART1MultConf,
    pub LPTIM1Mult: LPTIM1MultConf,
    pub LPTIM23Mult: LPTIM23MultConf,
    pub LPTIM45Mult: LPTIM45MultConf,
    pub SPI6Mult: SPI6MultConf,
    pub SPI45Mult: SPI45MultConf,
    pub HSEUSBPHYDevisor: HSEUSBPHYDevisorConf,
    pub USBPHYCLKMux: USBPHYCLKMuxConf,
    pub USBOCLKMux: USBOCLKMuxConf,
    pub FDCANMult: FDCANMultConf,
    pub XSPI1Mult: XSPI1MultConf,
    pub PSSIMult: PSSIMultConf,
    pub XSPI2Mult: XSPI2MultConf,
    pub ETHPHYMult: ETHPHYMultConf,
    pub ETH1Mult: ETH1MultConf,
    pub ADCMult: ADCMultConf,
    pub CECMult: CECMultConf,
    pub CSICECDevisor: CSICECDevisorConf,
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
            CPRE: CPREConf::DIV1,
            TPIUPrescaler: TPIUPrescalerConf::DIV3,
            CortexPrescaler: CortexPrescalerConf::DIV1,
            BMPRE: BMPREConf::DIV1,
            PPRE5: PPRE5Conf::DIV1,
            PPRE1: PPRE1Conf::DIV1,
            PPRE2: PPRE2Conf::DIV1,
            PPRE4: PPRE4Conf::DIV1,
            PLLSource: PLLSourceConf::HSIDiv,
            CKPERSource: CKPERSourceConf::HSIDiv,
            DIVM1: DIVM1Conf::Value(32),
            DIVM2: DIVM2Conf::Value(32),
            DIVM3: DIVM3Conf::Value(32),
            DIVN1: DIVN1Conf::Value(128),
            PLLFRACN: PLLFRACNConf::Value(0),
            DIVP1: DIVP1Conf::DIV2,
            DIVQ1: DIVQ1Conf::Value(2),
            DIVR1: DIVR1Conf::Value(2),
            DIVS1: DIVS1Conf::Value(2),
            DIVT1: DIVT1Conf::Value(2),
            DIVN2: DIVN2Conf::Value(128),
            PLL2FRACN: PLL2FRACNConf::Value(0),
            DIVP2: DIVP2Conf::Value(2),
            DIVQ2: DIVQ2Conf::Value(2),
            DIVR2: DIVR2Conf::Value(2),
            DIVS2: DIVS2Conf::Value(2),
            DIVT2: DIVT2Conf::Value(2),
            DIVN3: DIVN3Conf::Value(128),
            PLL3FRACN: PLL3FRACNConf::Value(0),
            DIVP3: DIVP3Conf::Value(2),
            DIVQ3: DIVQ3Conf::Value(2),
            DIVR3: DIVR3Conf::Value(2),
            DIVS3: DIVS3Conf::Value(2),
            DIVT3: DIVT3Conf::Value(2),
            HSERTCDevisor: HSERTCDevisorConf::DIV2,
            RTCClkSource: RTCClkSourceConf::LSIRC,
            HSI_DIV: HSI_DIVConf::DIV4,
            SPI1Mult: SPI1MultConf::DIVQ1,
            SPI23Mult: SPI23MultConf::DIVQ1,
            SAI1Mult: SAI1MultConf::DIVQ1,
            SAI2Mult: SAI2MultConf::DIVQ1,
            I2C1Mult: I2C1MultConf::PPRE1,
            I2C23Mult: I2C23MultConf::PPRE1,
            SPDIFMult: SPDIFMultConf::DIVQ1,
            FMCMult: FMCMultConf::AHB5Output,
            SDMMCMult: SDMMCMultConf::DIVS2,
            USART1Mult: USART1MultConf::PPRE2,
            ADFMult: ADFMultConf::AHBOutput,
            USART234578Mult: USART234578MultConf::PPRE1,
            LPUART1Mult: LPUART1MultConf::PPRE4,
            LPTIM1Mult: LPTIM1MultConf::PPRE1,
            LPTIM23Mult: LPTIM23MultConf::PPRE4,
            LPTIM45Mult: LPTIM45MultConf::PPRE4,
            SPI6Mult: SPI6MultConf::PPRE4,
            SPI45Mult: SPI45MultConf::PPRE2,
            HSEUSBPHYDevisor: HSEUSBPHYDevisorConf::DIV2,
            USBPHYCLKMux: USBPHYCLKMuxConf::HSEOSC,
            USBOCLKMux: USBOCLKMuxConf::RC48,
            FDCANMult: FDCANMultConf::HSEOSC,
            XSPI1Mult: XSPI1MultConf::AHB5Output,
            PSSIMult: PSSIMultConf::DIVR3,
            XSPI2Mult: XSPI2MultConf::AHB5Output,
            ETHPHYMult: ETHPHYMultConf::HSEOSC,
            ETH1Mult: ETH1MultConf::Dig_CKIN,
            ADCMult: ADCMultConf::DIVP2,
            CECMult: CECMultConf::LSEOSC,
            CSICECDevisor: CSICECDevisorConf::DIV122,
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
    pub fn Dig_CKIN_get(&self) -> Result<f32, ClockError> {
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
        if input > (600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 600000000),
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
            MCO1MultConf::HSIDiv => return self.HSIDiv_get(),
            MCO1MultConf::HSEOSC => return self.HSEOSC_get(),
            MCO1MultConf::LSEOSC => return self.LSEOSC_get(),
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
    fn CPRE_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()? as f32;
        let value = self.CPRE.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn CPREOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.CPRE_get()?;
        if input > (600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 600000000),
                from: ClockNodes::CPRE,
                to: ClockNodes::CPREOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::CPRE,
                to: ClockNodes::CPREOutput,
            });
        }
        Ok(input)
    }
    fn TPIUPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.CPREOutput_get()? as f32;
        let value = self.TPIUPrescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn TPIUOutput_get(&self) -> Result<f32, ClockError> {
        self.TPIUPrescaler_get()
    }
    pub fn CpuClockOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.CPREOutput_get()?;
        if input > (600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 600000000),
                from: ClockNodes::CPREOutput,
                to: ClockNodes::CpuClockOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::CPREOutput,
                to: ClockNodes::CpuClockOutput,
            });
        }
        Ok(input)
    }
    fn CortexPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.CPREOutput_get()? as f32;
        let value = self.CortexPrescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn CortexSysOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.CortexPrescaler_get()?;
        if input > (600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 600000000),
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
    fn BMPRE_get(&self) -> Result<f32, ClockError> {
        let input = self.CPREOutput_get()? as f32;
        let value = self.BMPRE.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn AHBOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.BMPRE_get()?;
        if input > (300000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 300000000),
                from: ClockNodes::BMPRE,
                to: ClockNodes::AHBOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::BMPRE,
                to: ClockNodes::AHBOutput,
            });
        }
        Ok(input)
    }
    pub fn AXIClockOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()?;
        if input > (300000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 300000000),
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
    pub fn AHB5Output_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()?;
        if input > (300000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 300000000),
                from: ClockNodes::AHBOutput,
                to: ClockNodes::AHB5Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::AHBOutput,
                to: ClockNodes::AHB5Output,
            });
        }
        Ok(input)
    }
    fn PPRE5_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.PPRE5.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB5Output_get(&self) -> Result<f32, ClockError> {
        let input = self.PPRE5_get()?;
        if input > (150000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 150000000),
                from: ClockNodes::PPRE5,
                to: ClockNodes::APB5Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::PPRE5,
                to: ClockNodes::APB5Output,
            });
        }
        Ok(input)
    }
    pub fn AHB1234Output_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()?;
        if input > (300000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 300000000),
                from: ClockNodes::AHBOutput,
                to: ClockNodes::AHB1234Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::AHBOutput,
                to: ClockNodes::AHB1234Output,
            });
        }
        Ok(input)
    }
    fn PPRE1_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.PPRE1.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB1Output_get(&self) -> Result<f32, ClockError> {
        let input = self.PPRE1_get()?;
        if input > (150000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 150000000),
                from: ClockNodes::PPRE1,
                to: ClockNodes::APB1Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::PPRE1,
                to: ClockNodes::APB1Output,
            });
        }
        Ok(input)
    }
    fn Tim1Mul_get(&self) -> Result<f32, ClockError> {
        let input = self.PPRE1_get()? as f32;
        let value = 2 as f32;
        Ok((input * value) as f32)
    }

    pub fn Tim1Output_get(&self) -> Result<f32, ClockError> {
        self.Tim1Mul_get()
    }
    fn PPRE2_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.PPRE2.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB2Output_get(&self) -> Result<f32, ClockError> {
        let input = self.PPRE2_get()?;
        if input > (150000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 150000000),
                from: ClockNodes::PPRE2,
                to: ClockNodes::APB2Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::PPRE2,
                to: ClockNodes::APB2Output,
            });
        }
        Ok(input)
    }
    fn Tim2Mul_get(&self) -> Result<f32, ClockError> {
        let input = self.PPRE2_get()? as f32;
        let value = 2 as f32;
        Ok((input * value) as f32)
    }

    pub fn Tim2Output_get(&self) -> Result<f32, ClockError> {
        self.Tim2Mul_get()
    }
    fn PPRE4_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.PPRE4.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB4Output_get(&self) -> Result<f32, ClockError> {
        let input = self.PPRE4_get()?;
        if input > (150000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 150000000),
                from: ClockNodes::PPRE4,
                to: ClockNodes::APB4Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::PPRE4,
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
    fn DIVS1_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN1_get()? as f32;
        let value = self.DIVS1.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn DIVS1output_get(&self) -> Result<f32, ClockError> {
        self.DIVS1_get()
    }
    fn DIVT1_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN1_get()? as f32;
        let value = self.DIVT1.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn DIVT1output_get(&self) -> Result<f32, ClockError> {
        self.DIVT1_get()
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
    fn DIVS2_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN2_get()? as f32;
        let value = self.DIVS2.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn DIVS2output_get(&self) -> Result<f32, ClockError> {
        self.DIVS2_get()
    }
    fn DIVT2_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN2_get()? as f32;
        let value = self.DIVT2.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn DIVT2output_get(&self) -> Result<f32, ClockError> {
        self.DIVT2_get()
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

    pub fn DIVR3output_get(&self) -> Result<f32, ClockError> {
        self.DIVR3_get()
    }
    fn DIVS3_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN3_get()? as f32;
        let value = self.DIVS3.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn DIVS3output_get(&self) -> Result<f32, ClockError> {
        self.DIVS3_get()
    }
    fn DIVT3_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN3_get()? as f32;
        let value = self.DIVT3.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn DIVT3output_get(&self) -> Result<f32, ClockError> {
        self.DIVT3_get()
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
    pub fn UCPDoutput_get(&self) -> Result<f32, ClockError> {
        self.HSI_DIV_get()
    }
    fn HSI_DIV_get(&self) -> Result<f32, ClockError> {
        let input = self.HSIRC_get()? as f32;
        let value = self.HSI_DIV.get()? as f32;
        Ok((input / value) as f32)
    }

    fn SPI1Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI1Mult {
            SPI1MultConf::DIVQ1 => return self.DIVQ1_get(),
            SPI1MultConf::DIVP2 => return self.DIVP2_get(),
            SPI1MultConf::DIVP3 => return self.DIVP3_get(),
            SPI1MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            SPI1MultConf::CKPERSource => return self.CKPERSource_get(),
        };
    }
    pub fn SPI1output_get(&self) -> Result<f32, ClockError> {
        self.SPI1Mult_get()
    }
    fn SPI23Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI23Mult {
            SPI23MultConf::DIVQ1 => return self.DIVQ1_get(),
            SPI23MultConf::DIVP2 => return self.DIVP2_get(),
            SPI23MultConf::DIVP3 => return self.DIVP3_get(),
            SPI23MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            SPI23MultConf::CKPERSource => return self.CKPERSource_get(),
        };
    }
    pub fn SPI23output_get(&self) -> Result<f32, ClockError> {
        self.SPI23Mult_get()
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
    pub fn SAI1output_get(&self) -> Result<f32, ClockError> {
        self.SAI1Mult_get()
    }
    fn SAI2Mult_get(&self) -> Result<f32, ClockError> {
        match self.SAI2Mult {
            SAI2MultConf::DIVQ1 => return self.DIVQ1_get(),
            SAI2MultConf::DIVP2 => return self.DIVP2_get(),
            SAI2MultConf::DIVP3 => return self.DIVP3_get(),
            SAI2MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            SAI2MultConf::CKPERSource => return self.CKPERSource_get(),
            SAI2MultConf::SPDIFMult => return self.SPDIFMult_get(),
        };
    }
    pub fn SAI2output_get(&self) -> Result<f32, ClockError> {
        self.SAI2Mult_get()
    }
    fn I2C1Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C1Mult {
            I2C1MultConf::PPRE1 => return self.PPRE1_get(),
            I2C1MultConf::DIVR3 => return self.DIVR3_get(),
            I2C1MultConf::HSIDiv => return self.HSIDiv_get(),
            I2C1MultConf::CSIRC => return self.CSIRC_get(),
        };
    }
    pub fn I2C1output_get(&self) -> Result<f32, ClockError> {
        self.I2C1Mult_get()
    }
    fn I2C23Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C23Mult {
            I2C23MultConf::PPRE1 => return self.PPRE1_get(),
            I2C23MultConf::DIVR3 => return self.DIVR3_get(),
            I2C23MultConf::HSIDiv => return self.HSIDiv_get(),
            I2C23MultConf::CSIRC => return self.CSIRC_get(),
        };
    }
    pub fn I2C23output_get(&self) -> Result<f32, ClockError> {
        self.I2C23Mult_get()
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
    pub fn LTDCOutput_get(&self) -> Result<f32, ClockError> {
        self.DIVR3_get()
    }
    fn FMCMult_get(&self) -> Result<f32, ClockError> {
        match self.FMCMult {
            FMCMultConf::AHB5Output => return self.AHB5Output_get(),
            FMCMultConf::DIVQ1 => return self.DIVQ1_get(),
            FMCMultConf::DIVR2 => return self.DIVR2_get(),
            FMCMultConf::HSIRC => return self.HSIRC_get(),
        };
    }
    pub fn FMCoutput_get(&self) -> Result<f32, ClockError> {
        self.FMCMult_get()
    }
    fn SDMMCMult_get(&self) -> Result<f32, ClockError> {
        match self.SDMMCMult {
            SDMMCMultConf::DIVS2 => return self.DIVS2_get(),
            SDMMCMultConf::DIVT2 => return self.DIVT2_get(),
        };
    }
    pub fn SDMMCoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SDMMCMult_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
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
    fn USART1Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART1Mult {
            USART1MultConf::PPRE2 => return self.PPRE2_get(),
            USART1MultConf::DIVQ2 => return self.DIVQ2_get(),
            USART1MultConf::DIVQ3 => return self.DIVQ3_get(),
            USART1MultConf::HSIDiv => return self.HSIDiv_get(),
            USART1MultConf::CSIRC => return self.CSIRC_get(),
            USART1MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn USART1output_get(&self) -> Result<f32, ClockError> {
        self.USART1Mult_get()
    }
    fn ADFMult_get(&self) -> Result<f32, ClockError> {
        match self.ADFMult {
            ADFMultConf::AHBOutput => return self.AHBOutput_get(),
            ADFMultConf::DIVP2 => return self.DIVP2_get(),
            ADFMultConf::DIVP3 => return self.DIVP3_get(),
            ADFMultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            ADFMultConf::CSIRC => return self.CSIRC_get(),
            ADFMultConf::HSIDiv => return self.HSIDiv_get(),
        };
    }
    pub fn ADFoutput_get(&self) -> Result<f32, ClockError> {
        self.ADFMult_get()
    }
    fn USART234578Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART234578Mult {
            USART234578MultConf::PPRE1 => return self.PPRE1_get(),
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
            LPUART1MultConf::PPRE4 => return self.PPRE4_get(),
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
            LPTIM1MultConf::PPRE1 => return self.PPRE1_get(),
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
    fn LPTIM23Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM23Mult {
            LPTIM23MultConf::PPRE4 => return self.PPRE4_get(),
            LPTIM23MultConf::DIVP2 => return self.DIVP2_get(),
            LPTIM23MultConf::DIVR3 => return self.DIVR3_get(),
            LPTIM23MultConf::LSEOSC => return self.LSEOSC_get(),
            LPTIM23MultConf::LSIRC => return self.LSIRC_get(),
            LPTIM23MultConf::CKPERSource => return self.CKPERSource_get(),
        };
    }
    pub fn LPTIM23output_get(&self) -> Result<f32, ClockError> {
        self.LPTIM23Mult_get()
    }
    fn LPTIM45Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM45Mult {
            LPTIM45MultConf::PPRE4 => return self.PPRE4_get(),
            LPTIM45MultConf::DIVP2 => return self.DIVP2_get(),
            LPTIM45MultConf::DIVR3 => return self.DIVR3_get(),
            LPTIM45MultConf::LSEOSC => return self.LSEOSC_get(),
            LPTIM45MultConf::LSIRC => return self.LSIRC_get(),
            LPTIM45MultConf::CKPERSource => return self.CKPERSource_get(),
        };
    }
    pub fn LPTIM45output_get(&self) -> Result<f32, ClockError> {
        self.LPTIM45Mult_get()
    }
    fn SPI6Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI6Mult {
            SPI6MultConf::PPRE4 => return self.PPRE4_get(),
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
    fn SPI45Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI45Mult {
            SPI45MultConf::PPRE2 => return self.PPRE2_get(),
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
    fn HSEUSBPHYDevisor_get(&self) -> Result<f32, ClockError> {
        let input = self.HSEOSC_get()? as f32;
        let value = self.HSEUSBPHYDevisor.get()? as f32;
        Ok((input / value) as f32)
    }

    fn USBPHYCLKMux_get(&self) -> Result<f32, ClockError> {
        match self.USBPHYCLKMux {
            USBPHYCLKMuxConf::HSEOSC => return self.HSEOSC_get(),
            USBPHYCLKMuxConf::HSEUSBPHYDevisor => return self.HSEUSBPHYDevisor_get(),
            USBPHYCLKMuxConf::DIVQ3 => return self.DIVQ3_get(),
        };
    }
    pub fn USBPHYCLKOutput_get(&self) -> Result<f32, ClockError> {
        self.USBPHYCLKMux_get()
    }
    pub fn USBPHYRC_get(&self) -> Result<f32, ClockError> {
        Ok(48000000 as f32)
    }
    pub fn USBPHYRC60_get(&self) -> Result<f32, ClockError> {
        Ok(60000000 as f32)
    }
    fn USBOCLKMux_get(&self) -> Result<f32, ClockError> {
        match self.USBOCLKMux {
            USBOCLKMuxConf::RC48 => return self.RC48_get(),
            USBOCLKMuxConf::DIVQ3 => return self.DIVQ3_get(),
            USBOCLKMuxConf::HSEOSC => return self.HSEOSC_get(),
            USBOCLKMuxConf::USBPHYRC => return self.USBPHYRC_get(),
        };
    }
    pub fn USBOFSCLKOutput_get(&self) -> Result<f32, ClockError> {
        self.USBOCLKMux_get()
    }
    pub fn RNGOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.RC48_get()?;
        if input > (48000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48000000),
                from: ClockNodes::RC48,
                to: ClockNodes::RNGOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::RC48,
                to: ClockNodes::RNGOutput,
            });
        }
        Ok(input)
    }
    pub fn DTSOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.LSEOSC_get()?;
        if input > (90000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 90000000),
                from: ClockNodes::LSEOSC,
                to: ClockNodes::DTSOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::LSEOSC,
                to: ClockNodes::DTSOutput,
            });
        }
        Ok(input)
    }
    fn FDCANMult_get(&self) -> Result<f32, ClockError> {
        match self.FDCANMult {
            FDCANMultConf::HSEOSC => return self.HSEOSC_get(),
            FDCANMultConf::DIVQ1 => return self.DIVQ1_get(),
            FDCANMultConf::DIVP2 => return self.DIVP2_get(),
        };
    }
    pub fn FDCANoutput_get(&self) -> Result<f32, ClockError> {
        self.FDCANMult_get()
    }
    fn XSPI1Mult_get(&self) -> Result<f32, ClockError> {
        match self.XSPI1Mult {
            XSPI1MultConf::AHB5Output => return self.AHB5Output_get(),
            XSPI1MultConf::DIVS2 => return self.DIVS2_get(),
            XSPI1MultConf::DIVT2 => return self.DIVT2_get(),
        };
    }
    pub fn XSPI1output_get(&self) -> Result<f32, ClockError> {
        self.XSPI1Mult_get()
    }
    fn PSSIMult_get(&self) -> Result<f32, ClockError> {
        match self.PSSIMult {
            PSSIMultConf::DIVR3 => return self.DIVR3_get(),
            PSSIMultConf::CKPERSource => return self.CKPERSource_get(),
        };
    }
    pub fn PSSIoutput_get(&self) -> Result<f32, ClockError> {
        self.PSSIMult_get()
    }
    fn XSPI2Mult_get(&self) -> Result<f32, ClockError> {
        match self.XSPI2Mult {
            XSPI2MultConf::AHB5Output => return self.AHB5Output_get(),
            XSPI2MultConf::DIVS2 => return self.DIVS2_get(),
            XSPI2MultConf::DIVT2 => return self.DIVT2_get(),
        };
    }
    pub fn XSPI2output_get(&self) -> Result<f32, ClockError> {
        self.XSPI2Mult_get()
    }
    fn ETHPHYMult_get(&self) -> Result<f32, ClockError> {
        match self.ETHPHYMult {
            ETHPHYMultConf::HSEOSC => return self.HSEOSC_get(),
            ETHPHYMultConf::DIVS3 => return self.DIVS3_get(),
        };
    }
    pub fn ETHPHYoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.ETHPHYMult_get()?;
        if input > (50000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 50000000),
                from: ClockNodes::ETHPHYMult,
                to: ClockNodes::ETHPHYoutput,
            });
        } else if input < (25000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 25000000),
                from: ClockNodes::ETHPHYMult,
                to: ClockNodes::ETHPHYoutput,
            });
        }
        Ok(input)
    }
    fn ETH1Mult_get(&self) -> Result<f32, ClockError> {
        match self.ETH1Mult {
            ETH1MultConf::Dig_CKIN => return self.Dig_CKIN_get(),
            ETH1MultConf::HSEOSC => return self.HSEOSC_get(),
            ETH1MultConf::ETHPHYMult => return self.ETHPHYMult_get(),
        };
    }
    pub fn ETH1output_get(&self) -> Result<f32, ClockError> {
        self.ETH1Mult_get()
    }
    fn ADCMult_get(&self) -> Result<f32, ClockError> {
        match self.ADCMult {
            ADCMultConf::DIVP2 => return self.DIVP2_get(),
            ADCMultConf::DIVR3 => return self.DIVR3_get(),
            ADCMultConf::CKPERSource => return self.CKPERSource_get(),
        };
    }
    pub fn ADCoutput_get(&self) -> Result<f32, ClockError> {
        self.ADCMult_get()
    }
    fn CECMult_get(&self) -> Result<f32, ClockError> {
        match self.CECMult {
            CECMultConf::LSEOSC => return self.LSEOSC_get(),
            CECMultConf::LSIRC => return self.LSIRC_get(),
            CECMultConf::CSICECDevisor => return self.CSICECDevisor_get(),
        };
    }
    pub fn CECoutput_get(&self) -> Result<f32, ClockError> {
        self.CECMult_get()
    }
    fn CSICECDevisor_get(&self) -> Result<f32, ClockError> {
        let input = self.CSIRC_get()? as f32;
        let value = self.CSICECDevisor.get()? as f32;
        Ok((input / value) as f32)
    }
}
