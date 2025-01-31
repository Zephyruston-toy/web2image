use clap::Parser;
use image::ImageFormat;
use std::{ffi::OsStr, path::Path};
use url::Url;

mod web2image;

/// website to image
#[derive(Parser, Debug)]
#[command(
    version,
    about,
    author = "zccccc01 <1351688749@qq.com>",
    help_template = "{before-help}{name} {version}\nAuthor: {author}\n{about-with-newline}\n{usage-heading} {usage}\n\n{all-args}{after-help}"
)]
struct Args {
    /// output file
    #[arg(short, long, default_value = "./shot.jpg", value_parser= valid_filename)]
    output: String,

    /// url to capture
    #[arg(short, long, value_parser= valid_url)]
    url: String,
}

fn get_image_format(path: &Path) -> Option<ImageFormat> {
    path.extension()
        .and_then(|p| OsStr::to_str(p))
        .and_then(|ext| {
            let ext = ext.to_lowercase();
            match ext.as_str() {
                "jpg" => Some(ImageFormat::Jpeg),
                "png" => Some(ImageFormat::Png),
                _ => None,
            }
        })
}

/// "/tmp/abc.pdf" => "/tmp" exists, pdf (png | jpg)
fn valid_filename(name: &str) -> Result<String, String> {
    let path = Path::new(name);
    let parent = path.parent().and_then(|p| p.is_dir().then_some(p));
    let ext = get_image_format(path);
    if parent.is_none() || ext.is_none() {
        return Err("invalid path and file must be jpg, png".into());
    }
    Ok(name.into())
}

/// make sure url is valid
fn valid_url(url: &str) -> Result<String, String> {
    match Url::parse(url) {
        Ok(_) => Ok(url.into()),
        Err(_) => Err("You must provide a valid url".into()),
    }
}
fn main() {
    let args = Args::parse();
    println!("{:#?}", args);
    let format = get_image_format(Path::new(&args.output)).unwrap();
    web2image::web2image(&args.url, &args.output, format).unwrap();
}
