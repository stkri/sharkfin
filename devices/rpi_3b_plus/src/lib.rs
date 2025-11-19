#![no_std]

use core::fmt::Write;
use driver_gpio_bcm2837::gpio_types::{GPIOIn, GPIOOut};
use driver_gpio_bcm2837::impls::gpio_in::InputPin;
use driver_gpio_bcm2837::impls::gpio_out::{OutputPin, StatefulOutputPin};
use driver_uart_bcm2837_pl011::impls::uart::{SerialInput, UART};

#[derive(Clone, Copy)]
pub enum GPIO {
    In(GPIOIn),
    Out(GPIOOut),
    Reserved,
    Uninit,
}

pub struct Device {
    uart: UART,
    led: Option<GPIOOut>,
    gpios: [GPIO; 54],
}

pub const BAUD_RATE: u32 = 115200;
pub const LED_PIN: u8 = 29;

impl Device {
    pub fn new() -> Self {
        let mut uart = unsafe { UART::new(BAUD_RATE) };

        writeln!(uart, "SHARKFIN 0.1.0").unwrap();
        writeln!(uart, "[   OK   ]\t\tInitialized UART").unwrap();

        let led = match unsafe { GPIOOut::new(29) } {
            Ok(pin) => {
                writeln!(uart, "[   OK   ]\t\tLED initialized").unwrap();
                Some(pin)
            }
            Err(e) => {
                writeln!(uart, "[ FAILED ]\tLED could not be initialised\n").unwrap();
                writeln!(uart, "[ REASON ]\t{}", e).unwrap();
                None
            }
        };

        let mut gpios: [GPIO; 54] = [GPIO::Uninit; 54];

        for pin in 0..=53 {
            match pin {
                14 | 15 | 29 => gpios[pin] = GPIO::Reserved,
                n => {
                    let pin = match unsafe { GPIOOut::new(n as u8) } {
                        Ok(pin) => {
                            writeln!(uart, "[   OK   ]\t\tGPIO pin {} initialized", n).unwrap();
                            GPIO::Out(pin)
                        }
                        Err(e) => {
                            writeln!(uart, "[ FAILED ]\tGPIO pin {} could not be initialised", n)
                                .unwrap();
                            writeln!(uart, "[ REASON ]\t{}", e).unwrap();
                            GPIO::Uninit
                        }
                    };
                    gpios[n] = pin;
                }
            }
        }

        Self { uart, led, gpios }
    }

    pub fn run_dsh(&mut self) {
        let mut buf = [0x0u8; 64];
        loop {
            write!(self.uart, "[dsh] > ").unwrap();
            let pos = self.read_input(&mut buf);

            let cmd_end = find_next_space(&buf, 0, pos);
            let cmd = &buf[0..cmd_end];
            let args = if cmd_end < pos {
                &buf[cmd_end + 1..pos]
            } else {
                &[]
            };

            self.execute_command(cmd, args);
        }
    }

    fn read_input(&mut self, buf: &mut [u8; 64]) -> usize {
        let mut pos = 0;
        loop {
            let c = self.uart.read_byte().unwrap();
            match c {
                b'\0' => {}
                b'\n' | b'\r' => {
                    writeln!(self.uart, "").unwrap();
                    return pos;
                }
                b'\x08' | b'\x7F' => {
                    if pos == 0 {
                        write!(self.uart, "\x07").unwrap();
                    } else {
                        write!(self.uart, "\x08 \x08").unwrap();
                        pos -= 1;
                    }
                }
                c if pos < buf.len() => {
                    write!(self.uart, "{}", c as char).unwrap();
                    buf[pos] = c;
                    pos += 1;
                }
                _ => write!(self.uart, "\x07").unwrap(),
            }
        }
    }

    fn execute_command(&mut self, cmd: &[u8], args: &[u8]) {
        match cmd {
            [] => {}
            b"c" | b"clear" => self.cmd_clear(),
            b"p" | b"panic" => self.cmd_panic(args),
            b"e" | b"echo" => self.cmd_echo(args),
            b"l" | b"led" => self.cmd_led(args),
            b"g" | b"gpio" => self.cmd_gpio(args),
            b"h" | b"help" => self.cmd_help(args),
            _ => writeln!(
                self.uart,
                "No such command: {}",
                core::str::from_utf8(cmd).unwrap_or("malformed UTF-8 string")
            )
            .unwrap(),
        }
    }

