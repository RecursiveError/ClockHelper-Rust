#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ClockNodes {
    None,
    HSIRC,
    MSIRC,
    HSIRCDiv,
    LSIRC,
    LSEOSC,
    RTCClkSource,
    RTCOutput,
    LCDOutput,
    IWDGOutput,
    SysClkSource,
    PLLSource,
    AHBPrescaler,
    SysCLKOutput,
    AHBOutput,
    HCLKOutput,
    TIMPrescaler,
    TIMOutput,
    FCLKCortexOutput,
    APB1Prescaler,
    APB1Output,
    TimPrescalerAPB1,
    TimPrescOut,
    APB2Prescaler,
    APB2Output,
    PeriphPrescaler,
    PeriphPrescOutput,
    LPTIMMult,
    LPTIMOutput,
    LPUARTMult,
    LPUARTOutput,
    USART2Mult,
    USART2Output,
    USART1Mult,
    USART1Output,
    I2C1Mult,
    I2C1Output,
    I2C3Mult,
    I2C3Output,
    MCOMult,
    MCODiv,
    MCOPin,
    HSI48MUL,
    USBOutput,
    RNGOutput,
    ADCOutput,
    VCOIIuput,
    PLLMUL,
    PLLDIV,
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
pub enum MSIRCConf {
    CLOCK_65_5,
    CLOCK_131_0,
    CLOCK_262_1,
    CLOCK_524_2,
    CLOCK_1048,
    CLOCK_2097,
    CLOCK_4194,
}

