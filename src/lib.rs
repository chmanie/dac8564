//! # dac8564 library
//! A small library for using the dac8564.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![allow(dead_code)]

use embedded_hal::blocking::spi::Write;
use embedded_hal::digital::v2::OutputPin;

/// DAC8564
pub struct DAC8564<SPI, NSS, LDAC, ENABLE>
where
    SPI: Write<u8>,
{
    spi: SPI,
    nss: NSS,
    ldac: LDAC,
    enable: ENABLE,
    active: bool,
}

/// Channel selection enum
/// DAC8564 has 4 different channels
#[allow(dead_code)]
#[repr(u8)]
#[derive(PartialEq)]
pub enum Channel {
    /// Channel A
    A = 0b0000,
    /// Channel B
    B = 0b0010,
    /// Channel C
    C = 0b0100,
    /// Channel D
    D = 0b0110,
    /// All Channels
    All = 0b0111,
}

impl From<u8> for Channel {
    /// Get the Channel enumeration from an index (begins at 0)
    fn from(index: u8) -> Self {
        match index {
            0 => Channel::A,
            1 => Channel::B,
            2 => Channel::C,
            3 => Channel::D,
            _ => panic!("Channel unknown for index {}", index),
        }
    }
}

/// DAC Related errors
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum DacError {
    /// Unable to write to bus
    BusWriteError,
}

/// Helper function to get the correct communication payload that
/// is sent down the wire to the DAC
fn get_payload(channel: Channel, value: u16) -> [u8; 3] {
    let mut command: [u8; 3] = [0; 3];

    // Channel select
    // a 0b00010000
    // b 0b00010010
    // c 0b00010100
    // d 0b00010110
    command[0] = 0b00010000 | (channel as u8);
    // Upper 8 bits
    command[1] = ((value & 0xFF00) >> 8) as u8;
    // Lower 8 bits
    command[2] = (value & 0xFF) as u8;

    command
}

/// Platform agnostic delay helper
fn delay() {
    let mut x = 0;
    while x < 10000 {
        x += 1;
    }
}

impl<SPI, NSS, LDAC, ENABLE, E> DAC8564<SPI, NSS, LDAC, ENABLE>
where
    SPI: Write<u8, Error = E>,
    NSS: OutputPin,
    LDAC: OutputPin,
    ENABLE: OutputPin,
{
    /// Initialize a new instance of DAC8564
    pub fn new(spi: SPI, nss: NSS, ldac: LDAC, enable: ENABLE) -> Self {
        Self {
            spi,
            nss,
            ldac,
            enable,
            active: false,
        }
    }

    /// Enables the DAC by toggling the Enable, NSS and LDAC lines
    pub fn enable(&mut self) {
        self.enable.set_low().unwrap_or_default();
        self.nss.set_low().unwrap_or_default();
        self.nss.set_high().unwrap_or_default();
        self.enable.set_high().unwrap_or_default();

        // Rising edge to reset the DAC registers
        self.ldac.set_low().unwrap_or_default();

        delay();
        self.ldac.set_high().unwrap_or_default();

        delay();
        self.ldac.set_low().unwrap_or_default();
        self.active = true;
    }

    /// Write to the DAC via a blocking call on the specified SPI interface
    pub fn write_blocking(
        &mut self,
        channel: Channel,
        value: u16,
    ) -> Result<(), DacError> {
        if !self.active {
            return Ok(());
        }
        let command: [u8; 3] = get_payload(channel, value);

        self.enable.set_low().unwrap_or_default();
        self.nss.set_low().unwrap_or_default();
        let result = self.spi.write(&command);
        self.nss.set_high().unwrap_or_default();
        self.enable.set_high().unwrap_or_default();

        match result {
            Ok(v) => Ok(v),
            Err(_e) => Err(DacError::BusWriteError),
        }
    }

    /// For asynchronous communication methods (e.g. Interrupt or DMA), this function
    /// prepares the DAC for the transfer, generates the command and passes it back to the initiator
    /// via the callback parameter
    pub fn prepare_transfer<F: FnMut([u8; 3]) -> ()>(
        &mut self,
        channel: Channel,
        value: u16,
        mut callback: F,
    ) {
        if !self.active {
            return;
        }
        let command: [u8; 3] = get_payload(channel, value);

        self.enable.set_low().unwrap_or_default();
        self.nss.set_low().unwrap_or_default();

        callback(command);

        self.nss.set_high().unwrap_or_default();
        self.enable.set_high().unwrap_or_default();
    }
}