    fn cmd_clear(&mut self) {
        for _ in 0..100 {
            writeln!(self.uart, "").unwrap();
        }
    }

    fn cmd_panic(&mut self, args: &[u8]) {
        if args.is_empty() {
            panic!("User initiated panic");
        } else {
            panic!(
                "{}",
                core::str::from_utf8(args).unwrap_or("User initiated panic")
            );
        }
    }

    fn cmd_echo(&mut self, args: &[u8]) {
        if args.is_empty() {
            writeln!(self.uart, "").unwrap();
        } else {
            writeln!(
                self.uart,
                "{}",
                core::str::from_utf8(args).unwrap_or(
                    "Error: malformed UTF-8 string"
                )
            )
            .unwrap();
        }
    }

    fn cmd_help(&mut self, args: &[u8]) {
        if args.is_empty() {
            writeln!(self.uart, "Commands:").unwrap();
            writeln!(self.uart, "\tclear (c)\tClear screen").unwrap();
            writeln!(self.uart, "\techo (e)\tEcho text").unwrap();
            writeln!(self.uart, "\thelp (h)\tShow this text").unwrap();
            writeln!(self.uart, "\tled (l)\t\tControl LED").unwrap();
            writeln!(self.uart, "\tgpio (g)\tControl GPIO").unwrap();
            writeln!(self.uart, "\tpanic (p)\tTrigger panic").unwrap();
            writeln!(self.uart).unwrap();
            writeln!(
                self.uart,
                "To get options for specific commands, use: help [command]"
            )
            .unwrap();
        } else {
            match &args[0..find_next_space(args, 0, args.len())] {
                b"c" | b"clear" => {
                    writeln!(self.uart, "\tclear (c)").unwrap();
                    writeln!(self.uart, "").unwrap();
                    writeln!(self.uart, "Clear the screen").unwrap();
                }
                b"p" | b"panic" => {
                    writeln!(self.uart, "\tpanic (p) [msg]").unwrap();
                    writeln!(self.uart, "").unwrap();
                    writeln!(self.uart, "Trigger a kernel panic with a message.").unwrap();
                    writeln!(self.uart, "The default message is \"User initiated panic\"").unwrap();
                }
                b"e" | b"echo" => {
                    writeln!(self.uart, "\techo (e) <msg>").unwrap();
                    writeln!(self.uart, "").unwrap();
                    writeln!(self.uart, "Echo a message").unwrap();
                }
                b"l" | b"led" => {
                    writeln!(self.uart, "\tled (l) <on|off|toggle|state>").unwrap();
                    writeln!(self.uart, "").unwrap();
                    writeln!(self.uart, "Control the on-board activity LED.").unwrap();
                    writeln!(self.uart, "Subcommands: ").unwrap();
                    writeln!(self.uart, "\ton (h)\t\tturn the led on").unwrap();
                    writeln!(self.uart, "\toff (l)\t\tturn the led off").unwrap();
                    writeln!(self.uart, "\ttoggle (t)\ttoggle the led").unwrap();
                    writeln!(self.uart, "\tstate (s)\tprint the current state").unwrap();
                    writeln!(self.uart, "\thigh\t\talias for on").unwrap();
                    writeln!(self.uart, "\tlow\t\talias for off").unwrap();
                }
                b"g" | b"gpio" => {
                    writeln!(self.uart, "\tgpio (g) <pin> <in|out|state> [args]").unwrap();
                    writeln!(self.uart, "Control the chosen GPIO pin").unwrap();
                    writeln!(self.uart, "").unwrap();
                    writeln!(self.uart, "\tgpio <pin> in (i)").unwrap();
                    writeln!(self.uart, "Set the pin for input and get voltage level").unwrap();
                    writeln!(self.uart, "").unwrap();
                    writeln!(self.uart, "\tgpio <pin> out (o) <on|off|toggle>").unwrap();
                    writeln!(self.uart, "Set the pin for output and set voltage").unwrap();
                    writeln!(self.uart, "Subcommands: ").unwrap();
                    writeln!(self.uart, "\ton (h)\t\tset the voltage high").unwrap();
                    writeln!(self.uart, "\toff (l)\t\tset the voltage low").unwrap();
                    writeln!(self.uart, "\ttoggle (t)\ttoggle the voltage").unwrap();
                    writeln!(self.uart, "\thigh\t\talias for on").unwrap();
                    writeln!(self.uart, "\tlow\t\talias for off").unwrap();
                    writeln!(self.uart).unwrap();
                    writeln!(self.uart, "\tgpio <pin> state (s)").unwrap();
                    writeln!(self.uart, "Print the current pin state").unwrap();
                }
                _ => self.cmd_help(b""),
            }
        }
    }

