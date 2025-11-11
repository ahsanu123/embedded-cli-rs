#[macro_use]
extern crate cli_macros;
enum BaseCommand {
    Status,
    Led { pin: u8 },
    Adc { ping: u8, time: u8 },
}
struct CliHandle;
trait BaseCommandTrait {
    fn handle_status(&mut self, handle_fn: Option<fn()>);
    fn handle_led(&mut self, handle_fn: Option<fn(u8)>);
    fn handle_adc(&mut self, handle_fn: Option<fn(u8, u8)>);
}
impl BaseCommandTrait for CliHandle {
    fn handle_status(&mut self, handle_fn: Option<fn()>) {
        ::core::panicking::panic("not yet implemented")
    }
    fn handle_led(&mut self, handle_fn: Option<fn(u8)>) {
        ::core::panicking::panic("not yet implemented")
    }
    fn handle_adc(&mut self, handle_fn: Option<fn(u8, u8)>) {
        ::core::panicking::panic("not yet implemented")
    }
}
