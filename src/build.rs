use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use std::{collections::BTreeMap, fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Language {
    title: String,
    img_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Picture {
    title: String,
    img_small_url: String,
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
            if path.is_dir() && !path.clone().to_str().unwrap().contains(".git") {
                languages.push(Language {
                    // don't have pound symbol in urls
                    title: path.file_name().unwrap().to_str().unwrap().to_string(),
                    img_url: {
                        let mut folder_path = PathBuf::new();
                        folder_path.push(".");
                        folder_path.push(path.clone());
                        let i = fs::read_dir(path).unwrap();
                        let mut r: String = String::new();
                        for img in i {
                            if let Ok(img) = img {
                                if img.path().is_file() {
                                    r = img
                                        .path()
                                        .file_name()
                                        .unwrap()
                                        .to_str()
                                        .unwrap()
                                        .to_string();
                                    break;
                                }
                            }
                        }
                        gen_small_img_url(
                            &"base".to_string(),
                            &entry
                                .path()
                                .file_name()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_string(),
                            &r,
                        )
                    },
                });
            }
        }
    }

    // languages now has folders, i.e. languages >_<

    // create the output folder
    fs::create_dir_all("www").expect("Could not create output folder 'www'");

    // a handlebars instance to be passed around
    let mut handlebars = Handlebars::new();

    // build template paths from folder name
    let index_template_path = {
        let mut p = template_path.clone();
        p.push("index.hbs".to_string());
        p
    };
    let gallery_template_path = {
        let mut p = template_path.clone();
        p.push("gallery.hbs".to_string());
        p
    };
    let base_template_path = {
        let mut p = template_path.clone();
        p.push("base.hbs".to_string());
        p
    };

    // register base template
    let hbs_base_template = fs::read_to_string(base_template_path).unwrap();

    handlebars
        .register_template_string("base", hbs_base_template)
        .unwrap();

    // render gallery for each language
    render_gallery(gallery_template_path, &languages, &handlebars);
    // render the index page
    render_index(index_template_path, &languages, &handlebars);
}

fn render_gallery(
    gallery_template_path: PathBuf,
    languages: &Vec<Language>,
    handlebars_g: &Handlebars,
) {
    let mut data: BTreeMap<String, Json> = BTreeMap::new();

    // data.insert("language".to_string(), language);
    data.insert("languages".to_string(), handlebars::to_json(&languages));

    let handlebars = {
        let mut h = handlebars_g.clone();
        let gallery_template = fs::read_to_string(&gallery_template_path).unwrap();
        h.register_template_string("gallery", gallery_template)
            .unwrap();
        h
    };

    let mut output_path = PathBuf::new();
    output_path.push(".");
    output_path.push("www");

    let img_url_base: String =
        "https://raw.githubusercontent.com/cat-milk/Anime-Girls-Holding-Programming-Books/master/"
            .to_string();

    for language in languages {
        let mut pictures: Vec<Picture> = vec![];

        println!("language : {}", language.title);
        let entries = fs::read_dir(format!("{}/{}", ".", language.title.clone()))
            .expect("Can't read current directory");

        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    pictures.push(Picture {
                        title: path.to_str().unwrap().to_string(),
                        img_url: gen_img_url(
                            &img_url_base,
                            &language.title,
                            &path
                                .clone()
                                .file_name()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_string(),
                        ),
                        img_small_url: gen_small_img_url(
                            &img_url_base,
                            &language.title,
                            &path
                                .clone()
                                .file_name()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_string(),
                        ),
                    });
                }
            }
        }

        data.insert(
            "language".to_string(),
            handlebars::to_json(language.title.clone()),
        );
        data.insert("pictures".to_string(), handlebars::to_json(&pictures));
        output_path.push(&language.title);
        output_path.set_extension("html");

        fs::write(
            &output_path,
            handlebars.render("gallery", &data).unwrap().to_string(),
        )
        .unwrap();
        output_path.pop();
    }
}

fn render_index(
    index_template_path: PathBuf,
    languages: &Vec<Language>,
    handlebars_g: &Handlebars,
) {
    let mut data: BTreeMap<String, Vec<Language>> = BTreeMap::new();
    data.insert("languages".to_string(), languages.clone().to_vec());

    let hbs_index_template = fs::read_to_string(&index_template_path).unwrap();

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

fn gen_img_url(base: &String, language: &String, img_name: &String) -> String {
    format!("{}{}/{}", base, language, img_name)
}

fn gen_small_img_url(base: &String, language: &String, img_name: &String) -> String {
    println!("file name {}", &img_name);
    format!("{}{}/s{}", base, language, img_name)
}
