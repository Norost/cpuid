#![no_std]

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::__cpuid;
#[cfg(target_arch = "x86")]
use core::arch::x86::__cpuid;
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
compile_error!("CPUID is only supported on x86 and x86_64");

pub struct Cpuid {
	max_basic_eax: u32,
	max_hypervisor_eax: u32,
	max_extended_eax: u32,
}

macro_rules! flag {
	(basic $flag:ident = $id:literal | $reg:ident[$bit:literal]) => {
		pub fn $flag(&self) -> bool {
			let id: u32 = $id;
			// SAFETY: id is in range
			id <= self.max_basic_eax && unsafe { (__cpuid(id).$reg & 1 << $bit) != 0 }
		}
	};
	(hypervisor $flag:ident = $id:literal | $reg:ident[$bit:literal]) => {
		pub fn $flag(&self) -> bool {
			let id: u32 = $id | 1 << 30;
			// SAFETY: id is in range
			id <= self.max_hypervisor_eax && unsafe { (__cpuid(id).$reg & 1 << $bit) != 0 }
		}
	};
	(extended $flag:ident = $id:literal | $reg:ident[$bit:literal]) => {
		pub fn $flag(&self) -> bool {
			let id: u32 = $id | 2 << 30;
			// SAFETY: id is in range
			id <= self.max_extended_eax && unsafe { (__cpuid(id).$reg & 1 << $bit) != 0 }
		}
	};
}

impl Cpuid {
	pub fn new() -> Self {
		// Get maximum EAX value for basic and extended features.
		//
		// Technically we should check flags to see if CPUID is supported, but basically every
		// CPU in use for anything serious supports it. CPUID was introduced in 1993 after all.
		unsafe {
			Self {
				max_basic_eax: __cpuid(0 << 31).eax,
				max_hypervisor_eax: __cpuid(1 << 30).eax,
				max_extended_eax: __cpuid(2 << 30).eax,
			}
		}
	}

	// List stolen from https://sandpile.org/x86/cpuid.htm
	flag!(basic osxsave = 0x1 | ecx[26]);
	flag!(basic mtrr = 0x1 | edx[12]);
	flag!(basic fsgsbase = 0x7 | ebx[0]);
	flag!(basic avx2 = 0x7 | ebx[5]);

	flag!(extended pdpe1gb = 0x1 | edx[26]);
	flag!(extended invariant_tsc = 0x7 | edx[8]);

	// https://docs.kernel.org/virt/kvm/x86/cpuid.html
	flag!(hypervisor kvm_feature_clocksource2 = 0x1 | eax[3]);
}
