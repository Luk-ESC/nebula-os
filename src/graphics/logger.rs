use core::fmt::Write;
use framebuffer::safe::Writer;
use hal::interrupts::without_interrupts;

pub(crate) static LOGGER: Writer = Writer::new();

#[macro_export]
macro_rules! log {
    () => {
        $crate::graphics::logger::_log("\n")
    };
    ($($arg:tt)*) => {
         $crate::graphics::logger::_log(format_args!("[LOG]: {}\n", format_args!($($arg)*)))
    };
}

#[doc(hidden)]
pub fn _log(args: core::fmt::Arguments) {
    without_interrupts(|| {
        if let Some(writer) = LOGGER.locked().get_mut() {
            writer.write_fmt(args).unwrap();
        }
    });
}
