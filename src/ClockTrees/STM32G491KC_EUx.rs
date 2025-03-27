#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ClockNodes {
    None,
    HSIRC,
    HSI48RC,
    CRSCLKoutput,
    HSEOSC,
    LSIRC,
    LSEOSC,
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
    I2SMult,
    I2Soutput,
    SAI1Mult,
    SAI1output,
    QSPIMult,
    QSPIoutput,
    FDCANMult,
    FDCANoutput,
    LPUART1Mult,
    LPUART1output,
    LPTIM1Mult,
    LPTIM1output,
    ADC12Mult,
    ADC12output,
    ADC345Mult,
    ADC345output,
    CK48Mult,
    CK48output,
    RNGoutput,
    I2C1Mult,
    I2C1output,
    I2C2Mult,
    I2C2output,
    I2C3Mult,
    I2C3output,
    I2S_CKIN,
    MCOMult,
    MCODiv,
    MCOPin,
    LSCOMult,
    LSCOOutput,
    AHBPrescaler,
    HRTIM1CLKoutput,
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
pub enum SysClkSourceConf {
    HSIRC,
    HSEOSC,
    PLLR,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSourceConf {
    HSIRC,
    HSEOSC,
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
    DIV9,
    DIV10,
    DIV11,
    DIV12,
    DIV13,
    DIV14,
    DIV15,
    DIV16,
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
            PLLMConf::DIV9 => return Ok(9.0),
            PLLMConf::DIV10 => return Ok(10.0),
            PLLMConf::DIV11 => return Ok(11.0),
            PLLMConf::DIV12 => return Ok(12.0),
            PLLMConf::DIV13 => return Ok(13.0),
            PLLMConf::DIV14 => return Ok(14.0),
            PLLMConf::DIV15 => return Ok(15.0),
            PLLMConf::DIV16 => return Ok(16.0),
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
pub enum I2SMultConf {
    SysCLKOutput,
    PLLQ,
    HSIRC,
    I2S_CKIN,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SAI1MultConf {
    SysCLKOutput,
    PLLQ,
    HSIRC,
    I2S_CKIN,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum QSPIMultConf {
    SysCLKOutput,
    PLLQ,
    HSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FDCANMultConf {
    APB1Prescaler,
    PLLQ,
    HSEOSC,
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
pub enum ADC12MultConf {
    SysCLKOutput,
    PLLP,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ADC345MultConf {
    SysCLKOutput,
    PLLP,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CK48MultConf {
    PLLQ,
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
pub enum MCOMultConf {
    LSEOSC,
    LSIRC,
    HSEOSC,
    HSIRC,
    PLLR,
    SysCLKOutput,
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
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ClockTree {
    pub HSEOSC: HSEOSCConf,
    pub LSEOSC: LSEOSCConf,
    pub SysClkSource: SysClkSourceConf,
    pub PLLSource: PLLSourceConf,
    pub PLLM: PLLMConf,
    pub RTCClkSource: RTCClkSourceConf,
    pub USART1Mult: USART1MultConf,
    pub USART2Mult: USART2MultConf,
    pub I2SMult: I2SMultConf,
    pub SAI1Mult: SAI1MultConf,
    pub QSPIMult: QSPIMultConf,
    pub FDCANMult: FDCANMultConf,
    pub LPUART1Mult: LPUART1MultConf,
    pub LPTIM1Mult: LPTIM1MultConf,
    pub ADC12Mult: ADC12MultConf,
    pub ADC345Mult: ADC345MultConf,
    pub CK48Mult: CK48MultConf,
    pub I2C1Mult: I2C1MultConf,
    pub I2C2Mult: I2C2MultConf,
    pub I2C3Mult: I2C3MultConf,
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
}

impl Default for ClockTree {
    fn default() -> Self {
        Self {
            HSEOSC: HSEOSCConf::Value(8000000),
            LSEOSC: LSEOSCConf::Value(32768),
            SysClkSource: SysClkSourceConf::HSIRC,
            PLLSource: PLLSourceConf::HSIRC,
            PLLM: PLLMConf::DIV1,
            RTCClkSource: RTCClkSourceConf::LSIRC,
            USART1Mult: USART1MultConf::APB2Prescaler,
            USART2Mult: USART2MultConf::APB1Prescaler,
            I2SMult: I2SMultConf::SysCLKOutput,
            SAI1Mult: SAI1MultConf::SysCLKOutput,
            QSPIMult: QSPIMultConf::SysCLKOutput,
            FDCANMult: FDCANMultConf::APB1Prescaler,
            LPUART1Mult: LPUART1MultConf::APB1Prescaler,
            LPTIM1Mult: LPTIM1MultConf::APB1Prescaler,
            ADC12Mult: ADC12MultConf::SysCLKOutput,
            ADC345Mult: ADC345MultConf::SysCLKOutput,
            CK48Mult: CK48MultConf::HSI48RC,
            I2C1Mult: I2C1MultConf::APB1Prescaler,
            I2C2Mult: I2C2MultConf::APB1Prescaler,
            I2C3Mult: I2C3MultConf::APB1Prescaler,
            MCOMult: MCOMultConf::HSIRC,
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
    pub fn CRSCLKoutput_get(&self) -> Result<f32, ClockError> {
        self.HSI48RC_get()
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
    fn SysClkSource_get(&self) -> Result<f32, ClockError> {
        match self.SysClkSource {
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
        let input = self.LSIRC_get()?;
        if input > (170000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 170000000),
                from: ClockNodes::LSIRC,
                to: ClockNodes::IWDGOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::LSIRC,
                to: ClockNodes::IWDGOutput,
            });
        }
        Ok(input)
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
    fn I2SMult_get(&self) -> Result<f32, ClockError> {
        match self.I2SMult {
            I2SMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            I2SMultConf::PLLQ => return self.PLLQ_get(),
            I2SMultConf::HSIRC => return self.HSIRC_get(),
            I2SMultConf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    pub fn I2Soutput_get(&self) -> Result<f32, ClockError> {
        self.I2SMult_get()
    }
    fn SAI1Mult_get(&self) -> Result<f32, ClockError> {
        match self.SAI1Mult {
            SAI1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            SAI1MultConf::PLLQ => return self.PLLQ_get(),
            SAI1MultConf::HSIRC => return self.HSIRC_get(),
            SAI1MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    pub fn SAI1output_get(&self) -> Result<f32, ClockError> {
        self.SAI1Mult_get()
    }
    fn QSPIMult_get(&self) -> Result<f32, ClockError> {
        match self.QSPIMult {
            QSPIMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            QSPIMultConf::PLLQ => return self.PLLQ_get(),
            QSPIMultConf::HSIRC => return self.HSIRC_get(),
        };
    }
    pub fn QSPIoutput_get(&self) -> Result<f32, ClockError> {
        self.QSPIMult_get()
    }
    fn FDCANMult_get(&self) -> Result<f32, ClockError> {
        match self.FDCANMult {
            FDCANMultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            FDCANMultConf::PLLQ => return self.PLLQ_get(),
            FDCANMultConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    pub fn FDCANoutput_get(&self) -> Result<f32, ClockError> {
        self.FDCANMult_get()
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
    fn ADC12Mult_get(&self) -> Result<f32, ClockError> {
        match self.ADC12Mult {
            ADC12MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            ADC12MultConf::PLLP => return self.PLLP_get(),
        };
    }
    pub fn ADC12output_get(&self) -> Result<f32, ClockError> {
        let input = self.ADC12Mult_get()?;
        if input > (170000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 170000000),
                from: ClockNodes::ADC12Mult,
                to: ClockNodes::ADC12output,
            });
        } else if input < (140000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 140000),
                from: ClockNodes::ADC12Mult,
                to: ClockNodes::ADC12output,
            });
        }
        Ok(input)
    }
    fn ADC345Mult_get(&self) -> Result<f32, ClockError> {
        match self.ADC345Mult {
            ADC345MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            ADC345MultConf::PLLP => return self.PLLP_get(),
        };
    }
    pub fn ADC345output_get(&self) -> Result<f32, ClockError> {
        let input = self.ADC345Mult_get()?;
        if input > (170000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 170000000),
                from: ClockNodes::ADC345Mult,
                to: ClockNodes::ADC345output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::ADC345Mult,
                to: ClockNodes::ADC345output,
            });
        }
        Ok(input)
    }
    fn CK48Mult_get(&self) -> Result<f32, ClockError> {
        match self.CK48Mult {
            CK48MultConf::PLLQ => return self.PLLQ_get(),
            CK48MultConf::HSI48RC => return self.HSI48RC_get(),
        };
    }
    pub fn CK48output_get(&self) -> Result<f32, ClockError> {
        self.CK48Mult_get()
    }
    pub fn RNGoutput_get(&self) -> Result<f32, ClockError> {
        self.CK48Mult_get()
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
    pub fn I2S_CKIN_get(&self) -> Result<f32, ClockError> {
        Ok(12288000 as f32)
    }
    fn MCOMult_get(&self) -> Result<f32, ClockError> {
        match self.MCOMult {
            MCOMultConf::LSEOSC => return self.LSEOSC_get(),
            MCOMultConf::LSIRC => return self.LSIRC_get(),
            MCOMultConf::HSEOSC => return self.HSEOSC_get(),
            MCOMultConf::HSIRC => return self.HSIRC_get(),
            MCOMultConf::PLLR => return self.PLLR_get(),
            MCOMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
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

    pub fn HRTIM1CLKoutput_get(&self) -> Result<f32, ClockError> {
        self.TimPrescOut2_get()
    }
    pub fn PWRCLKoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()?;
        if input > (170000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 170000000),
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
        if input > (170000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 170000000),
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
        if input > (170000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 170000000),
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

    pub fn CortexSysOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.CortexPrescaler_get()?;
        if input > (170000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 170000000),
                from: ClockNodes::CortexPrescaler,
                to: ClockNodes::CortexSysOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::CortexPrescaler,
                to: ClockNodes::CortexSysOutput,
            });
        }
        Ok(input)
    }
    pub fn FCLKCortexOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()?;
        if input > (170000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 170000000),
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
        if input > (170000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 170000000),
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
        if input > (170000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 170000000),
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
        if input > (170000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 170000000),
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
        let input = self.TimPrescalerAPB2_get()?;
        if input > (170000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 170000000),
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
