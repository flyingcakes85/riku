use handlebars::Handlebars;
use std::{collections::BTreeMap, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Language {
    title: String,
    img_url: String,
}

pub fn build(template_path: PathBuf) {
    // get list of files in working directory
    // and filter out only folders
    let entries = fs::read_dir(".").expect("Can't read current directory");

    let mut languages: Vec<Language> = vec![];

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                languages.push(Language {
                    title: path.to_str().unwrap().to_string(),
                    img_url: String::from("path/to/image"),
                });
            }
        }
    }

    // languages now has folders, i.e. languages >_<

    // create the output folder
    fs::create_dir_all("www").expect("Could not create output folder 'www'");

    let mut handlebars = Handlebars::new();
    let index_template_path = {
        let mut p = template_path.clone();
        p.push("index.hbs".to_string());
        p
    };
    let base_template_path = {
        let mut p = template_path.clone();
        p.push("base.hbs".to_string());
        p
    };

    let hbs_base_template = fs::read_to_string(base_template_path).unwrap();

    handlebars
        .register_template_string("base", hbs_base_template)
        .unwrap();

    // let mut handlebars = Handlebars::new();
    render_index(index_template_path, languages, &handlebars);
}

fn render_index(index_template_path: PathBuf, languages: Vec<Language>, handlebars_g: &Handlebars) {
    let mut data: BTreeMap<String, Vec<Language>> = BTreeMap::new();
    data.insert("languages".to_string(), languages);

    let hbs_index_template = fs::read_to_string(&index_template_path).unwrap();

    println!(
        "start read string \n{}\nend read string",
        hbs_index_template
    );

    let mut handlebars = handlebars_g.clone();

    handlebars
        .register_template_string("index", hbs_index_template)
        .unwrap();

    fs::write(
        {
            let mut p = PathBuf::new();
            p.push(".");
            p.push("www");
            p.push("index");
            p.set_extension("html");
            p
        },
        handlebars.render("index", &data).unwrap().to_string(),
    )
    .unwrap();
}
