#[macro_use]
extern crate cli_macros;

#[derive(MainCommand)]
enum BaseCommand {
    Status,
    Led { pin: u8 },
    Adc { ping: u8, time: u8 },
}
