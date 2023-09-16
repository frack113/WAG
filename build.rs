// build.rs

use winapi;
use winres;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();

        //res.set_icon("rust.ico");
        res.set("FileVersion", "1.0.0");
        res.set("InternalName", "🔓 your detection");
        res.set("CompanyName", "Frack113");
        res.set(
            "FileDescription",
            "Generate Some Windows Artefact to 👊 your EDR",
        );
        res.set("LegalCopyright", "Frack113");
        res.set("LegalTrademark","Frack113");
        res.set("OriginalFilename", "wag.exe");
        res.set("ProductName", "WindowsArtefactGenerator");
        res.set("ProductVersion", "0.1.0");
        res.set("Comments", "Catch me if you can😁");

        res.set_language(winapi::um::winnt::MAKELANGID(
            winapi::um::winnt::LANG_NEUTRAL,
            winapi::um::winnt::SUBLANG_NEUTRAL,
        ));

        res.set_icon("wag.ico");

        res.compile().unwrap();
    }
}
