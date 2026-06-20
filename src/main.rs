//! Rust bootstrap of the kernel.
use colored::Colorize;
use isobemak::{
    BiosBootInfo, BootInfo, IsoImage, IsoImageFile, IsoLayoutProfile, UefiBootInfo, build_iso,
};
use log::{error, info};
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::exit;
use walkdir::WalkDir;

/// The directories which needs to iterate
const DIRS: [&'static str; 2] = ["bootloader", "kernel"];

fn main() -> std::io::Result<()> {
    // Init logger
    env_logger::init();

    // Decide the build profile
    let key = "PROFILE";
    unsafe {
        if let Some(arg) = env::args().nth(1) {
            if arg == "release" {
                env::set_var(key, "release");
                info!(
                    "Will use {} mode to build projects!",
                    "release".cyan().bold()
                )
            } else {
                info!("Will use {} mode to build projects!", "debug".cyan().bold())
            }
        } else {
            info!("Will use {} mode to build projects!", "debug".cyan().bold())
        }
    }

    // Iterate each directories and build
    for dir in DIRS {
        info!("Entering directory \"{}\"", dir.green().bold());
        let path = Path::new(dir);
        if !path.exists() {
            error!("This path does not exist!");
            exit(1)
        }

        let result = Command::new("make")
            .arg("-C")
            .arg(dir)
            .status()
            .expect("Failed to execute command");

        if !result.success() {
            error!("This path does not exist!");
            exit(result.code().expect("Failed to get code"))
        }

        info!("Build complete");
    }

    info!("Start to arrange ISO structure...");
    arrange_iso()?;
    info!("ISO arrangement completed.");

    info!("Preparing to pack ISO...");
    let path = pack_iso()?;
    info!("Successfully packes an ISO file at {}.", path.display());

    Ok(())
}

fn arrange_iso() -> std::io::Result<()> {
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

    let src_bootsec = Path::new("./bootloader/output/BOOTMBR");
    let dst_bootsec = iso.join("bootsec.bin");
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

fn pack_iso() -> std::io::Result<PathBuf> {
    let iso = Path::new("iso");
    let bootsec_bin_path = PathBuf::from("iso/bootsec.bin");
    let kernel_path = PathBuf::from("iso/proka-kernel");
    let bootx64_efi_path = PathBuf::from("iso/EFI/Boot/bootx64.efi");
    let iso_output_path = PathBuf::from("proka.iso");

    let mut iso_image = IsoImage {
        volume_id: Some("ProkaOS".to_string()),
        files: vec![],
        boot_info: BootInfo {
            bios_boot: Some(BiosBootInfo {
                boot_image: bootsec_bin_path.clone(),
                destination_in_iso: "bootsec.bin".to_string(),
            }),
            uefi_boot: Some(UefiBootInfo {
                boot_image: bootx64_efi_path.clone(),
                kernel_image: kernel_path.clone(),
                destination_in_iso: "EFI/Boot/bootx64.efi".to_string(),
                additional_efi_boot_files: Vec::new(),
                grub_cfg_content: None,
            }),
        },
        layout_profile: IsoLayoutProfile::default(),
    };

    let iso_root = WalkDir::new(&iso)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file());
    for entry in iso_root {
        let path = entry.path().to_path_buf();
        let name = match path.strip_prefix("iso/") {
            Ok(rel_path) => rel_path.to_path_buf(),
            Err(_) => path.clone(),
        };
        iso_image.files.push(IsoImageFile {
            source: path,
            destination: name.display().to_string(),
        });
    }

    // Create a hybrid isohybrid ISO
    let (iso_path, _temp_fat, _iso_file, _fat_size) =
        build_iso(&iso_output_path, &iso_image, true)?;

    Ok(iso_path)
}
