use handlebars::Handlebars;
use std::{collections::BTreeMap, fs};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Language {
    title: String,
}

pub fn build() {
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
                });
            }
        }
    }

    // languages now has folders, i.e. languages >_<

    // create the output folder
    fs::create_dir_all("www").expect("Could not create output folder 'www'");

    // let mut handlebars = Handlebars::new();
    render_index(languages);
}

fn render_index(languages: Vec<Language>) {
    let mut data: BTreeMap<String, Vec<Language>> = BTreeMap::new();
    data.insert("languages".to_string(), languages);

    // let hbs_index_template = fs::read_to_string("./index.hbs").unwrap();
    let hbs_index_template: String = r#"
<html>
<head<title>Title</title></head>
<body>
Test
<br>
{{#each languages}}
{{this.title}}
{{/each}}
</body>
    "#
    .to_string();

    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("index", hbs_index_template)
        .unwrap();

    println!("{}", handlebars.render("index", &data).unwrap().to_string());
}