impl MSIRCConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MSIRCConf::CLOCK_65_5 => return Ok(65.536),
            MSIRCConf::CLOCK_131_0 => return Ok(131.072),
            MSIRCConf::CLOCK_262_1 => return Ok(262.144),
            MSIRCConf::CLOCK_524_2 => return Ok(524.288),
            MSIRCConf::CLOCK_1048 => return Ok(1048.0),
            MSIRCConf::CLOCK_2097 => return Ok(2097.0),
            MSIRCConf::CLOCK_4194 => return Ok(4194.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum HSIRCDivConf {
    DIV1,
    DIV4,
}

impl HSIRCDivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            HSIRCDivConf::DIV1 => return Ok(1.0),
            HSIRCDivConf::DIV4 => return Ok(4.0),
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
pub enum RTCClkSourceConf {
    LSEOSC,
    LSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SysClkSourceConf {
    MSIRC,
    HSIRCDiv,
    PLLDIV,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSourceConf {
    HSIRCDiv,
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
pub enum TIMPrescalerConf {
    DIV1,
    DIV8,
}

impl TIMPrescalerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            TIMPrescalerConf::DIV1 => return Ok(1.0),
            TIMPrescalerConf::DIV8 => return Ok(8.0),
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
pub enum LPTIMMultConf {
    LSIRC,
    HSIRCDiv,
    LSEOSC,
    APB1Prescaler,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPUARTMultConf {
    APB1Prescaler,
    LSEOSC,
    HSIRCDiv,
    SysCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART2MultConf {
    SysCLKOutput,
    HSIRCDiv,
    LSEOSC,
    APB1Prescaler,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART1MultConf {
    APB2Prescaler,
    SysCLKOutput,
    HSIRCDiv,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C1MultConf {
    APB1Prescaler,
    HSIRCDiv,
    SysCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C3MultConf {
    APB1Prescaler,
    HSIRCDiv,
    SysCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCOMultConf {
    LSEOSC,
    LSIRC,
    HSIRCDiv,
    PLLDIV,
    SysCLKOutput,
    MSIRC,
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
pub enum HSI48MULConf {}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLMULConf {
    MUL3,
    MUL4,
    MUL6,
    MUL8,
    MUL12,
    MUL16,
    MUL24,
    MUL32,
    MUL48,
}

impl PLLMULConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLMULConf::MUL3 => return Ok(3.0),
            PLLMULConf::MUL4 => return Ok(4.0),
            PLLMULConf::MUL6 => return Ok(6.0),
            PLLMULConf::MUL8 => return Ok(8.0),
            PLLMULConf::MUL12 => return Ok(12.0),
            PLLMULConf::MUL16 => return Ok(16.0),
            PLLMULConf::MUL24 => return Ok(24.0),
            PLLMULConf::MUL32 => return Ok(32.0),
            PLLMULConf::MUL48 => return Ok(48.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLDIVConf {
    DIV2,
    DIV3,
    DIV4,
}

impl PLLDIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLDIVConf::DIV2 => return Ok(2.0),
            PLLDIVConf::DIV3 => return Ok(3.0),
            PLLDIVConf::DIV4 => return Ok(4.0),
        }
    }
}
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ClockTree {
    pub MSIRC: MSIRCConf,
    pub HSIRCDiv: HSIRCDivConf,
    pub LSEOSC: LSEOSCConf,
    pub RTCClkSource: RTCClkSourceConf,
    pub SysClkSource: SysClkSourceConf,
    pub PLLSource: PLLSourceConf,
    pub AHBPrescaler: AHBPrescalerConf,
    pub TIMPrescaler: TIMPrescalerConf,
    pub APB1Prescaler: APB1PrescalerConf,
    pub APB2Prescaler: APB2PrescalerConf,
    pub LPTIMMult: LPTIMMultConf,
    pub LPUARTMult: LPUARTMultConf,
    pub USART2Mult: USART2MultConf,
    pub USART1Mult: USART1MultConf,
    pub I2C1Mult: I2C1MultConf,
    pub I2C3Mult: I2C3MultConf,
    pub MCOMult: MCOMultConf,
    pub MCODiv: MCODivConf,
    pub HSI48MUL: HSI48MULConf,
    pub PLLMUL: PLLMULConf,
    pub PLLDIV: PLLDIVConf,
}

impl Default for ClockTree {
    fn default() -> Self {
        Self {
            MSIRC: MSIRCConf::CLOCK_2097,
            HSIRCDiv: HSIRCDivConf::DIV1,
            LSEOSC: LSEOSCConf::Value(32768),
            RTCClkSource: RTCClkSourceConf::LSIRC,
            SysClkSource: SysClkSourceConf::MSIRC,
            PLLSource: PLLSourceConf::HSIRCDiv,
            AHBPrescaler: AHBPrescalerConf::DIV1,
            TIMPrescaler: TIMPrescalerConf::DIV1,
            APB1Prescaler: APB1PrescalerConf::DIV1,
            APB2Prescaler: APB2PrescalerConf::DIV1,
            LPTIMMult: LPTIMMultConf::APB1Prescaler,
            LPUARTMult: LPUARTMultConf::APB1Prescaler,
            USART2Mult: USART2MultConf::APB1Prescaler,
            USART1Mult: USART1MultConf::APB2Prescaler,
            I2C1Mult: I2C1MultConf::APB1Prescaler,
            I2C3Mult: I2C3MultConf::APB1Prescaler,
            MCOMult: MCOMultConf::SysCLKOutput,
            MCODiv: MCODivConf::DIV1,
            HSI48MUL: HSI48MULConf::DIV2USB,
            PLLMUL: PLLMULConf::MUL3,
            PLLDIV: PLLDIVConf::DIV2,
        }
    }
}
impl ClockTree {
    pub fn HSIRC_get(&self) -> Result<f32, ClockError> {
        Ok(16000000 as f32)
    }
    pub fn MSIRC_get(&self) -> Result<f32, ClockError> {
        self.MSIRC.get()
    }
    fn HSIRCDiv_get(&self) -> Result<f32, ClockError> {
        let input = self.HSIRC_get()? as f32;
        let value = self.HSIRCDiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn LSIRC_get(&self) -> Result<f32, ClockError> {
        Ok(37000 as f32)
    }
    pub fn LSEOSC_get(&self) -> Result<f32, ClockError> {
        self.LSEOSC.get()
    }
    fn RTCClkSource_get(&self) -> Result<f32, ClockError> {
        match self.RTCClkSource {
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
    fn SysClkSource_get(&self) -> Result<f32, ClockError> {
        match self.SysClkSource {
            SysClkSourceConf::MSIRC => return self.MSIRC_get(),
            SysClkSourceConf::HSIRCDiv => return self.HSIRCDiv_get(),
            SysClkSourceConf::PLLDIV => return self.PLLDIV_get(),
        };
    }
    fn PLLSource_get(&self) -> Result<f32, ClockError> {
        match self.PLLSource {
            PLLSourceConf::HSIRCDiv => return self.HSIRCDiv_get(),
        };
    }
    fn AHBPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()? as f32;
        let value = self.AHBPrescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn SysCLKOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SysClkSource_get()?;
        if input > (32000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 32000000),
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
    pub fn AHBOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBPrescaler_get()?;
        if input > (32000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 32000000),
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
    fn TIMPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.TIMPrescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn TIMOutput_get(&self) -> Result<f32, ClockError> {
        self.TIMPrescaler_get()
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
        if input > (32000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 32000000),
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

    pub fn TimPrescOut_get(&self) -> Result<f32, ClockError> {
        self.TimPrescalerAPB1_get()
    }
    fn APB2Prescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.APB2Prescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB2Output_get(&self) -> Result<f32, ClockError> {
        let input = self.APB2Prescaler_get()?;
        if input > (32000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 32000000),
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
    fn PeriphPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.APB2Prescaler_get()? as f32;
        let value = 2 as f32;
        Ok((input * value) as f32)
    }

    pub fn PeriphPrescOutput_get(&self) -> Result<f32, ClockError> {
        self.PeriphPrescaler_get()
    }
    fn LPTIMMult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIMMult {
            LPTIMMultConf::LSIRC => return self.LSIRC_get(),
            LPTIMMultConf::HSIRCDiv => return self.HSIRCDiv_get(),
            LPTIMMultConf::LSEOSC => return self.LSEOSC_get(),
            LPTIMMultConf::APB1Prescaler => return self.APB1Prescaler_get(),
        };
    }
    pub fn LPTIMOutput_get(&self) -> Result<f32, ClockError> {
        self.LPTIMMult_get()
    }
    fn LPUARTMult_get(&self) -> Result<f32, ClockError> {
        match self.LPUARTMult {
            LPUARTMultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            LPUARTMultConf::LSEOSC => return self.LSEOSC_get(),
            LPUARTMultConf::HSIRCDiv => return self.HSIRCDiv_get(),
            LPUARTMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
        };
    }
    pub fn LPUARTOutput_get(&self) -> Result<f32, ClockError> {
        self.LPUARTMult_get()
    }
    fn USART2Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART2Mult {
            USART2MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            USART2MultConf::HSIRCDiv => return self.HSIRCDiv_get(),
            USART2MultConf::LSEOSC => return self.LSEOSC_get(),
            USART2MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
        };
    }
    pub fn USART2Output_get(&self) -> Result<f32, ClockError> {
        self.USART2Mult_get()
    }
    fn USART1Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART1Mult {
            USART1MultConf::APB2Prescaler => return self.APB2Prescaler_get(),
            USART1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            USART1MultConf::HSIRCDiv => return self.HSIRCDiv_get(),
            USART1MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn USART1Output_get(&self) -> Result<f32, ClockError> {
        self.USART1Mult_get()
    }
    fn I2C1Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C1Mult {
            I2C1MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            I2C1MultConf::HSIRCDiv => return self.HSIRCDiv_get(),
            I2C1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
        };
    }
    pub fn I2C1Output_get(&self) -> Result<f32, ClockError> {
        self.I2C1Mult_get()
    }
    fn I2C3Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C3Mult {
            I2C3MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            I2C3MultConf::HSIRCDiv => return self.HSIRCDiv_get(),
            I2C3MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
        };
    }
    pub fn I2C3Output_get(&self) -> Result<f32, ClockError> {
        self.I2C3Mult_get()
    }
    fn MCOMult_get(&self) -> Result<f32, ClockError> {
        match self.MCOMult {
            MCOMultConf::LSEOSC => return self.LSEOSC_get(),
            MCOMultConf::LSIRC => return self.LSIRC_get(),
            MCOMultConf::HSIRCDiv => return self.HSIRCDiv_get(),
            MCOMultConf::PLLDIV => return self.PLLDIV_get(),
            MCOMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            MCOMultConf::MSIRC => return self.MSIRC_get(),
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
    fn HSI48MUL_get(&self) -> Result<f32, ClockError> {
        match self.HSI48MUL {};
    }
    pub fn USBOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.HSI48MUL_get()?;
        if input > (48120000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48120000),
                from: ClockNodes::HSI48MUL,
                to: ClockNodes::USBOutput,
            });
        } else if input < (47880000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 47880000),
                from: ClockNodes::HSI48MUL,
                to: ClockNodes::USBOutput,
            });
        }
        Ok(input)
    }
    pub fn RNGOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.HSI48MUL_get()?;
        if input > (48000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48000000),
                from: ClockNodes::HSI48MUL,
                to: ClockNodes::RNGOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::HSI48MUL,
                to: ClockNodes::RNGOutput,
            });
        }
        Ok(input)
    }
    pub fn ADCOutput_get(&self) -> Result<f32, ClockError> {
        self.HSIRCDiv_get()
    }
    pub fn VCOIIuput_get(&self) -> Result<f32, ClockError> {
        self.PLLSource_get()
    }
    fn PLLMUL_get(&self) -> Result<f32, ClockError> {
        let input = self.VCOIIuput_get()? as f32;
        let value = self.PLLMUL.get()? as f32;
        Ok((input * value) as f32)
    }

    fn PLLDIV_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLMUL_get()? as f32;
        let value = self.PLLDIV.get()? as f32;
        Ok((input / value) as f32)
    }
}
