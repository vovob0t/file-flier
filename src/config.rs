use std::{env, error::Error, process};

#[derive(Debug)]
enum ChartType {
    Pie,
    Colomns,
    Circle,
}

#[derive(Debug)]
pub enum SortType {
    Natural(SortDirection),
    Alphabetical(SortDirection),
    Size(SortDirection),
    Modification(SortDirection),
}

#[derive(Debug)]
pub enum SortDirection {
    Up,
    Down,
}

#[derive(Debug)]
pub struct Config {
    pub path: String,
    pub chart_type: ChartType,
    pub sort_type: SortType,
}

const HELP_MESSAGE: &str = "CLI tools to analyze directory space\nUsage:\n--help | -h  -  prints out this help message\n--path | -p  -  specify path of analyzing\n--sort | -s  -  specify sort algorithm by:\n\t\t\tsize, alphabetical, modification, natural";

impl Config {
    pub fn new(args: impl Iterator<Item = String>) -> Result<Self, Box<dyn Error>> {
        let arg_str = args.map(|el| el + " ").collect::<String>();
        if arg_str.contains("--help") || arg_str.contains("-h") {
            println!("{HELP_MESSAGE}");
            process::exit(1);
        };

        let args_parsed: Vec<(&str, &str)> = arg_str
            .trim()
            .split_whitespace()
            .filter(|str| str.contains("="))
            .map(|arg| {
                let a = arg.split("=").collect::<Vec<&str>>();
                (a[0], a[1])
            })
            .collect();

        let mut path = String::from("./");
        let mut chart_type = ChartType::Pie;
        let mut sort_type = SortType::Natural(SortDirection::Up);

        for (arg_name, value) in args_parsed {
            match arg_name.replace("-", "").as_str() {
                "p" | "path" => {
                    let home = if let Some(home_dir) = env::home_dir() {
                        home_dir.display().to_string()
                    } else {
                        "~".to_string()
                    };

                    path = String::from(value).replace("~", &home)
                }

                "c" | "chart" => {
                    chart_type = match value.to_lowercase().as_str() {
                        "pie" => ChartType::Pie,
                        "colomns" => ChartType::Colomns,
                        "circle" => ChartType::Circle,
                        _ => {
                            println!("Couldn't find appropriete chart type. Defaulting to Pie");
                            ChartType::Pie
                        }
                    }
                }

                "s" | "sort" => {
                    sort_type = match value.to_lowercase().as_str() {
                        "size" => SortType::Size(SortDirection::Down),
                        "alphabetical" => SortType::Alphabetical(SortDirection::Up),
                        "modification" => SortType::Modification(SortDirection::Up),
                        _ => {
                            println!("Couldn't find appropriete chart type. Defaulting to Pie");
                            SortType::Natural(SortDirection::Up)
                        }
                    }
                }
                "help" | "h" | _ => {
                    println!("{HELP_MESSAGE}");
                    process::exit(1);
                }
            }
        }

        Ok(Self {
            path,
            chart_type,
            sort_type,
        })
    }
}
