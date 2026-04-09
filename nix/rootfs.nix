{
  buildPackages,
  busybox,
  dropbear,
  callPackage,
  signtool,
  signingKeys,
}:
let
  kernel = callPackage ./kernel.nix { };
  initramfs = callPackage ./initramfs.nix { };
  rootPkgs = [
    busybox
    dropbear
  ];
  closure = buildPackages.closureInfo { rootPaths = rootPkgs; };
in
buildPackages.runCommand "rootfs.img"
  {
    nativeBuildInputs = with buildPackages; [
      e2fsprogs
      gptfdisk
      util-linux
    ];
  }
  ''
    mkdir -p $out/images

    root=$TMPDIR/rootfs
    mkdir -p $root/{bin,sbin,etc,proc,sys,dev,tmp,nix/store,run,var/log,root,boot}

    while read storePath; do
      cp -a "$storePath" "$root/nix/store/"
    done < ${closure}/store-paths

    cp -a ${busybox}/bin/* $root/bin/
    cp -a ${busybox}/sbin/* $root/sbin/ 2>/dev/null || true
    ln -sf ../bin/busybox $root/sbin/init

    cp -r ${kernel}/* $root/boot/
    cp ${initramfs} $root/boot/initrd.img
    cp -r ${kernel}/* $out/images/
    cp ${initramfs} $out/images/initrd.img

    # Sign kernel/initrd
    ${signtool}/bin/secbimg create \
      -i ${kernel}/Image \
      -a sha3 -s ed25519 -k ${signingKeys}/keys/ed25519_nix.key \
      -o $out/images/Image-signed 
    ${signtool}/bin/secbimg create \
      -i ${initramfs} \
      -a sha3 -s ed25519 -k ${signingKeys}/keys/ed25519_nix.key \
      -o $out/images/initrd.img-signed 
    cp $out/images/Image-signed $root/boot/
    cp $out/images/initrd.img-signed $root/boot/

    cat > $root/etc/passwd <<'EOF'
    root:x:0:0:root:/root:/bin/sh
    nobody:x:65534:65534:nobody:/nonexistent:/bin/sh
    EOF

    cat > $root/etc/group <<'EOF'
    root:x:0:
    nobody:x:65534:
    EOF

    cat > $root/etc/shadow <<'EOF'
    root::0:0:99999:7:::
    nobody:!:0:0:99999:7:::
    EOF
    chmod 640 $root/etc/shadow

    echo "secureboot" > $root/etc/hostname

    cat > $root/etc/resolv.conf <<'EOF'
    nameserver 10.0.2.3
    EOF

    mkdir -p $root/usr/share/udhcpc
    cat > $root/usr/share/udhcpc/default.script <<'EOF'
    #!/bin/sh
    case "$1" in
      bound|renew)
        ip addr flush dev "$interface"
        ip addr add "$ip/$mask" dev "$interface"
        [ -n "$router" ] && ip route add default via "$router"
        [ -n "$dns" ] && {
          : > /etc/resolv.conf
          for d in $dns; do echo "nameserver $d" >> /etc/resolv.conf; done
        }
        ;;
      deconfig)
        ip addr flush dev "$interface"
        ;;
    esac
    EOF
    chmod +x $root/usr/share/udhcpc/default.script

    cat > $root/etc/inittab <<'EOF'
    ::sysinit:/bin/mount -t proc proc /proc
    ::sysinit:/bin/mount -t sysfs sysfs /sys
    ::sysinit:/bin/mount -t devtmpfs devtmpfs /dev
    ::sysinit:/bin/ip link set lo up
    ::sysinit:/bin/ip link set eth0 up
    ::sysinit:/bin/udhcpc -i eth0 -s /usr/share/udhcpc/default.script -q
    console::respawn:/bin/getty -L console 115200 vt100
    EOF

    root_bytes=$(du -sb $root | awk '{print $1}')
    root_kb=$(( (root_bytes * 3 / 2) / 1024 ))
    [ $root_kb -lt 65536 ] && root_kb=65536

    total_kb=$(( 2048 + root_kb + 64 ))
    truncate -s ''${total_kb}K $out/images/rootfs.img

    sgdisk --clear \
      --new=1:2048:0 \
      --typecode=1:8300 \
      --change-name=1:rootfs \
      $out/images/rootfs.img

    part_size=$(( root_kb * 1024 ))
    truncate -s $part_size $TMPDIR/rootfs.img
    mkfs.ext4 -d $root -L rootfs $TMPDIR/rootfs.img
    dd if=$TMPDIR/rootfs.img of=$out/images/rootfs.img bs=512 seek=2048 conv=notrunc
  ''
