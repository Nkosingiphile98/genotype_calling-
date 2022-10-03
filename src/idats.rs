use std::fs::File; 
use std::io::prelude::*;
use std::path::Path;

fn main() 
{

    let  paths = vec!["/dataB/popdata/HapMap/h3a/H3Africa_HapMap_IDATs/201958790010",
    "/dataB/popdata/HapMap/h3a/H3Africa_HapMap_IDATs/201958790011",
"/dataB/popdata/HapMap/h3a/H3Africa_HapMap_IDATs/201958790012"];

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

    //Accessing all vector components/directories in the vector
    for x in 0..faxvec.capacity() {

        //Taking the name of the files from directories/paths..save it 
        //as string
        let file_name = faxvec[x].file_stem().unwrap().to_string_lossy();

        //Check if the name contains keyword "grn"
        if file_name.contains("2019"){

            //Open the file
            let mut file = File::open(faxvec[x].as_path()).expect("Cant open file");

            //Declare a variable to store file contents
            let mut contents = String::new();

            //Read the file and store content to contents
            file.read_to_string(&mut contents).expect("Cant read file");

            //Print contents
            println!("Content: {}",contents);

        }

    }

    }
}
