use std::{process::{Command, Output}, str::from_utf8, io::Error, env::current_dir};

#[cfg(target_os = "windows")]
#[derive(Debug, Clone)]
pub struct WindowsEntity {
    pub mode: Vec<String>,
    pub types: Vec<String>,
    pub owner: String,
    pub last_write_time: String,
    pub name: String,
    pub creation_time: String,
    pub attributes: String,
    pub last_access_time: String,
    pub size: i32,
    pub absolute_path: String
}

#[cfg(target_os = "windows")]
pub struct Permissions<'a> {
    pub entity_types: Vec<&'a str>,
    pub permissions: Vec<&'a str>
}


#[cfg(target_os = "windows")]
fn check_entity_permissions(perm_str: &str) -> Permissions {
    let split_the_perm_str: Vec<&str> = perm_str.split("").collect();

    let mut entity_types = vec![];
    let mut all_permissions = vec![];

    for permission_string in split_the_perm_str.into_iter() {
        match permission_string {
            "d" => {
                entity_types.push("directory");
                all_permissions.push("directory");
            },
            "a" => {
                entity_types.push("archive");
                all_permissions.push("archive");
            },
            "r" => all_permissions.push("read-only"),
            "h" => all_permissions.push("hidden"),
            "s" => all_permissions.push("system"),
            "l" => {
                entity_types.push("reparse-point-or-symlink");
                all_permissions.push("reparse-point-or-symlink");
            },
            &_ => ()
        }
    }

    return Permissions {
        entity_types,
        permissions: all_permissions
    }
}

#[cfg(target_os = "windows")]
fn extract_the_output<'a>(output: &'a Output) -> Vec<&'a str> {
    let our_output = from_utf8(&output.stdout).unwrap();
    let mut individual_lines: Vec<&'a str> = vec![];
    let mut start_num: usize = 0;
    let mut updated_start_num: usize = 0;

    for (index, line) in our_output.lines().into_iter().enumerate() {
        if index < 2 {
            continue;
        }

        if index == updated_start_num {
            start_num = start_num + 8 as usize;
        }

        if index >= 2 {
            if index == 2 {
                start_num = 2;
                updated_start_num = start_num + 8 as usize;
            }

            if index <= updated_start_num {
                individual_lines.push(line);
            }

            if index == updated_start_num {
                updated_start_num = updated_start_num + 8 as usize;
            }
        }
    }

    return individual_lines;
}

#[cfg(target_os = "windows")]
fn empty_lines_cleanup(lines_of_output: Vec<&str>) -> Vec<&str> {
    let empty_string = "".to_string();

    return lines_of_output.into_iter().filter(|line| *line != empty_string).collect();
}

#[cfg(target_os = "windows")]
fn format_the_output(lines_of_output: Vec<&str>) -> Vec<String> {
    let mut entity_strings_collector = vec![];
    let mut final_vector = vec![];
    let mut concatenation_num: i8 = 0;
    let seperator = " &*& ".to_string();
    
    for line in lines_of_output.into_iter() {
        match line {
            "" => (),
            &_ => {
                entity_strings_collector.push(line);
                entity_strings_collector.push(seperator.as_str());
                concatenation_num = concatenation_num + 1;
            }
        };

        if &concatenation_num == &9 {
            let entity_strings_collector = &entity_strings_collector;
            let entity_strings_collector = entity_strings_collector.join("");
            final_vector.push(entity_strings_collector);
            let _entity_strings_collector = &String::new();
            let _entity_strings_collector: Vec<String> = vec![];
            concatenation_num = 0
        }
    }

    return final_vector.into_iter().map(|s| s.clone()).collect();
}

#[cfg(target_os = "windows")]
fn split_the_formatted_output(formatted_output: Vec<String>) -> Vec<WindowsEntity> {
    let mut final_vector: Vec<WindowsEntity> = vec![];
    
    for line in formatted_output.into_iter() {
        let split_the_line: Vec<&str> = line.split("&*&").collect();

        let windows_entity = create_windows_entity(split_the_line);

        final_vector.push(windows_entity);
    }

    return final_vector
}

