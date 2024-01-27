# Windows File Info - Windows Entity Inspector

This crate is for gathering useful infos about Windows entities(archives, directories or reparse point/symlinks) with developer friendly way.

This crate is parallel crate of [linux-file-info](https://crates.io/crates/linux-file-info) crate, which developed for handling same tasks on linux by me. If you need a entity inspector for linux, check out that crate.

Altough because the windows is very different from linux, this crate made same things with more different ways and with way more code. And also has another functionalities which added specifically for windows kernel.

Current Situation: According to my tests, in current situation the most important functions, which `current_folder_info()`, `other_folder_info()` and `entity_info()` runs around 500 millisecond speed if there is no many entity to search and index, which always the case for `entity_info()` function and most of the times for `current_folder_info()` function. If there is around 50-60 entity that functions run with half of that speed.

You can check it via running the individual tests, i added a timer for all that three function.

On Future: In order to optimize that crate: probably we require way more distribution of the logic or writing it with different language(such as zig or c++) and make rust bindings. My first plan is optimize that crate with lifetimes, if i can't make it enough, my backup plans are rewrite that crate with zig, until that time, use that crate with caution, it may consume too many memory and may run slow.

If you like that crate, give a star that liblary on [github repo](https://github.com/Necoo33/windows_file_info_rs)

This crate returns that kind of struct when you use it:

```rust

#[cfg(target_os = "windows")]
#[derive(Debug, Clone)]
pub struct WindowsEntity {
    pub mode: Vec<String>, // which permissions that entity has
    pub types: Vec<String>, // which types that entity has, in windows, an entity can be archive, directory and reparse point or symlink in the same time
    pub owner: String, // owning user of that entity
    pub last_write_time: String,
    pub name: String, // name of that entity
    pub creation_time: String, 
    pub attributes: String,
    pub last_access_time: String,
    pub size: i32, // as bytes
    pub absolute_path: String
}

```

sample uses:

```rust

use win_file_info::*;

fn main(){
    // get your current user:
    let current_user = get_current_user();

    // get entities of you current working folder:
    let current_folder = current_folder_info();

    // get your user folder's entities:
    let format_user_path = format!("C:\\Users\\{}", current_user);
    let your_user_folder = other_folder_info(&format_user_path).unwrap();

    // get the info of "cargo.exe" entity:
    let cargo_path = format!("C:\\Users\\{}\\.cargo\\bin\\cargo.exe", current_user);
    let get_cargo_exe_entity = entity_info(&cargo_path).unwrap();

    // assuming you have a windows path with only one backslash on every level and you want to format it to make 
    // 2 backslash at every path level. if it returns from a variable, you can use it directly but if you want to 
    // give it via hardcoding, you have to write it with two backslash, because rust understands one backslash as 
    // escape sequence. Also because of that if you have that path already it's pointless to use that function, but 
    // for example, if you used "std::env::current_dir" function it returns windows paths with only one backslash, 
    // you can use it:

    let current_directory_path = std::env::current_dir().unwrap();
    let current_directory_path = current_directory_path.as_path().to_str().unwrap();
    let format_the_windows_path = windows_path_two_backslash(current_directory_path);

    // or if you want to have a path with only one backslash and you have a path that has two backslash, you can use that function:

    let format_desktop_path = format!("C:\\\\Users\\\\{}\\\\Desktop", current_user);
    let format_the_windows_path = windows_path_one_backslash(&format_desktop_path);

    // checking the entity type:

    let is_users_folder_a_directory = is_directory("C:\\Users");
    let is_users_folder_a_archive = is_archive("C:\\Users");
    let is_users_folder_a_reparse_point_or_symlink = is_reparse_point_or_symlink("C:\\Users");

    // in windows, entities can be both directory, archive or reparse point or symlink, because of that we have
    // functions which checks all situations. And since, "OneDrive" folder has all 3 of that types, we use it as
    // example:

    let format_one_drive_path = format!("C:\\\\Users\\\\{}\\\\OneDrive", current_user);
    let mixed_types_one = is_directory_and_archive(&format_one_drive_path);
    let mixed_types_two = is_directory_and_reparse_point_or_symlink(&format_one_drive_path);
    let mixed_types_three = is_archive_and_reparse_point_or_symlink(&format_one_drive_path);
    let mixed_types_four = is_directory_and_archive_and_reparse_point_or_symlink(&format_one_drive_path);


}

```
