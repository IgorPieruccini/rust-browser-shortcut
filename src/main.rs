use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::{env, io};
mod concat_with_exe_path;
mod prompt_options;
mod read_file;
mod save_to_file;

fn main() {
    let path_file: &String = &String::from("browserdata.txt");
    let path: String = concat_with_exe_path::get();

    env::set_current_dir(path.to_string()).expect("Could not set current dir");
    assert!(env::set_current_dir(&path).is_ok());

    let commands: Vec<&str> = vec!["Open website", "Add website"];

    let selected_command = prompt_options::show(&commands);

    match selected_command {
        "Add website" => {
            let mut input_alias: String = String::new();
            println!("Type an alias:");
            io::stdin()
                .read_line(&mut input_alias)
                .expect("Could not add alias");

            input_alias.pop();

            println!("Alias: {}", input_alias.to_string());

            println!("Type the url:");

            let mut input_url: String = String::new();

            io::stdin()
                .read_line(&mut input_url)
                .expect("Could not add url");

            input_url.pop();

            println!("URL: {}", input_url.to_string());

            let input: String = String::from(format!("{}={}{}", input_alias, input_url, "\n"));
            save_to_file::write(&input, &path, path_file.to_owned());
        }
        "Open website" => {
            println!("Choose a url:");
            let mut input = String::new();
            let alias = read_file::read(&mut input, &path, path_file.to_owned());
            let selected_option = prompt_options::show(&alias);
            let url: Vec<&str> = selected_option.split('=').collect();
            match url.len() == 2 {
                true => {
                    let command = format!(r"open /Applications/Brave\ Browser.app {}", url[1]);
                    let mut ctx = ClipboardContext::new().unwrap();
                    ctx.set_contents(command).unwrap();
                    println!("Awesome the command has been passed to your clipboard");
                }
                false => println!("Url is saved with the wrong format"),
            }
        }
        _ => print!("Error when selecting option"),
    }
}