#[cfg(target_os = "windows")]
fn create_windows_entity(get_splitted_line: Vec<&str>) -> WindowsEntity {
    let mut mode = "".to_string();
    let mut owner = "".to_string();
    let mut last_write_time = "".to_string();
    let mut name = "".to_string();
    let mut creation_time = "".to_string();
    let mut attributes = "".to_string();
    let mut last_access_time = "".to_string();
    let mut size: i32 = 0;
    let mut full_name = "".to_string();

    for splitted_line in get_splitted_line.into_iter() {
        let first_seventeen_character: String = splitted_line.chars().take(17).collect();

        match first_seventeen_character.as_str().trim() {
            "Mode           :" => mode = splitted_line.split(" :").collect::<Vec<&str>>()[1].trim().to_string(),
            "Owner          :" => owner = splitted_line.split(" :").collect::<Vec<&str>>()[1].trim().to_string(),
            "LastWriteTime  :" => last_write_time = splitted_line.split(" :").collect::<Vec<&str>>()[1].trim().to_string(),
            "Name           :" => name = splitted_line.split(" :").collect::<Vec<&str>>()[1].trim().to_string(),
            "CreationTime   :" => creation_time = splitted_line.split(" :").collect::<Vec<&str>>()[1].trim().to_string(),
            "Attributes     :" => attributes = splitted_line.split(" :").collect::<Vec<&str>>()[1].trim().to_string(),
            "LastAccessTime :" => last_access_time = splitted_line.split(" :").collect::<Vec<&str>>()[1].trim().to_string(),
            "Length         :" => size = splitted_line.split(" :").collect::<Vec<&str>>()[1].trim().parse().unwrap_or(0),
            "FullName       :" => full_name = splitted_line.split(" :").collect::<Vec<&str>>()[1].trim().to_string(),
            &_ => ()
        }
    }

    let entity_permissions = check_entity_permissions(&mode);

    return WindowsEntity {
        mode: entity_permissions.permissions.into_iter().map(|x| x.to_string()).collect(), 
        types: entity_permissions.entity_types.into_iter().map(|x| x.to_string()).collect(), 
        owner, last_write_time, name, creation_time, attributes, last_access_time, size, absolute_path: full_name
    };
}

#[cfg(target_os = "windows")]
pub fn current_folder_info() -> Vec<WindowsEntity> {
    let current_path = current_dir().unwrap();
    let current_path = current_path.as_path().to_str().unwrap();
    let current_path = format!("{}", current_path);
    let current_path = current_path.as_str();

    let _turn_terminal_into_utf8 = Command::new("cmd")
                                                    .args(&["/C", "chcp 65001"])
                                                    .output()
                                                    .expect("cannot convert terminal's character encoding to utf8");

    let get_files_command = Command::new("powershell")
                                                            .args(&["-Command", &format!("Get-ChildItem -Path '{}' | Select-Object Mode, @{{Name='Owner'; Expression={{(Get-Acl $_.FullName).Owner}}}}, LastWriteTime, Name, CreationTime, Attributes, LastAccessTime, Length, FullName", current_path)])
                                                            .output();

    match get_files_command {
        Ok(answer) => {
            println!("get files command: {}", from_utf8(&answer.stderr).unwrap());
            let extracted_output = extract_the_output(&answer);

            let output_cleanup = empty_lines_cleanup(extracted_output);

            let formatting_output = format_the_output(output_cleanup);

            return split_the_formatted_output(formatting_output);
        },
        Err(error) => {
            println!("Some Error Happened when we try to get files: {}", error);

            return vec![]
        }
    }
}

#[cfg(target_os = "windows")]
pub fn other_folder_info(path: &str) -> Result<Vec<WindowsEntity>, Error> {
    let _turn_terminal_into_utf8 = Command::new("cmd")
                                                    .args(&["/C", "chcp 65001"])
                                                    .output()
                                                    .expect("cannot convert terminal's character encoding to utf8");

    let get_files_command = Command::new("powershell")
                                                        .args(&["Get-ChildItem", "-Path", path, "|", "Select-Object", "Mode, @{Name='Owner'; Expression={(Get-Acl $_.FullName).Owner}}, LastWriteTime, Name, CreationTime, Attributes, LastAccessTime, Length, FullName"])
                                                        .output();

    return match get_files_command {
        Ok(answer) => {
            let extracted_output = extract_the_output(&answer);

            let output_cleanup = empty_lines_cleanup(extracted_output);

            let formatting_output = format_the_output(output_cleanup);

            Ok(split_the_formatted_output(formatting_output))
        },
        Err(error) => {
            println!("Some Error Happened when we try to get files: {}", error);

            Err(error)
        }
    }
}

