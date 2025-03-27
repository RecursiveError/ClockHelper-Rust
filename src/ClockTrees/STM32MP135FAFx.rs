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
    I2S_CKIN,
    SysClkSource,
    SysCLKOutput,
    MPUMult,
    MPUCLKOutput,
    CKPERMult,
    CKPERCLKOutput,
    AXIMult,
    AXICLKOutput,
    DACCLKOutput,
    AXIDIV,
    AXIOutput,
    Hclk5Output,
    Hclk6Output,
    APB4DIV,
    APB4DIVOutput,
    APB5DIV,
    APB5DIVOutput,
    APB6DIV,
    Tim3Mul,
    Tim3Output,
    APB6DIVOutput,
    MCO1Mult,
    MCO1Div,
    MCO1Pin,
    MCO2Mult,
    MCO2Div,
    MCO2Pin,
    MLAHBDIV,
    MLAHBClockOutput,
    APB3DIV,
    APB3Output,
    APB1DIV,
    Tim1Mul,
    Tim1Output,
    AHBOutput,
    APB1Output,
    APB2DIV,
    Tim2Mul,
    Tim2Output,
    APB2Output,
    DFSDM1Output,
    PLL12Source,
    DIVM1,
    DIVM2,
    PLL3Source,
    DIVM3,
    PLL4Source,
    DIVM4,
    MPUDIV,
    FreqCk1,
    DIVN1,
    PLL1FRACV,
    DIVN1Mul2Div2,
    DIVP1,
    DIVQ1,
    DIVQ1output,
    DIVR1,
    DIVR1output,
    FreqCk2,
    DIVN2,
    PLL2FRACV,
    DIVN2Mul2Div2,
    DIVP2,
    DIVQ2,
    DIVQ2output,
    DIVR2,
    DIVR2output,
    DIVN3,
    PLL3FRACV,
    DIVP3,
    DIVQ3,
    DIVQ3output,
    DIVR3,
    LTDCOutput,
    DIVR3output,
    DIVN4,
    PLL4FRACV,
    DIVP4,
    DIVP4output,
    DIVQ4,
    DIVQ4output,
    DIVR4,
    DIVR4output,
    HSERTCDevisor,
    RTCClkSource,
    RTCOutput,
    IWDGOutput,
    I2C12Mult,
    I2C12output,
    I2C3Mult,
    I2C3output,
    I2C4Mult,
    I2C4output,
    I2C5Mult,
    I2C5output,
    SPDIFMult,
    SPDIFoutput,
    QSPIMult,
    QSPIoutput,
    FMCMult,
    FMCoutput,
    SDMMC1Mult,
    SDMMC1output,
    SDMMC2Mult,
    SDMMC2output,
    STGENMult,
    STGENoutput,
    LPTIM45Mult,
    LPTIM45output,
    LPTIM2Mult,
    LPTIM2output,
    LPTIM1Mult,
    LPTIM1output,
    USART1Mult,
    USART1output,
    USART2Mult,
    USART2output,
    USART35Mult,
    USART35output,
    USART6Mult,
    USART6output,
    UART78Mult,
    USART78output,
    RNG1Mult,
    RNG1output,
    DCMIMult,
    DCMIoutput,
    SAESMult,
    SAESoutput,
    LPTIM3Mult,
    LPTIM3output,
    SPI4Mult,
    SPI4output,
    SAI2Mult,
    SAI2output,
    USART4Mult,
    USART4output,
    SPI1Mult,
    SPI1output,
    SPI23Mult,
    SPI23output,
    SAI1Mult,
    SAI1output,
    DFSDF1Audiooutput,
    SPI5Mult,
    SPI5output,
    FDCANMult,
    FDCANoutput,
    ETH1Mult,
    ETH1output,
    ETH2Mult,
    ETH2output,
    ADC2Mult,
    ADC2output,
    ADC1Mult,
    ADC1output,
    DDRPHYC,
    PUBL,
    DDRC,
    DDRPERFM,
    HSEUSBPHYDevisor,
    USBPHYCLKMux,
    USBPHYCLKOutput,
    USBPHYRC,
    USBOCLKMux,
    USBOFSCLKOutput,
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
    DIVP3,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MPUMultConf {
    DIVP1,
    MPUDIV,
    HSEOSC,
    HSIDiv,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CKPERMultConf {
    HSIDiv,
    CSIRC,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum AXIMultConf {
    HSEOSC,
    HSIDiv,
    DIVP2,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum AXIDIVConf {
    DIV1,
    DIV2,
    DIV3,
    DIV4,
}

impl AXIDIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            AXIDIVConf::DIV1 => return Ok(1.0),
            AXIDIVConf::DIV2 => return Ok(2.0),
            AXIDIVConf::DIV3 => return Ok(3.0),
            AXIDIVConf::DIV4 => return Ok(4.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum APB4DIVConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl APB4DIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            APB4DIVConf::DIV1 => return Ok(1.0),
            APB4DIVConf::DIV2 => return Ok(2.0),
            APB4DIVConf::DIV4 => return Ok(4.0),
            APB4DIVConf::DIV8 => return Ok(8.0),
            APB4DIVConf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum APB5DIVConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl APB5DIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            APB5DIVConf::DIV1 => return Ok(1.0),
            APB5DIVConf::DIV2 => return Ok(2.0),
            APB5DIVConf::DIV4 => return Ok(4.0),
            APB5DIVConf::DIV8 => return Ok(8.0),
            APB5DIVConf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum APB6DIVConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl APB6DIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            APB6DIVConf::DIV1 => return Ok(1.0),
            APB6DIVConf::DIV2 => return Ok(2.0),
            APB6DIVConf::DIV4 => return Ok(4.0),
            APB6DIVConf::DIV8 => return Ok(8.0),
            APB6DIVConf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCO1MultConf {
    HSIDiv,
    HSEOSC,
    CSIRC,
    LSIRC,
    LSEOSC,
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
    DIV16,
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
            MCO1DivConf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCO2MultConf {
    MPUCLKOutput,
    AXIDIV,
    SysCLKOutput,
    DIVP4,
    HSEOSC,
    HSIDiv,
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
pub enum MLAHBDIVConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
    DIV32,
    DIV64,
    DIV128,
    DIV256,
    DIV512,
}

impl MLAHBDIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MLAHBDIVConf::DIV1 => return Ok(1.0),
            MLAHBDIVConf::DIV2 => return Ok(2.0),
            MLAHBDIVConf::DIV4 => return Ok(4.0),
            MLAHBDIVConf::DIV8 => return Ok(8.0),
            MLAHBDIVConf::DIV16 => return Ok(16.0),
            MLAHBDIVConf::DIV32 => return Ok(32.0),
            MLAHBDIVConf::DIV64 => return Ok(64.0),
            MLAHBDIVConf::DIV128 => return Ok(128.0),
            MLAHBDIVConf::DIV256 => return Ok(256.0),
            MLAHBDIVConf::DIV512 => return Ok(512.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum APB3DIVConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl APB3DIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            APB3DIVConf::DIV1 => return Ok(1.0),
            APB3DIVConf::DIV2 => return Ok(2.0),
            APB3DIVConf::DIV4 => return Ok(4.0),
            APB3DIVConf::DIV8 => return Ok(8.0),
            APB3DIVConf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum APB1DIVConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl APB1DIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            APB1DIVConf::DIV1 => return Ok(1.0),
            APB1DIVConf::DIV2 => return Ok(2.0),
            APB1DIVConf::DIV4 => return Ok(4.0),
            APB1DIVConf::DIV8 => return Ok(8.0),
            APB1DIVConf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum APB2DIVConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl APB2DIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            APB2DIVConf::DIV1 => return Ok(1.0),
            APB2DIVConf::DIV2 => return Ok(2.0),
            APB2DIVConf::DIV4 => return Ok(4.0),
            APB2DIVConf::DIV8 => return Ok(8.0),
            APB2DIVConf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL12SourceConf {
    HSIDiv,
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
        64
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
        64
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
pub enum PLL3SourceConf {
    HSIDiv,
    CSIRC,
    HSEOSC,
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
        64
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
pub enum PLL4SourceConf {
    HSIDiv,
    CSIRC,
    HSEOSC,
    I2S_CKIN,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DIVM4Conf {
    Value(u32),
}

impl DIVM4Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVM4Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVM4,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVM4,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MPUDIVConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl MPUDIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MPUDIVConf::DIV1 => return Ok(1.0),
            MPUDIVConf::DIV2 => return Ok(2.0),
            MPUDIVConf::DIV4 => return Ok(4.0),
            MPUDIVConf::DIV8 => return Ok(8.0),
            MPUDIVConf::DIV16 => return Ok(16.0),
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
        25
    }

    pub const fn max() -> u32 {
        100
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
pub enum PLL1FRACVConf {
    Value(u32),
}

impl PLL1FRACVConf {
    pub const fn min() -> u32 {
        0
    }

    pub const fn max() -> u32 {
        8191
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
        25
    }

    pub const fn max() -> u32 {
        100
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
pub enum PLL2FRACVConf {
    Value(u32),
}

impl PLL2FRACVConf {
    pub const fn min() -> u32 {
        0
    }

    pub const fn max() -> u32 {
        8191
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
        25
    }

    pub const fn max() -> u32 {
        200
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
pub enum PLL3FRACVConf {
    Value(u32),
}

impl PLL3FRACVConf {
    pub const fn min() -> u32 {
        0
    }

    pub const fn max() -> u32 {
        8191
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
pub enum DIVN4Conf {
    Value(u32),
}

impl DIVN4Conf {
    pub const fn min() -> u32 {
        25
    }

    pub const fn max() -> u32 {
        200
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVN4Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVN4,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVN4,
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
        8191
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
pub enum DIVP4Conf {
    Value(u32),
}

impl DIVP4Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        128
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVP4Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVP4,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVP4,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DIVQ4Conf {
    Value(u32),
}

impl DIVQ4Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        128
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVQ4Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVQ4,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVQ4,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DIVR4Conf {
    Value(u32),
}

impl DIVR4Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        128
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            DIVR4Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVR4,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::DIVR4,
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
        1
    }

    pub const fn max() -> u32 {
        64
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C12MultConf {
    APB1DIV,
    DIVR4,
    HSIDiv,
    CSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C3MultConf {
    APB6DIV,
    DIVR4,
    HSIDiv,
    CSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C4MultConf {
    APB6DIV,
    DIVR4,
    HSIDiv,
    CSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C5MultConf {
    APB6DIV,
    DIVR4,
    HSIDiv,
    CSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPDIFMultConf {
    DIVP4,
    DIVQ3,
    HSIDiv,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum QSPIMultConf {
    AXIOutput,
    DIVP4,
    DIVR3,
    CKPERCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FMCMultConf {
    AXIOutput,
    DIVR3,
    DIVP4,
    CKPERCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SDMMC1MultConf {
    Hclk6Output,
    DIVR3,
    DIVP4,
    HSIDiv,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SDMMC2MultConf {
    Hclk6Output,
    DIVR3,
    DIVP4,
    HSIDiv,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum STGENMultConf {
    HSIDiv,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM45MultConf {
    APB3DIV,
    DIVP4,
    DIVQ3,
    LSEOSC,
    LSIRC,
    CKPERCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM2MultConf {
    APB3DIV,
    DIVQ4,
    CKPERCLKOutput,
    LSEOSC,
    LSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM1MultConf {
    APB1DIV,
    DIVP4,
    DIVQ3,
    LSEOSC,
    LSIRC,
    CKPERCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART1MultConf {
    APB6DIV,
    DIVQ4,
    DIVQ3,
    HSEOSC,
    CSIRC,
    HSIDiv,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART2MultConf {
    APB6DIV,
    DIVQ4,
    HSEOSC,
    CSIRC,
    HSIDiv,
    DIVQ3,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART35MultConf {
    APB1DIV,
    DIVQ4,
    HSEOSC,
    CSIRC,
    HSIDiv,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART6MultConf {
    APB2DIV,
    DIVQ4,
    HSEOSC,
    CSIRC,
    HSIDiv,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum UART78MultConf {
    APB1DIV,
    DIVQ4,
    HSEOSC,
    CSIRC,
    HSIDiv,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum RNG1MultConf {
    CSIRC,
    DIVR4,
    LSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DCMIMultConf {
    AXIOutput,
    DIVQ2,
    DIVP4,
    CKPERCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SAESMultConf {
    AXIOutput,
    CKPERCLKOutput,
    DIVR4,
    LSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM3MultConf {
    APB3DIV,
    DIVQ4,
    CKPERCLKOutput,
    LSEOSC,
    LSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI4MultConf {
    APB6DIV,
    DIVQ4,
    HSIDiv,
    CSIRC,
    HSEOSC,
    I2S_CKIN,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SAI2MultConf {
    DIVQ4,
    DIVQ3,
    I2S_CKIN,
    CKPERCLKOutput,
    SPDIFMult,
    DIVR3,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART4MultConf {
    APB1DIV,
    DIVQ4,
    HSEOSC,
    CSIRC,
    HSIDiv,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI1MultConf {
    DIVP4,
    DIVQ3,
    I2S_CKIN,
    CKPERCLKOutput,
    DIVR3,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI23MultConf {
    DIVP4,
    DIVQ3,
    I2S_CKIN,
    CKPERCLKOutput,
    DIVR3,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SAI1MultConf {
    DIVQ4,
    DIVQ3,
    I2S_CKIN,
    CKPERCLKOutput,
    DIVR3,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI5MultConf {
    APB6DIV,
    DIVQ4,
    HSIDiv,
    CSIRC,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FDCANMultConf {
    HSEOSC,
    DIVQ3,
    DIVQ4,
    DIVR4,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ETH1MultConf {
    DIVP4,
    DIVQ3,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ETH2MultConf {
    DIVP4,
    DIVQ3,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ADC2MultConf {
    DIVR4,
    CKPERCLKOutput,
    DIVQ3,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ADC1MultConf {
    DIVR4,
    CKPERCLKOutput,
    DIVQ3,
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
    HSEUSBPHYDevisor,
    HSEOSC,
    DIVR4,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USBOCLKMuxConf {
    DIVR4,
    USBPHYRC,
}
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ClockTree {
    pub HSIDiv: HSIDivConf,
    pub HSEOSC: HSEOSCConf,
    pub LSEOSC: LSEOSCConf,
    pub SysClkSource: SysClkSourceConf,
    pub MPUMult: MPUMultConf,
    pub CKPERMult: CKPERMultConf,
    pub AXIMult: AXIMultConf,
    pub AXIDIV: AXIDIVConf,
    pub APB4DIV: APB4DIVConf,
    pub APB5DIV: APB5DIVConf,
    pub APB6DIV: APB6DIVConf,
    pub MCO1Mult: MCO1MultConf,
    pub MCO1Div: MCO1DivConf,
    pub MCO2Mult: MCO2MultConf,
    pub MCO2Div: MCO2DivConf,
    pub MLAHBDIV: MLAHBDIVConf,
    pub APB3DIV: APB3DIVConf,
    pub APB1DIV: APB1DIVConf,
    pub APB2DIV: APB2DIVConf,
    pub PLL12Source: PLL12SourceConf,
    pub DIVM1: DIVM1Conf,
    pub DIVM2: DIVM2Conf,
    pub PLL3Source: PLL3SourceConf,
    pub DIVM3: DIVM3Conf,
    pub PLL4Source: PLL4SourceConf,
    pub DIVM4: DIVM4Conf,
    pub MPUDIV: MPUDIVConf,
    pub DIVN1: DIVN1Conf,
    pub PLL1FRACV: PLL1FRACVConf,
    pub DIVP1: DIVP1Conf,
    pub DIVQ1: DIVQ1Conf,
    pub DIVR1: DIVR1Conf,
    pub DIVN2: DIVN2Conf,
    pub PLL2FRACV: PLL2FRACVConf,
    pub DIVP2: DIVP2Conf,
    pub DIVQ2: DIVQ2Conf,
    pub DIVR2: DIVR2Conf,
    pub DIVN3: DIVN3Conf,
    pub PLL3FRACV: PLL3FRACVConf,
    pub DIVP3: DIVP3Conf,
    pub DIVQ3: DIVQ3Conf,
    pub DIVR3: DIVR3Conf,
    pub DIVN4: DIVN4Conf,
    pub PLL4FRACV: PLL4FRACVConf,
    pub DIVP4: DIVP4Conf,
    pub DIVQ4: DIVQ4Conf,
    pub DIVR4: DIVR4Conf,
    pub HSERTCDevisor: HSERTCDevisorConf,
    pub RTCClkSource: RTCClkSourceConf,
    pub I2C12Mult: I2C12MultConf,
    pub I2C3Mult: I2C3MultConf,
    pub I2C4Mult: I2C4MultConf,
    pub I2C5Mult: I2C5MultConf,
    pub SPDIFMult: SPDIFMultConf,
    pub QSPIMult: QSPIMultConf,
    pub FMCMult: FMCMultConf,
    pub SDMMC1Mult: SDMMC1MultConf,
    pub SDMMC2Mult: SDMMC2MultConf,
    pub STGENMult: STGENMultConf,
    pub LPTIM45Mult: LPTIM45MultConf,
    pub LPTIM2Mult: LPTIM2MultConf,
    pub LPTIM1Mult: LPTIM1MultConf,
    pub USART1Mult: USART1MultConf,
    pub USART2Mult: USART2MultConf,
    pub USART35Mult: USART35MultConf,
    pub USART6Mult: USART6MultConf,
    pub UART78Mult: UART78MultConf,
    pub RNG1Mult: RNG1MultConf,
    pub DCMIMult: DCMIMultConf,
    pub SAESMult: SAESMultConf,
    pub LPTIM3Mult: LPTIM3MultConf,
    pub SPI4Mult: SPI4MultConf,
    pub SAI2Mult: SAI2MultConf,
    pub USART4Mult: USART4MultConf,
    pub SPI1Mult: SPI1MultConf,
    pub SPI23Mult: SPI23MultConf,
    pub SAI1Mult: SAI1MultConf,
    pub SPI5Mult: SPI5MultConf,
    pub FDCANMult: FDCANMultConf,
    pub ETH1Mult: ETH1MultConf,
    pub ETH2Mult: ETH2MultConf,
    pub ADC2Mult: ADC2MultConf,
    pub ADC1Mult: ADC1MultConf,
    pub HSEUSBPHYDevisor: HSEUSBPHYDevisorConf,
    pub USBPHYCLKMux: USBPHYCLKMuxConf,
    pub USBOCLKMux: USBOCLKMuxConf,
}

impl Default for ClockTree {
    fn default() -> Self {
        Self {
            HSIDiv: HSIDivConf::DIV1,
            HSEOSC: HSEOSCConf::Value(24000000),
            LSEOSC: LSEOSCConf::Value(32768),
            SysClkSource: SysClkSourceConf::HSIDiv,
            MPUMult: MPUMultConf::HSIDiv,
            CKPERMult: CKPERMultConf::HSIDiv,
            AXIMult: AXIMultConf::HSIDiv,
            AXIDIV: AXIDIVConf::DIV1,
            APB4DIV: APB4DIVConf::DIV1,
            APB5DIV: APB5DIVConf::DIV1,
            APB6DIV: APB6DIVConf::DIV1,
            MCO1Mult: MCO1MultConf::HSIDiv,
            MCO1Div: MCO1DivConf::DIV1,
            MCO2Mult: MCO2MultConf::MPUCLKOutput,
            MCO2Div: MCO2DivConf::DIV1,
            MLAHBDIV: MLAHBDIVConf::DIV1,
            APB3DIV: APB3DIVConf::DIV1,
            APB1DIV: APB1DIVConf::DIV1,
            APB2DIV: APB2DIVConf::DIV1,
            PLL12Source: PLL12SourceConf::HSIDiv,
            DIVM1: DIVM1Conf::Value(2),
            DIVM2: DIVM2Conf::Value(2),
            PLL3Source: PLL3SourceConf::HSIDiv,
            DIVM3: DIVM3Conf::Value(2),
            PLL4Source: PLL4SourceConf::HSIDiv,
            DIVM4: DIVM4Conf::Value(2),
            MPUDIV: MPUDIVConf::DIV2,
            DIVN1: DIVN1Conf::Value(50),
            PLL1FRACV: PLL1FRACVConf::Value(0),
            DIVP1: DIVP1Conf::Value(1),
            DIVQ1: DIVQ1Conf::Value(2),
            DIVR1: DIVR1Conf::Value(2),
            DIVN2: DIVN2Conf::Value(100),
            PLL2FRACV: PLL2FRACVConf::Value(0),
            DIVP2: DIVP2Conf::Value(2),
            DIVQ2: DIVQ2Conf::Value(2),
            DIVR2: DIVR2Conf::Value(2),
            DIVN3: DIVN3Conf::Value(50),
            PLL3FRACV: PLL3FRACVConf::Value(0),
            DIVP3: DIVP3Conf::Value(2),
            DIVQ3: DIVQ3Conf::Value(2),
            DIVR3: DIVR3Conf::Value(2),
            DIVN4: DIVN4Conf::Value(50),
            PLL4FRACV: PLL4FRACVConf::Value(0),
            DIVP4: DIVP4Conf::Value(1),
            DIVQ4: DIVQ4Conf::Value(1),
            DIVR4: DIVR4Conf::Value(1),
            HSERTCDevisor: HSERTCDevisorConf::Value(1),
            RTCClkSource: RTCClkSourceConf::LSIRC,
            I2C12Mult: I2C12MultConf::APB1DIV,
            I2C3Mult: I2C3MultConf::APB6DIV,
            I2C4Mult: I2C4MultConf::APB6DIV,
            I2C5Mult: I2C5MultConf::APB6DIV,
            SPDIFMult: SPDIFMultConf::DIVP4,
            QSPIMult: QSPIMultConf::AXIOutput,
            FMCMult: FMCMultConf::AXIOutput,
            SDMMC1Mult: SDMMC1MultConf::Hclk6Output,
            SDMMC2Mult: SDMMC2MultConf::Hclk6Output,
            STGENMult: STGENMultConf::HSIDiv,
            LPTIM45Mult: LPTIM45MultConf::APB3DIV,
            LPTIM2Mult: LPTIM2MultConf::APB3DIV,
            LPTIM1Mult: LPTIM1MultConf::APB1DIV,
            USART1Mult: USART1MultConf::APB6DIV,
            USART2Mult: USART2MultConf::APB6DIV,
            USART35Mult: USART35MultConf::APB1DIV,
            USART6Mult: USART6MultConf::APB2DIV,
            UART78Mult: UART78MultConf::APB1DIV,
            RNG1Mult: RNG1MultConf::CSIRC,
            DCMIMult: DCMIMultConf::AXIOutput,
            SAESMult: SAESMultConf::AXIOutput,
            LPTIM3Mult: LPTIM3MultConf::APB3DIV,
            SPI4Mult: SPI4MultConf::APB6DIV,
            SAI2Mult: SAI2MultConf::DIVQ4,
            USART4Mult: USART4MultConf::APB1DIV,
            SPI1Mult: SPI1MultConf::DIVP4,
            SPI23Mult: SPI23MultConf::DIVP4,
            SAI1Mult: SAI1MultConf::DIVQ4,
            SPI5Mult: SPI5MultConf::APB6DIV,
            FDCANMult: FDCANMultConf::HSEOSC,
            ETH1Mult: ETH1MultConf::DIVP4,
            ETH2Mult: ETH2MultConf::DIVP4,
            ADC2Mult: ADC2MultConf::DIVR4,
            ADC1Mult: ADC1MultConf::DIVR4,
            HSEUSBPHYDevisor: HSEUSBPHYDevisorConf::DIV2,
            USBPHYCLKMux: USBPHYCLKMuxConf::DIVR4,
            USBOCLKMux: USBOCLKMuxConf::DIVR4,
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
        Ok(32000 as f32)
    }
    pub fn LSEOSC_get(&self) -> Result<f32, ClockError> {
        self.LSEOSC.get()
    }
    pub fn CSIRC_get(&self) -> Result<f32, ClockError> {
        Ok(4000000 as f32)
    }
    pub fn I2S_CKIN_get(&self) -> Result<f32, ClockError> {
        Ok(12288000 as f32)
    }
    fn SysClkSource_get(&self) -> Result<f32, ClockError> {
        match self.SysClkSource {
            SysClkSourceConf::HSIDiv => return self.HSIDiv_get(),
            SysClkSourceConf::CSIRC => return self.CSIRC_get(),
            SysClkSourceConf::HSEOSC => return self.HSEOSC_get(),
            SysClkSourceConf::DIVP3 => return self.DIVP3_get(),
        };
    }
    pub fn SysCLKOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SysClkSource_get()?;
        if input > (209000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 209000000),
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
    fn MPUMult_get(&self) -> Result<f32, ClockError> {
        match self.MPUMult {
            MPUMultConf::DIVP1 => return self.DIVP1_get(),
            MPUMultConf::MPUDIV => return self.MPUDIV_get(),
            MPUMultConf::HSEOSC => return self.HSEOSC_get(),
            MPUMultConf::HSIDiv => return self.HSIDiv_get(),
        };
    }
    pub fn MPUCLKOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.MPUMult_get()?;
        if input > (650000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 650000000),
                from: ClockNodes::MPUMult,
                to: ClockNodes::MPUCLKOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::MPUMult,
                to: ClockNodes::MPUCLKOutput,
            });
        }
        Ok(input)
    }
    fn CKPERMult_get(&self) -> Result<f32, ClockError> {
        match self.CKPERMult {
            CKPERMultConf::HSIDiv => return self.HSIDiv_get(),
            CKPERMultConf::CSIRC => return self.CSIRC_get(),
            CKPERMultConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    pub fn CKPERCLKOutput_get(&self) -> Result<f32, ClockError> {
        self.CKPERMult_get()
    }
    fn AXIMult_get(&self) -> Result<f32, ClockError> {
        match self.AXIMult {
            AXIMultConf::HSEOSC => return self.HSEOSC_get(),
            AXIMultConf::HSIDiv => return self.HSIDiv_get(),
            AXIMultConf::DIVP2 => return self.DIVP2_get(),
        };
    }
    pub fn AXICLKOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.AXIMult_get()?;
        if input > (266500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 266500000),
                from: ClockNodes::AXIMult,
                to: ClockNodes::AXICLKOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::AXIMult,
                to: ClockNodes::AXICLKOutput,
            });
        }
        Ok(input)
    }
    pub fn DACCLKOutput_get(&self) -> Result<f32, ClockError> {
        self.LSIRC_get()
    }
    fn AXIDIV_get(&self) -> Result<f32, ClockError> {
        let input = self.AXICLKOutput_get()? as f32;
        let value = self.AXIDIV.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn AXIOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.AXIDIV_get()?;
        if input > (266500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 266500000),
                from: ClockNodes::AXIDIV,
                to: ClockNodes::AXIOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::AXIDIV,
                to: ClockNodes::AXIOutput,
            });
        }
        Ok(input)
    }
    pub fn Hclk5Output_get(&self) -> Result<f32, ClockError> {
        let input = self.AXIDIV_get()?;
        if input > (266500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 266500000),
                from: ClockNodes::AXIDIV,
                to: ClockNodes::Hclk5Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::AXIDIV,
                to: ClockNodes::Hclk5Output,
            });
        }
        Ok(input)
    }
    pub fn Hclk6Output_get(&self) -> Result<f32, ClockError> {
        let input = self.AXIDIV_get()?;
        if input > (266500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 266500000),
                from: ClockNodes::AXIDIV,
                to: ClockNodes::Hclk6Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::AXIDIV,
                to: ClockNodes::Hclk6Output,
            });
        }
        Ok(input)
    }
    fn APB4DIV_get(&self) -> Result<f32, ClockError> {
        let input = self.AXIDIV_get()? as f32;
        let value = self.APB4DIV.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB4DIVOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.APB4DIV_get()?;
        if input > (133250000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133250000),
                from: ClockNodes::APB4DIV,
                to: ClockNodes::APB4DIVOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::APB4DIV,
                to: ClockNodes::APB4DIVOutput,
            });
        }
        Ok(input)
    }
    fn APB5DIV_get(&self) -> Result<f32, ClockError> {
        let input = self.AXIDIV_get()? as f32;
        let value = self.APB5DIV.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB5DIVOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.APB5DIV_get()?;
        if input > (67000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 67000000),
                from: ClockNodes::APB5DIV,
                to: ClockNodes::APB5DIVOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::APB5DIV,
                to: ClockNodes::APB5DIVOutput,
            });
        }
        Ok(input)
    }
    fn APB6DIV_get(&self) -> Result<f32, ClockError> {
        let input = self.MLAHBDIV_get()? as f32;
        let value = self.APB6DIV.get()? as f32;
        Ok((input / value) as f32)
    }

    fn Tim3Mul_get(&self) -> Result<f32, ClockError> {
        let input = self.APB6DIV_get()? as f32;
        let value = 2 as f32;
        Ok((input * value) as f32)
    }

    pub fn Tim3Output_get(&self) -> Result<f32, ClockError> {
        let input = self.Tim3Mul_get()?;
        if input > (209000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 209000000),
                from: ClockNodes::Tim3Mul,
                to: ClockNodes::Tim3Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::Tim3Mul,
                to: ClockNodes::Tim3Output,
            });
        }
        Ok(input)
    }
    pub fn APB6DIVOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.APB6DIV_get()?;
        if input > (104500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 104500000),
                from: ClockNodes::APB6DIV,
                to: ClockNodes::APB6DIVOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::APB6DIV,
                to: ClockNodes::APB6DIVOutput,
            });
        }
        Ok(input)
    }
    fn MCO1Mult_get(&self) -> Result<f32, ClockError> {
        match self.MCO1Mult {
            MCO1MultConf::HSIDiv => return self.HSIDiv_get(),
            MCO1MultConf::HSEOSC => return self.HSEOSC_get(),
            MCO1MultConf::CSIRC => return self.CSIRC_get(),
            MCO1MultConf::LSIRC => return self.LSIRC_get(),
            MCO1MultConf::LSEOSC => return self.LSEOSC_get(),
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
            MCO2MultConf::MPUCLKOutput => return self.MPUCLKOutput_get(),
            MCO2MultConf::AXIDIV => return self.AXIDIV_get(),
            MCO2MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            MCO2MultConf::DIVP4 => return self.DIVP4_get(),
            MCO2MultConf::HSEOSC => return self.HSEOSC_get(),
            MCO2MultConf::HSIDiv => return self.HSIDiv_get(),
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
    fn MLAHBDIV_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()? as f32;
        let value = self.MLAHBDIV.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn MLAHBClockOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.MLAHBDIV_get()?;
        if input > (209000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 209000000),
                from: ClockNodes::MLAHBDIV,
                to: ClockNodes::MLAHBClockOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::MLAHBDIV,
                to: ClockNodes::MLAHBClockOutput,
            });
        }
        Ok(input)
    }
    fn APB3DIV_get(&self) -> Result<f32, ClockError> {
        let input = self.MLAHBDIV_get()? as f32;
        let value = self.APB3DIV.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB3Output_get(&self) -> Result<f32, ClockError> {
        let input = self.APB3DIV_get()?;
        if input > (104500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 104500000),
                from: ClockNodes::APB3DIV,
                to: ClockNodes::APB3Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::APB3DIV,
                to: ClockNodes::APB3Output,
            });
        }
        Ok(input)
    }
    fn APB1DIV_get(&self) -> Result<f32, ClockError> {
        let input = self.MLAHBDIV_get()? as f32;
        let value = self.APB1DIV.get()? as f32;
        Ok((input / value) as f32)
    }

    fn Tim1Mul_get(&self) -> Result<f32, ClockError> {
        let input = self.APB1DIV_get()? as f32;
        let value = 2 as f32;
        Ok((input * value) as f32)
    }

    pub fn Tim1Output_get(&self) -> Result<f32, ClockError> {
        let input = self.Tim1Mul_get()?;
        if input > (209000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 209000000),
                from: ClockNodes::Tim1Mul,
                to: ClockNodes::Tim1Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::Tim1Mul,
                to: ClockNodes::Tim1Output,
            });
        }
        Ok(input)
    }
    pub fn AHBOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.MLAHBDIV_get()?;
        if input > (209000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 209000000),
                from: ClockNodes::MLAHBDIV,
                to: ClockNodes::AHBOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::MLAHBDIV,
                to: ClockNodes::AHBOutput,
            });
        }
        Ok(input)
    }
    pub fn APB1Output_get(&self) -> Result<f32, ClockError> {
        let input = self.APB1DIV_get()?;
        if input > (104500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 104500000),
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
        let input = self.MLAHBDIV_get()? as f32;
        let value = self.APB2DIV.get()? as f32;
        Ok((input / value) as f32)
    }

    fn Tim2Mul_get(&self) -> Result<f32, ClockError> {
        let input = self.APB2DIV_get()? as f32;
        let value = 2 as f32;
        Ok((input * value) as f32)
    }

    pub fn Tim2Output_get(&self) -> Result<f32, ClockError> {
        let input = self.Tim2Mul_get()?;
        if input > (209000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 209000000),
                from: ClockNodes::Tim2Mul,
                to: ClockNodes::Tim2Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::Tim2Mul,
                to: ClockNodes::Tim2Output,
            });
        }
        Ok(input)
    }
    pub fn APB2Output_get(&self) -> Result<f32, ClockError> {
        let input = self.APB2DIV_get()?;
        if input > (104500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 104500000),
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
    pub fn DFSDM1Output_get(&self) -> Result<f32, ClockError> {
        self.MLAHBDIV_get()
    }
    fn PLL12Source_get(&self) -> Result<f32, ClockError> {
        match self.PLL12Source {
            PLL12SourceConf::HSIDiv => return self.HSIDiv_get(),
            PLL12SourceConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    fn DIVM1_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL12Source_get()? as f32;
        let value = self.DIVM1.get()? as f32;
        Ok((input / value) as f32)
    }

    fn DIVM2_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL12Source_get()? as f32;
        let value = self.DIVM2.get()? as f32;
        Ok((input / value) as f32)
    }

    fn PLL3Source_get(&self) -> Result<f32, ClockError> {
        match self.PLL3Source {
            PLL3SourceConf::HSIDiv => return self.HSIDiv_get(),
            PLL3SourceConf::CSIRC => return self.CSIRC_get(),
            PLL3SourceConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    fn DIVM3_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL3Source_get()? as f32;
        let value = self.DIVM3.get()? as f32;
        Ok((input / value) as f32)
    }

    fn PLL4Source_get(&self) -> Result<f32, ClockError> {
        match self.PLL4Source {
            PLL4SourceConf::HSIDiv => return self.HSIDiv_get(),
            PLL4SourceConf::CSIRC => return self.CSIRC_get(),
            PLL4SourceConf::HSEOSC => return self.HSEOSC_get(),
            PLL4SourceConf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    fn DIVM4_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL4Source_get()? as f32;
        let value = self.DIVM4.get()? as f32;
        Ok((input / value) as f32)
    }

    fn MPUDIV_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVP1_get()? as f32;
        let value = self.MPUDIV.get()? as f32;
        Ok((input / value) as f32)
    }

    fn FreqCk1_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVM1_get()? as f32;
        let value = 2 as f32;
        Ok((input * value) as f32)
    }

    fn DIVN1_get(&self) -> Result<f32, ClockError> {
        let input = self.FreqCk1_get()? as f32;
        let frac = self.PLL1FRACV_get()? as f32;
        let frac_max = PLL1FRACVConf::max() as f32;
        let value = self.DIVN1.get()? as f32;
        let ret = (input * (value + (frac / frac_max)));
        Ok(ret as f32)
    }

    pub fn PLL1FRACV_get(&self) -> Result<f32, ClockError> {
        self.PLL1FRACV.get()
    }
    fn DIVN1Mul2Div2_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN1_get()? as f32;
        let value = 2 as f32;
        Ok((input / value) as f32)
    }

    fn DIVP1_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN1Mul2Div2_get()? as f32;
        let value = self.DIVP1.get()? as f32;
        Ok((input / value) as f32)
    }

    fn DIVQ1_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN1Mul2Div2_get()? as f32;
        let value = self.DIVQ1.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn DIVQ1output_get(&self) -> Result<f32, ClockError> {
        self.DIVQ1_get()
    }
    fn DIVR1_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN1Mul2Div2_get()? as f32;
        let value = self.DIVR1.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn DIVR1output_get(&self) -> Result<f32, ClockError> {
        self.DIVR1_get()
    }
    fn FreqCk2_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVM2_get()? as f32;
        let value = 2 as f32;
        Ok((input * value) as f32)
    }

    fn DIVN2_get(&self) -> Result<f32, ClockError> {
        let input = self.FreqCk2_get()? as f32;
        let frac = self.PLL2FRACV_get()? as f32;
        let frac_max = PLL2FRACVConf::max() as f32;
        let value = self.DIVN2.get()? as f32;
        let ret = (input * (value + (frac / frac_max)));
        Ok(ret as f32)
    }

    pub fn PLL2FRACV_get(&self) -> Result<f32, ClockError> {
        self.PLL2FRACV.get()
    }
    fn DIVN2Mul2Div2_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN2_get()? as f32;
        let value = 2 as f32;
        Ok((input / value) as f32)
    }

    fn DIVP2_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN2Mul2Div2_get()? as f32;
        let value = self.DIVP2.get()? as f32;
        Ok((input / value) as f32)
    }

    fn DIVQ2_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN2Mul2Div2_get()? as f32;
        let value = self.DIVQ2.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn DIVQ2output_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVQ2_get()?;
        if input > (800000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 800000000),
                from: ClockNodes::DIVQ2,
                to: ClockNodes::DIVQ2output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::DIVQ2,
                to: ClockNodes::DIVQ2output,
            });
        }
        Ok(input)
    }
    fn DIVR2_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN2Mul2Div2_get()? as f32;
        let value = self.DIVR2.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn DIVR2output_get(&self) -> Result<f32, ClockError> {
        self.DIVR2_get()
    }
    fn DIVN3_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVM3_get()? as f32;
        let frac = self.PLL3FRACV_get()? as f32;
        let frac_max = PLL3FRACVConf::max() as f32;
        let value = self.DIVN3.get()? as f32;
        let ret = (input * (value + (frac / frac_max)));
        Ok(ret as f32)
    }

    pub fn PLL3FRACV_get(&self) -> Result<f32, ClockError> {
        self.PLL3FRACV.get()
    }
    fn DIVP3_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN3_get()? as f32;
        let value = self.DIVP3.get()? as f32;
        Ok((input / value) as f32)
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
        let input = self.DIVQ4_get()?;
        if input > (90000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 90000000),
                from: ClockNodes::DIVQ4,
                to: ClockNodes::LTDCOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::DIVQ4,
                to: ClockNodes::LTDCOutput,
            });
        }
        Ok(input)
    }
    pub fn DIVR3output_get(&self) -> Result<f32, ClockError> {
        self.DIVR3_get()
    }
    fn DIVN4_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVM4_get()? as f32;
        let frac = self.PLL4FRACV_get()? as f32;
        let frac_max = PLL4FRACVConf::max() as f32;
        let value = self.DIVN4.get()? as f32;
        let ret = (input * (value + (frac / frac_max)));
        Ok(ret as f32)
    }

    pub fn PLL4FRACV_get(&self) -> Result<f32, ClockError> {
        self.PLL4FRACV.get()
    }
    fn DIVP4_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN4_get()? as f32;
        let value = self.DIVP4.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn DIVP4output_get(&self) -> Result<f32, ClockError> {
        self.DIVP4_get()
    }
    fn DIVQ4_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN4_get()? as f32;
        let value = self.DIVQ4.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn DIVQ4output_get(&self) -> Result<f32, ClockError> {
        self.DIVQ4_get()
    }
    fn DIVR4_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVN4_get()? as f32;
        let value = self.DIVR4.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn DIVR4output_get(&self) -> Result<f32, ClockError> {
        self.DIVR4_get()
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
    fn I2C12Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C12Mult {
            I2C12MultConf::APB1DIV => return self.APB1DIV_get(),
            I2C12MultConf::DIVR4 => return self.DIVR4_get(),
            I2C12MultConf::HSIDiv => return self.HSIDiv_get(),
            I2C12MultConf::CSIRC => return self.CSIRC_get(),
        };
    }
    pub fn I2C12output_get(&self) -> Result<f32, ClockError> {
        let input = self.I2C12Mult_get()?;
        if input > (104500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 104500000),
                from: ClockNodes::I2C12Mult,
                to: ClockNodes::I2C12output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::I2C12Mult,
                to: ClockNodes::I2C12output,
            });
        }
        Ok(input)
    }
    fn I2C3Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C3Mult {
            I2C3MultConf::APB6DIV => return self.APB6DIV_get(),
            I2C3MultConf::DIVR4 => return self.DIVR4_get(),
            I2C3MultConf::HSIDiv => return self.HSIDiv_get(),
            I2C3MultConf::CSIRC => return self.CSIRC_get(),
        };
    }
    pub fn I2C3output_get(&self) -> Result<f32, ClockError> {
        let input = self.I2C3Mult_get()?;
        if input > (104500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 104500000),
                from: ClockNodes::I2C3Mult,
                to: ClockNodes::I2C3output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::I2C3Mult,
                to: ClockNodes::I2C3output,
            });
        }
        Ok(input)
    }
    fn I2C4Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C4Mult {
            I2C4MultConf::APB6DIV => return self.APB6DIV_get(),
            I2C4MultConf::DIVR4 => return self.DIVR4_get(),
            I2C4MultConf::HSIDiv => return self.HSIDiv_get(),
            I2C4MultConf::CSIRC => return self.CSIRC_get(),
        };
    }
    pub fn I2C4output_get(&self) -> Result<f32, ClockError> {
        let input = self.I2C4Mult_get()?;
        if input > (104500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 104500000),
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
    fn I2C5Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C5Mult {
            I2C5MultConf::APB6DIV => return self.APB6DIV_get(),
            I2C5MultConf::DIVR4 => return self.DIVR4_get(),
            I2C5MultConf::HSIDiv => return self.HSIDiv_get(),
            I2C5MultConf::CSIRC => return self.CSIRC_get(),
        };
    }
    pub fn I2C5output_get(&self) -> Result<f32, ClockError> {
        let input = self.I2C5Mult_get()?;
        if input > (104500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 104500000),
                from: ClockNodes::I2C5Mult,
                to: ClockNodes::I2C5output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::I2C5Mult,
                to: ClockNodes::I2C5output,
            });
        }
        Ok(input)
    }
    fn SPDIFMult_get(&self) -> Result<f32, ClockError> {
        match self.SPDIFMult {
            SPDIFMultConf::DIVP4 => return self.DIVP4_get(),
            SPDIFMultConf::DIVQ3 => return self.DIVQ3_get(),
            SPDIFMultConf::HSIDiv => return self.HSIDiv_get(),
        };
    }
    pub fn SPDIFoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SPDIFMult_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
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
            QSPIMultConf::AXIOutput => return self.AXIOutput_get(),
            QSPIMultConf::DIVP4 => return self.DIVP4_get(),
            QSPIMultConf::DIVR3 => return self.DIVR3_get(),
            QSPIMultConf::CKPERCLKOutput => return self.CKPERCLKOutput_get(),
        };
    }
    pub fn QSPIoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.QSPIMult_get()?;
        if input > (266500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 266500000),
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
            FMCMultConf::AXIOutput => return self.AXIOutput_get(),
            FMCMultConf::DIVR3 => return self.DIVR3_get(),
            FMCMultConf::DIVP4 => return self.DIVP4_get(),
            FMCMultConf::CKPERCLKOutput => return self.CKPERCLKOutput_get(),
        };
    }
    pub fn FMCoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.FMCMult_get()?;
        if input > (266500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 266500000),
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
    fn SDMMC1Mult_get(&self) -> Result<f32, ClockError> {
        match self.SDMMC1Mult {
            SDMMC1MultConf::Hclk6Output => return self.Hclk6Output_get(),
            SDMMC1MultConf::DIVR3 => return self.DIVR3_get(),
            SDMMC1MultConf::DIVP4 => return self.DIVP4_get(),
            SDMMC1MultConf::HSIDiv => return self.HSIDiv_get(),
        };
    }
    pub fn SDMMC1output_get(&self) -> Result<f32, ClockError> {
        let input = self.SDMMC1Mult_get()?;
        if input > (266000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 266000000),
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
            SDMMC2MultConf::Hclk6Output => return self.Hclk6Output_get(),
            SDMMC2MultConf::DIVR3 => return self.DIVR3_get(),
            SDMMC2MultConf::DIVP4 => return self.DIVP4_get(),
            SDMMC2MultConf::HSIDiv => return self.HSIDiv_get(),
        };
    }
    pub fn SDMMC2output_get(&self) -> Result<f32, ClockError> {
        let input = self.SDMMC2Mult_get()?;
        if input > (266000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 266000000),
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
    fn STGENMult_get(&self) -> Result<f32, ClockError> {
        match self.STGENMult {
            STGENMultConf::HSIDiv => return self.HSIDiv_get(),
            STGENMultConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    pub fn STGENoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.STGENMult_get()?;
        if input > (64000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 64000000),
                from: ClockNodes::STGENMult,
                to: ClockNodes::STGENoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::STGENMult,
                to: ClockNodes::STGENoutput,
            });
        }
        Ok(input)
    }
    fn LPTIM45Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM45Mult {
            LPTIM45MultConf::APB3DIV => return self.APB3DIV_get(),
            LPTIM45MultConf::DIVP4 => return self.DIVP4_get(),
            LPTIM45MultConf::DIVQ3 => return self.DIVQ3_get(),
            LPTIM45MultConf::LSEOSC => return self.LSEOSC_get(),
            LPTIM45MultConf::LSIRC => return self.LSIRC_get(),
            LPTIM45MultConf::CKPERCLKOutput => return self.CKPERCLKOutput_get(),
        };
    }
    pub fn LPTIM45output_get(&self) -> Result<f32, ClockError> {
        let input = self.LPTIM45Mult_get()?;
        if input > (104500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 104500000),
                from: ClockNodes::LPTIM45Mult,
                to: ClockNodes::LPTIM45output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::LPTIM45Mult,
                to: ClockNodes::LPTIM45output,
            });
        }
        Ok(input)
    }
    fn LPTIM2Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM2Mult {
            LPTIM2MultConf::APB3DIV => return self.APB3DIV_get(),
            LPTIM2MultConf::DIVQ4 => return self.DIVQ4_get(),
            LPTIM2MultConf::CKPERCLKOutput => return self.CKPERCLKOutput_get(),
            LPTIM2MultConf::LSEOSC => return self.LSEOSC_get(),
            LPTIM2MultConf::LSIRC => return self.LSIRC_get(),
        };
    }
    pub fn LPTIM2output_get(&self) -> Result<f32, ClockError> {
        let input = self.LPTIM2Mult_get()?;
        if input > (104500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 104500000),
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
    fn LPTIM1Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM1Mult {
            LPTIM1MultConf::APB1DIV => return self.APB1DIV_get(),
            LPTIM1MultConf::DIVP4 => return self.DIVP4_get(),
            LPTIM1MultConf::DIVQ3 => return self.DIVQ3_get(),
            LPTIM1MultConf::LSEOSC => return self.LSEOSC_get(),
            LPTIM1MultConf::LSIRC => return self.LSIRC_get(),
            LPTIM1MultConf::CKPERCLKOutput => return self.CKPERCLKOutput_get(),
        };
    }
    pub fn LPTIM1output_get(&self) -> Result<f32, ClockError> {
        let input = self.LPTIM1Mult_get()?;
        if input > (104500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 104500000),
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
    fn USART1Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART1Mult {
            USART1MultConf::APB6DIV => return self.APB6DIV_get(),
            USART1MultConf::DIVQ4 => return self.DIVQ4_get(),
            USART1MultConf::DIVQ3 => return self.DIVQ3_get(),
            USART1MultConf::HSEOSC => return self.HSEOSC_get(),
            USART1MultConf::CSIRC => return self.CSIRC_get(),
            USART1MultConf::HSIDiv => return self.HSIDiv_get(),
        };
    }
    pub fn USART1output_get(&self) -> Result<f32, ClockError> {
        let input = self.USART1Mult_get()?;
        if input > (104500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 104500000),
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
            USART2MultConf::APB6DIV => return self.APB6DIV_get(),
            USART2MultConf::DIVQ4 => return self.DIVQ4_get(),
            USART2MultConf::HSEOSC => return self.HSEOSC_get(),
            USART2MultConf::CSIRC => return self.CSIRC_get(),
            USART2MultConf::HSIDiv => return self.HSIDiv_get(),
            USART2MultConf::DIVQ3 => return self.DIVQ3_get(),
        };
    }
    pub fn USART2output_get(&self) -> Result<f32, ClockError> {
        let input = self.USART2Mult_get()?;
        if input > (104500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 104500000),
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
    fn USART35Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART35Mult {
            USART35MultConf::APB1DIV => return self.APB1DIV_get(),
            USART35MultConf::DIVQ4 => return self.DIVQ4_get(),
            USART35MultConf::HSEOSC => return self.HSEOSC_get(),
            USART35MultConf::CSIRC => return self.CSIRC_get(),
            USART35MultConf::HSIDiv => return self.HSIDiv_get(),
        };
    }
    pub fn USART35output_get(&self) -> Result<f32, ClockError> {
        let input = self.USART35Mult_get()?;
        if input > (104500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 104500000),
                from: ClockNodes::USART35Mult,
                to: ClockNodes::USART35output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::USART35Mult,
                to: ClockNodes::USART35output,
            });
        }
        Ok(input)
    }
    fn USART6Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART6Mult {
            USART6MultConf::APB2DIV => return self.APB2DIV_get(),
            USART6MultConf::DIVQ4 => return self.DIVQ4_get(),
            USART6MultConf::HSEOSC => return self.HSEOSC_get(),
            USART6MultConf::CSIRC => return self.CSIRC_get(),
            USART6MultConf::HSIDiv => return self.HSIDiv_get(),
        };
    }
    pub fn USART6output_get(&self) -> Result<f32, ClockError> {
        let input = self.USART6Mult_get()?;
        if input > (104500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 104500000),
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
    fn UART78Mult_get(&self) -> Result<f32, ClockError> {
        match self.UART78Mult {
            UART78MultConf::APB1DIV => return self.APB1DIV_get(),
            UART78MultConf::DIVQ4 => return self.DIVQ4_get(),
            UART78MultConf::HSEOSC => return self.HSEOSC_get(),
            UART78MultConf::CSIRC => return self.CSIRC_get(),
            UART78MultConf::HSIDiv => return self.HSIDiv_get(),
        };
    }
    pub fn USART78output_get(&self) -> Result<f32, ClockError> {
        let input = self.UART78Mult_get()?;
        if input > (104500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 104500000),
                from: ClockNodes::UART78Mult,
                to: ClockNodes::USART78output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::UART78Mult,
                to: ClockNodes::USART78output,
            });
        }
        Ok(input)
    }
    fn RNG1Mult_get(&self) -> Result<f32, ClockError> {
        match self.RNG1Mult {
            RNG1MultConf::CSIRC => return self.CSIRC_get(),
            RNG1MultConf::DIVR4 => return self.DIVR4_get(),
            RNG1MultConf::LSIRC => return self.LSIRC_get(),
        };
    }
    pub fn RNG1output_get(&self) -> Result<f32, ClockError> {
        let input = self.RNG1Mult_get()?;
        if input > (360000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 360000000),
                from: ClockNodes::RNG1Mult,
                to: ClockNodes::RNG1output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::RNG1Mult,
                to: ClockNodes::RNG1output,
            });
        }
        Ok(input)
    }
    fn DCMIMult_get(&self) -> Result<f32, ClockError> {
        match self.DCMIMult {
            DCMIMultConf::AXIOutput => return self.AXIOutput_get(),
            DCMIMultConf::DIVQ2 => return self.DIVQ2_get(),
            DCMIMultConf::DIVP4 => return self.DIVP4_get(),
            DCMIMultConf::CKPERCLKOutput => return self.CKPERCLKOutput_get(),
        };
    }
    pub fn DCMIoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.DCMIMult_get()?;
        if input > (266500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 266500000),
                from: ClockNodes::DCMIMult,
                to: ClockNodes::DCMIoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::DCMIMult,
                to: ClockNodes::DCMIoutput,
            });
        }
        Ok(input)
    }
    fn SAESMult_get(&self) -> Result<f32, ClockError> {
        match self.SAESMult {
            SAESMultConf::AXIOutput => return self.AXIOutput_get(),
            SAESMultConf::CKPERCLKOutput => return self.CKPERCLKOutput_get(),
            SAESMultConf::DIVR4 => return self.DIVR4_get(),
            SAESMultConf::LSIRC => return self.LSIRC_get(),
        };
    }
    pub fn SAESoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SAESMult_get()?;
        if input > (360000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 360000000),
                from: ClockNodes::SAESMult,
                to: ClockNodes::SAESoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SAESMult,
                to: ClockNodes::SAESoutput,
            });
        }
        Ok(input)
    }
    fn LPTIM3Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM3Mult {
            LPTIM3MultConf::APB3DIV => return self.APB3DIV_get(),
            LPTIM3MultConf::DIVQ4 => return self.DIVQ4_get(),
            LPTIM3MultConf::CKPERCLKOutput => return self.CKPERCLKOutput_get(),
            LPTIM3MultConf::LSEOSC => return self.LSEOSC_get(),
            LPTIM3MultConf::LSIRC => return self.LSIRC_get(),
        };
    }
    pub fn LPTIM3output_get(&self) -> Result<f32, ClockError> {
        let input = self.LPTIM3Mult_get()?;
        if input > (104500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 104500000),
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
    fn SPI4Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI4Mult {
            SPI4MultConf::APB6DIV => return self.APB6DIV_get(),
            SPI4MultConf::DIVQ4 => return self.DIVQ4_get(),
            SPI4MultConf::HSIDiv => return self.HSIDiv_get(),
            SPI4MultConf::CSIRC => return self.CSIRC_get(),
            SPI4MultConf::HSEOSC => return self.HSEOSC_get(),
            SPI4MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    pub fn SPI4output_get(&self) -> Result<f32, ClockError> {
        let input = self.SPI4Mult_get()?;
        if input > (104500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 104500000),
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
    fn SAI2Mult_get(&self) -> Result<f32, ClockError> {
        match self.SAI2Mult {
            SAI2MultConf::DIVQ4 => return self.DIVQ4_get(),
            SAI2MultConf::DIVQ3 => return self.DIVQ3_get(),
            SAI2MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            SAI2MultConf::CKPERCLKOutput => return self.CKPERCLKOutput_get(),
            SAI2MultConf::SPDIFMult => return self.SPDIFMult_get(),
            SAI2MultConf::DIVR3 => return self.DIVR3_get(),
        };
    }
    pub fn SAI2output_get(&self) -> Result<f32, ClockError> {
        let input = self.SAI2Mult_get()?;
        if input > (133250000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133250000),
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
    fn USART4Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART4Mult {
            USART4MultConf::APB1DIV => return self.APB1DIV_get(),
            USART4MultConf::DIVQ4 => return self.DIVQ4_get(),
            USART4MultConf::HSEOSC => return self.HSEOSC_get(),
            USART4MultConf::CSIRC => return self.CSIRC_get(),
            USART4MultConf::HSIDiv => return self.HSIDiv_get(),
        };
    }
    pub fn USART4output_get(&self) -> Result<f32, ClockError> {
        let input = self.USART4Mult_get()?;
        if input > (104500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 104500000),
                from: ClockNodes::USART4Mult,
                to: ClockNodes::USART4output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::USART4Mult,
                to: ClockNodes::USART4output,
            });
        }
        Ok(input)
    }
    fn SPI1Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI1Mult {
            SPI1MultConf::DIVP4 => return self.DIVP4_get(),
            SPI1MultConf::DIVQ3 => return self.DIVQ3_get(),
            SPI1MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            SPI1MultConf::CKPERCLKOutput => return self.CKPERCLKOutput_get(),
            SPI1MultConf::DIVR3 => return self.DIVR3_get(),
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
    fn SPI23Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI23Mult {
            SPI23MultConf::DIVP4 => return self.DIVP4_get(),
            SPI23MultConf::DIVQ3 => return self.DIVQ3_get(),
            SPI23MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            SPI23MultConf::CKPERCLKOutput => return self.CKPERCLKOutput_get(),
            SPI23MultConf::DIVR3 => return self.DIVR3_get(),
        };
    }
    pub fn SPI23output_get(&self) -> Result<f32, ClockError> {
        let input = self.SPI23Mult_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
                from: ClockNodes::SPI23Mult,
                to: ClockNodes::SPI23output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SPI23Mult,
                to: ClockNodes::SPI23output,
            });
        }
        Ok(input)
    }
    fn SAI1Mult_get(&self) -> Result<f32, ClockError> {
        match self.SAI1Mult {
            SAI1MultConf::DIVQ4 => return self.DIVQ4_get(),
            SAI1MultConf::DIVQ3 => return self.DIVQ3_get(),
            SAI1MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            SAI1MultConf::CKPERCLKOutput => return self.CKPERCLKOutput_get(),
            SAI1MultConf::DIVR3 => return self.DIVR3_get(),
        };
    }
    pub fn SAI1output_get(&self) -> Result<f32, ClockError> {
        let input = self.SAI1Mult_get()?;
        if input > (133250000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133250000),
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
    pub fn DFSDF1Audiooutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SAI1Mult_get()?;
        if input > (133000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133000000),
                from: ClockNodes::SAI1Mult,
                to: ClockNodes::DFSDF1Audiooutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SAI1Mult,
                to: ClockNodes::DFSDF1Audiooutput,
            });
        }
        Ok(input)
    }
    fn SPI5Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI5Mult {
            SPI5MultConf::APB6DIV => return self.APB6DIV_get(),
            SPI5MultConf::DIVQ4 => return self.DIVQ4_get(),
            SPI5MultConf::HSIDiv => return self.HSIDiv_get(),
            SPI5MultConf::CSIRC => return self.CSIRC_get(),
            SPI5MultConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    pub fn SPI5output_get(&self) -> Result<f32, ClockError> {
        let input = self.SPI5Mult_get()?;
        if input > (133250000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133250000),
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
    fn FDCANMult_get(&self) -> Result<f32, ClockError> {
        match self.FDCANMult {
            FDCANMultConf::HSEOSC => return self.HSEOSC_get(),
            FDCANMultConf::DIVQ3 => return self.DIVQ3_get(),
            FDCANMultConf::DIVQ4 => return self.DIVQ4_get(),
            FDCANMultConf::DIVR4 => return self.DIVR4_get(),
        };
    }
    pub fn FDCANoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.FDCANMult_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
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
    fn ETH1Mult_get(&self) -> Result<f32, ClockError> {
        match self.ETH1Mult {
            ETH1MultConf::DIVP4 => return self.DIVP4_get(),
            ETH1MultConf::DIVQ3 => return self.DIVQ3_get(),
        };
    }
    pub fn ETH1output_get(&self) -> Result<f32, ClockError> {
        let input = self.ETH1Mult_get()?;
        if input > (125000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 125000000),
                from: ClockNodes::ETH1Mult,
                to: ClockNodes::ETH1output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::ETH1Mult,
                to: ClockNodes::ETH1output,
            });
        }
        Ok(input)
    }
    fn ETH2Mult_get(&self) -> Result<f32, ClockError> {
        match self.ETH2Mult {
            ETH2MultConf::DIVP4 => return self.DIVP4_get(),
            ETH2MultConf::DIVQ3 => return self.DIVQ3_get(),
        };
    }
    pub fn ETH2output_get(&self) -> Result<f32, ClockError> {
        let input = self.ETH2Mult_get()?;
        if input > (125000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 125000000),
                from: ClockNodes::ETH2Mult,
                to: ClockNodes::ETH2output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::ETH2Mult,
                to: ClockNodes::ETH2output,
            });
        }
        Ok(input)
    }
    fn ADC2Mult_get(&self) -> Result<f32, ClockError> {
        match self.ADC2Mult {
            ADC2MultConf::DIVR4 => return self.DIVR4_get(),
            ADC2MultConf::CKPERCLKOutput => return self.CKPERCLKOutput_get(),
            ADC2MultConf::DIVQ3 => return self.DIVQ3_get(),
        };
    }
    pub fn ADC2output_get(&self) -> Result<f32, ClockError> {
        let input = self.ADC2Mult_get()?;
        if input > (133250000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133250000),
                from: ClockNodes::ADC2Mult,
                to: ClockNodes::ADC2output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::ADC2Mult,
                to: ClockNodes::ADC2output,
            });
        }
        Ok(input)
    }
    fn ADC1Mult_get(&self) -> Result<f32, ClockError> {
        match self.ADC1Mult {
            ADC1MultConf::DIVR4 => return self.DIVR4_get(),
            ADC1MultConf::CKPERCLKOutput => return self.CKPERCLKOutput_get(),
            ADC1MultConf::DIVQ3 => return self.DIVQ3_get(),
        };
    }
    pub fn ADC1output_get(&self) -> Result<f32, ClockError> {
        let input = self.ADC1Mult_get()?;
        if input > (133250000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133250000),
                from: ClockNodes::ADC1Mult,
                to: ClockNodes::ADC1output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::ADC1Mult,
                to: ClockNodes::ADC1output,
            });
        }
        Ok(input)
    }
    pub fn DDRPHYC_get(&self) -> Result<f32, ClockError> {
        self.DIVR2_get()
    }
    pub fn PUBL_get(&self) -> Result<f32, ClockError> {
        self.DIVR2_get()
    }
    pub fn DDRC_get(&self) -> Result<f32, ClockError> {
        self.DIVR2_get()
    }
    pub fn DDRPERFM_get(&self) -> Result<f32, ClockError> {
        self.DIVR2_get()
    }
    fn HSEUSBPHYDevisor_get(&self) -> Result<f32, ClockError> {
        let input = self.HSEOSC_get()? as f32;
        let value = self.HSEUSBPHYDevisor.get()? as f32;
        Ok((input / value) as f32)
    }

    fn USBPHYCLKMux_get(&self) -> Result<f32, ClockError> {
        match self.USBPHYCLKMux {
            USBPHYCLKMuxConf::HSEUSBPHYDevisor => return self.HSEUSBPHYDevisor_get(),
            USBPHYCLKMuxConf::HSEOSC => return self.HSEOSC_get(),
            USBPHYCLKMuxConf::DIVR4 => return self.DIVR4_get(),
        };
    }
    pub fn USBPHYCLKOutput_get(&self) -> Result<f32, ClockError> {
        self.USBPHYCLKMux_get()
    }
    pub fn USBPHYRC_get(&self) -> Result<f32, ClockError> {
        Ok(48000000 as f32)
    }
    fn USBOCLKMux_get(&self) -> Result<f32, ClockError> {
        match self.USBOCLKMux {
            USBOCLKMuxConf::DIVR4 => return self.DIVR4_get(),
            USBOCLKMuxConf::USBPHYRC => return self.USBPHYRC_get(),
        };
    }
    pub fn USBOFSCLKOutput_get(&self) -> Result<f32, ClockError> {
        self.USBOCLKMux_get()
    }
}
