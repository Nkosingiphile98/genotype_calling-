extern crate base64;
use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

fn main() {
 //"/dataB/popdata/HapMap/h3a/H3Africa_HapMap_IDATs/201958790010"
   

    let mut input = String::new();


    //Declare dir_vec(vector) to store file directories
    let mut dir_vec: Vec<std::path::PathBuf> = Vec::new();

 //Declare dir_vec_counter to count directories pushed into dir_vec
    let mut dir_vec_counter=0;


 println!("Enter Path:");

            //Read input line and store it in 'input' variable on press enter
        match io::stdin().read_line(&mut input) {

            //When the input is not returning errors:
            Ok(_) => {


            },

            //If the input returns an error print the error
            Err(e) => println!("Something went wrong : {}", e)
        }


        //Loop for every file found in the paths vector
    for element in std::path::Path::new(input.as_str().trim()).read_dir().unwrap() { // .trim() to remove white space characters such as Enter/Tab/Space.

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

    //print all file directories
    println!("dir_vec capacity: {}",dir_vec.capacity());


    //Accessing all vector components/directories in the vector
    for x in 0..dir_vec_counter {

        read_idat(dir_vec[x].as_path()).ok();
    }

    fn read_idat(path: &Path) -> io::Result<()>{

        let f = File::open(path)?;
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();

       
  // Read file into vector.
        reader.read_to_end(&mut buffer)?;


        //For byte/value in vector
       for value in buffer {

          //convert byte to intergers
           let back_to_u8test: u8 = u8::from_be_bytes([value]);
           println!("{}", back_to_u8test);
        }



        Ok(())

    }
}