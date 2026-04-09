# Run Ubuntu
## Run Automatically
```
nix run .#run-ubuntu
```

This automatically downloads the Ubuntu image and signs the kernel.

## Create Image Manually
### Prerequisites
- [Signing Tools](./how-to-sign.md)
    - You should replace `keytool` `secbimg` in the following instruction with the actual path to the binary.
- U-Boot built with `qemu_riscv64_spl_sb_seq_ubuntu_defconfig`
    - `u-boot-spl.bin` and `u-boot-nodtb.bin` are required.
- OpenSBI binary (`fw_dynamic.bin`)
- [QEMU](./how-to-build.md#qemu) compatible with Secure Boot

### Steps
1. Download and unpack original image. Our example uses the Ubuntu 24.04.4 preinstalled server image for QEMU.
   ```bash
   wget https://cdimage.ubuntu.com/releases/24.04.4/release/ubuntu-24.04.4-preinstalled-server-riscv64.img.xz
   unxz ubuntu-24.04.4-preinstalled-server-riscv64.img.xz
   ```

2. Mount rootfs
   ```bash
   tmpdir=$(mktemp -d)
   mkdir -p $tmpdir/mnt
   sudo losetup -f --show -P ubuntu-24.04.4-preinstalled-server-riscv64.img     # this shows attached loopback device e.g.,) /dev/loop0
   sudo mount /dev/loopXp1 $tmpdir/mnt     # replace X with actual number
   ```

3. Generate keys
   ```bash
   keytool create -a ed25519 -k ubuntu -d
   ```

4. Sign kernel and initramfs
   ```bash
   secbimg create \
     -i u-boot-nodtb.bin \
     -o u-boot-signed.bin \
     -a sha3 -s ed25519 -k ed25519_ubuntu.key

   sudo secbimg create \
     -i $tmpdir/mnt/boot/vmlinuz \
     -o $tmpdir/mnt/boot/vmlinuz-signed \
     -a sha3 -s ed25519 -k ed25519_ubuntu.key

   sudo secbimg create \
     -i $tmpdir/mnt/boot/initrd.img \
     -o $tmpdir/mnt/boot/initrd.img-signed \
     -a sha3 -s ed25519 -k ed25519_ubuntu.key
   
   sudo umount $tmpdir/mnt
   sudo losetup -d /dev/loopX
   ```

5. Sign bootloader and firmware
   ```bash
   secbimg create \
     -i fw_dynamic.bin \
     -o fw_dynamic-signed.bin \
     -l 80100000 -t 16 \
     -a sha3 -s ed25519 -k ed25519_ubuntu.key

   secbimg create \
     -i u-boot-nodtb.bin \
     -o u-boot-signed.bin \
     -l 81200000 -t 32 \
     -a sha3 -s ed25519 -k ed25519_ubuntu.key

   secbimg union \
     -i fw_dynamic-signed.bin \
     -i u-boot-signed.bin \
     -k ed25519_ubuntu.key -o signed-loader.bin
   ```

6. Run
   ```bash
   qemu-system-riscv64 -cpu rva23s64 -smp cpus=2 -m 4G -nographic \
     -machine virt,keyfile=ed25519_pub_ubuntu.key,keyalgo=ed25519 \
     -bios u-boot-spl.bin \
     -device loader,file=signed-loader.bin,addr=0x80200000 \
     -drive file=ubuntu-24.04.4-preinstalled-server-riscv64.img,format=raw,if=virtio \
     -netdev user,id=net0 \
     -device virtio-net-device,netdev=net0 \
     -device virtio-rng-pci
   ```

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
Domain0 Next Arg1           : 0x0000000081289bb0
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
43736184 bytes read in 15 ms (2.7 GiB/s)
## Verifying integrity...OK
## Verifying authenticity...OK
73405667 bytes read in 18 ms (3.8 GiB/s)
## Verifying integrity...OK
## Verifying authenticity...OK
Moving Image from 0x84000000 to 0x80200000, end=82b9f000
## Flattened Device Tree blob at ff748120
   Booting using the fdt blob at 0xff748120
   Using Device Tree in place at 00000000ff748120, end 00000000ff74dcde

Starting kernel ...

[    0.000000] Booting Linux on hartid 0
[    0.000000] Linux version 6.17.0-14-generic (buildd@bos03-riscv64-029) (riscv64-linux-gnu-gcc-13 (Ubuntu 13.3.0-6ubuntu2~24.04) 13.3.0, GNU ld (GNU Binutils for Ubuntu) 2.42) #14.1~24.04.1-Ubuntu SMP PREEMPT_DYNAMIC Thu Jan 29 13:59:41 UTC (Ubuntu 6.17.0-14.14.1~24.04.1-generic 6.17.9)
[    0.000000] random: crng init done
[    0.000000] Machine model: riscv-virtio,qemu
[    0.000000] SBI specification v3.0 detected
[    0.000000] SBI implementation ID=0x1 Version=0x10008
[    0.000000] SBI TIME extension detected
[    0.000000] SBI IPI extension detected
[    0.000000] SBI RFENCE extension detected
[    0.000000] SBI SRST extension detected
[    0.000000] SBI DBCN extension detected
[    0.000000] SBI FWFT extension detected
[    0.000000] earlycon: sbi0 at I/O port 0x0 (options '')
[    0.000000] printk: legacy bootconsole [sbi0] enabled
[    0.000000] efi: UEFI not found.
[    0.000000] cma: Reserved 32 MiB at 0x00000000fd600000
[    0.000000] OF: reserved mem: 0x0000000080100000..0x000000008013ffff (256 KiB) nomap non-reusable mmode_resv1@80100000
[    0.000000] OF: reserved mem: 0x0000000080140000..0x000000008015ffff (128 KiB) nomap non-reusable mmode_resv0@80140000
[    0.000000] NUMA: Faking a node at [mem 0x0000000080000000-0x000000017fffffff]
[    0.000000] NODE_DATA(0) allocated [mem 0x17fdee080-0x17fdf083f]
[    0.000000] Zone ranges:
[    0.000000]   DMA32    [mem 0x0000000080000000-0x00000000ffffffff]
[    0.000000]   Normal   [mem 0x0000000100000000-0x000000017fffffff]
[    0.000000]   Device   empty
[    0.000000] Movable zone start for each node
[    0.000000] Early memory node ranges
[    0.000000]   node   0: [mem 0x0000000080000000-0x00000000800fffff]
[    0.000000]   node   0: [mem 0x0000000080100000-0x000000008015ffff]
[    0.000000]   node   0: [mem 0x0000000080160000-0x000000017fffffff]
[    0.000000] Initmem setup node 0 [mem 0x0000000080000000-0x000000017fffffff]
[    0.000000] SBI HSM extension detected
[    0.000000] riscv: base ISA extensions acdfhimv
[    0.000000] riscv: ELF capabilities acdfimv
[    0.000000] Queued spinlock using Ziccrse: enabled
[    0.000000] percpu: Embedded 54 pages/cpu s95640 r8192 d117352 u221184
[    0.000000] Kernel command line: root=LABEL=cloudimg-rootfs ro earlycon=sbi
[    0.000000] printk: log buffer data + meta data: 262144 + 917504 = 1179648 bytes
[    0.000000] Dentry cache hash table entries: 524288 (order: 10, 4194304 bytes, linear)
[    0.000000] Inode-cache hash table entries: 262144 (order: 9, 2097152 bytes, linear)
[    0.000000] software IO TLB: area num 2.
[    0.000000] software IO TLB: mapped [mem 0x00000000f9600000-0x00000000fd600000] (64MB)
[    0.000000] Fallback order for Node 0: 0
[    0.000000] Built 1 zonelists, mobility grouping on.  Total pages: 1048576
[    0.000000] Policy zone: Normal
[    0.000000] mem auto-init: stack:all(zero), heap alloc:on, heap free:off
[    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=2, Nodes=1
[    0.000000] Dynamic Preempt: voluntary
[    0.000000] rcu: Preemptible hierarchical RCU implementation.
[    0.000000] rcu:     RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=2.
[    0.000000]  Trampoline variant of Tasks RCU enabled.
[    0.000000]  Tracing variant of Tasks RCU enabled.
[    0.000000] rcu: RCU calculated value of scheduler-enlistment delay is 25 jiffies.
[    0.000000] rcu: Adjusting geometry for rcu_fanout_leaf=16, nr_cpu_ids=2
[    0.000000] RCU Tasks: Setting shift to 1 and lim to 1 rcu_task_cb_adjust=1 rcu_task_cpu_ids=2.
[    0.000000] RCU Tasks Trace: Setting shift to 1 and lim to 1 rcu_task_cb_adjust=1 rcu_task_cpu_ids=2.
[    0.000000] NR_IRQS: 64, nr_irqs: 64, preallocated irqs: 0
[    0.000000] riscv-intc: 64 local interrupts mapped
[    0.000000] riscv: providing IPIs using SBI IPI extension
[    0.000000] rcu: srcu_init: Setting srcu_struct sizes based on contention.
[    0.000000] clocksource: riscv_clocksource: mask: 0xffffffffffffffff max_cycles: 0x24e6a1710, max_idle_ns: 440795202120 ns
[    0.000056] sched_clock: 64 bits at 10MHz, resolution 100ns, wraps every 4398046511100ns
[    0.001234] riscv-timer: Timer interrupt in S-mode is available via sstc extension
[    0.013853] Console: colour dummy device 80x25
[    0.014135] printk: legacy console [tty0] enabled
[    0.014524] printk: legacy bootconsole [sbi0] disabled
[    0.000000] Booting Linux on hartid 0
[    0.000000] Linux version 6.17.0-14-generic (buildd@bos03-riscv64-029) (riscv64-linux-gnu-gcc-13 (Ubuntu 13.3.0-6ubuntu2~24.04) 13.3.0, GNU ld (GNU Binutils for Ubuntu) 2.42) #14.1~24.04.1-Ubuntu SMP PREEMPT_DYNAMIC Thu Jan 29 13:59:41 UTC (Ubuntu 6.17.0-14.14.1~24.04.1-generic 6.17.9)
[    0.000000] random: crng init done
[    0.000000] Machine model: riscv-virtio,qemu
[    0.000000] SBI specification v3.0 detected
[    0.000000] SBI implementation ID=0x1 Version=0x10008
[    0.000000] SBI TIME extension detected
[    0.000000] SBI IPI extension detected
[    0.000000] SBI RFENCE extension detected
[    0.000000] SBI SRST extension detected
[    0.000000] SBI DBCN extension detected
[    0.000000] SBI FWFT extension detected
[    0.000000] earlycon: sbi0 at I/O port 0x0 (options '')
[    0.000000] printk: legacy bootconsole [sbi0] enabled
[    0.000000] efi: UEFI not found.
[    0.000000] cma: Reserved 32 MiB at 0x00000000fd600000
[    0.000000] OF: reserved mem: 0x0000000080100000..0x000000008013ffff (256 KiB) nomap non-reusable mmode_resv1@80100000
[    0.000000] OF: reserved mem: 0x0000000080140000..0x000000008015ffff (128 KiB) nomap non-reusable mmode_resv0@80140000
[    0.000000] NUMA: Faking a node at [mem 0x0000000080000000-0x000000017fffffff]
[    0.000000] NODE_DATA(0) allocated [mem 0x17fdee080-0x17fdf083f]
[    0.000000] Zone ranges:
[    0.000000]   DMA32    [mem 0x0000000080000000-0x00000000ffffffff]
[    0.000000]   Normal   [mem 0x0000000100000000-0x000000017fffffff]
[    0.000000]   Device   empty
[    0.000000] Movable zone start for each node
[    0.000000] Early memory node ranges
[    0.000000]   node   0: [mem 0x0000000080000000-0x00000000800fffff]
[    0.000000]   node   0: [mem 0x0000000080100000-0x000000008015ffff]
[    0.000000]   node   0: [mem 0x0000000080160000-0x000000017fffffff]
[    0.000000] Initmem setup node 0 [mem 0x0000000080000000-0x000000017fffffff]
[    0.000000] SBI HSM extension detected
[    0.000000] riscv: base ISA extensions acdfhimv
[    0.000000] riscv: ELF capabilities acdfimv
[    0.000000] Queued spinlock using Ziccrse: enabled
[    0.000000] percpu: Embedded 54 pages/cpu s95640 r8192 d117352 u221184
[    0.000000] Kernel command line: root=LABEL=cloudimg-rootfs ro earlycon=sbi
[    0.000000] printk: log buffer data + meta data: 262144 + 917504 = 1179648 bytes
[    0.000000] Dentry cache hash table entries: 524288 (order: 10, 4194304 bytes, linear)
[    0.000000] Inode-cache hash table entries: 262144 (order: 9, 2097152 bytes, linear)
[    0.000000] software IO TLB: area num 2.
[    0.000000] software IO TLB: mapped [mem 0x00000000f9600000-0x00000000fd600000] (64MB)
[    0.000000] Fallback order for Node 0: 0
[    0.000000] Built 1 zonelists, mobility grouping on.  Total pages: 1048576
[    0.000000] Policy zone: Normal
[    0.000000] mem auto-init: stack:all(zero), heap alloc:on, heap free:off
[    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=2, Nodes=1
[    0.000000] Dynamic Preempt: voluntary
[    0.000000] rcu: Preemptible hierarchical RCU implementation.
[    0.000000] rcu:     RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=2.
[    0.000000]  Trampoline variant of Tasks RCU enabled.
[    0.000000]  Tracing variant of Tasks RCU enabled.
[    0.000000] rcu: RCU calculated value of scheduler-enlistment delay is 25 jiffies.
[    0.000000] rcu: Adjusting geometry for rcu_fanout_leaf=16, nr_cpu_ids=2
[    0.000000] RCU Tasks: Setting shift to 1 and lim to 1 rcu_task_cb_adjust=1 rcu_task_cpu_ids=2.
[    0.000000] RCU Tasks Trace: Setting shift to 1 and lim to 1 rcu_task_cb_adjust=1 rcu_task_cpu_ids=2.
[    0.000000] NR_IRQS: 64, nr_irqs: 64, preallocated irqs: 0
[    0.000000] riscv-intc: 64 local interrupts mapped
[    0.000000] riscv: providing IPIs using SBI IPI extension
[    0.000000] rcu: srcu_init: Setting srcu_struct sizes based on contention.
[    0.000000] clocksource: riscv_clocksource: mask: 0xffffffffffffffff max_cycles: 0x24e6a1710, max_idle_ns: 440795202120 ns
[    0.000056] sched_clock: 64 bits at 10MHz, resolution 100ns, wraps every 4398046511100ns
[    0.001234] riscv-timer: Timer interrupt in S-mode is available via sstc extension
[    0.013853] Console: colour dummy device 80x25
[    0.014135] printk: legacy console [tty0] enabled
[    0.014524] printk: legacy bootconsole [sbi0] disabled
[    0.016448] Calibrating delay loop (skipped), value calculated using timer frequency.. 20.00 BogoMIPS (lpj=40000)
[    0.016586] pid_max: default: 32768 minimum: 301
[    0.018026] LSM: initializing lsm=lockdown,capability,landlock,yama,apparmor,ima,evm
[    0.018485] landlock: Up and running.
[    0.018513] Yama: becoming mindful.
[    0.020150] AppArmor: AppArmor initialized
[    0.022024] Mount-cache hash table entries: 8192 (order: 4, 65536 bytes, linear)
[    0.022077] Mountpoint-cache hash table entries: 8192 (order: 4, 65536 bytes, linear)
[    0.047896] ASID allocator using 16 bits (65536 entries)
[    0.049089] rcu: Hierarchical SRCU implementation.
[    0.049141] rcu:     Max phase no-delay instances is 1000.
[    0.050658] Timer migration: 1 hierarchy levels; 8 children per group; 1 crossnode level
[    0.053453] EFI services will not be available.
[    0.055678] smp: Bringing up secondary CPUs ...
[    0.061122] smp: Brought up 1 node, 2 CPUs
[    0.064592] Memory: 3883528K/4194304K available (14183K kernel code, 8389K rwdata, 12288K rodata, 6585K init, 1018K bss, 272036K reserved, 32768K cma-reserved)
[    0.071725] devtmpfs: initialized
[    0.085071] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
[    0.085193] posixtimers hash table entries: 1024 (order: 2, 16384 bytes, linear)
[    0.085410] futex hash table entries: 512 (32768 bytes on 1 NUMA nodes, total 32 KiB, linear).
[    0.088342] pinctrl core: initialized pinctrl subsystem
[    0.093676] DMI not present or invalid.
[    0.102078] NET: Registered PF_NETLINK/PF_ROUTE protocol family
[    0.109375] DMA: preallocated 512 KiB GFP_KERNEL pool for atomic allocations
[    0.109967] DMA: preallocated 512 KiB GFP_KERNEL|GFP_DMA32 pool for atomic allocations
[    0.110138] audit: initializing netlink subsys (disabled)
[    0.111566] audit: type=2000 audit(0.100:1): state=initialized audit_enabled=0 res=1
[    0.114956] thermal_sys: Registered thermal governor 'fair_share'
[    0.114985] thermal_sys: Registered thermal governor 'bang_bang'
[    0.115040] thermal_sys: Registered thermal governor 'step_wise'
[    0.115062] thermal_sys: Registered thermal governor 'user_space'
[    0.115081] thermal_sys: Registered thermal governor 'power_allocator'
[    0.115518] cpuidle: using governor ladder
[    0.115844] cpuidle: using governor menu
[    0.117306] SBI misaligned access exception delegation ok
[    0.140562] cpu1: Ratio of byte access time to unaligned word access is 7.85, unaligned accesses are fast
[    0.164699] cpu0: Ratio of byte access time to unaligned word access is 7.42, unaligned accesses are fast
[    0.206436] cpu1: Ratio of vector byte access time to vector unaligned word access is 2.91, unaligned accesses are fast
[    0.206441] cpu0: Ratio of vector byte access time to vector unaligned word access is 2.89, unaligned accesses are fast
[    0.254513] HugeTLB: registered 1.00 GiB page size, pre-allocated 0 pages
[    0.254566] HugeTLB: 16380 KiB vmemmap can be freed for a 1.00 GiB page
[    0.254597] HugeTLB: registered 64.0 KiB page size, pre-allocated 0 pages
[    0.254621] HugeTLB: 0 KiB vmemmap can be freed for a 64.0 KiB page
[    0.254642] HugeTLB: registered 2.00 MiB page size, pre-allocated 0 pages
[    0.254662] HugeTLB: 28 KiB vmemmap can be freed for a 2.00 MiB page
[    0.263541] fbcon: Taking over console
[    0.264067] ACPI: Interpreter disabled.
[    0.265066] iommu: Default domain type: Translated
[    0.265108] iommu: DMA domain TLB invalidation policy: strict mode
[    0.270880] SCSI subsystem initialized
[    0.272561] usbcore: registered new interface driver usbfs
[    0.272800] usbcore: registered new interface driver hub
[    0.272943] usbcore: registered new device driver usb
[    0.273283] pps_core: LinuxPPS API ver. 1 registered
[    0.273309] pps_core: Software ver. 5.3.6 - Copyright 2005-2007 Rodolfo Giometti <giometti@linux.it>
[    0.273368] PTP clock support registered
[    0.273952] EDAC MC: Ver: 3.0.0
[    0.284759] NetLabel: Initializing
[    0.284790] NetLabel:  domain hash size = 128
[    0.284812] NetLabel:  protocols = UNLABELED CIPSOv4 CALIPSO
[    0.285484] NetLabel:  unlabeled traffic allowed by default
[    0.289405] mctp: management component transport protocol core
[    0.289617] NET: Registered PF_MCTP protocol family
[    0.291023] vgaarb: loaded
[    0.294204] clocksource: Switched to clocksource riscv_clocksource
[    0.297747] VFS: Disk quotas dquot_6.6.0
[    0.297855] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
[    0.304196] AppArmor: AppArmor Filesystem Enabled
[    0.304441] pnp: PnP ACPI: disabled
[    0.324886] NET: Registered PF_INET protocol family
[    0.325854] IP idents hash table entries: 65536 (order: 7, 524288 bytes, linear)
[    0.363992] tcp_listen_portaddr_hash hash table entries: 2048 (order: 3, 32768 bytes, linear)
[    0.364126] Table-perturb hash table entries: 65536 (order: 6, 262144 bytes, linear)
[    0.364236] TCP established hash table entries: 32768 (order: 6, 262144 bytes, linear)
[    0.364530] TCP bind hash table entries: 32768 (order: 8, 1048576 bytes, linear)
[    0.364749] TCP: Hash tables configured (established 32768 bind 32768)
[    0.365792] MPTCP token hash table entries: 4096 (order: 5, 98304 bytes, linear)
[    0.366053] UDP hash table entries: 2048 (order: 5, 131072 bytes, linear)
[    0.366267] UDP-Lite hash table entries: 2048 (order: 5, 131072 bytes, linear)
[    0.367378] NET: Registered PF_UNIX/PF_LOCAL protocol family
[    0.367698] NET: Registered PF_XDP protocol family
[    0.367844] PCI: CLS 0 bytes, default 64
[    0.371276] Trying to unpack rootfs image as initramfs...
[    0.436413] Initialise system trusted keyrings
[    0.437625] Key type blacklist registered
[    0.438301] workingset: timestamp_bits=44 max_order=20 bucket_order=0
[    0.444090] squashfs: version 4.0 (2009/01/31) Phillip Lougher
[    0.445209] fuse: init (API version 7.44)
[    0.447443] integrity: Platform Keyring initialized
[    0.447658] integrity: Machine keyring initialized
[    0.501624] Key type asymmetric registered
[    0.501741] Asymmetric key parser 'x509' registered
[    0.502018] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 242)
[    0.506578] io scheduler mq-deadline registered
[    0.514587] riscv-plic: plic@c000000: mapped 95 interrupts with 2 handlers for 4 contexts.
[    0.517540] ledtrig-cpu: registered to indicate activity on CPUs
[    0.519280] pci-host-generic 30000000.pci: host bridge /soc/pci@30000000 ranges:
[    0.519781] pci-host-generic 30000000.pci:       IO 0x0003000000..0x000300ffff -> 0x0000000000
[    0.520165] pci-host-generic 30000000.pci:      MEM 0x0040000000..0x007fffffff -> 0x0040000000
[    0.520234] pci-host-generic 30000000.pci:      MEM 0x0400000000..0x07ffffffff -> 0x0400000000
[    0.520730] pci-host-generic 30000000.pci: Memory resource size exceeds max for 32 bits
[    0.521074] pci-host-generic 30000000.pci: ECAM at [mem 0x30000000-0x3fffffff] for [bus 00-ff]
[    0.522326] pci-host-generic 30000000.pci: PCI host bridge to bus 0000:00
[    0.522549] pci_bus 0000:00: root bus resource [bus 00-ff]
[    0.522616] pci_bus 0000:00: root bus resource [io  0x0000-0xffff]
[    0.522659] pci_bus 0000:00: root bus resource [mem 0x40000000-0x7fffffff]
[    0.522683] pci_bus 0000:00: root bus resource [mem 0x400000000-0x7ffffffff]
[    0.523677] pci 0000:00:00.0: [1b36:0008] type 00 class 0x060000 conventional PCI endpoint
[    0.527388] pci 0000:00:01.0: [1af4:1005] type 00 class 0x00ff00 conventional PCI endpoint
[    0.528134] pci 0000:00:01.0: BAR 0 [io  0x1000-0x101f]
[    0.528187] pci 0000:00:01.0: BAR 1 [mem 0x40000000-0x40000fff]
[    0.528230] pci 0000:00:01.0: BAR 4 [mem 0x40004000-0x40007fff 64bit pref]
[    0.528680] pci 0000:00:02.0: [1af4:1001] type 00 class 0x010000 conventional PCI endpoint
[    0.529254] pci 0000:00:02.0: BAR 0 [io  0x1080-0x10ff]
[    0.529295] pci 0000:00:02.0: BAR 1 [mem 0x40008000-0x40008fff]
[    0.529322] pci 0000:00:02.0: BAR 4 [mem 0x4000c000-0x4000ffff 64bit pref]
[    0.531160] pci 0000:00:01.0: BAR 4 [mem 0x400000000-0x400003fff 64bit pref]: assigned
[    0.531739] pci 0000:00:02.0: BAR 4 [mem 0x400004000-0x400007fff 64bit pref]: assigned
[    0.532166] pci 0000:00:01.0: BAR 1 [mem 0x40000000-0x40000fff]: assigned
[    0.532213] pci 0000:00:02.0: BAR 1 [mem 0x40001000-0x40001fff]: assigned
[    0.532438] pci 0000:00:02.0: BAR 0 [io  0x0080-0x00ff]: assigned
[    0.532683] pci 0000:00:01.0: BAR 0 [io  0x0020-0x003f]: assigned
[    0.533079] pci_bus 0000:00: resource 4 [io  0x0000-0xffff]
[    0.533130] pci_bus 0000:00: resource 5 [mem 0x40000000-0x7fffffff]
[    0.533156] pci_bus 0000:00: resource 6 [mem 0x400000000-0x7ffffffff]
[    0.543833] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
[    0.567448] 10000000.serial: ttyS0 at MMIO 0x10000000 (irq = 14, base_baud = 230400) is a 16550A
[    0.568774] printk: legacy console [ttyS0] enabled
[    0.606816] loop: module loaded
[    0.607712] virtio_blk virtio2: 2/0/0 default/read/poll queues
[    0.611163] virtio_blk virtio2: [vda] 9437184 512-byte logical blocks (4.83 GB/4.50 GiB)
[    0.620497]  vda: vda1 vda12 vda13 vda14 vda15
[    0.626452] tun: Universal TUN/TAP device driver, 1.6
[    0.630859] PPP generic driver version 2.4.2
[    0.633405] mousedev: PS/2 mouse device common for all mice
[    0.635741] goldfish_rtc 101000.rtc: registered as rtc0
[    0.636275] goldfish_rtc 101000.rtc: setting system clock to 2026-04-09T08:36:43 UTC (1775723803)
[    0.636862] i2c_dev: i2c /dev entries driver
[    0.639572] device-mapper: core: CONFIG_IMA_DISABLE_HTABLE is disabled. Duplicate IMA measurements will not be recorded in the IMA log.
[    0.639928] device-mapper: uevent: version 1.0.3
[    0.640535] device-mapper: ioctl: 4.50.0-ioctl (2025-04-28) initialised: dm-devel@lists.linux.dev
[    0.641323] EDAC DEVICE0: Giving out device to module Sifive ECC Manager controller sifive_edac.0: DEV sifive_edac.0 (INTERRUPT)
[    0.642410] riscv-pmu-sbi: SBI PMU extension is available
[    0.642846] riscv-pmu-sbi: 16 firmware and 18 hardware counters
[    0.646072] drop_monitor: Initializing network drop monitor service
[    0.646961] NET: Registered PF_INET6 protocol family
[    0.654521] Segment Routing with IPv6
[    0.654781] In-situ OAM (IOAM) with IPv6
[    0.655306] NET: Registered PF_PACKET protocol family
[    0.655865] Key type dns_resolver registered
[    0.669375] registered taskstats version 1
[    0.674679] Loading compiled-in X.509 certificates
[    0.681692] Loaded X.509 cert 'Build time autogenerated kernel key: 51491f302983480bf0b91225294747f9a1f1ac89'
[    0.682868] Loaded X.509 cert 'Canonical Ltd. Live Patch Signing 2025 Kmod: d541cef61dc7e793b7eb7e899970a2eef0b5dc8c'
[    0.683845] Loaded X.509 cert 'Canonical Ltd. Live Patch Signing: 14df34d1a87cf37625abec039ef2bf521249b969'
[    0.684818] Loaded X.509 cert 'Canonical Ltd. Kernel Module Signing: 88f752e560a1e0737e31163a466ad7b70a850c19'
[    0.685034] blacklist: Loading compiled-in revocation X.509 certificates
[    0.685931] Loaded X.509 cert 'Canonical Ltd. Secure Boot Signing: 61482aa2830d0ab2ad5af10b7250da9033ddcef0'
[    0.686648] Loaded X.509 cert 'Canonical Ltd. Secure Boot Signing (2017): 242ade75ac4a15e50d50c84b0d45ff3eae707a03'
[    0.686914] Loaded X.509 cert 'Canonical Ltd. Secure Boot Signing (ESM 2018): 365188c1d374d6b07c3c8f240f8ef722433d6a8b'
[    0.687198] Loaded X.509 cert 'Canonical Ltd. Secure Boot Signing (2019): c0746fd6c5da3ae827864651ad66ae47fe24b3e8'
[    0.687457] Loaded X.509 cert 'Canonical Ltd. Secure Boot Signing (2021 v1): a8d54bbb3825cfb94fa13c9f8a594a195c107b8d'
[    0.687721] Loaded X.509 cert 'Canonical Ltd. Secure Boot Signing (2021 v2): 4cf046892d6fd3c9a5b03f98d845f90851dc6a8c'
[    0.687992] Loaded X.509 cert 'Canonical Ltd. Secure Boot Signing (2021 v3): 100437bb6de6e469b581e61cd66bce3ef4ed53af'
[    0.688256] Loaded X.509 cert 'Canonical Ltd. Secure Boot Signing (Ubuntu Core 2019): c1d57b8f6b743f23ee41f4f7ee292f06eecadfb9'
[    0.721705] Demotion targets for Node 0: null
[    0.722514] Key type .fscrypt registered
[    0.722618] Key type fscrypt-provisioning registered
[    0.723505] Key type big_key registered
[    1.420336] Freeing initrd memory: 71684K
[    1.619371] Key type encrypted registered
[    1.619510] AppArmor: AppArmor sha256 policy hashing enabled
[    1.619873] ima: No TPM chip found, activating TPM-bypass!
[    1.620045] Loading compiled-in module X.509 certificates
[    1.620968] Loaded X.509 cert 'Build time autogenerated kernel key: 51491f302983480bf0b91225294747f9a1f1ac89'
[    1.621173] ima: Allocated hash algorithm: sha256
[    1.622802] ima: No architecture policies found
[    1.623294] evm: Initialising EVM extended attributes:
[    1.623396] evm: security.selinux
[    1.623466] evm: security.SMACK64
[    1.623536] evm: security.SMACK64EXEC
[    1.623609] evm: security.SMACK64TRANSMUTE
[    1.623692] evm: security.SMACK64MMAP
[    1.623765] evm: security.apparmor
[    1.623837] evm: security.ima
[    1.623899] evm: security.capability
[    1.623993] evm: HMAC attrs: 0x1
[    1.667070] clk: Disabling unused clocks
[    1.667305] PM: genpd: Disabling unused power domains
[    1.711375] Freeing unused kernel image (initmem) memory: 6584K
[    1.725126] Checked W+X mappings: passed, no W+X pages found
[    1.725395] Run /init as init process
Loading, please wait...
Starting systemd-udevd version 255.4-1ubuntu8.12
Begin: Loading essential drivers ... [    4.758287] raid6: rvvx1    gen()   750 MB/s
[    4.826094] raid6: rvvx2    gen()   818 MB/s
[    4.894050] raid6: rvvx4    gen()   789 MB/s
[    4.962093] raid6: rvvx8    gen()   557 MB/s
[    5.030052] raid6: int64x8  gen()  1842 MB/s
[    5.098061] raid6: int64x4  gen()  4245 MB/s
[    5.166061] raid6: int64x2  gen()  5244 MB/s
[    5.234062] raid6: int64x1  gen()  4166 MB/s
[    5.234139] raid6: using algorithm int64x2 gen() 5244 MB/s
[    5.302073] raid6: .... xor() 3008 MB/s, rmw enabled
[    5.302202] raid6: using rvv recovery algorithm
[    5.344599] xor: measuring software checksum speed
[    5.345310]    8regs           :  5733 MB/sec
[    5.345926]    32regs          :  6206 MB/sec
[    5.347685]    rvv             :  2132 MB/sec
[    5.349438] xor: using function: 32regs (6206 MB/sec)
[    5.372385] async_tx: api initialized (async)
done.
Begin: Running /scripts/init-premount ... done.
Begin: Mounting root file system ... Begin: Running /scripts/local-top ... done.
Begin: Running /scripts/local-premount ... [    5.853041] Btrfs loaded, zoned=yes, fsverity=yes
Scanning for Btrfs filesystems
done.
Begin: Will now check root file system ... fsck from util-linux 2.39.3
[/usr/sbin/fsck.ext4 (1) -- /dev/vda1] fsck.ext4 -a -C0 /dev/vda1
cloudimg-rootfs: clean, 98375/575424 files, 697802/1150203 blocks
done.
[    6.152225] EXT4-fs (vda1): mounted filesystem 04ac500d-f0a9-4bf4-acdb-1292f0c859e8 ro with ordered data mode. Quota mode: none.
done.
Begin: Running /scripts/local-bottom ... done.
Begin: Running /scripts/init-bottom ... done.
[    6.846383] systemd[1]: Inserted module 'autofs4'
[    6.996632] systemd[1]: systemd 255.4-1ubuntu8.12 running in system mode (+PAM +AUDIT +SELINUX +APPARMOR +IMA +SMACK +SECCOMP +GCRYPT -GNUTLS +OPENSSL +ACL +BLKID +CURL +ELFUTILS +FIDO2 +IDN2 -IDN +IPTC +KMOD +LIBCRYPTSETUP +LIBFDISK +PCRE2 -PWQUALITY +P11KIT +QRENCODE +TPM2 +BZIP2 +LZ4 +XZ +ZLIB +ZSTD -BPF_FRAMEWORK -XKBCOMMON +UTMP +SYSVINIT default-hierarchy=unified)
[    6.997148] systemd[1]: Detected virtualization qemu.
[    6.997338] systemd[1]: Detected architecture riscv64.

Welcome to Ubuntu 24.04.4 LTS!

[    7.002203] systemd[1]: Hostname set to <ubuntu>.
[    7.005068] systemd[1]: Initializing machine ID from random generator.
[    7.006119] systemd[1]: Installed transient /etc/machine-id file.
[    8.458496] systemd[1]: Queued start job for default target graphical.target.
[    8.520727] systemd[1]: Created slice system-modprobe.slice - Slice /system/modprobe.
[  OK  ] Created slice system-modprobe.slice - Slice /system/modprobe.
[    8.525814] systemd[1]: Created slice system-serial\x2dgetty.slice - Slice /system/serial-getty.
[  OK  ] Created slice system-serial\x2dget…slice - Slice /system/serial-getty.
[    8.528995] systemd[1]: Created slice system-systemd\x2dfsck.slice - Slice /system/systemd-fsck.
[  OK  ] Created slice system-systemd\x2dfs…slice - Slice /system/systemd-fsck.
[    8.531411] systemd[1]: Created slice user.slice - User and Session Slice.
[  OK  ] Created slice user.slice - User and Session Slice.
[    8.532926] systemd[1]: Started systemd-ask-password-wall.path - Forward Password Requests to Wall Directory Watch.
[  OK  ] Started systemd-ask-password-wall.…d Requests to Wall Directory Watch.
[    8.539343] systemd[1]: Set up automount proc-sys-fs-binfmt_misc.automount - Arbitrary Executable File Formats File System Automount Point.
[  OK  ] Set up automount proc-sys-fs-binfm…ormats File System Automount Point.
[    8.544600] systemd[1]: Expecting device dev-disk-by\x2dlabel-UEFI.device - /dev/disk/by-label/UEFI...
         Expecting device dev-disk-by\x2dla…device - /dev/disk/by-label/UEFI...
[    8.545401] systemd[1]: Expecting device dev-ttyS0.device - /dev/ttyS0...
         Expecting device dev-ttyS0.device - /dev/ttyS0...
[    8.546540] systemd[1]: Reached target integritysetup.target - Local Integrity Protected Volumes.
[  OK  ] Reached target integritysetup.targ… Local Integrity Protected Volumes.
[    8.547387] systemd[1]: Reached target slices.target - Slice Units.
[  OK  ] Reached target slices.target - Slice Units.
[    8.548327] systemd[1]: Reached target snapd.mounts-pre.target - Mounting snaps.
[  OK  ] Reached target snapd.mounts-pre.target - Mounting snaps.
[    8.548927] systemd[1]: Reached target snapd.mounts.target - Mounted snaps.
[  OK  ] Reached target snapd.mounts.target - Mounted snaps.
[    8.549559] systemd[1]: Reached target swap.target - Swaps.
[  OK  ] Reached target swap.target - Swaps.
[    8.550232] systemd[1]: Reached target veritysetup.target - Local Verity Protected Volumes.
[  OK  ] Reached target veritysetup.target - Local Verity Protected Volumes.
[    8.553499] systemd[1]: Listening on dm-event.socket - Device-mapper event daemon FIFOs.
[  OK  ] Listening on dm-event.socket - Device-mapper event daemon FIFOs.
[    8.558077] systemd[1]: Listening on lvm2-lvmpolld.socket - LVM2 poll daemon socket.
[  OK  ] Listening on lvm2-lvmpolld.socket - LVM2 poll daemon socket.
[    8.560126] systemd[1]: Listening on multipathd.socket - multipathd control socket.
[  OK  ] Listening on multipathd.socket - multipathd control socket.
[    8.562064] systemd[1]: Listening on syslog.socket - Syslog Socket.
[  OK  ] Listening on syslog.socket - Syslog Socket.
[    8.563272] systemd[1]: Listening on systemd-fsckd.socket - fsck to fsckd communication Socket.
[  OK  ] Listening on systemd-fsckd.socket …fsck to fsckd communication Socket.
[    8.564399] systemd[1]: Listening on systemd-initctl.socket - initctl Compatibility Named Pipe.
[  OK  ] Listening on systemd-initctl.socke…- initctl Compatibility Named Pipe.
[    8.566058] systemd[1]: Listening on systemd-journald-dev-log.socket - Journal Socket (/dev/log).
[  OK  ] Listening on systemd-journald-dev-…socket - Journal Socket (/dev/log).
[    8.567559] systemd[1]: Listening on systemd-journald.socket - Journal Socket.
[  OK  ] Listening on systemd-journald.socket - Journal Socket.
[    8.570338] systemd[1]: Listening on systemd-networkd.socket - Network Service Netlink Socket.
[  OK  ] Listening on systemd-networkd.socket - Network Service Netlink Socket.
[    8.571340] systemd[1]: systemd-pcrextend.socket - TPM2 PCR Extension (Varlink) was skipped because of an unmet condition check (ConditionSecurity=measured-uki).
[    8.573047] systemd[1]: Listening on systemd-udevd-control.socket - udev Control Socket.
[  OK  ] Listening on systemd-udevd-control.socket - udev Control Socket.
[    8.574468] systemd[1]: Listening on systemd-udevd-kernel.socket - udev Kernel Socket.
[  OK  ] Listening on systemd-udevd-kernel.socket - udev Kernel Socket.
[    8.607201] systemd[1]: Mounting dev-hugepages.mount - Huge Pages File System...
         Mounting dev-hugepages.mount - Huge Pages File System...
[    8.618414] systemd[1]: Mounting dev-mqueue.mount - POSIX Message Queue File System...
         Mounting dev-mqueue.mount - POSIX Message Queue File System...
[    8.636777] systemd[1]: Mounting sys-kernel-debug.mount - Kernel Debug File System...
         Mounting sys-kernel-debug.mount - Kernel Debug File System...
[    8.651918] systemd[1]: Mounting sys-kernel-tracing.mount - Kernel Trace File System...
         Mounting sys-kernel-tracing.mount - Kernel Trace File System...
[    8.735886] systemd[1]: Starting systemd-journald.service - Journal Service...
         Starting systemd-journald.service - Journal Service...
[    8.766866] systemd[1]: Starting keyboard-setup.service - Set the console keyboard layout...
         Starting keyboard-setup.service - Set the console keyboard layout...
[    8.797139] systemd[1]: Starting kmod-static-nodes.service - Create List of Static Device Nodes...
         Starting kmod-static-nodes.service…eate List of Static Device Nodes...
[    8.823145] systemd[1]: Starting lvm2-monitor.service - Monitoring of LVM2 mirrors, snapshots etc. using dmeventd or progress polling...
         Starting lvm2-monitor.service - Mo…ing dmeventd or progress polling...
[    8.851114] systemd[1]: Starting modprobe@configfs.service - Load Kernel Module configfs...
         Starting modprobe@configfs.service - Load Kernel Module configfs...
[    8.902966] systemd[1]: Starting modprobe@dm_mod.service - Load Kernel Module dm_mod...
         Starting modprobe@dm_mod.service - Load Kernel Module dm_mod...
[    8.943175] systemd[1]: Starting modprobe@drm.service - Load Kernel Module drm...
[    8.943239] systemd-journald[295]: Collecting audit messages is disabled.
         Starting modprobe@drm.service - Load Kernel Module drm...
[    9.090847] systemd[1]: Starting modprobe@efi_pstore.service - Load Kernel Module efi_pstore...
         Starting modprobe@efi_pstore.servi… - Load Kernel Module efi_pstore...
[    9.142997] systemd[1]: Starting modprobe@fuse.service - Load Kernel Module fuse...
         Starting modprobe@fuse.service - Load Kernel Module fuse...
[    9.199241] systemd[1]: Starting modprobe@loop.service - Load Kernel Module loop...
         Starting modprobe@loop.service - Load Kernel Module loop...
[    9.203423] systemd[1]: netplan-ovs-cleanup.service - OpenVSwitch configuration for cleanup was skipped because of an unmet condition check (ConditionFileIsExecutable=/usr/bin/ovs-vsctl).
[    9.204712] systemd[1]: systemd-fsck-root.service - File System Check on Root Device was skipped because of an unmet condition check (ConditionPathExists=!/run/initramfs/fsck-root).
[    9.302834] systemd[1]: Starting systemd-modules-load.service - Load Kernel Modules...
         Starting systemd-modules-load.service - Load Kernel Modules...
[    9.306381] systemd[1]: systemd-pcrmachine.service - TPM2 PCR Machine ID Measurement was skipped because of an unmet condition check (ConditionSecurity=measured-uki).
[    9.346765] systemd[1]: Starting systemd-remount-fs.service - Remount Root and Kernel File Systems...
         Starting systemd-remount-fs.servic…unt Root and Kernel File Systems...
[    9.347731] systemd[1]: systemd-tpm2-setup-early.service - TPM2 SRK Setup (Early) was skipped because of an unmet condition check (ConditionSecurity=measured-uki).
[    9.393525] systemd[1]: Starting systemd-udev-trigger.service - Coldplug All udev Devices...
         Starting systemd-udev-trigger.service - Coldplug All udev Devices...
[    9.448208] systemd[1]: Started systemd-journald.service - Journal Service.
[  OK  ] Started systemd-journald.service - Journal Service.
[  OK  ] Mounted dev-hugepages.mount - Huge Pages File System.
[  OK  ] Mounted dev-mqueue.mount - POSIX Message Queue File System.
[  OK  ] Mounted sys-kernel-debug.mount - Kernel Debug File System.
[  OK  ] Mounted sys-kernel-tracing.mount - Kernel Trace File System.
[  OK  ] Finished kmod-static-nodes.service…Create List of Static Device Nodes.
[    9.504823] EXT4-fs (vda1): re-mounted 04ac500d-f0a9-4bf4-acdb-1292f0c859e8 r/w.
[  OK  ] Finished lvm2-monitor.service - Mo…using dmeventd or progress polling.
[  OK  ] Finished modprobe@configfs.service - Load Kernel Module configfs.
[  OK  ] Finished modprobe@dm_mod.service - Load Kernel Module dm_mod.
[  OK  ] Finished modprobe@drm.service - Load Kernel Module drm.
[  OK  ] Finished modprobe@efi_pstore.service - Load Kernel Module efi_pstore.
[  OK  ] Finished modprobe@fuse.service - Load Kernel Module fuse.
[  OK  ] Finished modprobe@loop.service - Load Kernel Module loop.
[  OK  ] Finished systemd-modules-load.service - Load Kernel Modules.
[  OK  ] Finished systemd-remount-fs.servic…mount Root and Kernel File Systems.
         Mounting sys-fs-fuse-connections.mount - FUSE Control File System...
         Mounting sys-kernel-config.mount - Kernel Configuration File System...
         Starting cloud-init-local.service …-init: Local Stage (pre-network)...
         Starting multipathd.service - Devi…pper Multipath Device Controller...
         Starting systemd-journal-flush.ser…sh Journal to Persistent Storage...
         Starting systemd-random-seed.service - Load/Save OS Random Seed...
         Starting systemd-sysctl.service - Apply Kernel Variables...
         Starting systemd-tmpfiles-setup-de… Device Nodes in /dev gracefully...
[  OK  ] Finished keyboard-setup.service - Set the console keyboard layout.
[  OK  ] Mounted sys-fs-fuse-connections.mount - FUSE Control File System.
[  OK  ] Mounted sys-kernel-config.mount - Kernel Configuration File System.
[   10.156276] systemd-journald[295]: Received client request to flush runtime journal.
[  OK  ] Finished systemd-random-seed.service - Load/Save OS Random Seed.
[  OK  ] Finished systemd-sysctl.service - Apply Kernel Variables.
[  OK  ] Finished systemd-journal-flush.ser…lush Journal to Persistent Storage.
[  OK  ] Finished systemd-tmpfiles-setup-de…ic Device Nodes in /dev gracefully.
[  OK  ] Finished systemd-udev-trigger.service - Coldplug All udev Devices.
         Starting systemd-sysusers.service - Create System Users...
[  OK  ] Started multipathd.service - Devic…Mapper Multipath Device Controller.
[  OK  ] Finished systemd-sysusers.service - Create System Users.
         Starting systemd-tmpfiles-setup-de…eate Static Device Nodes in /dev...
[  OK  ] Finished systemd-tmpfiles-setup-de…Create Static Device Nodes in /dev.
[  OK  ] Reached target local-fs-pre.target…Preparation for Local File Systems.
         Starting systemd-udevd.service - R…ager for Device Events and Files...
[  OK  ] Started systemd-udevd.service - Ru…anager for Device Events and Files.
[  OK  ] Started systemd-ask-password-conso…equests to Console Directory Watch.
[  OK  ] Reached target cryptsetup.target - Local Encrypted Volumes.
[  OK  ] Found device dev-ttyS0.device - /dev/ttyS0.
[  OK  ] Found device dev-disk-by\x2dlabel-…I.device - /dev/disk/by-label/UEFI.
[   14.554764] cloud-init[420]: Cloud-init v. 25.2-0ubuntu1~24.04.1 running 'init-local' at Thu, 09 Apr 2026 08:36:57 +0000. Up 14.33 seconds.
         Starting systemd-fsck@dev-disk-by\…Check on /dev/disk/by-label/UEFI...
[  OK  ] Started systemd-fsckd.service - Fi…stem Check Daemon to report status.
[  OK  ] Listening on systemd-rfkill.socket…ll Switch Status /dev/rfkill Watch.
[  OK  ] Finished systemd-fsck@dev-disk-by\…m Check on /dev/disk/by-label/UEFI.
         Mounting boot-efi.mount - /boot/efi...
[  OK  ] Mounted boot-efi.mount - /boot/efi.
[  OK  ] Reached target local-fs.target - Local File Systems.
[  OK  ] Listening on systemd-sysext.socket…tension Image Management (Varlink).
         Starting apparmor.service - Load AppArmor profiles...
         Starting console-setup.service - Set console font and keymap...
         Starting finalrd.service - Create …time dir for shutdown pivot root...
[   15.331567] cloud-init[420]: 2026-04-09 08:36:58,212 - DataSourceNoCloud.py[WARNING]: device /dev/vda12 with label=cidata not a valid seed.
         Starting ldconfig.service - Rebuild Dynamic Linker Cache...
         Starting plymouth-read-write.servi…ymouth To Write Out Runtime Data...
         Starting systemd-binfmt.service - Set Up Additional Binary Formats...
         Starting systemd-tmpfiles-setup.se…e Volatile Files and Directories...
         Starting ufw.service - Uncomplicated firewall...
[  OK  ] Finished console-setup.service - Set console font and keymap.
[  OK  ] Finished finalrd.service - Create …untime dir for shutdown pivot root.
[  OK  ] Finished plymouth-read-write.servi…Plymouth To Write Out Runtime Data.
         Mounting proc-sys-fs-binfmt_misc.m…cutable File Formats File System...
[  OK  ] Finished ufw.service - Uncomplicated firewall.
[  OK  ] Mounted proc-sys-fs-binfmt_misc.mo…xecutable File Formats File System.
[  OK  ] Finished systemd-binfmt.service - Set Up Additional Binary Formats.
[  OK  ] Finished systemd-tmpfiles-setup.se…ate Volatile Files and Directories.
         Starting systemd-journal-catalog-u…ervice - Rebuild Journal Catalog...
         Starting systemd-machine-id-commit…t a transient machine-id on disk...
         Starting systemd-resolved.service - Network Name Resolution...
         Starting systemd-timesyncd.service - Network Time Synchronization...
         Starting systemd-update-utmp.servi…ord System Boot/Shutdown in UTMP...
[  OK  ] Finished ldconfig.service - Rebuild Dynamic Linker Cache.
[  OK  ] Finished systemd-journal-catalog-u….service - Rebuild Journal Catalog.
[  OK  ] Finished systemd-machine-id-commit…mit a transient machine-id on disk.
         Starting systemd-update-done.service - Update is Completed...
[  OK  ] Finished systemd-update-utmp.servi…ecord System Boot/Shutdown in UTMP.
[  OK  ] Finished systemd-update-done.service - Update is Completed.
[  OK  ] Started systemd-timesyncd.service - Network Time Synchronization.
[  OK  ] Reached target time-set.target - System Time Set.
[  OK  ] Started systemd-resolved.service - Network Name Resolution.
[  OK  ] Reached target nss-lookup.target - Host and Network Name Lookups.
[  OK  ] Finished cloud-init-local.service …ud-init: Local Stage (pre-network).
[  OK  ] Reached target network-pre.target - Preparation for Network.
         Starting systemd-networkd.service - Network Configuration...
[  OK  ] Started systemd-networkd.service - Network Configuration.
         Starting cloud-init.service - Cloud-init: Network Stage...
[  OK  ] Finished apparmor.service - Load AppArmor profiles.
         Starting snapd.apparmor.service - …iles managed internally by snapd...
[  OK  ] Finished snapd.apparmor.service - …ofiles managed internally by snapd.
[   26.355300] cloud-init[658]: Cloud-init v. 25.2-0ubuntu1~24.04.1 running 'init' at Thu, 09 Apr 2026 08:37:09 +0000. Up 26.20 seconds.
[   26.432650] cloud-init[658]: ci-info: ++++++++++++++++++++++++++++++++++++++Net device info++++++++++++++++++++++++++++++++++++++
[   26.433897] cloud-init[658]: ci-info: +--------+------+----------------------------+---------------+--------+-------------------+
[   26.434749] cloud-init[658]: ci-info: | Device |  Up  |          Address           |      Mask     | Scope  |     Hw-Address    |
[   26.435496] cloud-init[658]: ci-info: +--------+------+----------------------------+---------------+--------+-------------------+
[   26.436270] cloud-init[658]: ci-info: |  eth0  | True |         10.0.2.15          | 255.255.255.0 | global | 52:54:00:12:34:56 |
[   26.437047] cloud-init[658]: ci-info: |  eth0  | True | fec0::5054:ff:fe12:3456/64 |       .       |  site  | 52:54:00:12:34:56 |
[   26.437739] cloud-init[658]: ci-info: |  eth0  | True | fe80::5054:ff:fe12:3456/64 |       .       |  link  | 52:54:00:12:34:56 |
[   26.438450] cloud-init[658]: ci-info: |   lo   | True |         127.0.0.1          |   255.0.0.0   |  host  |         .         |
[   26.439250] cloud-init[658]: ci-info: |   lo   | True |          ::1/128           |       .       |  host  |         .         |
[   26.439938] cloud-init[658]: ci-info: +--------+------+----------------------------+---------------+--------+-------------------+
[   26.440622] cloud-init[658]: ci-info: ++++++++++++++++++++++++++++Route IPv4 info+++++++++++++++++++++++++++++
[   26.441359] cloud-init[658]: ci-info: +-------+-------------+----------+-----------------+-----------+-------+
[   26.442121] cloud-init[658]: ci-info: | Route | Destination | Gateway  |     Genmask     | Interface | Flags |
[   26.442800] cloud-init[658]: ci-info: +-------+-------------+----------+-----------------+-----------+-------+
[   26.443368] cloud-init[658]: ci-info: |   0   |   0.0.0.0   | 10.0.2.2 |     0.0.0.0     |    eth0   |   UG  |
[   26.443963] cloud-init[658]: ci-info: |   1   |   10.0.2.0  | 0.0.0.0  |  255.255.255.0  |    eth0   |   U   |
[   26.444660] cloud-init[658]: ci-info: |   2   |   10.0.2.2  | 0.0.0.0  | 255.255.255.255 |    eth0   |   UH  |
[   26.445362] cloud-init[658]: ci-info: |   3   |   10.0.2.3  | 0.0.0.0  | 255.255.255.255 |    eth0   |   UH  |
[   26.446040] cloud-init[658]: ci-info: +-------+-------------+----------+-----------------+-----------+-------+
[   26.446623] cloud-init[658]: ci-info: +++++++++++++++++++Route IPv6 info+++++++++++++++++++
[   26.447272] cloud-init[658]: ci-info: +-------+-------------+---------+-----------+-------+
[   26.447907] cloud-init[658]: ci-info: | Route | Destination | Gateway | Interface | Flags |
[   26.448735] cloud-init[658]: ci-info: +-------+-------------+---------+-----------+-------+
[   26.449493] cloud-init[658]: ci-info: |   0   |  fe80::/64  |    ::   |    eth0   |   U   |
[   26.450057] cloud-init[658]: ci-info: |   1   |  fec0::/64  |    ::   |    eth0   |   Ue  |
[   26.450695] cloud-init[658]: ci-info: |   2   |     ::/0    | fe80::2 |    eth0   |  UGe  |
[   26.451434] cloud-init[658]: ci-info: |   4   |    local    |    ::   |    eth0   |   U   |
[   26.452358] cloud-init[658]: ci-info: |   5   |  multicast  |    ::   |    eth0   |   U   |
[   26.452987] cloud-init[658]: ci-info: +-------+-------------+---------+-----------+-------+
[   26.871387] cloud-init[658]: 2026-04-09 08:37:09,752 - loggers.py[DEPRECATED]: Deprecated cloud-config provided: chpasswd.list:  Deprecated in version 22.2. Use **users** instead.
[   31.798331] cloud-init[658]: 2026-04-09 08:37:14,677 - lifecycle.py[DEPRECATED]: Config key 'lists' is deprecated in 22.3 and scheduled to be removed in 27.3. Use 'users' instead.
[   32.232159] cloud-init[658]: Generating public/private rsa key pair.
[   32.233256] cloud-init[658]: Your identification has been saved in /etc/ssh/ssh_host_rsa_key
[   32.234081] cloud-init[658]: Your public key has been saved in /etc/ssh/ssh_host_rsa_key.pub
[   32.234759] cloud-init[658]: The key fingerprint is:
[   32.235294] cloud-init[658]: SHA256:zBVGUuDwKy7rezusoU5GDAbZGHO1G6BDQdigjiXVdCY root@ubuntu
[   32.235777] cloud-init[658]: The key's randomart image is:
[   32.236562] cloud-init[658]: +---[RSA 3072]----+
[   32.237171] cloud-init[658]: |B@=+E + o+=      |
[   32.237674] cloud-init[658]: |B=o..= + o .     |
[   32.238176] cloud-init[658]: |*.. o   o .      |
[   32.238654] cloud-init[658]: |+*   o o o       |
[   32.239131] cloud-init[658]: |..o . . S        |
[   32.239716] cloud-init[658]: | .   . .         |
[   32.240484] cloud-init[658]: |  o o..          |
[   32.241076] cloud-init[658]: | o . ++          |
[   32.241549] cloud-init[658]: | .o.=+.o         |
[   32.242193] cloud-init[658]: +----[SHA256]-----+
[   32.242854] cloud-init[658]: Generating public/private ecdsa key pair.
[   32.243469] cloud-init[658]: Your identification has been saved in /etc/ssh/ssh_host_ecdsa_key
[   32.244153] cloud-init[658]: Your public key has been saved in /etc/ssh/ssh_host_ecdsa_key.pub
[   32.244848] cloud-init[658]: The key fingerprint is:
[   32.245429] cloud-init[658]: SHA256:QxwPRDmRqAZ78K6XsZpfRuTz1pYA50bn6OZH+r3qT8Y root@ubuntu
[   32.245962] cloud-init[658]: The key's randomart image is:
[   32.246445] cloud-init[658]: +---[ECDSA 256]---+
[   32.247038] cloud-init[658]: |       +*+       |
[   32.247612] cloud-init[658]: |  o   ..++       |
[   32.248272] cloud-init[658]: |   = .o =.o      |
[   32.248837] cloud-init[658]: |  . =o * +       |
[   32.249460] cloud-init[658]: |   +  + S .      |
[   32.250047] cloud-init[658]: |    o. = +.o     |
[   32.250633] cloud-init[658]: |   . +o =o+ E    |
[   32.251304] cloud-init[658]: |  ..+o +...+     |
[   32.252131] cloud-init[658]: |  o+.   .++o+.   |
[   32.252722] cloud-init[658]: +----[SHA256]-----+
[   32.253235] cloud-init[658]: Generating public/private ed25519 key pair.
[   32.253876] cloud-init[658]: Your identification has been saved in /etc/ssh/ssh_host_ed25519_key
[   32.254525] cloud-init[658]: Your public key has been saved in /etc/ssh/ssh_host_ed25519_key.pub
[   32.255082] cloud-init[658]: The key fingerprint is:
[   32.255561] cloud-init[658]: SHA256:P8Sy/yy56+vlHzyqyYBlugNUpDDfpTgDluyGN2Y7SCA root@ubuntu
[   32.256180] cloud-init[658]: The key's randomart image is:
[   32.256682] cloud-init[658]: +--[ED25519 256]--+
[   32.257158] cloud-init[658]: | .=. .. .        |
[   32.257628] cloud-init[658]: |E.o= +.o         |
[   32.258103] cloud-init[658]: |oo  *.o          |
[   32.258712] cloud-init[658]: |..B .o   .       |
[   32.259454] cloud-init[658]: |.* +    S o      |
[   32.260114] cloud-init[658]: |. o .  = =   .   |
[   32.260741] cloud-init[658]: |   . .o o o.. +  |
[   32.261412] cloud-init[658]: |      .. +o* . o |
[   32.261989] cloud-init[658]: |      .. o@B=..  |
[   32.262590] cloud-init[658]: +----[SHA256]-----+
[  OK  ] Finished cloud-init.service - Cloud-init: Network Stage.
[  OK  ] Reached target cloud-config.target - Cloud-config availability.
[  OK  ] Reached target sysinit.target - System Initialization.
[  OK  ] Started apt-daily.timer - Daily apt download activities.
[  OK  ] Started apt-daily-upgrade.timer - …y apt upgrade and clean activities.
[  OK  ] Started dpkg-db-backup.timer - Daily dpkg database backup timer.
[  OK  ] Started e2scrub_all.timer - Period…Metadata Check for All Filesystems.
[  OK  ] Started fstrim.timer - Discard unused filesystem blocks once a week.
[  OK  ] Started fwupd-refresh.timer - Refresh fwupd metadata regularly.
[  OK  ] Started logrotate.timer - Daily rotation of log files.
[  OK  ] Started man-db.timer - Daily man-db regeneration.
[  OK  ] Started motd-news.timer - Message of the Day.
[  OK  ] Started sysstat-collect.timer - Ru…y accounting tool every 10 minutes.
[  OK  ] Started sysstat-summary.timer - Ge… of yesterday's process accounting.
[  OK  ] Started systemd-tmpfiles-clean.tim…y Cleanup of Temporary Directories.
[  OK  ] Reached target paths.target - Path Units.
[  OK  ] Listening on cloud-init-hotplugd.s…t - cloud-init hotplug hook socket.
[  OK  ] Listening on dbus.socket - D-Bus System Message Bus Socket.
[  OK  ] Listening on iscsid.socket - Open-iSCSI iscsid Socket.
         Starting lxd-installer.socket - He…er to install lxd snap on demand...
         Starting snapd.socket - Socket activation for snappy daemon...
[  OK  ] Listening on ssh.socket - OpenBSD Secure Shell server socket.
[  OK  ] Listening on uuidd.socket - UUID daemon activation socket.
[  OK  ] Listening on lxd-installer.socket …lper to install lxd snap on demand.
[  OK  ] Listening on snapd.socket - Socket activation for snappy daemon.
[  OK  ] Reached target sockets.target - Socket Units.
[  OK  ] Reached target basic.target - Basic System.
         Starting dbus.service - D-Bus System Message Bus...
[  OK  ] Started dmesg.service - Save initial kernel messages after boot.
         Starting e2scrub_reap.service - Re…ne ext4 Metadata Check Snapshots...
         Starting grub-common.service - Record successful boot for GRUB...
         Starting polkit.service - Authorization Manager...
         Starting rsyslog.service - System Logging Service...
[  OK  ] Reached target getty-pre.target - Preparation for Logins.
         Starting snapd.seeded.service - Wait until snapd is fully seeded...
         Starting snapd.service - Snap Daemon...
         Starting sysstat.service - Resets System Activity Logs...
         Starting systemd-logind.service - User Login Management...
         Starting udisks2.service - Disk Manager...
[  OK  ] Started dbus.service - D-Bus System Message Bus.
[  OK  ] Finished e2scrub_reap.service - Re…line ext4 Metadata Check Snapshots.
[  OK  ] Finished sysstat.service - Resets System Activity Logs.
         Starting wpa_supplicant.service - WPA supplicant...
[  OK  ] Started wpa_supplicant.service - WPA supplicant.
[  OK  ] Reached target network.target - Network.
[  OK  ] Reached target network-online.target - Network is Online.
[  OK  ] Started update-notifier-download.t…hat failed at package install time.
[  OK  ] Started update-notifier-motd.timer… a new version of Ubuntu available.
[  OK  ] Reached target timers.target - Timer Units.
         Starting cloud-config.service - Cloud-init: Config Stage...
[  OK  ] Reached target remote-fs-pre.targe…reparation for Remote File Systems.
[  OK  ] Reached target remote-fs.target - Remote File Systems.
         Starting apport.service - automatic crash report generation...
[  OK  ] Finished blk-availability.service - Availability of block devices.
[  OK  ] Started cron.service - Regular background program processing daemon.
         Starting pollinate.service - Polli…e pseudo random number generator...
         Starting systemd-user-sessions.service - Permit User Sessions...
[  OK  ] Started systemd-logind.service - User Login Management.
[  OK  ] Finished grub-common.service - Record successful boot for GRUB.
[  OK  ] Started polkit.service - Authorization Manager.
         Starting ModemManager.service - Modem Manager...
         Starting grub-initrd-fallback.service - GRUB failed boot detection...
[  OK  ] Started unattended-upgrades.service - Unattended Upgrades Shutdown.
[  OK  ] Started rsyslog.service - System Logging Service.
[  OK  ] Finished systemd-user-sessions.service - Permit User Sessions.
         Starting plymouth-quit-wait.servic…d until boot process finishes up...
         Starting plymouth-quit.service - Terminate Plymouth Boot Screen...
[  OK  ] Started udisks2.service - Disk Manager.
[  OK  ] Finished plymouth-quit-wait.servic…old until boot process finishes up.
[  OK  ] Started serial-getty@ttyS0.service - Serial Getty on ttyS0.
         Starting setvtrgb.service - Set console scheme...
[  OK  ] Finished plymouth-quit.service - Terminate Plymouth Boot Screen.
[  OK  ] Finished grub-initrd-fallback.service - GRUB failed boot detection.
[  OK  ] Started ModemManager.service - Modem Manager.
[  OK  ] Finished setvtrgb.service - Set console scheme.
[  OK  ] Created slice system-getty.slice - Slice /system/getty.
[  OK  ] Started getty@tty1.service - Getty on tty1.
[  OK  ] Reached target getty.target - Login Prompts.

Ubuntu 24.04.4 LTS ubuntu ttyS0

ubuntu login: [   40.561683] cloud-init[933]: Cloud-init v. 25.2-0ubuntu1~24.04.1 running 'modules:config' at Thu, 09 Apr 2026 08:37:23 +0000. Up 40.27 seconds.
[   45.228139] cloud-init[963]: Cloud-init v. 25.2-0ubuntu1~24.04.1 running 'modules:final' at Thu, 09 Apr 2026 08:37:27 +0000. Up 44.93 seconds.
ci-info: no authorized SSH keys fingerprints found for user ubuntu.
<14>Apr  9 08:37:28 cloud-init: #############################################################
<14>Apr  9 08:37:28 cloud-init: -----BEGIN SSH HOST KEY FINGERPRINTS-----
<14>Apr  9 08:37:28 cloud-init: 256 SHA256:QxwPRDmRqAZ78K6XsZpfRuTz1pYA50bn6OZH+r3qT8Y root@ubuntu (ECDSA)
<14>Apr  9 08:37:28 cloud-init: 256 SHA256:P8Sy/yy56+vlHzyqyYBlugNUpDDfpTgDluyGN2Y7SCA root@ubuntu (ED25519)
<14>Apr  9 08:37:28 cloud-init: 3072 SHA256:zBVGUuDwKy7rezusoU5GDAbZGHO1G6BDQdigjiXVdCY root@ubuntu (RSA)
<14>Apr  9 08:37:28 cloud-init: -----END SSH HOST KEY FINGERPRINTS-----
<14>Apr  9 08:37:28 cloud-init: #############################################################
-----BEGIN SSH HOST KEY KEYS-----
ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBAgfqe6FOstZvmmb0pmL7uWTRXrhzKS8O7TcNJKpiTDurcIRC2h0YqbPFs8SBiJa84mqXLw6eiOPCVDQZ0tRtFs= root@ubuntu
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIK6AJvdzM/Pv2hwiZ7yyJU/1AScgBSWBjfKL9CkP0vxA root@ubuntu
ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQDSW7s2Qx3UoDCsyvtIDo/lO9AbAZ6a1YozA+2CtpQWWyEFX9tGshzt1CwEUCExWnJzhX7D6ULYOMd9qoBf+0q/3TLAZsfU9vV8ICbs4s+q+Tg0MEmDc7tLikiefVEN0ssJL8l6tUfliMxoDcyJMps+ALXCawSV9m+mErkEgbiG2dKfBmXC1CyKy3QzrvKRLUYwxxopZ0lQyPf3njVGqeedN23nfh/4dsc3VjVG1QV4fJnu7d3KI1hMYIYR/pNOy3JqjDJZKBsG5MvDHw8Wg3Nw413gTpSIhAxY6Raj06oVtLd+erN5z1LV/4reyDdu8xlbc8nBl5rKxc6XWITXt+h9OY7ef9zsclciqCU0s4T9wBX6aYOE90KyacTPceWJpqLET/oTOK2+7E6UcWN9CdqfVp1rm/cYuLzbszpawhbYrL4tLnzQn1DSSoDP0bSaBZiyE4//EGp3rHAp5Zkhvr4azGDtwdM5lgsl++wW92Q4rhRiQrds0Sn53tiMWUn4ri0= root@ubuntu
-----END SSH HOST KEY KEYS-----
[   45.686635] cloud-init[963]: Cloud-init v. 25.2-0ubuntu1~24.04.1 finished at Thu, 09 Apr 2026 08:37:28 +0000. Datasource DataSourceNoCloud [seed=/var/lib/cloud/seed/nocloud-net].  Up 45.64 seconds

ubuntu login: ubuntu
Password:
Login timed out after 60 seconds.

Ubuntu 24.04.4 LTS ubuntu ttyS0

ubuntu login:
```
