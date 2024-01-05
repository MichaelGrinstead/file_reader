use::std::io;
use::std::path::PathBuf;
use::std::fs;

mod utils;


fn main() -> io::Result<()>{
    println!("Input Directory:");
    
    let mut inputted_path = String::new();
    io::stdin().read_line(&mut inputted_path)?;

    println!("You have entered path: {:?}", inputted_path.trim());

    let path = PathBuf::from(inputted_path.trim());


    let entries = utils::read_dir_contents(&path)?;

    let extensions = utils::list_all_extensions(&entries)?;
    let extensions_clone = extensions.clone();

    for extension in extensions {
        let extension_dir = path.join(extension);
        utils::create_dir(&extension_dir)?;
    }


    for entry in &entries {
        let entry_path = entry.path();
        let file_extension = entry_path.extension();

        if let Some(extension) = &file_extension {
            let extension = extension.to_os_string();
            
            // Check if a folder with the same extension exists
            if extensions_clone.contains(&extension) {
                let extension_str = extension.to_string_lossy().to_string();
                let destination_dir = path.join(extension_str);

                println!("Moving {:?} to {:?}", entry.file_name(), destination_dir);

                // Create the full destination path
                let destination_path = destination_dir.join(entry.file_name());

                // Move the file to the destination folder
                fs::rename(&entry.path(), &destination_path)?;
            }
        }
    }


    Ok(())
}
