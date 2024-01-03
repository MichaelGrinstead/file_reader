use::std::io;

mod utils;


fn main() -> io::Result<()>{
    let path = utils::get_desktop_path();
    let entries = utils::read_dir_contents(&path)?;
    let full_path = path.join("Example");
    utils::create_dir(&full_path)?;


    for entry in &entries {
        println!("{:?}", entry.path());
        let created = utils::read_file_created_time(&entry)?;
        println!("{:?}", created);

        
    }
    let extensions = utils::list_all_extensions(&entries)?;
    println!("{:?}", extensions);

    Ok(())
}
