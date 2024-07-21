use crate::{
    sys::control::{Control, MPriv, SPriv, TvecMode},
    trap::TrapCause,
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

// ------------ Interrupt condition -------------
pub fn update_interrupt(sys: &mut System) {
    // Update timer interrupt
    sys.ctrl.ip.set(&Interrupt::MTimer, sys.mem.timer.is_interrupt_set());
}

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

// ---------------- Trap stack ------------------
pub fn push_trap_m(sys: &mut System, trap: Trap) {
    // Save previous status
    sys.ctrl.mpie = sys.ctrl.mie;
    sys.ctrl.mpp = sys.ctrl.privilege;
    // Push new status
    sys.ctrl.mie = false;
    sys.ctrl.privilege = MPriv::M;
    // Trap information
    sys.ctrl.mepc = sys.pc();
    sys.ctrl.mtrap = trap;
    // Jump to trap vector
    *sys.pc_mut() = trap_vector_addr(&sys.ctrl.mtrap, sys.ctrl.mtvec_base, &sys.ctrl.mtvec_mode);
}

pub fn pop_trap_m(sys: &mut System) {
    // Restore previous status
    sys.ctrl.mie = sys.ctrl.mpie;
    sys.ctrl.privilege = sys.ctrl.mpp;
    // Push dummy status
    sys.ctrl.mpie = true;
    sys.ctrl.mpp = MPriv::U;
    // If move to a less privilege mode, clear MPRV
    if sys.ctrl.privilege != MPriv::M {
        sys.ctrl.mprv = false;
    }
    // Jump back to original PC
    *sys.pc_mut() = sys.ctrl.mepc;
    // Also clear LR reservation
    sys.mem.clear_reservation();
}

pub fn push_trap_s(sys: &mut System, trap: Trap) {
    // Save previous status
    sys.ctrl.spie = sys.ctrl.sie;
    sys.ctrl.spp = SPriv::from_m(sys.ctrl.privilege).expect("Cannot trap to S-mode from M-mode");
    // Push new status
    sys.ctrl.sie = false;
    sys.ctrl.privilege = MPriv::S;
    // Trap information
    sys.ctrl.sepc = sys.pc();
    sys.ctrl.strap = trap;
    // Jump to trap vector
    *sys.pc_mut() = trap_vector_addr(&sys.ctrl.strap, sys.ctrl.stvec_base, &sys.ctrl.stvec_mode);
}

pub fn pop_trap_s(sys: &mut System) {
    // Restore previous status
    sys.ctrl.sie = sys.ctrl.spie;
    sys.ctrl.privilege = MPriv::from_s(sys.ctrl.spp);
    // Push dummy status
    sys.ctrl.spie = true;
    sys.ctrl.spp = SPriv::U;
    // Clear MPRV (since SRET always change the privilege mode to either S or U)
    sys.ctrl.mprv = false;
    // Jump back to original PC
    *sys.pc_mut() = sys.ctrl.sepc;
    // Also clear LR reservation
    sys.mem.clear_reservation();
}

// -------------- Trap handling -----------------
pub fn handle_trap(sys: &mut System, trap: Trap) {
    let Control {
        privilege,
        medeleg,
        mideleg,
        ..
    } = &sys.ctrl;
    // Determine the mode (M or S) to handle the trap
    match trap.cause {
        TrapCause::Exception(ex) => {
            if *privilege == MPriv::M || !medeleg.get(&ex) {
                push_trap_m(sys, trap);
            } else {
                push_trap_s(sys, trap);
            }
        }
        TrapCause::Interrupt(int) => {
            if *privilege == MPriv::M || !mideleg.get(&int) {
                push_trap_m(sys, trap);
            } else {
                push_trap_s(sys, trap);
            }
        }
    }
}

pub fn trap_vector_addr(trap: &Trap, base: u32, mode: &TvecMode) -> u32 {
    match mode {
        TvecMode::Direct => base,
        TvecMode::Vectored => match trap.cause {
            TrapCause::Exception(_) => base,
            TrapCause::Interrupt(int) => base + (int.to_int() << 2),
        },
    }
}
