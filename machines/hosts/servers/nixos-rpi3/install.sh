sudo disko-install \
    --flake 'github:tghanken/playground/10-17-chore_bootstrap_new_syno_vm#syno-vm' \
    --disk boot /dev/mmcblk0;

sudo zpool export zroot;
