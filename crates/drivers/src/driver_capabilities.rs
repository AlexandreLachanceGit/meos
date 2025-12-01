use crate::driver::Driver;

pub trait UartDriver: Driver {
    fn set_baud(&mut self, baud: u32);
    fn put_char(&mut self, c: char);
    fn get_char(&mut self) -> Option<char>;
}
