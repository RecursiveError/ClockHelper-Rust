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
    ROOTClkSource,
    ROOTCLKOutput,
    TimerOutput,
    CLK_ROOT_DIV3,
    CLK_ROOT_DIV4,
    CLKROOTDIVSource,
    ClkROOTDIVOutput,
    ClkSMPSDiv4,
    ClkSMPSDiv2,
    ClkSMPSDIV,
    CLK_SPMS_KRM_DIV,
    ClkKRM,
    ClkSMPSOutput,
    LPUARTMult,
    ClkLPUARTOutput,
    LSCOMult,
    LSCOOutput,
    Div2,
    ROOTCLK48Prescaler,
    ROOTCLK64Prescaler,
    CLKSYSMult,
    CLKSYSOutput,
    CLKSPI3I2SMult,
    CLKSPI3I2SOutput,
    CLKROOTCDevisorON512,
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
