fn main() {
    // Only on Windows
    #[cfg(windows)]
    {
        // Embed icon and version info
        let mut res = winres::WindowsResource::new();
        res.set("ProductName", "Kaku Terminal");
        res.set("FileDescription", "Fast terminal for AI coding");
        res.set("LegalCopyright", "Copyright © 2026 lkds");
        res.set("OriginalFilename", "kaku-gui.exe");
        res.set_icon("assets/kaku.ico");
        res.compile().unwrap();
    }
}