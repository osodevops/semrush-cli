use clap::CommandFactory;
use clap_mangen::Man;
use semrush::cli::Cli;

fn main() -> std::io::Result<()> {
    let out_dir = std::env::args().nth(1).unwrap_or_else(|| "man".into());
    std::fs::create_dir_all(&out_dir)?;
    let cmd = Cli::command();
    let man = Man::new(cmd);
    let mut buf = Vec::new();
    man.render(&mut buf)?;
    std::fs::write(format!("{out_dir}/semrush.1"), buf)?;
    Ok(())
}
