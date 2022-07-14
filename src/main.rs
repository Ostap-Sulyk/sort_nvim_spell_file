use dirs::home_dir;
use std::fs;
/// if you want to conveniently sort 
/// ```
/// .config/nvim/spell/en.utf-8.add
/// ```
/// file
/// and you want to sort words in that file,
/// you can install this binary and it should work
///
/// i am still new to rust and i know that this code is really unoptimized
/// so if you know how to improve it just do a pull request
///
/// # Example
/// after installing 'sort_nvim_spell_file'
/// just type it in terminal and it should sort your file
/// ```
/// sort_nvim_spell_file
/// ```
/// and press enter
/// this should sort your dictionary file 

fn main() -> std::io::Result<()> {
    let user_home = home_dir().unwrap();
    let mut user_home: String = user_home.to_str().unwrap().to_owned();

    user_home.push_str("/.config/nvim/spell/en.utf-8.add");

    let path_to_spell_file: &str = user_home.as_str();

    let mut words_list: Vec<String> = fs::read_to_string(path_to_spell_file)?
        .split('\n')
        .map(|x| x.to_string())
        .collect();

    words_list.sort();

    let mut contents = String::new();

    for word in words_list.iter() {
        let mut word = word.clone();
        word.push('\n');
        contents.push_str(word.as_str());
    }

    fs::write(path_to_spell_file, contents.trim())?;

    Ok(())
}
