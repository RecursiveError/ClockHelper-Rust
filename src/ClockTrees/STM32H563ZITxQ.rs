#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ClockNodes {
    None,
    HSIRC,
    HSIDiv,
    CRSCLKoutput,
    HSI48RC,
    HSEOSC,
    LSIRC,
    LSEOSC,
    CSIRC,
    AUDIOCLK,
    SysClkSource,
    SysCLKOutput,
    PLLSource,
    PLL2Source,
    PLL3Source,
    PLLM,
    PLL2M,
    PLL3M,
    HSERTCDevisor,
    RTCClkSource,
    RTCOutput,
    IWDGOutput,
    CSIdivTohdmi,
    CECMult,
    CECoutput,
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
    UART9Mult,
    UART9output,
    UART8Mult,
    UART8output,
    USART10Mult,
    USART10output,
    USART11Mult,
    USART11output,
    UART12Mult,
    UART12output,
    LPUART1Mult,
    LPUART1output,
    LPTIM1Mult,
    LPTIM1output,
    LPTIM2Mult,
    LPTIM2output,
    DACMult,
    DACoutput,
    ADCMult,
    ADCoutput,
    CK48Mult,
    USBoutput,
    SDMMC1Mult,
    SDMMC1Output,
    SDMMC2Mult,
    SDMMC2Output,
    FDCANMult,
    FDCANOutput,
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
    I3C1Mult,
    I3C1output,
    OCTOSPIMMult,
    OCTOSPIMoutput,
    LPTIM3Mult,
    LPTIM3output,
    LPTIM4Mult,
    LPTIM4output,
    LPTIM5Mult,
    LPTIM5output,
    LPTIM6Mult,
    LPTIM6output,
    RNGMult,
    RNGoutput,
    MCOMult,
    MCODiv,
    MCOPin,
    MCO2Mult,
    MCO2Div,
    MCO2Pin,
    LSCOMult,
    LSCOOutput,
    CKPERMult,
    CKPERoutput,
    AHBPrescaler,
    PWRCLKoutput,
    AHBOutput,
    HCLKOutput,
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
    APB3Prescaler,
    APB3Output,
    TimPrescalerAPB2,
    TimPrescOut2,
    hsidivToUCPD,
    UCPD1Output,
    SPI1Mult,
    SPI1output,
    SPI3Mult,
    SPI3output,
    SPI4Mult,
    SPI4output,
    SPI5Mult,
    SPI5output,
    SPI6Mult,
    SPI6output,
    SPI2Mult,
    SPI2output,
    PLLN,
    PLLFRACN,
    PLL1P,
    PLL1Q,
    PLLQoutput,
    PLL1R,
    PLL2N,
    PLL2FRACN,
    PLL2P,
    PLL2Poutput,
    PLL2Q,
    PLL2Qoutput,
    PLL2R,
    PLL2Routput,
    PLL3N,
    PLL3FRACN,
    PLL3P,
    PLL3Poutput,
    PLL3Q,
    PLL3Qoutput,
    PLL3R,
    PLL3Routput,
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
pub enum SysClkSourceConf {
    HSIDiv,
    CSIRC,
    HSEOSC,
    PLL1P,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSourceConf {
    CSIRC,
    HSIDiv,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL2SourceConf {
    CSIRC,
    HSIDiv,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL3SourceConf {
    CSIRC,
    HSIDiv,
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
        63
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
pub enum PLL2MConf {
    Value(u32),
}

impl PLL2MConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        63
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL2MConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL2M,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL2M,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL3MConf {
    Value(u32),
}

impl PLL3MConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        63
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL3MConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL3M,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL3M,
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
            HSERTCDevisorConf::DIV1 => return Ok(1.0),
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
pub enum CECMultConf {
    LSEOSC,
    CSIdivTohdmi,
    LSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART1MultConf {
    APB2Prescaler,
    PLL2Q,
    HSIDiv,
    LSEOSC,
    CSIRC,
    PLL3Q,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART2MultConf {
    APB1Prescaler,
    PLL2Q,
    HSIDiv,
    LSEOSC,
    CSIRC,
    PLL3Q,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART3MultConf {
    APB1Prescaler,
    PLL2Q,
    HSIDiv,
    LSEOSC,
    CSIRC,
    PLL3Q,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum UART4MultConf {
    APB1Prescaler,
    PLL2Q,
    HSIDiv,
    LSEOSC,
    CSIRC,
    PLL3Q,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum UART5MultConf {
    APB1Prescaler,
    PLL2Q,
    HSIDiv,
    LSEOSC,
    CSIRC,
    PLL3Q,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART6MultConf {
    APB1Prescaler,
    PLL2Q,
    HSIDiv,
    LSEOSC,
    CSIRC,
    PLL3Q,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum UART7MultConf {
    APB1Prescaler,
    PLL2Q,
    HSIDiv,
    LSEOSC,
    CSIRC,
    PLL3Q,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum UART9MultConf {
    APB1Prescaler,
    PLL2Q,
    HSIDiv,
    LSEOSC,
    CSIRC,
    PLL3Q,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum UART8MultConf {
    APB1Prescaler,
    PLL2Q,
    HSIDiv,
    LSEOSC,
    CSIRC,
    PLL3Q,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART10MultConf {
    APB1Prescaler,
    PLL2Q,
    HSIDiv,
    LSEOSC,
    CSIRC,
    PLL3Q,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART11MultConf {
    APB1Prescaler,
    PLL2Q,
    HSIDiv,
    LSEOSC,
    CSIRC,
    PLL3Q,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum UART12MultConf {
    APB1Prescaler,
    PLL2Q,
    HSIDiv,
    LSEOSC,
    CSIRC,
    PLL3Q,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPUART1MultConf {
    APB3Output,
    PLL2Q,
    HSIDiv,
    LSEOSC,
    CSIRC,
    PLL3Q,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM1MultConf {
    APB3Output,
    PLL2P,
    LSEOSC,
    LSIRC,
    CKPERMult,
    PLL3R,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM2MultConf {
    APB1Prescaler,
    PLL2P,
    LSEOSC,
    LSIRC,
    CKPERMult,
    PLL3R,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum DACMultConf {
    LSEOSC,
    LSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ADCMultConf {
    AHBOutput,
    SysCLKOutput,
    PLL2R,
    HSEOSC,
    HSIDiv,
    CSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CK48MultConf {
    PLL3Q,
    PLL1Q,
    HSI48RC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SDMMC1MultConf {
    PLL1Q,
    PLL2R,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SDMMC2MultConf {
    PLL1Q,
    PLL2R,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FDCANMultConf {
    PLL1Q,
    PLL2Q,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C1MultConf {
    APB1Prescaler,
    PLL3R,
    HSIDiv,
    CSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C2MultConf {
    APB1Prescaler,
    PLL3R,
    HSIDiv,
    CSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C3MultConf {
    APB3Output,
    PLL3R,
    HSIDiv,
    CSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SAI1MultConf {
    PLL2P,
    PLL3P,
    PLL1Q,
    AUDIOCLK,
    CKPERMult,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SAI2MultConf {
    PLL2P,
    PLL3P,
    PLL1Q,
    AUDIOCLK,
    CKPERMult,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C4MultConf {
    APB3Output,
    PLL3R,
    HSIDiv,
    CSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I3C1MultConf {
    APB1Prescaler,
    PLL3R,
    HSIDiv,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum OCTOSPIMMultConf {
    AHBOutput,
    PLL1Q,
    PLL2R,
    CKPERMult,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM3MultConf {
    APB3Output,
    PLL2P,
    LSEOSC,
    LSIRC,
    CKPERMult,
    PLL3R,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM4MultConf {
    APB3Output,
    PLL2P,
    LSEOSC,
    LSIRC,
    CKPERMult,
    PLL3R,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM5MultConf {
    APB3Output,
    PLL2P,
    LSEOSC,
    LSIRC,
    CKPERMult,
    PLL3R,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM6MultConf {
    APB3Output,
    PLL2P,
    LSEOSC,
    LSIRC,
    CKPERMult,
    PLL3R,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum RNGMultConf {
    HSI48RC,
    PLL1Q,
    LSEOSC,
    LSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCOMultConf {
    LSEOSC,
    HSEOSC,
    HSIDiv,
    PLL1Q,
    HSI48RC,
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
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCO2MultConf {
    LSIRC,
    HSEOSC,
    CSIRC,
    PLL1P,
    PLL2P,
    SysCLKOutput,
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
pub enum LSCOMultConf {
    LSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CKPERMultConf {
    HSIDiv,
    HSEOSC,
    CSIRC,
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
pub enum CortexCLockSelectionConf {
    CortexPrescaler,
    LSEOSC,
    LSIRC,
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
pub enum APB3PrescalerConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl APB3PrescalerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            APB3PrescalerConf::DIV1 => return Ok(1.0),
            APB3PrescalerConf::DIV2 => return Ok(2.0),
            APB3PrescalerConf::DIV4 => return Ok(4.0),
            APB3PrescalerConf::DIV8 => return Ok(8.0),
            APB3PrescalerConf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI1MultConf {
    PLL1Q,
    PLL2P,
    PLL3P,
    AUDIOCLK,
    CKPERMult,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI3MultConf {
    PLL1Q,
    PLL2P,
    PLL3P,
    AUDIOCLK,
    CKPERMult,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI4MultConf {
    APB2Prescaler,
    PLL2Q,
    HSIDiv,
    CSIRC,
    HSEOSC,
    PLL3Q,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI5MultConf {
    APB3Output,
    PLL2Q,
    HSIDiv,
    CSIRC,
    HSEOSC,
    PLL3Q,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI6MultConf {
    APB2Prescaler,
    PLL2Q,
    HSIDiv,
    CSIRC,
    HSEOSC,
    PLL3Q,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI2MultConf {
    PLL1Q,
    PLL2P,
    PLL3P,
    AUDIOCLK,
    CKPERMult,
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL2NConf {
    Value(u32),
}

impl PLL2NConf {
    pub const fn min() -> u32 {
        4
    }

    pub const fn max() -> u32 {
        512
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL2NConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL2N,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL2N,
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
pub enum PLL2PConf {
    Value(u32),
}

impl PLL2PConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        128
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL2PConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL2P,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL2P,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL2QConf {
    Value(u32),
}

impl PLL2QConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        128
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL2QConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL2Q,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL2Q,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL2RConf {
    Value(u32),
}

impl PLL2RConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        128
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL2RConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL2R,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL2R,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL3NConf {
    Value(u32),
}

impl PLL3NConf {
    pub const fn min() -> u32 {
        4
    }

    pub const fn max() -> u32 {
        512
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL3NConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL3N,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL3N,
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
pub enum PLL3PConf {
    Value(u32),
}

impl PLL3PConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        128
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL3PConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL3P,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL3P,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL3QConf {
    Value(u32),
}

impl PLL3QConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        128
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL3QConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL3Q,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL3Q,
                    });
                }
                Ok(*val as f32)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL3RConf {
    Value(u32),
}

impl PLL3RConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        128
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL3RConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL3R,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL3R,
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
    pub HSIDiv: HSIDivConf,
    pub HSEOSC: HSEOSCConf,
    pub LSIRC: LSIRCConf,
    pub LSEOSC: LSEOSCConf,
    pub SysClkSource: SysClkSourceConf,
    pub PLLSource: PLLSourceConf,
    pub PLL2Source: PLL2SourceConf,
    pub PLL3Source: PLL3SourceConf,
    pub PLLM: PLLMConf,
    pub PLL2M: PLL2MConf,
    pub PLL3M: PLL3MConf,
    pub HSERTCDevisor: HSERTCDevisorConf,
    pub RTCClkSource: RTCClkSourceConf,
    pub CECMult: CECMultConf,
    pub USART1Mult: USART1MultConf,
    pub USART2Mult: USART2MultConf,
    pub USART3Mult: USART3MultConf,
    pub UART4Mult: UART4MultConf,
    pub UART5Mult: UART5MultConf,
    pub USART6Mult: USART6MultConf,
    pub UART7Mult: UART7MultConf,
    pub UART9Mult: UART9MultConf,
    pub UART8Mult: UART8MultConf,
    pub USART10Mult: USART10MultConf,
    pub USART11Mult: USART11MultConf,
    pub UART12Mult: UART12MultConf,
    pub LPUART1Mult: LPUART1MultConf,
    pub LPTIM1Mult: LPTIM1MultConf,
    pub LPTIM2Mult: LPTIM2MultConf,
    pub DACMult: DACMultConf,
    pub ADCMult: ADCMultConf,
    pub CK48Mult: CK48MultConf,
    pub SDMMC1Mult: SDMMC1MultConf,
    pub SDMMC2Mult: SDMMC2MultConf,
    pub FDCANMult: FDCANMultConf,
    pub I2C1Mult: I2C1MultConf,
    pub I2C2Mult: I2C2MultConf,
    pub I2C3Mult: I2C3MultConf,
    pub SAI1Mult: SAI1MultConf,
    pub SAI2Mult: SAI2MultConf,
    pub I2C4Mult: I2C4MultConf,
    pub I3C1Mult: I3C1MultConf,
    pub OCTOSPIMMult: OCTOSPIMMultConf,
    pub LPTIM3Mult: LPTIM3MultConf,
    pub LPTIM4Mult: LPTIM4MultConf,
    pub LPTIM5Mult: LPTIM5MultConf,
    pub LPTIM6Mult: LPTIM6MultConf,
    pub RNGMult: RNGMultConf,
    pub MCOMult: MCOMultConf,
    pub MCODiv: MCODivConf,
    pub MCO2Mult: MCO2MultConf,
    pub MCO2Div: MCO2DivConf,
    pub LSCOMult: LSCOMultConf,
    pub CKPERMult: CKPERMultConf,
    pub AHBPrescaler: AHBPrescalerConf,
    pub CortexPrescaler: CortexPrescalerConf,
    pub CortexCLockSelection: CortexCLockSelectionConf,
    pub APB1Prescaler: APB1PrescalerConf,
    pub APB2Prescaler: APB2PrescalerConf,
    pub APB3Prescaler: APB3PrescalerConf,
    pub SPI1Mult: SPI1MultConf,
    pub SPI3Mult: SPI3MultConf,
    pub SPI4Mult: SPI4MultConf,
    pub SPI5Mult: SPI5MultConf,
    pub SPI6Mult: SPI6MultConf,
    pub SPI2Mult: SPI2MultConf,
    pub PLLN: PLLNConf,
    pub PLLFRACN: PLLFRACNConf,
    pub PLL1P: PLL1PConf,
    pub PLL1Q: PLL1QConf,
    pub PLL1R: PLL1RConf,
    pub PLL2N: PLL2NConf,
    pub PLL2FRACN: PLL2FRACNConf,
    pub PLL2P: PLL2PConf,
    pub PLL2Q: PLL2QConf,
    pub PLL2R: PLL2RConf,
    pub PLL3N: PLL3NConf,
    pub PLL3FRACN: PLL3FRACNConf,
    pub PLL3P: PLL3PConf,
    pub PLL3Q: PLL3QConf,
    pub PLL3R: PLL3RConf,
}

impl Default for ClockTree {
    fn default() -> Self {
        Self {
            HSIDiv: HSIDivConf::DIV2,
            HSEOSC: HSEOSCConf::Value(25000000),
            LSIRC: LSIRCConf::Value(32000),
            LSEOSC: LSEOSCConf::Value(32768),
            SysClkSource: SysClkSourceConf::HSIDiv,
            PLLSource: PLLSourceConf::CSIRC,
            PLL2Source: PLL2SourceConf::CSIRC,
            PLL3Source: PLL3SourceConf::CSIRC,
            PLLM: PLLMConf::Value(1),
            PLL2M: PLL2MConf::Value(1),
            PLL3M: PLL3MConf::Value(1),
            HSERTCDevisor: HSERTCDevisorConf::DIV1,
            RTCClkSource: RTCClkSourceConf::LSIRC,
            CECMult: CECMultConf::LSIRC,
            USART1Mult: USART1MultConf::APB2Prescaler,
            USART2Mult: USART2MultConf::APB1Prescaler,
            USART3Mult: USART3MultConf::APB1Prescaler,
            UART4Mult: UART4MultConf::APB1Prescaler,
            UART5Mult: UART5MultConf::APB1Prescaler,
            USART6Mult: USART6MultConf::APB1Prescaler,
            UART7Mult: UART7MultConf::APB1Prescaler,
            UART9Mult: UART9MultConf::APB1Prescaler,
            UART8Mult: UART8MultConf::APB1Prescaler,
            USART10Mult: USART10MultConf::APB1Prescaler,
            USART11Mult: USART11MultConf::APB1Prescaler,
            UART12Mult: UART12MultConf::APB1Prescaler,
            LPUART1Mult: LPUART1MultConf::APB3Output,
            LPTIM1Mult: LPTIM1MultConf::APB3Output,
            LPTIM2Mult: LPTIM2MultConf::APB1Prescaler,
            DACMult: DACMultConf::LSEOSC,
            ADCMult: ADCMultConf::AHBOutput,
            CK48Mult: CK48MultConf::HSI48RC,
            SDMMC1Mult: SDMMC1MultConf::PLL1Q,
            SDMMC2Mult: SDMMC2MultConf::PLL1Q,
            FDCANMult: FDCANMultConf::HSEOSC,
            I2C1Mult: I2C1MultConf::APB1Prescaler,
            I2C2Mult: I2C2MultConf::APB1Prescaler,
            I2C3Mult: I2C3MultConf::APB3Output,
            SAI1Mult: SAI1MultConf::PLL2P,
            SAI2Mult: SAI2MultConf::PLL2P,
            I2C4Mult: I2C4MultConf::APB3Output,
            I3C1Mult: I3C1MultConf::APB1Prescaler,
            OCTOSPIMMult: OCTOSPIMMultConf::AHBOutput,
            LPTIM3Mult: LPTIM3MultConf::APB3Output,
            LPTIM4Mult: LPTIM4MultConf::APB3Output,
            LPTIM5Mult: LPTIM5MultConf::APB3Output,
            LPTIM6Mult: LPTIM6MultConf::APB3Output,
            RNGMult: RNGMultConf::HSI48RC,
            MCOMult: MCOMultConf::HSIDiv,
            MCODiv: MCODivConf::DIV1,
            MCO2Mult: MCO2MultConf::SysCLKOutput,
            MCO2Div: MCO2DivConf::DIV1,
            LSCOMult: LSCOMultConf::LSIRC,
            CKPERMult: CKPERMultConf::HSIDiv,
            AHBPrescaler: AHBPrescalerConf::DIV1,
            CortexPrescaler: CortexPrescalerConf::DIV1,
            CortexCLockSelection: CortexCLockSelectionConf::CortexPrescaler,
            APB1Prescaler: APB1PrescalerConf::DIV1,
            APB2Prescaler: APB2PrescalerConf::DIV1,
            APB3Prescaler: APB3PrescalerConf::DIV1,
            SPI1Mult: SPI1MultConf::PLL1Q,
            SPI3Mult: SPI3MultConf::PLL1Q,
            SPI4Mult: SPI4MultConf::APB2Prescaler,
            SPI5Mult: SPI5MultConf::APB3Output,
            SPI6Mult: SPI6MultConf::APB2Prescaler,
            SPI2Mult: SPI2MultConf::PLL1Q,
            PLLN: PLLNConf::Value(129),
            PLLFRACN: PLLFRACNConf::Value(0),
            PLL1P: PLL1PConf::DIV2,
            PLL1Q: PLL1QConf::Value(2),
            PLL1R: PLL1RConf::Value(2),
            PLL2N: PLL2NConf::Value(129),
            PLL2FRACN: PLL2FRACNConf::Value(0),
            PLL2P: PLL2PConf::Value(2),
            PLL2Q: PLL2QConf::Value(2),
            PLL2R: PLL2RConf::Value(2),
            PLL3N: PLL3NConf::Value(129),
            PLL3FRACN: PLL3FRACNConf::Value(0),
            PLL3P: PLL3PConf::Value(2),
            PLL3Q: PLL3QConf::Value(2),
            PLL3R: PLL3RConf::Value(2),
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

    pub fn CRSCLKoutput_get(&self) -> Result<f32, ClockError> {
        self.HSI48RC_get()
    }
    pub fn HSI48RC_get(&self) -> Result<f32, ClockError> {
        Ok(48000000 as f32)
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
    pub fn AUDIOCLK_get(&self) -> Result<f32, ClockError> {
        Ok(12288000 as f32)
    }
    fn SysClkSource_get(&self) -> Result<f32, ClockError> {
        match self.SysClkSource {
            SysClkSourceConf::HSIDiv => return self.HSIDiv_get(),
            SysClkSourceConf::CSIRC => return self.CSIRC_get(),
            SysClkSourceConf::HSEOSC => return self.HSEOSC_get(),
            SysClkSourceConf::PLL1P => return self.PLL1P_get(),
        };
    }
    pub fn SysCLKOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SysClkSource_get()?;
        if input > (250000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 250000000),
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
            PLLSourceConf::CSIRC => return self.CSIRC_get(),
            PLLSourceConf::HSIDiv => return self.HSIDiv_get(),
            PLLSourceConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    fn PLL2Source_get(&self) -> Result<f32, ClockError> {
        match self.PLL2Source {
            PLL2SourceConf::CSIRC => return self.CSIRC_get(),
            PLL2SourceConf::HSIDiv => return self.HSIDiv_get(),
            PLL2SourceConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    fn PLL3Source_get(&self) -> Result<f32, ClockError> {
        match self.PLL3Source {
            PLL3SourceConf::CSIRC => return self.CSIRC_get(),
            PLL3SourceConf::HSIDiv => return self.HSIDiv_get(),
            PLL3SourceConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    fn PLLM_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLSource_get()? as f32;
        let value = self.PLLM.get()? as f32;
        Ok((input / value) as f32)
    }

    fn PLL2M_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL2Source_get()? as f32;
        let value = self.PLL2M.get()? as f32;
        Ok((input / value) as f32)
    }

    fn PLL3M_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL3Source_get()? as f32;
        let value = self.PLL3M.get()? as f32;
        Ok((input / value) as f32)
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
        if input > (50000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 50000000),
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
    fn CSIdivTohdmi_get(&self) -> Result<f32, ClockError> {
        let input = self.CSIRC_get()? as f32;
        let value = 122 as f32;
        Ok((input / value) as f32)
    }

    fn CECMult_get(&self) -> Result<f32, ClockError> {
        match self.CECMult {
            CECMultConf::LSEOSC => return self.LSEOSC_get(),
            CECMultConf::CSIdivTohdmi => return self.CSIdivTohdmi_get(),
            CECMultConf::LSIRC => return self.LSIRC_get(),
        };
    }
    pub fn CECoutput_get(&self) -> Result<f32, ClockError> {
        self.CECMult_get()
    }
    fn USART1Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART1Mult {
            USART1MultConf::APB2Prescaler => return self.APB2Prescaler_get(),
            USART1MultConf::PLL2Q => return self.PLL2Q_get(),
            USART1MultConf::HSIDiv => return self.HSIDiv_get(),
            USART1MultConf::LSEOSC => return self.LSEOSC_get(),
            USART1MultConf::CSIRC => return self.CSIRC_get(),
            USART1MultConf::PLL3Q => return self.PLL3Q_get(),
        };
    }
    pub fn USART1output_get(&self) -> Result<f32, ClockError> {
        self.USART1Mult_get()
    }
    fn USART2Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART2Mult {
            USART2MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            USART2MultConf::PLL2Q => return self.PLL2Q_get(),
            USART2MultConf::HSIDiv => return self.HSIDiv_get(),
            USART2MultConf::LSEOSC => return self.LSEOSC_get(),
            USART2MultConf::CSIRC => return self.CSIRC_get(),
            USART2MultConf::PLL3Q => return self.PLL3Q_get(),
        };
    }
    pub fn USART2output_get(&self) -> Result<f32, ClockError> {
        self.USART2Mult_get()
    }
    fn USART3Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART3Mult {
            USART3MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            USART3MultConf::PLL2Q => return self.PLL2Q_get(),
            USART3MultConf::HSIDiv => return self.HSIDiv_get(),
            USART3MultConf::LSEOSC => return self.LSEOSC_get(),
            USART3MultConf::CSIRC => return self.CSIRC_get(),
            USART3MultConf::PLL3Q => return self.PLL3Q_get(),
        };
    }
    pub fn USART3output_get(&self) -> Result<f32, ClockError> {
        self.USART3Mult_get()
    }
    fn UART4Mult_get(&self) -> Result<f32, ClockError> {
        match self.UART4Mult {
            UART4MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            UART4MultConf::PLL2Q => return self.PLL2Q_get(),
            UART4MultConf::HSIDiv => return self.HSIDiv_get(),
            UART4MultConf::LSEOSC => return self.LSEOSC_get(),
            UART4MultConf::CSIRC => return self.CSIRC_get(),
            UART4MultConf::PLL3Q => return self.PLL3Q_get(),
        };
    }
    pub fn UART4output_get(&self) -> Result<f32, ClockError> {
        self.UART4Mult_get()
    }
    fn UART5Mult_get(&self) -> Result<f32, ClockError> {
        match self.UART5Mult {
            UART5MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            UART5MultConf::PLL2Q => return self.PLL2Q_get(),
            UART5MultConf::HSIDiv => return self.HSIDiv_get(),
            UART5MultConf::LSEOSC => return self.LSEOSC_get(),
            UART5MultConf::CSIRC => return self.CSIRC_get(),
            UART5MultConf::PLL3Q => return self.PLL3Q_get(),
        };
    }
    pub fn UART5output_get(&self) -> Result<f32, ClockError> {
        self.UART5Mult_get()
    }
    fn USART6Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART6Mult {
            USART6MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            USART6MultConf::PLL2Q => return self.PLL2Q_get(),
            USART6MultConf::HSIDiv => return self.HSIDiv_get(),
            USART6MultConf::LSEOSC => return self.LSEOSC_get(),
            USART6MultConf::CSIRC => return self.CSIRC_get(),
            USART6MultConf::PLL3Q => return self.PLL3Q_get(),
        };
    }
    pub fn USART6output_get(&self) -> Result<f32, ClockError> {
        self.USART6Mult_get()
    }
    fn UART7Mult_get(&self) -> Result<f32, ClockError> {
        match self.UART7Mult {
            UART7MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            UART7MultConf::PLL2Q => return self.PLL2Q_get(),
            UART7MultConf::HSIDiv => return self.HSIDiv_get(),
            UART7MultConf::LSEOSC => return self.LSEOSC_get(),
            UART7MultConf::CSIRC => return self.CSIRC_get(),
            UART7MultConf::PLL3Q => return self.PLL3Q_get(),
        };
    }
    pub fn UART7output_get(&self) -> Result<f32, ClockError> {
        self.UART7Mult_get()
    }
    fn UART9Mult_get(&self) -> Result<f32, ClockError> {
        match self.UART9Mult {
            UART9MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            UART9MultConf::PLL2Q => return self.PLL2Q_get(),
            UART9MultConf::HSIDiv => return self.HSIDiv_get(),
            UART9MultConf::LSEOSC => return self.LSEOSC_get(),
            UART9MultConf::CSIRC => return self.CSIRC_get(),
            UART9MultConf::PLL3Q => return self.PLL3Q_get(),
        };
    }
    pub fn UART9output_get(&self) -> Result<f32, ClockError> {
        self.UART9Mult_get()
    }
    fn UART8Mult_get(&self) -> Result<f32, ClockError> {
        match self.UART8Mult {
            UART8MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            UART8MultConf::PLL2Q => return self.PLL2Q_get(),
            UART8MultConf::HSIDiv => return self.HSIDiv_get(),
            UART8MultConf::LSEOSC => return self.LSEOSC_get(),
            UART8MultConf::CSIRC => return self.CSIRC_get(),
            UART8MultConf::PLL3Q => return self.PLL3Q_get(),
        };
    }
    pub fn UART8output_get(&self) -> Result<f32, ClockError> {
        self.UART8Mult_get()
    }
    fn USART10Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART10Mult {
            USART10MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            USART10MultConf::PLL2Q => return self.PLL2Q_get(),
            USART10MultConf::HSIDiv => return self.HSIDiv_get(),
            USART10MultConf::LSEOSC => return self.LSEOSC_get(),
            USART10MultConf::CSIRC => return self.CSIRC_get(),
            USART10MultConf::PLL3Q => return self.PLL3Q_get(),
        };
    }
    pub fn USART10output_get(&self) -> Result<f32, ClockError> {
        self.USART10Mult_get()
    }
    fn USART11Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART11Mult {
            USART11MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            USART11MultConf::PLL2Q => return self.PLL2Q_get(),
            USART11MultConf::HSIDiv => return self.HSIDiv_get(),
            USART11MultConf::LSEOSC => return self.LSEOSC_get(),
            USART11MultConf::CSIRC => return self.CSIRC_get(),
            USART11MultConf::PLL3Q => return self.PLL3Q_get(),
        };
    }
    pub fn USART11output_get(&self) -> Result<f32, ClockError> {
        self.USART11Mult_get()
    }
    fn UART12Mult_get(&self) -> Result<f32, ClockError> {
        match self.UART12Mult {
            UART12MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            UART12MultConf::PLL2Q => return self.PLL2Q_get(),
            UART12MultConf::HSIDiv => return self.HSIDiv_get(),
            UART12MultConf::LSEOSC => return self.LSEOSC_get(),
            UART12MultConf::CSIRC => return self.CSIRC_get(),
            UART12MultConf::PLL3Q => return self.PLL3Q_get(),
        };
    }
    pub fn UART12output_get(&self) -> Result<f32, ClockError> {
        self.UART12Mult_get()
    }
    fn LPUART1Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPUART1Mult {
            LPUART1MultConf::APB3Output => return self.APB3Output_get(),
            LPUART1MultConf::PLL2Q => return self.PLL2Q_get(),
            LPUART1MultConf::HSIDiv => return self.HSIDiv_get(),
            LPUART1MultConf::LSEOSC => return self.LSEOSC_get(),
            LPUART1MultConf::CSIRC => return self.CSIRC_get(),
            LPUART1MultConf::PLL3Q => return self.PLL3Q_get(),
        };
    }
    pub fn LPUART1output_get(&self) -> Result<f32, ClockError> {
        self.LPUART1Mult_get()
    }
    fn LPTIM1Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM1Mult {
            LPTIM1MultConf::APB3Output => return self.APB3Output_get(),
            LPTIM1MultConf::PLL2P => return self.PLL2P_get(),
            LPTIM1MultConf::LSEOSC => return self.LSEOSC_get(),
            LPTIM1MultConf::LSIRC => return self.LSIRC_get(),
            LPTIM1MultConf::CKPERMult => return self.CKPERMult_get(),
            LPTIM1MultConf::PLL3R => return self.PLL3R_get(),
        };
    }
    pub fn LPTIM1output_get(&self) -> Result<f32, ClockError> {
        self.LPTIM1Mult_get()
    }
    fn LPTIM2Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM2Mult {
            LPTIM2MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            LPTIM2MultConf::PLL2P => return self.PLL2P_get(),
            LPTIM2MultConf::LSEOSC => return self.LSEOSC_get(),
            LPTIM2MultConf::LSIRC => return self.LSIRC_get(),
            LPTIM2MultConf::CKPERMult => return self.CKPERMult_get(),
            LPTIM2MultConf::PLL3R => return self.PLL3R_get(),
        };
    }
    pub fn LPTIM2output_get(&self) -> Result<f32, ClockError> {
        self.LPTIM2Mult_get()
    }
    fn DACMult_get(&self) -> Result<f32, ClockError> {
        match self.DACMult {
            DACMultConf::LSEOSC => return self.LSEOSC_get(),
            DACMultConf::LSIRC => return self.LSIRC_get(),
        };
    }
    pub fn DACoutput_get(&self) -> Result<f32, ClockError> {
        self.DACMult_get()
    }
    fn ADCMult_get(&self) -> Result<f32, ClockError> {
        match self.ADCMult {
            ADCMultConf::AHBOutput => return self.AHBOutput_get(),
            ADCMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            ADCMultConf::PLL2R => return self.PLL2R_get(),
            ADCMultConf::HSEOSC => return self.HSEOSC_get(),
            ADCMultConf::HSIDiv => return self.HSIDiv_get(),
            ADCMultConf::CSIRC => return self.CSIRC_get(),
        };
    }
    pub fn ADCoutput_get(&self) -> Result<f32, ClockError> {
        self.ADCMult_get()
    }
    fn CK48Mult_get(&self) -> Result<f32, ClockError> {
        match self.CK48Mult {
            CK48MultConf::PLL3Q => return self.PLL3Q_get(),
            CK48MultConf::PLL1Q => return self.PLL1Q_get(),
            CK48MultConf::HSI48RC => return self.HSI48RC_get(),
        };
    }
    pub fn USBoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.CK48Mult_get()?;
        if input > (50000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 50000000),
                from: ClockNodes::CK48Mult,
                to: ClockNodes::USBoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::CK48Mult,
                to: ClockNodes::USBoutput,
            });
        }
        Ok(input)
    }
    fn SDMMC1Mult_get(&self) -> Result<f32, ClockError> {
        match self.SDMMC1Mult {
            SDMMC1MultConf::PLL1Q => return self.PLL1Q_get(),
            SDMMC1MultConf::PLL2R => return self.PLL2R_get(),
        };
    }
    pub fn SDMMC1Output_get(&self) -> Result<f32, ClockError> {
        self.SDMMC1Mult_get()
    }
    fn SDMMC2Mult_get(&self) -> Result<f32, ClockError> {
        match self.SDMMC2Mult {
            SDMMC2MultConf::PLL1Q => return self.PLL1Q_get(),
            SDMMC2MultConf::PLL2R => return self.PLL2R_get(),
        };
    }
    pub fn SDMMC2Output_get(&self) -> Result<f32, ClockError> {
        self.SDMMC2Mult_get()
    }
    fn FDCANMult_get(&self) -> Result<f32, ClockError> {
        match self.FDCANMult {
            FDCANMultConf::PLL1Q => return self.PLL1Q_get(),
            FDCANMultConf::PLL2Q => return self.PLL2Q_get(),
            FDCANMultConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    pub fn FDCANOutput_get(&self) -> Result<f32, ClockError> {
        self.FDCANMult_get()
    }
    fn I2C1Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C1Mult {
            I2C1MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            I2C1MultConf::PLL3R => return self.PLL3R_get(),
            I2C1MultConf::HSIDiv => return self.HSIDiv_get(),
            I2C1MultConf::CSIRC => return self.CSIRC_get(),
        };
    }
    pub fn I2C1output_get(&self) -> Result<f32, ClockError> {
        self.I2C1Mult_get()
    }
    fn I2C2Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C2Mult {
            I2C2MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            I2C2MultConf::PLL3R => return self.PLL3R_get(),
            I2C2MultConf::HSIDiv => return self.HSIDiv_get(),
            I2C2MultConf::CSIRC => return self.CSIRC_get(),
        };
    }
    pub fn I2C2output_get(&self) -> Result<f32, ClockError> {
        self.I2C2Mult_get()
    }
    fn I2C3Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C3Mult {
            I2C3MultConf::APB3Output => return self.APB3Output_get(),
            I2C3MultConf::PLL3R => return self.PLL3R_get(),
            I2C3MultConf::HSIDiv => return self.HSIDiv_get(),
            I2C3MultConf::CSIRC => return self.CSIRC_get(),
        };
    }
    pub fn I2C3output_get(&self) -> Result<f32, ClockError> {
        self.I2C3Mult_get()
    }
    fn SAI1Mult_get(&self) -> Result<f32, ClockError> {
        match self.SAI1Mult {
            SAI1MultConf::PLL2P => return self.PLL2P_get(),
            SAI1MultConf::PLL3P => return self.PLL3P_get(),
            SAI1MultConf::PLL1Q => return self.PLL1Q_get(),
            SAI1MultConf::AUDIOCLK => return self.AUDIOCLK_get(),
            SAI1MultConf::CKPERMult => return self.CKPERMult_get(),
        };
    }
    pub fn SAI1output_get(&self) -> Result<f32, ClockError> {
        self.SAI1Mult_get()
    }
    fn SAI2Mult_get(&self) -> Result<f32, ClockError> {
        match self.SAI2Mult {
            SAI2MultConf::PLL2P => return self.PLL2P_get(),
            SAI2MultConf::PLL3P => return self.PLL3P_get(),
            SAI2MultConf::PLL1Q => return self.PLL1Q_get(),
            SAI2MultConf::AUDIOCLK => return self.AUDIOCLK_get(),
            SAI2MultConf::CKPERMult => return self.CKPERMult_get(),
        };
    }
    pub fn SAI2output_get(&self) -> Result<f32, ClockError> {
        self.SAI2Mult_get()
    }
    fn I2C4Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C4Mult {
            I2C4MultConf::APB3Output => return self.APB3Output_get(),
            I2C4MultConf::PLL3R => return self.PLL3R_get(),
            I2C4MultConf::HSIDiv => return self.HSIDiv_get(),
            I2C4MultConf::CSIRC => return self.CSIRC_get(),
        };
    }
    pub fn I2C4output_get(&self) -> Result<f32, ClockError> {
        self.I2C4Mult_get()
    }
    fn I3C1Mult_get(&self) -> Result<f32, ClockError> {
        match self.I3C1Mult {
            I3C1MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            I3C1MultConf::PLL3R => return self.PLL3R_get(),
            I3C1MultConf::HSIDiv => return self.HSIDiv_get(),
        };
    }
    pub fn I3C1output_get(&self) -> Result<f32, ClockError> {
        self.I3C1Mult_get()
    }
    fn OCTOSPIMMult_get(&self) -> Result<f32, ClockError> {
        match self.OCTOSPIMMult {
            OCTOSPIMMultConf::AHBOutput => return self.AHBOutput_get(),
            OCTOSPIMMultConf::PLL1Q => return self.PLL1Q_get(),
            OCTOSPIMMultConf::PLL2R => return self.PLL2R_get(),
            OCTOSPIMMultConf::CKPERMult => return self.CKPERMult_get(),
        };
    }
    pub fn OCTOSPIMoutput_get(&self) -> Result<f32, ClockError> {
        self.OCTOSPIMMult_get()
    }
    fn LPTIM3Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM3Mult {
            LPTIM3MultConf::APB3Output => return self.APB3Output_get(),
            LPTIM3MultConf::PLL2P => return self.PLL2P_get(),
            LPTIM3MultConf::LSEOSC => return self.LSEOSC_get(),
            LPTIM3MultConf::LSIRC => return self.LSIRC_get(),
            LPTIM3MultConf::CKPERMult => return self.CKPERMult_get(),
            LPTIM3MultConf::PLL3R => return self.PLL3R_get(),
        };
    }
    pub fn LPTIM3output_get(&self) -> Result<f32, ClockError> {
        self.LPTIM3Mult_get()
    }
    fn LPTIM4Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM4Mult {
            LPTIM4MultConf::APB3Output => return self.APB3Output_get(),
            LPTIM4MultConf::PLL2P => return self.PLL2P_get(),
            LPTIM4MultConf::LSEOSC => return self.LSEOSC_get(),
            LPTIM4MultConf::LSIRC => return self.LSIRC_get(),
            LPTIM4MultConf::CKPERMult => return self.CKPERMult_get(),
            LPTIM4MultConf::PLL3R => return self.PLL3R_get(),
        };
    }
    pub fn LPTIM4output_get(&self) -> Result<f32, ClockError> {
        self.LPTIM4Mult_get()
    }
    fn LPTIM5Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM5Mult {
            LPTIM5MultConf::APB3Output => return self.APB3Output_get(),
            LPTIM5MultConf::PLL2P => return self.PLL2P_get(),
            LPTIM5MultConf::LSEOSC => return self.LSEOSC_get(),
            LPTIM5MultConf::LSIRC => return self.LSIRC_get(),
            LPTIM5MultConf::CKPERMult => return self.CKPERMult_get(),
            LPTIM5MultConf::PLL3R => return self.PLL3R_get(),
        };
    }
    pub fn LPTIM5output_get(&self) -> Result<f32, ClockError> {
        self.LPTIM5Mult_get()
    }
    fn LPTIM6Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM6Mult {
            LPTIM6MultConf::APB3Output => return self.APB3Output_get(),
            LPTIM6MultConf::PLL2P => return self.PLL2P_get(),
            LPTIM6MultConf::LSEOSC => return self.LSEOSC_get(),
            LPTIM6MultConf::LSIRC => return self.LSIRC_get(),
            LPTIM6MultConf::CKPERMult => return self.CKPERMult_get(),
            LPTIM6MultConf::PLL3R => return self.PLL3R_get(),
        };
    }
    pub fn LPTIM6output_get(&self) -> Result<f32, ClockError> {
        self.LPTIM6Mult_get()
    }
    fn RNGMult_get(&self) -> Result<f32, ClockError> {
        match self.RNGMult {
            RNGMultConf::HSI48RC => return self.HSI48RC_get(),
            RNGMultConf::PLL1Q => return self.PLL1Q_get(),
            RNGMultConf::LSEOSC => return self.LSEOSC_get(),
            RNGMultConf::LSIRC => return self.LSIRC_get(),
        };
    }
    pub fn RNGoutput_get(&self) -> Result<f32, ClockError> {
        self.RNGMult_get()
    }
    fn MCOMult_get(&self) -> Result<f32, ClockError> {
        match self.MCOMult {
            MCOMultConf::LSEOSC => return self.LSEOSC_get(),
            MCOMultConf::HSEOSC => return self.HSEOSC_get(),
            MCOMultConf::HSIDiv => return self.HSIDiv_get(),
            MCOMultConf::PLL1Q => return self.PLL1Q_get(),
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
    fn MCO2Mult_get(&self) -> Result<f32, ClockError> {
        match self.MCO2Mult {
            MCO2MultConf::LSIRC => return self.LSIRC_get(),
            MCO2MultConf::HSEOSC => return self.HSEOSC_get(),
            MCO2MultConf::CSIRC => return self.CSIRC_get(),
            MCO2MultConf::PLL1P => return self.PLL1P_get(),
            MCO2MultConf::PLL2P => return self.PLL2P_get(),
            MCO2MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
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
    fn LSCOMult_get(&self) -> Result<f32, ClockError> {
        match self.LSCOMult {
            LSCOMultConf::LSIRC => return self.LSIRC_get(),
            LSCOMultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn LSCOOutput_get(&self) -> Result<f32, ClockError> {
        self.LSCOMult_get()
    }
    fn CKPERMult_get(&self) -> Result<f32, ClockError> {
        match self.CKPERMult {
            CKPERMultConf::HSIDiv => return self.HSIDiv_get(),
            CKPERMultConf::HSEOSC => return self.HSEOSC_get(),
            CKPERMultConf::CSIRC => return self.CSIRC_get(),
        };
    }
    pub fn CKPERoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.CKPERMult_get()?;
        if input > (64000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 64000000),
                from: ClockNodes::CKPERMult,
                to: ClockNodes::CKPERoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::CKPERMult,
                to: ClockNodes::CKPERoutput,
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
        if input > (250000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 250000000),
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
        if input > (250000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 250000000),
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
        if input > (250000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 250000000),
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
    fn CortexPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.CortexPrescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    fn CortexCLockSelection_get(&self) -> Result<f32, ClockError> {
        match self.CortexCLockSelection {
            CortexCLockSelectionConf::CortexPrescaler => return self.CortexPrescaler_get(),
            CortexCLockSelectionConf::LSEOSC => return self.LSEOSC_get(),
            CortexCLockSelectionConf::LSIRC => return self.LSIRC_get(),
        };
    }
    pub fn CortexSysOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.CortexCLockSelection_get()?;
        if input > (250000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 250000000),
                from: ClockNodes::CortexCLockSelection,
                to: ClockNodes::CortexSysOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::CortexCLockSelection,
                to: ClockNodes::CortexSysOutput,
            });
        }
        Ok(input)
    }
    pub fn FCLKCortexOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()?;
        if input > (250000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 250000000),
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
        if input > (250000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 250000000),
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
        if input > (250000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 250000000),
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
        if input > (250000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 250000000),
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
    fn APB3Prescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.APB3Prescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB3Output_get(&self) -> Result<f32, ClockError> {
        let input = self.APB3Prescaler_get()?;
        if input > (250000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 250000000),
                from: ClockNodes::APB3Prescaler,
                to: ClockNodes::APB3Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::APB3Prescaler,
                to: ClockNodes::APB3Output,
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
        if input > (250000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 250000000),
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
    fn hsidivToUCPD_get(&self) -> Result<f32, ClockError> {
        let input = self.HSIDiv_get()? as f32;
        let value = 4 as f32;
        Ok((input / value) as f32)
    }

    pub fn UCPD1Output_get(&self) -> Result<f32, ClockError> {
        self.hsidivToUCPD_get()
    }
    fn SPI1Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI1Mult {
            SPI1MultConf::PLL1Q => return self.PLL1Q_get(),
            SPI1MultConf::PLL2P => return self.PLL2P_get(),
            SPI1MultConf::PLL3P => return self.PLL3P_get(),
            SPI1MultConf::AUDIOCLK => return self.AUDIOCLK_get(),
            SPI1MultConf::CKPERMult => return self.CKPERMult_get(),
        };
    }
    pub fn SPI1output_get(&self) -> Result<f32, ClockError> {
        self.SPI1Mult_get()
    }
    fn SPI3Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI3Mult {
            SPI3MultConf::PLL1Q => return self.PLL1Q_get(),
            SPI3MultConf::PLL2P => return self.PLL2P_get(),
            SPI3MultConf::PLL3P => return self.PLL3P_get(),
            SPI3MultConf::AUDIOCLK => return self.AUDIOCLK_get(),
            SPI3MultConf::CKPERMult => return self.CKPERMult_get(),
        };
    }
    pub fn SPI3output_get(&self) -> Result<f32, ClockError> {
        self.SPI3Mult_get()
    }
    fn SPI4Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI4Mult {
            SPI4MultConf::APB2Prescaler => return self.APB2Prescaler_get(),
            SPI4MultConf::PLL2Q => return self.PLL2Q_get(),
            SPI4MultConf::HSIDiv => return self.HSIDiv_get(),
            SPI4MultConf::CSIRC => return self.CSIRC_get(),
            SPI4MultConf::HSEOSC => return self.HSEOSC_get(),
            SPI4MultConf::PLL3Q => return self.PLL3Q_get(),
        };
    }
    pub fn SPI4output_get(&self) -> Result<f32, ClockError> {
        self.SPI4Mult_get()
    }
    fn SPI5Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI5Mult {
            SPI5MultConf::APB3Output => return self.APB3Output_get(),
            SPI5MultConf::PLL2Q => return self.PLL2Q_get(),
            SPI5MultConf::HSIDiv => return self.HSIDiv_get(),
            SPI5MultConf::CSIRC => return self.CSIRC_get(),
            SPI5MultConf::HSEOSC => return self.HSEOSC_get(),
            SPI5MultConf::PLL3Q => return self.PLL3Q_get(),
        };
    }
    pub fn SPI5output_get(&self) -> Result<f32, ClockError> {
        self.SPI5Mult_get()
    }
    fn SPI6Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI6Mult {
            SPI6MultConf::APB2Prescaler => return self.APB2Prescaler_get(),
            SPI6MultConf::PLL2Q => return self.PLL2Q_get(),
            SPI6MultConf::HSIDiv => return self.HSIDiv_get(),
            SPI6MultConf::CSIRC => return self.CSIRC_get(),
            SPI6MultConf::HSEOSC => return self.HSEOSC_get(),
            SPI6MultConf::PLL3Q => return self.PLL3Q_get(),
        };
    }
    pub fn SPI6output_get(&self) -> Result<f32, ClockError> {
        self.SPI6Mult_get()
    }
    fn SPI2Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI2Mult {
            SPI2MultConf::PLL1Q => return self.PLL1Q_get(),
            SPI2MultConf::PLL2P => return self.PLL2P_get(),
            SPI2MultConf::PLL3P => return self.PLL3P_get(),
            SPI2MultConf::AUDIOCLK => return self.AUDIOCLK_get(),
            SPI2MultConf::CKPERMult => return self.CKPERMult_get(),
        };
    }
    pub fn SPI2output_get(&self) -> Result<f32, ClockError> {
        self.SPI2Mult_get()
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

    fn PLL2N_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL2M_get()? as f32;
        let frac = self.PLL2FRACN_get()? as f32;
        let frac_max = PLL2FRACNConf::max() as f32;
        let value = self.PLL2N.get()? as f32;
        let ret = (input * (value + (frac / frac_max)));
        Ok(ret as f32)
    }

    pub fn PLL2FRACN_get(&self) -> Result<f32, ClockError> {
        self.PLL2FRACN.get()
    }
    fn PLL2P_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL2N_get()? as f32;
        let value = self.PLL2P.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn PLL2Poutput_get(&self) -> Result<f32, ClockError> {
        self.PLL2P_get()
    }
    fn PLL2Q_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL2N_get()? as f32;
        let value = self.PLL2Q.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn PLL2Qoutput_get(&self) -> Result<f32, ClockError> {
        self.PLL2Q_get()
    }
    fn PLL2R_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL2N_get()? as f32;
        let value = self.PLL2R.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn PLL2Routput_get(&self) -> Result<f32, ClockError> {
        self.PLL2R_get()
    }
    fn PLL3N_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL3M_get()? as f32;
        let frac = self.PLL3FRACN_get()? as f32;
        let frac_max = PLL3FRACNConf::max() as f32;
        let value = self.PLL3N.get()? as f32;
        let ret = (input * (value + (frac / frac_max)));
        Ok(ret as f32)
    }

    pub fn PLL3FRACN_get(&self) -> Result<f32, ClockError> {
        self.PLL3FRACN.get()
    }
    fn PLL3P_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL3N_get()? as f32;
        let value = self.PLL3P.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn PLL3Poutput_get(&self) -> Result<f32, ClockError> {
        self.PLL3P_get()
    }
    fn PLL3Q_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL3N_get()? as f32;
        let value = self.PLL3Q.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn PLL3Qoutput_get(&self) -> Result<f32, ClockError> {
        self.PLL3Q_get()
    }
    fn PLL3R_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL3N_get()? as f32;
        let value = self.PLL3R.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn PLL3Routput_get(&self) -> Result<f32, ClockError> {
        self.PLL3R_get()
    }
}
