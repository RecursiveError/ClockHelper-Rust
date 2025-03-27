#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ClockNodes {
    None,
    HSIRC,
    HSEOSC,
    LSIRC,
    LSIDIV,
    LSEOSC,
    MSIRC,
    LPTIM1Mult,
    LPTIM1output,
    USART2Mult,
    USART2output,
    LSCOMult,
    LSCOOutput,
    HSEPRESC,
    SysClkSource,
    SysCLKOutput,
    PLLSource,
    PLLM,
    HSERTCDevisor,
    RTCClkSource,
    RTCOutput,
    LCDOutput,
    IWDGOutput,
    USART1Mult,
    USART1output,
    LPUART1Mult,
    LPUART1output,
    LPTIM2Mult,
    LPTIM2output,
    LPTIM3Mult,
    LPTIM3output,
    RNGMult,
    RNGoutput,
    I2C2Mult,
    I2C2output,
    I2C1Mult,
    I2C1output,
    I2C3Mult,
    I2C3output,
    I2S2Mult,
    I2S2output,
    ADCMult,
    ADCoutput,
    MCOMult,
    MCODiv,
    MCOPin,
    AHB3Prescaler,
    AHB3Output,
    APB3Output,
    AHBPrescaler,
    PWRCLKoutput,
    AHBOutput,
    HCLKOutput,
    CortexPrescaler,
    CortexSysOutput,
    FCLKCortexOutput,
    APB1Prescaler,
    I2S_CKIN,
    APB1Output,
    TimPrescalerAPB1,
    TimPrescOut1,
    APB2Prescaler,
    APB2Output,
    TimPrescalerAPB2,
    TimPrescOut2,
    PLLN,
    PLLP,
    PLLPoutput,
    PLLQ,
    PLLQoutput,
    PLLR,
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
        32000000
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
        31040
    }

    pub const fn max() -> u32 {
        32960
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
pub enum MSIRCConf {
    CLOCK_100,
    CLOCK_200,
    CLOCK_400,
    CLOCK_800,
    CLOCK_1000,
    CLOCK_2000,
    CLOCK_4000,
    CLOCK_8000,
    CLOCK_16000,
    CLOCK_24000,
    CLOCK_32000,
    CLOCK_48000,
}

impl MSIRCConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MSIRCConf::CLOCK_100 => return Ok(100.0),
            MSIRCConf::CLOCK_200 => return Ok(200.0),
            MSIRCConf::CLOCK_400 => return Ok(400.0),
            MSIRCConf::CLOCK_800 => return Ok(800.0),
            MSIRCConf::CLOCK_1000 => return Ok(1000.0),
            MSIRCConf::CLOCK_2000 => return Ok(2000.0),
            MSIRCConf::CLOCK_4000 => return Ok(4000.0),
            MSIRCConf::CLOCK_8000 => return Ok(8000.0),
            MSIRCConf::CLOCK_16000 => return Ok(16000.0),
            MSIRCConf::CLOCK_24000 => return Ok(24000.0),
            MSIRCConf::CLOCK_32000 => return Ok(32000.0),
            MSIRCConf::CLOCK_48000 => return Ok(48000.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM1MultConf {
    APB1Prescaler,
    LSIDIV,
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
pub enum LSCOMultConf {
    LSIDIV,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum HSEPRESCConf {
    Value(u32),
}

impl HSEPRESCConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        2
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            HSEPRESCConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::HSEPRESC,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::HSEPRESC,
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
    MSIRC,
    HSIRC,
    HSEPRESC,
    PLLR,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSourceConf {
    MSIRC,
    HSIRC,
    HSEPRESC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLMConf {
    DIV1,
    DIV2,
    DIV3,
    DIV4,
    DIV5,
    DIV6,
    DIV7,
    DIV8,
}

impl PLLMConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLMConf::DIV1 => return Ok(1.0),
            PLLMConf::DIV2 => return Ok(2.0),
            PLLMConf::DIV3 => return Ok(3.0),
            PLLMConf::DIV4 => return Ok(4.0),
            PLLMConf::DIV5 => return Ok(5.0),
            PLLMConf::DIV6 => return Ok(6.0),
            PLLMConf::DIV7 => return Ok(7.0),
            PLLMConf::DIV8 => return Ok(8.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum RTCClkSourceConf {
    HSERTCDevisor,
    LSEOSC,
    LSIDIV,
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
    APB1Prescaler,
    SysCLKOutput,
    HSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM2MultConf {
    APB1Prescaler,
    LSIDIV,
    HSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM3MultConf {
    APB1Prescaler,
    LSIDIV,
    HSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum RNGMultConf {
    PLLQ,
    LSIDIV,
    LSEOSC,
    MSIRC,
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
pub enum I2C1MultConf {
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
pub enum I2S2MultConf {
    PLLQ,
    HSIRC,
    I2S_CKIN,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ADCMultConf {
    HSIRC,
    PLLP,
    SysCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCOMultConf {
    LSEOSC,
    LSIDIV,
    HSEOSC,
    HSIRC,
    PLLR,
    SysCLKOutput,
    MSIRC,
    PLLP,
    PLLQ,
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
pub enum AHB3PrescalerConf {
    DIV1,
    DIV2,
    DIV3,
    DIV4,
    DIV5,
    DIV6,
    DIV8,
    DIV10,
    DIV16,
    DIV32,
    DIV64,
    DIV128,
    DIV256,
    DIV512,
}

impl AHB3PrescalerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            AHB3PrescalerConf::DIV1 => return Ok(1.0),
            AHB3PrescalerConf::DIV2 => return Ok(2.0),
            AHB3PrescalerConf::DIV3 => return Ok(3.0),
            AHB3PrescalerConf::DIV4 => return Ok(4.0),
            AHB3PrescalerConf::DIV5 => return Ok(5.0),
            AHB3PrescalerConf::DIV6 => return Ok(6.0),
            AHB3PrescalerConf::DIV8 => return Ok(8.0),
            AHB3PrescalerConf::DIV10 => return Ok(10.0),
            AHB3PrescalerConf::DIV16 => return Ok(16.0),
            AHB3PrescalerConf::DIV32 => return Ok(32.0),
            AHB3PrescalerConf::DIV64 => return Ok(64.0),
            AHB3PrescalerConf::DIV128 => return Ok(128.0),
            AHB3PrescalerConf::DIV256 => return Ok(256.0),
            AHB3PrescalerConf::DIV512 => return Ok(512.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum AHBPrescalerConf {
    DIV1,
    DIV2,
    DIV3,
    DIV4,
    DIV5,
    DIV6,
    DIV8,
    DIV10,
    DIV16,
    DIV32,
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
            AHBPrescalerConf::DIV3 => return Ok(3.0),
            AHBPrescalerConf::DIV4 => return Ok(4.0),
            AHBPrescalerConf::DIV5 => return Ok(5.0),
            AHBPrescalerConf::DIV6 => return Ok(6.0),
            AHBPrescalerConf::DIV8 => return Ok(8.0),
            AHBPrescalerConf::DIV10 => return Ok(10.0),
            AHBPrescalerConf::DIV16 => return Ok(16.0),
            AHBPrescalerConf::DIV32 => return Ok(32.0),
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
        6
    }

    pub const fn max() -> u32 {
        127
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
}

impl PLLPConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLPConf::DIV2 => return Ok(2.0),
            PLLPConf::DIV3 => return Ok(3.0),
            PLLPConf::DIV4 => return Ok(4.0),
            PLLPConf::DIV5 => return Ok(5.0),
            PLLPConf::DIV6 => return Ok(6.0),
            PLLPConf::DIV7 => return Ok(7.0),
            PLLPConf::DIV8 => return Ok(8.0),
            PLLPConf::DIV9 => return Ok(9.0),
            PLLPConf::DIV10 => return Ok(10.0),
            PLLPConf::DIV11 => return Ok(11.0),
            PLLPConf::DIV12 => return Ok(12.0),
            PLLPConf::DIV13 => return Ok(13.0),
            PLLPConf::DIV14 => return Ok(14.0),
            PLLPConf::DIV15 => return Ok(15.0),
            PLLPConf::DIV16 => return Ok(16.0),
            PLLPConf::DIV17 => return Ok(17.0),
            PLLPConf::DIV18 => return Ok(18.0),
            PLLPConf::DIV19 => return Ok(19.0),
            PLLPConf::DIV20 => return Ok(20.0),
            PLLPConf::DIV21 => return Ok(21.0),
            PLLPConf::DIV22 => return Ok(22.0),
            PLLPConf::DIV23 => return Ok(23.0),
            PLLPConf::DIV24 => return Ok(24.0),
            PLLPConf::DIV25 => return Ok(25.0),
            PLLPConf::DIV26 => return Ok(26.0),
            PLLPConf::DIV27 => return Ok(27.0),
            PLLPConf::DIV28 => return Ok(28.0),
            PLLPConf::DIV29 => return Ok(29.0),
            PLLPConf::DIV30 => return Ok(30.0),
            PLLPConf::DIV31 => return Ok(31.0),
            PLLPConf::DIV32 => return Ok(32.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLQConf {
    DIV2,
    DIV3,
    DIV4,
    DIV5,
    DIV6,
    DIV7,
    DIV8,
}

impl PLLQConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLQConf::DIV2 => return Ok(2.0),
            PLLQConf::DIV3 => return Ok(3.0),
            PLLQConf::DIV4 => return Ok(4.0),
            PLLQConf::DIV5 => return Ok(5.0),
            PLLQConf::DIV6 => return Ok(6.0),
            PLLQConf::DIV7 => return Ok(7.0),
            PLLQConf::DIV8 => return Ok(8.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLRConf {
    DIV2,
    DIV3,
    DIV4,
    DIV5,
    DIV6,
    DIV7,
    DIV8,
}

impl PLLRConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLRConf::DIV2 => return Ok(2.0),
            PLLRConf::DIV3 => return Ok(3.0),
            PLLRConf::DIV4 => return Ok(4.0),
            PLLRConf::DIV5 => return Ok(5.0),
            PLLRConf::DIV6 => return Ok(6.0),
            PLLRConf::DIV7 => return Ok(7.0),
            PLLRConf::DIV8 => return Ok(8.0),
        }
    }
}
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ClockTree {
    pub HSEOSC: HSEOSCConf,
    pub LSIRC: LSIRCConf,
    pub LSIDIV: LSIDIVConf,
    pub LSEOSC: LSEOSCConf,
    pub MSIRC: MSIRCConf,
    pub LPTIM1Mult: LPTIM1MultConf,
    pub USART2Mult: USART2MultConf,
    pub LSCOMult: LSCOMultConf,
    pub HSEPRESC: HSEPRESCConf,
    pub SysClkSource: SysClkSourceConf,
    pub PLLSource: PLLSourceConf,
    pub PLLM: PLLMConf,
    pub RTCClkSource: RTCClkSourceConf,
    pub USART1Mult: USART1MultConf,
    pub LPUART1Mult: LPUART1MultConf,
    pub LPTIM2Mult: LPTIM2MultConf,
    pub LPTIM3Mult: LPTIM3MultConf,
    pub RNGMult: RNGMultConf,
    pub I2C2Mult: I2C2MultConf,
    pub I2C1Mult: I2C1MultConf,
    pub I2C3Mult: I2C3MultConf,
    pub I2S2Mult: I2S2MultConf,
    pub ADCMult: ADCMultConf,
    pub MCOMult: MCOMultConf,
    pub MCODiv: MCODivConf,
    pub AHB3Prescaler: AHB3PrescalerConf,
    pub AHBPrescaler: AHBPrescalerConf,
    pub CortexPrescaler: CortexPrescalerConf,
    pub APB1Prescaler: APB1PrescalerConf,
    pub APB2Prescaler: APB2PrescalerConf,
    pub PLLN: PLLNConf,
    pub PLLP: PLLPConf,
    pub PLLQ: PLLQConf,
    pub PLLR: PLLRConf,
}

impl Default for ClockTree {
    fn default() -> Self {
        Self {
            HSEOSC: HSEOSCConf::Value(8000000),
            LSIRC: LSIRCConf::Value(32000),
            LSIDIV: LSIDIVConf::DIV1,
            LSEOSC: LSEOSCConf::Value(32768),
            MSIRC: MSIRCConf::CLOCK_4000,
            LPTIM1Mult: LPTIM1MultConf::APB1Prescaler,
            USART2Mult: USART2MultConf::APB1Prescaler,
            LSCOMult: LSCOMultConf::LSIDIV,
            HSEPRESC: HSEPRESCConf::Value(1),
            SysClkSource: SysClkSourceConf::MSIRC,
            PLLSource: PLLSourceConf::MSIRC,
            PLLM: PLLMConf::DIV1,
            RTCClkSource: RTCClkSourceConf::LSIDIV,
            USART1Mult: USART1MultConf::APB2Prescaler,
            LPUART1Mult: LPUART1MultConf::APB1Prescaler,
            LPTIM2Mult: LPTIM2MultConf::APB1Prescaler,
            LPTIM3Mult: LPTIM3MultConf::APB1Prescaler,
            RNGMult: RNGMultConf::LSIDIV,
            I2C2Mult: I2C2MultConf::APB1Prescaler,
            I2C1Mult: I2C1MultConf::APB1Prescaler,
            I2C3Mult: I2C3MultConf::APB1Prescaler,
            I2S2Mult: I2S2MultConf::HSIRC,
            ADCMult: ADCMultConf::HSIRC,
            MCOMult: MCOMultConf::SysCLKOutput,
            MCODiv: MCODivConf::DIV1,
            AHB3Prescaler: AHB3PrescalerConf::DIV1,
            AHBPrescaler: AHBPrescalerConf::DIV1,
            CortexPrescaler: CortexPrescalerConf::DIV1,
            APB1Prescaler: APB1PrescalerConf::DIV1,
            APB2Prescaler: APB2PrescalerConf::DIV1,
            PLLN: PLLNConf::Value(8),
            PLLP: PLLPConf::DIV2,
            PLLQ: PLLQConf::DIV2,
            PLLR: PLLRConf::DIV2,
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
    fn LSIDIV_get(&self) -> Result<f32, ClockError> {
        let input = self.LSIRC_get()? as f32;
        let value = self.LSIDIV.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn LSEOSC_get(&self) -> Result<f32, ClockError> {
        self.LSEOSC.get()
    }
    pub fn MSIRC_get(&self) -> Result<f32, ClockError> {
        self.MSIRC.get()
    }
    fn LPTIM1Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM1Mult {
            LPTIM1MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            LPTIM1MultConf::LSIDIV => return self.LSIDIV_get(),
            LPTIM1MultConf::HSIRC => return self.HSIRC_get(),
            LPTIM1MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn LPTIM1output_get(&self) -> Result<f32, ClockError> {
        self.LPTIM1Mult_get()
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
    fn LSCOMult_get(&self) -> Result<f32, ClockError> {
        match self.LSCOMult {
            LSCOMultConf::LSIDIV => return self.LSIDIV_get(),
            LSCOMultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn LSCOOutput_get(&self) -> Result<f32, ClockError> {
        self.LSCOMult_get()
    }
    fn HSEPRESC_get(&self) -> Result<f32, ClockError> {
        let input = self.HSEOSC_get()? as f32;
        let value = self.HSEPRESC.get()? as f32;
        Ok((input / value) as f32)
    }

    fn SysClkSource_get(&self) -> Result<f32, ClockError> {
        match self.SysClkSource {
            SysClkSourceConf::MSIRC => return self.MSIRC_get(),
            SysClkSourceConf::HSIRC => return self.HSIRC_get(),
            SysClkSourceConf::HSEPRESC => return self.HSEPRESC_get(),
            SysClkSourceConf::PLLR => return self.PLLR_get(),
        };
    }
    pub fn SysCLKOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SysClkSource_get()?;
        if input > (48000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48000000),
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
            PLLSourceConf::MSIRC => return self.MSIRC_get(),
            PLLSourceConf::HSIRC => return self.HSIRC_get(),
            PLLSourceConf::HSEPRESC => return self.HSEPRESC_get(),
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
            RTCClkSourceConf::LSIDIV => return self.LSIDIV_get(),
        };
    }
    pub fn RTCOutput_get(&self) -> Result<f32, ClockError> {
        self.RTCClkSource_get()
    }
    pub fn LCDOutput_get(&self) -> Result<f32, ClockError> {
        self.RTCClkSource_get()
    }
    pub fn IWDGOutput_get(&self) -> Result<f32, ClockError> {
        self.LSIDIV_get()
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
            LPUART1MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            LPUART1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            LPUART1MultConf::HSIRC => return self.HSIRC_get(),
            LPUART1MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn LPUART1output_get(&self) -> Result<f32, ClockError> {
        self.LPUART1Mult_get()
    }
    fn LPTIM2Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM2Mult {
            LPTIM2MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            LPTIM2MultConf::LSIDIV => return self.LSIDIV_get(),
            LPTIM2MultConf::HSIRC => return self.HSIRC_get(),
            LPTIM2MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn LPTIM2output_get(&self) -> Result<f32, ClockError> {
        self.LPTIM2Mult_get()
    }
    fn LPTIM3Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM3Mult {
            LPTIM3MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            LPTIM3MultConf::LSIDIV => return self.LSIDIV_get(),
            LPTIM3MultConf::HSIRC => return self.HSIRC_get(),
            LPTIM3MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn LPTIM3output_get(&self) -> Result<f32, ClockError> {
        self.LPTIM3Mult_get()
    }
    fn RNGMult_get(&self) -> Result<f32, ClockError> {
        match self.RNGMult {
            RNGMultConf::PLLQ => return self.PLLQ_get(),
            RNGMultConf::LSIDIV => return self.LSIDIV_get(),
            RNGMultConf::LSEOSC => return self.LSEOSC_get(),
            RNGMultConf::MSIRC => return self.MSIRC_get(),
        };
    }
    pub fn RNGoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.RNGMult_get()?;
        if input > (48000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48000000),
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
    fn I2C1Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C1Mult {
            I2C1MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            I2C1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            I2C1MultConf::HSIRC => return self.HSIRC_get(),
        };
    }
    pub fn I2C1output_get(&self) -> Result<f32, ClockError> {
        self.I2C3Mult_get()
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
    fn I2S2Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2S2Mult {
            I2S2MultConf::PLLQ => return self.PLLQ_get(),
            I2S2MultConf::HSIRC => return self.HSIRC_get(),
            I2S2MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    pub fn I2S2output_get(&self) -> Result<f32, ClockError> {
        self.I2S2Mult_get()
    }
    fn ADCMult_get(&self) -> Result<f32, ClockError> {
        match self.ADCMult {
            ADCMultConf::HSIRC => return self.HSIRC_get(),
            ADCMultConf::PLLP => return self.PLLP_get(),
            ADCMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
        };
    }
    pub fn ADCoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.ADCMult_get()?;
        if input > (48000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48000000),
                from: ClockNodes::ADCMult,
                to: ClockNodes::ADCoutput,
            });
        } else if input < (1400000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 1400000),
                from: ClockNodes::ADCMult,
                to: ClockNodes::ADCoutput,
            });
        }
        Ok(input)
    }
    fn MCOMult_get(&self) -> Result<f32, ClockError> {
        match self.MCOMult {
            MCOMultConf::LSEOSC => return self.LSEOSC_get(),
            MCOMultConf::LSIDIV => return self.LSIDIV_get(),
            MCOMultConf::HSEOSC => return self.HSEOSC_get(),
            MCOMultConf::HSIRC => return self.HSIRC_get(),
            MCOMultConf::PLLR => return self.PLLR_get(),
            MCOMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            MCOMultConf::MSIRC => return self.MSIRC_get(),
            MCOMultConf::PLLP => return self.PLLP_get(),
            MCOMultConf::PLLQ => return self.PLLQ_get(),
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
    fn AHB3Prescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()? as f32;
        let value = self.AHB3Prescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn AHB3Output_get(&self) -> Result<f32, ClockError> {
        let input = self.AHB3Prescaler_get()?;
        if input > (48000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48000000),
                from: ClockNodes::AHB3Prescaler,
                to: ClockNodes::AHB3Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::AHB3Prescaler,
                to: ClockNodes::AHB3Output,
            });
        }
        Ok(input)
    }
    pub fn APB3Output_get(&self) -> Result<f32, ClockError> {
        let input = self.AHB3Prescaler_get()?;
        if input > (48000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48000000),
                from: ClockNodes::AHB3Prescaler,
                to: ClockNodes::APB3Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::AHB3Prescaler,
                to: ClockNodes::APB3Output,
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
        let input = self.SysCLKOutput_get()?;
        if input > (48000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48000000),
                from: ClockNodes::SysCLKOutput,
                to: ClockNodes::PWRCLKoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SysCLKOutput,
                to: ClockNodes::PWRCLKoutput,
            });
        }
        Ok(input)
    }
    pub fn AHBOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBPrescaler_get()?;
        if input > (48000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48000000),
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

    pub fn I2S_CKIN_get(&self) -> Result<f32, ClockError> {
        Ok(48000 as f32)
    }
    pub fn APB1Output_get(&self) -> Result<f32, ClockError> {
        let input = self.APB1Prescaler_get()?;
        if input > (48000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48000000),
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
        if input > (48000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48000000),
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

    pub fn PLLPoutput_get(&self) -> Result<f32, ClockError> {
        self.PLLP_get()
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
}
