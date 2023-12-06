use polars::prelude::*;
use polars::prelude::DataFrame;
use std::fs;
use std::path::Path;
//use chrono::prelude::*;
//use std::io::Prelude::*;

//const PROJECT_NAMES:[&str; 2] = ["JNp1", "JNp2"];
const EXPERIMENT_FOLDERS:[&str; 4] = ["Data", "Figures", "Scripts", "Writeup"];
const BASE_NAME: &str = "Experiment1";
const WORKING_DIR: &str = ".git";
const DATA_FILE: &str = "/home/raas/Documents/projects/rust/sunflower/Experiment1/Data/titers.csv";

slint::include_modules!();
fn main(){
    let df = open_dataframe(DATA_FILE);
    println!("{:?}", df.describe(None));
 //   let ex_ids = df.select([col("ex_id")]).collect()?;
 //   println!("{:?}", &ex_ids);

    checkfile(WORKING_DIR);
    let _ = create_experiment(BASE_NAME);
    MainWindow::new().unwrap().run().unwrap();
    //SplashWindow::new().unwrap().run().unwrap();
}

fn checkfile(working: &str){
    if Path::new(&working).exists() {
        println!("Found the thing");
    } else {
        println!("Did not find the thing");
    }
}

fn create_experiment(base_path: &str) {
    if Path::new(&base_path).exists(){
        println!("The directory {} already exists", base_path);
    } else {
        let _ = fs::create_dir(&base_path);
        for exp_folder in EXPERIMENT_FOLDERS{
            let path = format!("{BASE_NAME}/{exp_folder}");
            //println!("{}", path);
            if Path::new(&path).exists(){
                println!("The directory {} already exists", path);
            } else {
                //println!("Er, doesn't exist?");
                let _ = fs::create_dir(path);
            }
        }
    }
}

fn open_dataframe(base_path: &str) -> PolarsResult<DataFrame>{
    CsvReader::from_path(base_path)?
            .has_header(true)
            .finish()
}

//fn create_dataframe(){
//    let df = df!(
//            "id" => &[9, 4, 2],
//            "place" => &["Mars", "Earth", "Saturn"],
//        "date" => date_range("date",
//                NaiveDate::from_ymd_opt(2022, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(), NaiveDate::from_ymd_opt(2022, 1, 3).unwrap().and_hms_opt(0, 0, 0).unwrap(), Duration::parse("1d"),ClosedWindow::Both, TimeUnit::Milliseconds, None)?,
//           "sales" => &[33.4, 2142134.1, 44.7],
//            "has_people" => &[false, true, false],
//            "logged_at" => date_range("logged_at",
//                NaiveDate::from_ymd_opt(2022, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(), NaiveDate::from_ymd_opt(2022, 1, 1).unwrap().and_hms_opt(0, 0, 2).unwrap(), Duration::parse("1s"),ClosedWindow::Both, TimeUnit::Milliseconds, None)?,
//    )?
//    .with_row_count("rn", None)?;
//    println!("{}", &df);
//}
