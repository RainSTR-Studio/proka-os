//! The project builder.
mod iso;
use std::path::Path;
use std::fs;
use log::info;
pub use iso::*;

pub fn arrange_iso() -> std::io::Result<()> {
    // ISO root path
    let iso = Path::new("iso");
    let efi_boot = iso.join("EFI").join("Boot");

    // Create
    fs::create_dir_all(&efi_boot)?;
    info!("Created ISO directory tree at \"{}\"", iso.display());

    // Copy UEFI
    let src_uefi = Path::new("./bootloader/output/pkbl-uefi.efi");
    let dst_efi = efi_boot.join("bootx64.efi");
    fs::copy(src_uefi, &dst_efi)?;
    info!("Copied UEFI bootloader -> {}", dst_efi.display());

    // Copy PKLDR (BIOS)
    let src_pkldr = Path::new("./bootloader/output/pkldr");
    let dst_pkldr = iso.join("pkldr");
    fs::copy(src_pkldr, &dst_pkldr)?;
    info!("Copied legacy bootloader -> {}", dst_pkldr.display());

    let src_bootsec = Path::new("./bootloader/output/cdboot.bin");
    let dst_bootsec = iso.join("cdboot.bin");
    fs::copy(src_bootsec, &dst_bootsec)?;
    info!("Copied legacy boot sector -> {}", dst_pkldr.display());

    // Copy kernel
    let src_kernel = Path::new("./kernel/output/proka-kernel");
    let dst_kernel = iso.join("proka-kernel");
    fs::copy(src_kernel, &dst_kernel)?;
    info!("Copied kernel binary -> {}", dst_kernel.display());

    // TODO: Copy initprt (because no tools to gen)

    Ok(())
}
