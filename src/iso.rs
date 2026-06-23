//! The ISO packer.
use std::io;
use std::path::PathBuf;
use std::path::Path;
use isobemak::{
    BiosBootInfo, BootInfo, IsoImage, IsoImageFile, IsoLayoutProfile, UefiBootInfo, build_iso,
};
use walkdir::WalkDir;

pub fn pack_iso() -> io::Result<PathBuf> {
    let iso = Path::new("iso");
    let bootsec_bin_path = PathBuf::from("iso/cdboot.bin");
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
