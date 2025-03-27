#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ClockNodes {
    None,
    HSIRC,
    HSIDiv,
    HSIDivOutput,
    HSIDiv4,
    UCPDOutput,
    HSEOSC,
    HSEOSCDIV,
    HSEDIV,
    LSIRC,
    LSEOSC,
    MSIRC,
    I2S_CKIN,
    IC1,
    IC1Div,
    IC1Output,
    IC2,
    IC2Div,
    IC2Output,
    IC3,
    IC3Div,
    IC3Output,
    IC4,
    IC4Div,
    IC4Output,
    IC5,
    IC5Div,
    IC5Output,
    IC6,
    IC6Div,
    IC6Output,
    IC7,
    IC7Div,
    IC7Output,
    IC8,
    IC8Div,
    IC8Output,
    IC9,
    IC9Div,
    IC9Output,
    IC10,
    IC10Div,
    IC10Output,
    IC11,
    IC11Div,
    IC11Output,
    IC12,
    IC12Div,
    IC12Output,
    IC13,
    IC13Div,
    IC13Output,
    IC14,
    IC14Div,
    IC14Output,
    IC15,
    IC15Div,
    IC15Output,
    IC16,
    IC16Div,
    IC16Output,
    IC17,
    IC17Div,
    IC17Output,
    IC18,
    IC18Div,
    IC18Output,
    IC19,
    IC19Div,
    IC19Output,
    IC20,
    IC20Div,
    IC20Output,
    MCOMult,
    MCODiv,
    MCOPin,
    MCO2Mult,
    MCO2Div,
    MCO2Pin,
    CKPERSource,
    CKPERoutput,
    ADCMult,
    ADCDIV,
    ADCoutput,
    ADFMult,
    ADFoutput,
    MDF1Mult,
    MDFoutput,
    PSSIMult,
    PSSIoutput,
    FDCANMult,
    FDCANoutput,
    I2C1Mult,
    I2C1output,
    I2C2Mult,
    I2C2output,
    I2C3Mult,
    I2C3output,
    I2C4Mult,
    I2C4output,
    I3C1Mult,
    I3C1output,
    I3C2Mult,
    I3C2output,
    LPTIM1Mult,
    LPTIM1output,
    LPTIM3Mult,
    LPTIM3output,
    LPTIM2Mult,
    LPTIM2output,
    LPTIM4Mult,
    LPTIM4output,
    LPTIM5Mult,
    LPTIM5output,
    LTDCMult,
    LTDCoutput,
    DCMIPPMult,
    DCMIPPoutput,
    FMCMult,
    FMCoutput,
    SAI1Mult,
    SAI1output,
    SAI2Mult,
    SAI2output,
    USART1Mult,
    USART1output,
    USART2Mult,
    USART2output,
    USART3Mult,
    USART3output,
    UART4Mult,
    UART4output,
    UART5Mult,
    UART5output,
    USART6Mult,
    USART6output,
    UART7Mult,
    UART7output,
    UART8Mult,
    UART8output,
    UART9Mult,
    UART9output,
    LPUART1Mult,
    LPUART1output,
    USART10Mult,
    USART10output,
    SPI1Mult,
    SPI1output,
    SPI2Mult,
    SPI2output,
    SPI3Mult,
    SPI3output,
    SPI4Mult,
    SPI4output,
    SPI5Mult,
    SPI5output,
    SPI6Mult,
    SPI6output,
    XSPI1Mult,
    XSPI1output,
    XSPI2Mult,
    XSPI2output,
    OTGHS1Mult,
    OTGHS1output,
    OTGHS2Mult,
    OTGHS2output,
    XSPI3Mult,
    XSPI3output,
    OTGPHY1Mult,
    OTGPHY1output,
    OTGPHY2Mult,
    OTGPHY2output,
    SDMMC1Mult,
    SDMMC1output,
    SDMMC2Mult,
    SDMMC2output,
    ETH1Mult,
    ETH1output,
    SPDIFRX1Mult,
    SPDIFRX1output,
    SYSBClkSource,
    SYSCClkSource,
    SYSDClkSource,
    SYSBCLKOutput,
    SYSCCLKOutput,
    SYSDCLKOutput,
    SYSAClkSource,
    TPIUPrescaler,
    TPIUOutput,
    CortexPrescaler,
    CortexSysOutput,
    CpuClockOutput,
    AXIClockOutput,
    HPREDiv,
    APB4DIV,
    APB4Output,
    APB5DIV,
    APB5Output,
    TIMGDIV,
    TIMGOutput,
    APB1DIV,
    AHBOutput,
    APB1Output,
    APB2DIV,
    APB2Output,
    PLL1Source,
    FREFDIV1,
    PLL2Source,
    FREFDIV2,
    PLL3Source,
    FREFDIV3,
    PLL4Source,
    FREFDIV4,
    FBDIV1,
    PLL1FRACV,
    POSTDIV1_1,
    POSTDIV2_1,
    FOUTPOSTDIV1,
    FBDIV2,
    PLL2FRACV,
    POSTDIV1_2,
    POSTDIV2_2,
    FOUTPOSTDIV2,
    FBDIV3,
    PLL3FRACV,
    POSTDIV1_3,
    POSTDIV2_3,
    FOUTPOSTDIV3,
    FBDIV4,
    PLL4FRACV,
    POSTDIV1_4,
    POSTDIV2_4,
    FOUTPOSTDIV4,
    HSERTCDevisor,
    RTCClkSource,
    RTCOutput,
    IWDGOutput,
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
pub enum HSIDiv4Conf {
    DIV4,
}

