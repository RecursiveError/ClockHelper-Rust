#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ClockNodes {
    None,
    HSIRC,
    HSISYS,
    HSIKER,
    HSEOSC,
    LSIRC,
    LSEOSC,
    SysClkSource,
    SYSDIV,
    SysCLKOutput,
    HSERTCDevisor,
    RTCClkSource,
    RTCOutput,
    IWDGOutput,
    USART1Mult,
    USART1output,
    I2S1Mult,
    I2S1output,
    ADCMult,
    ADCoutput,
    I2C1Mult,
    I2C1output,
    I2S_CKIN,
    MCOMult,
    MCODiv,
    MCOPin,
    MCO2Mult,
    MCO2Div,
    MCO2Pin,
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
pub enum HSIKERConf {
    DIV1,
    DIV2,
    DIV3,
    DIV4,
    DIV5,
    DIV6,
    DIV7,
    DIV8,
}

impl HSIKERConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            HSIKERConf::DIV1 => return Ok(1.0),
            HSIKERConf::DIV2 => return Ok(2.0),
            HSIKERConf::DIV3 => return Ok(3.0),
            HSIKERConf::DIV4 => return Ok(4.0),
            HSIKERConf::DIV5 => return Ok(5.0),
            HSIKERConf::DIV6 => return Ok(6.0),
            HSIKERConf::DIV7 => return Ok(7.0),
            HSIKERConf::DIV8 => return Ok(8.0),
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
    LSIRC,
    HSEOSC,
    HSISYS,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SYSDIVConf {
    DIV1,
}