#[cfg(target_os = "windows")]
pub fn entity_info(path: &str) -> Result<WindowsEntity, Error> {
    let _turn_terminal_into_utf8 = Command::new("cmd")
                                                    .args(&["/C", "chcp 65001"])
                                                    .output()
                                                    .expect("cannot convert terminal's character encoding to utf8");

    let get_files_command = Command::new("powershell")
                                                        .args(&["Get-Item", "-Path", path, "|", "Select-Object", "Mode, @{Name='Owner'; Expression={(Get-Acl $_.FullName).Owner}}, LastWriteTime, Name, CreationTime, Attributes, LastAccessTime, Length, FullName"])
                                                        .output();

    match get_files_command {
        Ok(answer) => {
            let extracted_output = extract_the_output(&answer);

            let output_cleanup = empty_lines_cleanup(extracted_output);

            let formatting_output = format_the_output(output_cleanup);

            let get_results = split_the_formatted_output(formatting_output);

            return match get_results.len() as i8 {
                0_i8 => {
                    Ok(WindowsEntity {
                        mode: vec![],
                        types: vec![],
                        owner: "".to_string(),
                        last_write_time: "".to_string(),
                        last_access_time: "".to_string(),
                        name: "".to_string(),
                        creation_time: "".to_string(),
                        attributes: "".to_string(),
                        size: 0,
                        absolute_path: path.to_string()
                    })
                },
                1_i8 => Ok(get_results[0].clone()),
                _ => Err(Error::new(std::io::ErrorKind::Other, "Error"))
            }
        },
        Err(error) => {
            println!("Some Error Happened when we try to get entity: {}", error);

            Err(Error::new(std::io::ErrorKind::Other, error))
        }
    }
}

// this functions is here for the situations which you need a windows path with double backslash or one backslash on every level and you
// have only other.

// this is a little bit tricky because rust understands backslash as escape sequence, which means if you intend to write
// one backslash, you have to write two backslash instead, if you intend to write two, you have to write four and so on.

// warning, if you have a path that actually has 2 backslash, this function directly returns that path.
// if you want to make a path that has only one backslash and you have or can write a path with two backslash,
// use "windows_paths_one_backslash" function instead.

// sample use on your codebase: 

// let path = windows_paths_two_backslash("C:\\Users");

#[cfg(target_os = "windows")]
pub fn windows_paths_two_backslash(path: &str) -> String {
    if path.contains("\\") {
        let split_the_path: Vec<&str> = path.split("\\").collect();

        let length_of_the_split_the_path = split_the_path.len();
        let mut new_str = "".to_string();
    
        for (index, entity) in split_the_path.into_iter().enumerate() {
            if index == 0 {
                new_str.push_str(entity);
            }
    
            if index == length_of_the_split_the_path {
                break;   
            }
    
            if index != 0 {
                new_str.push_str("\\\\");
                new_str.push_str(entity);
            }
        }
    
        return new_str;
    } else {
        return path.to_string()
    }

}

// if you have a path that has two backslash but you want a path with one backslash, use this instead:

// sample use on codebase, if you type that path directly: 

// let path = windows_paths_one_backslash("C:\\\\Users");

#[cfg(target_os = "windows")]
pub fn windows_paths_one_backslash(path: &str) -> String {
    if path.contains("\\\\") {
        let split_the_path: Vec<&str> = path.split("\\\\").collect();

        let length_of_the_split_the_path = split_the_path.len();
        let mut new_str = "".to_string();
    
        for (index, entity) in split_the_path.into_iter().enumerate() {
            if index == 0 {
                new_str.push_str(entity);
            }
    
            if index == length_of_the_split_the_path {
                break;   
            }
    
            if index != 0 {
                new_str.push_str("\\");
                new_str.push_str(entity);
            }
        }
    
        return new_str;
    } else {
        return path.to_string()
    }

}

#[cfg(target_os = "windows")]
pub fn get_current_user() -> String {
    let current_user_command = Command::new("cmd")
                                                        .arg("/C")
                                                        .arg("echo")
                                                        .arg("%username%")
                                                        .output();

    match current_user_command {
        Ok(user) => {
            from_utf8(&user.stdout).unwrap().trim().to_string()
        },
        Err(error) => {
            eprintln!("This error occured on get_current_user() function: {}", error);

            "".to_string()
        }
    }
}

