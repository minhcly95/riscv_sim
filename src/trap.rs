use crate::Exception;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Trap {
    pub cause: TrapCause,
    pub val: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TrapCause {
    Exception(Exception),
}

impl Trap {
    pub fn from_code(cause: u32, val: u32) -> Option<Trap> {
        let cause = TrapCause::from(cause)?;
        Some(Trap { cause, val })
    }

    pub fn from_exception(ex: Exception, val: u32) -> Trap {
        let cause = TrapCause::Exception(ex);
        Trap { cause, val }
    }
}

impl TrapCause {
    pub fn from(code: u32) -> Option<TrapCause> {
        Some(TrapCause::Exception(Exception::from(code)?))
    }

    pub fn to_int(&self) -> u32 {
        match self {
            TrapCause::Exception(e) => e.to_int(),
        }
    }
}
