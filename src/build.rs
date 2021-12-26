use handlebars::Handlebars;
use image::{self, GenericImageView};
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

pub fn build(
    template_path: PathBuf,
    img_width: u32,
    img_url_base: String,
    img_small_base_url: String,
    output_folder: String,
) {
    // get list of files in working directory
    // and filter out only folders
    let entries = fs::read_dir(".").expect("Can't read current directory");

    let mut languages: Vec<Language> = vec![];

    for entry in entries.flatten() {
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
                    for img in i.flatten() {
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
                    gen_small_img_url(
                        &img_small_base_url,
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
    // languages now has folders, i.e. languages >_<

    // create the output folder
    fs::create_dir_all(&output_folder).expect("Could not create output folder");

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
        let mut p = template_path;
        p.push("base.hbs".to_string());
        p
    };

    // register base template
    let hbs_base_template = fs::read_to_string(base_template_path).unwrap();

    handlebars
        .register_template_string("base", hbs_base_template)
        .unwrap();

    // render gallery for each language
    render_gallery(
        gallery_template_path,
        &languages,
        &handlebars,
        &output_folder,
        &img_url_base,
        img_width,
    );
    // render the index page
    render_index(index_template_path, &languages, &handlebars, &output_folder);
}

fn render_gallery(
    gallery_template_path: PathBuf,
    languages: &[Language],
    handlebars_g: &Handlebars,
    output_folder: &str,
    img_url_base: &str,
    img_width: u32,
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

    for language in languages {
        let mut lang_path = PathBuf::new();
        lang_path.push(".");
        lang_path.push(&output_folder);
        lang_path.push(&language.title);
        fs::create_dir_all(lang_path).unwrap();

        let mut pictures: Vec<Picture> = vec![];

        println!("language : {}", language.title);
        let entries = fs::read_dir(format!("{}/{}", ".", language.title.clone()))
            .expect("Can't read current directory");

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                println!("{:?}", path);
                let img = image::open(&path);
                if img.is_err() {
                    continue;
                }
                let img = img.unwrap();
                let img = img.resize(
                    img_width,
                    img.height() * img_width / img.width(),
                    image::imageops::Gaussian,
                );
                println!(
                    "saving {}",
                    gen_small_img_url(
                        output_folder,
                        &language.title,
                        &path
                            .clone()
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string(),
                    )
                );
                let img_save_res = img.save(gen_small_img_url(
                    output_folder,
                    &language.title,
                    &path
                        .clone()
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                ));

                if img_save_res.is_err() {
                    continue;
                }

                pictures.push(Picture {
                    title: path.file_name().unwrap().to_str().unwrap().to_string(),
                    img_url: gen_img_url(
                        img_url_base,
                        &language.title,
                        &path
                            .clone()
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string(),
                    ),
                    img_small_url: format!(
                        "{}/s{}",
                        &language.title,
                        &path
                            .clone()
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string()
                    ),
                });
            }
        }

        data.insert(
            "language".to_string(),
            handlebars::to_json(language.title.clone()),
        );
        data.insert("pictures".to_string(), handlebars::to_json(&pictures));
        let mut output_file = PathBuf::from(&output_folder);
        output_file.push(&language.title);
        output_file.set_extension("html");

        fs::write(
            &output_file,
            handlebars.render("gallery", &data).unwrap().to_string(),
        )
        .unwrap();
        output_file.pop();
    }
}

fn render_index(
    index_template_path: PathBuf,
    languages: &[Language],
    handlebars_g: &Handlebars,
    output_folder: &str,
) {
    let mut data: BTreeMap<String, Vec<Language>> = BTreeMap::new();
    data.insert("languages".to_string(), languages.to_vec());

    let hbs_index_template = fs::read_to_string(&index_template_path).unwrap();

    let mut handlebars = handlebars_g.clone();

    handlebars
        .register_template_string("index", hbs_index_template)
        .unwrap();

    fs::write(
        {
            let mut p = PathBuf::new();
            p.push(".");
            p.push(output_folder);
            p.push("index");
            p.set_extension("html");
            p
        },
        handlebars.render("index", &data).unwrap(),
    )
    .unwrap();
}

fn gen_img_url(base: &str, language: &str, img_name: &str) -> String {
    format!("{}/{}/{}", base, language, img_name)
}

fn gen_small_img_url(base: &str, language: &str, img_name: &str) -> String {
    format!("{}/{}/s{}", base, language, img_name)
}