#[cfg(target_os = "windows")]
pub fn is_directory(path: &str) -> bool {
    let _turn_terminal_into_utf8 = Command::new("cmd")
                                                    .args(&["/C", "chcp 65001"])
                                                    .output()
                                                    .expect("cannot convert terminal's character encoding to utf8");

    let get_files_command = Command::new("powershell")
                                                        .args(&["Get-Item", "-Path", path, "|", "Select-Object", "Mode, @{Name='Owner'; Expression={(Get-Acl $_.FullName).Owner}}, LastWriteTime, Name, CreationTime, Attributes, LastAccessTime, Length, FullName"])
                                                        .output();

    match get_files_command {
        Ok(answer) => {
            let decode_output = from_utf8(&answer.stdout).unwrap();
            let mut result = false;

            for line in decode_output.lines().into_iter() {
                if line.starts_with("Mode") {
                    let split_the_line: &str = line.split(" :").collect::<Vec<&str>>()[1];

                    let check_the_permissions = check_entity_permissions(split_the_line.trim());

                    if check_the_permissions.entity_types.contains(&"directory") {
                        result = true
                    } else {
                        result = false
                    }
                } else {
                    continue;
                }
            }

            return result
        },
        Err(error) => {
            println!("This error occured: {}", error);

            return false
        }
    }
}

#[cfg(target_os = "windows")]
pub fn is_archive(path: &str) -> bool {
    let _turn_terminal_into_utf8 = Command::new("cmd")
                                                    .args(&["/C", "chcp 65001"])
                                                    .output()
                                                    .expect("cannot convert terminal's character encoding to utf8");

    let get_files_command = Command::new("powershell")
                                                        .args(&["Get-Item", "-Path", path, "|", "Select-Object", "Mode, @{Name='Owner'; Expression={(Get-Acl $_.FullName).Owner}}, LastWriteTime, Name, CreationTime, Attributes, LastAccessTime, Length, FullName"])
                                                        .output();

    match get_files_command {
        Ok(answer) => {
            let decode_output = from_utf8(&answer.stdout).unwrap();
            let mut result = false;

            for line in decode_output.lines().into_iter() {
                if line.starts_with("Mode") {
                    let split_the_line: &str = line.split(" :").collect::<Vec<&str>>()[1];

                    let check_the_permissions = check_entity_permissions(split_the_line.trim());

                    if check_the_permissions.entity_types.contains(&"archive") {
                        result = true
                    } else {
                        result = false
                    }
                } else {
                    continue;
                }
            }

            return result
        },
        Err(error) => {
            println!("This error occured: {}", error);

            return false
        }
    }
}

#[cfg(target_os = "windows")]
pub fn is_reparse_point_or_symlink(path: &str) -> bool {
    let _turn_terminal_into_utf8 = Command::new("cmd")
                                                    .args(&["/C", "chcp 65001"])
                                                    .output()
                                                    .expect("cannot convert terminal's character encoding to utf8");

    let get_files_command = Command::new("powershell")
                                                        .args(&["Get-Item", "-Path", path, "|", "Select-Object", "Mode, @{Name='Owner'; Expression={(Get-Acl $_.FullName).Owner}}, LastWriteTime, Name, CreationTime, Attributes, LastAccessTime, Length, FullName"])
                                                        .output();

    match get_files_command {
        Ok(answer) => {
            let decode_output = from_utf8(&answer.stdout).unwrap();
            let mut result = false;

            for line in decode_output.lines().into_iter() {
                if line.starts_with("Mode") {
                    let split_the_line: &str = line.split(" :").collect::<Vec<&str>>()[1];

                    let check_the_permissions = check_entity_permissions(split_the_line.trim());

                    if check_the_permissions.entity_types.contains(&"reparse-point-or-symlink") {
                        result = true
                    } else {
                        result = false
                    }
                } else {
                    continue;
                }
            }

            return result
        },
        Err(error) => {
            println!("This error occured: {}", error);

            return false
        }
    }
}

