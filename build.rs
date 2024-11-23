use static_files::resource_dir;

fn main() -> std::io::Result<()> {
    let _ = resource_dir("./front/dist").build();
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icons/santa.ico");
        res.compile().unwrap();
    }
    Ok(())
}
