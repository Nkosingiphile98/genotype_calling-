use std::ffi::OsStr;
use std::fs;
use std::io;
use std::fs::{DirEntry, File, read_dir};
use std::io::BufReader;
use std::io::prelude::*;
use std::ops::Add;
use std::path::{Path, PathBuf};


fn main() {

    

    let mut input = String::new();
    let mut string = "";

    //String to check the input(paths/command)
    let mut input_checker = String::new();

    //Declare dir_vec(vector) to store file directories
    let mut dir_vec: Vec<std::path::PathBuf> = Vec::new();

    //Declare dir_vec_counter to count directories pushed into dir_vec
    let mut dir_vec_counter=0;

    //Declare paths(vector) to store inputted paths
    let mut paths = vec![];

    //Index for counting inputted paths
    let mut index=1;

    println!("Enter Paths. When done: INPUT '*t'");


    //Create infinite loop for inputting as many paths as you want
        loop{

        println!("{}Enter Path{}:",string,index);

            //Read input line and store it in 'input' variable on press enter
        match io::stdin().read_line(&mut input) {

            //When the input is not returning errors:
            Ok(_) => {

                //Check if input contains '*' to break loop. The special character '*' was chosen because file/folder names cannot contain the character.
                if input.contains("*") {

                    break;
                }
                else {

                    //Else Push path into paths vector
                    paths.push(input.trim().to_string());

                }

            },

            //If the input returns an error print the error
            Err(e) => println!("Something went wrong : {}", e)
        }

            //clear the input to avoid combined paths
            input.clear();
            index=index+1;


            string="Input '*t' to End/"

    }





    //print paths and size of paths-vector
    println!("{:?} {}",paths,index-1);


    //loop as many times as the paths-vector capacity
    for index in  0..index-1{

        //Loop for every file found in the paths vector
    for element in std::path::Path::new(paths[index].as_str().trim()).read_dir().unwrap() { // .trim() to remove white space characters such as Enter/Tab/Space.

        let path = element.unwrap().path();  //save file directory to path variable

        if let Some(extension) = path.extension() { //Take the extension of every file.

            //Get and Change file extension to lower case because sometimes the extension is in upper case
            if extension.to_ascii_lowercase() == "idat" {         //Check if it is "IDAT".

                //Push file directory of all files of "IDAT" format into the dir_vec
                dir_vec.push(path);
                dir_vec_counter=dir_vec_counter+1; //count all entries

            }
        }
    }
}

    //print all file directories
    println!("dir_vec capacity: {}",dir_vec.capacity());

    //Accessing all vector components/directories in the vector
    for x in 0..dir_vec_counter {

       read_idat(dir_vec[x].as_path());
    }

    fn read_idat(path: &Path) -> io::Result<()>{

        let f = File::open(path)?;
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();

        // Read file into vector.
        reader.read_to_end(&mut buffer)?;

        // Read.
        for value in buffer {
            println!("BYTE: {:?}", value);
        }
        Ok(())

    }


}

