# idats.rs 

This code to reads individual pairs of green and red intensities from (*.idat) files. The intensities on a 2D vector and a k-means algoithm to determine clusters.
The output is displayed on a scatter plot.  

### Basic use 

A user is allowed to enter a path where the IDAT data is stored. If the all files on the given path are of (*.idat) format, the user should wait for the data to 
be extraction, stored, and clustering. After all the data is clustered, the program will begin to plot until a (*.bmp) image is created depicting the output.


### Requirements 

plotters = "0.3.4", ndarray = "0.15.6", linfa-clustering = "0.6.0", ndarray = "0.15.6", and rand = "0.8.5".


