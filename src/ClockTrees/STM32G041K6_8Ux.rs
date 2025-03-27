#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ClockNodes {
    None,
    HSIRC,
    HSISYS,
    HSI_DIV8,
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
    I2S1Mult,
    I2S1output,
    LPUART1Mult,
    LPUART1output,
    LPTIM1Mult,
    LPTIM1output,
    LPTIM2Mult,
    LPTIM2output,
    TIM1Mult,
    TIM1output,
    ADCMult,
    ADCoutput,
    RNGDIV,
    CK48Mult,
    RNGoutput,
    I2C1Mult,
    I2C1output,
    I2S_CKIN,
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
    APBPrescaler,
    APBOutput,
    TimPrescalerAPB,
    TimPrescOut1,
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
pub enum HSISYSConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
    DIV32,
    DIV64,
    DIV128,
}

impl HSISYSConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            HSISYSConf::DIV1 => return Ok(1.0),
            HSISYSConf::DIV2 => return Ok(2.0),
            HSISYSConf::DIV4 => return Ok(4.0),
            HSISYSConf::DIV8 => return Ok(8.0),
            HSISYSConf::DIV16 => return Ok(16.0),
            HSISYSConf::DIV32 => return Ok(32.0),
            HSISYSConf::DIV64 => return Ok(64.0),
            HSISYSConf::DIV128 => return Ok(128.0),
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
    LSEOSC,
    HSISYS,
    HSEOSC,
    PLLR,
    LSIRC,
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
    APBPrescaler,
    SysCLKOutput,
    HSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2S1MultConf {
    SysCLKOutput,
    PLLP,
    HSIRC,
    I2S_CKIN,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPUART1MultConf {
    APBPrescaler,
    SysCLKOutput,
    HSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM1MultConf {
    APBPrescaler,
    LSIRC,
    HSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM2MultConf {
    APBPrescaler,
    LSIRC,
    HSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum TIM1MultConf {
    TimPrescalerAPB,
    PLLQ,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ADCMultConf {
    SysCLKOutput,
    HSIRC,
    PLLP,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum RNGDIVConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
}

impl RNGDIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            RNGDIVConf::DIV1 => return Ok(1.0),
            RNGDIVConf::DIV2 => return Ok(2.0),
            RNGDIVConf::DIV4 => return Ok(4.0),
            RNGDIVConf::DIV8 => return Ok(8.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CK48MultConf {
    SysCLKOutput,
    PLLQ,
    HSI_DIV8,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C1MultConf {
    APBPrescaler,
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
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCODivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
    DIV32,
    DIV64,
    DIV128,
}

impl MCODivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MCODivConf::DIV1 => return Ok(1.0),
            MCODivConf::DIV2 => return Ok(2.0),
            MCODivConf::DIV4 => return Ok(4.0),
            MCODivConf::DIV8 => return Ok(8.0),
            MCODivConf::DIV16 => return Ok(16.0),
            MCODivConf::DIV32 => return Ok(32.0),
            MCODivConf::DIV64 => return Ok(64.0),
            MCODivConf::DIV128 => return Ok(128.0),
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
pub enum APBPrescalerConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
}

impl APBPrescalerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            APBPrescalerConf::DIV1 => return Ok(1.0),
            APBPrescalerConf::DIV2 => return Ok(2.0),
            APBPrescalerConf::DIV4 => return Ok(4.0),
            APBPrescalerConf::DIV8 => return Ok(8.0),
            APBPrescalerConf::DIV16 => return Ok(16.0),
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
    pub HSISYS: HSISYSConf,
    pub HSEOSC: HSEOSCConf,
    pub LSEOSC: LSEOSCConf,
    pub SysClkSource: SysClkSourceConf,
    pub PLLSource: PLLSourceConf,
    pub PLLM: PLLMConf,
    pub RTCClkSource: RTCClkSourceConf,
    pub USART1Mult: USART1MultConf,
    pub I2S1Mult: I2S1MultConf,
    pub LPUART1Mult: LPUART1MultConf,
    pub LPTIM1Mult: LPTIM1MultConf,
    pub LPTIM2Mult: LPTIM2MultConf,
    pub TIM1Mult: TIM1MultConf,
    pub ADCMult: ADCMultConf,
    pub RNGDIV: RNGDIVConf,
    pub CK48Mult: CK48MultConf,
    pub I2C1Mult: I2C1MultConf,
    pub MCOMult: MCOMultConf,
    pub MCODiv: MCODivConf,
    pub LSCOMult: LSCOMultConf,
    pub AHBPrescaler: AHBPrescalerConf,
    pub CortexPrescaler: CortexPrescalerConf,
    pub APBPrescaler: APBPrescalerConf,
    pub PLLN: PLLNConf,
    pub PLLP: PLLPConf,
    pub PLLQ: PLLQConf,
    pub PLLR: PLLRConf,
}

impl Default for ClockTree {
    fn default() -> Self {
        Self {
            HSISYS: HSISYSConf::DIV1,
            HSEOSC: HSEOSCConf::Value(8000000),
            LSEOSC: LSEOSCConf::Value(32768),
            SysClkSource: SysClkSourceConf::HSISYS,
            PLLSource: PLLSourceConf::HSIRC,
            PLLM: PLLMConf::DIV1,
            RTCClkSource: RTCClkSourceConf::LSIRC,
            USART1Mult: USART1MultConf::APBPrescaler,
            I2S1Mult: I2S1MultConf::SysCLKOutput,
            LPUART1Mult: LPUART1MultConf::APBPrescaler,
            LPTIM1Mult: LPTIM1MultConf::APBPrescaler,
            LPTIM2Mult: LPTIM2MultConf::APBPrescaler,
            TIM1Mult: TIM1MultConf::TimPrescalerAPB,
            ADCMult: ADCMultConf::SysCLKOutput,
            RNGDIV: RNGDIVConf::DIV1,
            CK48Mult: CK48MultConf::HSI_DIV8,
            I2C1Mult: I2C1MultConf::APBPrescaler,
            MCOMult: MCOMultConf::SysCLKOutput,
            MCODiv: MCODivConf::DIV1,
            LSCOMult: LSCOMultConf::LSIRC,
            AHBPrescaler: AHBPrescalerConf::DIV1,
            CortexPrescaler: CortexPrescalerConf::DIV1,
            APBPrescaler: APBPrescalerConf::DIV1,
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
    fn HSISYS_get(&self) -> Result<f32, ClockError> {
        let input = self.HSIRC_get()? as f32;
        let value = self.HSISYS.get()? as f32;
        Ok((input / value) as f32)
    }

    fn HSI_DIV8_get(&self) -> Result<f32, ClockError> {
        let input = self.HSIRC_get()? as f32;
        let value = 8 as f32;
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
    fn SysClkSource_get(&self) -> Result<f32, ClockError> {
        match self.SysClkSource {
            SysClkSourceConf::LSEOSC => return self.LSEOSC_get(),
            SysClkSourceConf::HSISYS => return self.HSISYS_get(),
            SysClkSourceConf::HSEOSC => return self.HSEOSC_get(),
            SysClkSourceConf::PLLR => return self.PLLR_get(),
            SysClkSourceConf::LSIRC => return self.LSIRC_get(),
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
        self.LSIRC_get()
    }
    fn USART1Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART1Mult {
            USART1MultConf::APBPrescaler => return self.APBPrescaler_get(),
            USART1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            USART1MultConf::HSIRC => return self.HSIRC_get(),
            USART1MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn USART1output_get(&self) -> Result<f32, ClockError> {
        self.USART1Mult_get()
    }
    fn I2S1Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2S1Mult {
            I2S1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            I2S1MultConf::PLLP => return self.PLLP_get(),
            I2S1MultConf::HSIRC => return self.HSIRC_get(),
            I2S1MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    pub fn I2S1output_get(&self) -> Result<f32, ClockError> {
        self.I2S1Mult_get()
    }
    fn LPUART1Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPUART1Mult {
            LPUART1MultConf::APBPrescaler => return self.APBPrescaler_get(),
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
            LPTIM1MultConf::APBPrescaler => return self.APBPrescaler_get(),
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
            LPTIM2MultConf::APBPrescaler => return self.APBPrescaler_get(),
            LPTIM2MultConf::LSIRC => return self.LSIRC_get(),
            LPTIM2MultConf::HSIRC => return self.HSIRC_get(),
            LPTIM2MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn LPTIM2output_get(&self) -> Result<f32, ClockError> {
        self.LPTIM2Mult_get()
    }
    fn TIM1Mult_get(&self) -> Result<f32, ClockError> {
        match self.TIM1Mult {
            TIM1MultConf::TimPrescalerAPB => return self.TimPrescalerAPB_get(),
            TIM1MultConf::PLLQ => return self.PLLQ_get(),
        };
    }
    pub fn TIM1output_get(&self) -> Result<f32, ClockError> {
        let input = self.TIM1Mult_get()?;
        if input > (128000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 128000000),
                from: ClockNodes::TIM1Mult,
                to: ClockNodes::TIM1output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::TIM1Mult,
                to: ClockNodes::TIM1output,
            });
        }
        Ok(input)
    }
    fn ADCMult_get(&self) -> Result<f32, ClockError> {
        match self.ADCMult {
            ADCMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            ADCMultConf::HSIRC => return self.HSIRC_get(),
            ADCMultConf::PLLP => return self.PLLP_get(),
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
        } else if input < (140000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 140000),
                from: ClockNodes::ADCMult,
                to: ClockNodes::ADCoutput,
            });
        }
        Ok(input)
    }
    fn RNGDIV_get(&self) -> Result<f32, ClockError> {
        let input = self.CK48Mult_get()? as f32;
        let value = self.RNGDIV.get()? as f32;
        Ok((input / value) as f32)
    }

    fn CK48Mult_get(&self) -> Result<f32, ClockError> {
        match self.CK48Mult {
            CK48MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            CK48MultConf::PLLQ => return self.PLLQ_get(),
            CK48MultConf::HSI_DIV8 => return self.HSI_DIV8_get(),
        };
    }
    pub fn RNGoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.RNGoutput_get()?;
        if input > (48000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48000000),
                from: ClockNodes::RNGoutput,
                to: ClockNodes::RNGoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::RNGoutput,
                to: ClockNodes::RNGoutput,
            });
        }
        Ok(input)
    }
    fn I2C1Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C1Mult {
            I2C1MultConf::APBPrescaler => return self.APBPrescaler_get(),
            I2C1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            I2C1MultConf::HSIRC => return self.HSIRC_get(),
        };
    }
    pub fn I2C1output_get(&self) -> Result<f32, ClockError> {
        self.I2C1Mult_get()
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
    fn APBPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.APBPrescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APBOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.APBPrescaler_get()?;
        if input > (64000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 64000000),
                from: ClockNodes::APBPrescaler,
                to: ClockNodes::APBOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::APBPrescaler,
                to: ClockNodes::APBOutput,
            });
        }
        Ok(input)
    }
    fn TimPrescalerAPB_get(&self) -> Result<f32, ClockError> {
        let input = self.APBPrescaler_get()? as f32;
        let value = 2 as f32;
        Ok((input * value) as f32)
    }

    pub fn TimPrescOut1_get(&self) -> Result<f32, ClockError> {
        self.TimPrescalerAPB_get()
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
        let input = self.PLLP_get()?;
        if input > (40000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 40000000),
                from: ClockNodes::PLLP,
                to: ClockNodes::PLLPoutput,
            });
        } else if input < (3090000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 3090000),
                from: ClockNodes::PLLP,
                to: ClockNodes::PLLPoutput,
            });
        }
        Ok(input)
    }
    fn PLLQ_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLN_get()? as f32;
        let value = self.PLLQ.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn PLLQoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLQ_get()?;
        if input > (33000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 33000000),
                from: ClockNodes::PLLQ,
                to: ClockNodes::PLLQoutput,
            });
        } else if input < (12000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 12000000),
                from: ClockNodes::PLLQ,
                to: ClockNodes::PLLQoutput,
            });
        }
        Ok(input)
    }
    fn PLLR_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLN_get()? as f32;
        let value = self.PLLR.get()? as f32;
        Ok((input / value) as f32)
    }
}
