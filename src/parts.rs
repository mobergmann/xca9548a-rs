use crate::{DoOnAcquired, Error, SelectChannels};
use core::marker::PhantomData;
use embedded_hal::i2c::I2c;

/// Slave I2C device
pub struct I2cSlave<'a, DEV: 'a, I2C>(&'a DEV, u8, PhantomData<I2C>);

macro_rules! parts {
    ( $name:ident; $( $i2cx:ident, $channel:expr ),+ ) => {

        /// Slave I2C devices
        pub struct $name<'a, DEV:'a, I2C> {
            $(
                /// Slave I2C device
                pub $i2cx: I2cSlave<'a, DEV, I2C>,
            )*
        }

        impl<'a, DEV:'a, I2C> $name<'a, DEV, I2C> {
            pub(crate) fn new(dev: &'a DEV) -> Self {
                $name {
                    $(
                        $i2cx: I2cSlave(&dev, $channel, PhantomData),
                    )*
                }
            }
        }
    }
}
parts!(
    Parts; i2c0, 0x01, i2c1, 0x02, i2c2, 0x04, i2c3, 0x08, i2c4, 0x10, i2c5, 0x20, i2c6, 0x40, i2c7, 0x80
);
parts!(
    Parts2; i2c0, 0x01, i2c1, 0x02
);
parts!(
    Parts4; i2c0, 0x01, i2c1, 0x02, i2c2, 0x04, i2c3, 0x08
);

impl<'a, DEV, I2C, E> I2cSlave<'a, DEV, I2C>
where
    DEV: DoOnAcquired<I2C>,
    I2C: I2c<Error = E>,
{
    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Error<E>> {
        self.0.do_on_acquired(|mut dev| {
            if dev.selected_channel_mask != self.1 {
                dev.select_channels(self.1)?;
            }
            dev.i2c.write(address, bytes).map_err(Error::I2C)
        })
    }
// }

// impl<'a, DEV, I2C, E> I2cSlave<'a, DEV, I2C>
// where
//     DEV: DoOnAcquired<I2C>,
//     I2C: I2c<Error = E> + I2c<Error = E>,
// {
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Error<E>> {
        self.0.do_on_acquired(|mut dev| {
            if dev.selected_channel_mask != self.1 {
                dev.select_channels(self.1)?;
            }
            dev.i2c.read(address, buffer).map_err(Error::I2C)
        })
    }

    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Error<E>> {
        self.0.do_on_acquired(|mut dev| {
            if dev.selected_channel_mask != self.1 {
                dev.select_channels(self.1)?;
            }
            dev.i2c
                .write_read(address, bytes, buffer)
                .map_err(Error::I2C)
        })
    }
}
