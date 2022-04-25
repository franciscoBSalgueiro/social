use ansi_term::{Colour, Style};
use chrono::Datelike;
use clap::{ArgEnum, Parser};
use select::{document::Document, predicate::Attr};

const WEEK_DAYS: [&str; 5] = ["Segunda", "Terça", "Quarta", "Quinta", "Sexta"];

fn ementa(day: usize, all: bool) {
    let url = "https://www.sas.ulisboa.pt/unidade-alimentar-tecnico-alameda";
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    let document = Document::from(response.as_str());
    let mut i = 0;
    for node in document.find(Attr("class", "menus")) {
        if all || i == day {
            for child in node.children() {
                for subchild in child.children() {
                    let mut text = subchild.text();
                    if text != "" && !text.contains("Alameda") && text != "Linha" {
                        if text.contains("202") {
                            text = format!("{}-Feira - {}", WEEK_DAYS[i], text);
                            println!("\n{}", Style::new().bold().underline().paint(text));
                        } else if text == "Almoço" || text == "Jantar" || text == "Macrobiótica" {
                            println!("\n{}", Style::new().bold().paint(text));
                        } else if text.contains("Contêm") {
                            println!("{}", Colour::Red.paint(text));
                        } else {
                            println!("{}", text);
                        }
                    }
                }
            }
        }
        i += 1;
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
enum WeekdayArg {
    Seg,
    Ter,
    Qua,
    Qui,
    Sex,
}

/// Command line tool to fetch the menu of the IST Alameda canteen
///
/// When no argument is provided, today's menu is shown
#[derive(Parser, Debug)]
struct Args {
    /// Prints all the menus of the week
    #[clap(short, long)]
    all: bool,

    /// Prints the menu from that day
    #[clap(short, long, arg_enum)]
    day: Option<WeekdayArg>,
}

fn main() {
    let args = Args::parse();

    // Convert the day argument from week day to an integer starting in monday with 0
    let day = match args.day {
        Some(WeekdayArg::Seg) => 0,
        Some(WeekdayArg::Ter) => 1,
        Some(WeekdayArg::Qua) => 2,
        Some(WeekdayArg::Qui) => 3,
        Some(WeekdayArg::Sex) => 4,
        None => chrono::offset::Local::today()
            .weekday()
            .num_days_from_monday()
            .try_into()
            .unwrap(),
    };

    ementa(day, args.all);
}
