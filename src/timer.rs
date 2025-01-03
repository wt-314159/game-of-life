use web_sys::console;

pub struct Timer<'a> {
    #[allow(dead_code)]
    name: &'a str,
}

impl<'a> Timer<'a> {
    #[allow(dead_code)]
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}