    fn cmd_led(&mut self, args: &[u8]) {
        if self.led.is_none() {
            writeln!(self.uart, "Error: LED uninitialised").unwrap();
            return;
        }

        let subcmd_end = find_next_space(args, 0, args.len());
        let subcmd = &args[0..subcmd_end];

        if subcmd.is_empty() {
            writeln!(self.uart, "Subcommand required").unwrap();
            writeln!(self.uart, "Usage: led [on|off|toggle|state]").unwrap();
            return;
        }

        let led = self.led.as_mut().unwrap();

        match subcmd {
            b"h" | b"high" | b"on" => led.set_high().unwrap(),
            b"l" | b"low" | b"off" => led.set_low().unwrap(),
            b"t" | b"toggle" => led.toggle().unwrap(),
            b"s" | b"state" => match led.is_set_high() {
                Ok(true) => writeln!(self.uart, "ON").unwrap(),
                Ok(false) => writeln!(self.uart, "OFF").unwrap(),
                Err(e) => writeln!(self.uart, "Error: {}", e).unwrap(),
            },
            _ => {
                writeln!(
                    self.uart,
                    "Unknown subcommand: {}",
                    core::str::from_utf8(subcmd).unwrap_or("malformed UTF-8 string")
                )
                .unwrap();
                writeln!(self.uart, "Usage: led [on|off|toggle|state]").unwrap();
            }
        }
    }

    fn cmd_gpio(&mut self, args: &[u8]) {
        let pin_id = match self.parse_gpio_pin(args) {
            Some(id) => id,
            None => return,
        };

        let pin_end = find_next_space(args, 0, args.len());
        let subcmd_start = if pin_end < args.len() {
            pin_end + 1
        } else {
            pin_end
        };
        let subcmd_end = find_next_space(args, subcmd_start, args.len());
        let subcmd = &args[subcmd_start..subcmd_end];

        if subcmd.is_empty() {
            writeln!(self.uart, "Subcommand required").unwrap();
            writeln!(self.uart, "Usage: gpio <pin> [in|out|state]").unwrap();
            return;
        }

        match subcmd {
            b"i" | b"in" => self.gpio_read_input(pin_id),
            b"o" | b"out" => {
                let output_args = if subcmd_end < args.len() {
                    &args[subcmd_end + 1..]
                } else {
                    &[]
                };
                self.gpio_write_output(pin_id, output_args);
            }
            b"s" | b"state" => self.gpio_show_state(pin_id),
            _ => {
                writeln!(
                    self.uart,
                    "Unknown GPIO subcommand: {}",
                    core::str::from_utf8(subcmd).unwrap_or("malformed UTF-8")
                )
                .unwrap();
                writeln!(self.uart, "Usage: gpio <pin> [in|out|state]").unwrap();
            }
        }
    }

    fn parse_gpio_pin(&mut self, args: &[u8]) -> Option<usize> {
        if args.is_empty() {
            writeln!(self.uart, "Missing pin ID").unwrap();
            return None;
        }

        let pin_end = find_next_space(args, 0, args.len());
        let pin_str = match core::str::from_utf8(&args[0..pin_end]) {
            Err(_) => {
                writeln!(self.uart, "Error: malformed UTF-8 string").unwrap();
                return None;
            }
            Ok(s) => s,
        };

        let pin_id = match pin_str.parse::<usize>() {
            Err(_) => {
                writeln!(self.uart, "Error: expected numeric argument").unwrap();
                return None;
            }
            Ok(i) => i,
        };

        if pin_id > 53 {
            writeln!(self.uart, "Error: pin {} doesn't exist", pin_id).unwrap();
            return None;
        }

        Some(pin_id)
    }

