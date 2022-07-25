#![no_std]
#![no_main]

// Link with panic_halt.
use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4::stm32f405::{self as stm, interrupt};

fn init_gpioc_ouput(rcc: &stm::RCC, gpioc: &stm::GPIOC) {
    // Enable the GPIO C clock.
    // Why do we need to enable the GPIO clock?
    // 8.3.10: Output configuration:
    // > The data present on the I/O pin are sampled into
    // > the input data register every AHB1 clock cycle

    // Bit 0 - IO port C clock enable.
    // No reader is needed, we know what value to write.
    // Exercise: why is a closure required here?
    rcc.ahb1enr.modify(
        |_, w| {
            w.gpiocen().enabled()
        });

    // Set PC1 to be an output.
    gpioc.moder.modify(
        |_, w| {
            w.moder1().output()
        });
}

/**
 * Timer counts up.
 */
fn init_timer2(prescale: u16,
               auto_reload: u32,
               rcc: &stm::RCC,
               tim2: &stm::TIM2) {
    // Enable timer interrupt.
    rcc.apb1enr.modify(|_, w| w.tim2en().enabled());
    // Enable update interupt. This is triggered when the
    // timer overflows and resets to 0.
    tim2.dier.write(|w| w.uie().enabled());

    // Divide counter clock.
    // 16MHz / prescaler.
    tim2.psc.write(|w| w.psc().bits(prescale));

    // Count up to auto_reload then reset.
    tim2.arr.write(|w| w.arr().bits(auto_reload));

    // Start countin'!
    tim2.cr1.write(|w| w.cen().enabled());
}

#[entry]
fn run() -> ! {
    // Acquire the device peripherals (if they exist). 
    // take() can only be called once.
    let peripherals = stm::Peripherals::take().unwrap();
    
    //
    // Setup GPIO to blink our LED.
    //
    init_gpioc_ouput(&peripherals.RCC,
                    &peripherals.GPIOC);

    //
    // Initialize General-purpose timer 2 (TIM2).
    // 32-bit timer.
    // 0.5[s] = reload * (prescale / 16MHz)[s]
    // 0.5[s] = reload * (1000 / 16Hz)[s]
    // reload = 0.5[s] / (1000 / 16Hz)[s]
    // reload = 8000
    init_timer2(1000,
                8000,
                &peripherals.RCC,
                &peripherals.TIM2);

    // Enable the TIM2 interrupt in the Nested vectored interrupt controller (NVIC).
    unsafe {
        cortex_m::peripheral::NVIC::unmask(stm::Interrupt::TIM2);
    }

    // The main thread can now go to sleep.
    // WFI (wait for interrupt) puts the core in sleep until an interrupt occurs.
    loop {
        cortex_m::asm::wfi();
    }
}

#[interrupt]
fn TIM2() {
    // NOTE(unsafe): We have to use unsafe to access the peripheral
    // registers in this interrupt handler because we already used `take()`
    // in the main code. In this case all our uses are safe, not least because
    // the main thread only calls `wfi()` after enabling the interrupt, so
    // no race conditions or other unsafe behaviour is possible.
    // For ways to avoid using unsafe here, consult the Concurrency chapter:
    // https://rust-embedded.github.io/book/concurrency/concurrency.html

    // Clear the UIF bit to indicate the interrupt has been serviced
    unsafe {
        (*stm::TIM2::ptr()).sr.modify(|_, w| w.uif().clear_bit())
    };

    // Read GPIO C ODR1 to see if the pin is set and flip the output state.
    // We use the atomic BSRR register to set/reset it without needing to
    // read-modify-write ODR.
    let ptr = stm::GPIOC::ptr();
    unsafe {
        if (*ptr).odr.read().odr1().is_high() {
            (*ptr).bsrr.write(|w| w.br1().set_bit());
        } else {
            (*ptr).bsrr.write(|w| w.bs1().set_bit());
        }
    }
}