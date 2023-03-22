use std::fs::{read_dir, ReadDir, self};

pub fn scan(directory: &str, show: bool, limit: u64){
    let files_result = read_dir(directory);

    let files: ReadDir = match files_result {
        Ok(file) => file,
        Err(error) => {
            get_file_size(directory, show, limit);
            return
        },
    };

    for file in files{
        let path = file.unwrap().path();
        
        if(show){
            println!("{:?}", path);
        }
        
        scan(&path.display().to_string(), show, limit)
    }
}

fn get_file_size(filename: &str, show: bool, limit: u64){
    let size_result = fs::metadata(filename);

    loop { match size_result {
            Ok(_) =>{
                break;
            }
            Err(error) => {
                return;
            }
        }
    }

    let size = size_result.unwrap().len();

    if(show){
        println!("{}kb : {}", size/1000, filename);
    }

    if(size > limit*1000){
        println!("{}kb : {}", size/1000, filename);
    }

}