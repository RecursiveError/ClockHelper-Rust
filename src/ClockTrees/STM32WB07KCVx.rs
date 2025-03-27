#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ClockNodes {
    None,
    HSIRC,
    PLL64RC,
    HSEOSC,
    LSEOSC,
    LSIRC,
    RC64MPLL,
    SysClkSource,
    SysCLKOutput,
    TimerOutput,
    CLK16MHzDiv2,
    CLK16MHzDiv4,
    CLK16MHzSource,
    Clk16MHzOutput,
    ClkSMPSDiv4,
    ClkSMPSDiv2,
    ClkSMPS,
    ClkSMPSOutput,
    LSCOMult,
    LSCOOutput,
    CLK32MHzDiv1,
    CLK32MHzDiv2,
    CLK32MHzSource,
    Clk32MHzOutput,
    BLEMult,
    BLEOutput,
    SYSCLK32Prescaler,
    SYSCLK64Prescaler,
    CLKSYSMult,
    CLKSYSOutput,
    CLKI2S3Mult,
    CLKI2S3Output,
    CLKI2S2Output,
    CLKI2S2Mult,
    CLK16RTCDevisor,
    RTCClkSource,
    RTCOutput,
    MCOMult,
    MCODiv,
    MCOPin,
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
pub enum RC64MPLLConf {
    HSIRC,
    PLL64RC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SysClkSourceConf {
    HSEOSC,
    RC64MPLL,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CLK16MHzSourceConf {
    CLK16MHzDiv2,
    CLK16MHzDiv4,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ClkSMPSConf {
    ClkSMPSDiv4,
    ClkSMPSDiv2,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LSCOMultConf {
    LSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CLK32MHzSourceConf {
    CLK32MHzDiv1,
    CLK32MHzDiv2,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum BLEMultConf {
    Clk16MHzOutput,
    Clk32MHzOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SYSCLK32PrescalerConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
    DIV32,
}

impl SYSCLK32PrescalerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            SYSCLK32PrescalerConf::DIV1 => return Ok(1.0),
            SYSCLK32PrescalerConf::DIV2 => return Ok(2.0),
            SYSCLK32PrescalerConf::DIV4 => return Ok(4.0),
            SYSCLK32PrescalerConf::DIV8 => return Ok(8.0),
            SYSCLK32PrescalerConf::DIV16 => return Ok(16.0),
            SYSCLK32PrescalerConf::DIV32 => return Ok(32.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SYSCLK64PrescalerConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
    DIV32,
    DIV64,
}

impl SYSCLK64PrescalerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            SYSCLK64PrescalerConf::DIV1 => return Ok(1.0),
            SYSCLK64PrescalerConf::DIV2 => return Ok(2.0),
            SYSCLK64PrescalerConf::DIV4 => return Ok(4.0),
            SYSCLK64PrescalerConf::DIV8 => return Ok(8.0),
            SYSCLK64PrescalerConf::DIV16 => return Ok(16.0),
            SYSCLK64PrescalerConf::DIV32 => return Ok(32.0),
            SYSCLK64PrescalerConf::DIV64 => return Ok(64.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CLKSYSMultConf {
    SYSCLK32Prescaler,
    SYSCLK64Prescaler,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CLKI2S3MultConf {
    Clk32MHzOutput,
    Clk16MHzOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum CLKI2S2MultConf {
    Clk32MHzOutput,
    Clk16MHzOutput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum RTCClkSourceConf {
    CLK16RTCDevisor,
    LSEOSC,
    LSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCOMultConf {
    Clk16MHzOutput,
    ClkSMPSOutput,
    CLKSYSOutput,
    HSEOSC,
    HSIRC,
    CLK16RTCDevisor,
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
        }
    }
}
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ClockTree {
    pub HSEOSC: HSEOSCConf,
    pub LSEOSC: LSEOSCConf,
    pub RC64MPLL: RC64MPLLConf,
    pub SysClkSource: SysClkSourceConf,
    pub CLK16MHzSource: CLK16MHzSourceConf,
    pub ClkSMPS: ClkSMPSConf,
    pub LSCOMult: LSCOMultConf,
    pub CLK32MHzSource: CLK32MHzSourceConf,
    pub BLEMult: BLEMultConf,
    pub SYSCLK32Prescaler: SYSCLK32PrescalerConf,
    pub SYSCLK64Prescaler: SYSCLK64PrescalerConf,
    pub CLKSYSMult: CLKSYSMultConf,
    pub CLKI2S3Mult: CLKI2S3MultConf,
    pub CLKI2S2Mult: CLKI2S2MultConf,
    pub RTCClkSource: RTCClkSourceConf,
    pub MCOMult: MCOMultConf,
    pub MCODiv: MCODivConf,
}

impl Default for ClockTree {
    fn default() -> Self {
        Self {
            HSEOSC: HSEOSCConf::Value(32000000),
            LSEOSC: LSEOSCConf::Value(32768),
            RC64MPLL: RC64MPLLConf::HSIRC,
            SysClkSource: SysClkSourceConf::RC64MPLL,
            CLK16MHzSource: CLK16MHzSourceConf::CLK16MHzDiv4,
            ClkSMPS: ClkSMPSConf::ClkSMPSDiv4,
            LSCOMult: LSCOMultConf::LSIRC,
            CLK32MHzSource: CLK32MHzSourceConf::CLK32MHzDiv2,
            BLEMult: BLEMultConf::Clk32MHzOutput,
            SYSCLK32Prescaler: SYSCLK32PrescalerConf::DIV1,
            SYSCLK64Prescaler: SYSCLK64PrescalerConf::DIV1,
            CLKSYSMult: CLKSYSMultConf::SYSCLK64Prescaler,
            CLKI2S3Mult: CLKI2S3MultConf::Clk32MHzOutput,
            CLKI2S2Mult: CLKI2S2MultConf::Clk32MHzOutput,
            RTCClkSource: RTCClkSourceConf::LSIRC,
            MCOMult: MCOMultConf::CLKSYSOutput,
            MCODiv: MCODivConf::DIV1,
        }
    }
}
impl ClockTree {
    pub fn HSIRC_get(&self) -> Result<f32, ClockError> {
        Ok(64000000 as f32)
    }
    pub fn PLL64RC_get(&self) -> Result<f32, ClockError> {
        Ok(64000000 as f32)
    }
    pub fn HSEOSC_get(&self) -> Result<f32, ClockError> {
        self.HSEOSC.get()
    }
    pub fn LSEOSC_get(&self) -> Result<f32, ClockError> {
        self.LSEOSC.get()
    }
    pub fn LSIRC_get(&self) -> Result<f32, ClockError> {
        Ok(32000 as f32)
    }
    fn RC64MPLL_get(&self) -> Result<f32, ClockError> {
        match self.RC64MPLL {
            RC64MPLLConf::HSIRC => return self.HSIRC_get(),
            RC64MPLLConf::PLL64RC => return self.PLL64RC_get(),
        };
    }
    fn SysClkSource_get(&self) -> Result<f32, ClockError> {
        match self.SysClkSource {
            SysClkSourceConf::HSEOSC => return self.HSEOSC_get(),
            SysClkSourceConf::RC64MPLL => return self.RC64MPLL_get(),
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
    pub fn TimerOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()?;
        if input > (64000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 64000000),
                from: ClockNodes::SysCLKOutput,
                to: ClockNodes::TimerOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::SysCLKOutput,
                to: ClockNodes::TimerOutput,
            });
        }
        Ok(input)
    }
    fn CLK16MHzDiv2_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()? as f32;
        let value = 2 as f32;
        Ok((input / value) as f32)
    }

    fn CLK16MHzDiv4_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()? as f32;
        let value = 4 as f32;
        Ok((input / value) as f32)
    }

    fn CLK16MHzSource_get(&self) -> Result<f32, ClockError> {
        match self.CLK16MHzSource {
            CLK16MHzSourceConf::CLK16MHzDiv2 => return self.CLK16MHzDiv2_get(),
            CLK16MHzSourceConf::CLK16MHzDiv4 => return self.CLK16MHzDiv4_get(),
        };
    }
    pub fn Clk16MHzOutput_get(&self) -> Result<f32, ClockError> {
        self.CLK16MHzSource_get()
    }
    fn ClkSMPSDiv4_get(&self) -> Result<f32, ClockError> {
        let input = self.Clk16MHzOutput_get()? as f32;
        let value = 4 as f32;
        Ok((input / value) as f32)
    }

    fn ClkSMPSDiv2_get(&self) -> Result<f32, ClockError> {
        let input = self.Clk16MHzOutput_get()? as f32;
        let value = 2 as f32;
        Ok((input / value) as f32)
    }

    fn ClkSMPS_get(&self) -> Result<f32, ClockError> {
        match self.ClkSMPS {
            ClkSMPSConf::ClkSMPSDiv4 => return self.ClkSMPSDiv4_get(),
            ClkSMPSConf::ClkSMPSDiv2 => return self.ClkSMPSDiv2_get(),
        };
    }
    pub fn ClkSMPSOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.ClkSMPS_get()?;
        if input > (64000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 64000000),
                from: ClockNodes::ClkSMPS,
                to: ClockNodes::ClkSMPSOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::ClkSMPS,
                to: ClockNodes::ClkSMPSOutput,
            });
        }
        Ok(input)
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
    fn CLK32MHzDiv1_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()? as f32;
        let value = 1 as f32;
        Ok((input / value) as f32)
    }

    fn CLK32MHzDiv2_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()? as f32;
        let value = 2 as f32;
        Ok((input / value) as f32)
    }

