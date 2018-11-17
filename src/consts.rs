pub const KVM_CAP_IRQCHIP: i32 = 0;
pub const KVM_CAP_HLT: i32 = 1;
pub const KVM_CAP_MMU_SHADOW_CACHE_CONTROL: i32 = 2;
pub const KVM_CAP_USER_MEMORY: i32 = 3;
pub const KVM_CAP_SET_TSS_ADDR: i32 = 4;
pub const KVM_CAP_VAPIC: i32 = 6;
pub const KVM_CAP_EXT_CPUID: i32 = 7;
pub const KVM_CAP_CLOCKSOURCE: i32 = 8;
pub const KVM_CAP_NR_VCPUS: i32 = 9;
pub const KVM_CAP_NR_MEMSLOTS: i32 = 10;
pub const KVM_CAP_PIT: i32 = 11;
pub const KVM_CAP_NOP_IO_DELAY: i32 = 12;
pub const KVM_CAP_PV_MMU: i32 = 13;
pub const KVM_CAP_MP_STATE: i32 = 14;
pub const KVM_CAP_COALESCED_MMIO: i32 = 15;
pub const KVM_CAP_SYNC_MMU: i32 = 16;
pub const KVM_CAP_IOMMU: i32 = 18;
pub const KVM_CAP_DESTROY_MEMORY_REGION_WORKS: i32 = 21;
pub const KVM_CAP_USER_NMI: i32 = 22;
pub const KVM_CAP_IRQ_ROUTING: i32 = 25;
pub const KVM_CAP_IRQ_INJECT_STATUS: i32 = 26;
pub const KVM_CAP_ASSIGN_DEV_IRQ: i32 = 29;
pub const KVM_CAP_JOIN_MEMORY_REGIONS_WORKS: i32 = 30;
pub const KVM_CAP_IRQFD: i32 = 32;
pub const KVM_CAP_PIT2: i32 = 33;
pub const KVM_CAP_SET_BOOT_CPU_ID: i32 = 34;
pub const KVM_CAP_IOEVENTFD: i32 = 36;
pub const KVM_CAP_SET_IDENTITY_MAP_ADDR: i32 = 37;
pub const KVM_CAP_XEN_HVM: i32 = 38;
pub const KVM_CAP_ADJUST_CLOCK: i32 = 39;
pub const KVM_CAP_INTERNAL_ERROR_DATA: i32 = 40;
pub const KVM_CAP_VCPU_EVENTS: i32 = 41;
pub const KVM_CAP_PPC_SEGSTATE: i32 = 43;
pub const KVM_CAP_HYPERV: i32 = 44;
pub const KVM_CAP_HYPERV_VAPIC: i32 = 45;
pub const KVM_CAP_HYPERV_SPIN: i32 = 46;
pub const KVM_CAP_PCI_SEGMENT: i32 = 47;
pub const KVM_CAP_PPC_PAIRED_SINGLES: i32 = 48;
pub const KVM_CAP_INTR_SHADOW: i32 = 49;
pub const KVM_CAP_X86_ROBUST_SINGLESTEP: i32 = 51;
pub const KVM_CAP_PPC_OSI: i32 = 52;
pub const KVM_CAP_PPC_UNSET_IRQ: i32 = 53;
pub const KVM_CAP_ENABLE_CAP: i32 = 54;
pub const KVM_CAP_XSAVE: i32 = 55;
pub const KVM_CAP_XCRS: i32 = 56;
pub const KVM_CAP_PPC_GET_PVINFO: i32 = 57;
pub const KVM_CAP_PPC_IRQ_LEVEL: i32 = 58;
pub const KVM_CAP_ASYNC_PF: i32 = 59;
pub const KVM_CAP_TSC_CONTROL: i32 = 60;
pub const KVM_CAP_GET_TSC_KHZ: i32 = 61;
pub const KVM_CAP_PPC_BOOKE_SREGS: i32 = 62;
pub const KVM_CAP_SPAPR_TCE: i32 = 63;
pub const KVM_CAP_PPC_SMT: i32 = 64;
pub const KVM_CAP_PPC_RMA: i32 = 65;
pub const KVM_CAP_MAX_VCPUS: i32 = 66;
pub const KVM_CAP_PPC_HIOR: i32 = 67;
pub const KVM_CAP_PPC_PAPR: i32 = 68;
pub const KVM_CAP_SW_TLB: i32 = 69;
pub const KVM_CAP_ONE_REG: i32 = 70;
pub const KVM_CAP_S390_GMAP: i32 = 71;
pub const KVM_CAP_TSC_DEADLINE_TIMER: i32 = 72;
pub const KVM_CAP_S390_UCONTROL: i32 = 73;
pub const KVM_CAP_SYNC_REGS: i32 = 74;
pub const KVM_CAP_PCI_2_3: i32 = 75;
pub const KVM_CAP_KVMCLOCK_CTRL: i32 = 76;
pub const KVM_CAP_SIGNAL_MSI: i32 = 77;
pub const KVM_CAP_PPC_GET_SMMU_INFO: i32 = 78;
pub const KVM_CAP_S390_COW: i32 = 79;
pub const KVM_CAP_PPC_ALLOC_HTAB: i32 = 80;
pub const KVM_CAP_READONLY_MEM: i32 = 81;
pub const KVM_CAP_IRQFD_RESAMPLE: i32 = 82;
pub const KVM_CAP_PPC_BOOKE_WATCHDOG: i32 = 83;
pub const KVM_CAP_PPC_HTAB_FD: i32 = 84;
pub const KVM_CAP_S390_CSS_SUPPORT: i32 = 85;
pub const KVM_CAP_PPC_EPR: i32 = 86;
pub const KVM_CAP_ARM_PSCI: i32 = 87;
pub const KVM_CAP_ARM_SET_DEVICE_ADDR: i32 = 88;
pub const KVM_CAP_DEVICE_CTRL: i32 = 89;
pub const KVM_CAP_IRQ_MPIC: i32 = 90;
pub const KVM_CAP_PPC_RTAS: i32 = 91;
pub const KVM_CAP_IRQ_XICS: i32 = 92;
pub const KVM_CAP_ARM_EL1_32BIT: i32 = 93;
pub const KVM_CAP_SPAPR_MULTITCE: i32 = 94;
pub const KVM_CAP_EXT_EMUL_CPUID: i32 = 95;
pub const KVM_CAP_HYPERV_TIME: i32 = 96;
pub const KVM_CAP_IOAPIC_POLARITY_IGNORED: i32 = 97;
pub const KVM_CAP_ENABLE_CAP_VM: i32 = 98;
pub const KVM_CAP_S390_IRQCHIP: i32 = 99;
pub const KVM_CAP_IOEVENTFD_NO_LENGTH: i32 = 100;
pub const KVM_CAP_VM_ATTRIBUTES: i32 = 101;
pub const KVM_CAP_ARM_PSCI_0_2: i32 = 102;
pub const KVM_CAP_PPC_FIXUP_HCALL: i32 = 103;
pub const KVM_CAP_PPC_ENABLE_HCALL: i32 = 104;
pub const KVM_CAP_CHECK_EXTENSION_VM: i32 = 105;
pub const KVM_CAP_S390_USER_SIGP: i32 = 106;
pub const KVM_CAP_S390_VECTOR_REGISTERS: i32 = 107;
pub const KVM_CAP_S390_MEM_OP: i32 = 108;
pub const KVM_CAP_S390_USER_STSI: i32 = 109;
pub const KVM_CAP_S390_SKEYS: i32 = 110;
pub const KVM_CAP_MIPS_FPU: i32 = 111;
pub const KVM_CAP_MIPS_MSA: i32 = 112;
pub const KVM_CAP_S390_INJECT_IRQ: i32 = 113;
pub const KVM_CAP_S390_IRQ_STATE: i32 = 114;
pub const KVM_CAP_PPC_HWRNG: i32 = 115;
pub const KVM_CAP_DISABLE_QUIRKS: i32 = 116;
pub const KVM_CAP_X86_SMM: i32 = 117;
pub const KVM_CAP_MULTI_ADDRESS_SPACE: i32 = 118;
pub const KVM_CAP_GUEST_DEBUG_HW_BPS: i32 = 119;
pub const KVM_CAP_GUEST_DEBUG_HW_WPS: i32 = 120;
pub const KVM_CAP_SPLIT_IRQCHIP: i32 = 121;
pub const KVM_CAP_IOEVENTFD_ANY_LENGTH: i32 = 122;
pub const KVM_CAP_HYPERV_SYNIC: i32 = 123;
pub const KVM_CAP_S390_RI: i32 = 124;
pub const KVM_CAP_SPAPR_TCE_64: i32 = 125;
pub const KVM_CAP_ARM_PMU_V3: i32 = 126;
pub const KVM_CAP_VCPU_ATTRIBUTES: i32 = 127;
pub const KVM_CAP_MAX_VCPU_ID: i32 = 128;
pub const KVM_CAP_X2APIC_API: i32 = 129;
pub const KVM_CAP_S390_USER_INSTR0: i32 = 130;
pub const KVM_CAP_MSI_DEVID: i32 = 131;
pub const KVM_CAP_PPC_HTM: i32 = 132;
pub const KVM_CAP_SPAPR_RESIZE_HPT: i32 = 133;
pub const KVM_CAP_PPC_MMU_RADIX: i32 = 134;
pub const KVM_CAP_PPC_MMU_HASH_V3: i32 = 135;
pub const KVM_CAP_IMMEDIATE_EXIT: i32 = 136;
pub const KVM_CAP_MIPS_VZ: i32 = 137;
pub const KVM_CAP_MIPS_TE: i32 = 138;
pub const KVM_CAP_MIPS_64BIT: i32 = 139;
pub const KVM_CAP_S390_GS: i32 = 140;
pub const KVM_CAP_S390_AIS: i32 = 141;
pub const KVM_CAP_SPAPR_TCE_VFIO: i32 = 142;
pub const KVM_CAP_X86_GUEST_MWAIT: i32 = 143;
pub const KVM_CAP_ARM_USER_IRQ: i32 = 144;
pub const KVM_CAP_S390_CMMA_MIGRATION: i32 = 145;
pub const KVM_CAP_PPC_FWNMI: i32 = 146;
pub const KVM_CAP_PPC_SMT_POSSIBLE: i32 = 147;
pub const KVM_CAP_HYPERV_SYNIC2: i32 = 148;
pub const KVM_CAP_HYPERV_VP_INDEX: i32 = 149;
pub const KVM_CAP_S390_AIS_MIGRATION: i32 = 150;
pub const KVM_CAP_PPC_GET_CPU_CHAR: i32 = 151;
pub const KVM_CAP_S390_BPB: i32 = 152;

