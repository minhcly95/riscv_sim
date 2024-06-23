use super::Result;
use crate::{Exception, System};

pub fn execute_system(_sys: &mut System) -> Result {
    Err(Exception::EcallFromM)
}
