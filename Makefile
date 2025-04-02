build:
	cargo build -Zbuild-std -Zbuild-std-features=compiler-builtins-mem --target x86_64-unknown-uefi

	rm -rf ./build
	mkdir ./build
	mkdir -p ./build/EFI/BOOT
	
	cp ./target/x86_64-unknown-uefi/debug/bootloader.efi ./build/EFI/BOOT/BOOTX64.EFI

	cd ./build && \
		dd if=/dev/zero of=uefi.img bs=1M count=64 && \
		parted -s uefi.img mklabel gpt && \
		parted -s uefi.img mkpart primary fat32 1MiB 100% && \
		parted -s uefi.img set 1 esp on && \
		mkfs.vfat -F 32 uefi.img && \
        mmd -i uefi.img ::/EFI && \
        mmd -i uefi.img ::/EFI/BOOT && \
        mcopy -i uefi.img EFI/BOOT/BOOTX64.EFI ::/EFI/BOOT/ && \
        chmod 666 ./uefi.img

.PHONY: build