use ansi_term::{Colour, Style};
use chrono::Datelike;
use clap::Parser;
use select::{document::Document, predicate::Attr};

const WEEK_DAYS_SHORT: [&str; 5] = ["seg", "ter", "qua", "qui", "sex"];
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

/// Command line tool to fetch the menu of the University of Lisbon
///
/// When no argument is provided, today's menu is shown
#[derive(Parser, Debug)]
struct Args {
    /// Show all the menus of the week
    #[clap(short, long)]
    all: bool,

    /// Shows the menu from the specified day
    #[clap(short, long)]
    day: Option<String>,
}

fn main() {
    let args = Args::parse();

    // Convert the day argument from week day to an integer starting in monday with 0
    let day = match args.day {
        Some(day) => WEEK_DAYS_SHORT
            .iter()
            .position(|&x| day.to_ascii_lowercase().contains(&x))
            .unwrap_or_else(|| {
                eprintln!("Invalid day argument\nValid days are: {}", WEEK_DAYS_SHORT.join(", "));
                std::process::exit(1);
            }),
        None => chrono::offset::Local::today()
            .weekday()
            .num_days_from_monday()
            .try_into()
            .unwrap(),
    };

    ementa(day, args.all);
}
