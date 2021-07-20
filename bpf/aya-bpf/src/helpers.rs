use core::mem::{self, MaybeUninit};

use aya_bpf_bindings::helpers as gen;
pub use gen::*;

use crate::cty::{c_char, c_long, c_void};

#[inline]
pub unsafe fn bpf_probe_read<T>(src: *const T) -> Result<T, c_long> {
    let mut v: MaybeUninit<T> = MaybeUninit::uninit();
    let ret = gen::bpf_probe_read(
        v.as_mut_ptr() as *mut c_void,
        mem::size_of::<T>() as u32,
        src as *const c_void,
    );
    if ret < 0 {
        return Err(ret);
    }

    Ok(v.assume_init())
}

#[inline]
pub unsafe fn bpf_probe_read_to_dst<T>(&mut dst: T, src: *const T) -> Result<(), c_long> {
    let ret = gen::bpf_probe_read(
        &mut dst as *mut _ as *mut c_void,
        mem::size_of::<T>() as u32,
        src as *const c_void,
    );
    if ret < 0 {
        return Err(ret);
    }

    Ok(())
}

#[inline]
pub fn bpf_get_current_comm() -> Result<[c_char; 16], ()> {
    let mut comm: [c_char; 16usize] = [0; 16];
    if unsafe { gen::bpf_get_current_comm(&mut comm as *mut _ as *mut c_void, 16u32) } == 0 {
        Ok(comm)
    } else {
        Err(())
    }
}

#[inline]
pub fn bpf_get_current_pid_tgid() -> u64 {
    unsafe { gen::bpf_get_current_pid_tgid() }
}