#[cfg(target_os = "windows")]
pub fn is_directory_and_archive(path: &str) -> bool {
    let _turn_terminal_into_utf8 = Command::new("cmd")
                                                    .args(&["/C", "chcp 65001"])
                                                    .output()
                                                    .expect("cannot convert terminal's character encoding to utf8");

    let get_files_command = Command::new("powershell")
                                                        .args(&["Get-Item", "-Path", path, "|", "Select-Object", "Mode, @{Name='Owner'; Expression={(Get-Acl $_.FullName).Owner}}, LastWriteTime, Name, CreationTime, Attributes, LastAccessTime, Length, FullName"])
                                                        .output();

    match get_files_command {
        Ok(answer) => {
            let decode_output = from_utf8(&answer.stdout).unwrap();
            let mut result_num: i8 = 0;

            for line in decode_output.lines().into_iter() {
                if line.starts_with("Mode") {
                    let split_the_line: &str = line.split(" :").collect::<Vec<&str>>()[1];

                    let check_the_permissions = check_entity_permissions(split_the_line.trim());

                    if check_the_permissions.entity_types.contains(&"archive") {
                        result_num = result_num + 1;
                    }

                    if check_the_permissions.entity_types.contains(&"directory") {
                        result_num = result_num + 1;
                    }
                } else {
                    continue;
                }
            }

            return match result_num {
                1 => false,
                2 => true,
                _ => false
            }
        },
        Err(error) => {
            println!("This error occured: {}", error);

            return false
        }
    }
}

#[cfg(target_os = "windows")]
pub fn is_directory_and_reparse_point_or_symlink(path: &str) -> bool {
    let _turn_terminal_into_utf8 = Command::new("cmd")
                                                    .args(&["/C", "chcp 65001"])
                                                    .output()
                                                    .expect("cannot convert terminal's character encoding to utf8");

    let get_files_command = Command::new("powershell")
                                                        .args(&["Get-Item", "-Path", path, "|", "Select-Object", "Mode, @{Name='Owner'; Expression={(Get-Acl $_.FullName).Owner}}, LastWriteTime, Name, CreationTime, Attributes, LastAccessTime, Length, FullName"])
                                                        .output();

    match get_files_command {
        Ok(answer) => {
            let decode_output = from_utf8(&answer.stdout).unwrap();
            let mut result_num: i8 = 0;

            for line in decode_output.lines().into_iter() {
                if line.starts_with("Mode") {
                    let split_the_line: &str = line.split(" :").collect::<Vec<&str>>()[1];

                    let check_the_permissions = check_entity_permissions(split_the_line.trim());

                    if check_the_permissions.entity_types.contains(&"directory") {
                        result_num = result_num + 1;
                    }

                    if check_the_permissions.entity_types.contains(&"reparse-point-or-symlink") {
                        result_num = result_num + 1;
                    }
                } else {
                    continue;
                }
            }

            return match result_num {
                1 => false,
                2 => true,
                _ => false
            }
        },
        Err(error) => {
            println!("This error occured: {}", error);

            return false
        }
    }
}

#[cfg(target_os = "windows")]
pub fn is_archive_and_reparse_point_or_symlink(path: &str) -> bool {
    let _turn_terminal_into_utf8 = Command::new("cmd")
                                                    .args(&["/C", "chcp 65001"])
                                                    .output()
                                                    .expect("cannot convert terminal's character encoding to utf8");

    let get_files_command = Command::new("powershell")
                                                        .args(&["Get-Item", "-Path", path, "|", "Select-Object", "Mode, @{Name='Owner'; Expression={(Get-Acl $_.FullName).Owner}}, LastWriteTime, Name, CreationTime, Attributes, LastAccessTime, Length, FullName"])
                                                        .output();

    match get_files_command {
        Ok(answer) => {
            let decode_output = from_utf8(&answer.stdout).unwrap();
            let mut result_num: i8 = 0;

            for line in decode_output.lines().into_iter() {
                if line.starts_with("Mode") {
                    let split_the_line: &str = line.split(" :").collect::<Vec<&str>>()[1];

                    let check_the_permissions = check_entity_permissions(split_the_line.trim());

                    if check_the_permissions.entity_types.contains(&"archive") {
                        result_num = result_num + 1;
                    }

                    if check_the_permissions.entity_types.contains(&"reparse-point-or-symlink") {
                        result_num = result_num + 1;
                    }
                } else {
                    continue;
                }
            }

            return match result_num {
                1 => false,
                2 => true,
                _ => false
            }
        },
        Err(error) => {
            println!("This error occured: {}", error);

            return false
        }
    }
}

