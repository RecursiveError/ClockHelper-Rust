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
    MSIKSource,
    MSISSource,
    MSIKDIV,
    MSISDIV,
    MSISOutput,
    SAI1_EXT,
    SysClkSource,
    SysCLKOutput,
    SPI1Mult,
    SPI1output,
    SPI3Mult,
    SPI3output,
    SPI2Mult,
    SPI2output,
    HSERTCDevisor,
    RTCClkSource,
    RTCOutput,
    IWDGOutput,
    USART1Mult,
    USART1output,
    USART3Mult,
    USART3output,
    UART4Mult,
    UART4output,
    ADCMult,
    ADCDiv,
    ADCoutput,
    LPUART1Mult,
    LPUART1output,
    LPTIM1Mult,
    LPTIM1output,
    LPTIM2Mult,
    LPTIM2output,
    DACMult,
    DACoutput,
    ICLKMult,
    SDMMC1Output,
    USBDiv,
    USBoutput,
    FDCANMult,
    FDCANOutput,
    I2C1Mult,
    I2C1output,
    I3C1Mult,
    I3C1output,
    I3C2Mult,
    I3C2output,
    I2C2Mult,
    I2C2output,
    I2C3Mult,
    I2C3output,
    SAI1Mult,
    SAI1output,
    ADF1Mult,
    ADF1output,
    OCTOSPIMMult,
    OCTOSPIMoutput,
    LPTIM3Mult,
    LPTIM3output,
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
pub enum MSIKSourceConf {
    CLOCK_96,
    CLOCK_24,
}

