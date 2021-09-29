// Based on: https://github.com/linebender/druid/issues/1990 but
// use Label instead of TextBox as the data is immutable.
use druid::widget::{Flex, Label};
use druid::{AppLauncher, Widget, WidgetExt, WindowDesc};
use std::env;

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

    let window = WindowDesc::new(build_widget(line_cnt)).title(format!("{} {}", args[0], line_cnt));
    AppLauncher::with_window(window)
        .log_to_console()
        .launch("".to_string())
        .expect("launch failed");
}

fn build_widget(line_cnt: u64) -> impl Widget<String> {
    let mut col = Flex::column();
    for i in 0..line_cnt {
        col.add_child(Label::new(format!("line {}", i)));
    }
    col.scroll().vertical()
}
