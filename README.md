# ClockHelper
ClockHelper is a utility to facilitate clock management on STM32 microcontrollers.

>warning:
>this library is still under development.
>
>this code was automatically generated from cubeMX data, names have not yet been ported to the snake_case standard and values ​​may be incorrect.

## usage
To use it, it's very simple:

Choose a controller family from the [list](#supported-families):

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

## supported families:
STM32L4R9
STM32H7S3
STM32F415
STM32L063
STM32L051
STM32H7S7
STM32H573
STM32G441
STM32F469
STM32F732
STM32L552
STM32L031
STM32H563
STM32G031
STM32G491
STM32L152
STM32MP23
STM32L432
STM32F413
STM32L462
STM32F769
STM32F101
STM32WB55
STM32F103
STM32G473
STM32F439
STM32C091
STM32F072
STM32G0B1
STM32F479
STM32L422
STM32L4P5
STM32F303
STM32L4S9
STM32F042
STM32WB05
STM32F301
STM32L496
STM32U575
STM32F030
STM32F446
STM32L162
STM32G0C1
STM32U385
STM32H523
STM32F100
STM32L476
STM32C051
STM32F051
STM32H562
STM32L442
STM32C092
STM32H7B0
STM32MP25
STM32F031
STM32F358
STM32L4S5
STM32MP15
STM32L431
STM32U375
STM32L452
STM32U545
STM32L151
STM32F091
STM32WB35
STM32F423
STM32L011
STM32U031
STM32H7R7
STM32WL5M
STM32C031
STM32F334
STM32U585
STM32N655
STM32L451
STM32F107
STM32G4A1
STM32H753
STM32F078
STM32G030
STM32G484
STM32H735
STM32L041
STM32U5A5
STM32MP13
STM32H7B3
STM32G471
STM32U073
STM32F756
STM32L073
STM32H745
STM32WBA5
STM32F730
STM32U595
STM32U5F9
STM32L062
STM32F410
STM32L4A6
STM32L010
STM32F733
STM32G414
STM32F098
STM32F405
STM32L4Q5
STM32C071
STM32U599
STM32N657
STM32F745
STM32G081
STM32H743
STM32WB15
STM32H725
STM32L471
STM32F302
STM32G411
STM32L475
STM32F071
STM32F205
STM32H755
STM32WL54
STM32G071
STM32WLE4
STM32F417
STM32H747
STM32N647
STM32L083
STM32F048
STM32F746
STM32H7R3
STM32L072
STM32F217
STM32F401
STM32L4S7
STM32H7A3
STM32H742
STM32WLE5
STM32G474
STM32H503
STM32F723
STM32WL33
STM32C011
STM32H723
STM32U535
STM32F777
STM32F070
STM32F767
STM32F038
STM32L562
STM32G041
STM32L071
STM32F058
STM32F750
STM32WB09
STM32F765
STM32F215
STM32F429
STM32G431
STM32L4R5
STM32L021
STM32N645
STM32L412
STM32L486
STM32H533
STM32F407
STM32F411
STM32L4R7
STM32L053
STM32H730
STM32F207
STM32F328
STM32L082
STM32F373
STM32G483
STM32L433
STM32F779
STM32F412
STM32F437
STM32WL55
STM32U5G9
STM32L100
STM32F722
STM32F318
STM32L081
STM32H757
STM32U5A9
STM32H733
STM32WB07
STM32WB5M
STM32L052
STM32F427
STM32U5F7
STM32F768
STM32F378
STM32L443
STM32WB06
STM32WB1M
STM32H750
STM32U083
STM32F102
STM32F105
STM32WB50
STM32U5G7
STM32WB10
STM32F778
STM32G0B0
STM32G070
STM32F398
STM32WB30
STM32L485
