# rss_gen

commandline rss feed generator

reads in html or php spits out a rss entry in a file you supply or one created with a config file.


TODO: 
* read in text from file.✓
* parse data til the h tag is reached. ✓
* stop parsing when the next h tag is found.✓
* create a post object that will store the post data before outputing it to the xml file. ✓
* attach post struct data with appropriate tags 
* insert data into existing xml file. 
* parse the body including pictures and save pictures file paths in data.

## modules
### input
handles parsing data from html files and later other file formats.
### config
config is the data objects for the configuration as well as parsing config files to create those objects.
### output
writing data to the output file as well as changing the config file through the binary eg \$ rss_gen --site_name: barfoo etc

### main parses args and delegates tasks based on the args provided.
