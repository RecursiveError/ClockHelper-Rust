#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ClockNodes {
    None,
    HSIRC,
    CRSCLKoutput,
    HSI48RC,
    HSEOSC,
    LSIRC,
    LSIDIV,
    LSEOSC,
    MSIRC,
    MSIKRC,
    SAI1_EXT,
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
    DACMult,
    DACoutput,
    ADCMult,
    ADCoutput,
    CK48Mult,
    CK48output,
    USBoutput,
    SDMMC1Mult,
    SDMMCC1Output,
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
    I2C4Mult,
    I2C4output,
    MDF1Mult,
    MDF1output,
    ADF1Mult,
    ADF1output,
    OCTOSPIMMult,
    OCTOSPIMoutput,
    LPTIM3Mult,
    LPTIM3output,
    HSI48DivToRNG,
    RNGMult,
    RNGoutput,
    MCOMult,
    MCODiv,
    MCOPin,
    LSCOMult,
    LSCOOutput,
    AHBPrescaler,
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
    UCPD1Output,
    SPI1Mult,
    SPI1output,
    SPI3Mult,
    SPI3output,
    SPI2Mult,
    SPI2output,
    PLLN,
    PLLFRACN,
    PLL1P,
    PLLPoutput,
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
        5000
    }

    pub const fn max() -> u32 {
        40000
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
    CLOCK_48000,
    CLOCK_24000,
    CLOCK_16000,
    CLOCK_12000,
    CLOCK_4000,
    CLOCK_2000,
    CLOCK_1330,
    CLOCK_1000,
    CLOCK_3072,
    CLOCK_1536,
    CLOCK_1024,
    CLOCK_768,
    CLOCK_400,
    CLOCK_200,
    CLOCK_133,
    CLOCK_100,
}

