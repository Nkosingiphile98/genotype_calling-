extern crate base64;
use std::io;
use std::io::Read;
use std::fs::File;
use std::io::BufReader;
//use std::io::prelude::*;
//use std::path;
use libm::atan;
use plotters::prelude::*;

fn main() ->io::Result<()> {

    //Create method to calculate arc tangent of red and green intensity ratio
    /*pub mod math {
        //use super::*;
        extern "C" {

            pub fn atan(x: f64) -> f64;


        }}*/
    pub const PI: f64 = 3.14159265358979323846264338327950288f64;






    //"/dataB/popdata/HapMap/h3a/H3Africa_HapMap_IDATs/201958790010"
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

    //Sort the array with directories so that you access the Grn and Red pairs accordingly
    dir_vec.sort();

    //Read Grn file into vector(buffer1)
    let file1 = File::open(dir_vec[0].as_path())?;
    let mut reader = BufReader::new(file1);
    let mut  buffer1 = Vec::new();

    // Read file into vector.
    reader.read_to_end(&mut buffer1)?;
    //For byte/value in vector


    //Read Red file into vector(buffer2)
    let file2 = File::open(dir_vec[1].as_path())?;
    let mut reader = BufReader::new(file2);
    let mut  buffer2 = Vec::new();

    // Read file into vector.
    reader.read_to_end(&mut buffer2)?;



    //****************************************************//
    //Read Red/Green file into vector to use as a capacity checker
    let file = File::open(dir_vec[1].as_path())?;
    let mut reader = BufReader::new(file);
    let mut  buffer3 = Vec::new();
    reader.read_to_end(&mut buffer3)?;

    let vec_capacity = buffer3;
    //***************************************************//

    //*******************create output bmp image************************//
    let root_area = BitMapBackend::new("Cluster.bmp", (800, 800)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40.0)
        .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
        .set_label_area_size(LabelAreaPosition::Right, 40.0)
        .set_label_area_size(LabelAreaPosition::Top, 40.0)
        .caption("Scatter Plot", ("sans-serif", 40.0))
        .build_cartesian_2d(0.0..255.0, 0..255)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();
    //*****************************************************************//

    //Assign data buffers/vectors to right vector names
    let mut green_data =vec![];
    let mut red_data =vec![];

    let  filename1 = dir_vec[0].as_path().file_stem().unwrap().to_ascii_lowercase();
    let  filename2 = dir_vec[1].as_path().file_stem().unwrap().to_ascii_lowercase();

    if filename1.to_string_lossy().contains("grn") {

        green_data = buffer1;

    }
    else {

        println!("Error assigning buffer names");

    }

    if filename2.to_string_lossy().contains("red"){

        red_data = buffer2;

    }
    else {

        println!("Error assigning buffer names");

    }


    //Create 2d vector
    let mut two_d_vector=vec![];
    //        Col
    two_d_vector.push(green_data); //        ___________
    two_d_vector.push(red_data);   //  Raw  |__|__|__|__|
    // Green |__|__|__|__|
    // Red   |__|__|__|__|




    for x in 0..vec_capacity.capacity(){

        // access green and red intensities
        let  green_int : u8 = u8::from_be_bytes([two_d_vector[0][x]]);
        let  red_int : u8 = u8::from_be_bytes([two_d_vector[1][x]]);

        let mut red=vec![];
        let mut green:Vec<i32>=vec![];

        if red_int !=0 {

            if atan((green_int/ red_int) as f64) !=0.0 {

                let polar_angle = 2.0/PI*atan((green_int /red_int) as f64);

                red.push(red_int.as_f64());
                green.push(green_int as i32);

                //[(red_int,green_int)]
                let vec3: Vec<(f64, i32)>= red.iter().cloned().zip(green.iter().cloned()).collect();

                println!("{:?}",vec3);


                red.clear();
                green.clear();

                if polar_angle<0.67 && polar_angle>0.33{

                    ctx.draw_series(
                        vec3.iter().map(|point| Circle::new(*point,  4.0_f64, &BLUE)),
                    ).unwrap();
                }

                if polar_angle>0.0 && polar_angle<0.34 {

                    ctx.draw_series(
                        vec3.iter().map(|point| Circle::new(*point,  4.0_f64, &RED)),
                    ).unwrap();
                }

                if polar_angle<=1.0 && polar_angle>0.67{

                    ctx.draw_series(
                        vec3.iter().map(|point| Circle::new(*point,  4.0_f64, &GREEN)),
                    ).unwrap();
                }

                // println!("{}",polar_angle);
            }

        }

    }

    Ok(())
}




