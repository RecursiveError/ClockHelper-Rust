# ClockHelper
ClockHelper is a utility to facilitate clock management on STM32 microcontrollers.

>warning:
>this library is still under development.
>
>this code was automatically generated from cubeMX data, names have not yet been ported to the snake_case standard and values ​​may be incorrect.

## usage
To use it, it's very simple:

Choose a controller from the [list](src/ClockTrees/):

```
[dependencies.ClockHelper]
features = ["<mcu_family_name>"]
```

Create a clock using the settings you want to use:
```Rust
use ClockHelper::ClockTrees::<mcu_generic_name> as Clock;

fn main() {
    let mut tree = Clock::ClockTree::default();
}
```

And that's it, ClockHelper will give you the clock value of each peripheral for this setting, and it will also warn you if a setting is invalid!

example for STM32F103C8Tx:

```Rust
use clock_helper::ClockTrees::STM32F103C8_BTx as Clock;

fn main() {
    let mut tree = Clock::ClockTree::default();

    println!("
    Default Config:
    SysClock: {}
    APB1: {}
    APB2: {}
    AHB: {}

    ", 
    tree.SysCLKOutput_get().unwrap(), 
    tree.APB1Output_get().unwrap(),
    tree.APB2Output_get().unwrap(),
    tree.AHBOutput_get().unwrap());


    tree.SysClkSource = Clock::SysClkSourceConf::PLLMUL;
    tree.PLLSource = Clock::PLLSourceConf::HSEDivPLL;
    tree.PLLMUL = Clock::PLLMULConf::MUL6;
    tree.APB1Prescaler = Clock::APB1PrescalerConf::DIV2;


    println!("
    HSE + PLLMUL6:
    SysClock: {}
    APB1: {}
    APB2: {}
    AHB: {}

    ", 
    tree.SysCLKOutput_get().unwrap(), 
    tree.APB1Output_get().unwrap(),
    tree.APB2Output_get().unwrap(),
    tree.AHBOutput_get().unwrap());

    //this will fail

    tree.APB1Prescaler = Clock::APB1PrescalerConf::DIV1;

    let _ = tree.APB1Output_get().unwrap();
}

```