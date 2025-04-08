fn main() {
    // This is the build script for the Slint UI framework.
    slint_build::compile("ui/app.slint").expect("Slint build failed");
    
}