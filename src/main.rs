use clap::{App, Arg};
use walkdir::{WalkDir};
use std::path::{PathBuf, Path};
use std::fs::rename;

// Structure for arguments
#[derive(Debug)]
struct CommandArguments {
    traverse_recursive : bool,
    skip_error: bool,
    source : PathBuf,
    target : PathBuf,
}

fn traverse_folder(recursive: bool, source_folder: PathBuf) -> Vec<PathBuf> {

    let mut item_list = Vec::<PathBuf>::new();
    
    let walker;
    
    if recursive {
        walker = WalkDir::new(source_folder);
    }
    else {
        walker = WalkDir::new(source_folder).max_depth(1);
    }

    for entry in walker {
        let item = entry.unwrap();
        let path = item.into_path();
        if path.is_file() {
            item_list.push(path);
        }
    }
    item_list
}

// Check if the source is a valid path name 
fn is_valid_source_path (val: String) -> Result<(), String> {
    let path = Path::new(&val);
    if (path.is_dir() || path.is_file()) && path.is_absolute() {
       Ok(()) 
    }
    else {
        Err(String::from("The source must be a valid absolute path or file name."))
    }
}

// Check if the target is a valid path name 
fn is_valid_target_path (val: String) -> Result<(), String> {
    let path = Path::new(&val);
    if path.is_dir() && path.is_absolute() {
       Ok(()) 
    }
    else {
        Err(String::from("The target must be a valid absolute path name."))
    }
}

// Move file to target folder
fn move_file(source_file_name: &Path, target_path: &Path) -> std::io::Result<()> {

    let source_folder_name = source_file_name.parent().unwrap();
    let file_name = source_file_name.file_name().unwrap();
    let mut target_file_name = target_path.to_path_buf();
    target_file_name.push(file_name);
    
    println!("fenmov::moving file {:?} from path {:?} to target path {:?}.", file_name, source_folder_name, target_path);
    rename(source_file_name, target_file_name.as_path())?;
    Ok(())
}

fn main() {

    // Set Command Line App - fenmov
    let app = App::new("fenmov")
    .about("A Command to  traverse file folder hierarchy in search of files and move them to a target path. ")
    .arg(Arg::with_name("recursive")
        .short("r")
        .long("recursive")
        .help("Traverses folder recursively")
        .required(false))
    .arg(Arg::with_name("source-path")
        .short("s")
        .long("source")
        .help("Sets source path name from where files are needed to be moved")
        .takes_value(true)
        .required(true)
        .validator(is_valid_source_path))
    .arg(Arg::with_name("target-path")
        .short("t")
        .long("target")
        .help("Sets target path where files will be moved")
        .takes_value(true)
        .required(true)
        .validator(is_valid_target_path))
    .arg(Arg::with_name("skip-error")
        .short("e")
        .long("skip-error")
        .help("Skip error while moving files")
        .required(false))
    .author("Fenergo")
    .version("0.1.1")
    .get_matches();

    // Get fenmov Arguments
    let args = CommandArguments {
        skip_error : app.is_present("skip-error"),
        traverse_recursive : app.is_present("recursive"),
        source : PathBuf::from(app.value_of("source-path").unwrap()),
        target : PathBuf::from(app.value_of("target-path").unwrap()),
    };

    println!("fenmov::Arguments - {:?}", args);   
    
    
    // Get list of file to be moved
    let items = traverse_folder(args.traverse_recursive, args.source);

    // Loop thru all items and move them to target
    let mut count= 0usize;
    for item in &items {
        match move_file(item.as_path(), args.target.as_path()) {
            Ok(()) => count += 1,
            Err(e) => {
                println!("fenmov:: Error {} occurred while moving file {:?}", e, &item);
                if !args.skip_error {
                    break;
                }
            },
        }
    }
    println!("fenmov::Number of file(s) moved = {}", count);
}
