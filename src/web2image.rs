use anyhow::Result;
use headless_chrome::{protocol::cdp::Page::CaptureScreenshotFormatOption, Browser, LaunchOptions};
use image::{imageops::overlay, load_from_memory, DynamicImage, ImageFormat, Luma};
use qrcode::QrCode;
use std::{thread, time::Instant};

fn url2image(url: &str) -> Result<DynamicImage> {
    let start = Instant::now();
    // 创建浏览器实例
    let options = LaunchOptions::default_builder()
        .window_size(Some((1200, 1600)))
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;
    let png_data: Vec<u8> = tab
        .navigate_to(url)?
        .wait_until_navigated()?
        .capture_screenshot(CaptureScreenshotFormatOption::Png, None, None, true)?;
    let result = load_from_memory(&png_data)?;
    println!("url2image took {:.2?}", start.elapsed());
    Ok(result)
}

fn gen_qrcode(url: &str) -> Result<DynamicImage> {
    let start = Instant::now();
    let code = QrCode::new(url.as_bytes())?;

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    println!("gen_qrcode took {:.2?}", start.elapsed());
    Ok(DynamicImage::ImageLuma8(image))
}

fn do_overlay(bottom: &mut DynamicImage, top: &DynamicImage) {
    let start = Instant::now();
    let x = bottom.width() - top.width() - 10;
    let y = bottom.height() - top.height() - 10;
    overlay(bottom, top, x.into(), y.into());
    println!("do_overlay took {:.2?}", start.elapsed());
}

pub fn web2image(url: &str, output: &str, format: ImageFormat) -> Result<()> {
    let url = url.to_owned();
    let url1 = url.clone();
    let bottom_handle = thread::spawn(move || url2image(&url).unwrap());
    let qrcode_handle = thread::spawn(move || gen_qrcode(&url1).unwrap());
    let mut bottom = bottom_handle.join().unwrap();
    let qrcode = qrcode_handle.join().unwrap();
    do_overlay(&mut bottom, &qrcode);

    let start = Instant::now();
    bottom.save_with_format(output, format)?;
    println!("web2image took {:.2?}", start.elapsed());
    Ok(())
}
