use core::time::Duration;

use litex_pac::constants::CONFIG_CLOCK_FREQUENCY;
use num_traits::float::Float;

use crate::util::combine_u32;

const CLOCK_CYCLE_PERIOD_NANOS: f64 = 1_000_000_000.0 / (CONFIG_CLOCK_FREQUENCY as f64);

///
/// Gets the number of nanoseconds that have occurred since core start
///
pub fn duration_since_launch() -> Duration {
    let peripherals = unsafe { litex_pac::Peripherals::steal() };

    unsafe {
        // Grab cycle count
        peripherals.TIMER0.uptime_latch.write(|w| w.bits(1));
    };

    let low_bits = peripherals.TIMER0.uptime_cycles0.read().bits();
    let high_bits = peripherals.TIMER0.uptime_cycles1.read().bits();
    let uptime_cycles = combine_u32(low_bits, high_bits);

    let duration = (CLOCK_CYCLE_PERIOD_NANOS * (uptime_cycles as f64)).floor() as u64;

    Duration::from_nanos(duration)
}

///
/// Wait for the provided duration
///
pub fn sleep(duration: Duration) {
    let start_timestamp = duration_since_launch();

    while duration_since_launch() - start_timestamp < duration {}
}
