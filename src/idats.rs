use std::io;
use std::fs::File;
use std::io::prelude::*;


fn main() {


    let mut input = String::new();

//Declare dir_vec(vector) to store file directories
    let mut faxvec: Vec<std::path::PathBuf> = Vec::new();

    println!("Enter Path:");


//Read input line and store it in 'input' variable on press enter
        match io::stdin().read_line(&mut input) {

            
            Ok(_) => {
              


                 println!("Path: {}",input) ;

            },
//If the input returns an error print the error
            Err(e) => println!("Something went wrong : {}", e)
        }


    
    for element in std::path::Path::new(input.as_str().trim()).read_dir().unwrap() {

        //save file directory to path variable
        let path = element.unwrap().path();

        //Take the extension of every file.
        if let Some(extension) = path.extension() {

            //Get and Change file extension to lower case because 
            //sometimes the extension is in upper case
            if extension.to_ascii_lowercase() == "idat" {
                
                //Push file directory of all files of idat format
                faxvec.push(path);
               

            }
        }
    
}

    //Accessing all vector components/directories in the vector
    for x in 0..faxvec.capacity() {

        //Taking the name of the files from directories/paths..save it as string
        let file_name = faxvec[x].file_stem().unwrap().to_string_lossy();

        //Check if the name contains keyword "2019"
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
