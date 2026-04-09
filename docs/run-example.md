# Run example system
Just execute this command in this repository:
```
nix run
```

This creates minimal kernel, initramfs and rootfs.

## Example output
```
U-Boot SPL 2022.07 (Jan 01 1980 - 00:00:00 +0000)
Trying to boot from RAM
loading firmware...
## Verifying integrity of image...OK
## Verifying authenticity of image...OK
loading loadables...
## Verifying integrity of image...OK
## Verifying authenticity of image...OK

OpenSBI v1.8
   ____                    _____ ____ _____
  / __ \                  / ____|  _ \_   _|
 | |  | |_ __   ___ _ __ | (___ | |_) || |
 | |  | | '_ \ / _ \ '_ \ \___ \|  _ < | |
 | |__| | |_) |  __/ | | |____) | |_) || |_
  \____/| .__/ \___|_| |_|_____/|____/_____|
        | |
        |_|

Platform Name               : riscv-virtio,qemu
Platform Features           : medeleg
Platform HART Count         : 2
Platform HART Protection    : ---
Platform IPI Device         : aclint-mswi
Platform Timer Device       : aclint-mtimer @ 10000000Hz
Platform Console Device     : uart8250
Platform HSM Device         : ---
Platform PMU Device         : ---
Platform Reboot Device      : syscon-reboot
Platform Shutdown Device    : syscon-poweroff
Platform Suspend Device     : ---
Platform CPPC Device        : ---
Firmware Base               : 0x80100000
Firmware Size               : 333 KB
Firmware RW Offset          : 0x40000
Firmware RW Size            : 77 KB
Firmware Heap Offset        : 0x49000
Firmware Heap Size          : 41 KB (total), 0 KB (reserved), 12 KB (used), 27 KB (free)
Firmware Scratch Size       : 4096 B (total), 1464 B (used), 2632 B (free)
Runtime SBI Version         : 3.0
Standard SBI Extensions     : time,rfnc,ipi,base,hsm,srst,pmu,dbcn,fwft,legacy,dbtr,sse
Experimental SBI Extensions : none

Domain0 Name                : root
Domain0 Boot HART           : 0
Domain0 HARTs               : 0*,1*
Domain0 Region00            : 0x0000000080140000-0x000000008015ffff M: (F,R,W) S/U: ()
Domain0 Region01            : 0x0000000080100000-0x000000008013ffff M: (F,R,X) S/U: ()
Domain0 Region02            : 0x0000000000100000-0x0000000000100fff M: (I,R,W) S/U: (R,W)
Domain0 Region03            : 0x0000000010000000-0x0000000010000fff M: (I,R,W) S/U: (R,W)
Domain0 Region04            : 0x0000000002000000-0x000000000200ffff M: (I,R,W) S/U: ()
Domain0 Region05            : 0x000000000c400000-0x000000000c5fffff M: (I,R,W) S/U: (R,W)
Domain0 Region06            : 0x000000000c000000-0x000000000c3fffff M: (I,R,W) S/U: (R,W)
Domain0 Region07            : 0x0000000000000000-0xffffffffffffffff M: () S/U: (R,W,X)
Domain0 Next Address        : 0x0000000081200000
Domain0 Next Arg1           : 0x0000000081289ba8
Domain0 Next Mode           : S-mode
Domain0 SysReset            : yes
Domain0 SysSuspend          : yes

Boot HART ID                : 0
Boot HART Domain            : root
Boot HART Priv Version      : v1.12
Boot HART Base ISA          : rv64imafdcbvh
Boot HART ISA Extensions    : smstateen,sscofpmf,sstc,zicntr,zihpm,zicboz,zicbom,svpbmt,sdtrig,svade,smnpm,ssstateen
Boot HART PMP Count         : 0
Boot HART PMP Granularity   : 0 bits
Boot HART PMP Address Bits  : 0
Boot HART MHPM Info         : 16 (0x0007fff8)
Boot HART Debug Triggers    : 2 triggers
Boot HART MIDELEG           : 0x0000000000003666
Boot HART MEDELEG           : 0x0000000000f4b509


U-Boot 2022.07 (Jan 01 1980 - 00:00:00 +0000)

CPU:   rv64imafdcbvh_zic64b_zicbom_zicbop_zicboz_ziccamoa_ziccif_zicclsm_ziccrse_zicond_zicntr_zicsr_zifencei_zihintntl_zihintpause_zihpm_zimop_zmmul_za64rs_zaamo_zalrsc_zawrs_zfa_zfhmin_zca_zcb_zcd_zcmop_zba_zbb_zbs_zkt_zvbb_zve32f_zve32x_zve64f_zve64d_zve64x_zvfhmin_zvkb_zModel: riscv-virtio,qemu
DRAM:  4 GiB
Core:  28 devices, 14 uclasses, devicetree: board
Flash: 32 MiB
Loading Environment from nowhere... OK
In:    serial@10000000
Out:   serial@10000000
Err:   serial@10000000
Net:   eth0: virtio-net#0
Hit any key to stop autoboot:  0
42086656 bytes read in 19 ms (2.1 GiB/s)
## Verifying integrity...OK
## Verifying authenticity...OK
1375017 bytes read in 1 ms (1.3 GiB/s)
## Verifying integrity...OK
## Verifying authenticity...OK
Moving Image from 0x84000000 to 0x80200000, end=82ab2000
## Flattened Device Tree blob at ff748120
   Booting using the fdt blob at 0xff748120
   Using Device Tree in place at 00000000ff748120, end 00000000ff74dcde

Starting kernel ...

[    0.000000] Linux version 6.12.78 (nixbld@localhost) (riscv64-unknown-linux-gnu-gcc (GCC) 15.2.0, GNU ld (GNU Binutils) 2.44) #1-NixOS SMP Wed Mar 25 10:08:58 UTC 2026
[    0.000000] random: crng init done
[    0.000000] Machine model: riscv-virtio,qemu
[    0.000000] SBI specification v3.0 detected
[    0.000000] SBI implementation ID=0x1 Version=0x10008
[    0.000000] SBI TIME extension detected
[    0.000000] SBI IPI extension detected
[    0.000000] SBI RFENCE extension detected
[    0.000000] SBI SRST extension detected
[    0.000000] SBI DBCN extension detected
[    0.000000] earlycon: sbi0 at I/O port 0x0 (options '')
[    0.000000] printk: legacy bootconsole [sbi0] enabled
[    0.000000] efi: UEFI not found.
[    0.000000] OF: reserved mem: 0x0000000080100000..0x000000008013ffff (256 KiB) nomap non-reusable mmode_resv1@80100000
[    0.000000] OF: reserved mem: 0x0000000080140000..0x000000008015ffff (128 KiB) nomap non-reusable mmode_resv0@80140000
[    0.000000] NUMA: Faking a node at [mem 0x0000000080000000-0x000000017fffffff]
[    0.000000] NODE_DATA(0) allocated [mem 0x17fdefa80-0x17fdf14ff]
[    0.000000] Zone ranges:
[    0.000000]   DMA32    [mem 0x0000000080000000-0x00000000ffffffff]
[    0.000000]   Normal   [mem 0x0000000100000000-0x000000017fffffff]
[    0.000000] Movable zone start for each node
[    0.000000] Early memory node ranges
[    0.000000]   node   0: [mem 0x0000000080000000-0x00000000800fffff]
[    0.000000]   node   0: [mem 0x0000000080100000-0x000000008015ffff]
[    0.000000]   node   0: [mem 0x0000000080160000-0x000000017fffffff]
[    0.000000] Initmem setup node 0 [mem 0x0000000080000000-0x000000017fffffff]
[    0.000000] SBI HSM extension detected
[    0.000000] riscv: base ISA extensions acdfhimv
[    0.000000] riscv: ELF capabilities acdfimv
[    0.000000] percpu: Embedded 80 pages/cpu s144168 r65536 d117976 u327680
[    0.000000] Kernel command line: root=LABEL=rootfs ro earlycon=sbi
[    0.000000] Dentry cache hash table entries: 524288 (order: 10, 4194304 bytes, linear)
[    0.000000] Inode-cache hash table entries: 262144 (order: 9, 2097152 bytes, linear)
[    0.000000] Fallback order for Node 0: 0
[    0.000000] Built 1 zonelists, mobility grouping on.  Total pages: 1048576
[    0.000000] Policy zone: Normal
[    0.000000] mem auto-init: stack:all(zero), heap alloc:on, heap free:off
[    0.000000] software IO TLB: area num 2.
[    0.000000] software IO TLB: mapped [mem 0x00000000fb748000-0x00000000ff748000] (64MB)
[    0.000000] Virtual kernel memory layout:
[    0.000000]       fixmap : 0xffffffc4fea00000 - 0xffffffc4ff000000   (6144 kB)
[    0.000000]       pci io : 0xffffffc4ff000000 - 0xffffffc500000000   (  16 MB)
[    0.000000]      vmemmap : 0xffffffc500000000 - 0xffffffc600000000   (4096 MB)
[    0.000000]      vmalloc : 0xffffffc600000000 - 0xffffffd600000000   (  64 GB)
[    0.000000]      modules : 0xffffffff028b2000 - 0xffffffff80000000   (2007 MB)
[    0.000000]       lowmem : 0xffffffd600000000 - 0xffffffd700000000   (4096 MB)
[    0.000000]       kernel : 0xffffffff80000000 - 0xffffffffffffffff   (2047 MB)
[    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=2, Nodes=1
[    0.000000] allocated 8388608 bytes of page_ext
[    0.000000] ftrace: allocating 43172 entries in 169 pages
[    0.000000] ftrace: allocated 169 pages with 4 groups
[    0.000000] rcu: Hierarchical RCU implementation.
[    0.000000] rcu:     RCU restricting CPUs from NR_CPUS=64 to nr_cpu_ids=2.
[    0.000000] rcu:     RCU debug extended QS entry/exit.
[    0.000000]  Rude variant of Tasks RCU enabled.
[    0.000000]  Tracing variant of Tasks RCU enabled.
[    0.000000] rcu: RCU calculated value of scheduler-enlistment delay is 25 jiffies.
[    0.000000] rcu: Adjusting geometry for rcu_fanout_leaf=16, nr_cpu_ids=2
[    0.000000] RCU Tasks Rude: Setting shift to 1 and lim to 1 rcu_task_cb_adjust=1 rcu_task_cpu_ids=2.
[    0.000000] RCU Tasks Trace: Setting shift to 1 and lim to 1 rcu_task_cb_adjust=1 rcu_task_cpu_ids=2.
[    0.000000] NR_IRQS: 64, nr_irqs: 64, preallocated irqs: 0
[    0.000000] riscv-intc: 64 local interrupts mapped
[    0.000000] riscv: providing IPIs using SBI IPI extension
[    0.000000] rcu: srcu_init: Setting srcu_struct sizes based on contention.
[    0.000000] clocksource: riscv_clocksource: mask: 0xffffffffffffffff max_cycles: 0x24e6a1710, max_idle_ns: 440795202120 ns
[    0.000036] sched_clock: 64 bits at 10MHz, resolution 100ns, wraps every 4398046511100ns
[    0.000262] riscv-timer: Timer interrupt in S-mode is available via sstc extension
[    0.003047] kfence: initialized - using 2097152 bytes for 255 objects at 0x(____ptrval____)-0x(____ptrval____)
[    0.006927] Console: colour dummy device 80x25
[    0.007188] printk: legacy console [tty0] enabled
[    0.007497] printk: legacy bootconsole [sbi0] disabled
[    0.000000] Linux version 6.12.78 (nixbld@localhost) (riscv64-unknown-linux-gnu-gcc (GCC) 15.2.0, GNU ld (GNU Binutils) 2.44) #1-NixOS SMP Wed Mar 25 10:08:58 UTC 2026
[    0.000000] random: crng init done
[    0.000000] Machine model: riscv-virtio,qemu
[    0.000000] SBI specification v3.0 detected
[    0.000000] SBI implementation ID=0x1 Version=0x10008
[    0.000000] SBI TIME extension detected
[    0.000000] SBI IPI extension detected
[    0.000000] SBI RFENCE extension detected
[    0.000000] SBI SRST extension detected
[    0.000000] SBI DBCN extension detected
[    0.000000] earlycon: sbi0 at I/O port 0x0 (options '')
[    0.000000] printk: legacy bootconsole [sbi0] enabled
[    0.000000] efi: UEFI not found.
[    0.000000] OF: reserved mem: 0x0000000080100000..0x000000008013ffff (256 KiB) nomap non-reusable mmode_resv1@80100000
[    0.000000] OF: reserved mem: 0x0000000080140000..0x000000008015ffff (128 KiB) nomap non-reusable mmode_resv0@80140000
[    0.000000] NUMA: Faking a node at [mem 0x0000000080000000-0x000000017fffffff]
[    0.000000] NODE_DATA(0) allocated [mem 0x17fdefa80-0x17fdf14ff]
[    0.000000] Zone ranges:
[    0.000000]   DMA32    [mem 0x0000000080000000-0x00000000ffffffff]
[    0.000000]   Normal   [mem 0x0000000100000000-0x000000017fffffff]
[    0.000000] Movable zone start for each node
[    0.000000] Early memory node ranges
[    0.000000]   node   0: [mem 0x0000000080000000-0x00000000800fffff]
[    0.000000]   node   0: [mem 0x0000000080100000-0x000000008015ffff]
[    0.000000]   node   0: [mem 0x0000000080160000-0x000000017fffffff]
[    0.000000] Initmem setup node 0 [mem 0x0000000080000000-0x000000017fffffff]
[    0.000000] SBI HSM extension detected
[    0.000000] riscv: base ISA extensions acdfhimv
[    0.000000] riscv: ELF capabilities acdfimv
[    0.000000] percpu: Embedded 80 pages/cpu s144168 r65536 d117976 u327680
[    0.000000] Kernel command line: root=LABEL=rootfs ro earlycon=sbi
[    0.000000] Dentry cache hash table entries: 524288 (order: 10, 4194304 bytes, linear)
[    0.000000] Inode-cache hash table entries: 262144 (order: 9, 2097152 bytes, linear)
[    0.000000] Fallback order for Node 0: 0
[    0.000000] Built 1 zonelists, mobility grouping on.  Total pages: 1048576
[    0.000000] Policy zone: Normal
[    0.000000] mem auto-init: stack:all(zero), heap alloc:on, heap free:off
[    0.000000] software IO TLB: area num 2.
[    0.000000] software IO TLB: mapped [mem 0x00000000fb748000-0x00000000ff748000] (64MB)
[    0.000000] Virtual kernel memory layout:
[    0.000000]       fixmap : 0xffffffc4fea00000 - 0xffffffc4ff000000   (6144 kB)
[    0.000000]       pci io : 0xffffffc4ff000000 - 0xffffffc500000000   (  16 MB)
[    0.000000]      vmemmap : 0xffffffc500000000 - 0xffffffc600000000   (4096 MB)
[    0.000000]      vmalloc : 0xffffffc600000000 - 0xffffffd600000000   (  64 GB)
[    0.000000]      modules : 0xffffffff028b2000 - 0xffffffff80000000   (2007 MB)
[    0.000000]       lowmem : 0xffffffd600000000 - 0xffffffd700000000   (4096 MB)
[    0.000000]       kernel : 0xffffffff80000000 - 0xffffffffffffffff   (2047 MB)
[    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=2, Nodes=1
[    0.000000] allocated 8388608 bytes of page_ext
[    0.000000] ftrace: allocating 43172 entries in 169 pages
[    0.000000] ftrace: allocated 169 pages with 4 groups
[    0.000000] rcu: Hierarchical RCU implementation.
[    0.000000] rcu:     RCU restricting CPUs from NR_CPUS=64 to nr_cpu_ids=2.
[    0.000000] rcu:     RCU debug extended QS entry/exit.
[    0.000000]  Rude variant of Tasks RCU enabled.
[    0.000000]  Tracing variant of Tasks RCU enabled.
[    0.000000] rcu: RCU calculated value of scheduler-enlistment delay is 25 jiffies.
[    0.000000] rcu: Adjusting geometry for rcu_fanout_leaf=16, nr_cpu_ids=2
[    0.000000] RCU Tasks Rude: Setting shift to 1 and lim to 1 rcu_task_cb_adjust=1 rcu_task_cpu_ids=2.
[    0.000000] RCU Tasks Trace: Setting shift to 1 and lim to 1 rcu_task_cb_adjust=1 rcu_task_cpu_ids=2.
[    0.000000] NR_IRQS: 64, nr_irqs: 64, preallocated irqs: 0
[    0.000000] riscv-intc: 64 local interrupts mapped
[    0.000000] riscv: providing IPIs using SBI IPI extension
[    0.000000] rcu: srcu_init: Setting srcu_struct sizes based on contention.
[    0.000000] clocksource: riscv_clocksource: mask: 0xffffffffffffffff max_cycles: 0x24e6a1710, max_idle_ns: 440795202120 ns
[    0.000036] sched_clock: 64 bits at 10MHz, resolution 100ns, wraps every 4398046511100ns
[    0.000262] riscv-timer: Timer interrupt in S-mode is available via sstc extension
[    0.003047] kfence: initialized - using 2097152 bytes for 255 objects at 0x(____ptrval____)-0x(____ptrval____)
[    0.006927] Console: colour dummy device 80x25
[    0.007188] printk: legacy console [tty0] enabled
[    0.007497] printk: legacy bootconsole [sbi0] disabled
[    0.009163] Calibrating delay loop (skipped), value calculated using timer frequency.. 20.00 BogoMIPS (lpj=40000)
[    0.009307] pid_max: default: 32768 minimum: 301
[    0.010767] LSM: initializing lsm=capability,landlock,yama,ipe,ima
[    0.011829] landlock: Up and running.
[    0.011869] Yama: becoming mindful.
[    0.013687] Mount-cache hash table entries: 8192 (order: 4, 65536 bytes, linear)
[    0.013753] Mountpoint-cache hash table entries: 8192 (order: 4, 65536 bytes, linear)
[    0.039503] riscv: ELF compat mode supported
[    0.040167] ASID allocator using 16 bits (65536 entries)
[    0.041183] rcu: Hierarchical SRCU implementation.
[    0.041222] rcu:     Max phase no-delay instances is 1000.
[    0.042314] Timer migration: 1 hierarchy levels; 8 children per group; 1 crossnode level
[    0.045427] EFI services will not be available.
[    0.046461] smp: Bringing up secondary CPUs ...
[    0.054225] smp: Brought up 1 node, 2 CPUs
[    0.060582] Memory: 3961112K/4194304K available (12409K kernel code, 7965K rwdata, 10240K rodata, 8558K init, 571K bss, 218584K reserved, 0K cma-reserved)
[    0.067049] devtmpfs: initialized
[    0.075497] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
[    0.075674] futex hash table entries: 512 (order: 3, 32768 bytes, linear)
[    0.077499] pinctrl core: initialized pinctrl subsystem
[    0.082825] DMI not present or invalid.
[    0.084508] NET: Registered PF_NETLINK/PF_ROUTE protocol family
[    0.087072] DMA: preallocated 512 KiB GFP_KERNEL pool for atomic allocations
[    0.087551] DMA: preallocated 512 KiB GFP_KERNEL|GFP_DMA32 pool for atomic allocations
[    0.087733] audit: initializing netlink subsys (disabled)
[    0.089148] audit: type=2000 audit(0.084:1): state=initialized audit_enabled=0 res=1
[    0.091790] thermal_sys: Registered thermal governor 'fair_share'
[    0.091826] thermal_sys: Registered thermal governor 'bang_bang'
[    0.091854] thermal_sys: Registered thermal governor 'step_wise'
[    0.091872] thermal_sys: Registered thermal governor 'user_space'
[    0.091887] thermal_sys: Registered thermal governor 'power_allocator'
[    0.092203] cpuidle: using governor ladder
[    0.092316] cpuidle: using governor menu
[    0.116396] cpu1: Ratio of byte access time to unaligned word access is 7.85, unaligned accesses are fast
[    0.140535] cpu0: Ratio of byte access time to unaligned word access is 7.42, unaligned accesses are fast
[    0.160827] HugeTLB: registered 64.0 KiB page size, pre-allocated 0 pages
[    0.160878] HugeTLB: 0 KiB vmemmap can be freed for a 64.0 KiB page
[    0.160911] HugeTLB: registered 2.00 MiB page size, pre-allocated 0 pages
[    0.160931] HugeTLB: 28 KiB vmemmap can be freed for a 2.00 MiB page
[    0.174448] fbcon: Taking over console
[    0.174715] ACPI: Interpreter disabled.
[    0.175697] iommu: Default domain type: Translated
[    0.175735] iommu: DMA domain TLB invalidation policy: strict mode
[    0.179745] SCSI subsystem initialized
[    0.191459] vgaarb: loaded
[    0.193303] clocksource: Switched to clocksource riscv_clocksource
[    0.197076] pnp: PnP ACPI: disabled
[    0.213904] NET: Registered PF_INET protocol family
[    0.214947] IP idents hash table entries: 65536 (order: 7, 524288 bytes, linear)
[    0.247291] tcp_listen_portaddr_hash hash table entries: 2048 (order: 4, 65536 bytes, linear)
[    0.247482] Table-perturb hash table entries: 65536 (order: 6, 262144 bytes, linear)
[    0.247614] TCP established hash table entries: 32768 (order: 6, 262144 bytes, linear)
[    0.248366] TCP bind hash table entries: 32768 (order: 9, 2097152 bytes, linear)
[    0.250279] TCP: Hash tables configured (established 32768 bind 32768)
[    0.251296] MPTCP token hash table entries: 4096 (order: 6, 196608 bytes, linear)
[    0.251696] UDP hash table entries: 2048 (order: 6, 196608 bytes, linear)
[    0.252035] UDP-Lite hash table entries: 2048 (order: 6, 196608 bytes, linear)
[    0.253231] NET: Registered PF_UNIX/PF_LOCAL protocol family
[    0.253558] NET: Registered PF_XDP protocol family
[    0.253693] PCI: CLS 0 bytes, default 64
[    0.256015] Initialise system trusted keyrings
[    0.258744] Unpacking initramfs...
[    0.261543] workingset: timestamp_bits=44 max_order=20 bucket_order=0
[    0.265050] 9p: Installing v9fs 9p2000 file system support
[    0.310101] Freeing initrd memory: 1340K
[    0.312098] NET: Registered PF_ALG protocol family
[    0.312261] Key type asymmetric registered
[    0.312337] Asymmetric key parser 'x509' registered
[    0.312587] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 243)
[    0.313215] io scheduler mq-deadline registered
[    0.313283] io scheduler kyber registered
[    0.316909] riscv-plic: plic@c000000: mapped 95 interrupts with 2 handlers for 4 contexts.
[    0.319425] pci-host-generic 30000000.pci: host bridge /soc/pci@30000000 ranges:
[    0.319942] pci-host-generic 30000000.pci:       IO 0x0003000000..0x000300ffff -> 0x0000000000
[    0.320282] pci-host-generic 30000000.pci:      MEM 0x0040000000..0x007fffffff -> 0x0040000000
[    0.320353] pci-host-generic 30000000.pci:      MEM 0x0400000000..0x07ffffffff -> 0x0400000000
[    0.320720] pci-host-generic 30000000.pci: Memory resource size exceeds max for 32 bits
[    0.321018] pci-host-generic 30000000.pci: ECAM at [mem 0x30000000-0x3fffffff] for [bus 00-ff]
[    0.322013] pci-host-generic 30000000.pci: PCI host bridge to bus 0000:00
[    0.322263] pci_bus 0000:00: root bus resource [bus 00-ff]
[    0.322344] pci_bus 0000:00: root bus resource [io  0x0000-0xffff]
[    0.322387] pci_bus 0000:00: root bus resource [mem 0x40000000-0x7fffffff]
[    0.322409] pci_bus 0000:00: root bus resource [mem 0x400000000-0x7ffffffff]
[    0.323297] pci 0000:00:00.0: [1b36:0008] type 00 class 0x060000 conventional PCI endpoint
[    0.325594] pci 0000:00:01.0: [1af4:1005] type 00 class 0x00ff00 conventional PCI endpoint
[    0.326400] pci 0000:00:01.0: BAR 0 [io  0x1000-0x101f]
[    0.326921] pci 0000:00:01.0: BAR 1 [mem 0x40000000-0x40000fff]
[    0.328408] pci 0000:00:01.0: BAR 4 [mem 0x40004000-0x40007fff 64bit pref]
[    0.329387] pci 0000:00:02.0: [1af4:1001] type 00 class 0x010000 conventional PCI endpoint
[    0.329916] pci 0000:00:02.0: BAR 0 [io  0x1080-0x10ff]
[    0.330418] pci 0000:00:02.0: BAR 1 [mem 0x40008000-0x40008fff]
[    0.331816] pci 0000:00:02.0: BAR 4 [mem 0x4000c000-0x4000ffff 64bit pref]
[    0.333828] pci 0000:00:01.0: BAR 4 [mem 0x400000000-0x400003fff 64bit pref]: assigned
[    0.334375] pci 0000:00:02.0: BAR 4 [mem 0x400004000-0x400007fff 64bit pref]: assigned
[    0.334755] pci 0000:00:01.0: BAR 1 [mem 0x40000000-0x40000fff]: assigned
[    0.334801] pci 0000:00:02.0: BAR 1 [mem 0x40001000-0x40001fff]: assigned
[    0.335001] pci 0000:00:02.0: BAR 0 [io  0x0080-0x00ff]: assigned
[    0.335230] pci 0000:00:01.0: BAR 0 [io  0x0020-0x003f]: assigned
[    0.335757] pci_bus 0000:00: resource 4 [io  0x0000-0xffff]
[    0.336579] pci_bus 0000:00: resource 5 [mem 0x40000000-0x7fffffff]
[    0.336629] pci_bus 0000:00: resource 6 [mem 0x400000000-0x7ffffffff]
[    0.340090] SBI CPPC extension NOT detected!!
[    0.442807] Serial: 8250/16550 driver, 4 ports, IRQ sharing disabled
[    0.453792] 10000000.serial: ttyS0 at MMIO 0x10000000 (irq = 14, base_baud = 230400) is a 16550A
[    0.454867] printk: legacy console [ttyS0] enabled
[    0.512476] loop: module loaded
[    0.512980] virtio_blk virtio2: 2/0/0 default/read/poll queues
[    0.514668] virtio_blk virtio2: [vda] 390664 512-byte logical blocks (200 MB/191 MiB)
[    0.523733]  vda: vda1
[    0.534524] e1000e: Intel(R) PRO/1000 Network Driver
[    0.534631] e1000e: Copyright(c) 1999 - 2015 Intel Corporation.
[    0.535656] mousedev: PS/2 mouse device common for all mice
[    0.538170] goldfish_rtc 101000.rtc: registered as rtc0
[    0.538587] goldfish_rtc 101000.rtc: setting system clock to 2026-04-09T07:13:29 UTC (1775718809)
[    0.543049] sdhci: Secure Digital Host Controller Interface driver
[    0.543194] sdhci: Copyright(c) Pierre Ossman
[    0.543332] Synopsys Designware Multimedia Card Interface Driver
[    0.543692] sdhci-pltfm: SDHCI platform and OF driver helper
[    0.544297] hid: raw HID events driver (C) Jiri Kosina
[    0.545072] riscv-pmu-sbi: SBI PMU extension is available
[    0.545591] riscv-pmu-sbi: 16 firmware and 18 hardware counters
[    0.547690] drop_monitor: Initializing network drop monitor service
[    0.548947] NET: Registered PF_INET6 protocol family
[    0.559856] Segment Routing with IPv6
[    0.560111] In-situ OAM (IOAM) with IPv6
[    0.560508] sit: IPv6, IPv4 and MPLS over IPv4 tunneling driver
[    0.563672] NET: Registered PF_PACKET protocol family
[    0.564474] 9pnet: Installing 9P2000 support
[    0.564779] Key type dns_resolver registered
[    0.592384] registered taskstats version 1
[    0.593946] Loading compiled-in X.509 certificates
[    0.631468] Demotion targets for Node 0: null
[    0.631635] debug_vm_pgtable: [debug_vm_pgtable         ]: Validating architecture page table helpers
[    0.635688] Key type .fscrypt registered
[    0.635775] Key type fscrypt-provisioning registered
[    0.637610] ima: No TPM chip found, activating TPM-bypass!
[    0.637737] ima: Allocated hash algorithm: sha1
[    0.639055] ima: No architecture policies found
[    0.641633] clk: Disabling unused clocks
[    0.641829] PM: genpd: Disabling unused power domains
[    0.691827] Freeing unused kernel image (initmem) memory: 8556K
[    0.692646] Run /init as init process
Waiting for LABEL=rootfs ...
Checking /dev/vda1 ...
fsck (busybox 1.37.0)
Mounting /dev/vda1 ...
[    0.999207] EXT4-fs (vda1): recovery complete
[    1.003139] EXT4-fs (vda1): mounted filesystem 7282fd6c-a641-4cbe-8cd3-48a43d519312 r/w with ordered data mode. Quota mode: disabled.
udhcpc: started, v1.37.0
udhcpc: broadcasting discover
udhcpc: broadcasting select for 10.0.2.15, server 10.0.2.2
udhcpc: lease of 10.0.2.15 obtained from 10.0.2.2, lease time 86400

(none) login:
```

