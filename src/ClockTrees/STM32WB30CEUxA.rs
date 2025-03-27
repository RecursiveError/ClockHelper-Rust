#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ClockNodes {
    None,
    HSIRC,
    HSI48RC,
    HSEOSC,
    LSIRC,
    LSI2RC,
    LSIMult,
    LSEOSC,
    MSIRC,
    HCLKRFMultDiv,
    HCLKRFMult,
    HCLKRFOutput,
    APB3Output,
    LPTIM1Mult,
    LPTIM1output,
    SAI1_EXT,
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
    SMPSMult,
    SMPSDivider,
    SMPSDiv2,
    SMPSoutput,
    LPTIM2Mult,
    LPTIM2output,
    HSERFWKPDevisor,
    RFWKPClkSource,
    RFWKPOutput,
    CK48Mult,
    CK48output,
    RNGDiv,
    RNGMult,
    RNGoutput,
    I2C1Mult,
    I2C1output,
    SAI1Mult,
    SAI1output,
    ADCMult,
    ADCoutput,
    MCOMult,
    MCODiv,
    MCOPin,
    AHB3Prescaler,
    AHB3Output,
    AHB2Prescaler,
    FCLK2CortexOutput,
    AHB2Output,
    Cortex2Prescaler,
    Cortex2SysOutput,
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
pub enum LSIMultConf {
    LSIRC,
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
pub enum HCLKRFMultConf {
    HCLKRFMultDiv,
    HSIRC,
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
pub enum LSCOMultConf {
    LSIRC,
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
pub enum SMPSMultConf {
    MSIRC,
    HSEOSC,
    HSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SMPSDividerConf {
    DIV1,
    DIV2,
    DIV3,
    DIV4,
    DIV6,
}

impl SMPSDividerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            SMPSDividerConf::DIV1 => return Ok(1.0),
            SMPSDividerConf::DIV2 => return Ok(2.0),
            SMPSDividerConf::DIV3 => return Ok(3.0),
            SMPSDividerConf::DIV4 => return Ok(4.0),
            SMPSDividerConf::DIV6 => return Ok(6.0),
        }
    }
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
pub enum RFWKPClkSourceConf {
    HSERFWKPDevisor,
    LSEOSC,
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
pub enum RNGMultConf {
    RNGDiv,
    LSIRC,
    LSEOSC,
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
pub enum SAI1MultConf {
    PLLSAI1P,
    PLLP,
    HSIRC,
    SAI1_EXT,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ADCMultConf {
    PLLSAI1R,
    PLLP,
    SysCLKOutput,
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
pub enum AHB3PrescalerConf {
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

impl AHB3PrescalerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            AHB3PrescalerConf::DIV1 => return Ok(1.0),
            AHB3PrescalerConf::DIV2 => return Ok(2.0),
            AHB3PrescalerConf::DIV4 => return Ok(4.0),
            AHB3PrescalerConf::DIV8 => return Ok(8.0),
            AHB3PrescalerConf::DIV16 => return Ok(16.0),
            AHB3PrescalerConf::DIV64 => return Ok(64.0),
            AHB3PrescalerConf::DIV128 => return Ok(128.0),
            AHB3PrescalerConf::DIV256 => return Ok(256.0),
            AHB3PrescalerConf::DIV512 => return Ok(512.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum AHB2PrescalerConf {
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

impl AHB2PrescalerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            AHB2PrescalerConf::DIV1 => return Ok(1.0),
            AHB2PrescalerConf::DIV2 => return Ok(2.0),
            AHB2PrescalerConf::DIV4 => return Ok(4.0),
            AHB2PrescalerConf::DIV8 => return Ok(8.0),
            AHB2PrescalerConf::DIV16 => return Ok(16.0),
            AHB2PrescalerConf::DIV64 => return Ok(64.0),
            AHB2PrescalerConf::DIV128 => return Ok(128.0),
            AHB2PrescalerConf::DIV256 => return Ok(256.0),
            AHB2PrescalerConf::DIV512 => return Ok(512.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum Cortex2PrescalerConf {
    DIV1,
    DIV8,
}

impl Cortex2PrescalerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            Cortex2PrescalerConf::DIV1 => return Ok(1.0),
            Cortex2PrescalerConf::DIV8 => return Ok(8.0),
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSAI1NConf {
    Value(u32),
}

impl PLLSAI1NConf {
    pub const fn min() -> u32 {
        4
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
    DIV3,
    DIV4,
    DIV5,
    DIV6,
    DIV7,
    DIV8,
}

impl PLLSAI1QConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLSAI1QConf::DIV2 => return Ok(2.0),
            PLLSAI1QConf::DIV3 => return Ok(3.0),
            PLLSAI1QConf::DIV4 => return Ok(4.0),
            PLLSAI1QConf::DIV5 => return Ok(5.0),
            PLLSAI1QConf::DIV6 => return Ok(6.0),
            PLLSAI1QConf::DIV7 => return Ok(7.0),
            PLLSAI1QConf::DIV8 => return Ok(8.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSAI1RConf {
    DIV2,
    DIV3,
    DIV4,
    DIV5,
    DIV6,
    DIV7,
    DIV8,
}

impl PLLSAI1RConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLSAI1RConf::DIV2 => return Ok(2.0),
            PLLSAI1RConf::DIV3 => return Ok(3.0),
            PLLSAI1RConf::DIV4 => return Ok(4.0),
            PLLSAI1RConf::DIV5 => return Ok(5.0),
            PLLSAI1RConf::DIV6 => return Ok(6.0),
            PLLSAI1RConf::DIV7 => return Ok(7.0),
            PLLSAI1RConf::DIV8 => return Ok(8.0),
        }
    }
}
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ClockTree {
    pub HSEOSC: HSEOSCConf,
    pub LSIMult: LSIMultConf,
    pub LSEOSC: LSEOSCConf,
    pub MSIRC: MSIRCConf,
    pub HCLKRFMult: HCLKRFMultConf,
    pub LPTIM1Mult: LPTIM1MultConf,
    pub LSCOMult: LSCOMultConf,
    pub HSEPRESC: HSEPRESCConf,
    pub SysClkSource: SysClkSourceConf,
    pub PLLSource: PLLSourceConf,
    pub PLLM: PLLMConf,
    pub RTCClkSource: RTCClkSourceConf,
    pub USART1Mult: USART1MultConf,
    pub SMPSMult: SMPSMultConf,
    pub SMPSDivider: SMPSDividerConf,
    pub LPTIM2Mult: LPTIM2MultConf,
    pub RFWKPClkSource: RFWKPClkSourceConf,
    pub CK48Mult: CK48MultConf,
    pub RNGMult: RNGMultConf,
    pub I2C1Mult: I2C1MultConf,
    pub SAI1Mult: SAI1MultConf,
    pub ADCMult: ADCMultConf,
    pub MCOMult: MCOMultConf,
    pub MCODiv: MCODivConf,
    pub AHB3Prescaler: AHB3PrescalerConf,
    pub AHB2Prescaler: AHB2PrescalerConf,
    pub Cortex2Prescaler: Cortex2PrescalerConf,
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
}

impl Default for ClockTree {
    fn default() -> Self {
        Self {
            HSEOSC: HSEOSCConf::Value(8000000),
            LSIMult: LSIMultConf::LSIRC,
            LSEOSC: LSEOSCConf::Value(32768),
            MSIRC: MSIRCConf::CLOCK_4000,
            HCLKRFMult: HCLKRFMultConf::HSIRC,
            LPTIM1Mult: LPTIM1MultConf::APB1Prescaler,
            LSCOMult: LSCOMultConf::LSIRC,
            HSEPRESC: HSEPRESCConf::Value(1),
            SysClkSource: SysClkSourceConf::MSIRC,
            PLLSource: PLLSourceConf::MSIRC,
            PLLM: PLLMConf::DIV1,
            RTCClkSource: RTCClkSourceConf::LSIRC,
            USART1Mult: USART1MultConf::APB2Prescaler,
            SMPSMult: SMPSMultConf::HSIRC,
            SMPSDivider: SMPSDividerConf::DIV2,
            LPTIM2Mult: LPTIM2MultConf::APB1Prescaler,
            RFWKPClkSource: RFWKPClkSourceConf::HSERFWKPDevisor,
            CK48Mult: CK48MultConf::PLLSAI1Q,
            RNGMult: RNGMultConf::LSIRC,
            I2C1Mult: I2C1MultConf::APB1Prescaler,
            SAI1Mult: SAI1MultConf::PLLSAI1P,
            ADCMult: ADCMultConf::PLLSAI1R,
            MCOMult: MCOMultConf::SysCLKOutput,
            MCODiv: MCODivConf::DIV1,
            AHB3Prescaler: AHB3PrescalerConf::DIV1,
            AHB2Prescaler: AHB2PrescalerConf::DIV1,
            Cortex2Prescaler: Cortex2PrescalerConf::DIV1,
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
    pub fn LSI2RC_get(&self) -> Result<f32, ClockError> {
        Ok(32000 as f32)
    }
    fn LSIMult_get(&self) -> Result<f32, ClockError> {
        match self.LSIMult {
            LSIMultConf::LSIRC => return self.LSIRC_get(),
            LSIMultConf::LSI2RC => return self.LSI2RC_get(),
        };
    }
    pub fn LSEOSC_get(&self) -> Result<f32, ClockError> {
        self.LSEOSC.get()
    }
    pub fn MSIRC_get(&self) -> Result<f32, ClockError> {
        self.MSIRC.get()
    }
    fn HCLKRFMultDiv_get(&self) -> Result<f32, ClockError> {
        let input = self.HSEOSC_get()? as f32;
        let value = 2 as f32;
        Ok((input / value) as f32)
    }

    fn HCLKRFMult_get(&self) -> Result<f32, ClockError> {
        match self.HCLKRFMult {
            HCLKRFMultConf::HCLKRFMultDiv => return self.HCLKRFMultDiv_get(),
            HCLKRFMultConf::HSIRC => return self.HSIRC_get(),
        };
    }
    pub fn HCLKRFOutput_get(&self) -> Result<f32, ClockError> {
        self.HCLKRFMult_get()
    }
    pub fn APB3Output_get(&self) -> Result<f32, ClockError> {
        self.HCLKRFMult_get()
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
    pub fn SAI1_EXT_get(&self) -> Result<f32, ClockError> {
        Ok(2097000 as f32)
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
        if input > (64000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 64000000),
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
    fn SMPSMult_get(&self) -> Result<f32, ClockError> {
        match self.SMPSMult {
            SMPSMultConf::MSIRC => return self.MSIRC_get(),
            SMPSMultConf::HSEOSC => return self.HSEOSC_get(),
            SMPSMultConf::HSIRC => return self.HSIRC_get(),
        };
    }
    fn SMPSDivider_get(&self) -> Result<f32, ClockError> {
        let input = self.SMPSMult_get()? as f32;
        let value = self.SMPSDivider.get()? as f32;
        Ok((input / value) as f32)
    }

    fn SMPSDiv2_get(&self) -> Result<f32, ClockError> {
        let input = self.SMPSDivider_get()? as f32;
        let value = 2 as f32;
        Ok((input / value) as f32)
    }

    pub fn SMPSoutput_get(&self) -> Result<f32, ClockError> {
        self.SMPSDiv2_get()
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
    fn HSERFWKPDevisor_get(&self) -> Result<f32, ClockError> {
        let input = self.HSEOSC_get()? as f32;
        let value = 1024 as f32;
        Ok((input / value) as f32)
    }

    fn RFWKPClkSource_get(&self) -> Result<f32, ClockError> {
        match self.RFWKPClkSource {
            RFWKPClkSourceConf::HSERFWKPDevisor => return self.HSERFWKPDevisor_get(),
            RFWKPClkSourceConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn RFWKPOutput_get(&self) -> Result<f32, ClockError> {
        self.RFWKPClkSource_get()
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
    fn RNGDiv_get(&self) -> Result<f32, ClockError> {
        let input = self.CK48Mult_get()? as f32;
        let value = 3 as f32;
        Ok((input / value) as f32)
    }

    fn RNGMult_get(&self) -> Result<f32, ClockError> {
        match self.RNGMult {
            RNGMultConf::RNGDiv => return self.RNGDiv_get(),
            RNGMultConf::LSIRC => return self.LSIRC_get(),
            RNGMultConf::LSEOSC => return self.LSEOSC_get(),
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
    fn SAI1Mult_get(&self) -> Result<f32, ClockError> {
        match self.SAI1Mult {
            SAI1MultConf::PLLSAI1P => return self.PLLSAI1P_get(),
            SAI1MultConf::PLLP => return self.PLLP_get(),
            SAI1MultConf::HSIRC => return self.HSIRC_get(),
            SAI1MultConf::SAI1_EXT => return self.SAI1_EXT_get(),
        };
    }
    pub fn SAI1output_get(&self) -> Result<f32, ClockError> {
        self.SAI1Mult_get()
    }
    fn ADCMult_get(&self) -> Result<f32, ClockError> {
        match self.ADCMult {
            ADCMultConf::PLLSAI1R => return self.PLLSAI1R_get(),
            ADCMultConf::PLLP => return self.PLLP_get(),
            ADCMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
        };
    }
    pub fn ADCoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.ADCMult_get()?;
        if input > (64000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 64000000),
                from: ClockNodes::ADCMult,
                to: ClockNodes::ADCoutput,
            });
        } else if input < (140000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 140000),
                from: ClockNodes::ADCMult,
                to: ClockNodes::ADCoutput,
            });
        }
        Ok(input)
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
    fn AHB3Prescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()? as f32;
        let value = self.AHB3Prescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn AHB3Output_get(&self) -> Result<f32, ClockError> {
        let input = self.AHB3Prescaler_get()?;
        if input > (64000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 64000000),
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
    fn AHB2Prescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()? as f32;
        let value = self.AHB2Prescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn FCLK2CortexOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.AHB2Prescaler_get()?;
        if input > (32000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 32000000),
                from: ClockNodes::AHB2Prescaler,
                to: ClockNodes::FCLK2CortexOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::AHB2Prescaler,
                to: ClockNodes::FCLK2CortexOutput,
            });
        }
        Ok(input)
    }
    pub fn AHB2Output_get(&self) -> Result<f32, ClockError> {
        let input = self.AHB2Prescaler_get()?;
        if input > (32000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 32000000),
                from: ClockNodes::AHB2Prescaler,
                to: ClockNodes::AHB2Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::AHB2Prescaler,
                to: ClockNodes::AHB2Output,
            });
        }
        Ok(input)
    }
    fn Cortex2Prescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.AHB2Prescaler_get()? as f32;
        let value = self.Cortex2Prescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn Cortex2SysOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.Cortex2Prescaler_get()?;
        if input > (32000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 32000000),
                from: ClockNodes::Cortex2Prescaler,
                to: ClockNodes::Cortex2SysOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::Cortex2Prescaler,
                to: ClockNodes::Cortex2SysOutput,
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
        if input > (64000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 64000000),
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
        if input > (64000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 64000000),
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

    pub fn APB1Output_get(&self) -> Result<f32, ClockError> {
        let input = self.APB1Prescaler_get()?;
        if input > (64000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 64000000),
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
        if input > (64000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 64000000),
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
}
