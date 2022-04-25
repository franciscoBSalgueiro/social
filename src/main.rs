use ansi_term::{Colour, Style};
use chrono::Datelike;
use clap::Parser;
use select::{document::Document, predicate::Attr};

const DIAS_DA_SEMANA: [&str; 5] = ["Segunda", "Terça", "Quarta", "Quinta", "Sexta"];

fn ementa(dia: Option<usize>, all: bool) {
    let url = "https://www.sas.ulisboa.pt/unidade-alimentar-tecnico-alameda";
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    let document = Document::from(response.as_str());
    let mut i = 0;
    for node in document.find(Attr("class", "menus")) {
        if all || i == dia.unwrap() {
            for child in node.children() {
                for subchild in child.children() {
                    let mut texto = subchild.text();
                    if texto != "" && !texto.contains("Alameda") && texto != "Linha" {
                        if texto.contains("2022") {
                            texto = format!("{}-Feira - {}", DIAS_DA_SEMANA[i], texto);
                            println!("\n{}", Style::new().bold().underline().paint(texto));
                        } else if texto == "Almoço" || texto == "Jantar" || texto == "Macrobiótica"
                        {
                            println!("\n{}", Style::new().bold().paint(texto));
                        } else if texto.contains("Contêm") {
                            println!("{}", Colour::Red.paint(texto));
                        } else {
                            println!("{}", texto);
                        }
                    }
                }
            }
        }
        i += 1;
    }
}

/// Program to fetch the menu of the University of Lisbon
#[derive(Parser, Debug)]
#[clap(about, long_about = None)]
struct Args {
    /// Show the menus from all days
    #[clap(short, long)]
    all: bool,

    /// Shows the menu from the specified day
    #[clap(short, long)]
    day: Option<usize>,
}

fn main() {
    let args = Args::parse();

    if !args.all && args.day.is_none() {
        let current_date: usize = chrono::offset::Local::today()
            .weekday()
            .num_days_from_monday()
            .try_into()
            .unwrap();
        ementa(Some(current_date), false);
    } else {
        ementa(args.day, args.all);
    }
}