impl MSIRCConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MSIRCConf::CLOCK_48000 => return Ok(48000.0),
            MSIRCConf::CLOCK_24000 => return Ok(24000.0),
            MSIRCConf::CLOCK_16000 => return Ok(16000.0),
            MSIRCConf::CLOCK_12000 => return Ok(12000.0),
            MSIRCConf::CLOCK_4000 => return Ok(4000.0),
            MSIRCConf::CLOCK_2000 => return Ok(2000.0),
            MSIRCConf::CLOCK_1330 => return Ok(1330.0),
            MSIRCConf::CLOCK_1000 => return Ok(1000.0),
            MSIRCConf::CLOCK_3072 => return Ok(3072.0),
            MSIRCConf::CLOCK_1536 => return Ok(1536.0),
            MSIRCConf::CLOCK_1024 => return Ok(1024.0),
            MSIRCConf::CLOCK_768 => return Ok(768.0),
            MSIRCConf::CLOCK_400 => return Ok(400.0),
            MSIRCConf::CLOCK_200 => return Ok(200.0),
            MSIRCConf::CLOCK_133 => return Ok(133.0),
            MSIRCConf::CLOCK_100 => return Ok(100.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MSIKRCConf {
    CLOCK_48000,
    CLOCK_24000,
    CLOCK_16000,
    CLOCK_12000,
    CLOCK_4000,
    CLOCK_2000,
    CLOCK_1330,
    CLOCK_1000,
    CLOCK_3072,
    CLOCK_1536,
    CLOCK_1024,
    CLOCK_768,
    CLOCK_400,
    CLOCK_200,
    CLOCK_133,
    CLOCK_100,
}

impl MSIKRCConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MSIKRCConf::CLOCK_48000 => return Ok(48000.0),
            MSIKRCConf::CLOCK_24000 => return Ok(24000.0),
            MSIKRCConf::CLOCK_16000 => return Ok(16000.0),
            MSIKRCConf::CLOCK_12000 => return Ok(12000.0),
            MSIKRCConf::CLOCK_4000 => return Ok(4000.0),
            MSIKRCConf::CLOCK_2000 => return Ok(2000.0),
            MSIKRCConf::CLOCK_1330 => return Ok(1330.0),
            MSIKRCConf::CLOCK_1000 => return Ok(1000.0),
            MSIKRCConf::CLOCK_3072 => return Ok(3072.0),
            MSIKRCConf::CLOCK_1536 => return Ok(1536.0),
            MSIKRCConf::CLOCK_1024 => return Ok(1024.0),
            MSIKRCConf::CLOCK_768 => return Ok(768.0),
            MSIKRCConf::CLOCK_400 => return Ok(400.0),
            MSIKRCConf::CLOCK_200 => return Ok(200.0),
            MSIKRCConf::CLOCK_133 => return Ok(133.0),
            MSIKRCConf::CLOCK_100 => return Ok(100.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SysClkSourceConf {
    MSIRC,
    HSIRC,
    HSEOSC,
    PLL1R,
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
pub enum PLL2SourceConf {
    MSIRC,
    HSIRC,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL3SourceConf {
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
        16
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
        16
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
        16
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
    APB3Output,
    SysCLKOutput,
    HSIRC,
    LSEOSC,
    MSIKRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM1MultConf {
    MSIKRC,
    LSIDIV,
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
pub enum DACMultConf {
    LSEOSC,
    LSIDIV,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ADCMultConf {
    AHBOutput,
    SysCLKOutput,
    PLL2R,
    HSEOSC,
    HSIRC,
    MSIKRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CK48MultConf {
    PLL2Q,
    PLL1Q,
    MSIKRC,
    HSI48RC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SDMMC1MultConf {
    PLL1P,
    CK48Mult,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FDCANMultConf {
    PLL1Q,
    PLL2P,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C1MultConf {
    APB1Prescaler,
    SysCLKOutput,
    HSIRC,
    MSIKRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C2MultConf {
    APB1Prescaler,
    SysCLKOutput,
    HSIRC,
    MSIKRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C3MultConf {
    APB3Output,
    SysCLKOutput,
    HSIRC,
    MSIKRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SAI1MultConf {
    PLL2P,
    PLL3P,
    PLL1P,
    SAI1_EXT,
    HSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C4MultConf {
    APB1Prescaler,
    SysCLKOutput,
    HSIRC,
    MSIKRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MDF1MultConf {
    AHBOutput,
    PLL1P,
    PLL3Q,
    SAI1_EXT,
    MSIKRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ADF1MultConf {
    AHBOutput,
    PLL1P,
    PLL3Q,
    SAI1_EXT,
    MSIKRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum OCTOSPIMMultConf {
    MSIKRC,
    SysCLKOutput,
    PLL1Q,
    PLL2Q,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM3MultConf {
    MSIKRC,
    LSIDIV,
    HSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum RNGMultConf {
    HSI48RC,
    HSI48DivToRNG,
    HSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCOMultConf {
    LSEOSC,
    LSIDIV,
    HSEOSC,
    HSIRC,
    PLL1R,
    SysCLKOutput,
    MSIRC,
    HSI48RC,
    MSIKRC,
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
    LSIDIV,
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
pub enum CortexCLockSelectionConf {
    CortexPrescaler,
    LSEOSC,
    LSIDIV,
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
    APB2Prescaler,
    SysCLKOutput,
    HSIRC,
    MSIKRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI3MultConf {
    APB3Output,
    SysCLKOutput,
    HSIRC,
    MSIKRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI2MultConf {
    APB1Prescaler,
    SysCLKOutput,
    HSIRC,
    MSIKRC,
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
    Value(u32),
}

impl PLL1PConf {
    pub const fn min() -> u32 {
        1
    }

    pub const fn max() -> u32 {
        128
    }

    pub const fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL1PConf::Value(val) => {
                if *val < Self::min() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Underflow(*val, Self::min()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL1P,
                    });
                } else if *val > Self::max() {
                    return Err(ClockError {
                        err_type: ClockErrorType::Overflow(*val, Self::max()),
                        from: ClockNodes::None,
                        to: ClockNodes::PLL1P,
                    });
                }
                Ok(*val as f32)
            }
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

impl PLL1RConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL1RConf::DIV1 => return Ok(1.0),
            PLL1RConf::DIV2 => return Ok(2.0),
            PLL1RConf::DIV4 => return Ok(4.0),
            PLL1RConf::DIV6 => return Ok(6.0),
            PLL1RConf::DIV8 => return Ok(8.0),
            PLL1RConf::DIV10 => return Ok(10.0),
            PLL1RConf::DIV12 => return Ok(12.0),
            PLL1RConf::DIV14 => return Ok(14.0),
            PLL1RConf::DIV16 => return Ok(16.0),
            PLL1RConf::DIV18 => return Ok(18.0),
            PLL1RConf::DIV20 => return Ok(20.0),
            PLL1RConf::DIV22 => return Ok(22.0),
            PLL1RConf::DIV24 => return Ok(24.0),
            PLL1RConf::DIV26 => return Ok(26.0),
            PLL1RConf::DIV28 => return Ok(28.0),
            PLL1RConf::DIV30 => return Ok(30.0),
            PLL1RConf::DIV32 => return Ok(32.0),
            PLL1RConf::DIV34 => return Ok(34.0),
            PLL1RConf::DIV36 => return Ok(36.0),
            PLL1RConf::DIV38 => return Ok(38.0),
            PLL1RConf::DIV40 => return Ok(40.0),
            PLL1RConf::DIV42 => return Ok(42.0),
            PLL1RConf::DIV44 => return Ok(44.0),
            PLL1RConf::DIV46 => return Ok(46.0),
            PLL1RConf::DIV48 => return Ok(48.0),
            PLL1RConf::DIV50 => return Ok(50.0),
            PLL1RConf::DIV52 => return Ok(52.0),
            PLL1RConf::DIV54 => return Ok(54.0),
            PLL1RConf::DIV56 => return Ok(56.0),
            PLL1RConf::DIV58 => return Ok(58.0),
            PLL1RConf::DIV60 => return Ok(60.0),
            PLL1RConf::DIV62 => return Ok(62.0),
            PLL1RConf::DIV64 => return Ok(64.0),
            PLL1RConf::DIV66 => return Ok(66.0),
            PLL1RConf::DIV68 => return Ok(68.0),
            PLL1RConf::DIV70 => return Ok(70.0),
            PLL1RConf::DIV72 => return Ok(72.0),
            PLL1RConf::DIV74 => return Ok(74.0),
            PLL1RConf::DIV76 => return Ok(76.0),
            PLL1RConf::DIV78 => return Ok(78.0),
            PLL1RConf::DIV80 => return Ok(80.0),
            PLL1RConf::DIV82 => return Ok(82.0),
            PLL1RConf::DIV84 => return Ok(84.0),
            PLL1RConf::DIV86 => return Ok(86.0),
            PLL1RConf::DIV88 => return Ok(88.0),
            PLL1RConf::DIV90 => return Ok(90.0),
            PLL1RConf::DIV92 => return Ok(92.0),
            PLL1RConf::DIV94 => return Ok(94.0),
            PLL1RConf::DIV96 => return Ok(96.0),
            PLL1RConf::DIV98 => return Ok(98.0),
            PLL1RConf::DIV100 => return Ok(100.0),
            PLL1RConf::DIV102 => return Ok(102.0),
            PLL1RConf::DIV104 => return Ok(104.0),
            PLL1RConf::DIV106 => return Ok(106.0),
            PLL1RConf::DIV108 => return Ok(108.0),
            PLL1RConf::DIV110 => return Ok(110.0),
            PLL1RConf::DIV112 => return Ok(112.0),
            PLL1RConf::DIV114 => return Ok(114.0),
            PLL1RConf::DIV116 => return Ok(116.0),
            PLL1RConf::DIV118 => return Ok(118.0),
            PLL1RConf::DIV120 => return Ok(120.0),
            PLL1RConf::DIV122 => return Ok(122.0),
            PLL1RConf::DIV124 => return Ok(124.0),
            PLL1RConf::DIV126 => return Ok(126.0),
            PLL1RConf::DIV128 => return Ok(128.0),
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
    pub HSEOSC: HSEOSCConf,
    pub LSIRC: LSIRCConf,
    pub LSIDIV: LSIDIVConf,
    pub LSEOSC: LSEOSCConf,
    pub MSIRC: MSIRCConf,
    pub MSIKRC: MSIKRCConf,
    pub SysClkSource: SysClkSourceConf,
    pub PLLSource: PLLSourceConf,
    pub PLL2Source: PLL2SourceConf,
    pub PLL3Source: PLL3SourceConf,
    pub PLLM: PLLMConf,
    pub PLL2M: PLL2MConf,
    pub PLL3M: PLL3MConf,
    pub RTCClkSource: RTCClkSourceConf,
    pub USART1Mult: USART1MultConf,
    pub USART2Mult: USART2MultConf,
    pub USART3Mult: USART3MultConf,
    pub UART4Mult: UART4MultConf,
    pub UART5Mult: UART5MultConf,
    pub LPUART1Mult: LPUART1MultConf,
    pub LPTIM1Mult: LPTIM1MultConf,
    pub LPTIM2Mult: LPTIM2MultConf,
    pub DACMult: DACMultConf,
    pub ADCMult: ADCMultConf,
    pub CK48Mult: CK48MultConf,
    pub SDMMC1Mult: SDMMC1MultConf,
    pub FDCANMult: FDCANMultConf,
    pub I2C1Mult: I2C1MultConf,
    pub I2C2Mult: I2C2MultConf,
    pub I2C3Mult: I2C3MultConf,
    pub SAI1Mult: SAI1MultConf,
    pub I2C4Mult: I2C4MultConf,
    pub MDF1Mult: MDF1MultConf,
    pub ADF1Mult: ADF1MultConf,
    pub OCTOSPIMMult: OCTOSPIMMultConf,
    pub LPTIM3Mult: LPTIM3MultConf,
    pub RNGMult: RNGMultConf,
    pub MCOMult: MCOMultConf,
    pub MCODiv: MCODivConf,
    pub LSCOMult: LSCOMultConf,
    pub AHBPrescaler: AHBPrescalerConf,
    pub CortexPrescaler: CortexPrescalerConf,
    pub CortexCLockSelection: CortexCLockSelectionConf,
    pub APB1Prescaler: APB1PrescalerConf,
    pub APB2Prescaler: APB2PrescalerConf,
    pub APB3Prescaler: APB3PrescalerConf,
    pub SPI1Mult: SPI1MultConf,
    pub SPI3Mult: SPI3MultConf,
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
            HSEOSC: HSEOSCConf::Value(16000000),
            LSIRC: LSIRCConf::Value(32000),
            LSIDIV: LSIDIVConf::DIV1,
            LSEOSC: LSEOSCConf::Value(32768),
            MSIRC: MSIRCConf::CLOCK_4000,
            MSIKRC: MSIKRCConf::CLOCK_4000,
            SysClkSource: SysClkSourceConf::MSIRC,
            PLLSource: PLLSourceConf::MSIRC,
            PLL2Source: PLL2SourceConf::MSIRC,
            PLL3Source: PLL3SourceConf::MSIRC,
            PLLM: PLLMConf::Value(1),
            PLL2M: PLL2MConf::Value(1),
            PLL3M: PLL3MConf::Value(1),
            RTCClkSource: RTCClkSourceConf::LSIDIV,
            USART1Mult: USART1MultConf::APB2Prescaler,
            USART2Mult: USART2MultConf::APB1Prescaler,
            USART3Mult: USART3MultConf::APB1Prescaler,
            UART4Mult: UART4MultConf::APB1Prescaler,
            UART5Mult: UART5MultConf::APB1Prescaler,
            LPUART1Mult: LPUART1MultConf::APB3Output,
            LPTIM1Mult: LPTIM1MultConf::MSIKRC,
            LPTIM2Mult: LPTIM2MultConf::APB1Prescaler,
            DACMult: DACMultConf::LSIDIV,
            ADCMult: ADCMultConf::HSIRC,
            CK48Mult: CK48MultConf::HSI48RC,
            SDMMC1Mult: SDMMC1MultConf::PLL1P,
            FDCANMult: FDCANMultConf::PLL1Q,
            I2C1Mult: I2C1MultConf::APB1Prescaler,
            I2C2Mult: I2C2MultConf::APB1Prescaler,
            I2C3Mult: I2C3MultConf::APB3Output,
            SAI1Mult: SAI1MultConf::PLL2P,
            I2C4Mult: I2C4MultConf::APB1Prescaler,
            MDF1Mult: MDF1MultConf::AHBOutput,
            ADF1Mult: ADF1MultConf::AHBOutput,
            OCTOSPIMMult: OCTOSPIMMultConf::SysCLKOutput,
            LPTIM3Mult: LPTIM3MultConf::MSIKRC,
            RNGMult: RNGMultConf::HSI48RC,
            MCOMult: MCOMultConf::SysCLKOutput,
            MCODiv: MCODivConf::DIV1,
            LSCOMult: LSCOMultConf::LSIDIV,
            AHBPrescaler: AHBPrescalerConf::DIV1,
            CortexPrescaler: CortexPrescalerConf::DIV1,
            CortexCLockSelection: CortexCLockSelectionConf::CortexPrescaler,
            APB1Prescaler: APB1PrescalerConf::DIV1,
            APB2Prescaler: APB2PrescalerConf::DIV1,
            APB3Prescaler: APB3PrescalerConf::DIV1,
            SPI1Mult: SPI1MultConf::SysCLKOutput,
            SPI3Mult: SPI3MultConf::SysCLKOutput,
            SPI2Mult: SPI2MultConf::SysCLKOutput,
            PLLN: PLLNConf::Value(129),
            PLLFRACN: PLLFRACNConf::Value(0),
            PLL1P: PLL1PConf::Value(2),
            PLL1Q: PLL1QConf::Value(2),
            PLL1R: PLL1RConf::DIV2,
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
        Ok(16000000 as f32)
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
    pub fn MSIKRC_get(&self) -> Result<f32, ClockError> {
        self.MSIKRC.get()
    }
    pub fn SAI1_EXT_get(&self) -> Result<f32, ClockError> {
        Ok(48000 as f32)
    }
    fn SysClkSource_get(&self) -> Result<f32, ClockError> {
        match self.SysClkSource {
            SysClkSourceConf::MSIRC => return self.MSIRC_get(),
            SysClkSourceConf::HSIRC => return self.HSIRC_get(),
            SysClkSourceConf::HSEOSC => return self.HSEOSC_get(),
            SysClkSourceConf::PLL1R => return self.PLL1R_get(),
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
    fn PLL2Source_get(&self) -> Result<f32, ClockError> {
        match self.PLL2Source {
            PLL2SourceConf::MSIRC => return self.MSIRC_get(),
            PLL2SourceConf::HSIRC => return self.HSIRC_get(),
            PLL2SourceConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    fn PLL3Source_get(&self) -> Result<f32, ClockError> {
        match self.PLL3Source {
            PLL3SourceConf::MSIRC => return self.MSIRC_get(),
            PLL3SourceConf::HSIRC => return self.HSIRC_get(),
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
            LPUART1MultConf::APB3Output => return self.APB3Output_get(),
            LPUART1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            LPUART1MultConf::HSIRC => return self.HSIRC_get(),
            LPUART1MultConf::LSEOSC => return self.LSEOSC_get(),
            LPUART1MultConf::MSIKRC => return self.MSIKRC_get(),
        };
    }
    pub fn LPUART1output_get(&self) -> Result<f32, ClockError> {
        self.LPUART1Mult_get()
    }
    fn LPTIM1Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM1Mult {
            LPTIM1MultConf::MSIKRC => return self.MSIKRC_get(),
            LPTIM1MultConf::LSIDIV => return self.LSIDIV_get(),
            LPTIM1MultConf::HSIRC => return self.HSIRC_get(),
            LPTIM1MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn LPTIM1output_get(&self) -> Result<f32, ClockError> {
        let input = self.LPTIM1Mult_get()?;
        if input > (48000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48000000),
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
    fn DACMult_get(&self) -> Result<f32, ClockError> {
        match self.DACMult {
            DACMultConf::LSEOSC => return self.LSEOSC_get(),
            DACMultConf::LSIDIV => return self.LSIDIV_get(),
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
            ADCMultConf::HSIRC => return self.HSIRC_get(),
            ADCMultConf::MSIKRC => return self.MSIKRC_get(),
        };
    }
    pub fn ADCoutput_get(&self) -> Result<f32, ClockError> {
        self.ADCMult_get()
    }
    fn CK48Mult_get(&self) -> Result<f32, ClockError> {
        match self.CK48Mult {
            CK48MultConf::PLL2Q => return self.PLL2Q_get(),
            CK48MultConf::PLL1Q => return self.PLL1Q_get(),
            CK48MultConf::MSIKRC => return self.MSIKRC_get(),
            CK48MultConf::HSI48RC => return self.HSI48RC_get(),
        };
    }
    pub fn CK48output_get(&self) -> Result<f32, ClockError> {
        let input = self.CK48Mult_get()?;
        if input > (48000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48000000),
                from: ClockNodes::CK48Mult,
                to: ClockNodes::CK48output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::CK48Mult,
                to: ClockNodes::CK48output,
            });
        }
        Ok(input)
    }
    pub fn USBoutput_get(&self) -> Result<f32, ClockError> {
        self.CK48Mult_get()
    }
    fn SDMMC1Mult_get(&self) -> Result<f32, ClockError> {
        match self.SDMMC1Mult {
            SDMMC1MultConf::PLL1P => return self.PLL1P_get(),
            SDMMC1MultConf::CK48Mult => return self.CK48Mult_get(),
        };
    }
    pub fn SDMMCC1Output_get(&self) -> Result<f32, ClockError> {
        let input = self.SDMMC1Mult_get()?;
        if input > (55000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 55000000),
                from: ClockNodes::SDMMC1Mult,
                to: ClockNodes::SDMMCC1Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SDMMC1Mult,
                to: ClockNodes::SDMMCC1Output,
            });
        }
        Ok(input)
    }
    fn FDCANMult_get(&self) -> Result<f32, ClockError> {
        match self.FDCANMult {
            FDCANMultConf::PLL1Q => return self.PLL1Q_get(),
            FDCANMultConf::PLL2P => return self.PLL2P_get(),
            FDCANMultConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    pub fn FDCANOutput_get(&self) -> Result<f32, ClockError> {
        self.FDCANMult_get()
    }
    fn I2C1Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C1Mult {
            I2C1MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            I2C1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            I2C1MultConf::HSIRC => return self.HSIRC_get(),
            I2C1MultConf::MSIKRC => return self.MSIKRC_get(),
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
            I2C2MultConf::MSIKRC => return self.MSIKRC_get(),
        };
    }
    pub fn I2C2output_get(&self) -> Result<f32, ClockError> {
        self.I2C2Mult_get()
    }
    fn I2C3Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C3Mult {
            I2C3MultConf::APB3Output => return self.APB3Output_get(),
            I2C3MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            I2C3MultConf::HSIRC => return self.HSIRC_get(),
            I2C3MultConf::MSIKRC => return self.MSIKRC_get(),
        };
    }
    pub fn I2C3output_get(&self) -> Result<f32, ClockError> {
        self.I2C3Mult_get()
    }
    fn SAI1Mult_get(&self) -> Result<f32, ClockError> {
        match self.SAI1Mult {
            SAI1MultConf::PLL2P => return self.PLL2P_get(),
            SAI1MultConf::PLL3P => return self.PLL3P_get(),
            SAI1MultConf::PLL1P => return self.PLL1P_get(),
            SAI1MultConf::SAI1_EXT => return self.SAI1_EXT_get(),
            SAI1MultConf::HSIRC => return self.HSIRC_get(),
        };
    }
    pub fn SAI1output_get(&self) -> Result<f32, ClockError> {
        self.SAI1Mult_get()
    }
    fn I2C4Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C4Mult {
            I2C4MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            I2C4MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            I2C4MultConf::HSIRC => return self.HSIRC_get(),
            I2C4MultConf::MSIKRC => return self.MSIKRC_get(),
        };
    }
    pub fn I2C4output_get(&self) -> Result<f32, ClockError> {
        self.I2C4Mult_get()
    }
    fn MDF1Mult_get(&self) -> Result<f32, ClockError> {
        match self.MDF1Mult {
            MDF1MultConf::AHBOutput => return self.AHBOutput_get(),
            MDF1MultConf::PLL1P => return self.PLL1P_get(),
            MDF1MultConf::PLL3Q => return self.PLL3Q_get(),
            MDF1MultConf::SAI1_EXT => return self.SAI1_EXT_get(),
            MDF1MultConf::MSIKRC => return self.MSIKRC_get(),
        };
    }
    pub fn MDF1output_get(&self) -> Result<f32, ClockError> {
        self.MDF1Mult_get()
    }
    fn ADF1Mult_get(&self) -> Result<f32, ClockError> {
        match self.ADF1Mult {
            ADF1MultConf::AHBOutput => return self.AHBOutput_get(),
            ADF1MultConf::PLL1P => return self.PLL1P_get(),
            ADF1MultConf::PLL3Q => return self.PLL3Q_get(),
            ADF1MultConf::SAI1_EXT => return self.SAI1_EXT_get(),
            ADF1MultConf::MSIKRC => return self.MSIKRC_get(),
        };
    }
    pub fn ADF1output_get(&self) -> Result<f32, ClockError> {
        self.ADF1Mult_get()
    }
    fn OCTOSPIMMult_get(&self) -> Result<f32, ClockError> {
        match self.OCTOSPIMMult {
            OCTOSPIMMultConf::MSIKRC => return self.MSIKRC_get(),
            OCTOSPIMMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            OCTOSPIMMultConf::PLL1Q => return self.PLL1Q_get(),
            OCTOSPIMMultConf::PLL2Q => return self.PLL2Q_get(),
        };
    }
    pub fn OCTOSPIMoutput_get(&self) -> Result<f32, ClockError> {
        self.OCTOSPIMMult_get()
    }
    fn LPTIM3Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM3Mult {
            LPTIM3MultConf::MSIKRC => return self.MSIKRC_get(),
            LPTIM3MultConf::LSIDIV => return self.LSIDIV_get(),
            LPTIM3MultConf::HSIRC => return self.HSIRC_get(),
            LPTIM3MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn LPTIM3output_get(&self) -> Result<f32, ClockError> {
        self.LPTIM3Mult_get()
    }
    fn HSI48DivToRNG_get(&self) -> Result<f32, ClockError> {
        let input = self.HSI48RC_get()? as f32;
        let value = 2 as f32;
        Ok((input / value) as f32)
    }

    fn RNGMult_get(&self) -> Result<f32, ClockError> {
        match self.RNGMult {
            RNGMultConf::HSI48RC => return self.HSI48RC_get(),
            RNGMultConf::HSI48DivToRNG => return self.HSI48DivToRNG_get(),
            RNGMultConf::HSIRC => return self.HSIRC_get(),
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
    fn MCOMult_get(&self) -> Result<f32, ClockError> {
        match self.MCOMult {
            MCOMultConf::LSEOSC => return self.LSEOSC_get(),
            MCOMultConf::LSIDIV => return self.LSIDIV_get(),
            MCOMultConf::HSEOSC => return self.HSEOSC_get(),
            MCOMultConf::HSIRC => return self.HSIRC_get(),
            MCOMultConf::PLL1R => return self.PLL1R_get(),
            MCOMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            MCOMultConf::MSIRC => return self.MSIRC_get(),
            MCOMultConf::HSI48RC => return self.HSI48RC_get(),
            MCOMultConf::MSIKRC => return self.MSIKRC_get(),
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
            LSCOMultConf::LSIDIV => return self.LSIDIV_get(),
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

    fn CortexCLockSelection_get(&self) -> Result<f32, ClockError> {
        match self.CortexCLockSelection {
            CortexCLockSelectionConf::CortexPrescaler => return self.CortexPrescaler_get(),
            CortexCLockSelectionConf::LSEOSC => return self.LSEOSC_get(),
            CortexCLockSelectionConf::LSIDIV => return self.LSIDIV_get(),
        };
    }
    pub fn CortexSysOutput_get(&self) -> Result<f32, ClockError> {
        self.CortexCLockSelection_get()
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
        self.APB1Prescaler_get()
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
        self.APB2Prescaler_get()
    }
    fn APB3Prescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.APB3Prescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB3Output_get(&self) -> Result<f32, ClockError> {
        self.APB3Prescaler_get()
    }
    fn TimPrescalerAPB2_get(&self) -> Result<f32, ClockError> {
        let input = self.APB2Prescaler_get()? as f32;
        let value = 2 as f32;
        Ok((input * value) as f32)
    }

    pub fn TimPrescOut2_get(&self) -> Result<f32, ClockError> {
        self.TimPrescalerAPB2_get()
    }
    pub fn UCPD1Output_get(&self) -> Result<f32, ClockError> {
        self.HSIRC_get()
    }
    fn SPI1Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI1Mult {
            SPI1MultConf::APB2Prescaler => return self.APB2Prescaler_get(),
            SPI1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            SPI1MultConf::HSIRC => return self.HSIRC_get(),
            SPI1MultConf::MSIKRC => return self.MSIKRC_get(),
        };
    }
    pub fn SPI1output_get(&self) -> Result<f32, ClockError> {
        self.SPI1Mult_get()
    }
    fn SPI3Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI3Mult {
            SPI3MultConf::APB3Output => return self.APB3Output_get(),
            SPI3MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            SPI3MultConf::HSIRC => return self.HSIRC_get(),
            SPI3MultConf::MSIKRC => return self.MSIKRC_get(),
        };
    }
    pub fn SPI3output_get(&self) -> Result<f32, ClockError> {
        self.SPI3Mult_get()
    }
    fn SPI2Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI2Mult {
            SPI2MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            SPI2MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            SPI2MultConf::HSIRC => return self.HSIRC_get(),
            SPI2MultConf::MSIKRC => return self.MSIKRC_get(),
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
