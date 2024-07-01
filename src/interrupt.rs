use crate::{
    sys::control::{Control, MPriv},
    Interrupt, Result, System, Trap,
};

const INTERRUPT_ORDER: [Interrupt; 6] = [
    Interrupt::MExt,
    Interrupt::MSoft,
    Interrupt::MTimer,
    Interrupt::SExt,
    Interrupt::SSoft,
    Interrupt::STimer,
];

pub fn check_interrupt(sys: &mut System) -> Result {
    let cond_m = int_cond_m(sys);
    let cond_s = int_cond_s(sys);
    let Control {
        ie, ip, mideleg, ..
    } = &sys.ctrl;
    for int in INTERRUPT_ORDER {
        if ie.get(&int) & ip.get(&int) {
            if mideleg.get(&int) {
                // Delegated, consider S-mode condition
                if cond_s {
                    return Err(Trap::from_interrupt(int, 0));
                }
            } else {
                // Not delegated, consider M-mode condition
                if cond_m {
                    return Err(Trap::from_interrupt(int, 0));
                }
            }
        }
    }
    Ok(())
}

fn int_cond_m(sys: &System) -> bool {
    sys.ctrl.privilege != MPriv::M || sys.ctrl.mie
}

fn int_cond_s(sys: &System) -> bool {
    sys.ctrl.privilege == MPriv::U || sys.ctrl.privilege == MPriv::S && sys.ctrl.sie
}
