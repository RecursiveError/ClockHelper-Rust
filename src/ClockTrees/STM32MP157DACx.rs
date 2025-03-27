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
    MCO1Mult,
    MCO1Div,
    MCO1Pin,
    MCO2Mult,
    MCO2Div,
    MCO2Pin,
    MCUDIV,
    CortexPrescaler,
    CortexSysOutput,
    McuClockOutput,
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
    PLL4DSIInput,
    DIVQ4,
    DIVQ4output,
    DIVR4,
    DIVR4output,
    DCMI,
    DSIPHYPrescaler,
    DSIMult,
    DSIoutput,
    DSITXPrescaler,
    DSITXCLKEsc,
    DSIPixel,
    HSERTCDevisor,
    RTCClkSource,
    RTCOutput,
    IWDGOutput,
    I2C12Mult,
    I2C12output,
    I2C35Mult,
    I2C35output,
    I2C46Mult,
    I2C46output,
    SPDIFMult,
    SPDIFoutput,
    QSPIMult,
    QSPIoutput,
    FMCMult,
    FMCoutput,
    SDMMC12Mult,
    SDMMC12output,
    SDMMC3Mult,
    SDMMC3output,
    STGENMult,
    STGENoutput,
    LPTIM45Mult,
    LPTIM45output,
    LPTIM23Mult,
    LPTIM23output,
    LPTIM1Mult,
    LPTIM1output,
    USART1Mult,
    USART1output,
    USART24Mult,
    USART24output,
    USART35Mult,
    USART35output,
    USART6Mult,
    USART6output,
    UART78Mult,
    USART78output,
    RNG1Mult,
    RNG1output,
    RNG2Mult,
    RNG2output,
    SPI6Mult,
    SPI6output,
    SPI45Mult,
    SPI45output,
    SAI2Mult,
    SAI2output,
    SAI4Mult,
    SAI4output,
    SPI1Mult,
    SPI1output,
    SPI23Mult,
    SPI23output,
    SAI1Mult,
    SAI1output,
    DFSDF1Audiooutput,
    SAI3Mult,
    SAI3output,
    FDCANMult,
    FDCANoutput,
    ETH1Mult,
    ETH1output,
    ADCMult,
    ADCoutput,
    CSICECDevisor,
    CECMult,
    CECoutput,
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
    PLLDSIIDF,
    PLLDSIMultiplicator,
    PLLDSINDIV,
    VCOoutput,
    PLLDSIDevisor,
    PLLDSIODF,
    PLLDSIoutput,
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
    MPUMult,
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
pub enum MCUDIVConf {
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

impl MCUDIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MCUDIVConf::DIV1 => return Ok(1.0),
            MCUDIVConf::DIV2 => return Ok(2.0),
            MCUDIVConf::DIV4 => return Ok(4.0),
            MCUDIVConf::DIV8 => return Ok(8.0),
            MCUDIVConf::DIV16 => return Ok(16.0),
            MCUDIVConf::DIV32 => return Ok(32.0),
            MCUDIVConf::DIV64 => return Ok(64.0),
            MCUDIVConf::DIV128 => return Ok(128.0),
            MCUDIVConf::DIV256 => return Ok(256.0),
            MCUDIVConf::DIV512 => return Ok(512.0),
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
        4
    }

    pub const fn max() -> u32 {
        512
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
pub enum DSIMultConf {
    PLL4DSIInput,
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
pub enum I2C35MultConf {
    APB1DIV,
    DIVR4,
    HSIDiv,
    CSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C46MultConf {
    APB5DIV,
    DIVQ3,
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
pub enum SDMMC12MultConf {
    Hclk6Output,
    DIVR3,
    DIVP4,
    HSIDiv,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SDMMC3MultConf {
    AHBOutput,
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
pub enum LPTIM23MultConf {
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
    APB5DIV,
    DIVQ4,
    DIVQ3,
    HSEOSC,
    CSIRC,
    HSIDiv,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART24MultConf {
    APB1DIV,
    DIVQ4,
    HSEOSC,
    CSIRC,
    HSIDiv,
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
    LSEOSC,
    LSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum RNG2MultConf {
    CSIRC,
    DIVR4,
    LSEOSC,
    LSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI6MultConf {
    APB5DIV,
    DIVQ4,
    DIVQ3,
    HSIDiv,
    CSIRC,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI45MultConf {
    APB1DIV,
    DIVQ4,
    HSIDiv,
    CSIRC,
    HSEOSC,
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
pub enum SAI4MultConf {
    DIVQ4,
    DIVQ3,
    I2S_CKIN,
    CKPERCLKOutput,
    DIVR3,
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
pub enum SAI3MultConf {
    DIVQ4,
    DIVQ3,
    I2S_CKIN,
    CKPERCLKOutput,
    DIVR3,
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
pub enum ADCMultConf {
    DIVR4,
    CKPERCLKOutput,
    DIVQ3,
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CECMultConf {
    CSICECDevisor,
    LSEOSC,
    LSIRC,
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
    pub MCO1Mult: MCO1MultConf,
    pub MCO1Div: MCO1DivConf,
    pub MCO2Mult: MCO2MultConf,
    pub MCO2Div: MCO2DivConf,
    pub MCUDIV: MCUDIVConf,
    pub CortexPrescaler: CortexPrescalerConf,
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
    pub DSIMult: DSIMultConf,
    pub DSITXPrescaler: DSITXPrescalerConf,
    pub HSERTCDevisor: HSERTCDevisorConf,
    pub RTCClkSource: RTCClkSourceConf,
    pub I2C12Mult: I2C12MultConf,
    pub I2C35Mult: I2C35MultConf,
    pub I2C46Mult: I2C46MultConf,
    pub SPDIFMult: SPDIFMultConf,
    pub QSPIMult: QSPIMultConf,
    pub FMCMult: FMCMultConf,
    pub SDMMC12Mult: SDMMC12MultConf,
    pub SDMMC3Mult: SDMMC3MultConf,
    pub STGENMult: STGENMultConf,
    pub LPTIM45Mult: LPTIM45MultConf,
    pub LPTIM23Mult: LPTIM23MultConf,
    pub LPTIM1Mult: LPTIM1MultConf,
    pub USART1Mult: USART1MultConf,
    pub USART24Mult: USART24MultConf,
    pub USART35Mult: USART35MultConf,
    pub USART6Mult: USART6MultConf,
    pub UART78Mult: UART78MultConf,
    pub RNG1Mult: RNG1MultConf,
    pub RNG2Mult: RNG2MultConf,
    pub SPI6Mult: SPI6MultConf,
    pub SPI45Mult: SPI45MultConf,
    pub SAI2Mult: SAI2MultConf,
    pub SAI4Mult: SAI4MultConf,
    pub SPI1Mult: SPI1MultConf,
    pub SPI23Mult: SPI23MultConf,
    pub SAI1Mult: SAI1MultConf,
    pub SAI3Mult: SAI3MultConf,
    pub FDCANMult: FDCANMultConf,
    pub ETH1Mult: ETH1MultConf,
    pub ADCMult: ADCMultConf,
    pub CSICECDevisor: CSICECDevisorConf,
    pub CECMult: CECMultConf,
    pub HSEUSBPHYDevisor: HSEUSBPHYDevisorConf,
    pub USBPHYCLKMux: USBPHYCLKMuxConf,
    pub USBOCLKMux: USBOCLKMuxConf,
    pub PLLDSIIDF: PLLDSIIDFConf,
    pub PLLDSINDIV: PLLDSINDIVConf,
    pub PLLDSIODF: PLLDSIODFConf,
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
            MCO1Mult: MCO1MultConf::HSIDiv,
            MCO1Div: MCO1DivConf::DIV1,
            MCO2Mult: MCO2MultConf::MPUMult,
            MCO2Div: MCO2DivConf::DIV1,
            MCUDIV: MCUDIVConf::DIV1,
            CortexPrescaler: CortexPrescalerConf::DIV1,
            APB3DIV: APB3DIVConf::DIV1,
            APB1DIV: APB1DIVConf::DIV1,
            APB2DIV: APB2DIVConf::DIV1,
            PLL12Source: PLL12SourceConf::HSIDiv,
            DIVM1: DIVM1Conf::Value(32),
            DIVM2: DIVM2Conf::Value(32),
            PLL3Source: PLL3SourceConf::HSIDiv,
            DIVM3: DIVM3Conf::Value(32),
            PLL4Source: PLL4SourceConf::HSIDiv,
            DIVM4: DIVM4Conf::Value(32),
            MPUDIV: MPUDIVConf::DIV2,
            DIVN1: DIVN1Conf::Value(25),
            PLL1FRACV: PLL1FRACVConf::Value(0),
            DIVP1: DIVP1Conf::Value(1),
            DIVQ1: DIVQ1Conf::Value(2),
            DIVR1: DIVR1Conf::Value(2),
            DIVN2: DIVN2Conf::Value(25),
            PLL2FRACV: PLL2FRACVConf::Value(0),
            DIVP2: DIVP2Conf::Value(2),
            DIVQ2: DIVQ2Conf::Value(2),
            DIVR2: DIVR2Conf::Value(2),
            DIVN3: DIVN3Conf::Value(25),
            PLL3FRACV: PLL3FRACVConf::Value(0),
            DIVP3: DIVP3Conf::Value(2),
            DIVQ3: DIVQ3Conf::Value(2),
            DIVR3: DIVR3Conf::Value(2),
            DIVN4: DIVN4Conf::Value(25),
            PLL4FRACV: PLL4FRACVConf::Value(0),
            DIVP4: DIVP4Conf::Value(2),
            DIVQ4: DIVQ4Conf::Value(2),
            DIVR4: DIVR4Conf::Value(2),
            DSIMult: DSIMultConf::DSIPHYPrescaler,
            DSITXPrescaler: DSITXPrescalerConf::Value(4),
            HSERTCDevisor: HSERTCDevisorConf::Value(1),
            RTCClkSource: RTCClkSourceConf::LSIRC,
            I2C12Mult: I2C12MultConf::APB1DIV,
            I2C35Mult: I2C35MultConf::APB1DIV,
            I2C46Mult: I2C46MultConf::APB5DIV,
            SPDIFMult: SPDIFMultConf::DIVP4,
            QSPIMult: QSPIMultConf::AXIOutput,
            FMCMult: FMCMultConf::AXIOutput,
            SDMMC12Mult: SDMMC12MultConf::Hclk6Output,
            SDMMC3Mult: SDMMC3MultConf::AHBOutput,
            STGENMult: STGENMultConf::HSIDiv,
            LPTIM45Mult: LPTIM45MultConf::APB3DIV,
            LPTIM23Mult: LPTIM23MultConf::APB3DIV,
            LPTIM1Mult: LPTIM1MultConf::APB1DIV,
            USART1Mult: USART1MultConf::APB5DIV,
            USART24Mult: USART24MultConf::APB1DIV,
            USART35Mult: USART35MultConf::APB1DIV,
            USART6Mult: USART6MultConf::APB2DIV,
            UART78Mult: UART78MultConf::APB1DIV,
            RNG1Mult: RNG1MultConf::CSIRC,
            RNG2Mult: RNG2MultConf::CSIRC,
            SPI6Mult: SPI6MultConf::APB5DIV,
            SPI45Mult: SPI45MultConf::APB1DIV,
            SAI2Mult: SAI2MultConf::DIVQ4,
            SAI4Mult: SAI4MultConf::DIVQ4,
            SPI1Mult: SPI1MultConf::DIVP4,
            SPI23Mult: SPI23MultConf::DIVP4,
            SAI1Mult: SAI1MultConf::DIVQ4,
            SAI3Mult: SAI3MultConf::DIVQ4,
            FDCANMult: FDCANMultConf::HSEOSC,
            ETH1Mult: ETH1MultConf::DIVP4,
            ADCMult: ADCMultConf::DIVR4,
            CSICECDevisor: CSICECDevisorConf::DIV122,
            CECMult: CECMultConf::LSEOSC,
            HSEUSBPHYDevisor: HSEUSBPHYDevisorConf::DIV2,
            USBPHYCLKMux: USBPHYCLKMuxConf::DIVR4,
            USBOCLKMux: USBOCLKMuxConf::DIVR4,
            PLLDSIIDF: PLLDSIIDFConf::DIV1,
            PLLDSINDIV: PLLDSINDIVConf::Value(20),
            PLLDSIODF: PLLDSIODFConf::DIV1,
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
            MCO2MultConf::MPUMult => return self.MPUMult_get(),
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
    fn MCUDIV_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()? as f32;
        let value = self.MCUDIV.get()? as f32;
        Ok((input / value) as f32)
    }

    fn CortexPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.MCUDIV_get()? as f32;
        let value = self.CortexPrescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn CortexSysOutput_get(&self) -> Result<f32, ClockError> {
        self.CortexPrescaler_get()
    }
    pub fn McuClockOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.MCUDIV_get()?;
        if input > (209000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 209000000),
                from: ClockNodes::MCUDIV,
                to: ClockNodes::McuClockOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::MCUDIV,
                to: ClockNodes::McuClockOutput,
            });
        }
        Ok(input)
    }
    fn APB3DIV_get(&self) -> Result<f32, ClockError> {
        let input = self.MCUDIV_get()? as f32;
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
        let input = self.MCUDIV_get()? as f32;
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
        let input = self.MCUDIV_get()?;
        if input > (209000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 209000000),
                from: ClockNodes::MCUDIV,
                to: ClockNodes::AHBOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::MCUDIV,
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
        let input = self.MCUDIV_get()? as f32;
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
        self.MCUDIV_get()
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
        if input > (533000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 533000000),
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
    pub fn PLL4DSIInput_get(&self) -> Result<f32, ClockError> {
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
    fn DSIPHYPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLDSIODF_get()? as f32;
        let value = 8 as f32;
        Ok((input / value) as f32)
    }

    fn DSIMult_get(&self) -> Result<f32, ClockError> {
        match self.DSIMult {
            DSIMultConf::PLL4DSIInput => return self.PLL4DSIInput_get(),
            DSIMultConf::DSIPHYPrescaler => return self.DSIPHYPrescaler_get(),
        };
    }
    pub fn DSIoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.DSIMult_get()?;
        if input > (125000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 125000000),
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
    pub fn DSIPixel_get(&self) -> Result<f32, ClockError> {
        let input = self.DIVQ4_get()?;
        if input > (90000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 90000000),
                from: ClockNodes::DIVQ4,
                to: ClockNodes::DSIPixel,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::DIVQ4,
                to: ClockNodes::DSIPixel,
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
    fn I2C35Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C35Mult {
            I2C35MultConf::APB1DIV => return self.APB1DIV_get(),
            I2C35MultConf::DIVR4 => return self.DIVR4_get(),
            I2C35MultConf::HSIDiv => return self.HSIDiv_get(),
            I2C35MultConf::CSIRC => return self.CSIRC_get(),
        };
    }
    pub fn I2C35output_get(&self) -> Result<f32, ClockError> {
        let input = self.I2C35Mult_get()?;
        if input > (104500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 104500000),
                from: ClockNodes::I2C35Mult,
                to: ClockNodes::I2C35output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::I2C35Mult,
                to: ClockNodes::I2C35output,
            });
        }
        Ok(input)
    }
    fn I2C46Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C46Mult {
            I2C46MultConf::APB5DIV => return self.APB5DIV_get(),
            I2C46MultConf::DIVQ3 => return self.DIVQ3_get(),
            I2C46MultConf::HSIDiv => return self.HSIDiv_get(),
            I2C46MultConf::CSIRC => return self.CSIRC_get(),
        };
    }
    pub fn I2C46output_get(&self) -> Result<f32, ClockError> {
        let input = self.I2C46Mult_get()?;
        if input > (133250000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133250000),
                from: ClockNodes::I2C46Mult,
                to: ClockNodes::I2C46output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::I2C46Mult,
                to: ClockNodes::I2C46output,
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
    fn SDMMC12Mult_get(&self) -> Result<f32, ClockError> {
        match self.SDMMC12Mult {
            SDMMC12MultConf::Hclk6Output => return self.Hclk6Output_get(),
            SDMMC12MultConf::DIVR3 => return self.DIVR3_get(),
            SDMMC12MultConf::DIVP4 => return self.DIVP4_get(),
            SDMMC12MultConf::HSIDiv => return self.HSIDiv_get(),
        };
    }
    pub fn SDMMC12output_get(&self) -> Result<f32, ClockError> {
        let input = self.SDMMC12Mult_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
                from: ClockNodes::SDMMC12Mult,
                to: ClockNodes::SDMMC12output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SDMMC12Mult,
                to: ClockNodes::SDMMC12output,
            });
        }
        Ok(input)
    }
    fn SDMMC3Mult_get(&self) -> Result<f32, ClockError> {
        match self.SDMMC3Mult {
            SDMMC3MultConf::AHBOutput => return self.AHBOutput_get(),
            SDMMC3MultConf::DIVR3 => return self.DIVR3_get(),
            SDMMC3MultConf::DIVP4 => return self.DIVP4_get(),
            SDMMC3MultConf::HSIDiv => return self.HSIDiv_get(),
        };
    }
    pub fn SDMMC3output_get(&self) -> Result<f32, ClockError> {
        let input = self.SDMMC3Mult_get()?;
        if input > (209000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 209000000),
                from: ClockNodes::SDMMC3Mult,
                to: ClockNodes::SDMMC3output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SDMMC3Mult,
                to: ClockNodes::SDMMC3output,
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
    fn LPTIM23Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM23Mult {
            LPTIM23MultConf::APB3DIV => return self.APB3DIV_get(),
            LPTIM23MultConf::DIVQ4 => return self.DIVQ4_get(),
            LPTIM23MultConf::CKPERCLKOutput => return self.CKPERCLKOutput_get(),
            LPTIM23MultConf::LSEOSC => return self.LSEOSC_get(),
            LPTIM23MultConf::LSIRC => return self.LSIRC_get(),
        };
    }
    pub fn LPTIM23output_get(&self) -> Result<f32, ClockError> {
        let input = self.LPTIM23Mult_get()?;
        if input > (104500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 104500000),
                from: ClockNodes::LPTIM23Mult,
                to: ClockNodes::LPTIM23output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::LPTIM23Mult,
                to: ClockNodes::LPTIM23output,
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
            USART1MultConf::APB5DIV => return self.APB5DIV_get(),
            USART1MultConf::DIVQ4 => return self.DIVQ4_get(),
            USART1MultConf::DIVQ3 => return self.DIVQ3_get(),
            USART1MultConf::HSEOSC => return self.HSEOSC_get(),
            USART1MultConf::CSIRC => return self.CSIRC_get(),
            USART1MultConf::HSIDiv => return self.HSIDiv_get(),
        };
    }
    pub fn USART1output_get(&self) -> Result<f32, ClockError> {
        let input = self.USART1Mult_get()?;
        if input > (133250000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133250000),
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
    fn USART24Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART24Mult {
            USART24MultConf::APB1DIV => return self.APB1DIV_get(),
            USART24MultConf::DIVQ4 => return self.DIVQ4_get(),
            USART24MultConf::HSEOSC => return self.HSEOSC_get(),
            USART24MultConf::CSIRC => return self.CSIRC_get(),
            USART24MultConf::HSIDiv => return self.HSIDiv_get(),
        };
    }
    pub fn USART24output_get(&self) -> Result<f32, ClockError> {
        let input = self.USART24Mult_get()?;
        if input > (104500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 104500000),
                from: ClockNodes::USART24Mult,
                to: ClockNodes::USART24output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::USART24Mult,
                to: ClockNodes::USART24output,
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
            RNG1MultConf::LSEOSC => return self.LSEOSC_get(),
            RNG1MultConf::LSIRC => return self.LSIRC_get(),
        };
    }
    pub fn RNG1output_get(&self) -> Result<f32, ClockError> {
        let input = self.RNG1Mult_get()?;
        if input > (48000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48000000),
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
    fn RNG2Mult_get(&self) -> Result<f32, ClockError> {
        match self.RNG2Mult {
            RNG2MultConf::CSIRC => return self.CSIRC_get(),
            RNG2MultConf::DIVR4 => return self.DIVR4_get(),
            RNG2MultConf::LSEOSC => return self.LSEOSC_get(),
            RNG2MultConf::LSIRC => return self.LSIRC_get(),
        };
    }
    pub fn RNG2output_get(&self) -> Result<f32, ClockError> {
        let input = self.RNG2Mult_get()?;
        if input > (48000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48000000),
                from: ClockNodes::RNG2Mult,
                to: ClockNodes::RNG2output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::RNG2Mult,
                to: ClockNodes::RNG2output,
            });
        }
        Ok(input)
    }
    fn SPI6Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI6Mult {
            SPI6MultConf::APB5DIV => return self.APB5DIV_get(),
            SPI6MultConf::DIVQ4 => return self.DIVQ4_get(),
            SPI6MultConf::DIVQ3 => return self.DIVQ3_get(),
            SPI6MultConf::HSIDiv => return self.HSIDiv_get(),
            SPI6MultConf::CSIRC => return self.CSIRC_get(),
            SPI6MultConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    pub fn SPI6output_get(&self) -> Result<f32, ClockError> {
        let input = self.SPI6Mult_get()?;
        if input > (133250000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133250000),
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
            SPI45MultConf::APB1DIV => return self.APB1DIV_get(),
            SPI45MultConf::DIVQ4 => return self.DIVQ4_get(),
            SPI45MultConf::HSIDiv => return self.HSIDiv_get(),
            SPI45MultConf::CSIRC => return self.CSIRC_get(),
            SPI45MultConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    pub fn SPI45output_get(&self) -> Result<f32, ClockError> {
        let input = self.SPI45Mult_get()?;
        if input > (133250000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133250000),
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
    fn SAI4Mult_get(&self) -> Result<f32, ClockError> {
        match self.SAI4Mult {
            SAI4MultConf::DIVQ4 => return self.DIVQ4_get(),
            SAI4MultConf::DIVQ3 => return self.DIVQ3_get(),
            SAI4MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            SAI4MultConf::CKPERCLKOutput => return self.CKPERCLKOutput_get(),
            SAI4MultConf::DIVR3 => return self.DIVR3_get(),
        };
    }
    pub fn SAI4output_get(&self) -> Result<f32, ClockError> {
        let input = self.SAI4Mult_get()?;
        if input > (133250000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133250000),
                from: ClockNodes::SAI4Mult,
                to: ClockNodes::SAI4output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SAI4Mult,
                to: ClockNodes::SAI4output,
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
        self.SAI1Mult_get()
    }
    fn SAI3Mult_get(&self) -> Result<f32, ClockError> {
        match self.SAI3Mult {
            SAI3MultConf::DIVQ4 => return self.DIVQ4_get(),
            SAI3MultConf::DIVQ3 => return self.DIVQ3_get(),
            SAI3MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
            SAI3MultConf::CKPERCLKOutput => return self.CKPERCLKOutput_get(),
            SAI3MultConf::DIVR3 => return self.DIVR3_get(),
        };
    }
    pub fn SAI3output_get(&self) -> Result<f32, ClockError> {
        let input = self.SAI3Mult_get()?;
        if input > (133250000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133250000),
                from: ClockNodes::SAI3Mult,
                to: ClockNodes::SAI3output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SAI3Mult,
                to: ClockNodes::SAI3output,
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
    fn ADCMult_get(&self) -> Result<f32, ClockError> {
        match self.ADCMult {
            ADCMultConf::DIVR4 => return self.DIVR4_get(),
            ADCMultConf::CKPERCLKOutput => return self.CKPERCLKOutput_get(),
            ADCMultConf::DIVQ3 => return self.DIVQ3_get(),
        };
    }
    pub fn ADCoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.ADCMult_get()?;
        if input > (133250000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133250000),
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
    fn CSICECDevisor_get(&self) -> Result<f32, ClockError> {
        let input = self.CSIRC_get()? as f32;
        let value = self.CSICECDevisor.get()? as f32;
        Ok((input / value) as f32)
    }

    fn CECMult_get(&self) -> Result<f32, ClockError> {
        match self.CECMult {
            CECMultConf::CSICECDevisor => return self.CSICECDevisor_get(),
            CECMultConf::LSEOSC => return self.LSEOSC_get(),
            CECMultConf::LSIRC => return self.LSIRC_get(),
        };
    }
    pub fn CECoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.CECMult_get()?;
        if input > (1000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1000000),
                from: ClockNodes::CECMult,
                to: ClockNodes::CECoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::CECMult,
                to: ClockNodes::CECoutput,
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
        let input = self.USBOCLKMux_get()?;
        if input > (50000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 50000000),
                from: ClockNodes::USBOCLKMux,
                to: ClockNodes::USBOFSCLKOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::USBOCLKMux,
                to: ClockNodes::USBOFSCLKOutput,
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
        if input > (2000000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 2000000000),
                from: ClockNodes::PLLDSINDIV,
                to: ClockNodes::VCOoutput,
            });
        } else if input < (1000000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 1000000000),
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
        if input > (1000000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1000000000),
                from: ClockNodes::PLLDSIODF,
                to: ClockNodes::PLLDSIoutput,
            });
        } else if input < (62500000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 62500000),
                from: ClockNodes::PLLDSIODF,
                to: ClockNodes::PLLDSIoutput,
            });
        }
        Ok(input)
    }
}
