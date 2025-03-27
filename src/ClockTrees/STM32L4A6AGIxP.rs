#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ClockNodes {
    None,
    HSIRC,
    HSI48RC,
    HSEOSC,
    LSIRC,
    LSEOSC,
    MSIRC,
    SAI1_EXT,
    SAI2_EXT,
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
    USART2Mult,
    USART2output,
    USART3Mult,
    USART3output,
    UART4Mult,
    UART4output,
    UART5Mult,
    UART5output,
    LPUART1Mult,
    LPUART1output,
    LPTIM1Mult,
    LPTIM1output,
    LPTIM2Mult,
    LPTIM2output,
    SWPMIMult,
    SWPMIoutput,
    DFSDMMult,
    DFSDMoutput,
    ADCMult,
    ADCoutput,
    CK48Mult,
    CK48output,
    SDMMCoutput,
    RNGoutput,
    I2C1Mult,
    I2C1output,
    I2C2Mult,
    I2C2output,
    I2C3Mult,
    I2C3output,
    SAI1Mult,
    SAI1output,
    SAI2Mult,
    SAI2output,
    I2C4Mult,
    I2C4output,
    MCOMult,
    MCODiv,
    MCOPin,
    LSCOMult,
    LSCOOutput,
    AHBPrescaler,
    PWRCLKoutput,
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
    PLLN,
    PLLP,
    PLLPoutput,
    PLLQ,
    PLLQoutput,
    PLLR,
    PLLSAI1N,
    PLLSAI1P,
    PLLSAI1Poutput,
    PLLSAI1Q,
    PLLSAI1Qoutput,
    PLLSAI1R,
    PLLSAI1Routput,
    PLLSAI2N,
    PLLSAI2P,
    PLLSAI2Poutput,
    PLLSAI2R,
    PLLSAI2Routput,
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
pub enum SysClkSourceConf {
    MSIRC,
    HSIRC,
    HSEOSC,
    PLLR,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSourceConf {
    MSIRC,
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
    LSIRC,
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
pub enum LPUART1MultConf {
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
pub enum LPTIM2MultConf {
    APB1Prescaler,
    LSIRC,
    HSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SWPMIMultConf {
    APB1Prescaler,
    HSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DFSDMMultConf {
    APB1Prescaler,
    SysCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ADCMultConf {
    PLLSAI1R,
    PLLSAI2R,
    SysCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CK48MultConf {
    PLLSAI1Q,
    PLLQ,
    MSIRC,
    HSI48RC,
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
pub enum SAI1MultConf {
    PLLSAI1P,
    PLLSAI2P,
    PLLP,
    SAI1_EXT,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SAI2MultConf {
    PLLSAI1P,
    PLLSAI2P,
    PLLP,
    SAI2_EXT,
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
pub enum MCOMultConf {
    LSEOSC,
    LSIRC,
    HSEOSC,
    HSIRC,
    PLLR,
    SysCLKOutput,
    MSIRC,
    HSI48RC,
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
    LSIRC,
    LSEOSC,
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
pub enum PLLNConf {
    Value(u32),
}

impl PLLNConf {
    pub const fn min() -> u32 {
        8
    }

    pub const fn max() -> u32 {
        86
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
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLQConf {
    DIV2,
    DIV4,
    DIV6,
    DIV8,
}

impl PLLQConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLQConf::DIV2 => return Ok(2.0),
            PLLQConf::DIV4 => return Ok(4.0),
            PLLQConf::DIV6 => return Ok(6.0),
            PLLQConf::DIV8 => return Ok(8.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLRConf {
    DIV2,
    DIV4,
    DIV6,
    DIV8,
}

impl PLLRConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLRConf::DIV2 => return Ok(2.0),
            PLLRConf::DIV4 => return Ok(4.0),
            PLLRConf::DIV6 => return Ok(6.0),
            PLLRConf::DIV8 => return Ok(8.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSAI1NConf {
    Value(u32),
}

impl PLLSAI1NConf {
    pub const fn min() -> u32 {
        8
    }

    pub const fn max() -> u32 {
        86
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLSAI1NConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLSAI1N,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLSAI1N,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSAI1PConf {
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

impl PLLSAI1PConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLSAI1PConf::DIV2 => return Ok(2.0),
            PLLSAI1PConf::DIV3 => return Ok(3.0),
            PLLSAI1PConf::DIV4 => return Ok(4.0),
            PLLSAI1PConf::DIV5 => return Ok(5.0),
            PLLSAI1PConf::DIV6 => return Ok(6.0),
            PLLSAI1PConf::DIV7 => return Ok(7.0),
            PLLSAI1PConf::DIV8 => return Ok(8.0),
            PLLSAI1PConf::DIV9 => return Ok(9.0),
            PLLSAI1PConf::DIV10 => return Ok(10.0),
            PLLSAI1PConf::DIV11 => return Ok(11.0),
            PLLSAI1PConf::DIV12 => return Ok(12.0),
            PLLSAI1PConf::DIV13 => return Ok(13.0),
            PLLSAI1PConf::DIV14 => return Ok(14.0),
            PLLSAI1PConf::DIV15 => return Ok(15.0),
            PLLSAI1PConf::DIV16 => return Ok(16.0),
            PLLSAI1PConf::DIV17 => return Ok(17.0),
            PLLSAI1PConf::DIV18 => return Ok(18.0),
            PLLSAI1PConf::DIV19 => return Ok(19.0),
            PLLSAI1PConf::DIV20 => return Ok(20.0),
            PLLSAI1PConf::DIV21 => return Ok(21.0),
            PLLSAI1PConf::DIV22 => return Ok(22.0),
            PLLSAI1PConf::DIV23 => return Ok(23.0),
            PLLSAI1PConf::DIV24 => return Ok(24.0),
            PLLSAI1PConf::DIV25 => return Ok(25.0),
            PLLSAI1PConf::DIV26 => return Ok(26.0),
            PLLSAI1PConf::DIV27 => return Ok(27.0),
            PLLSAI1PConf::DIV28 => return Ok(28.0),
            PLLSAI1PConf::DIV29 => return Ok(29.0),
            PLLSAI1PConf::DIV30 => return Ok(30.0),
            PLLSAI1PConf::DIV31 => return Ok(31.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSAI1QConf {
    DIV2,
    DIV4,
    DIV6,
    DIV8,
}

impl PLLSAI1QConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLSAI1QConf::DIV2 => return Ok(2.0),
            PLLSAI1QConf::DIV4 => return Ok(4.0),
            PLLSAI1QConf::DIV6 => return Ok(6.0),
            PLLSAI1QConf::DIV8 => return Ok(8.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSAI1RConf {
    DIV2,
    DIV4,
    DIV6,
    DIV8,
}

impl PLLSAI1RConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLSAI1RConf::DIV2 => return Ok(2.0),
            PLLSAI1RConf::DIV4 => return Ok(4.0),
            PLLSAI1RConf::DIV6 => return Ok(6.0),
            PLLSAI1RConf::DIV8 => return Ok(8.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSAI2NConf {
    Value(u32),
}

impl PLLSAI2NConf {
    pub const fn min() -> u32 {
        8
    }

    pub const fn max() -> u32 {
        86
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLSAI2NConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLSAI2N,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLLSAI2N,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSAI2PConf {
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

impl PLLSAI2PConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLSAI2PConf::DIV2 => return Ok(2.0),
            PLLSAI2PConf::DIV3 => return Ok(3.0),
            PLLSAI2PConf::DIV4 => return Ok(4.0),
            PLLSAI2PConf::DIV5 => return Ok(5.0),
            PLLSAI2PConf::DIV6 => return Ok(6.0),
            PLLSAI2PConf::DIV7 => return Ok(7.0),
            PLLSAI2PConf::DIV8 => return Ok(8.0),
            PLLSAI2PConf::DIV9 => return Ok(9.0),
            PLLSAI2PConf::DIV10 => return Ok(10.0),
            PLLSAI2PConf::DIV11 => return Ok(11.0),
            PLLSAI2PConf::DIV12 => return Ok(12.0),
            PLLSAI2PConf::DIV13 => return Ok(13.0),
            PLLSAI2PConf::DIV14 => return Ok(14.0),
            PLLSAI2PConf::DIV15 => return Ok(15.0),
            PLLSAI2PConf::DIV16 => return Ok(16.0),
            PLLSAI2PConf::DIV17 => return Ok(17.0),
            PLLSAI2PConf::DIV18 => return Ok(18.0),
            PLLSAI2PConf::DIV19 => return Ok(19.0),
            PLLSAI2PConf::DIV20 => return Ok(20.0),
            PLLSAI2PConf::DIV21 => return Ok(21.0),
            PLLSAI2PConf::DIV22 => return Ok(22.0),
            PLLSAI2PConf::DIV23 => return Ok(23.0),
            PLLSAI2PConf::DIV24 => return Ok(24.0),
            PLLSAI2PConf::DIV25 => return Ok(25.0),
            PLLSAI2PConf::DIV26 => return Ok(26.0),
            PLLSAI2PConf::DIV27 => return Ok(27.0),
            PLLSAI2PConf::DIV28 => return Ok(28.0),
            PLLSAI2PConf::DIV29 => return Ok(29.0),
            PLLSAI2PConf::DIV30 => return Ok(30.0),
            PLLSAI2PConf::DIV31 => return Ok(31.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSAI2RConf {
    DIV2,
    DIV4,
    DIV6,
    DIV8,
}

impl PLLSAI2RConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLSAI2RConf::DIV2 => return Ok(2.0),
            PLLSAI2RConf::DIV4 => return Ok(4.0),
            PLLSAI2RConf::DIV6 => return Ok(6.0),
            PLLSAI2RConf::DIV8 => return Ok(8.0),
        }
    }
}
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ClockTree {
    pub HSEOSC: HSEOSCConf,
    pub LSEOSC: LSEOSCConf,
    pub MSIRC: MSIRCConf,
    pub SysClkSource: SysClkSourceConf,
    pub PLLSource: PLLSourceConf,
    pub PLLM: PLLMConf,
    pub RTCClkSource: RTCClkSourceConf,
    pub USART1Mult: USART1MultConf,
    pub USART2Mult: USART2MultConf,
    pub USART3Mult: USART3MultConf,
    pub UART4Mult: UART4MultConf,
    pub UART5Mult: UART5MultConf,
    pub LPUART1Mult: LPUART1MultConf,
    pub LPTIM1Mult: LPTIM1MultConf,
    pub LPTIM2Mult: LPTIM2MultConf,
    pub SWPMIMult: SWPMIMultConf,
    pub DFSDMMult: DFSDMMultConf,
    pub ADCMult: ADCMultConf,
    pub CK48Mult: CK48MultConf,
    pub I2C1Mult: I2C1MultConf,
    pub I2C2Mult: I2C2MultConf,
    pub I2C3Mult: I2C3MultConf,
    pub SAI1Mult: SAI1MultConf,
    pub SAI2Mult: SAI2MultConf,
    pub I2C4Mult: I2C4MultConf,
    pub MCOMult: MCOMultConf,
    pub MCODiv: MCODivConf,
    pub LSCOMult: LSCOMultConf,
    pub AHBPrescaler: AHBPrescalerConf,
    pub CortexPrescaler: CortexPrescalerConf,
    pub APB1Prescaler: APB1PrescalerConf,
    pub APB2Prescaler: APB2PrescalerConf,
    pub PLLN: PLLNConf,
    pub PLLP: PLLPConf,
    pub PLLQ: PLLQConf,
    pub PLLR: PLLRConf,
    pub PLLSAI1N: PLLSAI1NConf,
    pub PLLSAI1P: PLLSAI1PConf,
    pub PLLSAI1Q: PLLSAI1QConf,
    pub PLLSAI1R: PLLSAI1RConf,
    pub PLLSAI2N: PLLSAI2NConf,
    pub PLLSAI2P: PLLSAI2PConf,
    pub PLLSAI2R: PLLSAI2RConf,
}

impl Default for ClockTree {
    fn default() -> Self {
        Self {
            HSEOSC: HSEOSCConf::Value(8000000),
            LSEOSC: LSEOSCConf::Value(32768),
            MSIRC: MSIRCConf::CLOCK_4000,
            SysClkSource: SysClkSourceConf::MSIRC,
            PLLSource: PLLSourceConf::MSIRC,
            PLLM: PLLMConf::Value(1),
            RTCClkSource: RTCClkSourceConf::LSIRC,
            USART1Mult: USART1MultConf::APB2Prescaler,
            USART2Mult: USART2MultConf::APB1Prescaler,
            USART3Mult: USART3MultConf::APB1Prescaler,
            UART4Mult: UART4MultConf::APB1Prescaler,
            UART5Mult: UART5MultConf::APB1Prescaler,
            LPUART1Mult: LPUART1MultConf::APB1Prescaler,
            LPTIM1Mult: LPTIM1MultConf::APB1Prescaler,
            LPTIM2Mult: LPTIM2MultConf::APB1Prescaler,
            SWPMIMult: SWPMIMultConf::APB1Prescaler,
            DFSDMMult: DFSDMMultConf::APB1Prescaler,
            ADCMult: ADCMultConf::PLLSAI1R,
            CK48Mult: CK48MultConf::PLLSAI1Q,
            I2C1Mult: I2C1MultConf::APB1Prescaler,
            I2C2Mult: I2C2MultConf::APB1Prescaler,
            I2C3Mult: I2C3MultConf::APB1Prescaler,
            SAI1Mult: SAI1MultConf::PLLSAI1P,
            SAI2Mult: SAI2MultConf::PLLSAI1P,
            I2C4Mult: I2C4MultConf::APB1Prescaler,
            MCOMult: MCOMultConf::SysCLKOutput,
            MCODiv: MCODivConf::DIV1,
            LSCOMult: LSCOMultConf::LSIRC,
            AHBPrescaler: AHBPrescalerConf::DIV1,
            CortexPrescaler: CortexPrescalerConf::DIV1,
            APB1Prescaler: APB1PrescalerConf::DIV1,
            APB2Prescaler: APB2PrescalerConf::DIV1,
            PLLN: PLLNConf::Value(8),
            PLLP: PLLPConf::DIV2,
            PLLQ: PLLQConf::DIV2,
            PLLR: PLLRConf::DIV2,
            PLLSAI1N: PLLSAI1NConf::Value(8),
            PLLSAI1P: PLLSAI1PConf::DIV2,
            PLLSAI1Q: PLLSAI1QConf::DIV2,
            PLLSAI1R: PLLSAI1RConf::DIV2,
            PLLSAI2N: PLLSAI2NConf::Value(8),
            PLLSAI2P: PLLSAI2PConf::DIV2,
            PLLSAI2R: PLLSAI2RConf::DIV2,
        }
    }
}
impl ClockTree {
    pub fn HSIRC_get(&self) -> Result<f32, ClockError> {
        Ok(16000000 as f32)
    }
    pub fn HSI48RC_get(&self) -> Result<f32, ClockError> {
        Ok(48000000 as f32)
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
    pub fn MSIRC_get(&self) -> Result<f32, ClockError> {
        self.MSIRC.get()
    }
    pub fn SAI1_EXT_get(&self) -> Result<f32, ClockError> {
        Ok(2097000 as f32)
    }
    pub fn SAI2_EXT_get(&self) -> Result<f32, ClockError> {
        Ok(2097000 as f32)
    }
    fn SysClkSource_get(&self) -> Result<f32, ClockError> {
        match self.SysClkSource {
            SysClkSourceConf::MSIRC => return self.MSIRC_get(),
            SysClkSourceConf::HSIRC => return self.HSIRC_get(),
            SysClkSourceConf::HSEOSC => return self.HSEOSC_get(),
            SysClkSourceConf::PLLR => return self.PLLR_get(),
        };
    }
    pub fn SysCLKOutput_get(&self) -> Result<f32, ClockError> {
        self.SysClkSource_get()
    }
    fn PLLSource_get(&self) -> Result<f32, ClockError> {
        match self.PLLSource {
            PLLSourceConf::MSIRC => return self.MSIRC_get(),
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
        let value = 32 as f32;
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
    pub fn LCDOutput_get(&self) -> Result<f32, ClockError> {
        self.RTCClkSource_get()
    }
    pub fn IWDGOutput_get(&self) -> Result<f32, ClockError> {
        self.LSIRC_get()
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
    fn LPTIM1Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM1Mult {
            LPTIM1MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            LPTIM1MultConf::LSIRC => return self.LSIRC_get(),
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
            LPTIM2MultConf::LSIRC => return self.LSIRC_get(),
            LPTIM2MultConf::HSIRC => return self.HSIRC_get(),
            LPTIM2MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn LPTIM2output_get(&self) -> Result<f32, ClockError> {
        self.LPTIM2Mult_get()
    }
    fn SWPMIMult_get(&self) -> Result<f32, ClockError> {
        match self.SWPMIMult {
            SWPMIMultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            SWPMIMultConf::HSIRC => return self.HSIRC_get(),
        };
    }
    pub fn SWPMIoutput_get(&self) -> Result<f32, ClockError> {
        self.SWPMIMult_get()
    }
    fn DFSDMMult_get(&self) -> Result<f32, ClockError> {
        match self.DFSDMMult {
            DFSDMMultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            DFSDMMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
        };
    }
    pub fn DFSDMoutput_get(&self) -> Result<f32, ClockError> {
        self.DFSDMMult_get()
    }
    fn ADCMult_get(&self) -> Result<f32, ClockError> {
        match self.ADCMult {
            ADCMultConf::PLLSAI1R => return self.PLLSAI1R_get(),
            ADCMultConf::PLLSAI2R => return self.PLLSAI2R_get(),
            ADCMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
        };
    }
    pub fn ADCoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.ADCMult_get()?;
        if input > (80000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 80000000),
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
    fn CK48Mult_get(&self) -> Result<f32, ClockError> {
        match self.CK48Mult {
            CK48MultConf::PLLSAI1Q => return self.PLLSAI1Q_get(),
            CK48MultConf::PLLQ => return self.PLLQ_get(),
            CK48MultConf::MSIRC => return self.MSIRC_get(),
            CK48MultConf::HSI48RC => return self.HSI48RC_get(),
        };
    }
    pub fn CK48output_get(&self) -> Result<f32, ClockError> {
        let input = self.CK48Mult_get()?;
        if input > (48120000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48120000),
                from: ClockNodes::CK48Mult,
                to: ClockNodes::CK48output,
            });
        } else if input < (47880000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 47880000),
                from: ClockNodes::CK48Mult,
                to: ClockNodes::CK48output,
            });
        }
        Ok(input)
    }
    pub fn SDMMCoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.CK48Mult_get()?;
        if input > (50000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 50000000),
                from: ClockNodes::CK48Mult,
                to: ClockNodes::SDMMCoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::CK48Mult,
                to: ClockNodes::SDMMCoutput,
            });
        }
        Ok(input)
    }
    pub fn RNGoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.CK48Mult_get()?;
        if input > (48000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48000000),
                from: ClockNodes::CK48Mult,
                to: ClockNodes::RNGoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::CK48Mult,
                to: ClockNodes::RNGoutput,
            });
        }
        Ok(input)
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
    fn SAI1Mult_get(&self) -> Result<f32, ClockError> {
        match self.SAI1Mult {
            SAI1MultConf::PLLSAI1P => return self.PLLSAI1P_get(),
            SAI1MultConf::PLLSAI2P => return self.PLLSAI2P_get(),
            SAI1MultConf::PLLP => return self.PLLP_get(),
            SAI1MultConf::SAI1_EXT => return self.SAI1_EXT_get(),
        };
    }
    pub fn SAI1output_get(&self) -> Result<f32, ClockError> {
        self.SAI1Mult_get()
    }
    fn SAI2Mult_get(&self) -> Result<f32, ClockError> {
        match self.SAI2Mult {
            SAI2MultConf::PLLSAI1P => return self.PLLSAI1P_get(),
            SAI2MultConf::PLLSAI2P => return self.PLLSAI2P_get(),
            SAI2MultConf::PLLP => return self.PLLP_get(),
            SAI2MultConf::SAI2_EXT => return self.SAI2_EXT_get(),
        };
    }
    pub fn SAI2output_get(&self) -> Result<f32, ClockError> {
        self.SAI2Mult_get()
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
    fn MCOMult_get(&self) -> Result<f32, ClockError> {
        match self.MCOMult {
            MCOMultConf::LSEOSC => return self.LSEOSC_get(),
            MCOMultConf::LSIRC => return self.LSIRC_get(),
            MCOMultConf::HSEOSC => return self.HSEOSC_get(),
            MCOMultConf::HSIRC => return self.HSIRC_get(),
            MCOMultConf::PLLR => return self.PLLR_get(),
            MCOMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            MCOMultConf::MSIRC => return self.MSIRC_get(),
            MCOMultConf::HSI48RC => return self.HSI48RC_get(),
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
            LSCOMultConf::LSIRC => return self.LSIRC_get(),
            LSCOMultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn LSCOOutput_get(&self) -> Result<f32, ClockError> {
        self.LSCOMult_get()
    }
    fn AHBPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()? as f32;
        let value = self.AHBPrescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn PWRCLKoutput_get(&self) -> Result<f32, ClockError> {
        self.SysCLKOutput_get()
    }
    pub fn AHBOutput_get(&self) -> Result<f32, ClockError> {
        self.AHBPrescaler_get()
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
        if input > (80000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 80000000),
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
        if input > (80000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 80000000),
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

    fn PLLSAI1N_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLM_get()? as f32;
        let value = self.PLLSAI1N.get()? as f32;
        Ok((input * value) as f32)
    }

    fn PLLSAI1P_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLSAI1N_get()? as f32;
        let value = self.PLLSAI1P.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn PLLSAI1Poutput_get(&self) -> Result<f32, ClockError> {
        self.PLLSAI1P_get()
    }
    fn PLLSAI1Q_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLSAI1N_get()? as f32;
        let value = self.PLLSAI1Q.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn PLLSAI1Qoutput_get(&self) -> Result<f32, ClockError> {
        self.PLLSAI1Q_get()
    }
    fn PLLSAI1R_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLSAI1N_get()? as f32;
        let value = self.PLLSAI1R.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn PLLSAI1Routput_get(&self) -> Result<f32, ClockError> {
        self.PLLSAI1R_get()
    }
    fn PLLSAI2N_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLM_get()? as f32;
        let value = self.PLLSAI2N.get()? as f32;
        Ok((input * value) as f32)
    }

    fn PLLSAI2P_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLSAI2N_get()? as f32;
        let value = self.PLLSAI2P.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn PLLSAI2Poutput_get(&self) -> Result<f32, ClockError> {
        self.PLLSAI2P_get()
    }
    fn PLLSAI2R_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLSAI2N_get()? as f32;
        let value = self.PLLSAI2R.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn PLLSAI2Routput_get(&self) -> Result<f32, ClockError> {
        self.PLLSAI2R_get()
    }
}
