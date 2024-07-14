use crate::{
    sys::{log_with_pc, mem_map::dtb::Dtb},
    trap::TrapCause,
    Exception, System, Trap,
};
use colored::*;
use std::{fs, io, path::Path};

pub fn load_image_from_file<P>(sys: &mut System, file_name: P, addr: u64) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let image = fs::read(file_name)?;
    let len = image.len();
    let addr = addr as usize;
    let end = addr + len;
    log_with_pc(
        sys,
        &format!("{} with {len} bytes", "Load image".blue()),
        false,
    );
    sys.mem.ram.as_u8_mut()[addr..end].copy_from_slice(&image);
    Ok(())
}

pub fn load_dtb_from_file<P>(sys: &mut System, file_name: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let mut image = fs::read(file_name)?;
    let len = image.len();
    log_with_pc(
        sys,
        &format!("{} with {len} bytes", "Load dtb".blue()),
        false,
    );
    while image.len() % 4 != 0 {
        image.push(0);
    }
    sys.mem.dtb = Dtb::new(image);
    Ok(())
}

pub fn run_until_trapped(sys: &mut System) -> Trap {
    loop {
        if let Err(trap) = sys.step() {
            log_with_pc(
                sys,
                &format!(
                    "{} due to a trap: {}",
                    "Break".yellow(),
                    format!("{:?}", trap).yellow()
                ),
                false,
            );
            return trap;
        }
    }
}

pub fn run_until_ecall(sys: &mut System) {
    loop {
        if let Err(Trap {
            cause: TrapCause::Exception(ex),
            ..
        }) = sys.step()
        {
            if ex == Exception::EcallFromM
                || ex == Exception::EcallFromS
                || ex == Exception::EcallFromU
            {
                log_with_pc(
                    sys,
                    &format!(
                        "{} due to an Ecall: {}",
                        "Break".yellow(),
                        format!("{:?}", ex).yellow()
                    ),
                    false,
                );
                return;
            }
        }
    }
}

pub fn run_for(sys: &mut System, repeat: usize) {
    for _ in 0..repeat {
        let _ = sys.step();
    }
}

pub fn run_forever(sys: &mut System) {
    loop {
        let _ = sys.step();
    }
}

pub fn run_for_or_until_ecall(sys: &mut System, repeat: usize) -> Result<(), Exception> {
    for _ in 0..repeat {
        if let Err(Trap {
            cause: TrapCause::Exception(ex),
            ..
        }) = sys.step()
        {
            if ex == Exception::EcallFromM
                || ex == Exception::EcallFromS
                || ex == Exception::EcallFromU
            {
                log_with_pc(
                    sys,
                    &format!(
                        "{} due to an exception: {}",
                        "Break".yellow(),
                        format!("{:?}", ex).yellow()
                    ),
                    false,
                );
                return Err(ex);
            }
        }
    }
    Ok(())
}
