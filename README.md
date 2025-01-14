# dac8564

A platform agnostic library for the Texas Instruments DAC8564.

- https://crates.io/crates/dac8564

## features

- Also supports the Texas Instruments DAC7565, DAC7564, DAC8164
- Full no-std support
- Implemented with embedded-hal (https://docs.rs/embedded-hal/0.2.3/embedded_hal)
- Blocking and non-blocking support

## example

Note: Quick example based on the `stm32h7xx-hal`.

### blocking

```rust
fn main() -> ! {
    // SPI interface pins
    let sck = sck.into_alternate_af5();
    let mosi = mosi.into_alternate_af5();

    // Control lines
    let ldac = ldac.into_push_pull_output();
    let enable = enable.into_push_pull_output();
    let nss = nss.into_push_pull_output();

    let spi: Spi<SPI2, Enabled> = interface.spi(
        (sck, NoMiso, mosi),
        spi::MODE_0,
        20.mhz(),
        prec,
        clocks,
    );

    let mut dac = dac8564::Dac::new(nss, ldac, enable);
    dac.enable();

    // Blocking call. Set value to 1000 on the DAC
    dac.write_blocking(&spi, Channel::A, 1000);
}

```

### non-blocking

```rust
fn main() -> ! {
    let mut dac = dac8564::Dac::new(nss, ldac, enable);
    dac.enable();

    // Prepare the transfer, the payload value here is the data that needs to be
    // written to some kind of buffer, e.g. for DMA or Interrupt usage.
    dac.prepare_transfer(Channel::A, 1000, |payload| {
        // Write payload values to a DMA buffer
    });
}
```