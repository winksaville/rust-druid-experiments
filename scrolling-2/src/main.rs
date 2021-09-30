// Based on scrolling-1 but create a Vec<String>.
use druid::widget::{Flex, Label};
use druid::{AppLauncher, Widget, WidgetExt, WindowDesc};
use std::env;

fn create_data(line_cnt: u64) -> Vec<String> {
    let mut lines = vec![];
    for i in 0..line_cnt {
        lines.push(format!("line {}:", i));
    }

    lines
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Usage: {} number_of_lines", args[0]);
        return;
    }
    let line_cnt = match args[1].parse::<u64>() {
        Ok(cnt) => cnt,
        _ => {
            println!("Expected first parameter to be an u64 number of lines");
            return;
        }
    };

    let lines = create_data(line_cnt);

    let window = WindowDesc::new(build_widget(lines)); //.title(format!("{} {}", args[0], line_cnt));
    AppLauncher::with_window(window)
        .log_to_console()
        .launch("".to_string())
        .expect("launch failed");
}

fn build_widget(lines: Vec<String>) -> impl Widget<String> {
    let mut col = Flex::column();
    for s in lines {
        col.add_child(Label::new(s));
    }
    col.scroll().vertical()
}
