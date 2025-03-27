#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ClockNodes {
    None,
    HSIRC,
    HSIRCDiv,
    FLITFCLKoutput,
    LSIRC,
    HSEOSC,
    HSEPLLsourceDevisor,
    PRESCALERUSB,
    USBoutput,
    SysClkSource,
    SysCLKOutput,
    HSERTCDevisor,
    RTCClkSource,
    RTCOutput,
    IWDGOutput,
    MCOMultDivisor,
    MCOMult,
    MCODivisor,
    MCOoutput,
    AHBPrescaler,
    AHBOutput,
    HCLKOutput,
    FCLKCortexOutput,
    CortexPrescaler,
    CortexSysOutput,
    ADC12PRES,
    ADC12output,
    ADC34PRES,
    ADC34output,
    APB1Prescaler,
    APB1Output,
    TimPrescalerAPB1,
    TimPrescOut1,
    APB2Prescaler,
    APB2Output,
    TimPrescalerAPB2,
    TimPrescOut2,
    TIMMUL,
    TIMMUX1,
    TIM1out,
    TIMMUX8,
    TIM8out,
    TIMMUX15,
    TIM15out,
    TIMMUX16,
    TIM16out,
    TIMMUX17,
    TIM17out,
    HRTIMMux,
    HRTIMout,
    I2C1Mult,
    I2C1Output,
    I2C2Mult,
    I2C2Output,
    I2C3Mult,
    I2C3Output,
    I2S_CKIN,
    I2SSrc,
    I2SClocksOutput,
    USART1Mult,
    USART1Output,
    UART4Mult,
    UART4Output,
    UART5Mult,
    UART5Output,
    PLLSource,
    VCO2output,
    PLLMUL,
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
pub enum HSEPLLsourceDevisorConf {
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

impl HSEPLLsourceDevisorConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            HSEPLLsourceDevisorConf::DIV1 => return Ok(1.0),
            HSEPLLsourceDevisorConf::DIV2 => return Ok(2.0),
            HSEPLLsourceDevisorConf::DIV3 => return Ok(3.0),
            HSEPLLsourceDevisorConf::DIV4 => return Ok(4.0),
            HSEPLLsourceDevisorConf::DIV5 => return Ok(5.0),
            HSEPLLsourceDevisorConf::DIV6 => return Ok(6.0),
            HSEPLLsourceDevisorConf::DIV7 => return Ok(7.0),
            HSEPLLsourceDevisorConf::DIV8 => return Ok(8.0),
            HSEPLLsourceDevisorConf::DIV9 => return Ok(9.0),
            HSEPLLsourceDevisorConf::DIV10 => return Ok(10.0),
            HSEPLLsourceDevisorConf::DIV11 => return Ok(11.0),
            HSEPLLsourceDevisorConf::DIV12 => return Ok(12.0),
            HSEPLLsourceDevisorConf::DIV13 => return Ok(13.0),
            HSEPLLsourceDevisorConf::DIV14 => return Ok(14.0),
            HSEPLLsourceDevisorConf::DIV15 => return Ok(15.0),
            HSEPLLsourceDevisorConf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PRESCALERUSBConf {
    DIV1,
    DIV1_5,
}

impl PRESCALERUSBConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PRESCALERUSBConf::DIV1 => return Ok(1.0),
            PRESCALERUSBConf::DIV1_5 => return Ok(1.5),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SysClkSourceConf {
    HSIRC,
    HSEOSC,
    PLLMUL,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum RTCClkSourceConf {
    HSERTCDevisor,
    LSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCOMultDivisorConf {
    DIV1,
    DIV2,
}

impl MCOMultDivisorConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MCOMultDivisorConf::DIV1 => return Ok(1.0),
            MCOMultDivisorConf::DIV2 => return Ok(2.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCOMultConf {
    HSIRC,
    HSEOSC,
    MCOMultDivisor,
    LSIRC,
    SysCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCODivisorConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
    DIV32,
    DIV64,
    DIV128,
}

impl MCODivisorConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MCODivisorConf::DIV1 => return Ok(1.0),
            MCODivisorConf::DIV2 => return Ok(2.0),
            MCODivisorConf::DIV4 => return Ok(4.0),
            MCODivisorConf::DIV8 => return Ok(8.0),
            MCODivisorConf::DIV16 => return Ok(16.0),
            MCODivisorConf::DIV32 => return Ok(32.0),
            MCODivisorConf::DIV64 => return Ok(64.0),
            MCODivisorConf::DIV128 => return Ok(128.0),
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
pub enum ADC12PRESConf {
    DIV1,
    DIV2,
    DIV4,
    DIV6,
    DIV8,
    DIV10,
    DIV12,
    DIV16,
    DIV32,
    DIV64,
    DIV128,
    DIV256,
}

impl ADC12PRESConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            ADC12PRESConf::DIV1 => return Ok(1.0),
            ADC12PRESConf::DIV2 => return Ok(2.0),
            ADC12PRESConf::DIV4 => return Ok(4.0),
            ADC12PRESConf::DIV6 => return Ok(6.0),
            ADC12PRESConf::DIV8 => return Ok(8.0),
            ADC12PRESConf::DIV10 => return Ok(10.0),
            ADC12PRESConf::DIV12 => return Ok(12.0),
            ADC12PRESConf::DIV16 => return Ok(16.0),
            ADC12PRESConf::DIV32 => return Ok(32.0),
            ADC12PRESConf::DIV64 => return Ok(64.0),
            ADC12PRESConf::DIV128 => return Ok(128.0),
            ADC12PRESConf::DIV256 => return Ok(256.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ADC34PRESConf {
    DIV1,
    DIV2,
    DIV4,
    DIV6,
    DIV8,
    DIV10,
    DIV12,
    DIV16,
    DIV32,
    DIV64,
    DIV128,
    DIV256,
}

impl ADC34PRESConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            ADC34PRESConf::DIV1 => return Ok(1.0),
            ADC34PRESConf::DIV2 => return Ok(2.0),
            ADC34PRESConf::DIV4 => return Ok(4.0),
            ADC34PRESConf::DIV6 => return Ok(6.0),
            ADC34PRESConf::DIV8 => return Ok(8.0),
            ADC34PRESConf::DIV10 => return Ok(10.0),
            ADC34PRESConf::DIV12 => return Ok(12.0),
            ADC34PRESConf::DIV16 => return Ok(16.0),
            ADC34PRESConf::DIV32 => return Ok(32.0),
            ADC34PRESConf::DIV64 => return Ok(64.0),
            ADC34PRESConf::DIV128 => return Ok(128.0),
            ADC34PRESConf::DIV256 => return Ok(256.0),
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
pub enum TIMMUX1Conf {
    TIMMUL,
    TimPrescOut2,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum TIMMUX8Conf {
    TIMMUL,
    TimPrescOut2,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum TIMMUX15Conf {
    TIMMUL,
    TimPrescOut2,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum TIMMUX16Conf {
    TIMMUL,
    TimPrescOut2,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum TIMMUX17Conf {
    TIMMUL,
    TimPrescOut2,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum HRTIMMuxConf {
    TIMMUL,
    TimPrescOut2,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C1MultConf {
    HSIRC,
    SysCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C2MultConf {
    HSIRC,
    SysCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C3MultConf {
    HSIRC,
    SysCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2SSrcConf {
    I2S_CKIN,
    SysCLKOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART1MultConf {
    SysCLKOutput,
    HSIRC,
    APB1Prescaler,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum UART4MultConf {
    SysCLKOutput,
    HSIRC,
    APB1Prescaler,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum UART5MultConf {
    SysCLKOutput,
    HSIRC,
    APB1Prescaler,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSourceConf {
    HSIRCDiv,
    HSEPLLsourceDevisor,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLMULConf {
    MUL2,
    MUL3,
    MUL4,
    MUL5,
    MUL6,
    MUL7,
    MUL8,
    MUL9,
    MUL10,
    MUL11,
    MUL12,
    MUL13,
    MUL14,
    MUL15,
    MUL16,
}

impl PLLMULConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLMULConf::MUL2 => return Ok(2.0),
            PLLMULConf::MUL3 => return Ok(3.0),
            PLLMULConf::MUL4 => return Ok(4.0),
            PLLMULConf::MUL5 => return Ok(5.0),
            PLLMULConf::MUL6 => return Ok(6.0),
            PLLMULConf::MUL7 => return Ok(7.0),
            PLLMULConf::MUL8 => return Ok(8.0),
            PLLMULConf::MUL9 => return Ok(9.0),
            PLLMULConf::MUL10 => return Ok(10.0),
            PLLMULConf::MUL11 => return Ok(11.0),
            PLLMULConf::MUL12 => return Ok(12.0),
            PLLMULConf::MUL13 => return Ok(13.0),
            PLLMULConf::MUL14 => return Ok(14.0),
            PLLMULConf::MUL15 => return Ok(15.0),
            PLLMULConf::MUL16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ClockTree {
    pub HSEOSC: HSEOSCConf,
    pub HSEPLLsourceDevisor: HSEPLLsourceDevisorConf,
    pub PRESCALERUSB: PRESCALERUSBConf,
    pub SysClkSource: SysClkSourceConf,
    pub RTCClkSource: RTCClkSourceConf,
    pub MCOMultDivisor: MCOMultDivisorConf,
    pub MCOMult: MCOMultConf,
    pub MCODivisor: MCODivisorConf,
    pub AHBPrescaler: AHBPrescalerConf,
    pub CortexPrescaler: CortexPrescalerConf,
    pub ADC12PRES: ADC12PRESConf,
    pub ADC34PRES: ADC34PRESConf,
    pub APB1Prescaler: APB1PrescalerConf,
    pub APB2Prescaler: APB2PrescalerConf,
    pub TIMMUX1: TIMMUX1Conf,
    pub TIMMUX8: TIMMUX8Conf,
    pub TIMMUX15: TIMMUX15Conf,
    pub TIMMUX16: TIMMUX16Conf,
    pub TIMMUX17: TIMMUX17Conf,
    pub HRTIMMux: HRTIMMuxConf,
    pub I2C1Mult: I2C1MultConf,
    pub I2C2Mult: I2C2MultConf,
    pub I2C3Mult: I2C3MultConf,
    pub I2SSrc: I2SSrcConf,
    pub USART1Mult: USART1MultConf,
    pub UART4Mult: UART4MultConf,
    pub UART5Mult: UART5MultConf,
    pub PLLSource: PLLSourceConf,
    pub PLLMUL: PLLMULConf,
}

impl Default for ClockTree {
    fn default() -> Self {
        Self {
            HSEOSC: HSEOSCConf::Value(8000000),
            HSEPLLsourceDevisor: HSEPLLsourceDevisorConf::DIV1,
            PRESCALERUSB: PRESCALERUSBConf::DIV1,
            SysClkSource: SysClkSourceConf::HSIRC,
            RTCClkSource: RTCClkSourceConf::LSIRC,
            MCOMultDivisor: MCOMultDivisorConf::DIV1,
            MCOMult: MCOMultConf::SysCLKOutput,
            MCODivisor: MCODivisorConf::DIV1,
            AHBPrescaler: AHBPrescalerConf::DIV1,
            CortexPrescaler: CortexPrescalerConf::DIV1,
            ADC12PRES: ADC12PRESConf::DIV1,
            ADC34PRES: ADC34PRESConf::DIV1,
            APB1Prescaler: APB1PrescalerConf::DIV1,
            APB2Prescaler: APB2PrescalerConf::DIV1,
            TIMMUX1: TIMMUX1Conf::TimPrescOut2,
            TIMMUX8: TIMMUX8Conf::TimPrescOut2,
            TIMMUX15: TIMMUX15Conf::TimPrescOut2,
            TIMMUX16: TIMMUX16Conf::TimPrescOut2,
            TIMMUX17: TIMMUX17Conf::TimPrescOut2,
            HRTIMMux: HRTIMMuxConf::TimPrescOut2,
            I2C1Mult: I2C1MultConf::HSIRC,
            I2C2Mult: I2C2MultConf::HSIRC,
            I2C3Mult: I2C3MultConf::HSIRC,
            I2SSrc: I2SSrcConf::SysCLKOutput,
            USART1Mult: USART1MultConf::SysCLKOutput,
            UART4Mult: UART4MultConf::APB1Prescaler,
            UART5Mult: UART5MultConf::APB1Prescaler,
            PLLSource: PLLSourceConf::HSIRCDiv,
            PLLMUL: PLLMULConf::MUL2,
        }
    }
}
impl ClockTree {
    pub fn HSIRC_get(&self) -> Result<f32, ClockError> {
        Ok(8000000 as f32)
    }
    fn HSIRCDiv_get(&self) -> Result<f32, ClockError> {
        let input = self.HSIRC_get()? as f32;
        let value = 2 as f32;
        Ok((input / value) as f32)
    }

    pub fn FLITFCLKoutput_get(&self) -> Result<f32, ClockError> {
        self.HSIRC_get()
    }
    pub fn LSIRC_get(&self) -> Result<f32, ClockError> {
        Ok(40000 as f32)
    }
    pub fn HSEOSC_get(&self) -> Result<f32, ClockError> {
        self.HSEOSC.get()
    }
    fn HSEPLLsourceDevisor_get(&self) -> Result<f32, ClockError> {
        let input = self.HSEOSC_get()? as f32;
        let value = self.HSEPLLsourceDevisor.get()? as f32;
        Ok((input / value) as f32)
    }

    fn PRESCALERUSB_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLMUL_get()? as f32;
        let value = self.PRESCALERUSB.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn USBoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.PRESCALERUSB_get()?;
        if input > (48120000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48120000),
                from: ClockNodes::PRESCALERUSB,
                to: ClockNodes::USBoutput,
            });
        } else if input < (47880000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 47880000),
                from: ClockNodes::PRESCALERUSB,
                to: ClockNodes::USBoutput,
            });
        }
        Ok(input)
    }
    fn SysClkSource_get(&self) -> Result<f32, ClockError> {
        match self.SysClkSource {
            SysClkSourceConf::HSIRC => return self.HSIRC_get(),
            SysClkSourceConf::HSEOSC => return self.HSEOSC_get(),
            SysClkSourceConf::PLLMUL => return self.PLLMUL_get(),
        };
    }
    pub fn SysCLKOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SysClkSource_get()?;
        if input > (72000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 72000000),
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
    fn HSERTCDevisor_get(&self) -> Result<f32, ClockError> {
        let input = self.HSEOSC_get()? as f32;
        let value = 32 as f32;
        Ok((input / value) as f32)
    }

    fn RTCClkSource_get(&self) -> Result<f32, ClockError> {
        match self.RTCClkSource {
            RTCClkSourceConf::HSERTCDevisor => return self.HSERTCDevisor_get(),
            RTCClkSourceConf::LSIRC => return self.LSIRC_get(),
        };
    }
    pub fn RTCOutput_get(&self) -> Result<f32, ClockError> {
        self.RTCClkSource_get()
    }
    pub fn IWDGOutput_get(&self) -> Result<f32, ClockError> {
        self.LSIRC_get()
    }
    fn MCOMultDivisor_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLMUL_get()? as f32;
        let value = self.MCOMultDivisor.get()? as f32;
        Ok((input / value) as f32)
    }

    fn MCOMult_get(&self) -> Result<f32, ClockError> {
        match self.MCOMult {
            MCOMultConf::HSIRC => return self.HSIRC_get(),
            MCOMultConf::HSEOSC => return self.HSEOSC_get(),
            MCOMultConf::MCOMultDivisor => return self.MCOMultDivisor_get(),
            MCOMultConf::LSIRC => return self.LSIRC_get(),
            MCOMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
        };
    }
    fn MCODivisor_get(&self) -> Result<f32, ClockError> {
        let input = self.MCOMult_get()? as f32;
        let value = self.MCODivisor.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn MCOoutput_get(&self) -> Result<f32, ClockError> {
        self.MCODivisor_get()
    }
    fn AHBPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()? as f32;
        let value = self.AHBPrescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn AHBOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBPrescaler_get()?;
        if input > (72000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 72000000),
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
    pub fn FCLKCortexOutput_get(&self) -> Result<f32, ClockError> {
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
    fn ADC12PRES_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLMUL_get()? as f32;
        let value = self.ADC12PRES.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn ADC12output_get(&self) -> Result<f32, ClockError> {
        self.ADC12PRES_get()
    }
    fn ADC34PRES_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLMUL_get()? as f32;
        let value = self.ADC34PRES.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn ADC34output_get(&self) -> Result<f32, ClockError> {
        self.ADC34PRES_get()
    }
    fn APB1Prescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.APB1Prescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn APB1Output_get(&self) -> Result<f32, ClockError> {
        let input = self.APB1Prescaler_get()?;
        if input > (36000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 36000000),
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
        if input > (72000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 72000000),
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
    fn TIMMUL_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLMUL_get()? as f32;
        let value = 2 as f32;
        Ok((input * value) as f32)
    }

    fn TIMMUX1_get(&self) -> Result<f32, ClockError> {
        match self.TIMMUX1 {
            TIMMUX1Conf::TIMMUL => return self.TIMMUL_get(),
            TIMMUX1Conf::TimPrescOut2 => return self.TimPrescOut2_get(),
        };
    }
    pub fn TIM1out_get(&self) -> Result<f32, ClockError> {
        self.TIMMUX1_get()
    }
    fn TIMMUX8_get(&self) -> Result<f32, ClockError> {
        match self.TIMMUX8 {
            TIMMUX8Conf::TIMMUL => return self.TIMMUL_get(),
            TIMMUX8Conf::TimPrescOut2 => return self.TimPrescOut2_get(),
        };
    }
    pub fn TIM8out_get(&self) -> Result<f32, ClockError> {
        self.TIMMUX8_get()
    }
    fn TIMMUX15_get(&self) -> Result<f32, ClockError> {
        match self.TIMMUX15 {
            TIMMUX15Conf::TIMMUL => return self.TIMMUL_get(),
            TIMMUX15Conf::TimPrescOut2 => return self.TimPrescOut2_get(),
        };
    }
    pub fn TIM15out_get(&self) -> Result<f32, ClockError> {
        self.TIMMUX15_get()
    }
    fn TIMMUX16_get(&self) -> Result<f32, ClockError> {
        match self.TIMMUX16 {
            TIMMUX16Conf::TIMMUL => return self.TIMMUL_get(),
            TIMMUX16Conf::TimPrescOut2 => return self.TimPrescOut2_get(),
        };
    }
    pub fn TIM16out_get(&self) -> Result<f32, ClockError> {
        self.TIMMUX16_get()
    }
    fn TIMMUX17_get(&self) -> Result<f32, ClockError> {
        match self.TIMMUX17 {
            TIMMUX17Conf::TIMMUL => return self.TIMMUL_get(),
            TIMMUX17Conf::TimPrescOut2 => return self.TimPrescOut2_get(),
        };
    }
    pub fn TIM17out_get(&self) -> Result<f32, ClockError> {
        self.TIMMUX17_get()
    }
    fn HRTIMMux_get(&self) -> Result<f32, ClockError> {
        match self.HRTIMMux {
            HRTIMMuxConf::TIMMUL => return self.TIMMUL_get(),
            HRTIMMuxConf::TimPrescOut2 => return self.TimPrescOut2_get(),
        };
    }
    pub fn HRTIMout_get(&self) -> Result<f32, ClockError> {
        self.HRTIMMux_get()
    }
    fn I2C1Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C1Mult {
            I2C1MultConf::HSIRC => return self.HSIRC_get(),
            I2C1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
        };
    }
    pub fn I2C1Output_get(&self) -> Result<f32, ClockError> {
        self.I2C1Mult_get()
    }
    fn I2C2Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C2Mult {
            I2C2MultConf::HSIRC => return self.HSIRC_get(),
            I2C2MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
        };
    }
    pub fn I2C2Output_get(&self) -> Result<f32, ClockError> {
        self.I2C2Mult_get()
    }
    fn I2C3Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C3Mult {
            I2C3MultConf::HSIRC => return self.HSIRC_get(),
            I2C3MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
        };
    }
    pub fn I2C3Output_get(&self) -> Result<f32, ClockError> {
        self.I2C3Mult_get()
    }
    pub fn I2S_CKIN_get(&self) -> Result<f32, ClockError> {
        Ok(8000000 as f32)
    }
    fn I2SSrc_get(&self) -> Result<f32, ClockError> {
        match self.I2SSrc {
            I2SSrcConf::I2S_CKIN => return self.I2S_CKIN_get(),
            I2SSrcConf::SysCLKOutput => return self.SysCLKOutput_get(),
        };
    }
    pub fn I2SClocksOutput_get(&self) -> Result<f32, ClockError> {
        self.I2SSrc_get()
    }
    fn USART1Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART1Mult {
            USART1MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            USART1MultConf::HSIRC => return self.HSIRC_get(),
            USART1MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
        };
    }
    pub fn USART1Output_get(&self) -> Result<f32, ClockError> {
        self.USART1Mult_get()
    }
    fn UART4Mult_get(&self) -> Result<f32, ClockError> {
        match self.UART4Mult {
            UART4MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            UART4MultConf::HSIRC => return self.HSIRC_get(),
            UART4MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
        };
    }
    pub fn UART4Output_get(&self) -> Result<f32, ClockError> {
        self.UART4Mult_get()
    }
    fn UART5Mult_get(&self) -> Result<f32, ClockError> {
        match self.UART5Mult {
            UART5MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            UART5MultConf::HSIRC => return self.HSIRC_get(),
            UART5MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
        };
    }
    pub fn UART5Output_get(&self) -> Result<f32, ClockError> {
        self.UART5Mult_get()
    }
    fn PLLSource_get(&self) -> Result<f32, ClockError> {
        match self.PLLSource {
            PLLSourceConf::HSIRCDiv => return self.HSIRCDiv_get(),
            PLLSourceConf::HSEPLLsourceDevisor => return self.HSEPLLsourceDevisor_get(),
        };
    }
    pub fn VCO2output_get(&self) -> Result<f32, ClockError> {
        self.PLLSource_get()
    }
    fn PLLMUL_get(&self) -> Result<f32, ClockError> {
        let input = self.VCO2output_get()? as f32;
        let value = self.PLLMUL.get()? as f32;
        Ok((input * value) as f32)
    }
}
