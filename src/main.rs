use anyhow::Result;
use docx_rs::*;

fn main() -> Result<()> {
    let mut docx = Docx::new();

    docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")));

    docx.build()
        .pack(std::fs::File::create("validator/002.docx")?)?;

    Ok(())
}
