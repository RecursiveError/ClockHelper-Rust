#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ClockNodes {
    None,
    HSIRC,
    FLITFCLKoutput,
    HSIDivPLL,
    LSIRC,
    LSEOSC,
    HSEOSC,
    Prediv2,
    Prediv2output,
    PLL2Mul,
    PLL2VCOMul2,
    PLL2VCOoutput,
    PLL2CLKoutput,
    PLL3Mul,
    PLL3VCOMul2,
    PLL3VCOoutput,
    PLL3CLKoutput,
    SysClkSource,
    SysCLKOutput,
    I2S2Mult,
    I2S2Output,
    I2S3Mult,
    I2S3Output,
    HSERTCDevisor,
    RTCClkSource,
    RTCOutput,
    IWDGOutput,
    MCOPLL3Div,
    MCOMultDivisor,
    MCOMult,
    MCOoutput,
    AHBPrescaler,
    AHBOutput,
    HCLKOutput,
    FCLKCortexOutput,
    TimSysPresc,
    TimSysOutput,
    APB1Prescaler,
    APB1Output,
    TimPrescalerAPB1,
    TimPrescOut1,
    APB2Prescaler,
    APB2Output,
    TimPrescalerAPB2,
    TimPrescOut2,
    ADCprescaler,
    ADCoutput,
    Prediv1Source,
    PreDiv1,
    PLLSource,
    VCO2output,
    PLLMUL,
    PLLVCOMul2,
    USBPrescaler,
    USBoutput,
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
pub enum LSEOSCConf {
    Value(u32),
}

