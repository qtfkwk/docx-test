use anyhow::Result;
use docx_rs::*;

fn main() -> Result<()> {
    let docx = Docx::new();
    docx.build()
        .pack(std::fs::File::create("validator/001.docx")?)?;
    Ok(())
}