impl SYSDIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            SYSDIVConf::DIV1 => return Ok(1.0),
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
    HSIKER,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2S1MultConf {
    SysCLKOutput,
    HSIKER,
    I2S_CKIN,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ADCMultConf {
    SysCLKOutput,
    HSIKER,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C1MultConf {
    APBPrescaler,
    SysCLKOutput,
    HSIKER,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCOMultConf {
    LSEOSC,
    LSIRC,
    HSEOSC,
    HSIRC,
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
pub enum MCO2MultConf {
    LSEOSC,
    LSIRC,
    HSEOSC,
    HSIRC,
    SysCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCO2DivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
    DIV32,
    DIV64,
    DIV128,
}

impl MCO2DivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MCO2DivConf::DIV1 => return Ok(1.0),
            MCO2DivConf::DIV2 => return Ok(2.0),
            MCO2DivConf::DIV4 => return Ok(4.0),
            MCO2DivConf::DIV8 => return Ok(8.0),
            MCO2DivConf::DIV16 => return Ok(16.0),
            MCO2DivConf::DIV32 => return Ok(32.0),
            MCO2DivConf::DIV64 => return Ok(64.0),
            MCO2DivConf::DIV128 => return Ok(128.0),
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
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ClockTree {
    pub HSISYS: HSISYSConf,
    pub HSIKER: HSIKERConf,
    pub HSEOSC: HSEOSCConf,
    pub LSIRC: LSIRCConf,
    pub LSEOSC: LSEOSCConf,
    pub SysClkSource: SysClkSourceConf,
    pub SYSDIV: SYSDIVConf,
    pub RTCClkSource: RTCClkSourceConf,
    pub USART1Mult: USART1MultConf,
    pub I2S1Mult: I2S1MultConf,
    pub ADCMult: ADCMultConf,
    pub I2C1Mult: I2C1MultConf,
    pub MCOMult: MCOMultConf,
    pub MCODiv: MCODivConf,
    pub MCO2Mult: MCO2MultConf,
    pub MCO2Div: MCO2DivConf,
    pub LSCOMult: LSCOMultConf,
    pub AHBPrescaler: AHBPrescalerConf,
    pub CortexPrescaler: CortexPrescalerConf,
    pub APBPrescaler: APBPrescalerConf,
}

impl Default for ClockTree {
    fn default() -> Self {
        Self {
            HSISYS: HSISYSConf::DIV4,
            HSIKER: HSIKERConf::DIV1,
            HSEOSC: HSEOSCConf::Value(8000000),
            LSIRC: LSIRCConf::Value(32000),
            LSEOSC: LSEOSCConf::Value(32768),
            SysClkSource: SysClkSourceConf::HSISYS,
            SYSDIV: SYSDIVConf::DIV1,
            RTCClkSource: RTCClkSourceConf::LSIRC,
            USART1Mult: USART1MultConf::APBPrescaler,
            I2S1Mult: I2S1MultConf::SysCLKOutput,
            ADCMult: ADCMultConf::SysCLKOutput,
            I2C1Mult: I2C1MultConf::APBPrescaler,
            MCOMult: MCOMultConf::SysCLKOutput,
            MCODiv: MCODivConf::DIV1,
            MCO2Mult: MCO2MultConf::SysCLKOutput,
            MCO2Div: MCO2DivConf::DIV1,
            LSCOMult: LSCOMultConf::LSIRC,
            AHBPrescaler: AHBPrescalerConf::DIV1,
            CortexPrescaler: CortexPrescalerConf::DIV1,
            APBPrescaler: APBPrescalerConf::DIV1,
        }
    }
}
impl ClockTree {
    pub fn HSIRC_get(&self) -> Result<f32, ClockError> {
        Ok(48000000 as f32)
    }
    fn HSISYS_get(&self) -> Result<f32, ClockError> {
        let input = self.HSIRC_get()? as f32;
        let value = self.HSISYS.get()? as f32;
        Ok((input / value) as f32)
    }

    fn HSIKER_get(&self) -> Result<f32, ClockError> {
        let input = self.HSIRC_get()? as f32;
        let value = self.HSIKER.get()? as f32;
        Ok((input / value) as f32)
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
    fn SysClkSource_get(&self) -> Result<f32, ClockError> {
        match self.SysClkSource {
            SysClkSourceConf::LSIRC => return self.LSIRC_get(),
            SysClkSourceConf::HSEOSC => return self.HSEOSC_get(),
            SysClkSourceConf::HSISYS => return self.HSISYS_get(),
            SysClkSourceConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    fn SYSDIV_get(&self) -> Result<f32, ClockError> {
        let input = self.SysClkSource_get()? as f32;
        let value = self.SYSDIV.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn SysCLKOutput_get(&self) -> Result<f32, ClockError> {
        self.SYSDIV_get()
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
    pub fn IWDGOutput_get(&self) -> Result<f32, ClockError> {
        self.LSIRC_get()
    }
    fn USART1Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART1Mult {
            USART1MultConf::APBPrescaler => return self.APBPrescaler_get(),
            USART1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            USART1MultConf::HSIKER => return self.HSIKER_get(),
            USART1MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn USART1output_get(&self) -> Result<f32, ClockError> {
        self.USART1Mult_get()
    }
    fn I2S1Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2S1Mult {
            I2S1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            I2S1MultConf::HSIKER => return self.HSIKER_get(),
            I2S1MultConf::I2S_CKIN => return self.I2S_CKIN_get(),
        };
    }
    pub fn I2S1output_get(&self) -> Result<f32, ClockError> {
        self.I2S1Mult_get()
    }
    fn ADCMult_get(&self) -> Result<f32, ClockError> {
        match self.ADCMult {
            ADCMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            ADCMultConf::HSIKER => return self.HSIKER_get(),
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
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::ADCMult,
                to: ClockNodes::ADCoutput,
            });
        }
        Ok(input)
    }
    fn I2C1Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C1Mult {
            I2C1MultConf::APBPrescaler => return self.APBPrescaler_get(),
            I2C1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            I2C1MultConf::HSIKER => return self.HSIKER_get(),
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
    fn MCO2Mult_get(&self) -> Result<f32, ClockError> {
        match self.MCO2Mult {
            MCO2MultConf::LSEOSC => return self.LSEOSC_get(),
            MCO2MultConf::LSIRC => return self.LSIRC_get(),
            MCO2MultConf::HSEOSC => return self.HSEOSC_get(),
            MCO2MultConf::HSIRC => return self.HSIRC_get(),
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
    fn APBPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.APBPrescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APBOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.APBPrescaler_get()?;
        if input > (48000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48000000),
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
        let input = self.TimPrescalerAPB_get()?;
        if input > (48000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48000000),
                from: ClockNodes::TimPrescalerAPB,
                to: ClockNodes::TimPrescOut1,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::TimPrescalerAPB,
                to: ClockNodes::TimPrescOut1,
            });
        }
        Ok(input)
    }
}
