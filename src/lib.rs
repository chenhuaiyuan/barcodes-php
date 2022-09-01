use phper::{
    classes::{StatefulClass, Visibility},
    functions::Argument,
    modules::{Module, ModuleContext},
    php_get_module,
};

macro_rules! bar_coder {
    ($({symbologies: $sym:ident, struct_name: $name:ident}),+) => {
            use  barcoders::{sym::{
                codabar::Codabar, code11::Code11, code128::Code128, code39::Code39, code93::Code93, ean13::EAN13, ean8::EAN8, ean_supp::EANSUPP,
            },error::Result, generators::image::*};
            use std::io::BufWriter;
            use std::fs::File;
            use std::path::Path;
            use std::io::Write;
            $(
                #[derive(Clone)]
                pub struct $name {
                    height: u32,
                    xdim: u32,
                    rotation: Rotation,
                    foreground: [u8; 4],
                    background: [u8; 4],
                }

                impl Default for $name {
                    fn default() -> Self {
                        $name {
                            height: 80,
                            xdim: 1,
                            rotation: Rotation::Zero,
                            foreground: [0, 0, 0, 255],
                            background: [255, 255, 255, 255]
                        }
                    }
                }

                impl $name {
                    pub fn new() -> Self {
                        $name {
                            height: 80,
                            xdim: 1,
                            rotation: Rotation::Zero,
                            foreground: [0, 0, 0, 255],
                            background: [255, 255, 255, 255]
                        }
                    }

                    pub fn set_foreground(&mut self, rgba: [u8; 4]) {
                        self.foreground = rgba;
                    }

                    pub fn set_background(&mut self, rgba: [u8; 4]) {
                        self.background = rgba;
                    }

                    pub fn set_height(&mut self, height: u32) {
                        self.height = height;
                    }

                    pub fn set_xdim(&mut self, xdim: u32) {
                        self.xdim = xdim;
                    }

                    pub fn generate_png<T: AsRef<str>>(&self, data: T, file_name: T) -> Result<()> {
                        let code = $sym::new(data)?;
                        let encode = code.encode();
                        let png = Image::PNG {
                            height: self.height,
                            xdim: self.xdim,
                            rotation: self.rotation,
                            foreground: Color::new(self.foreground),
                            background: Color::new(self.background)
                        };
                        let bytes = png.generate(&encode[..])?;
                        let file = File::create(&Path::new(file_name.as_ref())).unwrap();
                        let mut writer = BufWriter::new(file);
                        writer.write_all(&bytes[..]).unwrap();
                        Ok(())
                    }

                    pub fn generate_gif<T: AsRef<str>>(&self, data: T, file_name: T) -> Result<()> {
                        let code = $sym::new(data)?;
                        let encode = code.encode();
                        let png = Image::GIF {
                            height: self.height,
                            xdim: self.xdim,
                            rotation: self.rotation,
                            foreground: Color::new(self.foreground),
                            background: Color::new(self.background)
                        };
                        let bytes = png.generate(&encode[..])?;
                        let file = File::create(&Path::new(file_name.as_ref())).unwrap();
                        let mut writer = BufWriter::new(file);
                        writer.write_all(&bytes[..]).unwrap();
                        Ok(())
                    }

                    pub fn generate_jpeg<T: AsRef<str>>(&self, data: T, file_name: T) -> Result<()> {
                        let code = $sym::new(data)?;
                        let encode = code.encode();
                        let png = Image::JPEG {
                            height: self.height,
                            xdim: self.xdim,
                            rotation: self.rotation,
                            foreground: Color::new(self.foreground),
                            background: Color::new(self.background)
                        };
                        let bytes = png.generate(&encode[..])?;
                        let file = File::create(&Path::new(file_name.as_ref())).unwrap();
                        let mut writer = BufWriter::new(file);
                        writer.write_all(&bytes[..]).unwrap();
                        Ok(())
                    }
                }
            )+
    };
}

