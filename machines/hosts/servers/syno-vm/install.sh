sudo disko-install \
    --write-efi-boot-entries \
    --flake 'github:tghanken/playground#syno-vm' \
    --disk boot /dev/sda;
