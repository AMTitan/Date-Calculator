use chrono::Datelike;
use chrono::Local;
use chrono::NaiveDate;
use fltk::button::*;
use fltk::enums::*;
use fltk::frame::*;
use fltk::prelude::*;
use fltk::window::*;
use fltk::input::*;
use fltk::*;
use chrono::Duration;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Calculate,
}

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = Window::default()
        .with_size(600,300)
        .with_label("Rust Menu")
        .center_screen();
    //wind.make_resizable(true);
    
    let mut frame = Frame::new(270, 230, 0, 75, "");
    frame.set_label_color(Color::White);
    frame.set_label_size(35);
    frame.set_align(Align::Left);

    let now = Local::now();
    let date_now = now.format("%m/%d/%Y").to_string();

    let mut date = Input::new(100,25,300,25, "Start date");
    date.set_value(&date_now);

    let mut days = Input::new(100,100,300,25, "Days offset");
    days.set_value("0");

    let sunday = CheckButton::new(450, 30, 75, 30, "Sunday");
    let monday = CheckButton::new(450, 60, 75, 30, "Monday");
    let tuesday = CheckButton::new(450, 90, 75, 30, "Tuesday");
    let wednesday = CheckButton::new(450, 120, 75, 30, "Wednesday");
    let thursday = CheckButton::new(450, 150, 75, 30, "Thursday");
    let friday = CheckButton::new(450, 180, 75, 30, "Friday");
    let saturday = CheckButton::new(450, 210, 75, 30, "Saturday");
    let increase = CheckButton::new(450, 270, 75, 30, "Go forward to day");
    let mut calculate = Button::new(300, 230, 75, 75, "Calculate");

    wind.end();
    wind.show();

    let (s, r) = app::channel::<Message>();

    calculate.emit(s, Message::Calculate);

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::Calculate => {
                    let mut offset = 0;
                    let mut now;
                    if days.value().trim().parse::<i64>().unwrap() + offset > 0
                    {
                        now = NaiveDate::parse_from_str(&date.value(), "%m/%d/%Y").unwrap() + Duration::days(days.value().trim().parse::<i64>().unwrap() + offset);
                    }
                    else {
                        now = NaiveDate::parse_from_str(&date.value(), "%m/%d/%Y").unwrap() - Duration::days((days.value().trim().parse::<i64>().unwrap() + offset)*-1);
                    }
                    let mut dayes = Vec::new();
                    if sunday.is_checked() {
                        dayes.push("Sun");
                    }
                    if monday.is_checked() {
                        dayes.push("Mon");
                    }
                    if tuesday.is_checked() {
                        dayes.push("Tue");
                    }
                    if wednesday.is_checked() {
                        dayes.push("Wen");
                    }
                    if thursday.is_checked() {
                        dayes.push("Thu");
                    }
                    if friday.is_checked() {
                        dayes.push("Fri");
                    }
                    if saturday.is_checked() {
                        dayes.push("Sat");
                    }
                    frame.set_label("                           ");
                    if dayes.len() != 0 {
                        while !dayes.contains(&now.weekday().to_string().as_str()) {
                            if increase.is_checked() {
                                offset += 1;
                                if days.value().trim().parse::<i64>().unwrap() + offset > 0
                                {
                                    now = NaiveDate::parse_from_str(&date.value(), "%m/%d/%Y").unwrap() + Duration::days(days.value().trim().parse::<i64>().unwrap() + offset);
                                }
                                else {
                                    now = NaiveDate::parse_from_str(&date.value(), "%m/%d/%Y").unwrap() - Duration::days((days.value().trim().parse::<i64>().unwrap() + offset)*-1);
                                }
                            }
                            else {
                                offset -= 1;
                                if days.value().trim().parse::<i64>().unwrap() + offset > 0
                                {
                                    now = NaiveDate::parse_from_str(&date.value(), "%m/%d/%Y").unwrap() + Duration::days(days.value().trim().parse::<i64>().unwrap() + offset);
                                }
                                else {
                                    now = NaiveDate::parse_from_str(&date.value(), "%m/%d/%Y").unwrap() - Duration::days((days.value().trim().parse::<i64>().unwrap() + offset)*-1);
                                }
                            }
                        }
                        frame.set_label(&now.format("%m/%d/%Y").to_string());
                    }
                    else {
                        frame.set_label("no days picked");
                    }
                },
            }
        }
        calculate.redraw();
        calculate.clear_visible_focus();
    }
}