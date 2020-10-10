use std::fs;
use std::path::Path;
use structopt::StructOpt;
use trash;

#[derive(StructOpt, Debug)]
#[structopt(name = "node-cleaner")]
struct Cli {
    #[structopt(short, long, default_value = ".")]
    /// Path to run the script
    path: String,

    /// Directory to remove
    #[structopt(short, long, default_value = "node_modules")]
    target: String,

    /// Duration in days
    #[structopt(short, long, default_value = "30")]
    days: f64,

    /// whether or not move files to trash instead of permenant deletion
    #[structopt(short, long)]
    force: bool,
}
fn main() -> std::io::Result<()> {
    let args = Cli::from_args();
    let path = Path::new(&args.path);
    let target = args.target;
    let days = args.days;
    let force = args.force;
    let _ = scan_folder(path, &target, days, force);
    Ok(())
}
fn scan_folder(path: &Path, target: &str, days: f64, force: bool) -> std::io::Result<()> {
    let file_metadata = fs::metadata(path)?;
    if file_metadata.is_dir() == true {
        let files = fs::read_dir(path)?;
        for file in files {
            let data = file?;
            if data.file_name() == target && data.metadata()?.is_dir() {
                let time = path.metadata()?.modified()?;
                let how_old =
                    std::time::SystemTime::now()
                        .duration_since(time)
                        .expect("time travel is currently not possible")
                        .as_secs_f64()
                        / 86400.0 // 1 Day
                ;
                if how_old > days {
                    if force == true {
                        fs::remove_dir_all(data.path()).expect("cannot delete file");
                    } else {
                        trash::remove(data.path()).expect("cannot delete");
                    }
                    println!("deleted {:?}", data.path());
                }
            } else if data.metadata()?.is_dir() {
                let _ = scan_folder(&data.path(), target, days, force);
            }
        }
    }
    Ok(())
}