impl MSIKSourceConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MSIKSourceConf::CLOCK_96 => return Ok(96.0),
            MSIKSourceConf::CLOCK_24 => return Ok(24.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MSISSourceConf {
    CLOCK_96,
    CLOCK_24,
}

impl MSISSourceConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MSISSourceConf::CLOCK_96 => return Ok(96.0),
            MSISSourceConf::CLOCK_24 => return Ok(24.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MSIKDIVConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
}

impl MSIKDIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MSIKDIVConf::DIV1 => return Ok(1.0),
            MSIKDIVConf::DIV2 => return Ok(2.0),
            MSIKDIVConf::DIV4 => return Ok(4.0),
            MSIKDIVConf::DIV8 => return Ok(8.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MSISDIVConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
}

impl MSISDIVConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            MSISDIVConf::DIV1 => return Ok(1.0),
            MSISDIVConf::DIV2 => return Ok(2.0),
            MSISDIVConf::DIV4 => return Ok(4.0),
            MSISDIVConf::DIV8 => return Ok(8.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SysClkSourceConf {
    MSISOutput,
    HSIRC,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI1MultConf {
    APB2Prescaler,
    MSIKDIV,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI3MultConf {
    APB1Prescaler,
    MSIKDIV,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SPI2MultConf {
    APB1Prescaler,
    MSIKDIV,
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
    HSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USART3MultConf {
    APB1Prescaler,
    HSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum UART4MultConf {
    APB1Prescaler,
    HSIRC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ADCMultConf {
    AHBOutput,
    HSEOSC,
    MSIKDIV,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ADCDivConf {
    DIV1,
    DIV2,
    DIV4,
    DIV8,
    DIV16,
    DIV32,
    DIV64,
    DIV128,
    DIV256,
    DIV512,
}

impl ADCDivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            ADCDivConf::DIV1 => return Ok(1.0),
            ADCDivConf::DIV2 => return Ok(2.0),
            ADCDivConf::DIV4 => return Ok(4.0),
            ADCDivConf::DIV8 => return Ok(8.0),
            ADCDivConf::DIV16 => return Ok(16.0),
            ADCDivConf::DIV32 => return Ok(32.0),
            ADCDivConf::DIV64 => return Ok(64.0),
            ADCDivConf::DIV128 => return Ok(128.0),
            ADCDivConf::DIV256 => return Ok(256.0),
            ADCDivConf::DIV512 => return Ok(512.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPUART1MultConf {
    APB3Output,
    HSIRC,
    LSEOSC,
    MSIKDIV,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM1MultConf {
    MSIKDIV,
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
pub enum ICLKMultConf {
    SysCLKOutput,
    MSIKDIV,
    HSEOSC,
    HSI48RC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum USBDivConf {
    DIV1,
    DIV2,
}

impl USBDivConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
            USBDivConf::DIV1 => return Ok(1.0),
            USBDivConf::DIV2 => return Ok(2.0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum FDCANMultConf {
    SysCLKOutput,
    MSIKDIV,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C1MultConf {
    APB1Prescaler,
    MSIKDIV,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I3C1MultConf {
    APB1Prescaler,
    MSIKDIV,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I3C2MultConf {
    APB3Output,
    MSIKDIV,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C2MultConf {
    APB1Prescaler,
    MSIKDIV,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum I2C3MultConf {
    APB3Output,
    MSIKDIV,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SAI1MultConf {
    MSIKDIV,
    SAI1_EXT,
    HSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ADF1MultConf {
    AHBOutput,
    SAI1_EXT,
    MSIKDIV,
    SAI1output,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum OCTOSPIMMultConf {
    SysCLKOutput,
    MSIKDIV,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum LPTIM3MultConf {
    MSIKDIV,
    LSIDIV,
    HSIRC,
    LSEOSC,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum RNGMultConf {
    HSI48RC,
    MSIKDIV,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum MCOMultConf {
    LSEOSC,
    LSIDIV,
    HSEOSC,
    HSIRC,
    SysCLKOutput,
    MSISDIV,
    HSI48RC,
    MSIKDIV,
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
    LSIDIV,
    HSEOSC,
    HSIRC,
    SysCLKOutput,
    MSISDIV,
    HSI48RC,
    MSIKDIV,
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
    DIV8,
}

impl CortexPrescalerConf {
    pub fn get(&self) -> Result<f32, ClockError> {
        match self {
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
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ClockTree {
    pub HSEOSC: HSEOSCConf,
    pub LSIRC: LSIRCConf,
    pub LSIDIV: LSIDIVConf,
    pub LSEOSC: LSEOSCConf,
    pub MSIKSource: MSIKSourceConf,
    pub MSISSource: MSISSourceConf,
    pub MSIKDIV: MSIKDIVConf,
    pub MSISDIV: MSISDIVConf,
    pub SysClkSource: SysClkSourceConf,
    pub SPI1Mult: SPI1MultConf,
    pub SPI3Mult: SPI3MultConf,
    pub SPI2Mult: SPI2MultConf,
    pub RTCClkSource: RTCClkSourceConf,
    pub USART1Mult: USART1MultConf,
    pub USART3Mult: USART3MultConf,
    pub UART4Mult: UART4MultConf,
    pub ADCMult: ADCMultConf,
    pub ADCDiv: ADCDivConf,
    pub LPUART1Mult: LPUART1MultConf,
    pub LPTIM1Mult: LPTIM1MultConf,
    pub LPTIM2Mult: LPTIM2MultConf,
    pub DACMult: DACMultConf,
    pub ICLKMult: ICLKMultConf,
    pub USBDiv: USBDivConf,
    pub FDCANMult: FDCANMultConf,
    pub I2C1Mult: I2C1MultConf,
    pub I3C1Mult: I3C1MultConf,
    pub I3C2Mult: I3C2MultConf,
    pub I2C2Mult: I2C2MultConf,
    pub I2C3Mult: I2C3MultConf,
    pub SAI1Mult: SAI1MultConf,
    pub ADF1Mult: ADF1MultConf,
    pub OCTOSPIMMult: OCTOSPIMMultConf,
    pub LPTIM3Mult: LPTIM3MultConf,
    pub RNGMult: RNGMultConf,
    pub MCOMult: MCOMultConf,
    pub MCODiv: MCODivConf,
    pub MCO2Mult: MCO2MultConf,
    pub MCO2Div: MCO2DivConf,
    pub LSCOMult: LSCOMultConf,
    pub AHBPrescaler: AHBPrescalerConf,
    pub CortexPrescaler: CortexPrescalerConf,
    pub CortexCLockSelection: CortexCLockSelectionConf,
    pub APB1Prescaler: APB1PrescalerConf,
    pub APB2Prescaler: APB2PrescalerConf,
    pub APB3Prescaler: APB3PrescalerConf,
}

impl Default for ClockTree {
    fn default() -> Self {
        Self {
            HSEOSC: HSEOSCConf::Value(16000000),
            LSIRC: LSIRCConf::Value(32000),
            LSIDIV: LSIDIVConf::DIV1,
            LSEOSC: LSEOSCConf::Value(32768),
            MSIKSource: MSIKSourceConf::CLOCK_96,
            MSISSource: MSISSourceConf::CLOCK_96,
            MSIKDIV: MSIKDIVConf::DIV1,
            MSISDIV: MSISDIVConf::DIV8,
            SysClkSource: SysClkSourceConf::MSISOutput,
            SPI1Mult: SPI1MultConf::APB2Prescaler,
            SPI3Mult: SPI3MultConf::APB1Prescaler,
            SPI2Mult: SPI2MultConf::APB1Prescaler,
            RTCClkSource: RTCClkSourceConf::LSIDIV,
            USART1Mult: USART1MultConf::APB2Prescaler,
            USART3Mult: USART3MultConf::APB1Prescaler,
            UART4Mult: UART4MultConf::APB1Prescaler,
            ADCMult: ADCMultConf::AHBOutput,
            ADCDiv: ADCDivConf::DIV1,
            LPUART1Mult: LPUART1MultConf::APB3Output,
            LPTIM1Mult: LPTIM1MultConf::MSIKDIV,
            LPTIM2Mult: LPTIM2MultConf::APB1Prescaler,
            DACMult: DACMultConf::LSIDIV,
            ICLKMult: ICLKMultConf::SysCLKOutput,
            USBDiv: USBDivConf::DIV1,
            FDCANMult: FDCANMultConf::SysCLKOutput,
            I2C1Mult: I2C1MultConf::APB1Prescaler,
            I3C1Mult: I3C1MultConf::APB1Prescaler,
            I3C2Mult: I3C2MultConf::APB3Output,
            I2C2Mult: I2C2MultConf::APB1Prescaler,
            I2C3Mult: I2C3MultConf::APB3Output,
            SAI1Mult: SAI1MultConf::MSIKDIV,
            ADF1Mult: ADF1MultConf::AHBOutput,
            OCTOSPIMMult: OCTOSPIMMultConf::SysCLKOutput,
            LPTIM3Mult: LPTIM3MultConf::MSIKDIV,
            RNGMult: RNGMultConf::HSI48RC,
            MCOMult: MCOMultConf::SysCLKOutput,
            MCODiv: MCODivConf::DIV1,
            MCO2Mult: MCO2MultConf::SysCLKOutput,
            MCO2Div: MCO2DivConf::DIV1,
            LSCOMult: LSCOMultConf::LSIDIV,
            AHBPrescaler: AHBPrescalerConf::DIV1,
            CortexPrescaler: CortexPrescalerConf::DIV8,
            CortexCLockSelection: CortexCLockSelectionConf::CortexPrescaler,
            APB1Prescaler: APB1PrescalerConf::DIV1,
            APB2Prescaler: APB2PrescalerConf::DIV1,
            APB3Prescaler: APB3PrescalerConf::DIV1,
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
    pub fn MSIKSource_get(&self) -> Result<f32, ClockError> {
        self.MSIKSource.get()
    }
    pub fn MSISSource_get(&self) -> Result<f32, ClockError> {
        self.MSISSource.get()
    }
    fn MSIKDIV_get(&self) -> Result<f32, ClockError> {
        let input = self.MSIKSource_get()? as f32;
        let value = self.MSIKDIV.get()? as f32;
        Ok((input / value) as f32)
    }

    fn MSISDIV_get(&self) -> Result<f32, ClockError> {
        let input = self.MSISSource_get()? as f32;
        let value = self.MSISDIV.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn MSISOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.MSISDIV_get()?;
        if input > (96000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 96000000),
                from: ClockNodes::MSISDIV,
                to: ClockNodes::MSISOutput,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::MSISDIV,
                to: ClockNodes::MSISOutput,
            });
        }
        Ok(input)
    }
    pub fn SAI1_EXT_get(&self) -> Result<f32, ClockError> {
        Ok(48000 as f32)
    }
    fn SysClkSource_get(&self) -> Result<f32, ClockError> {
        match self.SysClkSource {
            SysClkSourceConf::MSISOutput => return self.MSISOutput_get(),
            SysClkSourceConf::HSIRC => return self.HSIRC_get(),
            SysClkSourceConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    pub fn SysCLKOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.SysClkSource_get()?;
        if input > (96000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 96000000),
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
    fn SPI1Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI1Mult {
            SPI1MultConf::APB2Prescaler => return self.APB2Prescaler_get(),
            SPI1MultConf::MSIKDIV => return self.MSIKDIV_get(),
        };
    }
    pub fn SPI1output_get(&self) -> Result<f32, ClockError> {
        self.SPI1Mult_get()
    }
    fn SPI3Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI3Mult {
            SPI3MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            SPI3MultConf::MSIKDIV => return self.MSIKDIV_get(),
        };
    }
    pub fn SPI3output_get(&self) -> Result<f32, ClockError> {
        self.SPI3Mult_get()
    }
    fn SPI2Mult_get(&self) -> Result<f32, ClockError> {
        match self.SPI2Mult {
            SPI2MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            SPI2MultConf::MSIKDIV => return self.MSIKDIV_get(),
        };
    }
    pub fn SPI2output_get(&self) -> Result<f32, ClockError> {
        self.SPI2Mult_get()
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
            USART1MultConf::HSIRC => return self.HSIRC_get(),
        };
    }
    pub fn USART1output_get(&self) -> Result<f32, ClockError> {
        self.USART1Mult_get()
    }
    fn USART3Mult_get(&self) -> Result<f32, ClockError> {
        match self.USART3Mult {
            USART3MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            USART3MultConf::HSIRC => return self.HSIRC_get(),
        };
    }
    pub fn USART3output_get(&self) -> Result<f32, ClockError> {
        self.USART3Mult_get()
    }
    fn UART4Mult_get(&self) -> Result<f32, ClockError> {
        match self.UART4Mult {
            UART4MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            UART4MultConf::HSIRC => return self.HSIRC_get(),
        };
    }
    pub fn UART4output_get(&self) -> Result<f32, ClockError> {
        self.UART4Mult_get()
    }
    fn ADCMult_get(&self) -> Result<f32, ClockError> {
        match self.ADCMult {
            ADCMultConf::AHBOutput => return self.AHBOutput_get(),
            ADCMultConf::HSEOSC => return self.HSEOSC_get(),
            ADCMultConf::MSIKDIV => return self.MSIKDIV_get(),
        };
    }
    fn ADCDiv_get(&self) -> Result<f32, ClockError> {
        let input = self.ADCMult_get()? as f32;
        let value = self.ADCDiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn ADCoutput_get(&self) -> Result<f32, ClockError> {
        self.ADCDiv_get()
    }
    fn LPUART1Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPUART1Mult {
            LPUART1MultConf::APB3Output => return self.APB3Output_get(),
            LPUART1MultConf::HSIRC => return self.HSIRC_get(),
            LPUART1MultConf::LSEOSC => return self.LSEOSC_get(),
            LPUART1MultConf::MSIKDIV => return self.MSIKDIV_get(),
        };
    }
    pub fn LPUART1output_get(&self) -> Result<f32, ClockError> {
        self.LPUART1Mult_get()
    }
    fn LPTIM1Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM1Mult {
            LPTIM1MultConf::MSIKDIV => return self.MSIKDIV_get(),
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
    fn ICLKMult_get(&self) -> Result<f32, ClockError> {
        match self.ICLKMult {
            ICLKMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            ICLKMultConf::MSIKDIV => return self.MSIKDIV_get(),
            ICLKMultConf::HSEOSC => return self.HSEOSC_get(),
            ICLKMultConf::HSI48RC => return self.HSI48RC_get(),
        };
    }
    pub fn SDMMC1Output_get(&self) -> Result<f32, ClockError> {
        let input = self.ICLKMult_get()?;
        if input > (55000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 55000000),
                from: ClockNodes::ICLKMult,
                to: ClockNodes::SDMMC1Output,
            });
        } else if input < (0 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Underflow(input as u32, 0),
                from: ClockNodes::ICLKMult,
                to: ClockNodes::SDMMC1Output,
            });
        }
        Ok(input)
    }
    fn USBDiv_get(&self) -> Result<f32, ClockError> {
        let input = self.ICLKMult_get()? as f32;
        let value = self.USBDiv.get()? as f32;
        Ok((input / value) as f32)
    }

    pub fn USBoutput_get(&self) -> Result<f32, ClockError> {
        self.USBDiv_get()
    }
    fn FDCANMult_get(&self) -> Result<f32, ClockError> {
        match self.FDCANMult {
            FDCANMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            FDCANMultConf::MSIKDIV => return self.MSIKDIV_get(),
        };
    }
    pub fn FDCANOutput_get(&self) -> Result<f32, ClockError> {
        self.FDCANMult_get()
    }
    fn I2C1Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C1Mult {
            I2C1MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            I2C1MultConf::MSIKDIV => return self.MSIKDIV_get(),
        };
    }
    pub fn I2C1output_get(&self) -> Result<f32, ClockError> {
        self.I2C1Mult_get()
    }
    fn I3C1Mult_get(&self) -> Result<f32, ClockError> {
        match self.I3C1Mult {
            I3C1MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            I3C1MultConf::MSIKDIV => return self.MSIKDIV_get(),
        };
    }
    pub fn I3C1output_get(&self) -> Result<f32, ClockError> {
        self.I3C1Mult_get()
    }
    fn I3C2Mult_get(&self) -> Result<f32, ClockError> {
        match self.I3C2Mult {
            I3C2MultConf::APB3Output => return self.APB3Output_get(),
            I3C2MultConf::MSIKDIV => return self.MSIKDIV_get(),
        };
    }
    pub fn I3C2output_get(&self) -> Result<f32, ClockError> {
        self.I3C2Mult_get()
    }
    fn I2C2Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C2Mult {
            I2C2MultConf::APB1Prescaler => return self.APB1Prescaler_get(),
            I2C2MultConf::MSIKDIV => return self.MSIKDIV_get(),
        };
    }
    pub fn I2C2output_get(&self) -> Result<f32, ClockError> {
        self.I2C2Mult_get()
    }
    fn I2C3Mult_get(&self) -> Result<f32, ClockError> {
        match self.I2C3Mult {
            I2C3MultConf::APB3Output => return self.APB3Output_get(),
            I2C3MultConf::MSIKDIV => return self.MSIKDIV_get(),
        };
    }
    pub fn I2C3output_get(&self) -> Result<f32, ClockError> {
        self.I2C3Mult_get()
    }
    fn SAI1Mult_get(&self) -> Result<f32, ClockError> {
        match self.SAI1Mult {
            SAI1MultConf::MSIKDIV => return self.MSIKDIV_get(),
            SAI1MultConf::SAI1_EXT => return self.SAI1_EXT_get(),
            SAI1MultConf::HSEOSC => return self.HSEOSC_get(),
        };
    }
    pub fn SAI1output_get(&self) -> Result<f32, ClockError> {
        self.SAI1Mult_get()
    }
    fn ADF1Mult_get(&self) -> Result<f32, ClockError> {
        match self.ADF1Mult {
            ADF1MultConf::AHBOutput => return self.AHBOutput_get(),
            ADF1MultConf::SAI1_EXT => return self.SAI1_EXT_get(),
            ADF1MultConf::MSIKDIV => return self.MSIKDIV_get(),
            ADF1MultConf::SAI1output => return self.SAI1output_get(),
        };
    }
    pub fn ADF1output_get(&self) -> Result<f32, ClockError> {
        self.ADF1Mult_get()
    }
    fn OCTOSPIMMult_get(&self) -> Result<f32, ClockError> {
        match self.OCTOSPIMMult {
            OCTOSPIMMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            OCTOSPIMMultConf::MSIKDIV => return self.MSIKDIV_get(),
        };
    }
    pub fn OCTOSPIMoutput_get(&self) -> Result<f32, ClockError> {
        self.OCTOSPIMMult_get()
    }
    fn LPTIM3Mult_get(&self) -> Result<f32, ClockError> {
        match self.LPTIM3Mult {
            LPTIM3MultConf::MSIKDIV => return self.MSIKDIV_get(),
            LPTIM3MultConf::LSIDIV => return self.LSIDIV_get(),
            LPTIM3MultConf::HSIRC => return self.HSIRC_get(),
            LPTIM3MultConf::LSEOSC => return self.LSEOSC_get(),
        };
    }
    pub fn LPTIM3output_get(&self) -> Result<f32, ClockError> {
        self.LPTIM3Mult_get()
    }
    fn RNGMult_get(&self) -> Result<f32, ClockError> {
        match self.RNGMult {
            RNGMultConf::HSI48RC => return self.HSI48RC_get(),
            RNGMultConf::MSIKDIV => return self.MSIKDIV_get(),
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
            MCOMultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            MCOMultConf::MSISDIV => return self.MSISDIV_get(),
            MCOMultConf::HSI48RC => return self.HSI48RC_get(),
            MCOMultConf::MSIKDIV => return self.MSIKDIV_get(),
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
            MCO2MultConf::LSIDIV => return self.LSIDIV_get(),
            MCO2MultConf::HSEOSC => return self.HSEOSC_get(),
            MCO2MultConf::HSIRC => return self.HSIRC_get(),
            MCO2MultConf::SysCLKOutput => return self.SysCLKOutput_get(),
            MCO2MultConf::MSISDIV => return self.MSISDIV_get(),
            MCO2MultConf::HSI48RC => return self.HSI48RC_get(),
            MCO2MultConf::MSIKDIV => return self.MSIKDIV_get(),
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
        let input = self.AHBPrescaler_get()?;
        if input > (96000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 96000000),
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
        if input > (96000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 96000000),
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
            CortexCLockSelectionConf::LSIDIV => return self.LSIDIV_get(),
        };
    }
    pub fn CortexSysOutput_get(&self) -> Result<f32, ClockError> {
        let input = self.CortexCLockSelection_get()?;
        if input > (96000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 96000000),
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
        if input > (96000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 96000000),
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
        if input > (96000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 96000000),
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
        if input > (96000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 96000000),
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
        if input > (96000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 96000000),
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
        if input > (96000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 96000000),
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
        if input > (96000000 as f32) {
            return Err(ClockError {
                err_type: ClockErrorType::Overflow(input as u32, 96000000),
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
}
