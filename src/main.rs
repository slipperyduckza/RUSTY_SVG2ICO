#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::widget::{button, column, container, image, row, scrollable, text, vertical_space};
use iced::{Alignment, Application, Color, Command, Element, Length, Settings, Size, Theme, alignment, window};

struct MyContainerStyle(Color);

impl container::StyleSheet for MyContainerStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(self.0)),
            border: iced::Border {
                color: Color::BLACK,
                width: 1.0,
                radius: 10.0.into(),
            },
            ..Default::default()
        }
    }
}

struct MainBgStyle(Color);

impl container::StyleSheet for MainBgStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(self.0)),
            ..Default::default()
        }
    }
}

struct LogoStyle;

impl container::StyleSheet for LogoStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(Color::from_rgb(144.0 / 255.0, 144.0 / 255.0, 144.0 / 255.0))), // #909090
            border: iced::Border {
                radius: 10.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
use std::io;

// Embed the logo image data at compile time so it's included in the executable
static LOGO_DATA: &[u8] = include_bytes!("../assets/RUSTYSVG2ICO420.png");

struct SvgToIcoApp {
    ico_data: Option<Vec<u8>>,
    images: Vec<(iced::widget::image::Handle, String)>,
    is_generated: bool,
    logo: Option<iced::widget::image::Handle>,
    is_dark: bool,
}

#[derive(Debug, Clone)]
enum Message {
    SelectSvg,
    OpenIco,
    SaveIcon,
    IcoLoaded(Vec<u8>, bool),
}

impl SvgToIcoApp {
    fn load_images(&mut self, data: &[u8]) {
        let icon_dir = ico::IconDir::read(io::Cursor::new(data)).unwrap();
        self.images.clear();
        for entry in icon_dir.entries() {
            let handle = iced::widget::image::Handle::from_memory(entry.data().to_vec());
            self.images.push((handle, format!("{} x {}", entry.width(), entry.height())));
        }
    }
}

impl Application for SvgToIcoApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = bool;

    fn new(flags: bool) -> (Self, Command<Message>) {
        let mut app = SvgToIcoApp {
            ico_data: None,
            images: vec![],
            is_generated: false,
            logo: None,
            is_dark: flags,
        };
        app.logo = Some(iced::widget::image::Handle::from_memory(LOGO_DATA.to_vec()));
        (app, Command::none())
    }

    fn title(&self) -> String {
        "Rusty SVG2ICO".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SelectSvg => {
                Command::perform(
                    async {
                        tokio::task::spawn_blocking(|| {
                            rfd::FileDialog::new().add_filter("SVG", &["svg"]).pick_file()
                        }).await.unwrap()
                    },
                    |path_opt| {
                        if let Some(path) = path_opt {
                            let temp_dir = tempfile::TempDir::new().unwrap();
                            let temp_path = temp_dir.path().join("temp.ico");
                            svg_to_ico::svg_to_ico(&path, 256.0, &temp_path, &[256u16, 128, 64, 48, 32, 24, 16]).unwrap();
                            let ico_data = std::fs::read(&temp_path).unwrap();
                            Message::IcoLoaded(ico_data, true)
                        } else {
                            Message::IcoLoaded(vec![], false)
                        }
                    }
                )
            }
            Message::OpenIco => {
                Command::perform(
                    async {
                        tokio::task::spawn_blocking(|| {
                            rfd::FileDialog::new().add_filter("ICO", &["ico"]).pick_file()
                        }).await.unwrap()
                    },
                    |path_opt| {
                        if let Some(path) = path_opt {
                            let ico_data = std::fs::read(path).unwrap();
                            Message::IcoLoaded(ico_data, false)
                        } else {
                            Message::IcoLoaded(vec![], false)
                        }
                    }
                )
            }
            Message::SaveIcon => {
                if let Some(data) = &self.ico_data {
                    let data = data.clone();
                    Command::perform(
                        async {
                            tokio::task::spawn_blocking(|| {
                                rfd::FileDialog::new().add_filter("ICO", &["ico"]).save_file()
                            }).await.unwrap()
                        },
                        move |path_opt| {
                            if let Some(path) = path_opt {
                                std::fs::write(path, &data).unwrap();
                            }
                            Message::IcoLoaded(vec![], false) // dummy
                        }
                    )
                } else {
                    Command::none()
                }
            }
            Message::IcoLoaded(data, generated) => {
                if !data.is_empty() {
                    self.ico_data = Some(data.clone());
                    self.is_generated = generated;
                    self.load_images(&data);
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let logo = container(image(self.logo.as_ref().unwrap().clone()).width(Length::Fixed(200.0)))
            .width(Length::Fill)
            .center_x()
            .center_y()
            .style(iced::theme::Container::Custom(Box::new(LogoStyle)))
            .padding([0, 20, 10, 20]); // top 0, right 20, bottom 10, left 20

        let select_button = button("Select SVG File").on_press(Message::SelectSvg);
        let open_button = button("Open ICO File").on_press(Message::OpenIco);

        let buttons_row = row![select_button, open_button].spacing(10);

        let save_button = if self.ico_data.is_some() && self.is_generated {
            Some(button("Save Icon").on_press(Message::SaveIcon))
        } else {
            None
        };

        let images_column = if self.images.is_empty() {
            column![].height(Length::Fixed(400.0))
        } else {
            let mut col = column![].spacing(10);
            for (handle, res) in &self.images {
                let img = image(handle.clone());
                let txt = text(res).style(iced::theme::Text::Color(Color::WHITE));
                let txt_container = container(txt).width(Length::Fill).align_x(alignment::Horizontal::Right);
                col = col.push(row![img, txt_container].spacing(10).align_items(Alignment::Center));
            }
            col
        };

        let scrollable_images = scrollable(images_column).height(Length::Fixed(646.0)).width(Length::Fixed(380.0));

        let container_bg_color = if self.is_dark {
            Color::from_rgb(128.0 / 255.0, 128.0 / 255.0, 128.0 / 255.0) // grey
        } else {
            Color::from_rgb(1.0, 1.0, 240.0 / 255.0) // ivory
        };
        let framed_images = container(scrollable_images)
            .height(Length::Fixed(646.0))
            .style(iced::theme::Container::Custom(Box::new(MyContainerStyle(container_bg_color))))
            .padding(6);

        let mut content = column![logo, buttons_row]
            .spacing(10)
            .align_items(Alignment::Center);

        if let Some(save) = save_button {
            content = content.push(save);
        }

        content = content.push(vertical_space().height(6));
        content = content.push(framed_images);

        let main_bg_color = Color::from_rgb(48.0 / 255.0, 48.0 / 255.0, 48.0 / 255.0); // #303030

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(iced::theme::Container::Custom(Box::new(MainBgStyle(main_bg_color))))
            .into()
    }
}

fn main() -> iced::Result {
    let is_dark = dark_light::detect().unwrap_or(dark_light::Mode::Light) == dark_light::Mode::Dark;
    let icon = iced::window::icon::from_file("rustysvg2ico.ico").ok();
    SvgToIcoApp::run(Settings {
        flags: is_dark,
        window: window::Settings {
            size: Size::new(420.0, 868.0),
            icon,
            ..Default::default()
        },
        ..Default::default()
    })
}