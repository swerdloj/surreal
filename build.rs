// https://doc.rust-lang.org/cargo/reference/build-scripts.html

use std::path::Path;
use std::fs;

fn main() {
    // Locate SDL2 for Windows development
    #[cfg(target_os = "windows")] 
    {
        // Link SDL2 library
        let sdl2_path = r"C:/Development/SDL2/lib/x64";
        println!(r"cargo:rustc-link-search={}", sdl2_path);
        
        // Copy over to project path (required to run the executable)
        let sdl2_runtime_dll = Path::new("./SDL2.dll");
        if !sdl2_runtime_dll.exists() {
            fs::copy(format!("{}/{}", sdl2_path, "SDL2.dll"), sdl2_runtime_dll).unwrap();
        }
    }  

    // TODO: Find .dll for runtime
    //  The above works when building, but the .exe itself cannot locate the .dll

    // TODO: In release configuration, generated a distributable folder
    //  containing the executable and shaders/resources
}