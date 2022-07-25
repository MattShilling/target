#![no_std]
#![no_main]

// Override panic.
use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4::stm32f405::{self, interrupt};

fn init_gpioc_ouput(rcc: &stm32f405::RCC, gpioc: &stm32f405::GPIOC) {
    // Enable the GPIO C clock.
    // Why do we need to enable the GPIO clock?
    // 8.3.10: Output configuration:
    // > The data present on the I/O pin are sampled into
    // > the input data register every AHB1 clock cycle

    // Bit 0 - IO port C clock enable.
    // No reader is needed, we know what value to write.
    // Question: why is a closure required here?
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

fn init_timer2(rcc: &stm32f405::RCC, tim2: &stm32f405::TIM2) {
    rcc.apb1enr.modify(|_, w| w.tim2en().enabled());
    tim2.dier.write(|w| w.uie().enabled());
    tim2.psc.write(|w| w.psc().bits(1000));
    tim2.arr.write(|w| w.arr().bits(2000));
    tim2.cr1.write(|w| w.cen().enabled());
}

#[entry]
fn run() -> ! {
    // Acquire the device peripherals (if they exist). 
    // take() can only be called once.
    let peripherals = stm32f405::Peripherals::take().unwrap();
    
    //
    // Setup GPIO to blink our LED.
    //
    init_gpioc_ouput(&peripherals.RCC,
                    &peripherals.GPIOC);

    //
    // Reset and clock control.
    //
    init_timer2(&peripherals.RCC,
               &peripherals.TIM2);

    // Enable the TIM2 interrupt in the Nested vectored interrupt controller (NVIC).
    unsafe {
        cortex_m::peripheral::NVIC::unmask(stm32f405::Interrupt::TIM2);
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
    unsafe { (*stm32f405::TIM2::ptr()).sr.modify(|_, w| w.uif().clear_bit()) };

    // Read ODR8 to see if the pin is set, and if so, clear it,
    // otherwise, set it. We use the atomic BSRR register to
    // set/reset it without needing to read-modify-write ODR.
    let ptr = stm32f405::GPIOC::ptr();
    unsafe {
        if (*ptr).odr.read().odr1().is_high() {
            (*ptr).bsrr.write(|w| w.br1().set_bit());
        } else {
            (*ptr).bsrr.write(|w| w.bs1().set_bit());
        }
    }
}