impl HSIDiv4Conf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            HSIDiv4Conf::DIV4 => return Ok(4.0),
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
        8000000
    }

    pub const fn max() -> u32 {
        48000000
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
pub enum HSEOSCDIVConf {
    DIV1,
    DIV2,
}

impl HSEOSCDIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            HSEOSCDIVConf::DIV1 => return Ok(1.0),
            HSEOSCDIVConf::DIV2 => return Ok(2.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum HSEDIVConf {
    DIV2,
}

impl HSEDIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            HSEDIVConf::DIV2 => return Ok(2.0),
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
pub enum MSIRCConf {
    CLOCK_16,
    CLOCK_4,
}

impl MSIRCConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MSIRCConf::CLOCK_16 => return Ok(16.0),
            MSIRCConf::CLOCK_4 => return Ok(4.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC1Conf {
    FOUTPOSTDIV1,
    FOUTPOSTDIV2,
    FOUTPOSTDIV3,
    FOUTPOSTDIV4,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC1DivConf {
    Value(u32),
}

impl IC1DivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        256
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            IC1DivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC1Div,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC1Div,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC2Conf {
    FOUTPOSTDIV1,
    FOUTPOSTDIV2,
    FOUTPOSTDIV3,
    FOUTPOSTDIV4,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC2DivConf {
    Value(u32),
}

impl IC2DivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        256
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            IC2DivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC2Div,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC2Div,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC3Conf {
    FOUTPOSTDIV1,
    FOUTPOSTDIV2,
    FOUTPOSTDIV3,
    FOUTPOSTDIV4,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC3DivConf {
    Value(u32),
}

impl IC3DivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        256
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            IC3DivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC3Div,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC3Div,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC4Conf {
    FOUTPOSTDIV1,
    FOUTPOSTDIV2,
    FOUTPOSTDIV3,
    FOUTPOSTDIV4,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC4DivConf {
    Value(u32),
}

impl IC4DivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        256
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            IC4DivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC4Div,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC4Div,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC5Conf {
    FOUTPOSTDIV1,
    FOUTPOSTDIV2,
    FOUTPOSTDIV3,
    FOUTPOSTDIV4,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC5DivConf {
    Value(u32),
}

impl IC5DivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        256
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            IC5DivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC5Div,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC5Div,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC6Conf {
    FOUTPOSTDIV1,
    FOUTPOSTDIV2,
    FOUTPOSTDIV3,
    FOUTPOSTDIV4,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC6DivConf {
    Value(u32),
}

impl IC6DivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        256
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            IC6DivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC6Div,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC6Div,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC7Conf {
    FOUTPOSTDIV1,
    FOUTPOSTDIV2,
    FOUTPOSTDIV3,
    FOUTPOSTDIV4,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC7DivConf {
    Value(u32),
}

impl IC7DivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        256
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            IC7DivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC7Div,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC7Div,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC8Conf {
    FOUTPOSTDIV1,
    FOUTPOSTDIV2,
    FOUTPOSTDIV3,
    FOUTPOSTDIV4,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC8DivConf {
    Value(u32),
}

impl IC8DivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        256
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            IC8DivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC8Div,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC8Div,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC9Conf {
    FOUTPOSTDIV1,
    FOUTPOSTDIV2,
    FOUTPOSTDIV3,
    FOUTPOSTDIV4,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC9DivConf {
    Value(u32),
}

impl IC9DivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        256
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            IC9DivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC9Div,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC9Div,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC10Conf {
    FOUTPOSTDIV1,
    FOUTPOSTDIV2,
    FOUTPOSTDIV3,
    FOUTPOSTDIV4,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC10DivConf {
    Value(u32),
}

impl IC10DivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        256
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            IC10DivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC10Div,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC10Div,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC11Conf {
    FOUTPOSTDIV1,
    FOUTPOSTDIV2,
    FOUTPOSTDIV3,
    FOUTPOSTDIV4,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC11DivConf {
    Value(u32),
}

impl IC11DivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        256
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            IC11DivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC11Div,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC11Div,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC12Conf {
    FOUTPOSTDIV1,
    FOUTPOSTDIV2,
    FOUTPOSTDIV3,
    FOUTPOSTDIV4,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC12DivConf {
    Value(u32),
}

impl IC12DivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        256
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            IC12DivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC12Div,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC12Div,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC13Conf {
    FOUTPOSTDIV1,
    FOUTPOSTDIV2,
    FOUTPOSTDIV3,
    FOUTPOSTDIV4,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC13DivConf {
    Value(u32),
}

impl IC13DivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        256
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            IC13DivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC13Div,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC13Div,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC14Conf {
    FOUTPOSTDIV1,
    FOUTPOSTDIV2,
    FOUTPOSTDIV3,
    FOUTPOSTDIV4,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC14DivConf {
    Value(u32),
}

impl IC14DivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        256
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            IC14DivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC14Div,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC14Div,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC15Conf {
    FOUTPOSTDIV1,
    FOUTPOSTDIV2,
    FOUTPOSTDIV3,
    FOUTPOSTDIV4,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC15DivConf {
    Value(u32),
}

impl IC15DivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        256
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            IC15DivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC15Div,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC15Div,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC16Conf {
    FOUTPOSTDIV1,
    FOUTPOSTDIV2,
    FOUTPOSTDIV3,
    FOUTPOSTDIV4,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC16DivConf {
    Value(u32),
}

impl IC16DivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        256
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            IC16DivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC16Div,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC16Div,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC17Conf {
    FOUTPOSTDIV1,
    FOUTPOSTDIV2,
    FOUTPOSTDIV3,
    FOUTPOSTDIV4,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC17DivConf {
    Value(u32),
}

impl IC17DivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        256
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            IC17DivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC17Div,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC17Div,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC18Conf {
    FOUTPOSTDIV1,
    FOUTPOSTDIV2,
    FOUTPOSTDIV3,
    FOUTPOSTDIV4,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC18DivConf {
    Value(u32),
}

impl IC18DivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        256
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            IC18DivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC18Div,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC18Div,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC19Conf {
    FOUTPOSTDIV1,
    FOUTPOSTDIV2,
    FOUTPOSTDIV3,
    FOUTPOSTDIV4,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC19DivConf {
    Value(u32),
}

impl IC19DivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        256
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            IC19DivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC19Div,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC19Div,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC20Conf {
    FOUTPOSTDIV1,
    FOUTPOSTDIV2,
    FOUTPOSTDIV3,
    FOUTPOSTDIV4,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum IC20DivConf {
    Value(u32),
}

impl IC20DivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        256
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            IC20DivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC20Div,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::IC20Div,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCOMultConf {
    HSIDivOutput,
    LSEOSC,
    MSIRC,
    LSIRC,
    HSEOSC,
    IC5Output,
    IC10Output,
    SYSAClkSource,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCODivConf {
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

impl MCODivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MCODivConf::DIV1 => return Ok(1.0),
            MCODivConf::DIV2 => return Ok(2.0),
            MCODivConf::DIV3 => return Ok(3.0),
            MCODivConf::DIV4 => return Ok(4.0),
            MCODivConf::DIV5 => return Ok(5.0),
            MCODivConf::DIV6 => return Ok(6.0),
            MCODivConf::DIV7 => return Ok(7.0),
            MCODivConf::DIV8 => return Ok(8.0),
            MCODivConf::DIV9 => return Ok(9.0),
            MCODivConf::DIV10 => return Ok(10.0),
            MCODivConf::DIV11 => return Ok(11.0),
            MCODivConf::DIV12 => return Ok(12.0),
            MCODivConf::DIV13 => return Ok(13.0),
            MCODivConf::DIV14 => return Ok(14.0),
            MCODivConf::DIV15 => return Ok(15.0),
            MCODivConf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCO2MultConf {
    HSIDivOutput,
    LSEOSC,
    MSIRC,
    LSIRC,
    HSEOSC,
    IC5Output,
    IC10Output,
    SYSBClkSource,
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
    DIV16,
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
            MCO2DivConf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CKPERSourceConf {
    HSIRC,
    MSIRC,
    HSEOSC,
    IC5Output,
    IC10Output,
    IC15Output,
    IC19Output,
    IC20Output,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ADCMultConf {
    AHBOutput,
    CKPERoutput,
    IC7Output,
    IC8Output,
    MSIRC,
    HSIDivOutput,
    I2S_CKIN,
    TIMGOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ADCDIVConf {
    Value(u32),
}

impl ADCDIVConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        256
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            ADCDIVConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::ADCDIV,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::ADCDIV,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ADFMultConf {
    AHBOutput,
    CKPERoutput,
    IC7Output,
    IC8Output,
    MSIRC,
    HSIDivOutput,
    I2S_CKIN,
    TIMGOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MDF1MultConf {
    AHBOutput,
    CKPERoutput,
    IC7Output,
    IC8Output,
    MSIRC,
    HSIDivOutput,
    I2S_CKIN,
    TIMGOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PSSIMultConf {
    AHBOutput,
    CKPERoutput,
    IC20Output,
    HSIDivOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FDCANMultConf {
    APB1Output,
    CKPERoutput,
    IC19Output,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C1MultConf {
    APB1Output,
    CKPERoutput,
    IC10Output,
    IC15Output,
    MSIRC,
    HSIDivOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C2MultConf {
    APB1Output,
    CKPERoutput,
    IC10Output,
    IC15Output,
    MSIRC,
    HSIDivOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C3MultConf {
    APB1Output,
    CKPERoutput,
    IC10Output,
    IC15Output,
    MSIRC,
    HSIDivOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C4MultConf {
    APB1Output,
    CKPERoutput,
    IC10Output,
    IC15Output,
    MSIRC,
    HSIDivOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I3C1MultConf {
    APB1Output,
    CKPERoutput,
    IC10Output,
    IC15Output,
    MSIRC,
    HSIDivOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I3C2MultConf {
    APB1Output,
    CKPERoutput,
    IC10Output,
    IC15Output,
    MSIRC,
    HSIDivOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM1MultConf {
    APB1Output,
    CKPERoutput,
    IC15Output,
    LSEOSC,
    LSIRC,
    TIMGOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM3MultConf {
    APB4Output,
    CKPERoutput,
    IC15Output,
    LSEOSC,
    LSIRC,
    TIMGOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM2MultConf {
    APB4Output,
    CKPERoutput,
    IC15Output,
    LSEOSC,
    LSIRC,
    TIMGOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM4MultConf {
    APB4Output,
    CKPERoutput,
    IC15Output,
    LSEOSC,
    LSIRC,
    TIMGOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM5MultConf {
    APB4Output,
    CKPERoutput,
    IC15Output,
    LSEOSC,
    LSIRC,
    TIMGOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LTDCMultConf {
    APB5Output,
    CKPERoutput,
    IC16Output,
    HSIDivOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DCMIPPMultConf {
    APB5Output,
    CKPERoutput,
    IC17Output,
    HSIDivOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FMCMultConf {
    AHBOutput,
    CKPERoutput,
    IC3Output,
    IC4Output,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SAI1MultConf {
    APB2Output,
    CKPERoutput,
    IC7Output,
    IC8Output,
    MSIRC,
    HSIDivOutput,
    I2S_CKIN,
    SPDIFRX1Mult,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SAI2MultConf {
    APB2Output,
    CKPERoutput,
    IC7Output,
    IC8Output,
    MSIRC,
    HSIDivOutput,
    I2S_CKIN,
    SPDIFRX1Mult,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART1MultConf {
    APB2Output,
    CKPERoutput,
    IC9Output,
    IC14Output,
    LSEOSC,
    MSIRC,
    HSIDivOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART2MultConf {
    APB1Output,
    CKPERoutput,
    IC9Output,
    IC14Output,
    LSEOSC,
    MSIRC,
    HSIDivOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART3MultConf {
    APB1Output,
    CKPERoutput,
    IC9Output,
    IC14Output,
    LSEOSC,
    MSIRC,
    HSIDivOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum UART4MultConf {
    APB1Output,
    CKPERoutput,
    IC9Output,
    IC14Output,
    LSEOSC,
    MSIRC,
    HSIDivOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum UART5MultConf {
    APB1Output,
    CKPERoutput,
    IC9Output,
    IC14Output,
    LSEOSC,
    MSIRC,
    HSIDivOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART6MultConf {
    APB2Output,
    CKPERoutput,
    IC9Output,
    IC14Output,
    LSEOSC,
    MSIRC,
    HSIDivOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum UART7MultConf {
    APB1Output,
    CKPERoutput,
    IC9Output,
    IC14Output,
    LSEOSC,
    MSIRC,
    HSIDivOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum UART8MultConf {
    APB1Output,
    CKPERoutput,
    IC9Output,
    IC14Output,
    LSEOSC,
    MSIRC,
    HSIDivOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum UART9MultConf {
    APB2Output,
    CKPERoutput,
    IC9Output,
    IC14Output,
    LSEOSC,
    MSIRC,
    HSIDivOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPUART1MultConf {
    APB4Output,
    CKPERoutput,
    IC9Output,
    IC14Output,
    LSEOSC,
    MSIRC,
    HSIDivOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART10MultConf {
    APB2Output,
    CKPERoutput,
    IC9Output,
    IC14Output,
    LSEOSC,
    MSIRC,
    HSIDivOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI1MultConf {
    APB2Output,
    CKPERoutput,
    IC8Output,
    IC9Output,
    MSIRC,
    HSIDivOutput,
    I2S_CKIN,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI2MultConf {
    APB1Output,
    CKPERoutput,
    IC8Output,
    IC9Output,
    MSIRC,
    HSIDivOutput,
    I2S_CKIN,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI3MultConf {
    APB1Output,
    CKPERoutput,
    IC8Output,
    IC9Output,
    MSIRC,
    HSIDivOutput,
    I2S_CKIN,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI4MultConf {
    APB2Output,
    CKPERoutput,
    IC9Output,
    IC14Output,
    MSIRC,
    HSIDivOutput,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI5MultConf {
    APB2Output,
    CKPERoutput,
    IC9Output,
    IC14Output,
    MSIRC,
    HSIDivOutput,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI6MultConf {
    APB4Output,
    CKPERoutput,
    IC8Output,
    IC9Output,
    MSIRC,
    HSIDivOutput,
    I2S_CKIN,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XSPI1MultConf {
    AHBOutput,
    CKPERoutput,
    IC3Output,
    IC4Output,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XSPI2MultConf {
    AHBOutput,
    CKPERoutput,
    IC3Output,
    IC4Output,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum OTGHS1MultConf {
    OTGPHY1output,
    HSEOSCDIV,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum OTGHS2MultConf {
    OTGPHY2output,
    HSEOSCDIV,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XSPI3MultConf {
    AHBOutput,
    CKPERoutput,
    IC3Output,
    IC4Output,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum OTGPHY1MultConf {
    HSEOSCDIV,
    CKPERoutput,
    IC15Output,
    HSEDIV,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum OTGPHY2MultConf {
    HSEOSCDIV,
    CKPERoutput,
    IC15Output,
    HSEDIV,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SDMMC1MultConf {
    AHBOutput,
    CKPERoutput,
    IC4Output,
    IC5Output,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SDMMC2MultConf {
    AHBOutput,
    CKPERoutput,
    IC4Output,
    IC5Output,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ETH1MultConf {
    AHBOutput,
    CKPERoutput,
    IC12Output,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPDIFRX1MultConf {
    APB1Output,
    CKPERoutput,
    IC7Output,
    IC8Output,
    MSIRC,
    HSIDivOutput,
    I2S_CKIN,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SYSBClkSourceConf {
    HSIRC,
    MSIRC,
    HSEOSC,
    IC2Output,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SYSCClkSourceConf {
    HSIRC,
    MSIRC,
    HSEOSC,
    IC6Output,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SYSDClkSourceConf {
    HSIRC,
    MSIRC,
    HSEOSC,
    IC11Output,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SYSAClkSourceConf {
    HSIRC,
    MSIRC,
    HSEOSC,
    IC1Output,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum TPIUPrescalerConf {
    DIV8,
}

impl TPIUPrescalerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            TPIUPrescalerConf::DIV8 => return Ok(8.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CortexPrescalerConf {
    DIV8,
}

impl CortexPrescalerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            CortexPrescalerConf::DIV8 => return Ok(8.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum HPREDivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
    DIV32,
    DIV64,
    DIV128,
}

impl HPREDivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            HPREDivConf::DIV1 => return Ok(1.0),
            HPREDivConf::DIV2 => return Ok(2.0),
            HPREDivConf::DIV4 => return Ok(4.0),
            HPREDivConf::DIV8 => return Ok(8.0),
            HPREDivConf::DIV16 => return Ok(16.0),
            HPREDivConf::DIV32 => return Ok(32.0),
            HPREDivConf::DIV64 => return Ok(64.0),
            HPREDivConf::DIV128 => return Ok(128.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum APB4DIVConf {
    DIV1,
}

impl APB4DIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            APB4DIVConf::DIV1 => return Ok(1.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum APB5DIVConf {
    DIV1,
}

impl APB5DIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            APB5DIVConf::DIV1 => return Ok(1.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum TIMGDIVConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
}

impl TIMGDIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            TIMGDIVConf::DIV1 => return Ok(1.0),
            TIMGDIVConf::DIV2 => return Ok(2.0),
            TIMGDIVConf::DIV4 => return Ok(4.0),
            TIMGDIVConf::DIV8 => return Ok(8.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum APB1DIVConf {
    DIV1,
}

impl APB1DIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            APB1DIVConf::DIV1 => return Ok(1.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum APB2DIVConf {
    DIV1,
}

impl APB2DIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            APB2DIVConf::DIV1 => return Ok(1.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL1SourceConf {
    HSIRC,
    MSIRC,
    HSEOSC,
    I2S_CKIN,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FREFDIV1Conf {
    Value(u32),
}

impl FREFDIV1Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        63
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            FREFDIV1Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::FREFDIV1,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::FREFDIV1,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL2SourceConf {
    HSIRC,
    MSIRC,
    HSEOSC,
    I2S_CKIN,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FREFDIV2Conf {
    Value(u32),
}

impl FREFDIV2Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        63
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            FREFDIV2Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::FREFDIV2,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::FREFDIV2,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL3SourceConf {
    HSIRC,
    MSIRC,
    HSEOSC,
    I2S_CKIN,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FREFDIV3Conf {
    Value(u32),
}

impl FREFDIV3Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        63
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            FREFDIV3Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::FREFDIV3,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::FREFDIV3,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL4SourceConf {
    HSIRC,
    MSIRC,
    HSEOSC,
    I2S_CKIN,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FREFDIV4Conf {
    Value(u32),
}

impl FREFDIV4Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        63
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            FREFDIV4Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::FREFDIV4,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::FREFDIV4,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FBDIV1Conf {
    Value(u32),
}

impl FBDIV1Conf {
    pub const fn min() -> u32 {
        10
    }

    pub const fn max() -> u32 {
        2500
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            FBDIV1Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::FBDIV1,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::FBDIV1,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL1FRACVConf {
    Value(u32),
}

impl PLL1FRACVConf {
    pub const fn min() -> u32 {
        0
    }

    pub const fn max() -> u32 {
        16777215
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL1FRACVConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL1FRACV,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL1FRACV,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum POSTDIV1_1Conf {
    Value(u32),
}

impl POSTDIV1_1Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        7
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            POSTDIV1_1Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV1_1,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV1_1,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum POSTDIV2_1Conf {
    Value(u32),
}

impl POSTDIV2_1Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        7
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            POSTDIV2_1Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV2_1,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV2_1,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FBDIV2Conf {
    Value(u32),
}

impl FBDIV2Conf {
    pub const fn min() -> u32 {
        20
    }

    pub const fn max() -> u32 {
        500
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            FBDIV2Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::FBDIV2,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::FBDIV2,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL2FRACVConf {
    Value(u32),
}

impl PLL2FRACVConf {
    pub const fn min() -> u32 {
        0
    }

    pub const fn max() -> u32 {
        16777215
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL2FRACVConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL2FRACV,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL2FRACV,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum POSTDIV1_2Conf {
    Value(u32),
}

impl POSTDIV1_2Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        7
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            POSTDIV1_2Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV1_2,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV1_2,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum POSTDIV2_2Conf {
    Value(u32),
}

impl POSTDIV2_2Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        7
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            POSTDIV2_2Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV2_2,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV2_2,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FBDIV3Conf {
    Value(u32),
}

impl FBDIV3Conf {
    pub const fn min() -> u32 {
        20
    }

    pub const fn max() -> u32 {
        500
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            FBDIV3Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::FBDIV3,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::FBDIV3,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL3FRACVConf {
    Value(u32),
}

impl PLL3FRACVConf {
    pub const fn min() -> u32 {
        0
    }

    pub const fn max() -> u32 {
        16777215
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL3FRACVConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL3FRACV,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL3FRACV,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum POSTDIV1_3Conf {
    Value(u32),
}

impl POSTDIV1_3Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        7
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            POSTDIV1_3Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV1_3,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV1_3,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum POSTDIV2_3Conf {
    Value(u32),
}

impl POSTDIV2_3Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        7
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            POSTDIV2_3Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV2_3,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV2_3,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FBDIV4Conf {
    Value(u32),
}

impl FBDIV4Conf {
    pub const fn min() -> u32 {
        20
    }

    pub const fn max() -> u32 {
        500
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            FBDIV4Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::FBDIV4,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::FBDIV4,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL4FRACVConf {
    Value(u32),
}

impl PLL4FRACVConf {
    pub const fn min() -> u32 {
        0
    }

    pub const fn max() -> u32 {
        16777215
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL4FRACVConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL4FRACV,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL4FRACV,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum POSTDIV1_4Conf {
    Value(u32),
}

impl POSTDIV1_4Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        7
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            POSTDIV1_4Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV1_4,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV1_4,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum POSTDIV2_4Conf {
    Value(u32),
}

impl POSTDIV2_4Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        7
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            POSTDIV2_4Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV2_4,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV2_4,
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
    Value(u32),
}

impl HSERTCDevisorConf {
    pub const fn min() -> u32 {
        2
    }

    pub const fn max() -> u32 {
        63
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            HSERTCDevisorConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::HSERTCDevisor,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::HSERTCDevisor,
                    });
                }
                Ok(*val as f32)
            }
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
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ClockTree {
    pub HSIDiv: HSIDivConf,
    pub HSIDiv4: HSIDiv4Conf,
    pub HSEOSC: HSEOSCConf,
    pub HSEOSCDIV: HSEOSCDIVConf,
    pub HSEDIV: HSEDIVConf,
    pub LSEOSC: LSEOSCConf,
    pub MSIRC: MSIRCConf,
    pub IC1: IC1Conf,
    pub IC1Div: IC1DivConf,
    pub IC2: IC2Conf,
    pub IC2Div: IC2DivConf,
    pub IC3: IC3Conf,
    pub IC3Div: IC3DivConf,
    pub IC4: IC4Conf,
    pub IC4Div: IC4DivConf,
    pub IC5: IC5Conf,
    pub IC5Div: IC5DivConf,
    pub IC6: IC6Conf,
    pub IC6Div: IC6DivConf,
    pub IC7: IC7Conf,
    pub IC7Div: IC7DivConf,
    pub IC8: IC8Conf,
    pub IC8Div: IC8DivConf,
    pub IC9: IC9Conf,
    pub IC9Div: IC9DivConf,
    pub IC10: IC10Conf,
    pub IC10Div: IC10DivConf,
    pub IC11: IC11Conf,
    pub IC11Div: IC11DivConf,
    pub IC12: IC12Conf,
    pub IC12Div: IC12DivConf,
    pub IC13: IC13Conf,
    pub IC13Div: IC13DivConf,
    pub IC14: IC14Conf,
    pub IC14Div: IC14DivConf,
    pub IC15: IC15Conf,
    pub IC15Div: IC15DivConf,
    pub IC16: IC16Conf,
    pub IC16Div: IC16DivConf,
    pub IC17: IC17Conf,
    pub IC17Div: IC17DivConf,
    pub IC18: IC18Conf,
    pub IC18Div: IC18DivConf,
    pub IC19: IC19Conf,
    pub IC19Div: IC19DivConf,
    pub IC20: IC20Conf,
    pub IC20Div: IC20DivConf,
    pub MCOMult: MCOMultConf,
    pub MCODiv: MCODivConf,
    pub MCO2Mult: MCO2MultConf,
    pub MCO2Div: MCO2DivConf,
    pub CKPERSource: CKPERSourceConf,
    pub ADCMult: ADCMultConf,
    pub ADCDIV: ADCDIVConf,
    pub ADFMult: ADFMultConf,
    pub MDF1Mult: MDF1MultConf,
    pub PSSIMult: PSSIMultConf,
    pub FDCANMult: FDCANMultConf,
    pub I2C1Mult: I2C1MultConf,
    pub I2C2Mult: I2C2MultConf,
    pub I2C3Mult: I2C3MultConf,
    pub I2C4Mult: I2C4MultConf,
    pub I3C1Mult: I3C1MultConf,
    pub I3C2Mult: I3C2MultConf,
    pub LPTIM1Mult: LPTIM1MultConf,
    pub LPTIM3Mult: LPTIM3MultConf,
    pub LPTIM2Mult: LPTIM2MultConf,
    pub LPTIM4Mult: LPTIM4MultConf,
    pub LPTIM5Mult: LPTIM5MultConf,
    pub LTDCMult: LTDCMultConf,
    pub DCMIPPMult: DCMIPPMultConf,
    pub FMCMult: FMCMultConf,
    pub SAI1Mult: SAI1MultConf,
    pub SAI2Mult: SAI2MultConf,
    pub USART1Mult: USART1MultConf,
    pub USART2Mult: USART2MultConf,
    pub USART3Mult: USART3MultConf,
    pub UART4Mult: UART4MultConf,
    pub UART5Mult: UART5MultConf,
    pub USART6Mult: USART6MultConf,
    pub UART7Mult: UART7MultConf,
    pub UART8Mult: UART8MultConf,
    pub UART9Mult: UART9MultConf,
    pub LPUART1Mult: LPUART1MultConf,
    pub USART10Mult: USART10MultConf,
    pub SPI1Mult: SPI1MultConf,
    pub SPI2Mult: SPI2MultConf,
    pub SPI3Mult: SPI3MultConf,
    pub SPI4Mult: SPI4MultConf,
    pub SPI5Mult: SPI5MultConf,
    pub SPI6Mult: SPI6MultConf,
    pub XSPI1Mult: XSPI1MultConf,
    pub XSPI2Mult: XSPI2MultConf,
    pub OTGHS1Mult: OTGHS1MultConf,
    pub OTGHS2Mult: OTGHS2MultConf,
    pub XSPI3Mult: XSPI3MultConf,
    pub OTGPHY1Mult: OTGPHY1MultConf,
    pub OTGPHY2Mult: OTGPHY2MultConf,
    pub SDMMC1Mult: SDMMC1MultConf,
    pub SDMMC2Mult: SDMMC2MultConf,
    pub ETH1Mult: ETH1MultConf,
    pub SPDIFRX1Mult: SPDIFRX1MultConf,
    pub SYSBClkSource: SYSBClkSourceConf,
    pub SYSCClkSource: SYSCClkSourceConf,
    pub SYSDClkSource: SYSDClkSourceConf,
    pub SYSAClkSource: SYSAClkSourceConf,
    pub TPIUPrescaler: TPIUPrescalerConf,
    pub CortexPrescaler: CortexPrescalerConf,
    pub HPREDiv: HPREDivConf,
    pub APB4DIV: APB4DIVConf,
    pub APB5DIV: APB5DIVConf,
    pub TIMGDIV: TIMGDIVConf,
    pub APB1DIV: APB1DIVConf,
    pub APB2DIV: APB2DIVConf,
    pub PLL1Source: PLL1SourceConf,
    pub FREFDIV1: FREFDIV1Conf,
    pub PLL2Source: PLL2SourceConf,
    pub FREFDIV2: FREFDIV2Conf,
    pub PLL3Source: PLL3SourceConf,
    pub FREFDIV3: FREFDIV3Conf,
    pub PLL4Source: PLL4SourceConf,
    pub FREFDIV4: FREFDIV4Conf,
    pub FBDIV1: FBDIV1Conf,
    pub PLL1FRACV: PLL1FRACVConf,
    pub POSTDIV1_1: POSTDIV1_1Conf,
    pub POSTDIV2_1: POSTDIV2_1Conf,
    pub FBDIV2: FBDIV2Conf,
    pub PLL2FRACV: PLL2FRACVConf,
    pub POSTDIV1_2: POSTDIV1_2Conf,
    pub POSTDIV2_2: POSTDIV2_2Conf,
    pub FBDIV3: FBDIV3Conf,
    pub PLL3FRACV: PLL3FRACVConf,
    pub POSTDIV1_3: POSTDIV1_3Conf,
    pub POSTDIV2_3: POSTDIV2_3Conf,
    pub FBDIV4: FBDIV4Conf,
    pub PLL4FRACV: PLL4FRACVConf,
    pub POSTDIV1_4: POSTDIV1_4Conf,
    pub POSTDIV2_4: POSTDIV2_4Conf,
    pub HSERTCDevisor: HSERTCDevisorConf,
    pub RTCClkSource: RTCClkSourceConf,
}

impl Default for ClockTree {
    fn default() -> Self {
        Self {
            HSIDiv: HSIDivConf::DIV1,
            HSIDiv4: HSIDiv4Conf::DIV4,
            HSEOSC: HSEOSCConf::Value(48000000),
            HSEOSCDIV: HSEOSCDIVConf::DIV1,
            HSEDIV: HSEDIVConf::DIV2,
            LSEOSC: LSEOSCConf::Value(32768),
            MSIRC: MSIRCConf::CLOCK_16,
            IC1: IC1Conf::FOUTPOSTDIV1,
            IC1Div: IC1DivConf::Value(3),
            IC2: IC2Conf::FOUTPOSTDIV1,
            IC2Div: IC2DivConf::Value(4),
            IC3: IC3Conf::FOUTPOSTDIV1,
            IC3Div: IC3DivConf::Value(1),
            IC4: IC4Conf::FOUTPOSTDIV1,
            IC4Div: IC4DivConf::Value(1),
            IC5: IC5Conf::FOUTPOSTDIV1,
            IC5Div: IC5DivConf::Value(1),
            IC6: IC6Conf::FOUTPOSTDIV1,
            IC6Div: IC6DivConf::Value(4),
            IC7: IC7Conf::FOUTPOSTDIV2,
            IC7Div: IC7DivConf::Value(1),
            IC8: IC8Conf::FOUTPOSTDIV2,
            IC8Div: IC8DivConf::Value(1),
            IC9: IC9Conf::FOUTPOSTDIV2,
            IC9Div: IC9DivConf::Value(1),
            IC10: IC10Conf::FOUTPOSTDIV2,
            IC10Div: IC10DivConf::Value(1),
            IC11: IC11Conf::FOUTPOSTDIV1,
            IC11Div: IC11DivConf::Value(4),
            IC12: IC12Conf::FOUTPOSTDIV3,
            IC12Div: IC12DivConf::Value(1),
            IC13: IC13Conf::FOUTPOSTDIV3,
            IC13Div: IC13DivConf::Value(1),
            IC14: IC14Conf::FOUTPOSTDIV3,
            IC14Div: IC14DivConf::Value(1),
            IC15: IC15Conf::FOUTPOSTDIV3,
            IC15Div: IC15DivConf::Value(1),
            IC16: IC16Conf::FOUTPOSTDIV4,
            IC16Div: IC16DivConf::Value(1),
            IC17: IC17Conf::FOUTPOSTDIV4,
            IC17Div: IC17DivConf::Value(1),
            IC18: IC18Conf::FOUTPOSTDIV4,
            IC18Div: IC18DivConf::Value(1),
            IC19: IC19Conf::FOUTPOSTDIV4,
            IC19Div: IC19DivConf::Value(1),
            IC20: IC20Conf::FOUTPOSTDIV4,
            IC20Div: IC20DivConf::Value(1),
            MCOMult: MCOMultConf::HSIDivOutput,
            MCODiv: MCODivConf::DIV1,
            MCO2Mult: MCO2MultConf::HSIDivOutput,
            MCO2Div: MCO2DivConf::DIV1,
            CKPERSource: CKPERSourceConf::HSIRC,
            ADCMult: ADCMultConf::AHBOutput,
            ADCDIV: ADCDIVConf::Value(1),
            ADFMult: ADFMultConf::AHBOutput,
            MDF1Mult: MDF1MultConf::AHBOutput,
            PSSIMult: PSSIMultConf::AHBOutput,
            FDCANMult: FDCANMultConf::APB1Output,
            I2C1Mult: I2C1MultConf::APB1Output,
            I2C2Mult: I2C2MultConf::APB1Output,
            I2C3Mult: I2C3MultConf::APB1Output,
            I2C4Mult: I2C4MultConf::APB1Output,
            I3C1Mult: I3C1MultConf::APB1Output,
            I3C2Mult: I3C2MultConf::APB1Output,
            LPTIM1Mult: LPTIM1MultConf::APB1Output,
            LPTIM3Mult: LPTIM3MultConf::APB4Output,
            LPTIM2Mult: LPTIM2MultConf::APB4Output,
            LPTIM4Mult: LPTIM4MultConf::APB4Output,
            LPTIM5Mult: LPTIM5MultConf::APB4Output,
            LTDCMult: LTDCMultConf::APB5Output,
            DCMIPPMult: DCMIPPMultConf::APB5Output,
            FMCMult: FMCMultConf::AHBOutput,
            SAI1Mult: SAI1MultConf::APB2Output,
            SAI2Mult: SAI2MultConf::APB2Output,
            USART1Mult: USART1MultConf::APB2Output,
            USART2Mult: USART2MultConf::APB1Output,
            USART3Mult: USART3MultConf::APB1Output,
            UART4Mult: UART4MultConf::APB1Output,
            UART5Mult: UART5MultConf::APB1Output,
            USART6Mult: USART6MultConf::APB2Output,
            UART7Mult: UART7MultConf::APB1Output,
            UART8Mult: UART8MultConf::APB1Output,
            UART9Mult: UART9MultConf::APB2Output,
            LPUART1Mult: LPUART1MultConf::APB4Output,
            USART10Mult: USART10MultConf::APB2Output,
            SPI1Mult: SPI1MultConf::APB2Output,
            SPI2Mult: SPI2MultConf::APB1Output,
            SPI3Mult: SPI3MultConf::APB1Output,
            SPI4Mult: SPI4MultConf::APB2Output,
            SPI5Mult: SPI5MultConf::APB2Output,
            SPI6Mult: SPI6MultConf::APB4Output,
            XSPI1Mult: XSPI1MultConf::AHBOutput,
            XSPI2Mult: XSPI2MultConf::AHBOutput,
            OTGHS1Mult: OTGHS1MultConf::OTGPHY1output,
            OTGHS2Mult: OTGHS2MultConf::OTGPHY2output,
            XSPI3Mult: XSPI3MultConf::AHBOutput,
            OTGPHY1Mult: OTGPHY1MultConf::HSEDIV,
            OTGPHY2Mult: OTGPHY2MultConf::HSEDIV,
            SDMMC1Mult: SDMMC1MultConf::AHBOutput,
            SDMMC2Mult: SDMMC2MultConf::AHBOutput,
            ETH1Mult: ETH1MultConf::AHBOutput,
            SPDIFRX1Mult: SPDIFRX1MultConf::APB1Output,
            SYSBClkSource: SYSBClkSourceConf::HSIRC,
            SYSCClkSource: SYSCClkSourceConf::HSIRC,
            SYSDClkSource: SYSDClkSourceConf::HSIRC,
            SYSAClkSource: SYSAClkSourceConf::HSIRC,
            TPIUPrescaler: TPIUPrescalerConf::DIV8,
            CortexPrescaler: CortexPrescalerConf::DIV8,
            HPREDiv: HPREDivConf::DIV2,
            APB4DIV: APB4DIVConf::DIV1,
            APB5DIV: APB5DIVConf::DIV1,
            TIMGDIV: TIMGDIVConf::DIV1,
            APB1DIV: APB1DIVConf::DIV1,
            APB2DIV: APB2DIVConf::DIV1,
            PLL1Source: PLL1SourceConf::HSIRC,
            FREFDIV1: FREFDIV1Conf::Value(1),
            PLL2Source: PLL2SourceConf::HSIRC,
            FREFDIV2: FREFDIV2Conf::Value(1),
            PLL3Source: PLL3SourceConf::HSIRC,
            FREFDIV3: FREFDIV3Conf::Value(1),
            PLL4Source: PLL4SourceConf::HSIRC,
            FREFDIV4: FREFDIV4Conf::Value(1),
            FBDIV1: FBDIV1Conf::Value(25),
            PLL1FRACV: PLL1FRACVConf::Value(0),
            POSTDIV1_1: POSTDIV1_1Conf::Value(1),
            POSTDIV2_1: POSTDIV2_1Conf::Value(1),
            FBDIV2: FBDIV2Conf::Value(25),
            PLL2FRACV: PLL2FRACVConf::Value(0),
            POSTDIV1_2: POSTDIV1_2Conf::Value(1),
            POSTDIV2_2: POSTDIV2_2Conf::Value(1),
            FBDIV3: FBDIV3Conf::Value(25),
            PLL3FRACV: PLL3FRACVConf::Value(0),
            POSTDIV1_3: POSTDIV1_3Conf::Value(1),
            POSTDIV2_3: POSTDIV2_3Conf::Value(1),
            FBDIV4: FBDIV4Conf::Value(25),
            PLL4FRACV: PLL4FRACVConf::Value(0),
            POSTDIV1_4: POSTDIV1_4Conf::Value(1),
            POSTDIV2_4: POSTDIV2_4Conf::Value(1),
            HSERTCDevisor: HSERTCDevisorConf::Value(1),
            RTCClkSource: RTCClkSourceConf::LSIRC,
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

    pub fn HSIDivOutput_get(&self) -> Result<f32, ClockError> {
        self.HSIDiv_get()
    }
    fn HSIDiv4_get(&self) -> Result<f32, ClockError> {
        let input = self.HSIRC_get()? as f32;
        let value = self.HSIDiv4.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn UCPDOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.HSIDiv4_get()?;
        if input > (25000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 25000000),
                from: ClockNodes::HSIDiv4,
                to: ClockNodes::UCPDOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::HSIDiv4,
                to: ClockNodes::UCPDOutput,
            });
        }
        Ok(input)
    }
    pub fn HSEOSC_get(&self) -> Result<f32, ClockError> {
        self.HSEOSC.get()
    }
    fn HSEOSCDIV_get(&self) -> Result<f32, ClockError> {
        let input = self.HSEOSC_get()? as f32;
        let value = self.HSEOSCDIV.get()? as f32;
        Ok((input / value) as f32)
    }

    fn HSEDIV_get(&self) -> Result<f32, ClockError> {
        let input = self.HSEOSC_get()? as f32;
        let value = self.HSEDIV.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn LSIRC_get(&self) -> Result<f32, ClockError> {
        Ok(32000 as f32)
    }
    pub fn LSEOSC_get(&self) -> Result<f32, ClockError> {
        self.LSEOSC.get()
    }
    pub fn MSIRC_get(&self) -> Result<f32, ClockError> {
        self.MSIRC.get()
    }
    pub fn I2S_CKIN_get(&self) -> Result<f32, ClockError> {
        Ok(12288000 as f32)
    }
    fn IC1_get(&self) -> Result<f32, ClockError> {
        match self.IC1 {
            IC1Conf::FOUTPOSTDIV1 => return self.FOUTPOSTDIV1_get(),
            IC1Conf::FOUTPOSTDIV2 => return self.FOUTPOSTDIV2_get(),
            IC1Conf::FOUTPOSTDIV3 => return self.FOUTPOSTDIV3_get(),
            IC1Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
        };
    }
    fn IC1Div_get(&self) -> Result<f32, ClockError> {
        let input = self.IC1_get()? as f32;
        let value = self.IC1Div.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn IC1Output_get(&self) -> Result<f32, ClockError> {
        let input = self.IC1Div_get()?;
        if input > (1600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1600000000),
                from: ClockNodes::IC1Div,
                to: ClockNodes::IC1Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::IC1Div,
                to: ClockNodes::IC1Output,
            });
        }
        Ok(input)
    }
    fn IC2_get(&self) -> Result<f32, ClockError> {
        match self.IC2 {
            IC2Conf::FOUTPOSTDIV1 => return self.FOUTPOSTDIV1_get(),
            IC2Conf::FOUTPOSTDIV2 => return self.FOUTPOSTDIV2_get(),
            IC2Conf::FOUTPOSTDIV3 => return self.FOUTPOSTDIV3_get(),
            IC2Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
        };
    }
    fn IC2Div_get(&self) -> Result<f32, ClockError> {
        let input = self.IC2_get()? as f32;
        let value = self.IC2Div.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn IC2Output_get(&self) -> Result<f32, ClockError> {
        let input = self.IC2Div_get()?;
        if input > (1600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1600000000),
                from: ClockNodes::IC2Div,
                to: ClockNodes::IC2Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::IC2Div,
                to: ClockNodes::IC2Output,
            });
        }
        Ok(input)
    }
    fn IC3_get(&self) -> Result<f32, ClockError> {
        match self.IC3 {
            IC3Conf::FOUTPOSTDIV1 => return self.FOUTPOSTDIV1_get(),
            IC3Conf::FOUTPOSTDIV2 => return self.FOUTPOSTDIV2_get(),
            IC3Conf::FOUTPOSTDIV3 => return self.FOUTPOSTDIV3_get(),
            IC3Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
        };
    }
    fn IC3Div_get(&self) -> Result<f32, ClockError> {
        let input = self.IC3_get()? as f32;
        let value = self.IC3Div.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn IC3Output_get(&self) -> Result<f32, ClockError> {
        let input = self.IC3Div_get()?;
        if input > (1600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1600000000),
                from: ClockNodes::IC3Div,
                to: ClockNodes::IC3Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::IC3Div,
                to: ClockNodes::IC3Output,
            });
        }
        Ok(input)
    }
    fn IC4_get(&self) -> Result<f32, ClockError> {
        match self.IC4 {
            IC4Conf::FOUTPOSTDIV1 => return self.FOUTPOSTDIV1_get(),
            IC4Conf::FOUTPOSTDIV2 => return self.FOUTPOSTDIV2_get(),
            IC4Conf::FOUTPOSTDIV3 => return self.FOUTPOSTDIV3_get(),
            IC4Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
        };
    }
    fn IC4Div_get(&self) -> Result<f32, ClockError> {
        let input = self.IC4_get()? as f32;
        let value = self.IC4Div.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn IC4Output_get(&self) -> Result<f32, ClockError> {
        let input = self.IC4Div_get()?;
        if input > (1600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1600000000),
                from: ClockNodes::IC4Div,
                to: ClockNodes::IC4Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::IC4Div,
                to: ClockNodes::IC4Output,
            });
        }
        Ok(input)
    }
    fn IC5_get(&self) -> Result<f32, ClockError> {
        match self.IC5 {
            IC5Conf::FOUTPOSTDIV1 => return self.FOUTPOSTDIV1_get(),
            IC5Conf::FOUTPOSTDIV2 => return self.FOUTPOSTDIV2_get(),
            IC5Conf::FOUTPOSTDIV3 => return self.FOUTPOSTDIV3_get(),
            IC5Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
        };
    }
    fn IC5Div_get(&self) -> Result<f32, ClockError> {
        let input = self.IC5_get()? as f32;
        let value = self.IC5Div.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn IC5Output_get(&self) -> Result<f32, ClockError> {
        let input = self.IC5Div_get()?;
        if input > (1600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1600000000),
                from: ClockNodes::IC5Div,
                to: ClockNodes::IC5Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::IC5Div,
                to: ClockNodes::IC5Output,
            });
        }
        Ok(input)
    }
    fn IC6_get(&self) -> Result<f32, ClockError> {
        match self.IC6 {
            IC6Conf::FOUTPOSTDIV1 => return self.FOUTPOSTDIV1_get(),
            IC6Conf::FOUTPOSTDIV2 => return self.FOUTPOSTDIV2_get(),
            IC6Conf::FOUTPOSTDIV3 => return self.FOUTPOSTDIV3_get(),
            IC6Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
        };
    }
    fn IC6Div_get(&self) -> Result<f32, ClockError> {
        let input = self.IC6_get()? as f32;
        let value = self.IC6Div.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn IC6Output_get(&self) -> Result<f32, ClockError> {
        let input = self.IC6Div_get()?;
        if input > (1600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1600000000),
                from: ClockNodes::IC6Div,
                to: ClockNodes::IC6Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::IC6Div,
                to: ClockNodes::IC6Output,
            });
        }
        Ok(input)
    }
    fn IC7_get(&self) -> Result<f32, ClockError> {
        match self.IC7 {
            IC7Conf::FOUTPOSTDIV1 => return self.FOUTPOSTDIV1_get(),
            IC7Conf::FOUTPOSTDIV2 => return self.FOUTPOSTDIV2_get(),
            IC7Conf::FOUTPOSTDIV3 => return self.FOUTPOSTDIV3_get(),
            IC7Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
        };
    }
    fn IC7Div_get(&self) -> Result<f32, ClockError> {
        let input = self.IC7_get()? as f32;
        let value = self.IC7Div.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn IC7Output_get(&self) -> Result<f32, ClockError> {
        let input = self.IC7Div_get()?;
        if input > (1600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1600000000),
                from: ClockNodes::IC7Div,
                to: ClockNodes::IC7Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::IC7Div,
                to: ClockNodes::IC7Output,
            });
        }
        Ok(input)
    }
    fn IC8_get(&self) -> Result<f32, ClockError> {
        match self.IC8 {
            IC8Conf::FOUTPOSTDIV1 => return self.FOUTPOSTDIV1_get(),
            IC8Conf::FOUTPOSTDIV2 => return self.FOUTPOSTDIV2_get(),
            IC8Conf::FOUTPOSTDIV3 => return self.FOUTPOSTDIV3_get(),
            IC8Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
        };
    }
    fn IC8Div_get(&self) -> Result<f32, ClockError> {
        let input = self.IC8_get()? as f32;
        let value = self.IC8Div.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn IC8Output_get(&self) -> Result<f32, ClockError> {
        let input = self.IC8Div_get()?;
        if input > (1600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1600000000),
                from: ClockNodes::IC8Div,
                to: ClockNodes::IC8Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::IC8Div,
                to: ClockNodes::IC8Output,
            });
        }
        Ok(input)
    }
    fn IC9_get(&self) -> Result<f32, ClockError> {
        match self.IC9 {
            IC9Conf::FOUTPOSTDIV1 => return self.FOUTPOSTDIV1_get(),
            IC9Conf::FOUTPOSTDIV2 => return self.FOUTPOSTDIV2_get(),
            IC9Conf::FOUTPOSTDIV3 => return self.FOUTPOSTDIV3_get(),
            IC9Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
        };
    }
    fn IC9Div_get(&self) -> Result<f32, ClockError> {
        let input = self.IC9_get()? as f32;
        let value = self.IC9Div.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn IC9Output_get(&self) -> Result<f32, ClockError> {
        let input = self.IC9Div_get()?;
        if input > (1600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1600000000),
                from: ClockNodes::IC9Div,
                to: ClockNodes::IC9Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::IC9Div,
                to: ClockNodes::IC9Output,
            });
        }
        Ok(input)
    }
    fn IC10_get(&self) -> Result<f32, ClockError> {
        match self.IC10 {
            IC10Conf::FOUTPOSTDIV1 => return self.FOUTPOSTDIV1_get(),
            IC10Conf::FOUTPOSTDIV2 => return self.FOUTPOSTDIV2_get(),
            IC10Conf::FOUTPOSTDIV3 => return self.FOUTPOSTDIV3_get(),
            IC10Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
        };
    }
    fn IC10Div_get(&self) -> Result<f32, ClockError> {
        let input = self.IC10_get()? as f32;
        let value = self.IC10Div.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn IC10Output_get(&self) -> Result<f32, ClockError> {
        let input = self.IC10Div_get()?;
        if input > (1600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1600000000),
                from: ClockNodes::IC10Div,
                to: ClockNodes::IC10Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::IC10Div,
                to: ClockNodes::IC10Output,
            });
        }
        Ok(input)
    }
    fn IC11_get(&self) -> Result<f32, ClockError> {
        match self.IC11 {
            IC11Conf::FOUTPOSTDIV1 => return self.FOUTPOSTDIV1_get(),
            IC11Conf::FOUTPOSTDIV2 => return self.FOUTPOSTDIV2_get(),
            IC11Conf::FOUTPOSTDIV3 => return self.FOUTPOSTDIV3_get(),
            IC11Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
        };
    }
    fn IC11Div_get(&self) -> Result<f32, ClockError> {
        let input = self.IC11_get()? as f32;
        let value = self.IC11Div.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn IC11Output_get(&self) -> Result<f32, ClockError> {
        let input = self.IC11Div_get()?;
        if input > (1600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1600000000),
                from: ClockNodes::IC11Div,
                to: ClockNodes::IC11Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::IC11Div,
                to: ClockNodes::IC11Output,
            });
        }
        Ok(input)
    }
    fn IC12_get(&self) -> Result<f32, ClockError> {
        match self.IC12 {
            IC12Conf::FOUTPOSTDIV1 => return self.FOUTPOSTDIV1_get(),
            IC12Conf::FOUTPOSTDIV2 => return self.FOUTPOSTDIV2_get(),
            IC12Conf::FOUTPOSTDIV3 => return self.FOUTPOSTDIV3_get(),
            IC12Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
        };
    }
    fn IC12Div_get(&self) -> Result<f32, ClockError> {
        let input = self.IC12_get()? as f32;
        let value = self.IC12Div.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn IC12Output_get(&self) -> Result<f32, ClockError> {
        let input = self.IC12Div_get()?;
        if input > (1600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1600000000),
                from: ClockNodes::IC12Div,
                to: ClockNodes::IC12Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::IC12Div,
                to: ClockNodes::IC12Output,
            });
        }
        Ok(input)
    }
    fn IC13_get(&self) -> Result<f32, ClockError> {
        match self.IC13 {
            IC13Conf::FOUTPOSTDIV1 => return self.FOUTPOSTDIV1_get(),
            IC13Conf::FOUTPOSTDIV2 => return self.FOUTPOSTDIV2_get(),
            IC13Conf::FOUTPOSTDIV3 => return self.FOUTPOSTDIV3_get(),
            IC13Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
        };
    }
    fn IC13Div_get(&self) -> Result<f32, ClockError> {
        let input = self.IC13_get()? as f32;
        let value = self.IC13Div.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn IC13Output_get(&self) -> Result<f32, ClockError> {
        let input = self.IC13Div_get()?;
        if input > (1600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1600000000),
                from: ClockNodes::IC13Div,
                to: ClockNodes::IC13Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::IC13Div,
                to: ClockNodes::IC13Output,
            });
        }
        Ok(input)
    }
    fn IC14_get(&self) -> Result<f32, ClockError> {
        match self.IC14 {
            IC14Conf::FOUTPOSTDIV1 => return self.FOUTPOSTDIV1_get(),
            IC14Conf::FOUTPOSTDIV2 => return self.FOUTPOSTDIV2_get(),
            IC14Conf::FOUTPOSTDIV3 => return self.FOUTPOSTDIV3_get(),
            IC14Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
        };
    }
    fn IC14Div_get(&self) -> Result<f32, ClockError> {
        let input = self.IC14_get()? as f32;
        let value = self.IC14Div.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn IC14Output_get(&self) -> Result<f32, ClockError> {
        let input = self.IC14Div_get()?;
        if input > (1600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1600000000),
                from: ClockNodes::IC14Div,
                to: ClockNodes::IC14Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::IC14Div,
                to: ClockNodes::IC14Output,
            });
        }
        Ok(input)
    }
    fn IC15_get(&self) -> Result<f32, ClockError> {
        match self.IC15 {
            IC15Conf::FOUTPOSTDIV1 => return self.FOUTPOSTDIV1_get(),
            IC15Conf::FOUTPOSTDIV2 => return self.FOUTPOSTDIV2_get(),
            IC15Conf::FOUTPOSTDIV3 => return self.FOUTPOSTDIV3_get(),
            IC15Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
        };
    }
    fn IC15Div_get(&self) -> Result<f32, ClockError> {
        let input = self.IC15_get()? as f32;
        let value = self.IC15Div.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn IC15Output_get(&self) -> Result<f32, ClockError> {
        let input = self.IC15Div_get()?;
        if input > (1600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1600000000),
                from: ClockNodes::IC15Div,
                to: ClockNodes::IC15Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::IC15Div,
                to: ClockNodes::IC15Output,
            });
        }
        Ok(input)
    }
    fn IC16_get(&self) -> Result<f32, ClockError> {
        match self.IC16 {
            IC16Conf::FOUTPOSTDIV1 => return self.FOUTPOSTDIV1_get(),
            IC16Conf::FOUTPOSTDIV2 => return self.FOUTPOSTDIV2_get(),
            IC16Conf::FOUTPOSTDIV3 => return self.FOUTPOSTDIV3_get(),
            IC16Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
        };
    }
    fn IC16Div_get(&self) -> Result<f32, ClockError> {
        let input = self.IC16_get()? as f32;
        let value = self.IC16Div.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn IC16Output_get(&self) -> Result<f32, ClockError> {
        let input = self.IC16Div_get()?;
        if input > (1600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1600000000),
                from: ClockNodes::IC16Div,
                to: ClockNodes::IC16Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::IC16Div,
                to: ClockNodes::IC16Output,
            });
        }
        Ok(input)
    }
    fn IC17_get(&self) -> Result<f32, ClockError> {
        match self.IC17 {
            IC17Conf::FOUTPOSTDIV1 => return self.FOUTPOSTDIV1_get(),
            IC17Conf::FOUTPOSTDIV2 => return self.FOUTPOSTDIV2_get(),
            IC17Conf::FOUTPOSTDIV3 => return self.FOUTPOSTDIV3_get(),
            IC17Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
        };
    }
    fn IC17Div_get(&self) -> Result<f32, ClockError> {
        let input = self.IC17_get()? as f32;
        let value = self.IC17Div.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn IC17Output_get(&self) -> Result<f32, ClockError> {
        let input = self.IC17Div_get()?;
        if input > (1600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1600000000),
                from: ClockNodes::IC17Div,
                to: ClockNodes::IC17Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::IC17Div,
                to: ClockNodes::IC17Output,
            });
        }
        Ok(input)
    }
    fn IC18_get(&self) -> Result<f32, ClockError> {
        match self.IC18 {
            IC18Conf::FOUTPOSTDIV1 => return self.FOUTPOSTDIV1_get(),
            IC18Conf::FOUTPOSTDIV2 => return self.FOUTPOSTDIV2_get(),
            IC18Conf::FOUTPOSTDIV3 => return self.FOUTPOSTDIV3_get(),
            IC18Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
        };
    }
    fn IC18Div_get(&self) -> Result<f32, ClockError> {
        let input = self.IC18_get()? as f32;
        let value = self.IC18Div.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn IC18Output_get(&self) -> Result<f32, ClockError> {
        let input = self.IC18Div_get()?;
        if input > (1600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1600000000),
                from: ClockNodes::IC18Div,
                to: ClockNodes::IC18Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::IC18Div,
                to: ClockNodes::IC18Output,
            });
        }
        Ok(input)
    }
    fn IC19_get(&self) -> Result<f32, ClockError> {
        match self.IC19 {
            IC19Conf::FOUTPOSTDIV1 => return self.FOUTPOSTDIV1_get(),
            IC19Conf::FOUTPOSTDIV2 => return self.FOUTPOSTDIV2_get(),
            IC19Conf::FOUTPOSTDIV3 => return self.FOUTPOSTDIV3_get(),
            IC19Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
        };
    }
    fn IC19Div_get(&self) -> Result<f32, ClockError> {
        let input = self.IC19_get()? as f32;
        let value = self.IC19Div.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn IC19Output_get(&self) -> Result<f32, ClockError> {
        let input = self.IC19Div_get()?;
        if input > (1600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1600000000),
                from: ClockNodes::IC19Div,
                to: ClockNodes::IC19Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::IC19Div,
                to: ClockNodes::IC19Output,
            });
        }
        Ok(input)
    }
    fn IC20_get(&self) -> Result<f32, ClockError> {
        match self.IC20 {
            IC20Conf::FOUTPOSTDIV1 => return self.FOUTPOSTDIV1_get(),
            IC20Conf::FOUTPOSTDIV2 => return self.FOUTPOSTDIV2_get(),
            IC20Conf::FOUTPOSTDIV3 => return self.FOUTPOSTDIV3_get(),
            IC20Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
        };
    }
    fn IC20Div_get(&self) -> Result<f32, ClockError> {
        let input = self.IC20_get()? as f32;
        let value = self.IC20Div.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn IC20Output_get(&self) -> Result<f32, ClockError> {
        let input = self.IC20Div_get()?;
        if input > (1600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1600000000),
                from: ClockNodes::IC20Div,
                to: ClockNodes::IC20Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::IC20Div,
                to: ClockNodes::IC20Output,
            });
        }
        Ok(input)
    }
    fn MCOMult_get(&self) -> Result<f32, ClockError> {
        match self.MCOMult {
            MCOMultConf::HSIDivOutput => return self.HSIDivOutput_get(),
            MCOMultConf::LSEOSC => return self.LSEOSC_get(),
            MCOMultConf::MSIRC => return self.MSIRC_get(),
            MCOMultConf::LSIRC => return self.LSIRC_get(),
            MCOMultConf::HSEOSC => return self.HSEOSC_get(),
            MCOMultConf::IC5Output => return self.IC5Output_get(),
            MCOMultConf::IC10Output => return self.IC10Output_get(),
            MCOMultConf::SYSAClkSource => return self.SYSAClkSource_get(),
        };
    }
    fn MCODiv_get(&self) -> Result<f32, ClockError> {
        let input = self.MCOMult_get()? as f32;
        let value = self.MCODiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn MCOPin_get(&self) -> Result<f32, ClockError> {
        self.MCODiv_get()
    }
    fn MCO2Mult_get(&self) -> Result<f32, ClockError> {
        match self.MCO2Mult {
            MCO2MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
            MCO2MultConf::LSEOSC => return self.LSEOSC_get(),
            MCO2MultConf::MSIRC => return self.MSIRC_get(),
            MCO2MultConf::LSIRC => return self.LSIRC_get(),
            MCO2MultConf::HSEOSC => return self.HSEOSC_get(),
            MCO2MultConf::IC5Output => return self.IC5Output_get(),
            MCO2MultConf::IC10Output => return self.IC10Output_get(),
            MCO2MultConf::SYSBClkSource => return self.SYSBClkSource_get(),
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
    fn CKPERSource_get(&self) -> Result<f32, ClockError> {
        match self.CKPERSource {
            CKPERSourceConf::HSIRC => return self.HSIRC_get(),
            CKPERSourceConf::MSIRC => return self.MSIRC_get(),
            CKPERSourceConf::HSEOSC => return self.HSEOSC_get(),
            CKPERSourceConf::IC5Output => return self.IC5Output_get(),
            CKPERSourceConf::IC10Output => return self.IC10Output_get(),
            CKPERSourceConf::IC15Output => return self.IC15Output_get(),
            CKPERSourceConf::IC19Output => return self.IC19Output_get(),
            CKPERSourceConf::IC20Output => return self.IC20Output_get(),
        };
    }
    pub fn CKPERoutput_get(&self) -> Result<f32, ClockError> {
        self.CKPERSource_get()
    }
    fn ADCMult_get(&self) -> Result<f32, ClockError> {
        match self.ADCMult {
            ADCMultConf::AHBOutput => return self.AHBOutput_get(),
            ADCMultConf::CKPERoutput => return self.CKPERoutput_get(),
            ADCMultConf::IC7Output => return self.IC7Output_get(),
            ADCMultConf::IC8Output => return self.IC8Output_get(),
            ADCMultConf::MSIRC => return self.MSIRC_get(),
            ADCMultConf::HSIDivOutput => return self.HSIDivOutput_get(),
            ADCMultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            ADCMultConf::TIMGOutput => return self.TIMGOutput_get(),
        };
    }
    fn ADCDIV_get(&self) -> Result<f32, ClockError> {
        let input = self.ADCMult_get()? as f32;
        let value = self.ADCDIV.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn ADCoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.ADCDIV_get()?;
        if input > (133000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133000000),
                from: ClockNodes::ADCDIV,
                to: ClockNodes::ADCoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::ADCDIV,
                to: ClockNodes::ADCoutput,
            });
        }
        Ok(input)
    }
    fn ADFMult_get(&self) -> Result<f32, ClockError> {
        match self.ADFMult {
            ADFMultConf::AHBOutput => return self.AHBOutput_get(),
            ADFMultConf::CKPERoutput => return self.CKPERoutput_get(),
            ADFMultConf::IC7Output => return self.IC7Output_get(),
            ADFMultConf::IC8Output => return self.IC8Output_get(),
            ADFMultConf::MSIRC => return self.MSIRC_get(),
            ADFMultConf::HSIDivOutput => return self.HSIDivOutput_get(),
            ADFMultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            ADFMultConf::TIMGOutput => return self.TIMGOutput_get(),
        };
    }
    pub fn ADFoutput_get(&self) -> Result<f32, ClockError> {
        self.ADFMult_get()
    }
    fn MDF1Mult_get(&self) -> Result<f32, ClockError> {
        match self.MDF1Mult {
            MDF1MultConf::AHBOutput => return self.AHBOutput_get(),
            MDF1MultConf::CKPERoutput => return self.CKPERoutput_get(),
            MDF1MultConf::IC7Output => return self.IC7Output_get(),
            MDF1MultConf::IC8Output => return self.IC8Output_get(),
            MDF1MultConf::MSIRC => return self.MSIRC_get(),
            MDF1MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
            MDF1MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            MDF1MultConf::TIMGOutput => return self.TIMGOutput_get(),
        };
    }
    pub fn MDFoutput_get(&self) -> Result<f32, ClockError> {
        self.MDF1Mult_get()
    }
    fn PSSIMult_get(&self) -> Result<f32, ClockError> {
        match self.PSSIMult {
            PSSIMultConf::AHBOutput => return self.AHBOutput_get(),
            PSSIMultConf::CKPERoutput => return self.CKPERoutput_get(),
            PSSIMultConf::IC20Output => return self.IC20Output_get(),
            PSSIMultConf::HSIDivOutput => return self.HSIDivOutput_get(),
        };
    }
    pub fn PSSIoutput_get(&self) -> Result<f32, ClockError> {
        self.PSSIMult_get()
    }
    fn FDCANMult_get(&self) -> Result<f32, ClockError> {
        match self.FDCANMult {
            FDCANMultConf::APB1Output => return self.APB1Output_get(),
            FDCANMultConf::CKPERoutput => return self.CKPERoutput_get(),
            FDCANMultConf::IC19Output => return self.IC19Output_get(),
            FDCANMultConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    pub fn FDCANoutput_get(&self) -> Result<f32, ClockError> {
        self.FDCANMult_get()
    }
    fn I2C1Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C1Mult {
            I2C1MultConf::APB1Output => return self.APB1Output_get(),
            I2C1MultConf::CKPERoutput => return self.CKPERoutput_get(),
            I2C1MultConf::IC10Output => return self.IC10Output_get(),
            I2C1MultConf::IC15Output => return self.IC15Output_get(),
            I2C1MultConf::MSIRC => return self.MSIRC_get(),
            I2C1MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
        };
    }
    pub fn I2C1output_get(&self) -> Result<f32, ClockError> {
        self.I2C1Mult_get()
    }
    fn I2C2Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C2Mult {
            I2C2MultConf::APB1Output => return self.APB1Output_get(),
            I2C2MultConf::CKPERoutput => return self.CKPERoutput_get(),
            I2C2MultConf::IC10Output => return self.IC10Output_get(),
            I2C2MultConf::IC15Output => return self.IC15Output_get(),
            I2C2MultConf::MSIRC => return self.MSIRC_get(),
            I2C2MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
        };
    }
    pub fn I2C2output_get(&self) -> Result<f32, ClockError> {
        self.I2C2Mult_get()
    }
    fn I2C3Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C3Mult {
            I2C3MultConf::APB1Output => return self.APB1Output_get(),
            I2C3MultConf::CKPERoutput => return self.CKPERoutput_get(),
            I2C3MultConf::IC10Output => return self.IC10Output_get(),
            I2C3MultConf::IC15Output => return self.IC15Output_get(),
            I2C3MultConf::MSIRC => return self.MSIRC_get(),
            I2C3MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
        };
    }
    pub fn I2C3output_get(&self) -> Result<f32, ClockError> {
        self.I2C3Mult_get()
    }
    fn I2C4Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C4Mult {
            I2C4MultConf::APB1Output => return self.APB1Output_get(),
            I2C4MultConf::CKPERoutput => return self.CKPERoutput_get(),
            I2C4MultConf::IC10Output => return self.IC10Output_get(),
            I2C4MultConf::IC15Output => return self.IC15Output_get(),
            I2C4MultConf::MSIRC => return self.MSIRC_get(),
            I2C4MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
        };
    }
    pub fn I2C4output_get(&self) -> Result<f32, ClockError> {
        self.I2C4Mult_get()
    }
    fn I3C1Mult_get(&self) -> Result<f32, ClockError> {
        match self.I3C1Mult {
            I3C1MultConf::APB1Output => return self.APB1Output_get(),
            I3C1MultConf::CKPERoutput => return self.CKPERoutput_get(),
            I3C1MultConf::IC10Output => return self.IC10Output_get(),
            I3C1MultConf::IC15Output => return self.IC15Output_get(),
            I3C1MultConf::MSIRC => return self.MSIRC_get(),
            I3C1MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
        };
    }
    pub fn I3C1output_get(&self) -> Result<f32, ClockError> {
        self.I3C1Mult_get()
    }
    fn I3C2Mult_get(&self) -> Result<f32, ClockError> {
        match self.I3C2Mult {
            I3C2MultConf::APB1Output => return self.APB1Output_get(),
            I3C2MultConf::CKPERoutput => return self.CKPERoutput_get(),
            I3C2MultConf::IC10Output => return self.IC10Output_get(),
            I3C2MultConf::IC15Output => return self.IC15Output_get(),
            I3C2MultConf::MSIRC => return self.MSIRC_get(),
            I3C2MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
        };
    }
    pub fn I3C2output_get(&self) -> Result<f32, ClockError> {
        self.I3C2Mult_get()
    }
    fn LPTIM1Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM1Mult {
            LPTIM1MultConf::APB1Output => return self.APB1Output_get(),
            LPTIM1MultConf::CKPERoutput => return self.CKPERoutput_get(),
            LPTIM1MultConf::IC15Output => return self.IC15Output_get(),
            LPTIM1MultConf::LSEOSC => return self.LSEOSC_get(),
            LPTIM1MultConf::LSIRC => return self.LSIRC_get(),
            LPTIM1MultConf::TIMGOutput => return self.TIMGOutput_get(),
        };
    }
    pub fn LPTIM1output_get(&self) -> Result<f32, ClockError> {
        let input = self.LPTIM1Mult_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
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
    fn LPTIM3Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM3Mult {
            LPTIM3MultConf::APB4Output => return self.APB4Output_get(),
            LPTIM3MultConf::CKPERoutput => return self.CKPERoutput_get(),
            LPTIM3MultConf::IC15Output => return self.IC15Output_get(),
            LPTIM3MultConf::LSEOSC => return self.LSEOSC_get(),
            LPTIM3MultConf::LSIRC => return self.LSIRC_get(),
            LPTIM3MultConf::TIMGOutput => return self.TIMGOutput_get(),
        };
    }
    pub fn LPTIM3output_get(&self) -> Result<f32, ClockError> {
        let input = self.LPTIM3Mult_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
                from: ClockNodes::LPTIM3Mult,
                to: ClockNodes::LPTIM3output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::LPTIM3Mult,
                to: ClockNodes::LPTIM3output,
            });
        }
        Ok(input)
    }
    fn LPTIM2Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM2Mult {
            LPTIM2MultConf::APB4Output => return self.APB4Output_get(),
            LPTIM2MultConf::CKPERoutput => return self.CKPERoutput_get(),
            LPTIM2MultConf::IC15Output => return self.IC15Output_get(),
            LPTIM2MultConf::LSEOSC => return self.LSEOSC_get(),
            LPTIM2MultConf::LSIRC => return self.LSIRC_get(),
            LPTIM2MultConf::TIMGOutput => return self.TIMGOutput_get(),
        };
    }
    pub fn LPTIM2output_get(&self) -> Result<f32, ClockError> {
        let input = self.LPTIM2Mult_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
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
    fn LPTIM4Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM4Mult {
            LPTIM4MultConf::APB4Output => return self.APB4Output_get(),
            LPTIM4MultConf::CKPERoutput => return self.CKPERoutput_get(),
            LPTIM4MultConf::IC15Output => return self.IC15Output_get(),
            LPTIM4MultConf::LSEOSC => return self.LSEOSC_get(),
            LPTIM4MultConf::LSIRC => return self.LSIRC_get(),
            LPTIM4MultConf::TIMGOutput => return self.TIMGOutput_get(),
        };
    }
    pub fn LPTIM4output_get(&self) -> Result<f32, ClockError> {
        let input = self.LPTIM4Mult_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
                from: ClockNodes::LPTIM4Mult,
                to: ClockNodes::LPTIM4output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::LPTIM4Mult,
                to: ClockNodes::LPTIM4output,
            });
        }
        Ok(input)
    }
    fn LPTIM5Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM5Mult {
            LPTIM5MultConf::APB4Output => return self.APB4Output_get(),
            LPTIM5MultConf::CKPERoutput => return self.CKPERoutput_get(),
            LPTIM5MultConf::IC15Output => return self.IC15Output_get(),
            LPTIM5MultConf::LSEOSC => return self.LSEOSC_get(),
            LPTIM5MultConf::LSIRC => return self.LSIRC_get(),
            LPTIM5MultConf::TIMGOutput => return self.TIMGOutput_get(),
        };
    }
    pub fn LPTIM5output_get(&self) -> Result<f32, ClockError> {
        let input = self.LPTIM5Mult_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
                from: ClockNodes::LPTIM5Mult,
                to: ClockNodes::LPTIM5output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::LPTIM5Mult,
                to: ClockNodes::LPTIM5output,
            });
        }
        Ok(input)
    }
    fn LTDCMult_get(&self) -> Result<f32, ClockError> {
        match self.LTDCMult {
            LTDCMultConf::APB5Output => return self.APB5Output_get(),
            LTDCMultConf::CKPERoutput => return self.CKPERoutput_get(),
            LTDCMultConf::IC16Output => return self.IC16Output_get(),
            LTDCMultConf::HSIDivOutput => return self.HSIDivOutput_get(),
        };
    }
    pub fn LTDCoutput_get(&self) -> Result<f32, ClockError> {
        self.LTDCMult_get()
    }
    fn DCMIPPMult_get(&self) -> Result<f32, ClockError> {
        match self.DCMIPPMult {
            DCMIPPMultConf::APB5Output => return self.APB5Output_get(),
            DCMIPPMultConf::CKPERoutput => return self.CKPERoutput_get(),
            DCMIPPMultConf::IC17Output => return self.IC17Output_get(),
            DCMIPPMultConf::HSIDivOutput => return self.HSIDivOutput_get(),
        };
    }
    pub fn DCMIPPoutput_get(&self) -> Result<f32, ClockError> {
        self.DCMIPPMult_get()
    }
    fn FMCMult_get(&self) -> Result<f32, ClockError> {
        match self.FMCMult {
            FMCMultConf::AHBOutput => return self.AHBOutput_get(),
            FMCMultConf::CKPERoutput => return self.CKPERoutput_get(),
            FMCMultConf::IC3Output => return self.IC3Output_get(),
            FMCMultConf::IC4Output => return self.IC4Output_get(),
        };
    }
    pub fn FMCoutput_get(&self) -> Result<f32, ClockError> {
        self.FMCMult_get()
    }
    fn SAI1Mult_get(&self) -> Result<f32, ClockError> {
        match self.SAI1Mult {
            SAI1MultConf::APB2Output => return self.APB2Output_get(),
            SAI1MultConf::CKPERoutput => return self.CKPERoutput_get(),
            SAI1MultConf::IC7Output => return self.IC7Output_get(),
            SAI1MultConf::IC8Output => return self.IC8Output_get(),
            SAI1MultConf::MSIRC => return self.MSIRC_get(),
            SAI1MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
            SAI1MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            SAI1MultConf::SPDIFRX1Mult => return self.SPDIFRX1Mult_get(),
        };
    }
    pub fn SAI1output_get(&self) -> Result<f32, ClockError> {
        let input = self.SAI1Mult_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
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
    fn SAI2Mult_get(&self) -> Result<f32, ClockError> {
        match self.SAI2Mult {
            SAI2MultConf::APB2Output => return self.APB2Output_get(),
            SAI2MultConf::CKPERoutput => return self.CKPERoutput_get(),
            SAI2MultConf::IC7Output => return self.IC7Output_get(),
            SAI2MultConf::IC8Output => return self.IC8Output_get(),
            SAI2MultConf::MSIRC => return self.MSIRC_get(),
            SAI2MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
            SAI2MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            SAI2MultConf::SPDIFRX1Mult => return self.SPDIFRX1Mult_get(),
        };
    }
    pub fn SAI2output_get(&self) -> Result<f32, ClockError> {
        let input = self.SAI2Mult_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
                from: ClockNodes::SAI2Mult,
                to: ClockNodes::SAI2output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SAI2Mult,
                to: ClockNodes::SAI2output,
            });
        }
        Ok(input)
    }
    fn USART1Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART1Mult {
            USART1MultConf::APB2Output => return self.APB2Output_get(),
            USART1MultConf::CKPERoutput => return self.CKPERoutput_get(),
            USART1MultConf::IC9Output => return self.IC9Output_get(),
            USART1MultConf::IC14Output => return self.IC14Output_get(),
            USART1MultConf::LSEOSC => return self.LSEOSC_get(),
            USART1MultConf::MSIRC => return self.MSIRC_get(),
            USART1MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
        };
    }
    pub fn USART1output_get(&self) -> Result<f32, ClockError> {
        let input = self.USART1Mult_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::USART1Mult,
                to: ClockNodes::USART1output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::USART1Mult,
                to: ClockNodes::USART1output,
            });
        }
        Ok(input)
    }
    fn USART2Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART2Mult {
            USART2MultConf::APB1Output => return self.APB1Output_get(),
            USART2MultConf::CKPERoutput => return self.CKPERoutput_get(),
            USART2MultConf::IC9Output => return self.IC9Output_get(),
            USART2MultConf::IC14Output => return self.IC14Output_get(),
            USART2MultConf::LSEOSC => return self.LSEOSC_get(),
            USART2MultConf::MSIRC => return self.MSIRC_get(),
            USART2MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
        };
    }
    pub fn USART2output_get(&self) -> Result<f32, ClockError> {
        let input = self.USART2Mult_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::USART2Mult,
                to: ClockNodes::USART2output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::USART2Mult,
                to: ClockNodes::USART2output,
            });
        }
        Ok(input)
    }
    fn USART3Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART3Mult {
            USART3MultConf::APB1Output => return self.APB1Output_get(),
            USART3MultConf::CKPERoutput => return self.CKPERoutput_get(),
            USART3MultConf::IC9Output => return self.IC9Output_get(),
            USART3MultConf::IC14Output => return self.IC14Output_get(),
            USART3MultConf::LSEOSC => return self.LSEOSC_get(),
            USART3MultConf::MSIRC => return self.MSIRC_get(),
            USART3MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
        };
    }
    pub fn USART3output_get(&self) -> Result<f32, ClockError> {
        let input = self.USART3Mult_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::USART3Mult,
                to: ClockNodes::USART3output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::USART3Mult,
                to: ClockNodes::USART3output,
            });
        }
        Ok(input)
    }
    fn UART4Mult_get(&self) -> Result<f32, ClockError> {
        match self.UART4Mult {
            UART4MultConf::APB1Output => return self.APB1Output_get(),
            UART4MultConf::CKPERoutput => return self.CKPERoutput_get(),
            UART4MultConf::IC9Output => return self.IC9Output_get(),
            UART4MultConf::IC14Output => return self.IC14Output_get(),
            UART4MultConf::LSEOSC => return self.LSEOSC_get(),
            UART4MultConf::MSIRC => return self.MSIRC_get(),
            UART4MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
        };
    }
    pub fn UART4output_get(&self) -> Result<f32, ClockError> {
        let input = self.UART4Mult_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::UART4Mult,
                to: ClockNodes::UART4output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::UART4Mult,
                to: ClockNodes::UART4output,
            });
        }
        Ok(input)
    }
    fn UART5Mult_get(&self) -> Result<f32, ClockError> {
        match self.UART5Mult {
            UART5MultConf::APB1Output => return self.APB1Output_get(),
            UART5MultConf::CKPERoutput => return self.CKPERoutput_get(),
            UART5MultConf::IC9Output => return self.IC9Output_get(),
            UART5MultConf::IC14Output => return self.IC14Output_get(),
            UART5MultConf::LSEOSC => return self.LSEOSC_get(),
            UART5MultConf::MSIRC => return self.MSIRC_get(),
            UART5MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
        };
    }
    pub fn UART5output_get(&self) -> Result<f32, ClockError> {
        let input = self.UART5Mult_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::UART5Mult,
                to: ClockNodes::UART5output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::UART5Mult,
                to: ClockNodes::UART5output,
            });
        }
        Ok(input)
    }
    fn USART6Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART6Mult {
            USART6MultConf::APB2Output => return self.APB2Output_get(),
            USART6MultConf::CKPERoutput => return self.CKPERoutput_get(),
            USART6MultConf::IC9Output => return self.IC9Output_get(),
            USART6MultConf::IC14Output => return self.IC14Output_get(),
            USART6MultConf::LSEOSC => return self.LSEOSC_get(),
            USART6MultConf::MSIRC => return self.MSIRC_get(),
            USART6MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
        };
    }
    pub fn USART6output_get(&self) -> Result<f32, ClockError> {
        let input = self.USART6Mult_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::USART6Mult,
                to: ClockNodes::USART6output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::USART6Mult,
                to: ClockNodes::USART6output,
            });
        }
        Ok(input)
    }
    fn UART7Mult_get(&self) -> Result<f32, ClockError> {
        match self.UART7Mult {
            UART7MultConf::APB1Output => return self.APB1Output_get(),
            UART7MultConf::CKPERoutput => return self.CKPERoutput_get(),
            UART7MultConf::IC9Output => return self.IC9Output_get(),
            UART7MultConf::IC14Output => return self.IC14Output_get(),
            UART7MultConf::LSEOSC => return self.LSEOSC_get(),
            UART7MultConf::MSIRC => return self.MSIRC_get(),
            UART7MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
        };
    }
    pub fn UART7output_get(&self) -> Result<f32, ClockError> {
        let input = self.UART7Mult_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::UART7Mult,
                to: ClockNodes::UART7output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::UART7Mult,
                to: ClockNodes::UART7output,
            });
        }
        Ok(input)
    }
    fn UART8Mult_get(&self) -> Result<f32, ClockError> {
        match self.UART8Mult {
            UART8MultConf::APB1Output => return self.APB1Output_get(),
            UART8MultConf::CKPERoutput => return self.CKPERoutput_get(),
            UART8MultConf::IC9Output => return self.IC9Output_get(),
            UART8MultConf::IC14Output => return self.IC14Output_get(),
            UART8MultConf::LSEOSC => return self.LSEOSC_get(),
            UART8MultConf::MSIRC => return self.MSIRC_get(),
            UART8MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
        };
    }
    pub fn UART8output_get(&self) -> Result<f32, ClockError> {
        let input = self.UART8Mult_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::UART8Mult,
                to: ClockNodes::UART8output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::UART8Mult,
                to: ClockNodes::UART8output,
            });
        }
        Ok(input)
    }
    fn UART9Mult_get(&self) -> Result<f32, ClockError> {
        match self.UART9Mult {
            UART9MultConf::APB2Output => return self.APB2Output_get(),
            UART9MultConf::CKPERoutput => return self.CKPERoutput_get(),
            UART9MultConf::IC9Output => return self.IC9Output_get(),
            UART9MultConf::IC14Output => return self.IC14Output_get(),
            UART9MultConf::LSEOSC => return self.LSEOSC_get(),
            UART9MultConf::MSIRC => return self.MSIRC_get(),
            UART9MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
        };
    }
    pub fn UART9output_get(&self) -> Result<f32, ClockError> {
        let input = self.UART9Mult_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::UART9Mult,
                to: ClockNodes::UART9output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::UART9Mult,
                to: ClockNodes::UART9output,
            });
        }
        Ok(input)
    }
    fn LPUART1Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPUART1Mult {
            LPUART1MultConf::APB4Output => return self.APB4Output_get(),
            LPUART1MultConf::CKPERoutput => return self.CKPERoutput_get(),
            LPUART1MultConf::IC9Output => return self.IC9Output_get(),
            LPUART1MultConf::IC14Output => return self.IC14Output_get(),
            LPUART1MultConf::LSEOSC => return self.LSEOSC_get(),
            LPUART1MultConf::MSIRC => return self.MSIRC_get(),
            LPUART1MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
        };
    }
    pub fn LPUART1output_get(&self) -> Result<f32, ClockError> {
        let input = self.LPUART1Mult_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
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
    fn USART10Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART10Mult {
            USART10MultConf::APB2Output => return self.APB2Output_get(),
            USART10MultConf::CKPERoutput => return self.CKPERoutput_get(),
            USART10MultConf::IC9Output => return self.IC9Output_get(),
            USART10MultConf::IC14Output => return self.IC14Output_get(),
            USART10MultConf::LSEOSC => return self.LSEOSC_get(),
            USART10MultConf::MSIRC => return self.MSIRC_get(),
            USART10MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
        };
    }
    pub fn USART10output_get(&self) -> Result<f32, ClockError> {
        let input = self.USART10Mult_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::USART10Mult,
                to: ClockNodes::USART10output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::USART10Mult,
                to: ClockNodes::USART10output,
            });
        }
        Ok(input)
    }
    fn SPI1Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI1Mult {
            SPI1MultConf::APB2Output => return self.APB2Output_get(),
            SPI1MultConf::CKPERoutput => return self.CKPERoutput_get(),
            SPI1MultConf::IC8Output => return self.IC8Output_get(),
            SPI1MultConf::IC9Output => return self.IC9Output_get(),
            SPI1MultConf::MSIRC => return self.MSIRC_get(),
            SPI1MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
            SPI1MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    pub fn SPI1output_get(&self) -> Result<f32, ClockError> {
        let input = self.SPI1Mult_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
                from: ClockNodes::SPI1Mult,
                to: ClockNodes::SPI1output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SPI1Mult,
                to: ClockNodes::SPI1output,
            });
        }
        Ok(input)
    }
    fn SPI2Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI2Mult {
            SPI2MultConf::APB1Output => return self.APB1Output_get(),
            SPI2MultConf::CKPERoutput => return self.CKPERoutput_get(),
            SPI2MultConf::IC8Output => return self.IC8Output_get(),
            SPI2MultConf::IC9Output => return self.IC9Output_get(),
            SPI2MultConf::MSIRC => return self.MSIRC_get(),
            SPI2MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
            SPI2MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    pub fn SPI2output_get(&self) -> Result<f32, ClockError> {
        let input = self.SPI2Mult_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
                from: ClockNodes::SPI2Mult,
                to: ClockNodes::SPI2output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SPI2Mult,
                to: ClockNodes::SPI2output,
            });
        }
        Ok(input)
    }
    fn SPI3Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI3Mult {
            SPI3MultConf::APB1Output => return self.APB1Output_get(),
            SPI3MultConf::CKPERoutput => return self.CKPERoutput_get(),
            SPI3MultConf::IC8Output => return self.IC8Output_get(),
            SPI3MultConf::IC9Output => return self.IC9Output_get(),
            SPI3MultConf::MSIRC => return self.MSIRC_get(),
            SPI3MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
            SPI3MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    pub fn SPI3output_get(&self) -> Result<f32, ClockError> {
        let input = self.SPI3Mult_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
                from: ClockNodes::SPI3Mult,
                to: ClockNodes::SPI3output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SPI3Mult,
                to: ClockNodes::SPI3output,
            });
        }
        Ok(input)
    }
    fn SPI4Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI4Mult {
            SPI4MultConf::APB2Output => return self.APB2Output_get(),
            SPI4MultConf::CKPERoutput => return self.CKPERoutput_get(),
            SPI4MultConf::IC9Output => return self.IC9Output_get(),
            SPI4MultConf::IC14Output => return self.IC14Output_get(),
            SPI4MultConf::MSIRC => return self.MSIRC_get(),
            SPI4MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
            SPI4MultConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    pub fn SPI4output_get(&self) -> Result<f32, ClockError> {
        let input = self.SPI4Mult_get()?;
        if input > (133000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133000000),
                from: ClockNodes::SPI4Mult,
                to: ClockNodes::SPI4output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SPI4Mult,
                to: ClockNodes::SPI4output,
            });
        }
        Ok(input)
    }
    fn SPI5Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI5Mult {
            SPI5MultConf::APB2Output => return self.APB2Output_get(),
            SPI5MultConf::CKPERoutput => return self.CKPERoutput_get(),
            SPI5MultConf::IC9Output => return self.IC9Output_get(),
            SPI5MultConf::IC14Output => return self.IC14Output_get(),
            SPI5MultConf::MSIRC => return self.MSIRC_get(),
            SPI5MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
            SPI5MultConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    pub fn SPI5output_get(&self) -> Result<f32, ClockError> {
        let input = self.SPI5Mult_get()?;
        if input > (133000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133000000),
                from: ClockNodes::SPI5Mult,
                to: ClockNodes::SPI5output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SPI5Mult,
                to: ClockNodes::SPI5output,
            });
        }
        Ok(input)
    }
    fn SPI6Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI6Mult {
            SPI6MultConf::APB4Output => return self.APB4Output_get(),
            SPI6MultConf::CKPERoutput => return self.CKPERoutput_get(),
            SPI6MultConf::IC8Output => return self.IC8Output_get(),
            SPI6MultConf::IC9Output => return self.IC9Output_get(),
            SPI6MultConf::MSIRC => return self.MSIRC_get(),
            SPI6MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
            SPI6MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    pub fn SPI6output_get(&self) -> Result<f32, ClockError> {
        let input = self.SPI6Mult_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
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
    fn XSPI1Mult_get(&self) -> Result<f32, ClockError> {
        match self.XSPI1Mult {
            XSPI1MultConf::AHBOutput => return self.AHBOutput_get(),
            XSPI1MultConf::CKPERoutput => return self.CKPERoutput_get(),
            XSPI1MultConf::IC3Output => return self.IC3Output_get(),
            XSPI1MultConf::IC4Output => return self.IC4Output_get(),
        };
    }
    pub fn XSPI1output_get(&self) -> Result<f32, ClockError> {
        self.XSPI1Mult_get()
    }
    fn XSPI2Mult_get(&self) -> Result<f32, ClockError> {
        match self.XSPI2Mult {
            XSPI2MultConf::AHBOutput => return self.AHBOutput_get(),
            XSPI2MultConf::CKPERoutput => return self.CKPERoutput_get(),
            XSPI2MultConf::IC3Output => return self.IC3Output_get(),
            XSPI2MultConf::IC4Output => return self.IC4Output_get(),
        };
    }
    pub fn XSPI2output_get(&self) -> Result<f32, ClockError> {
        self.XSPI2Mult_get()
    }
    fn OTGHS1Mult_get(&self) -> Result<f32, ClockError> {
        match self.OTGHS1Mult {
            OTGHS1MultConf::OTGPHY1output => return self.OTGPHY1output_get(),
            OTGHS1MultConf::HSEOSCDIV => return self.HSEOSCDIV_get(),
        };
    }
    pub fn OTGHS1output_get(&self) -> Result<f32, ClockError> {
        let input = self.OTGHS1Mult_get()?;
        if input > (60000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 60000000),
                from: ClockNodes::OTGHS1Mult,
                to: ClockNodes::OTGHS1output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::OTGHS1Mult,
                to: ClockNodes::OTGHS1output,
            });
        }
        Ok(input)
    }
    fn OTGHS2Mult_get(&self) -> Result<f32, ClockError> {
        match self.OTGHS2Mult {
            OTGHS2MultConf::OTGPHY2output => return self.OTGPHY2output_get(),
            OTGHS2MultConf::HSEOSCDIV => return self.HSEOSCDIV_get(),
        };
    }
    pub fn OTGHS2output_get(&self) -> Result<f32, ClockError> {
        let input = self.OTGHS2Mult_get()?;
        if input > (60000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 60000000),
                from: ClockNodes::OTGHS2Mult,
                to: ClockNodes::OTGHS2output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::OTGHS2Mult,
                to: ClockNodes::OTGHS2output,
            });
        }
        Ok(input)
    }
    fn XSPI3Mult_get(&self) -> Result<f32, ClockError> {
        match self.XSPI3Mult {
            XSPI3MultConf::AHBOutput => return self.AHBOutput_get(),
            XSPI3MultConf::CKPERoutput => return self.CKPERoutput_get(),
            XSPI3MultConf::IC3Output => return self.IC3Output_get(),
            XSPI3MultConf::IC4Output => return self.IC4Output_get(),
        };
    }
    pub fn XSPI3output_get(&self) -> Result<f32, ClockError> {
        self.XSPI3Mult_get()
    }
    fn OTGPHY1Mult_get(&self) -> Result<f32, ClockError> {
        match self.OTGPHY1Mult {
            OTGPHY1MultConf::HSEOSCDIV => return self.HSEOSCDIV_get(),
            OTGPHY1MultConf::CKPERoutput => return self.CKPERoutput_get(),
            OTGPHY1MultConf::IC15Output => return self.IC15Output_get(),
            OTGPHY1MultConf::HSEDIV => return self.HSEDIV_get(),
        };
    }
    pub fn OTGPHY1output_get(&self) -> Result<f32, ClockError> {
        self.OTGPHY1Mult_get()
    }
    fn OTGPHY2Mult_get(&self) -> Result<f32, ClockError> {
        match self.OTGPHY2Mult {
            OTGPHY2MultConf::HSEOSCDIV => return self.HSEOSCDIV_get(),
            OTGPHY2MultConf::CKPERoutput => return self.CKPERoutput_get(),
            OTGPHY2MultConf::IC15Output => return self.IC15Output_get(),
            OTGPHY2MultConf::HSEDIV => return self.HSEDIV_get(),
        };
    }
    pub fn OTGPHY2output_get(&self) -> Result<f32, ClockError> {
        self.OTGPHY2Mult_get()
    }
    fn SDMMC1Mult_get(&self) -> Result<f32, ClockError> {
        match self.SDMMC1Mult {
            SDMMC1MultConf::AHBOutput => return self.AHBOutput_get(),
            SDMMC1MultConf::CKPERoutput => return self.CKPERoutput_get(),
            SDMMC1MultConf::IC4Output => return self.IC4Output_get(),
            SDMMC1MultConf::IC5Output => return self.IC5Output_get(),
        };
    }
    pub fn SDMMC1output_get(&self) -> Result<f32, ClockError> {
        let input = self.SDMMC1Mult_get()?;
        if input > (208000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 208000000),
                from: ClockNodes::SDMMC1Mult,
                to: ClockNodes::SDMMC1output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SDMMC1Mult,
                to: ClockNodes::SDMMC1output,
            });
        }
        Ok(input)
    }
    fn SDMMC2Mult_get(&self) -> Result<f32, ClockError> {
        match self.SDMMC2Mult {
            SDMMC2MultConf::AHBOutput => return self.AHBOutput_get(),
            SDMMC2MultConf::CKPERoutput => return self.CKPERoutput_get(),
            SDMMC2MultConf::IC4Output => return self.IC4Output_get(),
            SDMMC2MultConf::IC5Output => return self.IC5Output_get(),
        };
    }
    pub fn SDMMC2output_get(&self) -> Result<f32, ClockError> {
        let input = self.SDMMC2Mult_get()?;
        if input > (208000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 208000000),
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
    fn ETH1Mult_get(&self) -> Result<f32, ClockError> {
        match self.ETH1Mult {
            ETH1MultConf::AHBOutput => return self.AHBOutput_get(),
            ETH1MultConf::CKPERoutput => return self.CKPERoutput_get(),
            ETH1MultConf::IC12Output => return self.IC12Output_get(),
            ETH1MultConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    pub fn ETH1output_get(&self) -> Result<f32, ClockError> {
        self.ETH1Mult_get()
    }
    fn SPDIFRX1Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPDIFRX1Mult {
            SPDIFRX1MultConf::APB1Output => return self.APB1Output_get(),
            SPDIFRX1MultConf::CKPERoutput => return self.CKPERoutput_get(),
            SPDIFRX1MultConf::IC7Output => return self.IC7Output_get(),
            SPDIFRX1MultConf::IC8Output => return self.IC8Output_get(),
            SPDIFRX1MultConf::MSIRC => return self.MSIRC_get(),
            SPDIFRX1MultConf::HSIDivOutput => return self.HSIDivOutput_get(),
            SPDIFRX1MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    pub fn SPDIFRX1output_get(&self) -> Result<f32, ClockError> {
        let input = self.SPDIFRX1Mult_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
                from: ClockNodes::SPDIFRX1Mult,
                to: ClockNodes::SPDIFRX1output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SPDIFRX1Mult,
                to: ClockNodes::SPDIFRX1output,
            });
        }
        Ok(input)
    }
    fn SYSBClkSource_get(&self) -> Result<f32, ClockError> {
        match self.SYSBClkSource {
            SYSBClkSourceConf::HSIRC => return self.HSIRC_get(),
            SYSBClkSourceConf::MSIRC => return self.MSIRC_get(),
            SYSBClkSourceConf::HSEOSC => return self.HSEOSC_get(),
            SYSBClkSourceConf::IC2Output => return self.IC2Output_get(),
        };
    }
    fn SYSCClkSource_get(&self) -> Result<f32, ClockError> {
        match self.SYSCClkSource {
            SYSCClkSourceConf::HSIRC => return self.HSIRC_get(),
            SYSCClkSourceConf::MSIRC => return self.MSIRC_get(),
            SYSCClkSourceConf::HSEOSC => return self.HSEOSC_get(),
            SYSCClkSourceConf::IC6Output => return self.IC6Output_get(),
        };
    }
    fn SYSDClkSource_get(&self) -> Result<f32, ClockError> {
        match self.SYSDClkSource {
            SYSDClkSourceConf::HSIRC => return self.HSIRC_get(),
            SYSDClkSourceConf::MSIRC => return self.MSIRC_get(),
            SYSDClkSourceConf::HSEOSC => return self.HSEOSC_get(),
            SYSDClkSourceConf::IC11Output => return self.IC11Output_get(),
        };
    }
    pub fn SYSBCLKOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SYSBClkSource_get()?;
        if input > (800000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 800000000),
                from: ClockNodes::SYSBClkSource,
                to: ClockNodes::SYSBCLKOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SYSBClkSource,
                to: ClockNodes::SYSBCLKOutput,
            });
        }
        Ok(input)
    }
    pub fn SYSCCLKOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SYSCClkSource_get()?;
        if input > (1000000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1000000000),
                from: ClockNodes::SYSCClkSource,
                to: ClockNodes::SYSCCLKOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SYSCClkSource,
                to: ClockNodes::SYSCCLKOutput,
            });
        }
        Ok(input)
    }
    pub fn SYSDCLKOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SYSDClkSource_get()?;
        if input > (800000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 800000000),
                from: ClockNodes::SYSDClkSource,
                to: ClockNodes::SYSDCLKOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SYSDClkSource,
                to: ClockNodes::SYSDCLKOutput,
            });
        }
        Ok(input)
    }
    fn SYSAClkSource_get(&self) -> Result<f32, ClockError> {
        match self.SYSAClkSource {
            SYSAClkSourceConf::HSIRC => return self.HSIRC_get(),
            SYSAClkSourceConf::MSIRC => return self.MSIRC_get(),
            SYSAClkSourceConf::HSEOSC => return self.HSEOSC_get(),
            SYSAClkSourceConf::IC1Output => return self.IC1Output_get(),
        };
    }
    fn TPIUPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.SYSAClkSource_get()? as f32;
        let value = self.TPIUPrescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn TPIUOutput_get(&self) -> Result<f32, ClockError> {
        self.TPIUPrescaler_get()
    }
    fn CortexPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.SYSAClkSource_get()? as f32;
        let value = self.CortexPrescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn CortexSysOutput_get(&self) -> Result<f32, ClockError> {
        self.CortexPrescaler_get()
    }
    pub fn CpuClockOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SYSAClkSource_get()?;
        if input > (800000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 800000000),
                from: ClockNodes::SYSAClkSource,
                to: ClockNodes::CpuClockOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SYSAClkSource,
                to: ClockNodes::CpuClockOutput,
            });
        }
        Ok(input)
    }
    pub fn AXIClockOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SYSBClkSource_get()?;
        if input > (800000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 800000000),
                from: ClockNodes::SYSBClkSource,
                to: ClockNodes::AXIClockOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SYSBClkSource,
                to: ClockNodes::AXIClockOutput,
            });
        }
        Ok(input)
    }
    fn HPREDiv_get(&self) -> Result<f32, ClockError> {
        let input = self.SYSBCLKOutput_get()? as f32;
        let value = self.HPREDiv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn APB4DIV_get(&self) -> Result<f32, ClockError> {
        let input = self.HPREDiv_get()? as f32;
        let value = self.APB4DIV.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB4Output_get(&self) -> Result<f32, ClockError> {
        let input = self.APB4DIV_get()?;
        if input > (400000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 400000000),
                from: ClockNodes::APB4DIV,
                to: ClockNodes::APB4Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::APB4DIV,
                to: ClockNodes::APB4Output,
            });
        }
        Ok(input)
    }
    fn APB5DIV_get(&self) -> Result<f32, ClockError> {
        let input = self.HPREDiv_get()? as f32;
        let value = self.APB5DIV.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB5Output_get(&self) -> Result<f32, ClockError> {
        let input = self.APB5DIV_get()?;
        if input > (400000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 400000000),
                from: ClockNodes::APB5DIV,
                to: ClockNodes::APB5Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::APB5DIV,
                to: ClockNodes::APB5Output,
            });
        }
        Ok(input)
    }
    fn TIMGDIV_get(&self) -> Result<f32, ClockError> {
        let input = self.SYSBCLKOutput_get()? as f32;
        let value = self.TIMGDIV.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn TIMGOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.TIMGDIV_get()?;
        if input > (400000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 400000000),
                from: ClockNodes::TIMGDIV,
                to: ClockNodes::TIMGOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::TIMGDIV,
                to: ClockNodes::TIMGOutput,
            });
        }
        Ok(input)
    }
    fn APB1DIV_get(&self) -> Result<f32, ClockError> {
        let input = self.HPREDiv_get()? as f32;
        let value = self.APB1DIV.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn AHBOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.HPREDiv_get()?;
        if input > (400000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 400000000),
                from: ClockNodes::HPREDiv,
                to: ClockNodes::AHBOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::HPREDiv,
                to: ClockNodes::AHBOutput,
            });
        }
        Ok(input)
    }
    pub fn APB1Output_get(&self) -> Result<f32, ClockError> {
        let input = self.APB1DIV_get()?;
        if input > (400000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 400000000),
                from: ClockNodes::APB1DIV,
                to: ClockNodes::APB1Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::APB1DIV,
                to: ClockNodes::APB1Output,
            });
        }
        Ok(input)
    }
    fn APB2DIV_get(&self) -> Result<f32, ClockError> {
        let input = self.HPREDiv_get()? as f32;
        let value = self.APB2DIV.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB2Output_get(&self) -> Result<f32, ClockError> {
        let input = self.APB2DIV_get()?;
        if input > (400000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 400000000),
                from: ClockNodes::APB2DIV,
                to: ClockNodes::APB2Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::APB2DIV,
                to: ClockNodes::APB2Output,
            });
        }
        Ok(input)
    }
    fn PLL1Source_get(&self) -> Result<f32, ClockError> {
        match self.PLL1Source {
            PLL1SourceConf::HSIRC => return self.HSIRC_get(),
            PLL1SourceConf::MSIRC => return self.MSIRC_get(),
            PLL1SourceConf::HSEOSC => return self.HSEOSC_get(),
            PLL1SourceConf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    fn FREFDIV1_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL1Source_get()? as f32;
        let value = self.FREFDIV1.get()? as f32;
        Ok((input / value) as f32)
    }

    fn PLL2Source_get(&self) -> Result<f32, ClockError> {
        match self.PLL2Source {
            PLL2SourceConf::HSIRC => return self.HSIRC_get(),
            PLL2SourceConf::MSIRC => return self.MSIRC_get(),
            PLL2SourceConf::HSEOSC => return self.HSEOSC_get(),
            PLL2SourceConf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    fn FREFDIV2_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL2Source_get()? as f32;
        let value = self.FREFDIV2.get()? as f32;
        Ok((input / value) as f32)
    }

    fn PLL3Source_get(&self) -> Result<f32, ClockError> {
        match self.PLL3Source {
            PLL3SourceConf::HSIRC => return self.HSIRC_get(),
            PLL3SourceConf::MSIRC => return self.MSIRC_get(),
            PLL3SourceConf::HSEOSC => return self.HSEOSC_get(),
            PLL3SourceConf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    fn FREFDIV3_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL3Source_get()? as f32;
        let value = self.FREFDIV3.get()? as f32;
        Ok((input / value) as f32)
    }

    fn PLL4Source_get(&self) -> Result<f32, ClockError> {
        match self.PLL4Source {
            PLL4SourceConf::HSIRC => return self.HSIRC_get(),
            PLL4SourceConf::MSIRC => return self.MSIRC_get(),
            PLL4SourceConf::HSEOSC => return self.HSEOSC_get(),
            PLL4SourceConf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    fn FREFDIV4_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL4Source_get()? as f32;
        let value = self.FREFDIV4.get()? as f32;
        Ok((input / value) as f32)
    }

    fn FBDIV1_get(&self) -> Result<f32, ClockError> {
        let input = self.FREFDIV1_get()? as f32;
        let value = self.FBDIV1.get()? as f32;
        Ok((input * value) as f32)
    }

    pub fn PLL1FRACV_get(&self) -> Result<f32, ClockError> {
        self.PLL1FRACV.get()
    }
    fn POSTDIV1_1_get(&self) -> Result<f32, ClockError> {
        let input = self.FBDIV1_get()? as f32;
        let value = self.POSTDIV1_1.get()? as f32;
        Ok((input / value) as f32)
    }

    fn POSTDIV2_1_get(&self) -> Result<f32, ClockError> {
        let input = self.POSTDIV1_1_get()? as f32;
        let value = self.POSTDIV2_1.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn FOUTPOSTDIV1_get(&self) -> Result<f32, ClockError> {
        let input = self.POSTDIV2_1_get()?;
        if input > (3200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 3200000000),
                from: ClockNodes::POSTDIV2_1,
                to: ClockNodes::FOUTPOSTDIV1,
            });
        } else if input < (16000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 16000000),
                from: ClockNodes::POSTDIV2_1,
                to: ClockNodes::FOUTPOSTDIV1,
            });
        }
        Ok(input)
    }
    fn FBDIV2_get(&self) -> Result<f32, ClockError> {
        let input = self.FREFDIV2_get()? as f32;
        let frac = self.PLL2FRACV_get()? as f32;
        let frac_max = PLL2FRACVConf::max() as f32;
        let value = self.FBDIV2.get()? as f32;
        let ret = (input * (value + (frac / frac_max)));
        Ok(ret as f32)
    }

    pub fn PLL2FRACV_get(&self) -> Result<f32, ClockError> {
        self.PLL2FRACV.get()
    }
    fn POSTDIV1_2_get(&self) -> Result<f32, ClockError> {
        let input = self.FBDIV2_get()? as f32;
        let value = self.POSTDIV1_2.get()? as f32;
        Ok((input / value) as f32)
    }

    fn POSTDIV2_2_get(&self) -> Result<f32, ClockError> {
        let input = self.POSTDIV1_2_get()? as f32;
        let value = self.POSTDIV2_2.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn FOUTPOSTDIV2_get(&self) -> Result<f32, ClockError> {
        let input = self.POSTDIV2_2_get()?;
        if input > (3200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 3200000000),
                from: ClockNodes::POSTDIV2_2,
                to: ClockNodes::FOUTPOSTDIV2,
            });
        } else if input < (16000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 16000000),
                from: ClockNodes::POSTDIV2_2,
                to: ClockNodes::FOUTPOSTDIV2,
            });
        }
        Ok(input)
    }
    fn FBDIV3_get(&self) -> Result<f32, ClockError> {
        let input = self.FREFDIV3_get()? as f32;
        let frac = self.PLL3FRACV_get()? as f32;
        let frac_max = PLL3FRACVConf::max() as f32;
        let value = self.FBDIV3.get()? as f32;
        let ret = (input * (value + (frac / frac_max)));
        Ok(ret as f32)
    }

    pub fn PLL3FRACV_get(&self) -> Result<f32, ClockError> {
        self.PLL3FRACV.get()
    }
    fn POSTDIV1_3_get(&self) -> Result<f32, ClockError> {
        let input = self.FBDIV3_get()? as f32;
        let value = self.POSTDIV1_3.get()? as f32;
        Ok((input / value) as f32)
    }

    fn POSTDIV2_3_get(&self) -> Result<f32, ClockError> {
        let input = self.POSTDIV1_3_get()? as f32;
        let value = self.POSTDIV2_3.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn FOUTPOSTDIV3_get(&self) -> Result<f32, ClockError> {
        let input = self.POSTDIV2_3_get()?;
        if input > (3200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 3200000000),
                from: ClockNodes::POSTDIV2_3,
                to: ClockNodes::FOUTPOSTDIV3,
            });
        } else if input < (16000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 16000000),
                from: ClockNodes::POSTDIV2_3,
                to: ClockNodes::FOUTPOSTDIV3,
            });
        }
        Ok(input)
    }
    fn FBDIV4_get(&self) -> Result<f32, ClockError> {
        let input = self.FREFDIV4_get()? as f32;
        let frac = self.PLL4FRACV_get()? as f32;
        let frac_max = PLL4FRACVConf::max() as f32;
        let value = self.FBDIV4.get()? as f32;
        let ret = (input * (value + (frac / frac_max)));
        Ok(ret as f32)
    }

    pub fn PLL4FRACV_get(&self) -> Result<f32, ClockError> {
        self.PLL4FRACV.get()
    }
    fn POSTDIV1_4_get(&self) -> Result<f32, ClockError> {
        let input = self.FBDIV4_get()? as f32;
        let value = self.POSTDIV1_4.get()? as f32;
        Ok((input / value) as f32)
    }

    fn POSTDIV2_4_get(&self) -> Result<f32, ClockError> {
        let input = self.POSTDIV1_4_get()? as f32;
        let value = self.POSTDIV2_4.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn FOUTPOSTDIV4_get(&self) -> Result<f32, ClockError> {
        let input = self.POSTDIV2_4_get()?;
        if input > (3200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 3200000000),
                from: ClockNodes::POSTDIV2_4,
                to: ClockNodes::FOUTPOSTDIV4,
            });
        } else if input < (16000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 16000000),
                from: ClockNodes::POSTDIV2_4,
                to: ClockNodes::FOUTPOSTDIV4,
            });
        }
        Ok(input)
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
        let input = self.RTCClkSource_get()?;
        if input > (4000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 4000000),
                from: ClockNodes::RTCClkSource,
                to: ClockNodes::RTCOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::RTCClkSource,
                to: ClockNodes::RTCOutput,
            });
        }
        Ok(input)
    }
    pub fn IWDGOutput_get(&self) -> Result<f32, ClockError> {
        self.LSIRC_get()
    }
}