#[cfg(target_os = "windows")]
pub fn is_directory_and_archive_and_reparse_point_or_symlink(path: &str) -> bool {
    let _turn_terminal_into_utf8 = Command::new("cmd")
                                                    .args(&["/C", "chcp 65001"])
                                                    .output()
                                                    .expect("cannot convert terminal's character encoding to utf8");

    let get_files_command = Command::new("powershell")
                                                        .args(&["Get-Item", "-Path", path, "|", "Select-Object", "Mode, @{Name='Owner'; Expression={(Get-Acl $_.FullName).Owner}}, LastWriteTime, Name, CreationTime, Attributes, LastAccessTime, Length, FullName"])
                                                        .output();

    match get_files_command {
        Ok(answer) => {
            let decode_output = from_utf8(&answer.stdout).unwrap();
            let mut result_num: i8 = 0;

            for line in decode_output.lines().into_iter() {
                if line.starts_with("Mode") {
                    let split_the_line: &str = line.split(" :").collect::<Vec<&str>>()[1];

                    let check_the_permissions = check_entity_permissions(split_the_line.trim());

                    if check_the_permissions.entity_types.contains(&"archive") {
                        result_num = result_num + 1;
                    }

                    if check_the_permissions.entity_types.contains(&"directory") {
                        result_num = result_num + 1;
                    }

                    if check_the_permissions.entity_types.contains(&"reparse-point-or-symlink") {
                        result_num = result_num + 1;
                    }
                } else {
                    continue;
                }
            }

            return match result_num {
                1 => false,
                2 => false,
                3 => true,
                _ => false
            }
        },
        Err(error) => {
            println!("This error occured: {}", error);

            return false
        }
    }
}

#[cfg(target_os = "windows")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_user(){
        println!("your current user: {}", get_current_user())
    }

    #[test]
    fn test_current_folder_info(){
        let time = std::time::Instant::now();
        println!("your current folder's entities: {:#?}", current_folder_info());
        let function_ended = time.elapsed();
        println!("current_folder_info() function running time as milliseconds: {}", function_ended.as_millis())
    }

    #[test]
    fn test_other_folder_info(){
        let current_user = get_current_user();

        let our_path = format!("C:\\Users\\{}\\Desktop", current_user);

        // getting all entities infos of your desktop:
        let time = std::time::Instant::now();
        println!("other folder's entities: {:#?}", other_folder_info(&our_path).unwrap());
        let function_ended = time.elapsed();
        println!("other_folder_info() function running time as milliseconds: {}", function_ended.as_millis())
    }

    #[test]
    fn test_entity_info() {
        let current_user = get_current_user();

        let our_path = format!("C:\\Users\\{}\\Desktop", current_user);

        // getting infos of your current user's desktop directory:

        let time = std::time::Instant::now();
        println!("your desktop folder: {:#?}", entity_info(&our_path).unwrap());
        let function_ended = time.elapsed();
        println!("entity_info() function running time as milliseconds: {}", function_ended.as_millis())
    }

    #[test]
    fn test_is_directory(){
        let current_user = get_current_user();

        let our_path = format!("C:\\Users\\{}\\Desktop", current_user);

        assert_eq!(is_directory(&our_path), true)
    }

    #[test]
    fn test_is_archive(){
        let current_user = get_current_user();

        let our_path = format!("C:\\Users\\{}\\.cargo\\bin\\cargo.exe", current_user);

        assert_eq!(is_archive(&our_path), true)
    }

    #[test]
    fn test_is_symlink_or_reparse_point(){
        let current_user = get_current_user();
        let format_the_path = format!("C:\\Users\\{}\\OneDrive", current_user);

        assert_eq!(true, is_reparse_point_or_symlink(&format_the_path))
    }

    #[test]
    fn test_is_directory_and_archive(){
        let current_user = get_current_user();
        let format_the_path = format!("C:\\Users\\{}\\OneDrive", current_user);

        assert_eq!(true, is_directory_and_archive(&format_the_path))
    }

    #[test]
    fn test_is_directory_and_reparse_point_or_symlink(){
        let current_user = get_current_user();
        let format_the_path = format!("C:\\Users\\{}\\OneDrive", current_user);

        assert_eq!(true, is_directory_and_reparse_point_or_symlink(&format_the_path))
    }

    #[test]
    fn test_is_archive_and_reparse_point_or_symlink(){
        let current_user = get_current_user();
        let format_the_path = format!("C:\\Users\\{}\\OneDrive", current_user);

        assert_eq!(true, is_archive_and_reparse_point_or_symlink(&format_the_path))
    }

    #[test]
    fn test_is_directory_and_archive_and_reparse_point_or_symlink(){
        let current_user = get_current_user();
        let format_the_path = format!("C:\\Users\\{}\\OneDrive", current_user);

        assert_eq!(true, is_directory_and_archive_and_reparse_point_or_symlink(&format_the_path))
    }
}