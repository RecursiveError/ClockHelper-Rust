#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ClockNodes {
    None,
    HSIRC,
    HSEOSC,
    HseDiv,
    LSIRC,
    LSI2RC,
    LSIDIV,
    LSIOut,
    LSEOSC,
    SAI1_EXT,
    HSERSTDevisor,
    RSTClkSource,
    RSTOutput,
    RSTRFOutput,
    SysClkSource,
    SysCLKOutput,
    PLLSource,
    PLLM,
    HSERTCDevisor,
    RTCClkSource,
    RTCOutput,
    IWDGOutput,
    USART2Mult,
    USART2output,
    USART1Mult,
    USART1output,
    LPUART1Mult,
    LPUART1output,
    LPTIM1Mult,
    LPTIM1output,
    LPTIM2Mult,
    LPTIM2output,
    ADCMult,
    ADCoutput,
    ASMult,
    ASoutput,
    I2C1Mult,
    I2C1output,
    I2C3Mult,
    I2C3output,
    SAI1Mult,
    SAI1output,
    pllqDivToRNG,
    RNGMult,
    RNGoutput,
    MCOMult,
    MCODiv,
    MCOPin,
    LSCOMult,
    LSCOOutput,
    AHB5Prescaler,
    AHB5Output,
    SAESOutput,
    AHBPrescaler,
    AHBOutput,
    HCLKOutput,
    HCLK4Output,
    CortexPrescaler,
    CortexCLockSelection,
    CortexSysOutput,
    FCLKCortexOutput,
    APB1Prescaler,
    APB1Output,
    TimPrescalerAPB1,
    TimPrescOut1,
    APB2Prescaler,
    APB2Output,
    APB7Prescaler,
    APB7Output,
    TimPrescalerAPB2,
    TimPrescOut2,
    SPI1Mult,
    SPI1output,
    SPI3Mult,
    SPI3output,
    PLLN,
    PLLFRACN,
    PLL1P,
    PLLPoutput,
    PLL1Q,
    PLLQoutput,
    PLL1R,
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
pub enum HseDivConf {
    DIV1,
    DIV2,
}

impl HseDivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            HseDivConf::DIV1 => return Ok(1.0),
            HseDivConf::DIV2 => return Ok(2.0),
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
pub enum LSI2RCConf {
    Value(u32),
}

impl LSI2RCConf {
    pub const fn min() -> u32 {
        31400
    }

