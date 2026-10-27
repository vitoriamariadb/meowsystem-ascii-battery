use std::{env, fs, path::Path};
use xdgen::{App, Context, FluentString};

fn main() {
    let ctx = Context::new("i18n", env::var("CARGO_PKG_NAME").unwrap()).unwrap();
    let app = App::new(FluentString("app-title"))
        .comment(FluentString("app-comment"))
        .keywords(FluentString("app-keywords"));

    let desktop_entry = app.expand_desktop("resources/io.github.meowsystem.AsciiBattery.desktop", &ctx).unwrap();
    let metainfo = app
        .expand_metainfo("resources/io.github.meowsystem.AsciiBattery.metainfo.xml", &ctx)
        .unwrap();

    let output = Path::new("target/xdgen/");
    fs::create_dir_all(output).unwrap();
    fs::write(output.join("io.github.meowsystem.AsciiBattery.desktop"), desktop_entry).unwrap();
    fs::write(output.join("io.github.meowsystem.AsciiBattery.metainfo.xml"), metainfo).unwrap();
}
