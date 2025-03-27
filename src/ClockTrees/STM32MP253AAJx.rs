#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ClockNodes {
    None,
    HSIRC,
    HSEOSC,
    HSEDIV2,
    SPDIF,
    LSIRC,
    LSEOSC,
    MSIRC,
    I2S_CKIN,
    XBAR0,
    XBAR0Prediv,
    XBAR0Findiv,
    XBAR0Output,
    XBAR1,
    XBAR1Prediv,
    XBAR1Findiv,
    XBAR1Output,
    XBAR2,
    XBAR2Prediv,
    XBAR2Findiv,
    XBAR2Output,
    XBAR3,
    XBAR3Prediv,
    XBAR3Findiv,
    XBAR3Output,
    XBAR4,
    XBAR4Prediv,
    XBAR4Findiv,
    XBAR4Output,
    XBAR5,
    XBAR5Prediv,
    XBAR5Findiv,
    XBAR5Output,
    XBAR6,
    XBAR6Prediv,
    XBAR6Findiv,
    XBAR6Output,
    XBAR7,
    XBAR7Prediv,
    XBAR7Findiv,
    XBAR7Output,
    XBAR8,
    XBAR8Prediv,
    XBAR8Findiv,
    XBAR8Output,
    XBAR9,
    XBAR9Prediv,
    XBAR9Findiv,
    XBAR9Output,
    XBAR10,
    XBAR10Prediv,
    XBAR10Findiv,
    XBAR10Output,
    XBAR11,
    XBAR11Prediv,
    XBAR11Findiv,
    XBAR11Output,
    XBAR12,
    XBAR12Prediv,
    XBAR12Findiv,
    XBAR12Output,
    XBAR13,
    XBAR13Prediv,
    XBAR13Findiv,
    XBAR13Output,
    XBAR14,
    XBAR14Prediv,
    XBAR14Findiv,
    XBAR14Output,
    XBAR15,
    XBAR15Prediv,
    XBAR15Findiv,
    XBAR15Output,
    XBAR16,
    XBAR16Prediv,
    XBAR16Findiv,
    XBAR16Output,
    XBAR17,
    XBAR17Prediv,
    XBAR17Findiv,
    XBAR17Output,
    XBAR18,
    XBAR18Prediv,
    XBAR18Findiv,
    XBAR18Output,
    XBAR19,
    XBAR19Prediv,
    XBAR19Findiv,
    XBAR19Output,
    XBAR20,
    XBAR20Prediv,
    XBAR20Findiv,
    XBAR20Output,
    XBAR21,
    XBAR21Prediv,
    XBAR21Findiv,
    XBAR21Output,
    XBAR22,
    XBAR22Prediv,
    XBAR22Findiv,
    XBAR22Output,
    XBAR23,
    XBAR23Prediv,
    XBAR23Findiv,
    XBAR23Output,
    XBAR24,
    XBAR24Prediv,
    XBAR24Findiv,
    XBAR24Output,
    XBAR25,
    XBAR25Prediv,
    XBAR25Findiv,
    XBAR25Output,
    XBAR26,
    XBAR26Prediv,
    XBAR26Findiv,
    XBAR26Output,
    XBAR27,
    XBAR27Prediv,
    XBAR27Findiv,
    XBAR27Output,
    XBAR28,
    XBAR28Prediv,
    XBAR28Findiv,
    XBAR28Output,
    XBAR29,
    XBAR29Prediv,
    XBAR29Findiv,
    XBAR29Output,
    XBAR30,
    XBAR30Prediv,
    XBAR30Findiv,
    XBAR30Output,
    XBAR31,
    XBAR31Prediv,
    XBAR31Findiv,
    XBAR31Output,
    XBAR32,
    XBAR32Prediv,
    XBAR32Findiv,
    XBAR32Output,
    XBAR33,
    XBAR33Prediv,
    XBAR33Findiv,
    XBAR33Output,
    XBAR34,
    XBAR34Prediv,
    XBAR34Findiv,
    XBAR34Output,
    XBAR35,
    XBAR35Prediv,
    XBAR35Findiv,
    XBAR35Output,
    XBAR36,
    XBAR36Prediv,
    XBAR36Findiv,
    XBAR36Output,
    XBAR37,
    XBAR37Prediv,
    XBAR37Findiv,
    XBAR37Output,
    XBAR38,
    XBAR38Prediv,
    XBAR38Findiv,
    XBAR38Output,
    XBAR39,
    XBAR39Prediv,
    XBAR39Findiv,
    XBAR39Output,
    XBAR40,
    XBAR40Prediv,
    XBAR40Findiv,
    XBAR40Output,
    XBAR41,
    XBAR41Prediv,
    XBAR41Findiv,
    XBAR41Output,
    XBAR42,
    XBAR42Prediv,
    XBAR42Findiv,
    XBAR42Output,
    XBAR43,
    XBAR43Prediv,
    XBAR43Findiv,
    XBAR43Output,
    XBAR44,
    XBAR44Prediv,
    XBAR44Findiv,
    XBAR44Output,
    XBAR45,
    XBAR45Prediv,
    XBAR45Findiv,
    XBAR45Output,
    XBAR46,
    XBAR46Prediv,
    XBAR46Findiv,
    XBAR46Output,
    XBAR47,
    XBAR47Prediv,
    XBAR47Findiv,
    XBAR47Output,
    XBAR48,
    XBAR48Prediv,
    XBAR48Findiv,
    XBAR48Output,
    XBAR49,
    XBAR49Prediv,
    XBAR49Findiv,
    XBAR49Output,
    XBAR50,
    XBAR50Prediv,
    XBAR50Findiv,
    XBAR50Output,
    XBAR51,
    XBAR51Prediv,
    XBAR51Findiv,
    XBAR51Output,
    XBAR52,
    XBAR52Prediv,
    XBAR52Findiv,
    XBAR52Output,
    XBAR53,
    XBAR53Prediv,
    XBAR53Findiv,
    XBAR53Output,
    XBAR54,
    XBAR54Prediv,
    XBAR54Findiv,
    XBAR54Output,
    XBAR55,
    XBAR55Prediv,
    XBAR55Findiv,
    XBAR55Output,
    XBAR56,
    XBAR56Prediv,
    XBAR56Findiv,
    XBAR56Output,
    XBAR57,
    XBAR57Prediv,
    XBAR57Findiv,
    XBAR57Output,
    XBAR58,
    XBAR58Prediv,
    XBAR58Findiv,
    XBAR58Output,
    XBAR59,
    XBAR59Prediv,
    XBAR59Findiv,
    XBAR59Output,
    XBAR60,
    XBAR60Prediv,
    XBAR60Findiv,
    XBAR60Output,
    XBAR61,
    XBAR61Prediv,
    XBAR61Findiv,
    XBAR61Output,
    XBAR62,
    XBAR62Prediv,
    XBAR62Findiv,
    XBAR62Output,
    XBAR63,
    XBAR63Prediv,
    XBAR63Findiv,
    XBAR63Output,
    CKINTSEL0,
    CKEXTSEL0,
    CKINTSEL1,
    CKEXTSEL1,
    OBS0,
    OBS0Output,
    OBS1,
    OBS1Output,
    MCO1Mult,
    MCO1Pin,
    MCO2Mult,
    MCO2Pin,
    D3PER,
    D3PEROutput,
    DTS,
    DTSOutput,
    DSIPHY,
    DSIPHYOutput,
    DSIBLANE,
    DSIBLANEOutput,
    USB2PHY1,
    USB2PHY1Output,
    USB2PHY2,
    USB2PHY2Output,
    USB3PCIPHY,
    USB3PCIPHYOutput,
    SysClkSource,
    SysCLKOutput,
    MCUDIV,
    McuClockOutput,
    APB3DIV,
    APB3Output,
    APB4DIV,
    APB4Output,
    APBDBGDIV,
    APBDBGOutput,
    APB1DIV,
    Tim1Mul,
    Tim1Output,
    AHBOutput,
    APB1Output,
    APB2DIV,
    Tim2Mul,
    Tim2Output,
    APB2Output,
    ADC12Mult,
    ADC12output,
    ADC3Mult,
    ADC3output,
    PLL1Source,
    FREFDIV1,
    PLL2Source,
    FREFDIV2,
    PLL3Source,
    FREFDIV3,
    PLL4Source,
    FREFDIV4,
    FBDIV1,
    POSTDIV1_1,
    POSTDIV2_1,
    FOUTPOSTDIV1,
    PLL1Div42,
    FBDIV2,
    PLL2FRACV,
    POSTDIV1_2,
    POSTDIV2_2,
    FOUTPOSTDIV2,
    PLL2Div4,
    FBDIV3,
    PLL3FRACV,
    POSTDIV1_3,
    POSTDIV2_3,
    FOUTPOSTDIV3,
    PLL3Div2,
    FBDIV4,
    PLL4FRACV,
    POSTDIV1_4,
    POSTDIV2_4,
    FOUTPOSTDIV4,
    PLL5Source,
    FREFDIV5,
    FBDIV5,
    PLL5FRACV,
    POSTDIV1_5,
    POSTDIV2_5,
    FOUTPOSTDIV5,
    PLL6Source,
    FREFDIV6,
    FBDIV6,
    PLL6FRACV,
    POSTDIV1_6,
    POSTDIV2_6,
    FOUTPOSTDIV6,
    PLL7Source,
    FREFDIV7,
    FBDIV7,
    PLL7FRACV,
    POSTDIV1_7,
    POSTDIV2_7,
    FOUTPOSTDIV7,
    PLL8Source,
    FREFDIV8,
    FBDIV8,
    PLL8FRACV,
    POSTDIV1_8,
    POSTDIV2_8,
    FOUTPOSTDIV8,
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
pub enum HSEOSCConf {
    Value(u32),
}

