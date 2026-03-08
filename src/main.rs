#![no_std]
#![no_main]

use arduino_hal::spi;

use panic_halt as _;
use smart_leds::SmartLedsWrite;
use smart_leds::RGB8;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").unwrap();

    let (spi, _) = arduino_hal::Spi::new(
        dp.SPI,
        pins.d52.into_output(),
        pins.d51.into_output(),
        pins.d50.into_pull_up_input(),
        pins.d53.into_output(),
        spi::Settings::default(),
    );

    let mut leds = ws2812_spi::Ws2812::new(spi);

    let mut data = [RGB8::default(); 8 * 8];

    data[0] = [10, 0, 0].into();

    loop {
        leds.write(data.iter().cloned()).ok();
        data.rotate_left(1);
        arduino_hal::delay_ms(100);
    }
}
