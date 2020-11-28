# rss_gen

commandline rss feed generator

reads in html or php spits out a rss entry in a file you supply or one created with a config file.


TODO: 
1 read in text from file.✓
2 parse data til the h tag is reached. ✓
3 stop parsing when the next h tag is found.✓
4 create a post object that will store the post data before outputing it to the xml file. ✓
5 refactor output for multiple item objects so that you can feed in a directory and it will create a list to plug into the xml rss.
6 attach post struct data with appropriate tags 
7 insert data into existing xml file. 
8 parse the body including pictures and save pictures file paths in data.

## modules
### input
handles parsing data from html files and later other file formats.
### config
config is the data objects for the configuration as well as parsing config files to create those objects.
### output
writing data to the output file as well as changing the config file through the binary eg \$ rss_gen --site_name: barfoo etc

### main parses args and delegates tasks based on the args provided.

### 6
steps: 
	read in file. 
	find the description tag. 
	format the inputed text. 
	write to the next line.