    fn CLK32MHzSource_get(&self) -> Result<f32, ClockError> {
        match self.CLK32MHzSource {
            CLK32MHzSourceConf::CLK32MHzDiv1 => return self.CLK32MHzDiv1_get(),
            CLK32MHzSourceConf::CLK32MHzDiv2 => return self.CLK32MHzDiv2_get(),
        };
    }
    pub fn Clk32MHzOutput_get(&self) -> Result<f32, ClockError> {
        self.CLK32MHzSource_get()
    }
    fn BLEMult_get(&self) -> Result<f32, ClockError> {
        match self.BLEMult {
            BLEMultConf::Clk16MHzOutput => return self.Clk16MHzOutput_get(),
            BLEMultConf::Clk32MHzOutput => return self.Clk32MHzOutput_get(),
        };
    }
    pub fn BLEOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.BLEMult_get()?;
        if input > (64000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 64000000),
                from: ClockNodes::BLEMult,
                to: ClockNodes::BLEOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::BLEMult,
                to: ClockNodes::BLEOutput,
            });
        }
        Ok(input)
    }
    fn SYSCLK32Prescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()? as f32;
        let value = self.SYSCLK32Prescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    fn SYSCLK64Prescaler_get(&self) -> Result<f32, ClockError> {
        let input = self.SysCLKOutput_get()? as f32;
        let value = self.SYSCLK64Prescaler.get()? as f32;
        Ok((input / value) as f32)
    }

    fn CLKSYSMult_get(&self) -> Result<f32, ClockError> {
        match self.CLKSYSMult {
            CLKSYSMultConf::SYSCLK32Prescaler => return self.SYSCLK32Prescaler_get(),
            CLKSYSMultConf::SYSCLK64Prescaler => return self.SYSCLK64Prescaler_get(),
        };
    }
    pub fn CLKSYSOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.CLKSYSMult_get()?;
        if input > (64000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 64000000),
                from: ClockNodes::CLKSYSMult,
                to: ClockNodes::CLKSYSOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::CLKSYSMult,
                to: ClockNodes::CLKSYSOutput,
            });
        }
        Ok(input)
    }
    fn CLKI2S3Mult_get(&self) -> Result<f32, ClockError> {
        match self.CLKI2S3Mult {
            CLKI2S3MultConf::Clk32MHzOutput => return self.Clk32MHzOutput_get(),
            CLKI2S3MultConf::Clk16MHzOutput => return self.Clk16MHzOutput_get(),
        };
    }
    pub fn CLKI2S3Output_get(&self) -> Result<f32, ClockError> {
        let input = self.CLKI2S3Mult_get()?;
        if input > (64000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 64000000),
                from: ClockNodes::CLKI2S3Mult,
                to: ClockNodes::CLKI2S3Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::CLKI2S3Mult,
                to: ClockNodes::CLKI2S3Output,
            });
        }
        Ok(input)
    }
    pub fn CLKI2S2Output_get(&self) -> Result<f32, ClockError> {
        let input = self.CLKI2S2Mult_get()?;
        if input > (64000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 64000000),
                from: ClockNodes::CLKI2S2Mult,
                to: ClockNodes::CLKI2S2Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::CLKI2S2Mult,
                to: ClockNodes::CLKI2S2Output,
            });
        }
        Ok(input)
    }
    fn CLKI2S2Mult_get(&self) -> Result<f32, ClockError> {
        match self.CLKI2S2Mult {
            CLKI2S2MultConf::Clk32MHzOutput => return self.Clk32MHzOutput_get(),
            CLKI2S2MultConf::Clk16MHzOutput => return self.Clk16MHzOutput_get(),
        };
    }
    fn CLK16RTCDevisor_get(&self) -> Result<f32, ClockError> {
        let input = self.Clk16MHzOutput_get()? as f32;
        let value = 512 as f32;
        Ok((input / value) as f32)
    }

    fn RTCClkSource_get(&self) -> Result<f32, ClockError> {
        match self.RTCClkSource {
            RTCClkSourceConf::CLK16RTCDevisor => return self.CLK16RTCDevisor_get(),
            RTCClkSourceConf::LSEOSC => return self.LSEOSC_get(),
            RTCClkSourceConf::LSIRC => return self.LSIRC_get(),
        };
    }
    pub fn RTCOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.RTCClkSource_get()?;
        if input > (1000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 1000000),
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
    fn MCOMult_get(&self) -> Result<f32, ClockError> {
        match self.MCOMult {
            MCOMultConf::Clk16MHzOutput => return self.Clk16MHzOutput_get(),
            MCOMultConf::ClkSMPSOutput => return self.ClkSMPSOutput_get(),
            MCOMultConf::CLKSYSOutput => return self.CLKSYSOutput_get(),
            MCOMultConf::HSEOSC => return self.HSEOSC_get(),
            MCOMultConf::HSIRC => return self.HSIRC_get(),
            MCOMultConf::CLK16RTCDevisor => return self.CLK16RTCDevisor_get(),
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
}
