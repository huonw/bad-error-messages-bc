extern crate a;

pub struct Bar;

impl a::Foo for Bar {
    fn bar(&self) {}
}
