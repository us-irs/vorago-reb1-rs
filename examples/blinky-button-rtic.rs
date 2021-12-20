//! Blinky button application for the REB1 board using RTIC
#![no_main]
#![no_std]

#[rtic::app(device = pac)]
mod app {
    use panic_rtt_target as _;
    use rtt_target::{rprintln, rtt_init_default, set_print_channel};
    use va108xx_hal::{
        clock::{set_clk_div_register, FilterClkSel},
        gpio::{FilterType, InterruptEdge, PinsA},
        pac,
        prelude::*,
        time::Hertz,
        timer::{default_ms_irq_handler, set_up_ms_timer, IrqCfg},
    };
    use vorago_reb1::button::Button;
    use vorago_reb1::leds::Leds;

    #[derive(Debug, PartialEq)]
    pub enum PressMode {
        Toggle,
        Keep,
    }

    #[derive(Debug, PartialEq)]
    pub enum CfgMode {
        Prompt,
        Fixed,
    }

    const CFG_MODE: CfgMode = CfgMode::Fixed;
    // You can change the press mode here
    const DEFAULT_MODE: PressMode = PressMode::Toggle;

    #[local]
    struct Local {
        leds: Leds,
        button: Button,
        mode: PressMode,
    }

    #[shared]
    struct Shared {}

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let channels = rtt_init_default!();
        set_print_channel(channels.up.0);
        rprintln!("-- Vorago Button IRQ Example --");
        let mode = match CFG_MODE {
            // Ask mode from user via RTT
            CfgMode::Prompt => prompt_mode(channels.down.0),
            // Use mode hardcoded in `DEFAULT_MODE`
            CfgMode::Fixed => DEFAULT_MODE,
        };
        rprintln!("Using {:?} mode", mode);

        let mut dp = ctx.device;
        let pinsa = PinsA::new(&mut dp.SYSCONFIG, Some(dp.IOCONFIG), dp.PORTA);
        let edge_irq = match mode {
            PressMode::Toggle => InterruptEdge::HighToLow,
            PressMode::Keep => InterruptEdge::BothEdges,
        };

        // Configure an edge interrupt on the button and route it to interrupt vector 15
        let mut button = Button::new(pinsa.pa11.into_floating_input()).edge_irq(
            edge_irq,
            IrqCfg::new(pac::interrupt::OC15, true, true),
            Some(&mut dp.SYSCONFIG),
            Some(&mut dp.IRQSEL),
        );

        if mode == PressMode::Toggle {
            // This filter debounces the switch for edge based interrupts
            button = button.filter_type(FilterType::FilterFourClockCycles, FilterClkSel::Clk1);
            set_clk_div_register(
                &mut dp.SYSCONFIG,
                FilterClkSel::Clk1,
                Hertz::from(50.khz()).0,
            );
        }
        let mut leds = Leds::new(
            pinsa.pa10.into_push_pull_output(),
            pinsa.pa7.into_push_pull_output(),
            pinsa.pa6.into_push_pull_output(),
        );
        for led in leds.iter_mut() {
            led.off();
        }
        set_up_ms_timer(
            IrqCfg::new(pac::Interrupt::OC0, true, true),
            &mut dp.SYSCONFIG,
            Some(&mut dp.IRQSEL),
            50.mhz(),
            dp.TIM0,
        );
        (Shared {}, Local { leds, button, mode }, init::Monotonics())
    }

    // `shared` cannot be accessed from this context
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {}
    }

    #[task(binds = OC15, local=[button, leds, mode])]
    fn button_task(cx: button_task::Context) {
        let leds = cx.local.leds;
        let button = cx.local.button;
        let mode = cx.local.mode;
        if *mode == PressMode::Toggle {
            leds[0].toggle();
        } else {
            if button.released() {
                leds[0].off();
            } else {
                leds[0].on();
            }
        }
    }

    #[task(binds = OC0)]
    fn ms_tick(_cx: ms_tick::Context) {
        default_ms_irq_handler();
    }

    fn prompt_mode(mut down_channel: rtt_target::DownChannel) -> PressMode {
        rprintln!("Using prompt mode");
        rprintln!("Please enter the mode [0: Toggle, 1: Keep]");
        let mut read_buf: [u8; 16] = [0; 16];
        let mut read;
        loop {
            read = down_channel.read(&mut read_buf);
            for i in 0..read {
                let val = read_buf[i] as char;
                if val == '0' || val == '1' {
                    if val == '0' {
                        return PressMode::Toggle;
                    } else {
                        return PressMode::Keep;
                    }
                }
            }
        }
    }
}
