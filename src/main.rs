use gtk::glib;
use std::{fs, io, path::Path};

mod app;
mod cli;
mod config;
mod env;

use cli::{Cli, Commands, Parser};


fn main() -> io::Result<glib::ExitCode> {
    let cli = Cli::parse();

    match cli.command.clone() {
        Commands::InitConfig => {
            write_config_defaults(&cli)?;
            Ok(glib::ExitCode::SUCCESS)
        },
        Commands::Launcher => {
            let ctx = app::AppContext::with_app_list(cli)?;
            let app = app::App::new(ctx);
            Ok(app.start())
        },
        Commands::Menu { file } => {
            let ctx = app::AppContext::with_menu_list_file(cli, &file)?;
            let app = app::App::new(ctx);
            Ok(app.start())
        }
    }
}

fn write_config_defaults(cli: &Cli) -> io::Result<()> {
    let config_path = cli.get_config_path();
    write_file_if_not_exists(&config_path, include_bytes!("../assets/config.jsonc"))?;
    println!("Created {}", config_path.to_string_lossy());

    let style_path = cli.get_style_path();
    write_file_if_not_exists(&style_path, include_bytes!("../assets/style.css"))?;
    println!("Created {}", style_path.to_string_lossy());

    Ok(())
}

fn write_file_if_not_exists(file_path: &Path, content: &[u8]) -> io::Result<()> {
    if file_path.exists() {
        eprintln!("{} already exists, refusing to overwrite", file_path.to_string_lossy());
    } else {
        fs::create_dir_all(file_path.parent().unwrap())?;
        fs::write(file_path, content)?;
    }

    Ok(())
}
