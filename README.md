# idats.rs 

This code to reads individual pairs of green and red intensities from (*.idat) files. The intensities are stored on a 2D vector. A K-means machine learning algoithm 
partions the data points based on thier minimum distance to centroids. After genotype clusters are determined, the output is visualised on a scatter plot.  

### Basic use 

A user is allowed to enter a path where the IDAT intensity data is stored. If the all files on the given path are of (*.idat) format, the user waits for the data to 
be extracted, stored, and clustered. After clustering is done, the program will begin to plot until a (*.bmp) image will be created depicting the output.


### Requirements 

plotters = "0.3.4", ndarray = "0.15.6", linfa-clustering = "0.6.0", ndarray = "0.15.6", and rand = "0.8.5".


