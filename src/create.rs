use std::path::Path;

use tera::Context;

use crate::template::Template;


fn create_project_files(
	template_path: &Path,
	template: &Template,
	project_name: &str,
	output_root: &Path  
) -> anyhow::Result<()> {
	let slug = slug::slugify(project_name);
	let date = chrono::Local::now().format("%Y-%m-%d").to_string();

	let mut ctx = Context::new();
	ctx.insert("project", project_name);
	ctx.insert("slug", &slug);
	ctx.insert("title", project_name);
	ctx.insert("date", &date);

	let base_output_dir = output_root.join("WTD").join(project_name);
	// 1. Wantの生成
    let want_template_path = template_path.join(&template.want.template);
    let want_content = Template::render_template(&want_template_path, &ctx)?;
    let want_output_path = base_output_dir.join("Want").join(template.want.filename.replace("{{slug}}", &slug));
    std::fs::create_dir_all(want_output_path.parent().unwrap())?;
    std::fs::write(&want_output_path, want_content)?;

    // 2. Develop-Docsの生成
    for doc in &template.docs {
        let doc_template_path = template_path.join(&doc.template);
        let doc_content = Template::render_template(&doc_template_path, &ctx)?;
        let doc_output_path = base_output_dir.join("Develop-Docs").join(&doc.filename);
        std::fs::create_dir_all(doc_output_path.parent().unwrap())?;
        std::fs::write(&doc_output_path, doc_content)?;
    }

    println!("✅ プロジェクト '{}' を生成しました。", project_name);
    println!("📁 {}", base_output_dir.display());
    Ok(())
}