impl LSEOSCConf {
    pub const fn min() -> u32 {
        0
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
pub enum HSEOSCConf {
    Value(u32),
}

impl HSEOSCConf {
    pub const fn min() -> u32 {
        3000000
    }

    pub const fn max() -> u32 {
        25000000
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
pub enum Prediv2Conf {
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

impl Prediv2Conf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            Prediv2Conf::DIV1 => return Ok(1.0),
            Prediv2Conf::DIV2 => return Ok(2.0),
            Prediv2Conf::DIV3 => return Ok(3.0),
            Prediv2Conf::DIV4 => return Ok(4.0),
            Prediv2Conf::DIV5 => return Ok(5.0),
            Prediv2Conf::DIV6 => return Ok(6.0),
            Prediv2Conf::DIV7 => return Ok(7.0),
            Prediv2Conf::DIV8 => return Ok(8.0),
            Prediv2Conf::DIV9 => return Ok(9.0),
            Prediv2Conf::DIV10 => return Ok(10.0),
            Prediv2Conf::DIV11 => return Ok(11.0),
            Prediv2Conf::DIV12 => return Ok(12.0),
            Prediv2Conf::DIV13 => return Ok(13.0),
            Prediv2Conf::DIV14 => return Ok(14.0),
            Prediv2Conf::DIV15 => return Ok(15.0),
            Prediv2Conf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL2MulConf {
    MUL8,
    MUL9,
    MUL10,
    MUL11,
    MUL12,
    MUL13,
    MUL14,
    MUL16,
    MUL20,
}

impl PLL2MulConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL2MulConf::MUL8 => return Ok(8.0),
            PLL2MulConf::MUL9 => return Ok(9.0),
            PLL2MulConf::MUL10 => return Ok(10.0),
            PLL2MulConf::MUL11 => return Ok(11.0),
            PLL2MulConf::MUL12 => return Ok(12.0),
            PLL2MulConf::MUL13 => return Ok(13.0),
            PLL2MulConf::MUL14 => return Ok(14.0),
            PLL2MulConf::MUL16 => return Ok(16.0),
            PLL2MulConf::MUL20 => return Ok(20.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLL3MulConf {
    MUL8,
    MUL9,
    MUL10,
    MUL11,
    MUL12,
    MUL13,
    MUL14,
    MUL16,
    MUL20,
}

impl PLL3MulConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLL3MulConf::MUL8 => return Ok(8.0),
            PLL3MulConf::MUL9 => return Ok(9.0),
            PLL3MulConf::MUL10 => return Ok(10.0),
            PLL3MulConf::MUL11 => return Ok(11.0),
            PLL3MulConf::MUL12 => return Ok(12.0),
            PLL3MulConf::MUL13 => return Ok(13.0),
            PLL3MulConf::MUL14 => return Ok(14.0),
            PLL3MulConf::MUL16 => return Ok(16.0),
            PLL3MulConf::MUL20 => return Ok(20.0),
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
pub enum I2S2MultConf {
    SysCLKOutput,
    PLL3VCOoutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2S3MultConf {
    SysCLKOutput,
    PLL3VCOoutput,
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
pub enum MCOPLL3DivConf {
    DIV1,
    DIV2,
}

impl MCOPLL3DivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MCOPLL3DivConf::DIV1 => return Ok(1.0),
            MCOPLL3DivConf::DIV2 => return Ok(2.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCOMultConf {
    HSEOSC,
    HSIRC,
    SysCLKOutput,
    MCOMultDivisor,
    PLL2CLKoutput,
    MCOPLL3Div,
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
pub enum TimSysPrescConf {
    DIV1,
    DIV8,
}

impl TimSysPrescConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            TimSysPrescConf::DIV1 => return Ok(1.0),
            TimSysPrescConf::DIV8 => return Ok(8.0),
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
pub enum ADCprescalerConf {
    DIV2,
    DIV4,
    DIV6,
    DIV8,
}

impl ADCprescalerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            ADCprescalerConf::DIV2 => return Ok(2.0),
            ADCprescalerConf::DIV4 => return Ok(4.0),
            ADCprescalerConf::DIV6 => return Ok(6.0),
            ADCprescalerConf::DIV8 => return Ok(8.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum Prediv1SourceConf {
    HSEOSC,
    PLL2CLKoutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PreDiv1Conf {
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

impl PreDiv1Conf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PreDiv1Conf::DIV1 => return Ok(1.0),
            PreDiv1Conf::DIV2 => return Ok(2.0),
            PreDiv1Conf::DIV3 => return Ok(3.0),
            PreDiv1Conf::DIV4 => return Ok(4.0),
            PreDiv1Conf::DIV5 => return Ok(5.0),
            PreDiv1Conf::DIV6 => return Ok(6.0),
            PreDiv1Conf::DIV7 => return Ok(7.0),
            PreDiv1Conf::DIV8 => return Ok(8.0),
            PreDiv1Conf::DIV9 => return Ok(9.0),
            PreDiv1Conf::DIV10 => return Ok(10.0),
            PreDiv1Conf::DIV11 => return Ok(11.0),
            PreDiv1Conf::DIV12 => return Ok(12.0),
            PreDiv1Conf::DIV13 => return Ok(13.0),
            PreDiv1Conf::DIV14 => return Ok(14.0),
            PreDiv1Conf::DIV15 => return Ok(15.0),
            PreDiv1Conf::DIV16 => return Ok(16.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLSourceConf {
    HSIDivPLL,
    PreDiv1,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum PLLMULConf {
    MUL4,
    MUL5,
    MUL6,
    MUL6_5,
    MUL7,
    MUL8,
    MUL9,
}

impl PLLMULConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            PLLMULConf::MUL4 => return Ok(4.0),
            PLLMULConf::MUL5 => return Ok(5.0),
            PLLMULConf::MUL6 => return Ok(6.0),
            PLLMULConf::MUL6_5 => return Ok(6.5),
            PLLMULConf::MUL7 => return Ok(7.0),
            PLLMULConf::MUL8 => return Ok(8.0),
            PLLMULConf::MUL9 => return Ok(9.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USBPrescalerConf {
    DIV2,
    DIV3,
}

impl USBPrescalerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            USBPrescalerConf::DIV2 => return Ok(2.0),
            USBPrescalerConf::DIV3 => return Ok(3.0),
        }
    }
}
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ClockTree {
    pub LSEOSC: LSEOSCConf,
    pub HSEOSC: HSEOSCConf,
    pub Prediv2: Prediv2Conf,
    pub PLL2Mul: PLL2MulConf,
    pub PLL3Mul: PLL3MulConf,
    pub SysClkSource: SysClkSourceConf,
    pub I2S2Mult: I2S2MultConf,
    pub I2S3Mult: I2S3MultConf,
    pub RTCClkSource: RTCClkSourceConf,
    pub MCOPLL3Div: MCOPLL3DivConf,
    pub MCOMult: MCOMultConf,
    pub AHBPrescaler: AHBPrescalerConf,
    pub TimSysPresc: TimSysPrescConf,
    pub APB1Prescaler: APB1PrescalerConf,
    pub APB2Prescaler: APB2PrescalerConf,
    pub ADCprescaler: ADCprescalerConf,
    pub Prediv1Source: Prediv1SourceConf,
    pub PreDiv1: PreDiv1Conf,
    pub PLLSource: PLLSourceConf,
    pub PLLMUL: PLLMULConf,
    pub USBPrescaler: USBPrescalerConf,
}

impl Default for ClockTree {
    fn default() -> Self {
        Self {
            LSEOSC: LSEOSCConf::Value(32768),
            HSEOSC: HSEOSCConf::Value(8000000),
            Prediv2: Prediv2Conf::DIV1,
            PLL2Mul: PLL2MulConf::MUL8,
            PLL3Mul: PLL3MulConf::MUL8,
            SysClkSource: SysClkSourceConf::HSIRC,
            I2S2Mult: I2S2MultConf::SysCLKOutput,
            I2S3Mult: I2S3MultConf::SysCLKOutput,
            RTCClkSource: RTCClkSourceConf::LSIRC,
            MCOPLL3Div: MCOPLL3DivConf::DIV1,
            MCOMult: MCOMultConf::SysCLKOutput,
            AHBPrescaler: AHBPrescalerConf::DIV1,
            TimSysPresc: TimSysPrescConf::DIV1,
            APB1Prescaler: APB1PrescalerConf::DIV1,
            APB2Prescaler: APB2PrescalerConf::DIV1,
            ADCprescaler: ADCprescalerConf::DIV2,
            Prediv1Source: Prediv1SourceConf::HSEOSC,
            PreDiv1: PreDiv1Conf::DIV1,
            PLLSource: PLLSourceConf::HSIDivPLL,
            PLLMUL: PLLMULConf::MUL4,
            USBPrescaler: USBPrescalerConf::DIV3,
        }
    }
}
impl ClockTree {
    pub fn HSIRC_get(&self) -> Result<f32, ClockError> {
        Ok(8000000 as f32)
    }
    pub fn FLITFCLKoutput_get(&self) -> Result<f32, ClockError> {
        self.HSIRC_get()
    }
    fn HSIDivPLL_get(&self) -> Result<f32, ClockError> {
        let input = self.HSIRC_get()? as f32;
        let value = 2 as f32;
        Ok((input / value) as f32)
    }

    pub fn LSIRC_get(&self) -> Result<f32, ClockError> {
        Ok(40000 as f32)
    }
    pub fn LSEOSC_get(&self) -> Result<f32, ClockError> {
        self.LSEOSC.get()
    }
    pub fn HSEOSC_get(&self) -> Result<f32, ClockError> {
        self.HSEOSC.get()
    }
    fn Prediv2_get(&self) -> Result<f32, ClockError> {
        let input = self.HSEOSC_get()? as f32;
        let value = self.Prediv2.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn Prediv2output_get(&self) -> Result<f32, ClockError> {
        self.Prediv2_get()
    }
    fn PLL2Mul_get(&self) -> Result<f32, ClockError> {
        let input = self.Prediv2output_get()? as f32;
        let value = self.PLL2Mul.get()? as f32;
        Ok((input * value) as f32)
    }

    fn PLL2VCOMul2_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL2Mul_get()? as f32;
        let value = 2 as f32;
        Ok((input * value) as f32)
    }

    pub fn PLL2VCOoutput_get(&self) -> Result<f32, ClockError> {
        self.PLL2VCOMul2_get()
    }
    pub fn PLL2CLKoutput_get(&self) -> Result<f32, ClockError> {
        self.PLL2Mul_get()
    }
    fn PLL3Mul_get(&self) -> Result<f32, ClockError> {
        let input = self.Prediv2output_get()? as f32;
        let value = self.PLL3Mul.get()? as f32;
        Ok((input * value) as f32)
    }

    fn PLL3VCOMul2_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL3Mul_get()? as f32;
        let value = 2 as f32;
        Ok((input * value) as f32)
    }

    pub fn PLL3VCOoutput_get(&self) -> Result<f32, ClockError> {
        self.PLL3VCOMul2_get()
    }
    pub fn PLL3CLKoutput_get(&self) -> Result<f32, ClockError> {
        self.PLL3Mul_get()
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
    fn I2S2Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2S2Mult {
            I2S2MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            I2S2MultConf::PLL3VCOoutput => return self.PLL3VCOoutput_get(),
        };
    }
    pub fn I2S2Output_get(&self) -> Result<f32, ClockError> {
        self.I2S2Mult_get()
    }
    fn I2S3Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2S3Mult {
            I2S3MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            I2S3MultConf::PLL3VCOoutput => return self.PLL3VCOoutput_get(),
        };
    }
    pub fn I2S3Output_get(&self) -> Result<f32, ClockError> {
        self.I2S3Mult_get()
    }
    fn HSERTCDevisor_get(&self) -> Result<f32, ClockError> {
        let input = self.HSEOSC_get()? as f32;
        let value = 128 as f32;
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
    fn MCOPLL3Div_get(&self) -> Result<f32, ClockError> {
        let input = self.PLL3CLKoutput_get()? as f32;
        let value = self.MCOPLL3Div.get()? as f32;
        Ok((input / value) as f32)
    }

    fn MCOMultDivisor_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLMUL_get()? as f32;
        let value = 2 as f32;
        Ok((input / value) as f32)
    }

    fn MCOMult_get(&self) -> Result<f32, ClockError> {
        match self.MCOMult {
            MCOMultConf::HSEOSC => return self.HSEOSC_get(),
            MCOMultConf::HSIRC => return self.HSIRC_get(),
            MCOMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            MCOMultConf::MCOMultDivisor => return self.MCOMultDivisor_get(),
            MCOMultConf::PLL2CLKoutput => return self.PLL2CLKoutput_get(),
            MCOMultConf::MCOPLL3Div => return self.MCOPLL3Div_get(),
        };
    }
    pub fn MCOoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.MCOMult_get()?;
        if input > (50000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 50000000),
                from: ClockNodes::MCOMult,
                to: ClockNodes::MCOoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::MCOMult,
                to: ClockNodes::MCOoutput,
            });
        }
        Ok(input)
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
    fn TimSysPresc_get(&self) -> Result<f32, ClockError> {
        let input = self.AHBOutput_get()? as f32;
        let value = self.TimSysPresc.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn TimSysOutput_get(&self) -> Result<f32, ClockError> {
        self.TimSysPresc_get()
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
    fn ADCprescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.APB2Prescaler_get()? as f32;
        let value = self.ADCprescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn ADCoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.ADCprescaler_get()?;
        if input > (14000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 14000000),
                from: ClockNodes::ADCprescaler,
                to: ClockNodes::ADCoutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::ADCprescaler,
                to: ClockNodes::ADCoutput,
            });
        }
        Ok(input)
    }
    fn Prediv1Source_get(&self) -> Result<f32, ClockError> {
        match self.Prediv1Source {
            Prediv1SourceConf::HSEOSC => return self.HSEOSC_get(),
            Prediv1SourceConf::PLL2CLKoutput => return self.PLL2CLKoutput_get(),
        };
    }
    fn PreDiv1_get(&self) -> Result<f32, ClockError> {
        let input = self.Prediv1Source_get()? as f32;
        let value = self.PreDiv1.get()? as f32;
        Ok((input / value) as f32)
    }

    fn PLLSource_get(&self) -> Result<f32, ClockError> {
        match self.PLLSource {
            PLLSourceConf::HSIDivPLL => return self.HSIDivPLL_get(),
            PLLSourceConf::PreDiv1 => return self.PreDiv1_get(),
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

    fn PLLVCOMul2_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLMUL_get()? as f32;
        let value = 2 as f32;
        Ok((input * value) as f32)
    }

    fn USBPrescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.PLLVCOMul2_get()? as f32;
        let value = self.USBPrescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn USBoutput_get(&self) -> Result<f32, ClockError> {
        let input = self.USBPrescaler_get()?;
        if input > (48120000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 48120000),
                from: ClockNodes::USBPrescaler,
                to: ClockNodes::USBoutput,
            });
        } else if input < (47880000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 47880000),
                from: ClockNodes::USBPrescaler,
                to: ClockNodes::USBoutput,
            });
        }
        Ok(input)
    }
}