pub const KVM_S390_RESET_POR: u64 = 1;
pub const KVM_S390_RESET_CLEAR: u64 = 2;
pub const KVM_S390_RESET_SUBSYSTEM: u64 = 4;
pub const KVM_S390_RESET_CPU_INIT: u64 = 8;
pub const KVM_S390_RESET_IPL: u64 = 16;
pub const KVM_EXIT_IO_IN: u8 = 0;
pub const KVM_EXIT_IO_OUT: u8 = 1;
pub const KVM_SYSTEM_EVENT_SHUTDOWN: u32 = 1;
pub const KVM_SYSTEM_EVENT_RESET: u32 = 2;
pub const KVM_SYSTEM_EVENT_CRASH: u32 = 3;
pub const KVM_EXIT_UNKNOWN: u32 = 0;
pub const KVM_EXIT_EXCEPTION: u32 = 1;
pub const KVM_EXIT_IO: u32 = 2;
pub const KVM_EXIT_HYPERCALL: u32 = 3;
pub const KVM_EXIT_DEBUG: u32 = 4;
pub const KVM_EXIT_HLT: u32 = 5;
pub const KVM_EXIT_MMIO: u32 = 6;
pub const KVM_EXIT_IRQ_WINDOW_OPEN: u32 = 7;
pub const KVM_EXIT_SHUTDOWN: u32 = 8;
pub const KVM_EXIT_FAIL_ENTRY: u32 = 9;
pub const KVM_EXIT_INTR: u32 = 10;
pub const KVM_EXIT_SET_TPR: u32 = 11;
pub const KVM_EXIT_TPR_ACCESS: u32 = 12;
pub const KVM_EXIT_S390_SIEIC: u32 = 13;
pub const KVM_EXIT_S390_RESET: u32 = 14;
pub const KVM_EXIT_DCR: u32 = 15; /* deprecated */
pub const KVM_EXIT_NMI: u32 = 16;
pub const KVM_EXIT_INTERNAL_ERROR: u32 = 17;
pub const KVM_EXIT_OSI: u32 = 18;
pub const KVM_EXIT_PAPR_HCALL: u32 = 19;
pub const KVM_EXIT_S390_UCONTROL: u32 = 20;
pub const KVM_EXIT_WATCHDOG: u32 = 21;
pub const KVM_EXIT_S390_TSCH: u32 = 22;
pub const KVM_EXIT_EPR: u32 = 23;
pub const KVM_EXIT_SYSTEM_EVENT: u32 = 24;
pub const KVM_EXIT_S390_STSI: u32 = 25;
pub const KVM_EXIT_IOAPIC_EOI: u32 = 26;
pub const KVM_EXIT_HYPERV: u32 = 27;