    fn gpio_read_input(&mut self, pin_id: usize) {
        match self.gpios[pin_id] {
            GPIO::Reserved => {
                writeln!(self.uart, "Error: Pin {} is reserved", pin_id).unwrap();
            }
            GPIO::Uninit => {
                writeln!(self.uart, "Error: Pin {} is uninitialized", pin_id).unwrap();
            }
            GPIO::Out(pin) => match GPIOIn::try_from(pin) {
                Err(e) => writeln!(self.uart, "Error: {}", e).unwrap(),
                Ok(input_pin) => {
                    self.read_pin_state(input_pin);
                    self.gpios[pin_id] = GPIO::In(input_pin);
                }
            },
            GPIO::In(pin) => {
                self.read_pin_state(pin);
            }
        }
    }

    fn gpio_write_output(&mut self, pin_id: usize, args: &[u8]) {
        if args.is_empty() {
            writeln!(self.uart, "Output command required").unwrap();
            writeln!(self.uart, "Usage: gpio <pin> out [on|off|toggle]").unwrap();
            return;
        }

        let cmd_end = find_next_space(args, 0, args.len());
        let cmd = &args[0..cmd_end];

        let mut output_pin = match self.gpios[pin_id] {
            GPIO::Reserved => {
                writeln!(self.uart, "Error: Pin {} is reserved", pin_id).unwrap();
                return;
            }
            GPIO::Uninit => {
                writeln!(self.uart, "Error: Pin {} is uninitialized", pin_id).unwrap();
                return;
            }
            GPIO::In(pin) => match GPIOOut::try_from(pin) {
                Err(e) => {
                    writeln!(self.uart, "Error: {}", e).unwrap();
                    return;
                }
                Ok(out_pin) => {
                    self.gpios[pin_id] = GPIO::Out(out_pin);
                    out_pin
                }
            },
            GPIO::Out(pin) => pin,
        };

        match cmd {
            b"h" | b"high" | b"on" => {
                if let Err(e) = output_pin.set_high() {
                    writeln!(self.uart, "Error: {}", e).unwrap();
                } else {
                    self.gpios[pin_id] = GPIO::Out(output_pin);
                }
            }
            b"l" | b"low" | b"off" => {
                if let Err(e) = output_pin.set_low() {
                    writeln!(self.uart, "Error: {}", e).unwrap();
                } else {
                    self.gpios[pin_id] = GPIO::Out(output_pin);
                }
            }
            b"t" | b"toggle" => {
                if let Err(e) = output_pin.toggle() {
                    writeln!(self.uart, "Error: {}", e).unwrap();
                } else {
                    self.gpios[pin_id] = GPIO::Out(output_pin);
                }
            }
            _ => writeln!(
                self.uart,
                "Unknown output command: {}",
                core::str::from_utf8(cmd).unwrap_or("malformed UTF-8")
            )
            .unwrap(),
        }
    }

    fn gpio_show_state(&mut self, pin_id: usize) {
        match self.gpios[pin_id] {
            GPIO::Reserved => {
                writeln!(self.uart, "Pin {} is reserved", pin_id).unwrap();
            }
            GPIO::Uninit => {
                writeln!(self.uart, "Pin {} is uninitialized", pin_id).unwrap();
            }
            GPIO::In(_) => {
                writeln!(self.uart, "Pin {} is in INPUT mode", pin_id).unwrap();
            }
            GPIO::Out(pin) => {
                write!(self.uart, "Pin {} is in OUTPUT mode, state: ", pin_id).unwrap();
                match pin.is_set_high() {
                    Ok(true) => writeln!(self.uart, "HIGH").unwrap(),
                    Ok(false) => writeln!(self.uart, "LOW").unwrap(),
                    Err(e) => writeln!(self.uart, "Error: {}", e).unwrap(),
                }
            }
        }
    }

    fn read_pin_state(&mut self, pin: GPIOIn) {
        match pin.is_high() {
            Ok(true) => writeln!(self.uart, "HIGH").unwrap(),
            Ok(false) => writeln!(self.uart, "LOW").unwrap(),
            Err(e) => writeln!(self.uart, "Error: {}", e).unwrap(),
        }
    }
}

fn find_next_space(buf: &[u8], start: usize, end: usize) -> usize {
    let mut i = start;
    while i < end && buf[i] != b' ' {
        i += 1;
    }
    i
}
