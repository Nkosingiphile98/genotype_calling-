
extern crate base64;
use std::io;
use std::io::Read;
use std::fs::File;
use std::io::BufReader;

// Import the linfa prelude and KMeans algorithm
use linfa::prelude::*;
use linfa_clustering::KMeans;
// We'll build our dataset on our own using ndarray and rand
use ndarray::prelude::*;
//use ndarray::stack;
use rand::prelude::*;
// Import the plotters crate to create the scatter plot
use plotters::prelude::*;
use ndarray::arr2;

fn main() ->io::Result<()> {


    let mut input = String::new();

    let mut loop_counter=0;

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
        .build_cartesian_2d(0.0..2147483647.0, 0..2147483647)
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
    two_d_vector.push(green_data); //        ___________
    two_d_vector.push(red_data);//        Col



    //let mut green_data_32bit=vec![];
    //let mut red_data_32bit=vec![];

    let mut all_data=vec![];

    for _x in 0..(vec_capacity.capacity()/4) {

        // access green and red intensities
        let  green_int: u32 = u32::from_be_bytes([two_d_vector[0][loop_counter], two_d_vector[0][loop_counter + 1], two_d_vector[0][loop_counter + 2], two_d_vector[0][loop_counter + 3]]);
        let  red_int: u32 = u32::from_be_bytes([two_d_vector[1][loop_counter], two_d_vector[1][loop_counter + 1], two_d_vector[1][loop_counter + 2], two_d_vector[1][loop_counter + 3]]);

            all_data.push([(red_int/100000) as f64,(green_int/100000) as f64]);

        println!("{}",green_int);


        loop_counter=loop_counter+4;

    }

    println!("Storing data completed");

    //Cluster Data

    let boxed = all_data.into_boxed_slice();
    let data = arr2(&boxed);
    println!("Clustering data...");
    let dataset = DatasetBase::from(data);
    let rng = thread_rng(); // Random number generator
    let n_clusters = 3;
    let model = KMeans::params_with_rng(n_clusters, rng)
        .max_n_iterations(2000)
        .tolerance(1e-9)
        .fit(&dataset)
        .expect("Error while fitting KMeans to the dataset");

    let dataset = model.predict(dataset);
    println!("{:?}", dataset.records.shape());
    println!("{:?}", dataset.targets.shape());


    let root = BitMapBackend::new("Clustered Data.bmp", (600, 400)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let x_lim = 0.0..55500.0f64;
    let y_lim = 0.0..55500.0f64;

    let mut ctx = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 40) // Put in some margins
        .set_label_area_size(LabelAreaPosition::Right, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("KMeans Demo", ("sans-serif", 25)) // Set a caption and font
        .build_cartesian_2d(x_lim, y_lim)
        .expect("Couldn't build our ChartBuilder");

    ctx.configure_mesh().draw().unwrap();
    let root_area = ctx.plotting_area();

    // check_array_for_plotting(&dataset.records);

    for i in 0..dataset.records.shape()[0] {
        let coordinates = dataset.records.slice(s![i, 0..2]);
        println!("plot..");
        let point = match dataset.targets[i] {
            0 => Circle::new(
                (coordinates[0], coordinates[1]),
                3,
                ShapeStyle::from(&RED).filled(),
            ),
            1 => Circle::new(
                (coordinates[0], coordinates[1]),
                3,
                ShapeStyle::from(&GREEN).filled(),
            ),

            2 => Circle::new(


                (coordinates[0], coordinates[1]),
                3,
                ShapeStyle::from(&BLUE).filled(),
            ),
// Making sure our pattern-matching is exhaustive
            _ => Circle::new(
                (coordinates[0], coordinates[1]),
                3,
                ShapeStyle::from(&BLACK).filled(),
            ),
        };
        println!("plotting...");
          root_area
           .draw(&point)
           .expect("An error occurred while drawing the point!");

    }


//End of clustering
    Ok(())
}





