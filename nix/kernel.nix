{
  lib,
  buildPackages,
  linux_6_12,
}:
linux_6_12.override {
  autoModules = false;
  preferBuiltin = true;
  buildFlags = [ "Image" ];
  installFlags = [ "install" ];
  postInstall = "";
  structuredExtraConfig = with buildPackages.lib.kernel; {
    SMP = yes;
    PRINTK = yes;
    BLK_DEV = yes;
    TTY = yes;
    UNIX = yes;
    PROC_FS = yes;
    SYSFS = yes;
    TMPFS = yes;
    DEVTMPFS = yes;
    DEVTMPFS_MOUNT = yes;
    CGROUPS = yes;
    MULTIUSER = yes;
    SYSVIPC = yes;
    BLK_DEV_INITRD = yes;
    RD_GZIP = yes;
    BLOCK = yes;
    EXT4_FS = yes;
    MSDOS_FS = yes;
    VFAT_FS = yes;
    VIRTIO = yes;
    VIRTIO_MENU = yes;
    VIRTIO_PCI = yes;
    VIRTIO_MMIO = yes;
    VIRTIO_BLK = yes;
    VIRTIO_NET = yes;
    VIRTIO_CONSOLE = yes;
    SERIAL_8250 = yes;
    SERIAL_8250_CONSOLE = yes;
    SERIAL_OF_PLATFORM = yes;
    NET = yes;
    INET = yes;
    PACKET = yes;
    NETDEVICES = yes;
    SOUND = no;
    DRM = lib.mkForce no;
    USB_SUPPORT = lib.mkForce no;
    WIRELESS = no;
    WLAN = no;
    BLUETOOTH = no;
    NFS_FS = lib.mkForce no;
    SECURITY_SELINUX = no;
  };
}
