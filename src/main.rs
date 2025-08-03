use anyhow::Result;

fn main() -> Result<()> {
    // Blocking scan of “.” or parent .git
    let repo = gix::discover(".")?;
    println!("Found repo at: {:?}", repo.path());
    Ok(())
}
