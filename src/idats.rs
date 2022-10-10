use std::io;
use std::fs::File;
use std::io::prelude::*;

fn main() 
{

   
let mut input = String::new();
let mut string = "";

//String to check the input(paths/command)
  
//Declare dir_vec(vector) to store file directories
let mut faxvec: Vec<std::path::PathBuf> = Vec::new();
let mut faxvec_counter=0;


//Declare paths(vector) to store input paths
let mut paths = vec![];

//Index for counting input paths
let mut index=1;

println!("Enter Paths. When done: INPUT '*'");

//Create infinite loop for inputting as many paths as you want
loop 
    {

        println!("{}Enter Path{}:",string,index);
    
//Read input line and store it in 'input' variable on press enter
        match io::stdin().read_line(&mut input) 
        {

//When the input is not returning errors:
            Ok(_) => 
            {
              
//Check if input contains '*' to break loop. The special
// character '*' was chosen because file/folder names cannot contain the character.
                if input.contains("*") 
                {


                    break;
                }
                else {
                    paths.push(input.trim().to_string());

        }

            },

//If the input returns an error print the error
            Err(e) => println!("Something went wrong : {}", e)
        }
//clear the input to avoid combined paths
        input.clear();
            index=index+1;
            string="Input '*' to End/"

    }


    println!("{:?} {}",paths,index-1);

//loop as many times as the paths-vector capacity
    for index in  0..index-1{
        
//Loop for every file found in the paths vector
// .trim() to remove white space characters such as Enter/Tab/Space.
    for element in std::path::Path::new(paths[index].as_str().trim()).read_dir().unwrap() 
    {

//save file directory to path variable
        let path = element.unwrap().path();

//Take the extension of every file.
        if let Some(extension) = path.extension() {

//Get and Change file extension to lower case because 
//sometimes the extension is in upper case
            if extension.to_ascii_lowercase() == "idat" {
//Push file directory of all files of idat format
                faxvec.push(path);
                faxvec_counter=faxvec_counter+1;//count all entries

            }
        }
    }
}

//print all file directories
    println!("Faxvec capacity: {}",faxvec.capacity());

    //Accessing all vector components/directories in the vector
    for x in 0..faxvec_counter {

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