bar_coder! {
    {symbologies: Codabar, struct_name: PHPCodabar},
    {symbologies: Code11, struct_name: PHPCode11},
    {symbologies: Code128, struct_name: PHPCode128},
    {symbologies: Code39, struct_name: PHPCode39},
    {symbologies: Code93, struct_name: PHPCode93},
    {symbologies: EAN13, struct_name: PHPEAN13},
    {symbologies: EAN8, struct_name: PHPEAN8},
    {symbologies: EANSUPP, struct_name: PHPEANSUPP}
}

pub struct PHPTF {
    height: u32,
    xdim: u32,
    rotation: Rotation,
    foreground: [u8; 4],
    background: [u8; 4],
}

impl Default for PHPTF {
    fn default() -> Self {
        PHPTF {
            height: 80,
            xdim: 1,
            rotation: Rotation::Zero,
            foreground: [0, 0, 0, 255],
            background: [255, 255, 255, 255],
        }
    }
}

impl PHPTF {
    pub fn new() -> PHPTF {
        PHPTF {
            height: 80,
            xdim: 1,
            rotation: Rotation::Zero,
            foreground: [0, 0, 0, 255],
            background: [255, 255, 255, 255],
        }
    }

    pub fn set_foreground(&mut self, rgba: [u8; 4]) {
        self.foreground = rgba;
    }