pub const KVM_CLOCK_TSC_STABLE: u32 = 2;
pub const KVM_PIT_SPEAKER_DUMMY: u32 = 1;

/// The vCPU is currently running.  Only supported on x86, ARM, and arm64.
pub const KVM_MP_STATE_RUNNABLE: u32 = 0;
/// The vCPU is an application processor which has not yet received an INIT
/// signal.  Only supported on x86.
pub const KVM_MP_STATE_UNINITIALIZED: u32 = 1;
/// The vCPU has received an INIT signal, and is now ready for a SIPI.
/// Only supoprted on x86.
pub const KVM_MP_STATE_INIT_RECEIVED: u32 = 2;
/// The vCPU has executed a HLT instruction and is waiting for an interrupt.
/// Only supported on x86
pub const KVM_MP_STATE_HALTED: u32 = 3;
/// The vCPU has just received a SIPI.  Only supported on x86.
pub const KVM_MP_STATE_SIPI_RECEIVED: u32 = 4;
/// The vCPU is stopped.  Only supported on s390, ARM, and arm64.
pub const KVM_MP_STATE_STOPPED: u32 = 5;
/// The vCPU is in a special error state.  Only supported on s390.
pub const KVM_MP_STATE_CHECK_STOP: u32 = 6;
/// The vCPU is operating (running or halted).  Only supported on s390.
pub const KVM_MP_STATE_OPERATING: u32 = 7;
/// The vCPU is in a special load/startup state.  Only supported on s390
pub const KVM_MP_STATE_LOAD: u32 = 8;

pub const KVM_IOEVENTFD_FLAG_DATAMATCH: u32 = 1 << 0;
pub const KVM_IOEVENTFD_FLAG_PIO: u32 = 1 << 1;
pub const KVM_IOEVENTFD_FLAG_DEASSIGN: u32 = 1 << 2;
pub const KVM_IOEVENTFD_FLAG_VIRTIO_CCW_NOTIFY: u32 = 1 << 3;

pub const KVM_IRQFD_FLAG_DEASSIGN: u32 = 1 << 0;
pub const KVM_IRQFD_FLAG_RESAMPLE: u32 = 1 << 1;