    pub const fn max() -> u32 {
        32600
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            LSI2RCConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::LSI2RC,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::LSI2RC,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LSIDIVConf {
    DIV1,
    DIV128,
}

impl LSIDIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            LSIDIVConf::DIV1 => return Ok(1.0),
            LSIDIVConf::DIV128 => return Ok(128.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LSIOutConf {
    LSIDIV,
    LSI2RC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LSEOSCConf {
    Value(u32),
}

impl LSEOSCConf {
    pub const fn min() -> u32 {
        1000
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
pub enum RSTClkSourceConf {
    HSERSTDevisor,
    LSEOSC,
    LSIOut,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SysClkSourceConf {
    HSIRC,
    HseDiv,
    PLL1R,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSourceConf {
    HSIRC,
    HseDiv,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLMConf {
    Value(u32),
}

impl PLLMConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        8
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
pub enum RTCClkSourceConf {
    HSERTCDevisor,
    LSEOSC,
    LSIOut,
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
pub enum USART1MultConf {
    APB2Prescaler,
    SysCLKOutput,
    HSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPUART1MultConf {
    APB7Output,
    SysCLKOutput,
    HSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM1MultConf {
    APB7Output,
    LSIOut,
    HSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM2MultConf {
    APB1Prescaler,
    LSIOut,
    HSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ADCMultConf {
    AHBOutput,
    SysCLKOutput,
    HSEOSC,
    HSIRC,
    PLL1P,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ASMultConf {
    PLL1P,
    PLL1Q,
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
pub enum I2C3MultConf {
    APB7Output,
    SysCLKOutput,
    HSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SAI1MultConf {
    PLL1P,
    HSIRC,
    PLL1Q,
    SAI1_EXT,
    SysCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum RNGMultConf {
    LSEOSC,
    pllqDivToRNG,
    HSIRC,
    LSIOut,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCOMultConf {
    LSEOSC,
    LSIOut,
    HSEOSC,
    HSIRC,
    PLL1R,
    SysCLKOutput,
    PLL1P,
    PLL1Q,
    AHB5Output,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCODivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl MCODivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MCODivConf::DIV1 => return Ok(1.0),
            MCODivConf::DIV2 => return Ok(2.0),
            MCODivConf::DIV4 => return Ok(4.0),
            MCODivConf::DIV8 => return Ok(8.0),
            MCODivConf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LSCOMultConf {
    LSIOut,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum AHB5PrescalerConf {
    DIV1,
    DIV2,
}

impl AHB5PrescalerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            AHB5PrescalerConf::DIV1 => return Ok(1.0),
            AHB5PrescalerConf::DIV2 => return Ok(2.0),
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
}

impl AHBPrescalerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            AHBPrescalerConf::DIV1 => return Ok(1.0),
            AHBPrescalerConf::DIV2 => return Ok(2.0),
            AHBPrescalerConf::DIV4 => return Ok(4.0),
            AHBPrescalerConf::DIV8 => return Ok(8.0),
            AHBPrescalerConf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CortexCLockSelectionConf {
    CortexPrescaler,
    LSEOSC,
    LSIOut,
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
pub enum APB7PrescalerConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl APB7PrescalerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            APB7PrescalerConf::DIV1 => return Ok(1.0),
            APB7PrescalerConf::DIV2 => return Ok(2.0),
            APB7PrescalerConf::DIV4 => return Ok(4.0),
            APB7PrescalerConf::DIV8 => return Ok(8.0),
            APB7PrescalerConf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI1MultConf {
    APB2Prescaler,
    SysCLKOutput,
    HSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI3MultConf {
    APB7Output,
    SysCLKOutput,
    HSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLNConf {
    Value(u32),
}

impl PLLNConf {
    pub const fn min() -> u32 {
        4
    }

    pub const fn max() -> u32 {
        512
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
pub enum PLL1PConf {
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

impl PLL1PConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL1PConf::DIV1 => return Ok(1.0),
            PLL1PConf::DIV2 => return Ok(2.0),
            PLL1PConf::DIV4 => return Ok(4.0),
            PLL1PConf::DIV6 => return Ok(6.0),
            PLL1PConf::DIV8 => return Ok(8.0),
            PLL1PConf::DIV10 => return Ok(10.0),
            PLL1PConf::DIV12 => return Ok(12.0),
            PLL1PConf::DIV14 => return Ok(14.0),
            PLL1PConf::DIV16 => return Ok(16.0),
            PLL1PConf::DIV18 => return Ok(18.0),
            PLL1PConf::DIV20 => return Ok(20.0),
            PLL1PConf::DIV22 => return Ok(22.0),
            PLL1PConf::DIV24 => return Ok(24.0),
            PLL1PConf::DIV26 => return Ok(26.0),
            PLL1PConf::DIV28 => return Ok(28.0),
            PLL1PConf::DIV30 => return Ok(30.0),
            PLL1PConf::DIV32 => return Ok(32.0),
            PLL1PConf::DIV34 => return Ok(34.0),
            PLL1PConf::DIV36 => return Ok(36.0),
            PLL1PConf::DIV38 => return Ok(38.0),
            PLL1PConf::DIV40 => return Ok(40.0),
            PLL1PConf::DIV42 => return Ok(42.0),
            PLL1PConf::DIV44 => return Ok(44.0),
            PLL1PConf::DIV46 => return Ok(46.0),
            PLL1PConf::DIV48 => return Ok(48.0),
            PLL1PConf::DIV50 => return Ok(50.0),
            PLL1PConf::DIV52 => return Ok(52.0),
            PLL1PConf::DIV54 => return Ok(54.0),
            PLL1PConf::DIV56 => return Ok(56.0),
            PLL1PConf::DIV58 => return Ok(58.0),
            PLL1PConf::DIV60 => return Ok(60.0),
            PLL1PConf::DIV62 => return Ok(62.0),
            PLL1PConf::DIV64 => return Ok(64.0),
            PLL1PConf::DIV66 => return Ok(66.0),
            PLL1PConf::DIV68 => return Ok(68.0),
            PLL1PConf::DIV70 => return Ok(70.0),
            PLL1PConf::DIV72 => return Ok(72.0),
            PLL1PConf::DIV74 => return Ok(74.0),
            PLL1PConf::DIV76 => return Ok(76.0),
            PLL1PConf::DIV78 => return Ok(78.0),
            PLL1PConf::DIV80 => return Ok(80.0),
            PLL1PConf::DIV82 => return Ok(82.0),
            PLL1PConf::DIV84 => return Ok(84.0),
            PLL1PConf::DIV86 => return Ok(86.0),
            PLL1PConf::DIV88 => return Ok(88.0),
            PLL1PConf::DIV90 => return Ok(90.0),
            PLL1PConf::DIV92 => return Ok(92.0),
            PLL1PConf::DIV94 => return Ok(94.0),
            PLL1PConf::DIV96 => return Ok(96.0),
            PLL1PConf::DIV98 => return Ok(98.0),
            PLL1PConf::DIV100 => return Ok(100.0),
            PLL1PConf::DIV102 => return Ok(102.0),
            PLL1PConf::DIV104 => return Ok(104.0),
            PLL1PConf::DIV106 => return Ok(106.0),
            PLL1PConf::DIV108 => return Ok(108.0),
            PLL1PConf::DIV110 => return Ok(110.0),
            PLL1PConf::DIV112 => return Ok(112.0),
            PLL1PConf::DIV114 => return Ok(114.0),
            PLL1PConf::DIV116 => return Ok(116.0),
            PLL1PConf::DIV118 => return Ok(118.0),
            PLL1PConf::DIV120 => return Ok(120.0),
            PLL1PConf::DIV122 => return Ok(122.0),
            PLL1PConf::DIV124 => return Ok(124.0),
            PLL1PConf::DIV126 => return Ok(126.0),
            PLL1PConf::DIV128 => return Ok(128.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL1QConf {
    Value(u32),
}

impl PLL1QConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        128
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL1QConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL1Q,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL1Q,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL1RConf {
    Value(u32),
}

impl PLL1RConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        128
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL1RConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL1R,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL1R,
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
    pub HseDiv: HseDivConf,
    pub LSIRC: LSIRCConf,
    pub LSI2RC: LSI2RCConf,
    pub LSIDIV: LSIDIVConf,
    pub LSIOut: LSIOutConf,
    pub LSEOSC: LSEOSCConf,
    pub RSTClkSource: RSTClkSourceConf,
    pub SysClkSource: SysClkSourceConf,
    pub PLLSource: PLLSourceConf,
    pub PLLM: PLLMConf,
    pub RTCClkSource: RTCClkSourceConf,
    pub USART2Mult: USART2MultConf,
    pub USART1Mult: USART1MultConf,
    pub LPUART1Mult: LPUART1MultConf,
    pub LPTIM1Mult: LPTIM1MultConf,
    pub LPTIM2Mult: LPTIM2MultConf,
    pub ADCMult: ADCMultConf,
    pub ASMult: ASMultConf,
    pub I2C1Mult: I2C1MultConf,
    pub I2C3Mult: I2C3MultConf,
    pub SAI1Mult: SAI1MultConf,
    pub RNGMult: RNGMultConf,
    pub MCOMult: MCOMultConf,
    pub MCODiv: MCODivConf,
    pub LSCOMult: LSCOMultConf,
    pub AHB5Prescaler: AHB5PrescalerConf,
    pub AHBPrescaler: AHBPrescalerConf,
    pub CortexCLockSelection: CortexCLockSelectionConf,
    pub APB1Prescaler: APB1PrescalerConf,
    pub APB2Prescaler: APB2PrescalerConf,
    pub APB7Prescaler: APB7PrescalerConf,
    pub SPI1Mult: SPI1MultConf,
    pub SPI3Mult: SPI3MultConf,
    pub PLLN: PLLNConf,
    pub PLLFRACN: PLLFRACNConf,
    pub PLL1P: PLL1PConf,
    pub PLL1Q: PLL1QConf,
    pub PLL1R: PLL1RConf,
}

impl Default for ClockTree {
    fn default() -> Self {
        Self {
            HseDiv: HseDivConf::DIV1,
            LSIRC: LSIRCConf::Value(32000),
            LSI2RC: LSI2RCConf::Value(32000),
            LSIDIV: LSIDIVConf::DIV1,
            LSIOut: LSIOutConf::LSIDIV,
            LSEOSC: LSEOSCConf::Value(32768),
            RSTClkSource: RSTClkSourceConf::HSERSTDevisor,
            SysClkSource: SysClkSourceConf::HSIRC,
            PLLSource: PLLSourceConf::HSIRC,
            PLLM: PLLMConf::Value(1),
            RTCClkSource: RTCClkSourceConf::LSIOut,
            USART2Mult: USART2MultConf::APB1Prescaler,
            USART1Mult: USART1MultConf::APB2Prescaler,
            LPUART1Mult: LPUART1MultConf::APB7Output,
            LPTIM1Mult: LPTIM1MultConf::APB7Output,
            LPTIM2Mult: LPTIM2MultConf::APB1Prescaler,
            ADCMult: ADCMultConf::AHBOutput,
            ASMult: ASMultConf::PLL1P,
            I2C1Mult: I2C1MultConf::APB1Prescaler,
            I2C3Mult: I2C3MultConf::APB7Output,
            SAI1Mult: SAI1MultConf::HSIRC,
            RNGMult: RNGMultConf::HSIRC,
            MCOMult: MCOMultConf::SysCLKOutput,
            MCODiv: MCODivConf::DIV1,
            LSCOMult: LSCOMultConf::LSIOut,
            AHB5Prescaler: AHB5PrescalerConf::DIV1,
            AHBPrescaler: AHBPrescalerConf::DIV1,
            CortexCLockSelection: CortexCLockSelectionConf::CortexPrescaler,
            APB1Prescaler: APB1PrescalerConf::DIV1,
            APB2Prescaler: APB2PrescalerConf::DIV1,
            APB7Prescaler: APB7PrescalerConf::DIV1,
            SPI1Mult: SPI1MultConf::APB2Prescaler,
            SPI3Mult: SPI3MultConf::APB7Output,
            PLLN: PLLNConf::Value(129),
            PLLFRACN: PLLFRACNConf::Value(0),
            PLL1P: PLL1PConf::DIV2,
            PLL1Q: PLL1QConf::Value(2),
            PLL1R: PLL1RConf::Value(2),
        }
    }
}
impl ClockTree {
    pub fn HSIRC_get(&self) -> Result<f32, ClockError> {
        Ok(16000000 as f32)
    }
    pub fn HSEOSC_get(&self) -> Result<f32, ClockError> {
        Ok(16000000 as f32)
    }
    fn HseDiv_get(&self) -> Result<f32, ClockError> {
        let input = self.HSEOSC_get()? as f32;
        let value = self.HseDiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn LSIRC_get(&self) -> Result<f32, ClockError> {
        self.LSIRC.get()
    }
    pub fn LSI2RC_get(&self) -> Result<f32, ClockError> {
        self.LSI2RC.get()
    }
    fn LSIDIV_get(&self) -> Result<f32, ClockError> {
        let input = self.LSIRC_get()? as f32;
        let value = self.LSIDIV.get()? as f32;
        Ok((input / value) as f32)
    }

    fn LSIOut_get(&self) -> Result<f32, ClockError> {
        match self.LSIOut {
            LSIOutConf::LSIDIV => return self.LSIDIV_get(),
            LSIOutConf::LSI2RC => return self.LSI2RC_get(),
        };
    }
    pub fn LSEOSC_get(&self) -> Result<f32, ClockError> {
        self.LSEOSC.get()
    }
    pub fn SAI1_EXT_get(&self) -> Result<f32, ClockError> {
        Ok(48000 as f32)
    }
    fn HSERSTDevisor_get(&self) -> Result<f32, ClockError> {
        let input = self.HSEOSC_get()? as f32;
        let value = 1000 as f32;
        Ok((input / value) as f32)
    }

    fn RSTClkSource_get(&self) -> Result<f32, ClockError> {
        match self.RSTClkSource {
            RSTClkSourceConf::HSERSTDevisor => return self.HSERSTDevisor_get(),
            RSTClkSourceConf::LSEOSC => return self.LSEOSC_get(),
            RSTClkSourceConf::LSIOut => return self.LSIOut_get(),
        };
    }
    pub fn RSTOutput_get(&self) -> Result<f32, ClockError> {
        self.RSTClkSource_get()
    }
    pub fn RSTRFOutput_get(&self) -> Result<f32, ClockError> {
        self.HSEOSC_get()
    }
    fn SysClkSource_get(&self) -> Result<f32, ClockError> {
        match self.SysClkSource {
            SysClkSourceConf::HSIRC => return self.HSIRC_get(),
            SysClkSourceConf::HseDiv => return self.HseDiv_get(),
            SysClkSourceConf::PLL1R => return self.PLL1R_get(),
        };
    }
    pub fn SysCLKOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SysClkSource_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
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
            PLLSourceConf::HseDiv => return self.HseDiv_get(),
        };
    }
    fn PLLM_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLSource_get()? as f32;
        let value = self.PLLM.get()? as f32;
        Ok((input / value) as f32)
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
            RTCClkSourceConf::LSIOut => return self.LSIOut_get(),
        };
    }
    pub fn RTCOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.RTCClkSource_get()?;
        if input > (1562500 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1562500),
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
        self.LSIOut_get()
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
    fn LPUART1Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPUART1Mult {
            LPUART1MultConf::APB7Output => return self.APB7Output_get(),
            LPUART1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            LPUART1MultConf::HSIRC => return self.HSIRC_get(),
            LPUART1MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn LPUART1output_get(&self) -> Result<f32, ClockError> {
        self.LPUART1Mult_get()
    }
    fn LPTIM1Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM1Mult {
            LPTIM1MultConf::APB7Output => return self.APB7Output_get(),
            LPTIM1MultConf::LSIOut => return self.LSIOut_get(),
            LPTIM1MultConf::HSIRC => return self.HSIRC_get(),
            LPTIM1MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn LPTIM1output_get(&self) -> Result<f32, ClockError> {
        self.LPTIM1Mult_get()
    }
    fn LPTIM2Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM2Mult {
            LPTIM2MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            LPTIM2MultConf::LSIOut => return self.LSIOut_get(),
            LPTIM2MultConf::HSIRC => return self.HSIRC_get(),
            LPTIM2MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn LPTIM2output_get(&self) -> Result<f32, ClockError> {
        self.LPTIM2Mult_get()
    }
    fn ADCMult_get(&self) -> Result<f32, ClockError> {
        match self.ADCMult {
            ADCMultConf::AHBOutput => return self.AHBOutput_get(),
            ADCMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            ADCMultConf::HSEOSC => return self.HSEOSC_get(),
            ADCMultConf::HSIRC => return self.HSIRC_get(),
            ADCMultConf::PLL1P => return self.PLL1P_get(),
        };
    }
    pub fn ADCoutput_get(&self) -> Result<f32, ClockError> {
        self.ADCMult_get()
    }
    fn ASMult_get(&self) -> Result<f32, ClockError> {
        match self.ASMult {
            ASMultConf::PLL1P => return self.PLL1P_get(),
            ASMultConf::PLL1Q => return self.PLL1Q_get(),
        };
    }
    pub fn ASoutput_get(&self) -> Result<f32, ClockError> {
        self.ASMult_get()
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
    fn I2C3Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C3Mult {
            I2C3MultConf::APB7Output => return self.APB7Output_get(),
            I2C3MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            I2C3MultConf::HSIRC => return self.HSIRC_get(),
        };
    }
    pub fn I2C3output_get(&self) -> Result<f32, ClockError> {
        self.I2C3Mult_get()
    }
    fn SAI1Mult_get(&self) -> Result<f32, ClockError> {
        match self.SAI1Mult {
            SAI1MultConf::PLL1P => return self.PLL1P_get(),
            SAI1MultConf::HSIRC => return self.HSIRC_get(),
            SAI1MultConf::PLL1Q => return self.PLL1Q_get(),
            SAI1MultConf::SAI1_EXT => return self.SAI1_EXT_get(),
            SAI1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
        };
    }
    pub fn SAI1output_get(&self) -> Result<f32, ClockError> {
        self.SAI1Mult_get()
    }
    fn pllqDivToRNG_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL1Q_get()? as f32;
        let value = 2 as f32;
        Ok((input / value) as f32)
    }

    fn RNGMult_get(&self) -> Result<f32, ClockError> {
        match self.RNGMult {
            RNGMultConf::LSEOSC => return self.LSEOSC_get(),
            RNGMultConf::pllqDivToRNG => return self.pllqDivToRNG_get(),
            RNGMultConf::HSIRC => return self.HSIRC_get(),
            RNGMultConf::LSIOut => return self.LSIOut_get(),
        };
    }
    pub fn RNGoutput_get(&self) -> Result<f32, ClockError> {
        self.RNGMult_get()
    }
    fn MCOMult_get(&self) -> Result<f32, ClockError> {
        match self.MCOMult {
            MCOMultConf::LSEOSC => return self.LSEOSC_get(),
            MCOMultConf::LSIOut => return self.LSIOut_get(),
            MCOMultConf::HSEOSC => return self.HSEOSC_get(),
            MCOMultConf::HSIRC => return self.HSIRC_get(),
            MCOMultConf::PLL1R => return self.PLL1R_get(),
            MCOMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            MCOMultConf::PLL1P => return self.PLL1P_get(),
            MCOMultConf::PLL1Q => return self.PLL1Q_get(),
            MCOMultConf::AHB5Output => return self.AHB5Output_get(),
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
    fn LSCOMult_get(&self) -> Result<f32, ClockError> {
        match self.LSCOMult {
            LSCOMultConf::LSIOut => return self.LSIOut_get(),
            LSCOMultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn LSCOOutput_get(&self) -> Result<f32, ClockError> {
        self.LSCOMult_get()
    }
    fn AHB5Prescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()? as f32;
        let value = self.AHB5Prescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn AHB5Output_get(&self) -> Result<f32, ClockError> {
        let input = self.AHB5Prescaler_get()?;
        if input > (32000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 32000000),
                from: ClockNodes::AHB5Prescaler,
                to: ClockNodes::AHB5Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::AHB5Prescaler,
                to: ClockNodes::AHB5Output,
            });
        }
        Ok(input)
    }
    pub fn SAESOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::AHBOutput,
                to: ClockNodes::SAESOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::AHBOutput,
                to: ClockNodes::SAESOutput,
            });
        }
        Ok(input)
    }
    fn AHBPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()? as f32;
        let value = self.AHBPrescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn AHBOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBPrescaler_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
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
        let input = self.AHBOutput_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::AHBOutput,
                to: ClockNodes::HCLKOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::AHBOutput,
                to: ClockNodes::HCLKOutput,
            });
        }
        Ok(input)
    }
    pub fn HCLK4Output_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::AHBOutput,
                to: ClockNodes::HCLK4Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::AHBOutput,
                to: ClockNodes::HCLK4Output,
            });
        }
        Ok(input)
    }
    fn CortexPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = 8 as f32;
        Ok((input / value) as f32)
    }

    fn CortexCLockSelection_get(&self) -> Result<f32, ClockError> {
        match self.CortexCLockSelection {
            CortexCLockSelectionConf::CortexPrescaler => return self.CortexPrescaler_get(),
            CortexCLockSelectionConf::LSEOSC => return self.LSEOSC_get(),
            CortexCLockSelectionConf::LSIOut => return self.LSIOut_get(),
        };
    }
    pub fn CortexSysOutput_get(&self) -> Result<f32, ClockError> {
        self.CortexCLockSelection_get()
    }
    pub fn FCLKCortexOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::AHBOutput,
                to: ClockNodes::FCLKCortexOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::AHBOutput,
                to: ClockNodes::FCLKCortexOutput,
            });
        }
        Ok(input)
    }
    fn APB1Prescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.APB1Prescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB1Output_get(&self) -> Result<f32, ClockError> {
        let input = self.APB1Prescaler_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
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
        let input = self.TimPrescalerAPB1_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::TimPrescalerAPB1,
                to: ClockNodes::TimPrescOut1,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::TimPrescalerAPB1,
                to: ClockNodes::TimPrescOut1,
            });
        }
        Ok(input)
    }
    fn APB2Prescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.APB2Prescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB2Output_get(&self) -> Result<f32, ClockError> {
        let input = self.APB2Prescaler_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
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
    fn APB7Prescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.APB7Prescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB7Output_get(&self) -> Result<f32, ClockError> {
        let input = self.APB7Prescaler_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::APB7Prescaler,
                to: ClockNodes::APB7Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::APB7Prescaler,
                to: ClockNodes::APB7Output,
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
        let input = self.TimPrescalerAPB2_get()?;
        if input > (100000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 100000000),
                from: ClockNodes::TimPrescalerAPB2,
                to: ClockNodes::TimPrescOut2,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::TimPrescalerAPB2,
                to: ClockNodes::TimPrescOut2,
            });
        }
        Ok(input)
    }
    fn SPI1Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI1Mult {
            SPI1MultConf::APB2Prescaler => return self.APB2Prescaler_get(),
            SPI1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            SPI1MultConf::HSIRC => return self.HSIRC_get(),
        };
    }
    pub fn SPI1output_get(&self) -> Result<f32, ClockError> {
        self.SPI1Mult_get()
    }
    fn SPI3Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI3Mult {
            SPI3MultConf::APB7Output => return self.APB7Output_get(),
            SPI3MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            SPI3MultConf::HSIRC => return self.HSIRC_get(),
        };
    }
    pub fn SPI3output_get(&self) -> Result<f32, ClockError> {
        self.SPI3Mult_get()
    }
    fn PLLN_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLM_get()? as f32;
        let frac = self.PLLFRACN_get()? as f32;
        let frac_max = PLLFRACNConf::max() as f32;
        let value = self.PLLN.get()? as f32;
        let ret = (input * (value + (frac / frac_max)));
        Ok(ret as f32)
    }

    pub fn PLLFRACN_get(&self) -> Result<f32, ClockError> {
        self.PLLFRACN.get()
    }
    fn PLL1P_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLN_get()? as f32;
        let value = self.PLL1P.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn PLLPoutput_get(&self) -> Result<f32, ClockError> {
        self.PLL1P_get()
    }
    fn PLL1Q_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLN_get()? as f32;
        let value = self.PLL1Q.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn PLLQoutput_get(&self) -> Result<f32, ClockError> {
        self.PLL1Q_get()
    }
    fn PLL1R_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLN_get()? as f32;
        let value = self.PLL1R.get()? as f32;
        Ok((input / value) as f32)
    }
}
