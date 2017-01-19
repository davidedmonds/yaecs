use ::{Entities, Globals};
use std::fmt::{Debug, Formatter, Result};

pub trait System {
    fn process(&self, entities: &mut Entities, globals: &mut Globals);
}

impl Debug for System {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "System")
    }
}
