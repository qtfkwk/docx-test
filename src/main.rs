use anyhow::Result;
use docx_rs::*;

fn main() -> Result<()> {
    let mut docx = Docx::new();

    docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")));

    docx = docx.add_style(
        Style::new("Heading 1", StyleType::Character)
            .name("Heading 1")
            .next("Body Text")
            .size(18 * 2)
            .bold(),
    );
    docx = docx.add_paragraph(
        Paragraph::new()
            .add_run(Run::new().add_text("Heading 1"))
            .style("Heading 1"),
    );

    docx.build()
        .pack(std::fs::File::create("validator/003.docx")?)?;

    Ok(())
}
