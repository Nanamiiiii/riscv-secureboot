{ buildPackages, busybox }:
let
  busyboxStatic = busybox.override {
    enableStatic = true;
  };
  init = buildPackages.writeScript "init" ''
    #!/bin/busybox sh
    /bin/busybox mkdir -p /proc /sys /dev /mnt/root /run
    /bin/busybox mount -t proc proc /proc
    /bin/busybox mount -t sysfs sysfs /sys
    /bin/busybox mount -t devtmpfs devtmpfs /dev

    root_label=""
    want_shell=""
    for param in $(cat /proc/cmdline); do
      case "$param" in
        root=LABEL=*) root_label="''${param#root=LABEL=}" ;;
        break|rd.shell) want_shell=1 ;;
      esac
    done

    [ -n "$want_shell" ] && {
      echo "Dropping to debug shell (exit to continue boot)..."
      exec /bin/sh
    }

    [ -z "$root_label" ] && { echo "No root=LABEL= found"; exec /bin/sh; }

    echo "Waiting for LABEL=$root_label ..."
    root_dev=""
    attempts=0
    while [ -z "$root_dev" ] && [ $attempts -lt 30 ]; do
      for dev in /dev/vda*; do
        [ -b "$dev" ] || continue
        if /bin/busybox blkid "$dev" 2>/dev/null | /bin/busybox grep -q "LABEL=\"$root_label\""; then
          root_dev="$dev"
          break
        fi
      done
      [ -z "$root_dev" ] && { sleep 0.2; attempts=$((attempts + 1)); }
    done

    [ -z "$root_dev" ] && { echo "LABEL=$root_label not found"; exec /bin/sh; }

    echo "Checking $root_dev ..."
    /bin/busybox fsck -y "$root_dev" 2>/dev/null || true

    echo "Mounting $root_dev ..."
    /bin/busybox mount -t ext4 "$root_dev" /mnt/root
    exec /bin/busybox switch_root /mnt/root /sbin/init
  '';
in
buildPackages.runCommand "initrd.img"
  {
    nativeBuildInputs = with buildPackages; [
      cpio
      gzip
    ];
  }
  ''
    root=$TMPDIR/initramfs
    mkdir -p $root/{bin,dev,proc,sys,mnt/root}

    cp -a ${busyboxStatic}/bin/* $root/bin/

    cp ${init} $root/init
    chmod +x $root/init

    (cd $root && find . -print0 | cpio --null -oH newc) | gzip -9 > $out
  ''