impl HSEOSCConf {
    pub const fn min() -> u32 {
        16000000
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
pub enum HSEDIV2Conf {
    DIV2,
}

impl HSEDIV2Conf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            HSEDIV2Conf::DIV2 => return Ok(2.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPDIFConf {
    Value(u32),
}

impl SPDIFConf {
    pub const fn min() -> u32 {
        3072000
    }

    pub const fn max() -> u32 {
        12288000
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            SPDIFConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::SPDIF,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::SPDIF,
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
pub enum XBAR0Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
    SPDIF,
    I2S_CKIN,
    LSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR0PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR0PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR0PredivConf::DIV1 => return Ok(1.0),
            XBAR0PredivConf::DIV2 => return Ok(2.0),
            XBAR0PredivConf::DIV4 => return Ok(4.0),
            XBAR0PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR0FindivConf {
    Value(u32),
}

impl XBAR0FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR0FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR0Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR0Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR1Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
    SPDIF,
    I2S_CKIN,
    LSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR1PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR1PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR1PredivConf::DIV1 => return Ok(1.0),
            XBAR1PredivConf::DIV2 => return Ok(2.0),
            XBAR1PredivConf::DIV4 => return Ok(4.0),
            XBAR1PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR1FindivConf {
    Value(u32),
}

impl XBAR1FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR1FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR1Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR1Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR2Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
    SPDIF,
    I2S_CKIN,
    LSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR2PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR2PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR2PredivConf::DIV1 => return Ok(1.0),
            XBAR2PredivConf::DIV2 => return Ok(2.0),
            XBAR2PredivConf::DIV4 => return Ok(4.0),
            XBAR2PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR2FindivConf {
    Value(u32),
}

impl XBAR2FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR2FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR2Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR2Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR3Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
    SPDIF,
    I2S_CKIN,
    LSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR3PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR3PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR3PredivConf::DIV1 => return Ok(1.0),
            XBAR3PredivConf::DIV2 => return Ok(2.0),
            XBAR3PredivConf::DIV4 => return Ok(4.0),
            XBAR3PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR3FindivConf {
    Value(u32),
}

impl XBAR3FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR3FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR3Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR3Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR4Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
    SPDIF,
    I2S_CKIN,
    LSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR4PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR4PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR4PredivConf::DIV1 => return Ok(1.0),
            XBAR4PredivConf::DIV2 => return Ok(2.0),
            XBAR4PredivConf::DIV4 => return Ok(4.0),
            XBAR4PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR4FindivConf {
    Value(u32),
}

impl XBAR4FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR4FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR4Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR4Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR5Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
    SPDIF,
    I2S_CKIN,
    LSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR5PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR5PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR5PredivConf::DIV1 => return Ok(1.0),
            XBAR5PredivConf::DIV2 => return Ok(2.0),
            XBAR5PredivConf::DIV4 => return Ok(4.0),
            XBAR5PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR5FindivConf {
    Value(u32),
}

impl XBAR5FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR5FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR5Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR5Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR6Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
    SPDIF,
    I2S_CKIN,
    LSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR6PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR6PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR6PredivConf::DIV1 => return Ok(1.0),
            XBAR6PredivConf::DIV2 => return Ok(2.0),
            XBAR6PredivConf::DIV4 => return Ok(4.0),
            XBAR6PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR6FindivConf {
    Value(u32),
}

impl XBAR6FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR6FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR6Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR6Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR7Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    LSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR7PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR7PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR7PredivConf::DIV1 => return Ok(1.0),
            XBAR7PredivConf::DIV2 => return Ok(2.0),
            XBAR7PredivConf::DIV4 => return Ok(4.0),
            XBAR7PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR7FindivConf {
    Value(u32),
}

impl XBAR7FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR7FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR7Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR7Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR8Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR8PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR8PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR8PredivConf::DIV1 => return Ok(1.0),
            XBAR8PredivConf::DIV2 => return Ok(2.0),
            XBAR8PredivConf::DIV4 => return Ok(4.0),
            XBAR8PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR8FindivConf {
    Value(u32),
}

impl XBAR8FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR8FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR8Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR8Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR9Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR9PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR9PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR9PredivConf::DIV1 => return Ok(1.0),
            XBAR9PredivConf::DIV2 => return Ok(2.0),
            XBAR9PredivConf::DIV4 => return Ok(4.0),
            XBAR9PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR9FindivConf {
    Value(u32),
}

impl XBAR9FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR9FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR9Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR9Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR10Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
    I2S_CKIN,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR10PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR10PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR10PredivConf::DIV1 => return Ok(1.0),
            XBAR10PredivConf::DIV2 => return Ok(2.0),
            XBAR10PredivConf::DIV4 => return Ok(4.0),
            XBAR10PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR10FindivConf {
    Value(u32),
}

impl XBAR10FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR10FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR10Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR10Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR11Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR11PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR11PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR11PredivConf::DIV1 => return Ok(1.0),
            XBAR11PredivConf::DIV2 => return Ok(2.0),
            XBAR11PredivConf::DIV4 => return Ok(4.0),
            XBAR11PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR11FindivConf {
    Value(u32),
}

impl XBAR11FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR11FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR11Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR11Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR12Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    MSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR12PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR12PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR12PredivConf::DIV1 => return Ok(1.0),
            XBAR12PredivConf::DIV2 => return Ok(2.0),
            XBAR12PredivConf::DIV4 => return Ok(4.0),
            XBAR12PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR12FindivConf {
    Value(u32),
}

impl XBAR12FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR12FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR12Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR12Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR13Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    MSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR13PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR13PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR13PredivConf::DIV1 => return Ok(1.0),
            XBAR13PredivConf::DIV2 => return Ok(2.0),
            XBAR13PredivConf::DIV4 => return Ok(4.0),
            XBAR13PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR13FindivConf {
    Value(u32),
}

impl XBAR13FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR13FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR13Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR13Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR14Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    MSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR14PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR14PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR14PredivConf::DIV1 => return Ok(1.0),
            XBAR14PredivConf::DIV2 => return Ok(2.0),
            XBAR14PredivConf::DIV4 => return Ok(4.0),
            XBAR14PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR14FindivConf {
    Value(u32),
}

impl XBAR14FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR14FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR14Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR14Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR15Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    MSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR15PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR15PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR15PredivConf::DIV1 => return Ok(1.0),
            XBAR15PredivConf::DIV2 => return Ok(2.0),
            XBAR15PredivConf::DIV4 => return Ok(4.0),
            XBAR15PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR15FindivConf {
    Value(u32),
}

impl XBAR15FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR15FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR15Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR15Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR16Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
    I2S_CKIN,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR16PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR16PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR16PredivConf::DIV1 => return Ok(1.0),
            XBAR16PredivConf::DIV2 => return Ok(2.0),
            XBAR16PredivConf::DIV4 => return Ok(4.0),
            XBAR16PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR16FindivConf {
    Value(u32),
}

impl XBAR16FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR16FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR16Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR16Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR17Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
    I2S_CKIN,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR17PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR17PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR17PredivConf::DIV1 => return Ok(1.0),
            XBAR17PredivConf::DIV2 => return Ok(2.0),
            XBAR17PredivConf::DIV4 => return Ok(4.0),
            XBAR17PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR17FindivConf {
    Value(u32),
}

impl XBAR17FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR17FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR17Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR17Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR18Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
    I2S_CKIN,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR18PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR18PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR18PredivConf::DIV1 => return Ok(1.0),
            XBAR18PredivConf::DIV2 => return Ok(2.0),
            XBAR18PredivConf::DIV4 => return Ok(4.0),
            XBAR18PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR18FindivConf {
    Value(u32),
}

impl XBAR18FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR18FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR18Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR18Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR19Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR19PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR19PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR19PredivConf::DIV1 => return Ok(1.0),
            XBAR19PredivConf::DIV2 => return Ok(2.0),
            XBAR19PredivConf::DIV4 => return Ok(4.0),
            XBAR19PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR19FindivConf {
    Value(u32),
}

impl XBAR19FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR19FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR19Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR19Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR20Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR20PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR20PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR20PredivConf::DIV1 => return Ok(1.0),
            XBAR20PredivConf::DIV2 => return Ok(2.0),
            XBAR20PredivConf::DIV4 => return Ok(4.0),
            XBAR20PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR20FindivConf {
    Value(u32),
}

impl XBAR20FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR20FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR20Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR20Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR21Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR21PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR21PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR21PredivConf::DIV1 => return Ok(1.0),
            XBAR21PredivConf::DIV2 => return Ok(2.0),
            XBAR21PredivConf::DIV4 => return Ok(4.0),
            XBAR21PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR21FindivConf {
    Value(u32),
}

impl XBAR21FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR21FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR21Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR21Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR22Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR22PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR22PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR22PredivConf::DIV1 => return Ok(1.0),
            XBAR22PredivConf::DIV2 => return Ok(2.0),
            XBAR22PredivConf::DIV4 => return Ok(4.0),
            XBAR22PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR22FindivConf {
    Value(u32),
}

impl XBAR22FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR22FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR22Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR22Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR23Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
    SPDIF,
    I2S_CKIN,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR23PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR23PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR23PredivConf::DIV1 => return Ok(1.0),
            XBAR23PredivConf::DIV2 => return Ok(2.0),
            XBAR23PredivConf::DIV4 => return Ok(4.0),
            XBAR23PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR23FindivConf {
    Value(u32),
}

impl XBAR23FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR23FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR23Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR23Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR24Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
    SPDIF,
    I2S_CKIN,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR24PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR24PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR24PredivConf::DIV1 => return Ok(1.0),
            XBAR24PredivConf::DIV2 => return Ok(2.0),
            XBAR24PredivConf::DIV4 => return Ok(4.0),
            XBAR24PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR24FindivConf {
    Value(u32),
}

impl XBAR24FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR24FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR24Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR24Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR25Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
    SPDIF,
    I2S_CKIN,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR25PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR25PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR25PredivConf::DIV1 => return Ok(1.0),
            XBAR25PredivConf::DIV2 => return Ok(2.0),
            XBAR25PredivConf::DIV4 => return Ok(4.0),
            XBAR25PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR25FindivConf {
    Value(u32),
}

impl XBAR25FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR25FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR25Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR25Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR26Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR26PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR26PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR26PredivConf::DIV1 => return Ok(1.0),
            XBAR26PredivConf::DIV2 => return Ok(2.0),
            XBAR26PredivConf::DIV4 => return Ok(4.0),
            XBAR26PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR26FindivConf {
    Value(u32),
}

impl XBAR26FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR26FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR26Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR26Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR27Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR27PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR27PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR27PredivConf::DIV1 => return Ok(1.0),
            XBAR27PredivConf::DIV2 => return Ok(2.0),
            XBAR27PredivConf::DIV4 => return Ok(4.0),
            XBAR27PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR27FindivConf {
    Value(u32),
}

impl XBAR27FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR27FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR27Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR27Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR28Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR28PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR28PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR28PredivConf::DIV1 => return Ok(1.0),
            XBAR28PredivConf::DIV2 => return Ok(2.0),
            XBAR28PredivConf::DIV4 => return Ok(4.0),
            XBAR28PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR28FindivConf {
    Value(u32),
}

impl XBAR28FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR28FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR28Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR28Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR29Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR29PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR29PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR29PredivConf::DIV1 => return Ok(1.0),
            XBAR29PredivConf::DIV2 => return Ok(2.0),
            XBAR29PredivConf::DIV4 => return Ok(4.0),
            XBAR29PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR29FindivConf {
    Value(u32),
}

impl XBAR29FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR29FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR29Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR29Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR30Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR30PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR30PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR30PredivConf::DIV1 => return Ok(1.0),
            XBAR30PredivConf::DIV2 => return Ok(2.0),
            XBAR30PredivConf::DIV4 => return Ok(4.0),
            XBAR30PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR30FindivConf {
    Value(u32),
}

impl XBAR30FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR30FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR30Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR30Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR31Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR31PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR31PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR31PredivConf::DIV1 => return Ok(1.0),
            XBAR31PredivConf::DIV2 => return Ok(2.0),
            XBAR31PredivConf::DIV4 => return Ok(4.0),
            XBAR31PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR31FindivConf {
    Value(u32),
}

impl XBAR31FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR31FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR31Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR31Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR32Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR32PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR32PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR32PredivConf::DIV1 => return Ok(1.0),
            XBAR32PredivConf::DIV2 => return Ok(2.0),
            XBAR32PredivConf::DIV4 => return Ok(4.0),
            XBAR32PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR32FindivConf {
    Value(u32),
}

impl XBAR32FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR32FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR32Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR32Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR33Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR33PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR33PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR33PredivConf::DIV1 => return Ok(1.0),
            XBAR33PredivConf::DIV2 => return Ok(2.0),
            XBAR33PredivConf::DIV4 => return Ok(4.0),
            XBAR33PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR33FindivConf {
    Value(u32),
}

impl XBAR33FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR33FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR33Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR33Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR34Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR34PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR34PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR34PredivConf::DIV1 => return Ok(1.0),
            XBAR34PredivConf::DIV2 => return Ok(2.0),
            XBAR34PredivConf::DIV4 => return Ok(4.0),
            XBAR34PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR34FindivConf {
    Value(u32),
}

impl XBAR34FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR34FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR34Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR34Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR35Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR35PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR35PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR35PredivConf::DIV1 => return Ok(1.0),
            XBAR35PredivConf::DIV2 => return Ok(2.0),
            XBAR35PredivConf::DIV4 => return Ok(4.0),
            XBAR35PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR35FindivConf {
    Value(u32),
}

impl XBAR35FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR35FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR35Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR35Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR36Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    MSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR36PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR36PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR36PredivConf::DIV1 => return Ok(1.0),
            XBAR36PredivConf::DIV2 => return Ok(2.0),
            XBAR36PredivConf::DIV4 => return Ok(4.0),
            XBAR36PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR36FindivConf {
    Value(u32),
}

impl XBAR36FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR36FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR36Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR36Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR37Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
    I2S_CKIN,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR37PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR37PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR37PredivConf::DIV1 => return Ok(1.0),
            XBAR37PredivConf::DIV2 => return Ok(2.0),
            XBAR37PredivConf::DIV4 => return Ok(4.0),
            XBAR37PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR37FindivConf {
    Value(u32),
}

impl XBAR37FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR37FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR37Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR37Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR38Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    MSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR38PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR38PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR38PredivConf::DIV1 => return Ok(1.0),
            XBAR38PredivConf::DIV2 => return Ok(2.0),
            XBAR38PredivConf::DIV4 => return Ok(4.0),
            XBAR38PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR38FindivConf {
    Value(u32),
}

impl XBAR38FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR38FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR38Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR38Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR39Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR39PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR39PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR39PredivConf::DIV1 => return Ok(1.0),
            XBAR39PredivConf::DIV2 => return Ok(2.0),
            XBAR39PredivConf::DIV4 => return Ok(4.0),
            XBAR39PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR39FindivConf {
    Value(u32),
}

impl XBAR39FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR39FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR39Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR39Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR40Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    LSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR40PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR40PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR40PredivConf::DIV1 => return Ok(1.0),
            XBAR40PredivConf::DIV2 => return Ok(2.0),
            XBAR40PredivConf::DIV4 => return Ok(4.0),
            XBAR40PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR40FindivConf {
    Value(u32),
}

impl XBAR40FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR40FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR40Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR40Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR41Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    LSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR41PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR41PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR41PredivConf::DIV1 => return Ok(1.0),
            XBAR41PredivConf::DIV2 => return Ok(2.0),
            XBAR41PredivConf::DIV4 => return Ok(4.0),
            XBAR41PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR41FindivConf {
    Value(u32),
}

impl XBAR41FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR41FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR41Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR41Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR42Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR42PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR42PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR42PredivConf::DIV1 => return Ok(1.0),
            XBAR42PredivConf::DIV2 => return Ok(2.0),
            XBAR42PredivConf::DIV4 => return Ok(4.0),
            XBAR42PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR42FindivConf {
    Value(u32),
}

impl XBAR42FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR42FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR42Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR42Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR43Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR43PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR43PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR43PredivConf::DIV1 => return Ok(1.0),
            XBAR43PredivConf::DIV2 => return Ok(2.0),
            XBAR43PredivConf::DIV4 => return Ok(4.0),
            XBAR43PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR43FindivConf {
    Value(u32),
}

impl XBAR43FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR43FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR43Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR43Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR44Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR44PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR44PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR44PredivConf::DIV1 => return Ok(1.0),
            XBAR44PredivConf::DIV2 => return Ok(2.0),
            XBAR44PredivConf::DIV4 => return Ok(4.0),
            XBAR44PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR44FindivConf {
    Value(u32),
}

impl XBAR44FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR44FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR44Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR44Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR45Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR45PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR45PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR45PredivConf::DIV1 => return Ok(1.0),
            XBAR45PredivConf::DIV2 => return Ok(2.0),
            XBAR45PredivConf::DIV4 => return Ok(4.0),
            XBAR45PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR45FindivConf {
    Value(u32),
}

impl XBAR45FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR45FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR45Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR45Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR46Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR46PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR46PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR46PredivConf::DIV1 => return Ok(1.0),
            XBAR46PredivConf::DIV2 => return Ok(2.0),
            XBAR46PredivConf::DIV4 => return Ok(4.0),
            XBAR46PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR46FindivConf {
    Value(u32),
}

impl XBAR46FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR46FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR46Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR46Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR47Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR47PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR47PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR47PredivConf::DIV1 => return Ok(1.0),
            XBAR47PredivConf::DIV2 => return Ok(2.0),
            XBAR47PredivConf::DIV4 => return Ok(4.0),
            XBAR47PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR47FindivConf {
    Value(u32),
}

impl XBAR47FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR47FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR47Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR47Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR48Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR48PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR48PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR48PredivConf::DIV1 => return Ok(1.0),
            XBAR48PredivConf::DIV2 => return Ok(2.0),
            XBAR48PredivConf::DIV4 => return Ok(4.0),
            XBAR48PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR48FindivConf {
    Value(u32),
}

impl XBAR48FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR48FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR48Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR48Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR49Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR49PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR49PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR49PredivConf::DIV1 => return Ok(1.0),
            XBAR49PredivConf::DIV2 => return Ok(2.0),
            XBAR49PredivConf::DIV4 => return Ok(4.0),
            XBAR49PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR49FindivConf {
    Value(u32),
}

impl XBAR49FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR49FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR49Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR49Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR50Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR50PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR50PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR50PredivConf::DIV1 => return Ok(1.0),
            XBAR50PredivConf::DIV2 => return Ok(2.0),
            XBAR50PredivConf::DIV4 => return Ok(4.0),
            XBAR50PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR50FindivConf {
    Value(u32),
}

impl XBAR50FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR50FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR50Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR50Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR51Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR51PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR51PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR51PredivConf::DIV1 => return Ok(1.0),
            XBAR51PredivConf::DIV2 => return Ok(2.0),
            XBAR51PredivConf::DIV4 => return Ok(4.0),
            XBAR51PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR51FindivConf {
    Value(u32),
}

impl XBAR51FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR51FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR51Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR51Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR52Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR52PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR52PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR52PredivConf::DIV1 => return Ok(1.0),
            XBAR52PredivConf::DIV2 => return Ok(2.0),
            XBAR52PredivConf::DIV4 => return Ok(4.0),
            XBAR52PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR52FindivConf {
    Value(u32),
}

impl XBAR52FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR52FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR52Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR52Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR53Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR53PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR53PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR53PredivConf::DIV1 => return Ok(1.0),
            XBAR53PredivConf::DIV2 => return Ok(2.0),
            XBAR53PredivConf::DIV4 => return Ok(4.0),
            XBAR53PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR53FindivConf {
    Value(u32),
}

impl XBAR53FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR53FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR53Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR53Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR54Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR54PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR54PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR54PredivConf::DIV1 => return Ok(1.0),
            XBAR54PredivConf::DIV2 => return Ok(2.0),
            XBAR54PredivConf::DIV4 => return Ok(4.0),
            XBAR54PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR54FindivConf {
    Value(u32),
}

impl XBAR54FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR54FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR54Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR54Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR55Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR55PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR55PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR55PredivConf::DIV1 => return Ok(1.0),
            XBAR55PredivConf::DIV2 => return Ok(2.0),
            XBAR55PredivConf::DIV4 => return Ok(4.0),
            XBAR55PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR55FindivConf {
    Value(u32),
}

impl XBAR55FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR55FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR55Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR55Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR56Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR56PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR56PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR56PredivConf::DIV1 => return Ok(1.0),
            XBAR56PredivConf::DIV2 => return Ok(2.0),
            XBAR56PredivConf::DIV4 => return Ok(4.0),
            XBAR56PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR56FindivConf {
    Value(u32),
}

impl XBAR56FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR56FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR56Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR56Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR57Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR57PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR57PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR57PredivConf::DIV1 => return Ok(1.0),
            XBAR57PredivConf::DIV2 => return Ok(2.0),
            XBAR57PredivConf::DIV4 => return Ok(4.0),
            XBAR57PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR57FindivConf {
    Value(u32),
}

impl XBAR57FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR57FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR57Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR57Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR58Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR58PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR58PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR58PredivConf::DIV1 => return Ok(1.0),
            XBAR58PredivConf::DIV2 => return Ok(2.0),
            XBAR58PredivConf::DIV4 => return Ok(4.0),
            XBAR58PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR58FindivConf {
    Value(u32),
}

impl XBAR58FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR58FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR58Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR58Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR59Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR59PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR59PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR59PredivConf::DIV1 => return Ok(1.0),
            XBAR59PredivConf::DIV2 => return Ok(2.0),
            XBAR59PredivConf::DIV4 => return Ok(4.0),
            XBAR59PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR59FindivConf {
    Value(u32),
}

impl XBAR59FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR59FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR59Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR59Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR60Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR60PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR60PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR60PredivConf::DIV1 => return Ok(1.0),
            XBAR60PredivConf::DIV2 => return Ok(2.0),
            XBAR60PredivConf::DIV4 => return Ok(4.0),
            XBAR60PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR60FindivConf {
    Value(u32),
}

impl XBAR60FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR60FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR60Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR60Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR61Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
    SPDIF,
    I2S_CKIN,
    LSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR61PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR61PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR61PredivConf::DIV1 => return Ok(1.0),
            XBAR61PredivConf::DIV2 => return Ok(2.0),
            XBAR61PredivConf::DIV4 => return Ok(4.0),
            XBAR61PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR61FindivConf {
    Value(u32),
}

impl XBAR61FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR61FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR61Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR61Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR62Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
    SPDIF,
    I2S_CKIN,
    LSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR62PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR62PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR62PredivConf::DIV1 => return Ok(1.0),
            XBAR62PredivConf::DIV2 => return Ok(2.0),
            XBAR62PredivConf::DIV4 => return Ok(4.0),
            XBAR62PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR62FindivConf {
    Value(u32),
}

impl XBAR62FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR62FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR62Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR62Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR63Conf {
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    HSIRC,
    HSEOSC,
    MSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR63PredivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV1024,
}

impl XBAR63PredivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR63PredivConf::DIV1 => return Ok(1.0),
            XBAR63PredivConf::DIV2 => return Ok(2.0),
            XBAR63PredivConf::DIV4 => return Ok(4.0),
            XBAR63PredivConf::DIV1024 => return Ok(1024.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum XBAR63FindivConf {
    Value(u32),
}

impl XBAR63FindivConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        64
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            XBAR63FindivConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR63Findiv,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::XBAR63Findiv,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CKINTSEL0Conf {
    HSIRC,
    HSEOSC,
    MSIRC,
    PLL4Source,
    PLL5Source,
    PLL6Source,
    PLL7Source,
    PLL8Source,
    PLL1Source,
    PLL2Source,
    PLL3Source,
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    SPDIF,
    I2S_CKIN,
    LSIRC,
    LSEOSC,
    XBAR0Output,
    XBAR1Output,
    XBAR2Output,
    XBAR3Output,
    XBAR4Output,
    XBAR5Output,
    XBAR6Output,
    XBAR7Output,
    XBAR8Output,
    XBAR9Output,
    XBAR10Output,
    XBAR11Output,
    XBAR12Output,
    XBAR13Output,
    XBAR14Output,
    XBAR15Output,
    XBAR16Output,
    XBAR17Output,
    XBAR18Output,
    XBAR19Output,
    XBAR20Output,
    XBAR21Output,
    XBAR22Output,
    XBAR23Output,
    XBAR24Output,
    XBAR25Output,
    XBAR26Output,
    XBAR27Output,
    XBAR28Output,
    XBAR29Output,
    XBAR30Output,
    XBAR31Output,
    XBAR32Output,
    XBAR33Output,
    XBAR34Output,
    XBAR35Output,
    XBAR36Output,
    XBAR37Output,
    XBAR38Output,
    XBAR39Output,
    XBAR40Output,
    XBAR41Output,
    XBAR42Output,
    XBAR43Output,
    XBAR44Output,
    XBAR45Output,
    XBAR46Output,
    XBAR47Output,
    XBAR48Output,
    XBAR49Output,
    XBAR50Output,
    XBAR51Output,
    XBAR52Output,
    XBAR53Output,
    XBAR54Output,
    XBAR55Output,
    XBAR56Output,
    XBAR57Output,
    XBAR58Output,
    XBAR59Output,
    XBAR60Output,
    XBAR61Output,
    XBAR62Output,
    XBAR63Output,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CKEXTSEL0Conf {
    PLL1Div42,
    PLL2Div4,
    PLL3Div2,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CKINTSEL1Conf {
    HSIRC,
    HSEOSC,
    MSIRC,
    PLL4Source,
    PLL5Source,
    PLL6Source,
    PLL7Source,
    PLL8Source,
    PLL1Source,
    PLL2Source,
    PLL3Source,
    FOUTPOSTDIV4,
    FOUTPOSTDIV5,
    FOUTPOSTDIV6,
    FOUTPOSTDIV7,
    FOUTPOSTDIV8,
    SPDIF,
    I2S_CKIN,
    LSIRC,
    LSEOSC,
    XBAR0Output,
    XBAR1Output,
    XBAR2Output,
    XBAR3Output,
    XBAR4Output,
    XBAR5Output,
    XBAR6Output,
    XBAR7Output,
    XBAR8Output,
    XBAR9Output,
    XBAR10Output,
    XBAR11Output,
    XBAR12Output,
    XBAR13Output,
    XBAR14Output,
    XBAR15Output,
    XBAR16Output,
    XBAR17Output,
    XBAR18Output,
    XBAR19Output,
    XBAR20Output,
    XBAR21Output,
    XBAR22Output,
    XBAR23Output,
    XBAR24Output,
    XBAR25Output,
    XBAR26Output,
    XBAR27Output,
    XBAR28Output,
    XBAR29Output,
    XBAR30Output,
    XBAR31Output,
    XBAR32Output,
    XBAR33Output,
    XBAR34Output,
    XBAR35Output,
    XBAR36Output,
    XBAR37Output,
    XBAR38Output,
    XBAR39Output,
    XBAR40Output,
    XBAR41Output,
    XBAR42Output,
    XBAR43Output,
    XBAR44Output,
    XBAR45Output,
    XBAR46Output,
    XBAR47Output,
    XBAR48Output,
    XBAR49Output,
    XBAR50Output,
    XBAR51Output,
    XBAR52Output,
    XBAR53Output,
    XBAR54Output,
    XBAR55Output,
    XBAR56Output,
    XBAR57Output,
    XBAR58Output,
    XBAR59Output,
    XBAR60Output,
    XBAR61Output,
    XBAR62Output,
    XBAR63Output,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CKEXTSEL1Conf {
    PLL1Div42,
    PLL2Div4,
    PLL3Div2,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum OBS0Conf {
    CKINTSEL0,
    CKEXTSEL0,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum OBS1Conf {
    CKINTSEL1,
    CKEXTSEL1,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCO1MultConf {
    XBAR61Output,
    OBS0Output,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCO2MultConf {
    XBAR62Output,
    OBS1Output,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum D3PERConf {
    MSIRC,
    LSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DTSConf {
    MSIRC,
    HSIRC,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DSIPHYConf {
    XBAR28Output,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DSIBLANEConf {
    DSIPHYOutput,
    XBAR27Output,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USB2PHY1Conf {
    XBAR57Output,
    HSEDIV2,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USB2PHY2Conf {
    XBAR58Output,
    HSEDIV2,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USB3PCIPHYConf {
    XBAR34Output,
    HSEDIV2,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SysClkSourceConf {
    XBAR0Output,
    HSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCUDIVConf {
    DIV1,
    DIV2,
}

impl MCUDIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MCUDIVConf::DIV1 => return Ok(1.0),
            MCUDIVConf::DIV2 => return Ok(2.0),
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
pub enum APBDBGDIVConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl APBDBGDIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            APBDBGDIVConf::DIV1 => return Ok(1.0),
            APBDBGDIVConf::DIV2 => return Ok(2.0),
            APBDBGDIVConf::DIV4 => return Ok(4.0),
            APBDBGDIVConf::DIV8 => return Ok(8.0),
            APBDBGDIVConf::DIV16 => return Ok(16.0),
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
pub enum ADC12MultConf {
    XBAR46Output,
    XBAR0Output,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ADC3MultConf {
    XBAR47Output,
    XBAR0Output,
    XBAR46Output,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL1SourceConf {
    HSIRC,
    HSEOSC,
    MSIRC,
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
    HSEOSC,
    MSIRC,
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
        16
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
pub enum PLL5SourceConf {
    HSIRC,
    MSIRC,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FREFDIV5Conf {
    Value(u32),
}

impl FREFDIV5Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        63
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            FREFDIV5Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::FREFDIV5,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::FREFDIV5,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FBDIV5Conf {
    Value(u32),
}

impl FBDIV5Conf {
    pub const fn min() -> u32 {
        20
    }

    pub const fn max() -> u32 {
        500
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            FBDIV5Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::FBDIV5,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::FBDIV5,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL5FRACVConf {
    Value(u32),
}

impl PLL5FRACVConf {
    pub const fn min() -> u32 {
        0
    }

    pub const fn max() -> u32 {
        16777215
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL5FRACVConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL5FRACV,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL5FRACV,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum POSTDIV1_5Conf {
    Value(u32),
}

impl POSTDIV1_5Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        7
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            POSTDIV1_5Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV1_5,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV1_5,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum POSTDIV2_5Conf {
    Value(u32),
}

impl POSTDIV2_5Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        7
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            POSTDIV2_5Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV2_5,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV2_5,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL6SourceConf {
    HSIRC,
    MSIRC,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FREFDIV6Conf {
    Value(u32),
}

impl FREFDIV6Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        63
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            FREFDIV6Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::FREFDIV6,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::FREFDIV6,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FBDIV6Conf {
    Value(u32),
}

impl FBDIV6Conf {
    pub const fn min() -> u32 {
        20
    }

    pub const fn max() -> u32 {
        500
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            FBDIV6Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::FBDIV6,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::FBDIV6,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL6FRACVConf {
    Value(u32),
}

impl PLL6FRACVConf {
    pub const fn min() -> u32 {
        0
    }

    pub const fn max() -> u32 {
        16777215
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL6FRACVConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL6FRACV,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL6FRACV,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum POSTDIV1_6Conf {
    Value(u32),
}

impl POSTDIV1_6Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        7
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            POSTDIV1_6Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV1_6,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV1_6,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum POSTDIV2_6Conf {
    Value(u32),
}

impl POSTDIV2_6Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        7
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            POSTDIV2_6Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV2_6,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV2_6,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL7SourceConf {
    HSIRC,
    MSIRC,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FREFDIV7Conf {
    Value(u32),
}

impl FREFDIV7Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        63
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            FREFDIV7Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::FREFDIV7,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::FREFDIV7,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FBDIV7Conf {
    Value(u32),
}

impl FBDIV7Conf {
    pub const fn min() -> u32 {
        20
    }

    pub const fn max() -> u32 {
        500
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            FBDIV7Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::FBDIV7,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::FBDIV7,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL7FRACVConf {
    Value(u32),
}

impl PLL7FRACVConf {
    pub const fn min() -> u32 {
        0
    }

    pub const fn max() -> u32 {
        16777215
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL7FRACVConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL7FRACV,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL7FRACV,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum POSTDIV1_7Conf {
    Value(u32),
}

impl POSTDIV1_7Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        7
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            POSTDIV1_7Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV1_7,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV1_7,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum POSTDIV2_7Conf {
    Value(u32),
}

impl POSTDIV2_7Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        7
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            POSTDIV2_7Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV2_7,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV2_7,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL8SourceConf {
    HSIRC,
    MSIRC,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FREFDIV8Conf {
    Value(u32),
}

impl FREFDIV8Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        63
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            FREFDIV8Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::FREFDIV8,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::FREFDIV8,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FBDIV8Conf {
    Value(u32),
}

impl FBDIV8Conf {
    pub const fn min() -> u32 {
        20
    }

    pub const fn max() -> u32 {
        500
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            FBDIV8Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::FBDIV8,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::FBDIV8,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL8FRACVConf {
    Value(u32),
}

impl PLL8FRACVConf {
    pub const fn min() -> u32 {
        0
    }

    pub const fn max() -> u32 {
        16777215
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL8FRACVConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL8FRACV,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL8FRACV,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum POSTDIV1_8Conf {
    Value(u32),
}

impl POSTDIV1_8Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        7
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            POSTDIV1_8Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV1_8,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV1_8,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum POSTDIV2_8Conf {
    Value(u32),
}

impl POSTDIV2_8Conf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        7
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            POSTDIV2_8Conf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV2_8,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::POSTDIV2_8,
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
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ClockTree {
    pub HSEOSC: HSEOSCConf,
    pub HSEDIV2: HSEDIV2Conf,
    pub SPDIF: SPDIFConf,
    pub LSEOSC: LSEOSCConf,
    pub MSIRC: MSIRCConf,
    pub XBAR0: XBAR0Conf,
    pub XBAR0Prediv: XBAR0PredivConf,
    pub XBAR0Findiv: XBAR0FindivConf,
    pub XBAR1: XBAR1Conf,
    pub XBAR1Prediv: XBAR1PredivConf,
    pub XBAR1Findiv: XBAR1FindivConf,
    pub XBAR2: XBAR2Conf,
    pub XBAR2Prediv: XBAR2PredivConf,
    pub XBAR2Findiv: XBAR2FindivConf,
    pub XBAR3: XBAR3Conf,
    pub XBAR3Prediv: XBAR3PredivConf,
    pub XBAR3Findiv: XBAR3FindivConf,
    pub XBAR4: XBAR4Conf,
    pub XBAR4Prediv: XBAR4PredivConf,
    pub XBAR4Findiv: XBAR4FindivConf,
    pub XBAR5: XBAR5Conf,
    pub XBAR5Prediv: XBAR5PredivConf,
    pub XBAR5Findiv: XBAR5FindivConf,
    pub XBAR6: XBAR6Conf,
    pub XBAR6Prediv: XBAR6PredivConf,
    pub XBAR6Findiv: XBAR6FindivConf,
    pub XBAR7: XBAR7Conf,
    pub XBAR7Prediv: XBAR7PredivConf,
    pub XBAR7Findiv: XBAR7FindivConf,
    pub XBAR8: XBAR8Conf,
    pub XBAR8Prediv: XBAR8PredivConf,
    pub XBAR8Findiv: XBAR8FindivConf,
    pub XBAR9: XBAR9Conf,
    pub XBAR9Prediv: XBAR9PredivConf,
    pub XBAR9Findiv: XBAR9FindivConf,
    pub XBAR10: XBAR10Conf,
    pub XBAR10Prediv: XBAR10PredivConf,
    pub XBAR10Findiv: XBAR10FindivConf,
    pub XBAR11: XBAR11Conf,
    pub XBAR11Prediv: XBAR11PredivConf,
    pub XBAR11Findiv: XBAR11FindivConf,
    pub XBAR12: XBAR12Conf,
    pub XBAR12Prediv: XBAR12PredivConf,
    pub XBAR12Findiv: XBAR12FindivConf,
    pub XBAR13: XBAR13Conf,
    pub XBAR13Prediv: XBAR13PredivConf,
    pub XBAR13Findiv: XBAR13FindivConf,
    pub XBAR14: XBAR14Conf,
    pub XBAR14Prediv: XBAR14PredivConf,
    pub XBAR14Findiv: XBAR14FindivConf,
    pub XBAR15: XBAR15Conf,
    pub XBAR15Prediv: XBAR15PredivConf,
    pub XBAR15Findiv: XBAR15FindivConf,
    pub XBAR16: XBAR16Conf,
    pub XBAR16Prediv: XBAR16PredivConf,
    pub XBAR16Findiv: XBAR16FindivConf,
    pub XBAR17: XBAR17Conf,
    pub XBAR17Prediv: XBAR17PredivConf,
    pub XBAR17Findiv: XBAR17FindivConf,
    pub XBAR18: XBAR18Conf,
    pub XBAR18Prediv: XBAR18PredivConf,
    pub XBAR18Findiv: XBAR18FindivConf,
    pub XBAR19: XBAR19Conf,
    pub XBAR19Prediv: XBAR19PredivConf,
    pub XBAR19Findiv: XBAR19FindivConf,
    pub XBAR20: XBAR20Conf,
    pub XBAR20Prediv: XBAR20PredivConf,
    pub XBAR20Findiv: XBAR20FindivConf,
    pub XBAR21: XBAR21Conf,
    pub XBAR21Prediv: XBAR21PredivConf,
    pub XBAR21Findiv: XBAR21FindivConf,
    pub XBAR22: XBAR22Conf,
    pub XBAR22Prediv: XBAR22PredivConf,
    pub XBAR22Findiv: XBAR22FindivConf,
    pub XBAR23: XBAR23Conf,
    pub XBAR23Prediv: XBAR23PredivConf,
    pub XBAR23Findiv: XBAR23FindivConf,
    pub XBAR24: XBAR24Conf,
    pub XBAR24Prediv: XBAR24PredivConf,
    pub XBAR24Findiv: XBAR24FindivConf,
    pub XBAR25: XBAR25Conf,
    pub XBAR25Prediv: XBAR25PredivConf,
    pub XBAR25Findiv: XBAR25FindivConf,
    pub XBAR26: XBAR26Conf,
    pub XBAR26Prediv: XBAR26PredivConf,
    pub XBAR26Findiv: XBAR26FindivConf,
    pub XBAR27: XBAR27Conf,
    pub XBAR27Prediv: XBAR27PredivConf,
    pub XBAR27Findiv: XBAR27FindivConf,
    pub XBAR28: XBAR28Conf,
    pub XBAR28Prediv: XBAR28PredivConf,
    pub XBAR28Findiv: XBAR28FindivConf,
    pub XBAR29: XBAR29Conf,
    pub XBAR29Prediv: XBAR29PredivConf,
    pub XBAR29Findiv: XBAR29FindivConf,
    pub XBAR30: XBAR30Conf,
    pub XBAR30Prediv: XBAR30PredivConf,
    pub XBAR30Findiv: XBAR30FindivConf,
    pub XBAR31: XBAR31Conf,
    pub XBAR31Prediv: XBAR31PredivConf,
    pub XBAR31Findiv: XBAR31FindivConf,
    pub XBAR32: XBAR32Conf,
    pub XBAR32Prediv: XBAR32PredivConf,
    pub XBAR32Findiv: XBAR32FindivConf,
    pub XBAR33: XBAR33Conf,
    pub XBAR33Prediv: XBAR33PredivConf,
    pub XBAR33Findiv: XBAR33FindivConf,
    pub XBAR34: XBAR34Conf,
    pub XBAR34Prediv: XBAR34PredivConf,
    pub XBAR34Findiv: XBAR34FindivConf,
    pub XBAR35: XBAR35Conf,
    pub XBAR35Prediv: XBAR35PredivConf,
    pub XBAR35Findiv: XBAR35FindivConf,
    pub XBAR36: XBAR36Conf,
    pub XBAR36Prediv: XBAR36PredivConf,
    pub XBAR36Findiv: XBAR36FindivConf,
    pub XBAR37: XBAR37Conf,
    pub XBAR37Prediv: XBAR37PredivConf,
    pub XBAR37Findiv: XBAR37FindivConf,
    pub XBAR38: XBAR38Conf,
    pub XBAR38Prediv: XBAR38PredivConf,
    pub XBAR38Findiv: XBAR38FindivConf,
    pub XBAR39: XBAR39Conf,
    pub XBAR39Prediv: XBAR39PredivConf,
    pub XBAR39Findiv: XBAR39FindivConf,
    pub XBAR40: XBAR40Conf,
    pub XBAR40Prediv: XBAR40PredivConf,
    pub XBAR40Findiv: XBAR40FindivConf,
    pub XBAR41: XBAR41Conf,
    pub XBAR41Prediv: XBAR41PredivConf,
    pub XBAR41Findiv: XBAR41FindivConf,
    pub XBAR42: XBAR42Conf,
    pub XBAR42Prediv: XBAR42PredivConf,
    pub XBAR42Findiv: XBAR42FindivConf,
    pub XBAR43: XBAR43Conf,
    pub XBAR43Prediv: XBAR43PredivConf,
    pub XBAR43Findiv: XBAR43FindivConf,
    pub XBAR44: XBAR44Conf,
    pub XBAR44Prediv: XBAR44PredivConf,
    pub XBAR44Findiv: XBAR44FindivConf,
    pub XBAR45: XBAR45Conf,
    pub XBAR45Prediv: XBAR45PredivConf,
    pub XBAR45Findiv: XBAR45FindivConf,
    pub XBAR46: XBAR46Conf,
    pub XBAR46Prediv: XBAR46PredivConf,
    pub XBAR46Findiv: XBAR46FindivConf,
    pub XBAR47: XBAR47Conf,
    pub XBAR47Prediv: XBAR47PredivConf,
    pub XBAR47Findiv: XBAR47FindivConf,
    pub XBAR48: XBAR48Conf,
    pub XBAR48Prediv: XBAR48PredivConf,
    pub XBAR48Findiv: XBAR48FindivConf,
    pub XBAR49: XBAR49Conf,
    pub XBAR49Prediv: XBAR49PredivConf,
    pub XBAR49Findiv: XBAR49FindivConf,
    pub XBAR50: XBAR50Conf,
    pub XBAR50Prediv: XBAR50PredivConf,
    pub XBAR50Findiv: XBAR50FindivConf,
    pub XBAR51: XBAR51Conf,
    pub XBAR51Prediv: XBAR51PredivConf,
    pub XBAR51Findiv: XBAR51FindivConf,
    pub XBAR52: XBAR52Conf,
    pub XBAR52Prediv: XBAR52PredivConf,
    pub XBAR52Findiv: XBAR52FindivConf,
    pub XBAR53: XBAR53Conf,
    pub XBAR53Prediv: XBAR53PredivConf,
    pub XBAR53Findiv: XBAR53FindivConf,
    pub XBAR54: XBAR54Conf,
    pub XBAR54Prediv: XBAR54PredivConf,
    pub XBAR54Findiv: XBAR54FindivConf,
    pub XBAR55: XBAR55Conf,
    pub XBAR55Prediv: XBAR55PredivConf,
    pub XBAR55Findiv: XBAR55FindivConf,
    pub XBAR56: XBAR56Conf,
    pub XBAR56Prediv: XBAR56PredivConf,
    pub XBAR56Findiv: XBAR56FindivConf,
    pub XBAR57: XBAR57Conf,
    pub XBAR57Prediv: XBAR57PredivConf,
    pub XBAR57Findiv: XBAR57FindivConf,
    pub XBAR58: XBAR58Conf,
    pub XBAR58Prediv: XBAR58PredivConf,
    pub XBAR58Findiv: XBAR58FindivConf,
    pub XBAR59: XBAR59Conf,
    pub XBAR59Prediv: XBAR59PredivConf,
    pub XBAR59Findiv: XBAR59FindivConf,
    pub XBAR60: XBAR60Conf,
    pub XBAR60Prediv: XBAR60PredivConf,
    pub XBAR60Findiv: XBAR60FindivConf,
    pub XBAR61: XBAR61Conf,
    pub XBAR61Prediv: XBAR61PredivConf,
    pub XBAR61Findiv: XBAR61FindivConf,
    pub XBAR62: XBAR62Conf,
    pub XBAR62Prediv: XBAR62PredivConf,
    pub XBAR62Findiv: XBAR62FindivConf,
    pub XBAR63: XBAR63Conf,
    pub XBAR63Prediv: XBAR63PredivConf,
    pub XBAR63Findiv: XBAR63FindivConf,
    pub CKINTSEL0: CKINTSEL0Conf,
    pub CKEXTSEL0: CKEXTSEL0Conf,
    pub CKINTSEL1: CKINTSEL1Conf,
    pub CKEXTSEL1: CKEXTSEL1Conf,
    pub OBS0: OBS0Conf,
    pub OBS1: OBS1Conf,
    pub MCO1Mult: MCO1MultConf,
    pub MCO2Mult: MCO2MultConf,
    pub D3PER: D3PERConf,
    pub DTS: DTSConf,
    pub DSIPHY: DSIPHYConf,
    pub DSIBLANE: DSIBLANEConf,
    pub USB2PHY1: USB2PHY1Conf,
    pub USB2PHY2: USB2PHY2Conf,
    pub USB3PCIPHY: USB3PCIPHYConf,
    pub SysClkSource: SysClkSourceConf,
    pub MCUDIV: MCUDIVConf,
    pub APB3DIV: APB3DIVConf,
    pub APB4DIV: APB4DIVConf,
    pub APBDBGDIV: APBDBGDIVConf,
    pub APB1DIV: APB1DIVConf,
    pub APB2DIV: APB2DIVConf,
    pub ADC12Mult: ADC12MultConf,
    pub ADC3Mult: ADC3MultConf,
    pub PLL1Source: PLL1SourceConf,
    pub FREFDIV1: FREFDIV1Conf,
    pub PLL2Source: PLL2SourceConf,
    pub FREFDIV2: FREFDIV2Conf,
    pub PLL3Source: PLL3SourceConf,
    pub FREFDIV3: FREFDIV3Conf,
    pub PLL4Source: PLL4SourceConf,
    pub FREFDIV4: FREFDIV4Conf,
    pub FBDIV1: FBDIV1Conf,
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
    pub PLL5Source: PLL5SourceConf,
    pub FREFDIV5: FREFDIV5Conf,
    pub FBDIV5: FBDIV5Conf,
    pub PLL5FRACV: PLL5FRACVConf,
    pub POSTDIV1_5: POSTDIV1_5Conf,
    pub POSTDIV2_5: POSTDIV2_5Conf,
    pub PLL6Source: PLL6SourceConf,
    pub FREFDIV6: FREFDIV6Conf,
    pub FBDIV6: FBDIV6Conf,
    pub PLL6FRACV: PLL6FRACVConf,
    pub POSTDIV1_6: POSTDIV1_6Conf,
    pub POSTDIV2_6: POSTDIV2_6Conf,
    pub PLL7Source: PLL7SourceConf,
    pub FREFDIV7: FREFDIV7Conf,
    pub FBDIV7: FBDIV7Conf,
    pub PLL7FRACV: PLL7FRACVConf,
    pub POSTDIV1_7: POSTDIV1_7Conf,
    pub POSTDIV2_7: POSTDIV2_7Conf,
    pub PLL8Source: PLL8SourceConf,
    pub FREFDIV8: FREFDIV8Conf,
    pub FBDIV8: FBDIV8Conf,
    pub PLL8FRACV: PLL8FRACVConf,
    pub POSTDIV1_8: POSTDIV1_8Conf,
    pub POSTDIV2_8: POSTDIV2_8Conf,
    pub HSERTCDevisor: HSERTCDevisorConf,
    pub RTCClkSource: RTCClkSourceConf,
}

impl Default for ClockTree {
    fn default() -> Self {
        Self {
            HSEOSC: HSEOSCConf::Value(40000000),
            HSEDIV2: HSEDIV2Conf::DIV2,
            SPDIF: SPDIFConf::Value(12288000),
            LSEOSC: LSEOSCConf::Value(32768),
            MSIRC: MSIRCConf::CLOCK_16,
            XBAR0: XBAR0Conf::HSIRC,
            XBAR0Prediv: XBAR0PredivConf::DIV1,
            XBAR0Findiv: XBAR0FindivConf::Value(1),
            XBAR1: XBAR1Conf::HSIRC,
            XBAR1Prediv: XBAR1PredivConf::DIV1,
            XBAR1Findiv: XBAR1FindivConf::Value(1),
            XBAR2: XBAR2Conf::HSIRC,
            XBAR2Prediv: XBAR2PredivConf::DIV1,
            XBAR2Findiv: XBAR2FindivConf::Value(1),
            XBAR3: XBAR3Conf::HSIRC,
            XBAR3Prediv: XBAR3PredivConf::DIV1,
            XBAR3Findiv: XBAR3FindivConf::Value(1),
            XBAR4: XBAR4Conf::HSIRC,
            XBAR4Prediv: XBAR4PredivConf::DIV1,
            XBAR4Findiv: XBAR4FindivConf::Value(1),
            XBAR5: XBAR5Conf::HSIRC,
            XBAR5Prediv: XBAR5PredivConf::DIV1,
            XBAR5Findiv: XBAR5FindivConf::Value(1),
            XBAR6: XBAR6Conf::HSIRC,
            XBAR6Prediv: XBAR6PredivConf::DIV1,
            XBAR6Findiv: XBAR6FindivConf::Value(1),
            XBAR7: XBAR7Conf::LSIRC,
            XBAR7Prediv: XBAR7PredivConf::DIV1,
            XBAR7Findiv: XBAR7FindivConf::Value(1),
            XBAR8: XBAR8Conf::HSIRC,
            XBAR8Prediv: XBAR8PredivConf::DIV1,
            XBAR8Findiv: XBAR8FindivConf::Value(1),
            XBAR9: XBAR9Conf::HSIRC,
            XBAR9Prediv: XBAR9PredivConf::DIV1,
            XBAR9Findiv: XBAR9FindivConf::Value(1),
            XBAR10: XBAR10Conf::HSIRC,
            XBAR10Prediv: XBAR10PredivConf::DIV1,
            XBAR10Findiv: XBAR10FindivConf::Value(1),
            XBAR11: XBAR11Conf::HSIRC,
            XBAR11Prediv: XBAR11PredivConf::DIV1,
            XBAR11Findiv: XBAR11FindivConf::Value(1),
            XBAR12: XBAR12Conf::HSIRC,
            XBAR12Prediv: XBAR12PredivConf::DIV1,
            XBAR12Findiv: XBAR12FindivConf::Value(1),
            XBAR13: XBAR13Conf::HSIRC,
            XBAR13Prediv: XBAR13PredivConf::DIV1,
            XBAR13Findiv: XBAR13FindivConf::Value(1),
            XBAR14: XBAR14Conf::HSIRC,
            XBAR14Prediv: XBAR14PredivConf::DIV1,
            XBAR14Findiv: XBAR14FindivConf::Value(1),
            XBAR15: XBAR15Conf::HSIRC,
            XBAR15Prediv: XBAR15PredivConf::DIV1,
            XBAR15Findiv: XBAR15FindivConf::Value(1),
            XBAR16: XBAR16Conf::HSIRC,
            XBAR16Prediv: XBAR16PredivConf::DIV1,
            XBAR16Findiv: XBAR16FindivConf::Value(1),
            XBAR17: XBAR17Conf::HSIRC,
            XBAR17Prediv: XBAR17PredivConf::DIV1,
            XBAR17Findiv: XBAR17FindivConf::Value(1),
            XBAR18: XBAR18Conf::HSIRC,
            XBAR18Prediv: XBAR18PredivConf::DIV1,
            XBAR18Findiv: XBAR18FindivConf::Value(1),
            XBAR19: XBAR19Conf::HSIRC,
            XBAR19Prediv: XBAR19PredivConf::DIV1,
            XBAR19Findiv: XBAR19FindivConf::Value(1),
            XBAR20: XBAR20Conf::HSIRC,
            XBAR20Prediv: XBAR20PredivConf::DIV1,
            XBAR20Findiv: XBAR20FindivConf::Value(1),
            XBAR21: XBAR21Conf::HSIRC,
            XBAR21Prediv: XBAR21PredivConf::DIV1,
            XBAR21Findiv: XBAR21FindivConf::Value(1),
            XBAR22: XBAR22Conf::HSIRC,
            XBAR22Prediv: XBAR22PredivConf::DIV1,
            XBAR22Findiv: XBAR22FindivConf::Value(1),
            XBAR23: XBAR23Conf::HSIRC,
            XBAR23Prediv: XBAR23PredivConf::DIV1,
            XBAR23Findiv: XBAR23FindivConf::Value(1),
            XBAR24: XBAR24Conf::HSIRC,
            XBAR24Prediv: XBAR24PredivConf::DIV1,
            XBAR24Findiv: XBAR24FindivConf::Value(1),
            XBAR25: XBAR25Conf::HSIRC,
            XBAR25Prediv: XBAR25PredivConf::DIV1,
            XBAR25Findiv: XBAR25FindivConf::Value(1),
            XBAR26: XBAR26Conf::HSEOSC,
            XBAR26Prediv: XBAR26PredivConf::DIV1,
            XBAR26Findiv: XBAR26FindivConf::Value(1),
            XBAR27: XBAR27Conf::FOUTPOSTDIV8,
            XBAR27Prediv: XBAR27PredivConf::DIV1,
            XBAR27Findiv: XBAR27FindivConf::Value(1),
            XBAR28: XBAR28Conf::HSEOSC,
            XBAR28Prediv: XBAR28PredivConf::DIV1,
            XBAR28Findiv: XBAR28FindivConf::Value(1),
            XBAR29: XBAR29Conf::FOUTPOSTDIV8,
            XBAR29Prediv: XBAR29PredivConf::DIV1,
            XBAR29Findiv: XBAR29FindivConf::Value(1),
            XBAR30: XBAR30Conf::HSEOSC,
            XBAR30Prediv: XBAR30PredivConf::DIV1,
            XBAR30Findiv: XBAR30FindivConf::Value(1),
            XBAR31: XBAR31Conf::HSEOSC,
            XBAR31Prediv: XBAR31PredivConf::DIV1,
            XBAR31Findiv: XBAR31FindivConf::Value(1),
            XBAR32: XBAR32Conf::HSEOSC,
            XBAR32Prediv: XBAR32PredivConf::DIV1,
            XBAR32Findiv: XBAR32FindivConf::Value(1),
            XBAR33: XBAR33Conf::HSIRC,
            XBAR33Prediv: XBAR33PredivConf::DIV1,
            XBAR33Findiv: XBAR33FindivConf::Value(1),
            XBAR34: XBAR34Conf::HSEOSC,
            XBAR34Prediv: XBAR34PredivConf::DIV1,
            XBAR34Findiv: XBAR34FindivConf::Value(1),
            XBAR35: XBAR35Conf::HSIRC,
            XBAR35Prediv: XBAR35PredivConf::DIV1,
            XBAR35Findiv: XBAR35FindivConf::Value(1),
            XBAR36: XBAR36Conf::MSIRC,
            XBAR36Prediv: XBAR36PredivConf::DIV1,
            XBAR36Findiv: XBAR36FindivConf::Value(1),
            XBAR37: XBAR37Conf::HSIRC,
            XBAR37Prediv: XBAR37PredivConf::DIV1,
            XBAR37Findiv: XBAR37FindivConf::Value(1),
            XBAR38: XBAR38Conf::HSIRC,
            XBAR38Prediv: XBAR38PredivConf::DIV1,
            XBAR38Findiv: XBAR38FindivConf::Value(1),
            XBAR39: XBAR39Conf::HSIRC,
            XBAR39Prediv: XBAR39PredivConf::DIV1,
            XBAR39Findiv: XBAR39FindivConf::Value(1),
            XBAR40: XBAR40Conf::LSIRC,
            XBAR40Prediv: XBAR40PredivConf::DIV1,
            XBAR40Findiv: XBAR40FindivConf::Value(1),
            XBAR41: XBAR41Conf::LSIRC,
            XBAR41Prediv: XBAR41PredivConf::DIV1,
            XBAR41Findiv: XBAR41FindivConf::Value(1),
            XBAR42: XBAR42Conf::HSIRC,
            XBAR42Prediv: XBAR42PredivConf::DIV1,
            XBAR42Findiv: XBAR42FindivConf::Value(1),
            XBAR43: XBAR43Conf::FOUTPOSTDIV8,
            XBAR43Prediv: XBAR43PredivConf::DIV1,
            XBAR43Findiv: XBAR43FindivConf::Value(1),
            XBAR44: XBAR44Conf::FOUTPOSTDIV8,
            XBAR44Prediv: XBAR44PredivConf::DIV1,
            XBAR44Findiv: XBAR44FindivConf::Value(1),
            XBAR45: XBAR45Conf::FOUTPOSTDIV8,
            XBAR45Prediv: XBAR45PredivConf::DIV1,
            XBAR45Findiv: XBAR45FindivConf::Value(1),
            XBAR46: XBAR46Conf::HSIRC,
            XBAR46Prediv: XBAR46PredivConf::DIV1,
            XBAR46Findiv: XBAR46FindivConf::Value(1),
            XBAR47: XBAR47Conf::FOUTPOSTDIV8,
            XBAR47Prediv: XBAR47PredivConf::DIV1,
            XBAR47Findiv: XBAR47FindivConf::Value(1),
            XBAR48: XBAR48Conf::FOUTPOSTDIV8,
            XBAR48Prediv: XBAR48PredivConf::DIV1,
            XBAR48Findiv: XBAR48FindivConf::Value(1),
            XBAR49: XBAR49Conf::FOUTPOSTDIV8,
            XBAR49Prediv: XBAR49PredivConf::DIV1,
            XBAR49Findiv: XBAR49FindivConf::Value(1),
            XBAR50: XBAR50Conf::FOUTPOSTDIV8,
            XBAR50Prediv: XBAR50PredivConf::DIV1,
            XBAR50Findiv: XBAR50FindivConf::Value(1),
            XBAR51: XBAR51Conf::HSIRC,
            XBAR51Prediv: XBAR51PredivConf::DIV1,
            XBAR51Findiv: XBAR51FindivConf::Value(1),
            XBAR52: XBAR52Conf::HSIRC,
            XBAR52Prediv: XBAR52PredivConf::DIV1,
            XBAR52Findiv: XBAR52FindivConf::Value(1),
            XBAR53: XBAR53Conf::HSIRC,
            XBAR53Prediv: XBAR53PredivConf::DIV1,
            XBAR53Findiv: XBAR53FindivConf::Value(1),
            XBAR54: XBAR54Conf::FOUTPOSTDIV8,
            XBAR54Prediv: XBAR54PredivConf::DIV1,
            XBAR54Findiv: XBAR54FindivConf::Value(1),
            XBAR55: XBAR55Conf::FOUTPOSTDIV8,
            XBAR55Prediv: XBAR55PredivConf::DIV1,
            XBAR55Findiv: XBAR55FindivConf::Value(1),
            XBAR56: XBAR56Conf::HSIRC,
            XBAR56Prediv: XBAR56PredivConf::DIV1,
            XBAR56Findiv: XBAR56FindivConf::Value(1),
            XBAR57: XBAR57Conf::HSEOSC,
            XBAR57Prediv: XBAR57PredivConf::DIV1,
            XBAR57Findiv: XBAR57FindivConf::Value(1),
            XBAR58: XBAR58Conf::HSEOSC,
            XBAR58Prediv: XBAR58PredivConf::DIV1,
            XBAR58Findiv: XBAR58FindivConf::Value(1),
            XBAR59: XBAR59Conf::FOUTPOSTDIV8,
            XBAR59Prediv: XBAR59PredivConf::DIV1,
            XBAR59Findiv: XBAR59FindivConf::Value(1),
            XBAR60: XBAR60Conf::FOUTPOSTDIV8,
            XBAR60Prediv: XBAR60PredivConf::DIV1,
            XBAR60Findiv: XBAR60FindivConf::Value(1),
            XBAR61: XBAR61Conf::HSIRC,
            XBAR61Prediv: XBAR61PredivConf::DIV1,
            XBAR61Findiv: XBAR61FindivConf::Value(1),
            XBAR62: XBAR62Conf::HSIRC,
            XBAR62Prediv: XBAR62PredivConf::DIV1,
            XBAR62Findiv: XBAR62FindivConf::Value(1),
            XBAR63: XBAR63Conf::HSIRC,
            XBAR63Prediv: XBAR63PredivConf::DIV1,
            XBAR63Findiv: XBAR63FindivConf::Value(1),
            CKINTSEL0: CKINTSEL0Conf::MSIRC,
            CKEXTSEL0: CKEXTSEL0Conf::PLL1Div42,
            CKINTSEL1: CKINTSEL1Conf::MSIRC,
            CKEXTSEL1: CKEXTSEL1Conf::PLL1Div42,
            OBS0: OBS0Conf::CKINTSEL0,
            OBS1: OBS1Conf::CKINTSEL1,
            MCO1Mult: MCO1MultConf::XBAR61Output,
            MCO2Mult: MCO2MultConf::XBAR62Output,
            D3PER: D3PERConf::MSIRC,
            DTS: DTSConf::MSIRC,
            DSIPHY: DSIPHYConf::XBAR28Output,
            DSIBLANE: DSIBLANEConf::DSIPHYOutput,
            USB2PHY1: USB2PHY1Conf::XBAR57Output,
            USB2PHY2: USB2PHY2Conf::XBAR58Output,
            USB3PCIPHY: USB3PCIPHYConf::XBAR34Output,
            SysClkSource: SysClkSourceConf::XBAR0Output,
            MCUDIV: MCUDIVConf::DIV1,
            APB3DIV: APB3DIVConf::DIV1,
            APB4DIV: APB4DIVConf::DIV1,
            APBDBGDIV: APBDBGDIVConf::DIV1,
            APB1DIV: APB1DIVConf::DIV1,
            APB2DIV: APB2DIVConf::DIV1,
            ADC12Mult: ADC12MultConf::XBAR46Output,
            ADC3Mult: ADC3MultConf::XBAR47Output,
            PLL1Source: PLL1SourceConf::HSIRC,
            FREFDIV1: FREFDIV1Conf::Value(1),
            PLL2Source: PLL2SourceConf::HSIRC,
            FREFDIV2: FREFDIV2Conf::Value(1),
            PLL3Source: PLL3SourceConf::HSIRC,
            FREFDIV3: FREFDIV3Conf::Value(1),
            PLL4Source: PLL4SourceConf::HSIRC,
            FREFDIV4: FREFDIV4Conf::Value(1),
            FBDIV1: FBDIV1Conf::Value(25),
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
            PLL5Source: PLL5SourceConf::HSIRC,
            FREFDIV5: FREFDIV5Conf::Value(1),
            FBDIV5: FBDIV5Conf::Value(25),
            PLL5FRACV: PLL5FRACVConf::Value(0),
            POSTDIV1_5: POSTDIV1_5Conf::Value(1),
            POSTDIV2_5: POSTDIV2_5Conf::Value(1),
            PLL6Source: PLL6SourceConf::HSIRC,
            FREFDIV6: FREFDIV6Conf::Value(1),
            FBDIV6: FBDIV6Conf::Value(25),
            PLL6FRACV: PLL6FRACVConf::Value(0),
            POSTDIV1_6: POSTDIV1_6Conf::Value(1),
            POSTDIV2_6: POSTDIV2_6Conf::Value(1),
            PLL7Source: PLL7SourceConf::HSIRC,
            FREFDIV7: FREFDIV7Conf::Value(1),
            FBDIV7: FBDIV7Conf::Value(25),
            PLL7FRACV: PLL7FRACVConf::Value(0),
            POSTDIV1_7: POSTDIV1_7Conf::Value(1),
            POSTDIV2_7: POSTDIV2_7Conf::Value(1),
            PLL8Source: PLL8SourceConf::HSIRC,
            FREFDIV8: FREFDIV8Conf::Value(1),
            FBDIV8: FBDIV8Conf::Value(25),
            PLL8FRACV: PLL8FRACVConf::Value(0),
            POSTDIV1_8: POSTDIV1_8Conf::Value(1),
            POSTDIV2_8: POSTDIV2_8Conf::Value(1),
            HSERTCDevisor: HSERTCDevisorConf::Value(1),
            RTCClkSource: RTCClkSourceConf::LSIRC,
        }
    }
}
impl ClockTree {
    pub fn HSIRC_get(&self) -> Result<f32, ClockError> {
        Ok(64000000 as f32)
    }
    pub fn HSEOSC_get(&self) -> Result<f32, ClockError> {
        self.HSEOSC.get()
    }
    fn HSEDIV2_get(&self) -> Result<f32, ClockError> {
        let input = self.HSEOSC_get()? as f32;
        let value = self.HSEDIV2.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn SPDIF_get(&self) -> Result<f32, ClockError> {
        self.SPDIF.get()
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
    fn XBAR0_get(&self) -> Result<f32, ClockError> {
        match self.XBAR0 {
            XBAR0Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR0Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR0Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR0Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR0Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR0Conf::HSIRC => return self.HSIRC_get(),
            XBAR0Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR0Conf::MSIRC => return self.MSIRC_get(),
            XBAR0Conf::SPDIF => return self.SPDIF_get(),
            XBAR0Conf::I2S_CKIN => return self.I2S_CKIN_get(),
            XBAR0Conf::LSIRC => return self.LSIRC_get(),
            XBAR0Conf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    fn XBAR0Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR0_get()? as f32;
        let value = self.XBAR0Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR0Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR0Prediv_get()? as f32;
        let value = self.XBAR0Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR0Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR0Findiv_get()?;
        if input > (400000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 400000000),
                from: ClockNodes::XBAR0Findiv,
                to: ClockNodes::XBAR0Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR0Findiv,
                to: ClockNodes::XBAR0Output,
            });
        }
        Ok(input)
    }
    fn XBAR1_get(&self) -> Result<f32, ClockError> {
        match self.XBAR1 {
            XBAR1Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR1Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR1Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR1Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR1Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR1Conf::HSIRC => return self.HSIRC_get(),
            XBAR1Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR1Conf::MSIRC => return self.MSIRC_get(),
            XBAR1Conf::SPDIF => return self.SPDIF_get(),
            XBAR1Conf::I2S_CKIN => return self.I2S_CKIN_get(),
            XBAR1Conf::LSIRC => return self.LSIRC_get(),
            XBAR1Conf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    fn XBAR1Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR1_get()? as f32;
        let value = self.XBAR1Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR1Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR1Prediv_get()? as f32;
        let value = self.XBAR1Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR1Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR1Findiv_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
                from: ClockNodes::XBAR1Findiv,
                to: ClockNodes::XBAR1Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR1Findiv,
                to: ClockNodes::XBAR1Output,
            });
        }
        Ok(input)
    }
    fn XBAR2_get(&self) -> Result<f32, ClockError> {
        match self.XBAR2 {
            XBAR2Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR2Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR2Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR2Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR2Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR2Conf::HSIRC => return self.HSIRC_get(),
            XBAR2Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR2Conf::MSIRC => return self.MSIRC_get(),
            XBAR2Conf::SPDIF => return self.SPDIF_get(),
            XBAR2Conf::I2S_CKIN => return self.I2S_CKIN_get(),
            XBAR2Conf::LSIRC => return self.LSIRC_get(),
            XBAR2Conf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    fn XBAR2Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR2_get()? as f32;
        let value = self.XBAR2Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR2Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR2Prediv_get()? as f32;
        let value = self.XBAR2Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR2Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR2Findiv_get()?;
        if input > (600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 600000000),
                from: ClockNodes::XBAR2Findiv,
                to: ClockNodes::XBAR2Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR2Findiv,
                to: ClockNodes::XBAR2Output,
            });
        }
        Ok(input)
    }
    fn XBAR3_get(&self) -> Result<f32, ClockError> {
        match self.XBAR3 {
            XBAR3Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR3Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR3Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR3Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR3Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR3Conf::HSIRC => return self.HSIRC_get(),
            XBAR3Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR3Conf::MSIRC => return self.MSIRC_get(),
            XBAR3Conf::SPDIF => return self.SPDIF_get(),
            XBAR3Conf::I2S_CKIN => return self.I2S_CKIN_get(),
            XBAR3Conf::LSIRC => return self.LSIRC_get(),
            XBAR3Conf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    fn XBAR3Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR3_get()? as f32;
        let value = self.XBAR3Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR3Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR3Prediv_get()? as f32;
        let value = self.XBAR3Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR3Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR3Findiv_get()?;
        if input > (400000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 400000000),
                from: ClockNodes::XBAR3Findiv,
                to: ClockNodes::XBAR3Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR3Findiv,
                to: ClockNodes::XBAR3Output,
            });
        }
        Ok(input)
    }
    fn XBAR4_get(&self) -> Result<f32, ClockError> {
        match self.XBAR4 {
            XBAR4Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR4Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR4Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR4Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR4Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR4Conf::HSIRC => return self.HSIRC_get(),
            XBAR4Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR4Conf::MSIRC => return self.MSIRC_get(),
            XBAR4Conf::SPDIF => return self.SPDIF_get(),
            XBAR4Conf::I2S_CKIN => return self.I2S_CKIN_get(),
            XBAR4Conf::LSIRC => return self.LSIRC_get(),
            XBAR4Conf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    fn XBAR4Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR4_get()? as f32;
        let value = self.XBAR4Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR4Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR4Prediv_get()? as f32;
        let value = self.XBAR4Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR4Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR4Findiv_get()?;
        if input > (300000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 300000000),
                from: ClockNodes::XBAR4Findiv,
                to: ClockNodes::XBAR4Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR4Findiv,
                to: ClockNodes::XBAR4Output,
            });
        }
        Ok(input)
    }
    fn XBAR5_get(&self) -> Result<f32, ClockError> {
        match self.XBAR5 {
            XBAR5Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR5Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR5Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR5Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR5Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR5Conf::HSIRC => return self.HSIRC_get(),
            XBAR5Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR5Conf::MSIRC => return self.MSIRC_get(),
            XBAR5Conf::SPDIF => return self.SPDIF_get(),
            XBAR5Conf::I2S_CKIN => return self.I2S_CKIN_get(),
            XBAR5Conf::LSIRC => return self.LSIRC_get(),
            XBAR5Conf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    fn XBAR5Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR5_get()? as f32;
        let value = self.XBAR5Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR5Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR5Prediv_get()? as f32;
        let value = self.XBAR5Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR5Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR5Findiv_get()?;
        if input > (400000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 400000000),
                from: ClockNodes::XBAR5Findiv,
                to: ClockNodes::XBAR5Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR5Findiv,
                to: ClockNodes::XBAR5Output,
            });
        }
        Ok(input)
    }
    fn XBAR6_get(&self) -> Result<f32, ClockError> {
        match self.XBAR6 {
            XBAR6Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR6Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR6Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR6Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR6Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR6Conf::HSIRC => return self.HSIRC_get(),
            XBAR6Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR6Conf::MSIRC => return self.MSIRC_get(),
            XBAR6Conf::SPDIF => return self.SPDIF_get(),
            XBAR6Conf::I2S_CKIN => return self.I2S_CKIN_get(),
            XBAR6Conf::LSIRC => return self.LSIRC_get(),
            XBAR6Conf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    fn XBAR6Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR6_get()? as f32;
        let value = self.XBAR6Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR6Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR6Prediv_get()? as f32;
        let value = self.XBAR6Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR6Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR6Findiv_get()?;
        if input > (600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 600000000),
                from: ClockNodes::XBAR6Findiv,
                to: ClockNodes::XBAR6Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR6Findiv,
                to: ClockNodes::XBAR6Output,
            });
        }
        Ok(input)
    }
    fn XBAR7_get(&self) -> Result<f32, ClockError> {
        match self.XBAR7 {
            XBAR7Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR7Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR7Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR7Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR7Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR7Conf::LSIRC => return self.LSIRC_get(),
            XBAR7Conf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    fn XBAR7Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR7_get()? as f32;
        let value = self.XBAR7Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR7Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR7Prediv_get()? as f32;
        let value = self.XBAR7Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR7Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR7Findiv_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::XBAR7Findiv,
                to: ClockNodes::XBAR7Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR7Findiv,
                to: ClockNodes::XBAR7Output,
            });
        }
        Ok(input)
    }
    fn XBAR8_get(&self) -> Result<f32, ClockError> {
        match self.XBAR8 {
            XBAR8Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR8Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR8Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR8Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR8Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR8Conf::HSIRC => return self.HSIRC_get(),
            XBAR8Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR8Conf::MSIRC => return self.MSIRC_get(),
        };
    }
    fn XBAR8Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR8_get()? as f32;
        let value = self.XBAR8Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR8Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR8Prediv_get()? as f32;
        let value = self.XBAR8Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR8Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR8Findiv_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::XBAR8Findiv,
                to: ClockNodes::XBAR8Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR8Findiv,
                to: ClockNodes::XBAR8Output,
            });
        }
        Ok(input)
    }
    fn XBAR9_get(&self) -> Result<f32, ClockError> {
        match self.XBAR9 {
            XBAR9Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR9Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR9Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR9Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR9Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR9Conf::HSIRC => return self.HSIRC_get(),
            XBAR9Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR9Conf::MSIRC => return self.MSIRC_get(),
        };
    }
    fn XBAR9Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR9_get()? as f32;
        let value = self.XBAR9Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR9Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR9Prediv_get()? as f32;
        let value = self.XBAR9Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR9Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR9Findiv_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::XBAR9Findiv,
                to: ClockNodes::XBAR9Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR9Findiv,
                to: ClockNodes::XBAR9Output,
            });
        }
        Ok(input)
    }
    fn XBAR10_get(&self) -> Result<f32, ClockError> {
        match self.XBAR10 {
            XBAR10Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR10Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR10Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR10Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR10Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR10Conf::HSIRC => return self.HSIRC_get(),
            XBAR10Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR10Conf::MSIRC => return self.MSIRC_get(),
            XBAR10Conf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    fn XBAR10Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR10_get()? as f32;
        let value = self.XBAR10Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR10Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR10Prediv_get()? as f32;
        let value = self.XBAR10Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR10Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR10Findiv_get()?;
        if input > (133000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133000000),
                from: ClockNodes::XBAR10Findiv,
                to: ClockNodes::XBAR10Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR10Findiv,
                to: ClockNodes::XBAR10Output,
            });
        }
        Ok(input)
    }
    fn XBAR11_get(&self) -> Result<f32, ClockError> {
        match self.XBAR11 {
            XBAR11Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR11Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR11Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR11Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR11Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR11Conf::HSIRC => return self.HSIRC_get(),
        };
    }
    fn XBAR11Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR11_get()? as f32;
        let value = self.XBAR11Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR11Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR11Prediv_get()? as f32;
        let value = self.XBAR11Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR11Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR11Findiv_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
                from: ClockNodes::XBAR11Findiv,
                to: ClockNodes::XBAR11Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR11Findiv,
                to: ClockNodes::XBAR11Output,
            });
        }
        Ok(input)
    }
    fn XBAR12_get(&self) -> Result<f32, ClockError> {
        match self.XBAR12 {
            XBAR12Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR12Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR12Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR12Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR12Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR12Conf::HSIRC => return self.HSIRC_get(),
            XBAR12Conf::MSIRC => return self.MSIRC_get(),
        };
    }
    fn XBAR12Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR12_get()? as f32;
        let value = self.XBAR12Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR12Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR12Prediv_get()? as f32;
        let value = self.XBAR12Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR12Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR12Findiv_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
                from: ClockNodes::XBAR12Findiv,
                to: ClockNodes::XBAR12Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR12Findiv,
                to: ClockNodes::XBAR12Output,
            });
        }
        Ok(input)
    }
    fn XBAR13_get(&self) -> Result<f32, ClockError> {
        match self.XBAR13 {
            XBAR13Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR13Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR13Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR13Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR13Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR13Conf::HSIRC => return self.HSIRC_get(),
            XBAR13Conf::MSIRC => return self.MSIRC_get(),
        };
    }
    fn XBAR13Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR13_get()? as f32;
        let value = self.XBAR13Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR13Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR13Prediv_get()? as f32;
        let value = self.XBAR13Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR13Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR13Findiv_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::XBAR13Findiv,
                to: ClockNodes::XBAR13Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR13Findiv,
                to: ClockNodes::XBAR13Output,
            });
        }
        Ok(input)
    }
    fn XBAR14_get(&self) -> Result<f32, ClockError> {
        match self.XBAR14 {
            XBAR14Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR14Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR14Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR14Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR14Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR14Conf::HSIRC => return self.HSIRC_get(),
            XBAR14Conf::MSIRC => return self.MSIRC_get(),
        };
    }
    fn XBAR14Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR14_get()? as f32;
        let value = self.XBAR14Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR14Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR14Prediv_get()? as f32;
        let value = self.XBAR14Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR14Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR14Findiv_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::XBAR14Findiv,
                to: ClockNodes::XBAR14Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR14Findiv,
                to: ClockNodes::XBAR14Output,
            });
        }
        Ok(input)
    }
    fn XBAR15_get(&self) -> Result<f32, ClockError> {
        match self.XBAR15 {
            XBAR15Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR15Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR15Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR15Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR15Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR15Conf::HSIRC => return self.HSIRC_get(),
            XBAR15Conf::MSIRC => return self.MSIRC_get(),
        };
    }
    fn XBAR15Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR15_get()? as f32;
        let value = self.XBAR15Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR15Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR15Prediv_get()? as f32;
        let value = self.XBAR15Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR15Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR15Findiv_get()?;
        if input > (133000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133000000),
                from: ClockNodes::XBAR15Findiv,
                to: ClockNodes::XBAR15Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR15Findiv,
                to: ClockNodes::XBAR15Output,
            });
        }
        Ok(input)
    }
    fn XBAR16_get(&self) -> Result<f32, ClockError> {
        match self.XBAR16 {
            XBAR16Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR16Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR16Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR16Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR16Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR16Conf::HSIRC => return self.HSIRC_get(),
            XBAR16Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR16Conf::MSIRC => return self.MSIRC_get(),
            XBAR16Conf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    fn XBAR16Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR16_get()? as f32;
        let value = self.XBAR16Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR16Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR16Prediv_get()? as f32;
        let value = self.XBAR16Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR16Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR16Findiv_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
                from: ClockNodes::XBAR16Findiv,
                to: ClockNodes::XBAR16Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR16Findiv,
                to: ClockNodes::XBAR16Output,
            });
        }
        Ok(input)
    }
    fn XBAR17_get(&self) -> Result<f32, ClockError> {
        match self.XBAR17 {
            XBAR17Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR17Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR17Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR17Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR17Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR17Conf::HSIRC => return self.HSIRC_get(),
            XBAR17Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR17Conf::MSIRC => return self.MSIRC_get(),
            XBAR17Conf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    fn XBAR17Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR17_get()? as f32;
        let value = self.XBAR17Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR17Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR17Prediv_get()? as f32;
        let value = self.XBAR17Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR17Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR17Findiv_get()?;
        if input > (133000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133000000),
                from: ClockNodes::XBAR17Findiv,
                to: ClockNodes::XBAR17Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR17Findiv,
                to: ClockNodes::XBAR17Output,
            });
        }
        Ok(input)
    }
    fn XBAR18_get(&self) -> Result<f32, ClockError> {
        match self.XBAR18 {
            XBAR18Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR18Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR18Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR18Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR18Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR18Conf::HSIRC => return self.HSIRC_get(),
            XBAR18Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR18Conf::MSIRC => return self.MSIRC_get(),
            XBAR18Conf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    fn XBAR18Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR18_get()? as f32;
        let value = self.XBAR18Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR18Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR18Prediv_get()? as f32;
        let value = self.XBAR18Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR18Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR18Findiv_get()?;
        if input > (133000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133000000),
                from: ClockNodes::XBAR18Findiv,
                to: ClockNodes::XBAR18Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR18Findiv,
                to: ClockNodes::XBAR18Output,
            });
        }
        Ok(input)
    }
    fn XBAR19_get(&self) -> Result<f32, ClockError> {
        match self.XBAR19 {
            XBAR19Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR19Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR19Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR19Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR19Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR19Conf::HSIRC => return self.HSIRC_get(),
            XBAR19Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR19Conf::MSIRC => return self.MSIRC_get(),
        };
    }
    fn XBAR19Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR19_get()? as f32;
        let value = self.XBAR19Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR19Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR19Prediv_get()? as f32;
        let value = self.XBAR19Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR19Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR19Findiv_get()?;
        if input > (133000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133000000),
                from: ClockNodes::XBAR19Findiv,
                to: ClockNodes::XBAR19Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR19Findiv,
                to: ClockNodes::XBAR19Output,
            });
        }
        Ok(input)
    }
    fn XBAR20_get(&self) -> Result<f32, ClockError> {
        match self.XBAR20 {
            XBAR20Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR20Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR20Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR20Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR20Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR20Conf::HSIRC => return self.HSIRC_get(),
            XBAR20Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR20Conf::MSIRC => return self.MSIRC_get(),
        };
    }
    fn XBAR20Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR20_get()? as f32;
        let value = self.XBAR20Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR20Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR20Prediv_get()? as f32;
        let value = self.XBAR20Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR20Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR20Findiv_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::XBAR20Findiv,
                to: ClockNodes::XBAR20Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR20Findiv,
                to: ClockNodes::XBAR20Output,
            });
        }
        Ok(input)
    }
    fn XBAR21_get(&self) -> Result<f32, ClockError> {
        match self.XBAR21 {
            XBAR21Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR21Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR21Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR21Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR21Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR21Conf::HSIRC => return self.HSIRC_get(),
            XBAR21Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR21Conf::MSIRC => return self.MSIRC_get(),
        };
    }
    fn XBAR21Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR21_get()? as f32;
        let value = self.XBAR21Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR21Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR21Prediv_get()? as f32;
        let value = self.XBAR21Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR21Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR21Findiv_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::XBAR21Findiv,
                to: ClockNodes::XBAR21Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR21Findiv,
                to: ClockNodes::XBAR21Output,
            });
        }
        Ok(input)
    }
    fn XBAR22_get(&self) -> Result<f32, ClockError> {
        match self.XBAR22 {
            XBAR22Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR22Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR22Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR22Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR22Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR22Conf::HSIRC => return self.HSIRC_get(),
            XBAR22Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR22Conf::MSIRC => return self.MSIRC_get(),
        };
    }
    fn XBAR22Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR22_get()? as f32;
        let value = self.XBAR22Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR22Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR22Prediv_get()? as f32;
        let value = self.XBAR22Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR22Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR22Findiv_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::XBAR22Findiv,
                to: ClockNodes::XBAR22Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR22Findiv,
                to: ClockNodes::XBAR22Output,
            });
        }
        Ok(input)
    }
    fn XBAR23_get(&self) -> Result<f32, ClockError> {
        match self.XBAR23 {
            XBAR23Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR23Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR23Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR23Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR23Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR23Conf::HSIRC => return self.HSIRC_get(),
            XBAR23Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR23Conf::MSIRC => return self.MSIRC_get(),
            XBAR23Conf::SPDIF => return self.SPDIF_get(),
            XBAR23Conf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    fn XBAR23Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR23_get()? as f32;
        let value = self.XBAR23Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR23Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR23Prediv_get()? as f32;
        let value = self.XBAR23Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR23Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR23Findiv_get()?;
        if input > (133000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133000000),
                from: ClockNodes::XBAR23Findiv,
                to: ClockNodes::XBAR23Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR23Findiv,
                to: ClockNodes::XBAR23Output,
            });
        }
        Ok(input)
    }
    fn XBAR24_get(&self) -> Result<f32, ClockError> {
        match self.XBAR24 {
            XBAR24Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR24Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR24Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR24Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR24Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR24Conf::HSIRC => return self.HSIRC_get(),
            XBAR24Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR24Conf::MSIRC => return self.MSIRC_get(),
            XBAR24Conf::SPDIF => return self.SPDIF_get(),
            XBAR24Conf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    fn XBAR24Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR24_get()? as f32;
        let value = self.XBAR24Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR24Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR24Prediv_get()? as f32;
        let value = self.XBAR24Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR24Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR24Findiv_get()?;
        if input > (133000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133000000),
                from: ClockNodes::XBAR24Findiv,
                to: ClockNodes::XBAR24Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR24Findiv,
                to: ClockNodes::XBAR24Output,
            });
        }
        Ok(input)
    }
    fn XBAR25_get(&self) -> Result<f32, ClockError> {
        match self.XBAR25 {
            XBAR25Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR25Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR25Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR25Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR25Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR25Conf::HSIRC => return self.HSIRC_get(),
            XBAR25Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR25Conf::MSIRC => return self.MSIRC_get(),
            XBAR25Conf::SPDIF => return self.SPDIF_get(),
            XBAR25Conf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    fn XBAR25Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR25_get()? as f32;
        let value = self.XBAR25Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR25Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR25Prediv_get()? as f32;
        let value = self.XBAR25Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR25Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR25Findiv_get()?;
        if input > (133000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133000000),
                from: ClockNodes::XBAR25Findiv,
                to: ClockNodes::XBAR25Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR25Findiv,
                to: ClockNodes::XBAR25Output,
            });
        }
        Ok(input)
    }
    fn XBAR26_get(&self) -> Result<f32, ClockError> {
        match self.XBAR26 {
            XBAR26Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR26Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR26Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR26Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR26Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR26Conf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    fn XBAR26Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR26_get()? as f32;
        let value = self.XBAR26Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR26Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR26Prediv_get()? as f32;
        let value = self.XBAR26Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR26Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR26Findiv_get()?;
        if input > (133000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133000000),
                from: ClockNodes::XBAR26Findiv,
                to: ClockNodes::XBAR26Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR26Findiv,
                to: ClockNodes::XBAR26Output,
            });
        }
        Ok(input)
    }
    fn XBAR27_get(&self) -> Result<f32, ClockError> {
        match self.XBAR27 {
            XBAR27Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR27Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR27Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR27Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR27Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
        };
    }
    fn XBAR27Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR27_get()? as f32;
        let value = self.XBAR27Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR27Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR27Prediv_get()? as f32;
        let value = self.XBAR27Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR27Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR27Findiv_get()?;
        if input > (314000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 314000000),
                from: ClockNodes::XBAR27Findiv,
                to: ClockNodes::XBAR27Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR27Findiv,
                to: ClockNodes::XBAR27Output,
            });
        }
        Ok(input)
    }
    fn XBAR28_get(&self) -> Result<f32, ClockError> {
        match self.XBAR28 {
            XBAR28Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR28Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR28Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR28Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR28Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR28Conf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    fn XBAR28Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR28_get()? as f32;
        let value = self.XBAR28Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR28Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR28Prediv_get()? as f32;
        let value = self.XBAR28Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR28Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR28Findiv_get()?;
        if input > (27000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 27000000),
                from: ClockNodes::XBAR28Findiv,
                to: ClockNodes::XBAR28Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR28Findiv,
                to: ClockNodes::XBAR28Output,
            });
        }
        Ok(input)
    }
    fn XBAR29_get(&self) -> Result<f32, ClockError> {
        match self.XBAR29 {
            XBAR29Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR29Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR29Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR29Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR29Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
        };
    }
    fn XBAR29Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR29_get()? as f32;
        let value = self.XBAR29Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR29Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR29Prediv_get()? as f32;
        let value = self.XBAR29Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR29Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR29Findiv_get()?;
        if input > (333000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 333000000),
                from: ClockNodes::XBAR29Findiv,
                to: ClockNodes::XBAR29Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR29Findiv,
                to: ClockNodes::XBAR29Output,
            });
        }
        Ok(input)
    }
    fn XBAR30_get(&self) -> Result<f32, ClockError> {
        match self.XBAR30 {
            XBAR30Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR30Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR30Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR30Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR30Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR30Conf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    fn XBAR30Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR30_get()? as f32;
        let value = self.XBAR30Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR30Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR30Prediv_get()? as f32;
        let value = self.XBAR30Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR30Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR30Findiv_get()?;
        if input > (20000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 20000000),
                from: ClockNodes::XBAR30Findiv,
                to: ClockNodes::XBAR30Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR30Findiv,
                to: ClockNodes::XBAR30Output,
            });
        }
        Ok(input)
    }
    fn XBAR31_get(&self) -> Result<f32, ClockError> {
        match self.XBAR31 {
            XBAR31Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR31Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR31Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR31Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR31Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR31Conf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    fn XBAR31Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR31_get()? as f32;
        let value = self.XBAR31Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR31Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR31Prediv_get()? as f32;
        let value = self.XBAR31Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR31Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR31Findiv_get()?;
        if input > (27000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 27000000),
                from: ClockNodes::XBAR31Findiv,
                to: ClockNodes::XBAR31Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR31Findiv,
                to: ClockNodes::XBAR31Output,
            });
        }
        Ok(input)
    }
    fn XBAR32_get(&self) -> Result<f32, ClockError> {
        match self.XBAR32 {
            XBAR32Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR32Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR32Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR32Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR32Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR32Conf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    fn XBAR32Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR32_get()? as f32;
        let value = self.XBAR32Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR32Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR32Prediv_get()? as f32;
        let value = self.XBAR32Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR32Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR32Findiv_get()?;
        if input > (27000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 27000000),
                from: ClockNodes::XBAR32Findiv,
                to: ClockNodes::XBAR32Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR32Findiv,
                to: ClockNodes::XBAR32Output,
            });
        }
        Ok(input)
    }
    fn XBAR33_get(&self) -> Result<f32, ClockError> {
        match self.XBAR33 {
            XBAR33Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR33Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR33Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR33Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR33Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR33Conf::HSIRC => return self.HSIRC_get(),
            XBAR33Conf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    fn XBAR33Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR33_get()? as f32;
        let value = self.XBAR33Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR33Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR33Prediv_get()? as f32;
        let value = self.XBAR33Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR33Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR33Findiv_get()?;
        if input > (64000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 64000000),
                from: ClockNodes::XBAR33Findiv,
                to: ClockNodes::XBAR33Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR33Findiv,
                to: ClockNodes::XBAR33Output,
            });
        }
        Ok(input)
    }
    fn XBAR34_get(&self) -> Result<f32, ClockError> {
        match self.XBAR34 {
            XBAR34Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR34Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR34Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR34Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR34Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR34Conf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    fn XBAR34Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR34_get()? as f32;
        let value = self.XBAR34Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR34Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR34Prediv_get()? as f32;
        let value = self.XBAR34Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR34Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR34Findiv_get()?;
        if input > (25000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 25000000),
                from: ClockNodes::XBAR34Findiv,
                to: ClockNodes::XBAR34Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR34Findiv,
                to: ClockNodes::XBAR34Output,
            });
        }
        Ok(input)
    }
    fn XBAR35_get(&self) -> Result<f32, ClockError> {
        match self.XBAR35 {
            XBAR35Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR35Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR35Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR35Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR35Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR35Conf::HSIRC => return self.HSIRC_get(),
            XBAR35Conf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    fn XBAR35Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR35_get()? as f32;
        let value = self.XBAR35Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR35Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR35Prediv_get()? as f32;
        let value = self.XBAR35Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR35Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR35Findiv_get()?;
        if input > (64000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 64000000),
                from: ClockNodes::XBAR35Findiv,
                to: ClockNodes::XBAR35Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR35Findiv,
                to: ClockNodes::XBAR35Output,
            });
        }
        Ok(input)
    }
    fn XBAR36_get(&self) -> Result<f32, ClockError> {
        match self.XBAR36 {
            XBAR36Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR36Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR36Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR36Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR36Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR36Conf::HSIRC => return self.HSIRC_get(),
            XBAR36Conf::MSIRC => return self.MSIRC_get(),
        };
    }
    fn XBAR36Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR36_get()? as f32;
        let value = self.XBAR36Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR36Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR36Prediv_get()? as f32;
        let value = self.XBAR36Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR36Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR36Findiv_get()?;
        if input > (133000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133000000),
                from: ClockNodes::XBAR36Findiv,
                to: ClockNodes::XBAR36Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR36Findiv,
                to: ClockNodes::XBAR36Output,
            });
        }
        Ok(input)
    }
    fn XBAR37_get(&self) -> Result<f32, ClockError> {
        match self.XBAR37 {
            XBAR37Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR37Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR37Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR37Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR37Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR37Conf::HSIRC => return self.HSIRC_get(),
            XBAR37Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR37Conf::MSIRC => return self.MSIRC_get(),
            XBAR37Conf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    fn XBAR37Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR37_get()? as f32;
        let value = self.XBAR37Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR37Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR37Prediv_get()? as f32;
        let value = self.XBAR37Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR37Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR37Findiv_get()?;
        if input > (133000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133000000),
                from: ClockNodes::XBAR37Findiv,
                to: ClockNodes::XBAR37Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR37Findiv,
                to: ClockNodes::XBAR37Output,
            });
        }
        Ok(input)
    }
    fn XBAR38_get(&self) -> Result<f32, ClockError> {
        match self.XBAR38 {
            XBAR38Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR38Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR38Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR38Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR38Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR38Conf::HSIRC => return self.HSIRC_get(),
            XBAR38Conf::MSIRC => return self.MSIRC_get(),
        };
    }
    fn XBAR38Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR38_get()? as f32;
        let value = self.XBAR38Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR38Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR38Prediv_get()? as f32;
        let value = self.XBAR38Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR38Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR38Findiv_get()?;
        if input > (133000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133000000),
                from: ClockNodes::XBAR38Findiv,
                to: ClockNodes::XBAR38Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR38Findiv,
                to: ClockNodes::XBAR38Output,
            });
        }
        Ok(input)
    }
    fn XBAR39_get(&self) -> Result<f32, ClockError> {
        match self.XBAR39 {
            XBAR39Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR39Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR39Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR39Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR39Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR39Conf::HSIRC => return self.HSIRC_get(),
            XBAR39Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR39Conf::MSIRC => return self.MSIRC_get(),
        };
    }
    fn XBAR39Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR39_get()? as f32;
        let value = self.XBAR39Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR39Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR39Prediv_get()? as f32;
        let value = self.XBAR39Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR39Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR39Findiv_get()?;
        if input > (133000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133000000),
                from: ClockNodes::XBAR39Findiv,
                to: ClockNodes::XBAR39Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR39Findiv,
                to: ClockNodes::XBAR39Output,
            });
        }
        Ok(input)
    }
    fn XBAR40_get(&self) -> Result<f32, ClockError> {
        match self.XBAR40 {
            XBAR40Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR40Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR40Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR40Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR40Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR40Conf::LSIRC => return self.LSIRC_get(),
            XBAR40Conf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    fn XBAR40Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR40_get()? as f32;
        let value = self.XBAR40Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR40Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR40Prediv_get()? as f32;
        let value = self.XBAR40Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR40Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR40Findiv_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::XBAR40Findiv,
                to: ClockNodes::XBAR40Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR40Findiv,
                to: ClockNodes::XBAR40Output,
            });
        }
        Ok(input)
    }
    fn XBAR41_get(&self) -> Result<f32, ClockError> {
        match self.XBAR41 {
            XBAR41Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR41Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR41Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR41Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR41Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR41Conf::LSIRC => return self.LSIRC_get(),
            XBAR41Conf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    fn XBAR41Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR41_get()? as f32;
        let value = self.XBAR41Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR41Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR41Prediv_get()? as f32;
        let value = self.XBAR41Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR41Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR41Findiv_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::XBAR41Findiv,
                to: ClockNodes::XBAR41Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR41Findiv,
                to: ClockNodes::XBAR41Output,
            });
        }
        Ok(input)
    }
    fn XBAR42_get(&self) -> Result<f32, ClockError> {
        match self.XBAR42 {
            XBAR42Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR42Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR42Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR42Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR42Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR42Conf::HSIRC => return self.HSIRC_get(),
            XBAR42Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR42Conf::MSIRC => return self.MSIRC_get(),
        };
    }
    fn XBAR42Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR42_get()? as f32;
        let value = self.XBAR42Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR42Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR42Prediv_get()? as f32;
        let value = self.XBAR42Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR42Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR42Findiv_get()?;
        if input > (133000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133000000),
                from: ClockNodes::XBAR42Findiv,
                to: ClockNodes::XBAR42Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR42Findiv,
                to: ClockNodes::XBAR42Output,
            });
        }
        Ok(input)
    }
    fn XBAR43_get(&self) -> Result<f32, ClockError> {
        match self.XBAR43 {
            XBAR43Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR43Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR43Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR43Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR43Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
        };
    }
    fn XBAR43Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR43_get()? as f32;
        let value = self.XBAR43Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR43Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR43Prediv_get()? as f32;
        let value = self.XBAR43Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR43Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR43Findiv_get()?;
        if input > (64000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 64000000),
                from: ClockNodes::XBAR43Findiv,
                to: ClockNodes::XBAR43Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR43Findiv,
                to: ClockNodes::XBAR43Output,
            });
        }
        Ok(input)
    }
    fn XBAR44_get(&self) -> Result<f32, ClockError> {
        match self.XBAR44 {
            XBAR44Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR44Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR44Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR44Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR44Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
        };
    }
    fn XBAR44Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR44_get()? as f32;
        let value = self.XBAR44Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR44Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR44Prediv_get()? as f32;
        let value = self.XBAR44Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR44Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR44Findiv_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
                from: ClockNodes::XBAR44Findiv,
                to: ClockNodes::XBAR44Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR44Findiv,
                to: ClockNodes::XBAR44Output,
            });
        }
        Ok(input)
    }
    fn XBAR45_get(&self) -> Result<f32, ClockError> {
        match self.XBAR45 {
            XBAR45Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR45Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR45Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR45Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR45Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
        };
    }
    fn XBAR45Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR45_get()? as f32;
        let value = self.XBAR45Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR45Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR45Prediv_get()? as f32;
        let value = self.XBAR45Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR45Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR45Findiv_get()?;
        if input > (400000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 400000000),
                from: ClockNodes::XBAR45Findiv,
                to: ClockNodes::XBAR45Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR45Findiv,
                to: ClockNodes::XBAR45Output,
            });
        }
        Ok(input)
    }
    fn XBAR46_get(&self) -> Result<f32, ClockError> {
        match self.XBAR46 {
            XBAR46Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR46Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR46Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR46Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR46Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR46Conf::HSIRC => return self.HSIRC_get(),
            XBAR46Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR46Conf::MSIRC => return self.MSIRC_get(),
        };
    }
    fn XBAR46Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR46_get()? as f32;
        let value = self.XBAR46Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR46Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR46Prediv_get()? as f32;
        let value = self.XBAR46Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR46Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR46Findiv_get()?;
        if input > (133000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133000000),
                from: ClockNodes::XBAR46Findiv,
                to: ClockNodes::XBAR46Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR46Findiv,
                to: ClockNodes::XBAR46Output,
            });
        }
        Ok(input)
    }
    fn XBAR47_get(&self) -> Result<f32, ClockError> {
        match self.XBAR47 {
            XBAR47Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR47Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR47Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR47Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR47Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
        };
    }
    fn XBAR47Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR47_get()? as f32;
        let value = self.XBAR47Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR47Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR47Prediv_get()? as f32;
        let value = self.XBAR47Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR47Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR47Findiv_get()?;
        if input > (133000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133000000),
                from: ClockNodes::XBAR47Findiv,
                to: ClockNodes::XBAR47Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR47Findiv,
                to: ClockNodes::XBAR47Output,
            });
        }
        Ok(input)
    }
    fn XBAR48_get(&self) -> Result<f32, ClockError> {
        match self.XBAR48 {
            XBAR48Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR48Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR48Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR48Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR48Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
        };
    }
    fn XBAR48Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR48_get()? as f32;
        let value = self.XBAR48Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR48Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR48Prediv_get()? as f32;
        let value = self.XBAR48Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR48Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR48Findiv_get()?;
        if input > (133000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133000000),
                from: ClockNodes::XBAR48Findiv,
                to: ClockNodes::XBAR48Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR48Findiv,
                to: ClockNodes::XBAR48Output,
            });
        }
        Ok(input)
    }
    fn XBAR49_get(&self) -> Result<f32, ClockError> {
        match self.XBAR49 {
            XBAR49Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR49Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR49Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR49Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR49Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
        };
    }
    fn XBAR49Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR49_get()? as f32;
        let value = self.XBAR49Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR49Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR49Prediv_get()? as f32;
        let value = self.XBAR49Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR49Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR49Findiv_get()?;
        if input > (133000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133000000),
                from: ClockNodes::XBAR49Findiv,
                to: ClockNodes::XBAR49Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR49Findiv,
                to: ClockNodes::XBAR49Output,
            });
        }
        Ok(input)
    }
    fn XBAR50_get(&self) -> Result<f32, ClockError> {
        match self.XBAR50 {
            XBAR50Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR50Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR50Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR50Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR50Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
        };
    }
    fn XBAR50Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR50_get()? as f32;
        let value = self.XBAR50Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR50Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR50Prediv_get()? as f32;
        let value = self.XBAR50Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR50Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR50Findiv_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
                from: ClockNodes::XBAR50Findiv,
                to: ClockNodes::XBAR50Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR50Findiv,
                to: ClockNodes::XBAR50Output,
            });
        }
        Ok(input)
    }
    fn XBAR51_get(&self) -> Result<f32, ClockError> {
        match self.XBAR51 {
            XBAR51Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR51Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR51Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR51Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR51Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR51Conf::HSIRC => return self.HSIRC_get(),
        };
    }
    fn XBAR51Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR51_get()? as f32;
        let value = self.XBAR51Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR51Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR51Prediv_get()? as f32;
        let value = self.XBAR51Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR51Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR51Findiv_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
                from: ClockNodes::XBAR51Findiv,
                to: ClockNodes::XBAR51Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR51Findiv,
                to: ClockNodes::XBAR51Output,
            });
        }
        Ok(input)
    }
    fn XBAR52_get(&self) -> Result<f32, ClockError> {
        match self.XBAR52 {
            XBAR52Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR52Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR52Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR52Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR52Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR52Conf::HSIRC => return self.HSIRC_get(),
        };
    }
    fn XBAR52Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR52_get()? as f32;
        let value = self.XBAR52Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR52Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR52Prediv_get()? as f32;
        let value = self.XBAR52Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR52Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR52Findiv_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
                from: ClockNodes::XBAR52Findiv,
                to: ClockNodes::XBAR52Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR52Findiv,
                to: ClockNodes::XBAR52Output,
            });
        }
        Ok(input)
    }
    fn XBAR53_get(&self) -> Result<f32, ClockError> {
        match self.XBAR53 {
            XBAR53Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR53Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR53Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR53Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR53Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR53Conf::HSIRC => return self.HSIRC_get(),
        };
    }
    fn XBAR53Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR53_get()? as f32;
        let value = self.XBAR53Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR53Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR53Prediv_get()? as f32;
        let value = self.XBAR53Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR53Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR53Findiv_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
                from: ClockNodes::XBAR53Findiv,
                to: ClockNodes::XBAR53Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR53Findiv,
                to: ClockNodes::XBAR53Output,
            });
        }
        Ok(input)
    }
    fn XBAR54_get(&self) -> Result<f32, ClockError> {
        match self.XBAR54 {
            XBAR54Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR54Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR54Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR54Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR54Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
        };
    }
    fn XBAR54Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR54_get()? as f32;
        let value = self.XBAR54Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR54Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR54Prediv_get()? as f32;
        let value = self.XBAR54Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR54Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR54Findiv_get()?;
        if input > (125000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 125000000),
                from: ClockNodes::XBAR54Findiv,
                to: ClockNodes::XBAR54Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR54Findiv,
                to: ClockNodes::XBAR54Output,
            });
        }
        Ok(input)
    }
    fn XBAR55_get(&self) -> Result<f32, ClockError> {
        match self.XBAR55 {
            XBAR55Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR55Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR55Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR55Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR55Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
        };
    }
    fn XBAR55Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR55_get()? as f32;
        let value = self.XBAR55Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR55Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR55Prediv_get()? as f32;
        let value = self.XBAR55Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR55Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR55Findiv_get()?;
        if input > (125000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 125000000),
                from: ClockNodes::XBAR55Findiv,
                to: ClockNodes::XBAR55Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR55Findiv,
                to: ClockNodes::XBAR55Output,
            });
        }
        Ok(input)
    }
    fn XBAR56_get(&self) -> Result<f32, ClockError> {
        match self.XBAR56 {
            XBAR56Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR56Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR56Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR56Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR56Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR56Conf::HSIRC => return self.HSIRC_get(),
            XBAR56Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR56Conf::MSIRC => return self.MSIRC_get(),
        };
    }
    fn XBAR56Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR56_get()? as f32;
        let value = self.XBAR56Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR56Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR56Prediv_get()? as f32;
        let value = self.XBAR56Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR56Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR56Findiv_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
                from: ClockNodes::XBAR56Findiv,
                to: ClockNodes::XBAR56Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR56Findiv,
                to: ClockNodes::XBAR56Output,
            });
        }
        Ok(input)
    }
    fn XBAR57_get(&self) -> Result<f32, ClockError> {
        match self.XBAR57 {
            XBAR57Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR57Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR57Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR57Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR57Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR57Conf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    fn XBAR57Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR57_get()? as f32;
        let value = self.XBAR57Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR57Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR57Prediv_get()? as f32;
        let value = self.XBAR57Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR57Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR57Findiv_get()?;
        if input > (24000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 24000000),
                from: ClockNodes::XBAR57Findiv,
                to: ClockNodes::XBAR57Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR57Findiv,
                to: ClockNodes::XBAR57Output,
            });
        }
        Ok(input)
    }
    fn XBAR58_get(&self) -> Result<f32, ClockError> {
        match self.XBAR58 {
            XBAR58Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR58Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR58Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR58Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR58Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR58Conf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    fn XBAR58Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR58_get()? as f32;
        let value = self.XBAR58Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR58Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR58Prediv_get()? as f32;
        let value = self.XBAR58Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR58Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR58Findiv_get()?;
        if input > (24000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 24000000),
                from: ClockNodes::XBAR58Findiv,
                to: ClockNodes::XBAR58Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR58Findiv,
                to: ClockNodes::XBAR58Output,
            });
        }
        Ok(input)
    }
    fn XBAR59_get(&self) -> Result<f32, ClockError> {
        match self.XBAR59 {
            XBAR59Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR59Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR59Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR59Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR59Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
        };
    }
    fn XBAR59Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR59_get()? as f32;
        let value = self.XBAR59Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR59Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR59Prediv_get()? as f32;
        let value = self.XBAR59Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR59Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR59Findiv_get()?;
        if input > (600000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 600000000),
                from: ClockNodes::XBAR59Findiv,
                to: ClockNodes::XBAR59Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR59Findiv,
                to: ClockNodes::XBAR59Output,
            });
        }
        Ok(input)
    }
    fn XBAR60_get(&self) -> Result<f32, ClockError> {
        match self.XBAR60 {
            XBAR60Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR60Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR60Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR60Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR60Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
        };
    }
    fn XBAR60Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR60_get()? as f32;
        let value = self.XBAR60Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR60Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR60Prediv_get()? as f32;
        let value = self.XBAR60Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR60Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR60Findiv_get()?;
        if input > (50000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 50000000),
                from: ClockNodes::XBAR60Findiv,
                to: ClockNodes::XBAR60Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR60Findiv,
                to: ClockNodes::XBAR60Output,
            });
        }
        Ok(input)
    }
    fn XBAR61_get(&self) -> Result<f32, ClockError> {
        match self.XBAR61 {
            XBAR61Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR61Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR61Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR61Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR61Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR61Conf::HSIRC => return self.HSIRC_get(),
            XBAR61Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR61Conf::MSIRC => return self.MSIRC_get(),
            XBAR61Conf::SPDIF => return self.SPDIF_get(),
            XBAR61Conf::I2S_CKIN => return self.I2S_CKIN_get(),
            XBAR61Conf::LSIRC => return self.LSIRC_get(),
            XBAR61Conf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    fn XBAR61Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR61_get()? as f32;
        let value = self.XBAR61Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR61Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR61Prediv_get()? as f32;
        let value = self.XBAR61Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR61Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR61Findiv_get()?;
        if input > (160000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 160000000),
                from: ClockNodes::XBAR61Findiv,
                to: ClockNodes::XBAR61Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR61Findiv,
                to: ClockNodes::XBAR61Output,
            });
        }
        Ok(input)
    }
    fn XBAR62_get(&self) -> Result<f32, ClockError> {
        match self.XBAR62 {
            XBAR62Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR62Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR62Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR62Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR62Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR62Conf::HSIRC => return self.HSIRC_get(),
            XBAR62Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR62Conf::MSIRC => return self.MSIRC_get(),
            XBAR62Conf::SPDIF => return self.SPDIF_get(),
            XBAR62Conf::I2S_CKIN => return self.I2S_CKIN_get(),
            XBAR62Conf::LSIRC => return self.LSIRC_get(),
            XBAR62Conf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    fn XBAR62Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR62_get()? as f32;
        let value = self.XBAR62Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR62Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR62Prediv_get()? as f32;
        let value = self.XBAR62Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR62Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR62Findiv_get()?;
        if input > (160000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 160000000),
                from: ClockNodes::XBAR62Findiv,
                to: ClockNodes::XBAR62Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR62Findiv,
                to: ClockNodes::XBAR62Output,
            });
        }
        Ok(input)
    }
    fn XBAR63_get(&self) -> Result<f32, ClockError> {
        match self.XBAR63 {
            XBAR63Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            XBAR63Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            XBAR63Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            XBAR63Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            XBAR63Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            XBAR63Conf::HSIRC => return self.HSIRC_get(),
            XBAR63Conf::HSEOSC => return self.HSEOSC_get(),
            XBAR63Conf::MSIRC => return self.MSIRC_get(),
        };
    }
    fn XBAR63Prediv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR63_get()? as f32;
        let value = self.XBAR63Prediv.get()? as f32;
        Ok((input / value) as f32)
    }

    fn XBAR63Findiv_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR63Prediv_get()? as f32;
        let value = self.XBAR63Findiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn XBAR63Output_get(&self) -> Result<f32, ClockError> {
        let input = self.XBAR63Findiv_get()?;
        if input > (400000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 400000000),
                from: ClockNodes::XBAR63Findiv,
                to: ClockNodes::XBAR63Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::XBAR63Findiv,
                to: ClockNodes::XBAR63Output,
            });
        }
        Ok(input)
    }
    fn CKINTSEL0_get(&self) -> Result<f32, ClockError> {
        match self.CKINTSEL0 {
            CKINTSEL0Conf::HSIRC => return self.HSIRC_get(),
            CKINTSEL0Conf::HSEOSC => return self.HSEOSC_get(),
            CKINTSEL0Conf::MSIRC => return self.MSIRC_get(),
            CKINTSEL0Conf::PLL4Source => return self.PLL4Source_get(),
            CKINTSEL0Conf::PLL5Source => return self.PLL5Source_get(),
            CKINTSEL0Conf::PLL6Source => return self.PLL6Source_get(),
            CKINTSEL0Conf::PLL7Source => return self.PLL7Source_get(),
            CKINTSEL0Conf::PLL8Source => return self.PLL8Source_get(),
            CKINTSEL0Conf::PLL1Source => return self.PLL1Source_get(),
            CKINTSEL0Conf::PLL2Source => return self.PLL2Source_get(),
            CKINTSEL0Conf::PLL3Source => return self.PLL3Source_get(),
            CKINTSEL0Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            CKINTSEL0Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            CKINTSEL0Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            CKINTSEL0Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            CKINTSEL0Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            CKINTSEL0Conf::SPDIF => return self.SPDIF_get(),
            CKINTSEL0Conf::I2S_CKIN => return self.I2S_CKIN_get(),
            CKINTSEL0Conf::LSIRC => return self.LSIRC_get(),
            CKINTSEL0Conf::LSEOSC => return self.LSEOSC_get(),
            CKINTSEL0Conf::XBAR0Output => return self.XBAR0Output_get(),
            CKINTSEL0Conf::XBAR1Output => return self.XBAR1Output_get(),
            CKINTSEL0Conf::XBAR2Output => return self.XBAR2Output_get(),
            CKINTSEL0Conf::XBAR3Output => return self.XBAR3Output_get(),
            CKINTSEL0Conf::XBAR4Output => return self.XBAR4Output_get(),
            CKINTSEL0Conf::XBAR5Output => return self.XBAR5Output_get(),
            CKINTSEL0Conf::XBAR6Output => return self.XBAR6Output_get(),
            CKINTSEL0Conf::XBAR7Output => return self.XBAR7Output_get(),
            CKINTSEL0Conf::XBAR8Output => return self.XBAR8Output_get(),
            CKINTSEL0Conf::XBAR9Output => return self.XBAR9Output_get(),
            CKINTSEL0Conf::XBAR10Output => return self.XBAR10Output_get(),
            CKINTSEL0Conf::XBAR11Output => return self.XBAR11Output_get(),
            CKINTSEL0Conf::XBAR12Output => return self.XBAR12Output_get(),
            CKINTSEL0Conf::XBAR13Output => return self.XBAR13Output_get(),
            CKINTSEL0Conf::XBAR14Output => return self.XBAR14Output_get(),
            CKINTSEL0Conf::XBAR15Output => return self.XBAR15Output_get(),
            CKINTSEL0Conf::XBAR16Output => return self.XBAR16Output_get(),
            CKINTSEL0Conf::XBAR17Output => return self.XBAR17Output_get(),
            CKINTSEL0Conf::XBAR18Output => return self.XBAR18Output_get(),
            CKINTSEL0Conf::XBAR19Output => return self.XBAR19Output_get(),
            CKINTSEL0Conf::XBAR20Output => return self.XBAR20Output_get(),
            CKINTSEL0Conf::XBAR21Output => return self.XBAR21Output_get(),
            CKINTSEL0Conf::XBAR22Output => return self.XBAR22Output_get(),
            CKINTSEL0Conf::XBAR23Output => return self.XBAR23Output_get(),
            CKINTSEL0Conf::XBAR24Output => return self.XBAR24Output_get(),
            CKINTSEL0Conf::XBAR25Output => return self.XBAR25Output_get(),
            CKINTSEL0Conf::XBAR26Output => return self.XBAR26Output_get(),
            CKINTSEL0Conf::XBAR27Output => return self.XBAR27Output_get(),
            CKINTSEL0Conf::XBAR28Output => return self.XBAR28Output_get(),
            CKINTSEL0Conf::XBAR29Output => return self.XBAR29Output_get(),
            CKINTSEL0Conf::XBAR30Output => return self.XBAR30Output_get(),
            CKINTSEL0Conf::XBAR31Output => return self.XBAR31Output_get(),
            CKINTSEL0Conf::XBAR32Output => return self.XBAR32Output_get(),
            CKINTSEL0Conf::XBAR33Output => return self.XBAR33Output_get(),
            CKINTSEL0Conf::XBAR34Output => return self.XBAR34Output_get(),
            CKINTSEL0Conf::XBAR35Output => return self.XBAR35Output_get(),
            CKINTSEL0Conf::XBAR36Output => return self.XBAR36Output_get(),
            CKINTSEL0Conf::XBAR37Output => return self.XBAR37Output_get(),
            CKINTSEL0Conf::XBAR38Output => return self.XBAR38Output_get(),
            CKINTSEL0Conf::XBAR39Output => return self.XBAR39Output_get(),
            CKINTSEL0Conf::XBAR40Output => return self.XBAR40Output_get(),
            CKINTSEL0Conf::XBAR41Output => return self.XBAR41Output_get(),
            CKINTSEL0Conf::XBAR42Output => return self.XBAR42Output_get(),
            CKINTSEL0Conf::XBAR43Output => return self.XBAR43Output_get(),
            CKINTSEL0Conf::XBAR44Output => return self.XBAR44Output_get(),
            CKINTSEL0Conf::XBAR45Output => return self.XBAR45Output_get(),
            CKINTSEL0Conf::XBAR46Output => return self.XBAR46Output_get(),
            CKINTSEL0Conf::XBAR47Output => return self.XBAR47Output_get(),
            CKINTSEL0Conf::XBAR48Output => return self.XBAR48Output_get(),
            CKINTSEL0Conf::XBAR49Output => return self.XBAR49Output_get(),
            CKINTSEL0Conf::XBAR50Output => return self.XBAR50Output_get(),
            CKINTSEL0Conf::XBAR51Output => return self.XBAR51Output_get(),
            CKINTSEL0Conf::XBAR52Output => return self.XBAR52Output_get(),
            CKINTSEL0Conf::XBAR53Output => return self.XBAR53Output_get(),
            CKINTSEL0Conf::XBAR54Output => return self.XBAR54Output_get(),
            CKINTSEL0Conf::XBAR55Output => return self.XBAR55Output_get(),
            CKINTSEL0Conf::XBAR56Output => return self.XBAR56Output_get(),
            CKINTSEL0Conf::XBAR57Output => return self.XBAR57Output_get(),
            CKINTSEL0Conf::XBAR58Output => return self.XBAR58Output_get(),
            CKINTSEL0Conf::XBAR59Output => return self.XBAR59Output_get(),
            CKINTSEL0Conf::XBAR60Output => return self.XBAR60Output_get(),
            CKINTSEL0Conf::XBAR61Output => return self.XBAR61Output_get(),
            CKINTSEL0Conf::XBAR62Output => return self.XBAR62Output_get(),
            CKINTSEL0Conf::XBAR63Output => return self.XBAR63Output_get(),
        };
    }
    fn CKEXTSEL0_get(&self) -> Result<f32, ClockError> {
        match self.CKEXTSEL0 {
            CKEXTSEL0Conf::PLL1Div42 => return self.PLL1Div42_get(),
            CKEXTSEL0Conf::PLL2Div4 => return self.PLL2Div4_get(),
            CKEXTSEL0Conf::PLL3Div2 => return self.PLL3Div2_get(),
        };
    }
    fn CKINTSEL1_get(&self) -> Result<f32, ClockError> {
        match self.CKINTSEL1 {
            CKINTSEL1Conf::HSIRC => return self.HSIRC_get(),
            CKINTSEL1Conf::HSEOSC => return self.HSEOSC_get(),
            CKINTSEL1Conf::MSIRC => return self.MSIRC_get(),
            CKINTSEL1Conf::PLL4Source => return self.PLL4Source_get(),
            CKINTSEL1Conf::PLL5Source => return self.PLL5Source_get(),
            CKINTSEL1Conf::PLL6Source => return self.PLL6Source_get(),
            CKINTSEL1Conf::PLL7Source => return self.PLL7Source_get(),
            CKINTSEL1Conf::PLL8Source => return self.PLL8Source_get(),
            CKINTSEL1Conf::PLL1Source => return self.PLL1Source_get(),
            CKINTSEL1Conf::PLL2Source => return self.PLL2Source_get(),
            CKINTSEL1Conf::PLL3Source => return self.PLL3Source_get(),
            CKINTSEL1Conf::FOUTPOSTDIV4 => return self.FOUTPOSTDIV4_get(),
            CKINTSEL1Conf::FOUTPOSTDIV5 => return self.FOUTPOSTDIV5_get(),
            CKINTSEL1Conf::FOUTPOSTDIV6 => return self.FOUTPOSTDIV6_get(),
            CKINTSEL1Conf::FOUTPOSTDIV7 => return self.FOUTPOSTDIV7_get(),
            CKINTSEL1Conf::FOUTPOSTDIV8 => return self.FOUTPOSTDIV8_get(),
            CKINTSEL1Conf::SPDIF => return self.SPDIF_get(),
            CKINTSEL1Conf::I2S_CKIN => return self.I2S_CKIN_get(),
            CKINTSEL1Conf::LSIRC => return self.LSIRC_get(),
            CKINTSEL1Conf::LSEOSC => return self.LSEOSC_get(),
            CKINTSEL1Conf::XBAR0Output => return self.XBAR0Output_get(),
            CKINTSEL1Conf::XBAR1Output => return self.XBAR1Output_get(),
            CKINTSEL1Conf::XBAR2Output => return self.XBAR2Output_get(),
            CKINTSEL1Conf::XBAR3Output => return self.XBAR3Output_get(),
            CKINTSEL1Conf::XBAR4Output => return self.XBAR4Output_get(),
            CKINTSEL1Conf::XBAR5Output => return self.XBAR5Output_get(),
            CKINTSEL1Conf::XBAR6Output => return self.XBAR6Output_get(),
            CKINTSEL1Conf::XBAR7Output => return self.XBAR7Output_get(),
            CKINTSEL1Conf::XBAR8Output => return self.XBAR8Output_get(),
            CKINTSEL1Conf::XBAR9Output => return self.XBAR9Output_get(),
            CKINTSEL1Conf::XBAR10Output => return self.XBAR10Output_get(),
            CKINTSEL1Conf::XBAR11Output => return self.XBAR11Output_get(),
            CKINTSEL1Conf::XBAR12Output => return self.XBAR12Output_get(),
            CKINTSEL1Conf::XBAR13Output => return self.XBAR13Output_get(),
            CKINTSEL1Conf::XBAR14Output => return self.XBAR14Output_get(),
            CKINTSEL1Conf::XBAR15Output => return self.XBAR15Output_get(),
            CKINTSEL1Conf::XBAR16Output => return self.XBAR16Output_get(),
            CKINTSEL1Conf::XBAR17Output => return self.XBAR17Output_get(),
            CKINTSEL1Conf::XBAR18Output => return self.XBAR18Output_get(),
            CKINTSEL1Conf::XBAR19Output => return self.XBAR19Output_get(),
            CKINTSEL1Conf::XBAR20Output => return self.XBAR20Output_get(),
            CKINTSEL1Conf::XBAR21Output => return self.XBAR21Output_get(),
            CKINTSEL1Conf::XBAR22Output => return self.XBAR22Output_get(),
            CKINTSEL1Conf::XBAR23Output => return self.XBAR23Output_get(),
            CKINTSEL1Conf::XBAR24Output => return self.XBAR24Output_get(),
            CKINTSEL1Conf::XBAR25Output => return self.XBAR25Output_get(),
            CKINTSEL1Conf::XBAR26Output => return self.XBAR26Output_get(),
            CKINTSEL1Conf::XBAR27Output => return self.XBAR27Output_get(),
            CKINTSEL1Conf::XBAR28Output => return self.XBAR28Output_get(),
            CKINTSEL1Conf::XBAR29Output => return self.XBAR29Output_get(),
            CKINTSEL1Conf::XBAR30Output => return self.XBAR30Output_get(),
            CKINTSEL1Conf::XBAR31Output => return self.XBAR31Output_get(),
            CKINTSEL1Conf::XBAR32Output => return self.XBAR32Output_get(),
            CKINTSEL1Conf::XBAR33Output => return self.XBAR33Output_get(),
            CKINTSEL1Conf::XBAR34Output => return self.XBAR34Output_get(),
            CKINTSEL1Conf::XBAR35Output => return self.XBAR35Output_get(),
            CKINTSEL1Conf::XBAR36Output => return self.XBAR36Output_get(),
            CKINTSEL1Conf::XBAR37Output => return self.XBAR37Output_get(),
            CKINTSEL1Conf::XBAR38Output => return self.XBAR38Output_get(),
            CKINTSEL1Conf::XBAR39Output => return self.XBAR39Output_get(),
            CKINTSEL1Conf::XBAR40Output => return self.XBAR40Output_get(),
            CKINTSEL1Conf::XBAR41Output => return self.XBAR41Output_get(),
            CKINTSEL1Conf::XBAR42Output => return self.XBAR42Output_get(),
            CKINTSEL1Conf::XBAR43Output => return self.XBAR43Output_get(),
            CKINTSEL1Conf::XBAR44Output => return self.XBAR44Output_get(),
            CKINTSEL1Conf::XBAR45Output => return self.XBAR45Output_get(),
            CKINTSEL1Conf::XBAR46Output => return self.XBAR46Output_get(),
            CKINTSEL1Conf::XBAR47Output => return self.XBAR47Output_get(),
            CKINTSEL1Conf::XBAR48Output => return self.XBAR48Output_get(),
            CKINTSEL1Conf::XBAR49Output => return self.XBAR49Output_get(),
            CKINTSEL1Conf::XBAR50Output => return self.XBAR50Output_get(),
            CKINTSEL1Conf::XBAR51Output => return self.XBAR51Output_get(),
            CKINTSEL1Conf::XBAR52Output => return self.XBAR52Output_get(),
            CKINTSEL1Conf::XBAR53Output => return self.XBAR53Output_get(),
            CKINTSEL1Conf::XBAR54Output => return self.XBAR54Output_get(),
            CKINTSEL1Conf::XBAR55Output => return self.XBAR55Output_get(),
            CKINTSEL1Conf::XBAR56Output => return self.XBAR56Output_get(),
            CKINTSEL1Conf::XBAR57Output => return self.XBAR57Output_get(),
            CKINTSEL1Conf::XBAR58Output => return self.XBAR58Output_get(),
            CKINTSEL1Conf::XBAR59Output => return self.XBAR59Output_get(),
            CKINTSEL1Conf::XBAR60Output => return self.XBAR60Output_get(),
            CKINTSEL1Conf::XBAR61Output => return self.XBAR61Output_get(),
            CKINTSEL1Conf::XBAR62Output => return self.XBAR62Output_get(),
            CKINTSEL1Conf::XBAR63Output => return self.XBAR63Output_get(),
        };
    }
    fn CKEXTSEL1_get(&self) -> Result<f32, ClockError> {
        match self.CKEXTSEL1 {
            CKEXTSEL1Conf::PLL1Div42 => return self.PLL1Div42_get(),
            CKEXTSEL1Conf::PLL2Div4 => return self.PLL2Div4_get(),
            CKEXTSEL1Conf::PLL3Div2 => return self.PLL3Div2_get(),
        };
    }
    fn OBS0_get(&self) -> Result<f32, ClockError> {
        match self.OBS0 {
            OBS0Conf::CKINTSEL0 => return self.CKINTSEL0_get(),
            OBS0Conf::CKEXTSEL0 => return self.CKEXTSEL0_get(),
        };
    }
    pub fn OBS0Output_get(&self) -> Result<f32, ClockError> {
        self.OBS0_get()
    }
    fn OBS1_get(&self) -> Result<f32, ClockError> {
        match self.OBS1 {
            OBS1Conf::CKINTSEL1 => return self.CKINTSEL1_get(),
            OBS1Conf::CKEXTSEL1 => return self.CKEXTSEL1_get(),
        };
    }
    pub fn OBS1Output_get(&self) -> Result<f32, ClockError> {
        self.OBS1_get()
    }
    fn MCO1Mult_get(&self) -> Result<f32, ClockError> {
        match self.MCO1Mult {
            MCO1MultConf::XBAR61Output => return self.XBAR61Output_get(),
            MCO1MultConf::OBS0Output => return self.OBS0Output_get(),
        };
    }
    pub fn MCO1Pin_get(&self) -> Result<f32, ClockError> {
        self.MCO1Mult_get()
    }
    fn MCO2Mult_get(&self) -> Result<f32, ClockError> {
        match self.MCO2Mult {
            MCO2MultConf::XBAR62Output => return self.XBAR62Output_get(),
            MCO2MultConf::OBS1Output => return self.OBS1Output_get(),
        };
    }
    pub fn MCO2Pin_get(&self) -> Result<f32, ClockError> {
        self.MCO2Mult_get()
    }
    fn D3PER_get(&self) -> Result<f32, ClockError> {
        match self.D3PER {
            D3PERConf::MSIRC => return self.MSIRC_get(),
            D3PERConf::LSIRC => return self.LSIRC_get(),
            D3PERConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn D3PEROutput_get(&self) -> Result<f32, ClockError> {
        let input = self.D3PER_get()?;
        if input > (64000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 64000000),
                from: ClockNodes::D3PER,
                to: ClockNodes::D3PEROutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::D3PER,
                to: ClockNodes::D3PEROutput,
            });
        }
        Ok(input)
    }
    fn DTS_get(&self) -> Result<f32, ClockError> {
        match self.DTS {
            DTSConf::MSIRC => return self.MSIRC_get(),
            DTSConf::HSIRC => return self.HSIRC_get(),
            DTSConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    pub fn DTSOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.DTS_get()?;
        if input > (64000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 64000000),
                from: ClockNodes::DTS,
                to: ClockNodes::DTSOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::DTS,
                to: ClockNodes::DTSOutput,
            });
        }
        Ok(input)
    }
    fn DSIPHY_get(&self) -> Result<f32, ClockError> {
        match self.DSIPHY {
            DSIPHYConf::XBAR28Output => return self.XBAR28Output_get(),
            DSIPHYConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    pub fn DSIPHYOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.DSIPHY_get()?;
        if input > (27000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 27000000),
                from: ClockNodes::DSIPHY,
                to: ClockNodes::DSIPHYOutput,
            });
        } else if input < (17000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 17000000),
                from: ClockNodes::DSIPHY,
                to: ClockNodes::DSIPHYOutput,
            });
        }
        Ok(input)
    }
    fn DSIBLANE_get(&self) -> Result<f32, ClockError> {
        match self.DSIBLANE {
            DSIBLANEConf::DSIPHYOutput => return self.DSIPHYOutput_get(),
            DSIBLANEConf::XBAR27Output => return self.XBAR27Output_get(),
        };
    }
    pub fn DSIBLANEOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.DSIBLANE_get()?;
        if input > (314000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 314000000),
                from: ClockNodes::DSIBLANE,
                to: ClockNodes::DSIBLANEOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::DSIBLANE,
                to: ClockNodes::DSIBLANEOutput,
            });
        }
        Ok(input)
    }
    fn USB2PHY1_get(&self) -> Result<f32, ClockError> {
        match self.USB2PHY1 {
            USB2PHY1Conf::XBAR57Output => return self.XBAR57Output_get(),
            USB2PHY1Conf::HSEDIV2 => return self.HSEDIV2_get(),
        };
    }
    pub fn USB2PHY1Output_get(&self) -> Result<f32, ClockError> {
        let input = self.USB2PHY1_get()?;
        if input > (24000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 24000000),
                from: ClockNodes::USB2PHY1,
                to: ClockNodes::USB2PHY1Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::USB2PHY1,
                to: ClockNodes::USB2PHY1Output,
            });
        }
        Ok(input)
    }
    fn USB2PHY2_get(&self) -> Result<f32, ClockError> {
        match self.USB2PHY2 {
            USB2PHY2Conf::XBAR58Output => return self.XBAR58Output_get(),
            USB2PHY2Conf::HSEDIV2 => return self.HSEDIV2_get(),
        };
    }
    pub fn USB2PHY2Output_get(&self) -> Result<f32, ClockError> {
        let input = self.USB2PHY2_get()?;
        if input > (24000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 24000000),
                from: ClockNodes::USB2PHY2,
                to: ClockNodes::USB2PHY2Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::USB2PHY2,
                to: ClockNodes::USB2PHY2Output,
            });
        }
        Ok(input)
    }
    fn USB3PCIPHY_get(&self) -> Result<f32, ClockError> {
        match self.USB3PCIPHY {
            USB3PCIPHYConf::XBAR34Output => return self.XBAR34Output_get(),
            USB3PCIPHYConf::HSEDIV2 => return self.HSEDIV2_get(),
        };
    }
    pub fn USB3PCIPHYOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.USB3PCIPHY_get()?;
        if input > (25000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 25000000),
                from: ClockNodes::USB3PCIPHY,
                to: ClockNodes::USB3PCIPHYOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::USB3PCIPHY,
                to: ClockNodes::USB3PCIPHYOutput,
            });
        }
        Ok(input)
    }
    fn SysClkSource_get(&self) -> Result<f32, ClockError> {
        match self.SysClkSource {
            SysClkSourceConf::XBAR0Output => return self.XBAR0Output_get(),
            SysClkSourceConf::HSIRC => return self.HSIRC_get(),
        };
    }
    pub fn SysCLKOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SysClkSource_get()?;
        if input > (400000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 400000000),
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
    fn MCUDIV_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()? as f32;
        let value = self.MCUDIV.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn McuClockOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()?;
        if input > (400000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 400000000),
                from: ClockNodes::SysCLKOutput,
                to: ClockNodes::McuClockOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SysCLKOutput,
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
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
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
    fn APB4DIV_get(&self) -> Result<f32, ClockError> {
        let input = self.MCUDIV_get()? as f32;
        let value = self.APB4DIV.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB4Output_get(&self) -> Result<f32, ClockError> {
        let input = self.APB4DIV_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
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
    fn APBDBGDIV_get(&self) -> Result<f32, ClockError> {
        let input = self.MCUDIV_get()? as f32;
        let value = self.APBDBGDIV.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APBDBGOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.APBDBGDIV_get()?;
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
                from: ClockNodes::APBDBGDIV,
                to: ClockNodes::APBDBGOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::APBDBGDIV,
                to: ClockNodes::APBDBGOutput,
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
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
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
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
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
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
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
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
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
        if input > (200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 200000000),
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
    fn ADC12Mult_get(&self) -> Result<f32, ClockError> {
        match self.ADC12Mult {
            ADC12MultConf::XBAR46Output => return self.XBAR46Output_get(),
            ADC12MultConf::XBAR0Output => return self.XBAR0Output_get(),
        };
    }
    pub fn ADC12output_get(&self) -> Result<f32, ClockError> {
        let input = self.ADC12Mult_get()?;
        if input > (133000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133000000),
                from: ClockNodes::ADC12Mult,
                to: ClockNodes::ADC12output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::ADC12Mult,
                to: ClockNodes::ADC12output,
            });
        }
        Ok(input)
    }
    fn ADC3Mult_get(&self) -> Result<f32, ClockError> {
        match self.ADC3Mult {
            ADC3MultConf::XBAR47Output => return self.XBAR47Output_get(),
            ADC3MultConf::XBAR0Output => return self.XBAR0Output_get(),
            ADC3MultConf::XBAR46Output => return self.XBAR46Output_get(),
        };
    }
    pub fn ADC3output_get(&self) -> Result<f32, ClockError> {
        let input = self.ADC3Mult_get()?;
        if input > (133000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 133000000),
                from: ClockNodes::ADC3Mult,
                to: ClockNodes::ADC3output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::ADC3Mult,
                to: ClockNodes::ADC3output,
            });
        }
        Ok(input)
    }
    fn PLL1Source_get(&self) -> Result<f32, ClockError> {
        match self.PLL1Source {
            PLL1SourceConf::HSIRC => return self.HSIRC_get(),
            PLL1SourceConf::HSEOSC => return self.HSEOSC_get(),
            PLL1SourceConf::MSIRC => return self.MSIRC_get(),
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
            PLL2SourceConf::HSEOSC => return self.HSEOSC_get(),
            PLL2SourceConf::MSIRC => return self.MSIRC_get(),
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
    fn PLL1Div42_get(&self) -> Result<f32, ClockError> {
        let input = self.FOUTPOSTDIV1_get()? as f32;
        let value = 42 as f32;
        Ok((input / value) as f32)
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
    fn PLL2Div4_get(&self) -> Result<f32, ClockError> {
        let input = self.FOUTPOSTDIV2_get()? as f32;
        let value = 4 as f32;
        Ok((input / value) as f32)
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
    fn PLL3Div2_get(&self) -> Result<f32, ClockError> {
        let input = self.FOUTPOSTDIV3_get()? as f32;
        let value = 2 as f32;
        Ok((input / value) as f32)
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
    fn PLL5Source_get(&self) -> Result<f32, ClockError> {
        match self.PLL5Source {
            PLL5SourceConf::HSIRC => return self.HSIRC_get(),
            PLL5SourceConf::MSIRC => return self.MSIRC_get(),
            PLL5SourceConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    fn FREFDIV5_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL5Source_get()? as f32;
        let value = self.FREFDIV5.get()? as f32;
        Ok((input / value) as f32)
    }

    fn FBDIV5_get(&self) -> Result<f32, ClockError> {
        let input = self.FREFDIV5_get()? as f32;
        let frac = self.PLL5FRACV_get()? as f32;
        let frac_max = PLL5FRACVConf::max() as f32;
        let value = self.FBDIV5.get()? as f32;
        let ret = (input * (value + (frac / frac_max)));
        Ok(ret as f32)
    }

    pub fn PLL5FRACV_get(&self) -> Result<f32, ClockError> {
        self.PLL5FRACV.get()
    }
    fn POSTDIV1_5_get(&self) -> Result<f32, ClockError> {
        let input = self.FBDIV5_get()? as f32;
        let value = self.POSTDIV1_5.get()? as f32;
        Ok((input / value) as f32)
    }

    fn POSTDIV2_5_get(&self) -> Result<f32, ClockError> {
        let input = self.POSTDIV1_5_get()? as f32;
        let value = self.POSTDIV2_5.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn FOUTPOSTDIV5_get(&self) -> Result<f32, ClockError> {
        let input = self.POSTDIV2_5_get()?;
        if input > (3200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 3200000000),
                from: ClockNodes::POSTDIV2_5,
                to: ClockNodes::FOUTPOSTDIV5,
            });
        } else if input < (16000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 16000000),
                from: ClockNodes::POSTDIV2_5,
                to: ClockNodes::FOUTPOSTDIV5,
            });
        }
        Ok(input)
    }
    fn PLL6Source_get(&self) -> Result<f32, ClockError> {
        match self.PLL6Source {
            PLL6SourceConf::HSIRC => return self.HSIRC_get(),
            PLL6SourceConf::MSIRC => return self.MSIRC_get(),
            PLL6SourceConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    fn FREFDIV6_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL6Source_get()? as f32;
        let value = self.FREFDIV6.get()? as f32;
        Ok((input / value) as f32)
    }

    fn FBDIV6_get(&self) -> Result<f32, ClockError> {
        let input = self.FREFDIV6_get()? as f32;
        let frac = self.PLL6FRACV_get()? as f32;
        let frac_max = PLL6FRACVConf::max() as f32;
        let value = self.FBDIV6.get()? as f32;
        let ret = (input * (value + (frac / frac_max)));
        Ok(ret as f32)
    }

    pub fn PLL6FRACV_get(&self) -> Result<f32, ClockError> {
        self.PLL6FRACV.get()
    }
    fn POSTDIV1_6_get(&self) -> Result<f32, ClockError> {
        let input = self.FBDIV6_get()? as f32;
        let value = self.POSTDIV1_6.get()? as f32;
        Ok((input / value) as f32)
    }

    fn POSTDIV2_6_get(&self) -> Result<f32, ClockError> {
        let input = self.POSTDIV1_6_get()? as f32;
        let value = self.POSTDIV2_6.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn FOUTPOSTDIV6_get(&self) -> Result<f32, ClockError> {
        let input = self.POSTDIV2_6_get()?;
        if input > (3200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 3200000000),
                from: ClockNodes::POSTDIV2_6,
                to: ClockNodes::FOUTPOSTDIV6,
            });
        } else if input < (16000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 16000000),
                from: ClockNodes::POSTDIV2_6,
                to: ClockNodes::FOUTPOSTDIV6,
            });
        }
        Ok(input)
    }
    fn PLL7Source_get(&self) -> Result<f32, ClockError> {
        match self.PLL7Source {
            PLL7SourceConf::HSIRC => return self.HSIRC_get(),
            PLL7SourceConf::MSIRC => return self.MSIRC_get(),
            PLL7SourceConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    fn FREFDIV7_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL7Source_get()? as f32;
        let value = self.FREFDIV7.get()? as f32;
        Ok((input / value) as f32)
    }

    fn FBDIV7_get(&self) -> Result<f32, ClockError> {
        let input = self.FREFDIV7_get()? as f32;
        let frac = self.PLL7FRACV_get()? as f32;
        let frac_max = PLL7FRACVConf::max() as f32;
        let value = self.FBDIV7.get()? as f32;
        let ret = (input * (value + (frac / frac_max)));
        Ok(ret as f32)
    }

    pub fn PLL7FRACV_get(&self) -> Result<f32, ClockError> {
        self.PLL7FRACV.get()
    }
    fn POSTDIV1_7_get(&self) -> Result<f32, ClockError> {
        let input = self.FBDIV7_get()? as f32;
        let value = self.POSTDIV1_7.get()? as f32;
        Ok((input / value) as f32)
    }

    fn POSTDIV2_7_get(&self) -> Result<f32, ClockError> {
        let input = self.POSTDIV1_7_get()? as f32;
        let value = self.POSTDIV2_7.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn FOUTPOSTDIV7_get(&self) -> Result<f32, ClockError> {
        let input = self.POSTDIV2_7_get()?;
        if input > (3200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 3200000000),
                from: ClockNodes::POSTDIV2_7,
                to: ClockNodes::FOUTPOSTDIV7,
            });
        } else if input < (16000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 16000000),
                from: ClockNodes::POSTDIV2_7,
                to: ClockNodes::FOUTPOSTDIV7,
            });
        }
        Ok(input)
    }
    fn PLL8Source_get(&self) -> Result<f32, ClockError> {
        match self.PLL8Source {
            PLL8SourceConf::HSIRC => return self.HSIRC_get(),
            PLL8SourceConf::MSIRC => return self.MSIRC_get(),
            PLL8SourceConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    fn FREFDIV8_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL8Source_get()? as f32;
        let value = self.FREFDIV8.get()? as f32;
        Ok((input / value) as f32)
    }

    fn FBDIV8_get(&self) -> Result<f32, ClockError> {
        let input = self.FREFDIV8_get()? as f32;
        let frac = self.PLL8FRACV_get()? as f32;
        let frac_max = PLL8FRACVConf::max() as f32;
        let value = self.FBDIV8.get()? as f32;
        let ret = (input * (value + (frac / frac_max)));
        Ok(ret as f32)
    }

    pub fn PLL8FRACV_get(&self) -> Result<f32, ClockError> {
        self.PLL8FRACV.get()
    }
    fn POSTDIV1_8_get(&self) -> Result<f32, ClockError> {
        let input = self.FBDIV8_get()? as f32;
        let value = self.POSTDIV1_8.get()? as f32;
        Ok((input / value) as f32)
    }

    fn POSTDIV2_8_get(&self) -> Result<f32, ClockError> {
        let input = self.POSTDIV1_8_get()? as f32;
        let value = self.POSTDIV2_8.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn FOUTPOSTDIV8_get(&self) -> Result<f32, ClockError> {
        let input = self.POSTDIV2_8_get()?;
        if input > (3200000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 3200000000),
                from: ClockNodes::POSTDIV2_8,
                to: ClockNodes::FOUTPOSTDIV8,
            });
        } else if input < (16000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 16000000),
                from: ClockNodes::POSTDIV2_8,
                to: ClockNodes::FOUTPOSTDIV8,
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
