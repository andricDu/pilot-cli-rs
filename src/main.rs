use clap::{Parser, Subcommand};
use std::thread;
use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

#[derive(Subcommand)]
enum Commands {
    Upload { path: String },
    Download { target: String },
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Upload { path }) => {
            println!("Uploading {}", path);

            let m = MultiProgress::new();
            let sty = ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .progress_chars("##-");

            let pb = m.add(ProgressBar::new(512));
            pb.set_style(sty.clone());

            let pb2 = m.add(ProgressBar::new(128));
            pb2.set_style(sty.clone());

            let pb3 = m.add(ProgressBar::new(1024));
            pb3.set_style(sty);

            let h1 = thread::spawn(move || {
                for i in 0..512 {
                    pb.set_message(format!("medium_file.tgz #{}", i + 1));
                    pb.inc(1);
                    thread::sleep(Duration::from_millis(15));
                }
                pb.finish_with_message("done");
            });

            let h2 = thread::spawn(move || {
                for i in 0..128 {
                    pb2.set_message(format!("small.txt #{}", i + 1));
                    pb2.inc(1);
                    thread::sleep(Duration::from_millis(15));
                }
                pb2.finish_with_message("done");
            });

            let _ = thread::spawn(move || {
                for i in 0..1024 {
                    pb3.set_message(format!("large_image.dcm #{}", i + 1));
                    pb3.inc(1);
                    thread::sleep(Duration::from_millis(15));
                }
                pb3.finish_with_message("done");
            });

            let _ = m.join();
        }
        Some(Commands::Download { target: _ }) => {
            println!("Starting download!");
        }
        None => {}
    }
}