    pub fn set_background(&mut self, rgba: [u8; 4]) {
        self.background = rgba;
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn set_xdim(&mut self, xdim: u32) {
        self.xdim = xdim;
    }

    pub fn generate_png<T: AsRef<str>>(&self, data: T, file_name: T) -> Result<()> {
        let code = barcoders::sym::tf::TF::standard(data)?;
        let encode = code.encode();
        let png = Image::PNG {
            height: self.height,
            xdim: self.xdim,
            rotation: self.rotation,
            foreground: Color::new(self.foreground),
            background: Color::new(self.background),
        };
        let bytes = png.generate(&encode[..])?;
        let file = File::create(&Path::new(file_name.as_ref())).unwrap();
        let mut writer = BufWriter::new(file);
        writer.write_all(&bytes[..]).unwrap();
        Ok(())
    }

    pub fn generate_gif<T: AsRef<str>>(&self, data: T, file_name: T) -> Result<()> {
        let code = barcoders::sym::tf::TF::standard(data)?;
        let encode = code.encode();
        let png = Image::GIF {
            height: self.height,
            xdim: self.xdim,
            rotation: self.rotation,
            foreground: Color::new(self.foreground),
            background: Color::new(self.background),
        };
        let bytes = png.generate(&encode[..])?;
        let file = File::create(&Path::new(file_name.as_ref())).unwrap();
        let mut writer = BufWriter::new(file);
        writer.write_all(&bytes[..]).unwrap();
        Ok(())
    }

    pub fn generate_jpeg<T: AsRef<str>>(&self, data: T, file_name: T) -> Result<()> {
        let code = barcoders::sym::tf::TF::standard(data)?;
        let encode = code.encode();
        let png = Image::JPEG {
            height: self.height,
            xdim: self.xdim,
            rotation: self.rotation,
            foreground: Color::new(self.foreground),
            background: Color::new(self.background),
        };
        let bytes = png.generate(&encode[..])?;
        let file = File::create(&Path::new(file_name.as_ref())).unwrap();
        let mut writer = BufWriter::new(file);
        writer.write_all(&bytes[..]).unwrap();
        Ok(())
    }
}

macro_rules! make_class {
    (fn_name: $fn:ident, class_name: $class:expr, struct_name: $name:ident) => {
        fn $fn() -> StatefulClass<$name> {
            let mut class = StatefulClass::new_with_default_state($class);
            class.add_method(
                "__construct",
                Visibility::Public,
                |this, _| {
                    *this.as_mut_state() = $name::new();
                    Ok::<_, phper::Error>(())
                },
                vec![],
            );
            class.add_method(
                "setXdim",
                Visibility::Public,
                |this, arguments| {
                    this.as_mut_state()
                        .set_xdim(arguments[0].clone().expect_long()? as u32);
                    Ok::<_, phper::Error>(())
                },
                vec![Argument::by_val("xdim")],
            );
            class.add_method(
                "setHeight",
                Visibility::Public,
                |this, argument| {
                    this.as_mut_state()
                        .set_height(argument[0].clone().expect_long()? as u32);
                    Ok::<_, phper::Error>(())
                },
                vec![Argument::by_val("height")],
            );
            class.add_method(
                "setForeground",
                Visibility::Public,
                |this, argument| {
                    let mut param: [u8; 4] = [0, 0, 0, 255];
                    let mut i: usize = 0;
                    for argv in argument[0].clone().expect_z_arr()?.iter() {
                        param[i] = argv.1.expect_long()? as u8;
                        i += 1;
                    }
                    this.as_mut_state().set_foreground(param);
                    Ok::<_, phper::Error>(())
                },
                vec![Argument::by_val("foreground")],
            );
            class.add_method(
                "setBackground",
                Visibility::Public,
                |this, argument| {
                    let mut param: [u8; 4] = [255, 255, 255, 255];
                    let mut i: usize = 0;
                    for argv in argument[0].clone().expect_z_arr()?.iter() {
                        param[i] = argv.1.expect_long()? as u8;
                        i += 1;
                    }
                    this.as_mut_state().set_background(param);
                    Ok::<_, phper::Error>(())
                },
                vec![Argument::by_val("background")],
            );
            class.add_method(
                "generatePng",
                Visibility::Public,
                |this, argument| {
                    this.as_state()
                        .generate_png(
                            argument[0].clone().expect_z_str()?.to_str()?,
                            argument[1].clone().expect_z_str()?.to_str()?,
                        )
                        .unwrap();
                    Ok::<_, phper::Error>(())
                },
                vec![Argument::by_val("data"), Argument::by_val("fileName")],
            );
            class.add_method(
                "generateGif",
                Visibility::Public,
                |this, argument| {
                    this.as_mut_state()
                        .generate_gif(
                            argument[0].clone().expect_z_str()?.to_str()?,
                            argument[1].clone().expect_z_str()?.to_str()?,
                        )
                        .unwrap();
                    Ok::<_, phper::Error>(())
                },
                vec![Argument::by_val("data"), Argument::by_val("fileName")],
            );
            class.add_method(
                "generateJpeg",
                Visibility::Public,
                |this, argument| {
                    this.as_mut_state()
                        .generate_jpeg(
                            argument[0].clone().expect_z_str()?.to_str()?,
                            argument[1].clone().expect_z_str()?.to_str()?,
                        )
                        .unwrap();
                    Ok::<_, phper::Error>(())
                },
                vec![Argument::by_val("data"), Argument::by_val("fileName")],
            );
            class
        }
    };
}

make_class! {fn_name: make_codabar_class, class_name: "BarCodes\\Codabar", struct_name: PHPCodabar}
make_class! {fn_name: make_code11_class, class_name: "BarCodes\\Code11", struct_name: PHPCode11}
make_class! {fn_name: make_code128_class, class_name: "BarCodes\\Code128", struct_name: PHPCode128}
make_class! {fn_name: make_code39_class, class_name: "BarCodes\\Code39", struct_name: PHPCode39}
make_class! {fn_name: make_code93_class, class_name: "BarCodes\\Code93", struct_name: PHPCode93}
make_class! {fn_name: make_ean13_class, class_name: "BarCodes\\EAN13", struct_name: PHPEAN13}
make_class! {fn_name: make_ean8_class, class_name: "BarCodes\\EAN8", struct_name: PHPEAN8}
make_class! {fn_name: make_eansupp_class, class_name: "BarCodes\\EANSUPP", struct_name: PHPEANSUPP}
make_class! {fn_name: make_tf_class, class_name: "BarCodes\\TF", struct_name: PHPTF}

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    module.on_module_init(|_: ModuleContext| true);
    module.on_module_shutdown(|_| true);
    module.on_request_init(|_| true);
    module.on_request_shutdown(|_| true);

    module.add_class(make_codabar_class());
    module.add_class(make_code11_class());
    module.add_class(make_code128_class());
    module.add_class(make_code39_class());
    module.add_class(make_code93_class());
    module.add_class(make_ean13_class());
    module.add_class(make_ean8_class());
    module.add_class(make_eansupp_class());
    module.add_class(make_tf_class());

    module
}
