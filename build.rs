use static_files::resource_dir;

fn main() -> std::io::Result<()> {
    resource_dir("./front/dist").build()
}
