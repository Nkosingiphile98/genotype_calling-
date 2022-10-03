use std::fs::File; 
use std::io::prelude::*;
use std::path::Path;

fn main() 
{

    let  paths = vec!["/dataB/popdata/HapMap/h3a/H3Africa_HapMap_IDATs/201958790010",
    "/dataB/popdata/HapMap/h3a/H3Africa_HapMap_IDATs/201958790011"];

    for index in 0..paths.capacity()
    {

        let mut faxvec: Vec<std::path::PathBuf> = Vec::new();

    for element in Path::new(paths[index]).read_dir().unwrap()
     {
        let path = element.unwrap().path();
        if let Some(extension) = path.extension() 
        {
            if extension == "idat"
             {

                //Push file directory of all files of idat format
                faxvec.push(path);
            

            }
        }
    }

   
    }
}
