# About

This is a service written in rust which goes to various news websites (of the user's choice) and finds all of the articles which relate to certain key search terms.

You might say to yourself, "Why not use a search engine?" and the answer to that is: does your search engine mass find articles that contain certain key search terms, log and export them into a .html file for easy readability?

## Functionality / Features

The functionality of this bot is as follows:

- uses a webscraping library to look at all of the article headlines
- for each article of interest (headlines that contain key terms) it will
	- search the body for more terms
	- order them by the unique occurences of terms (most to least)
- create a database of these articles
- functionality to export the table/DB to:
	- an HTML file
	- more?
- maybe throw in an webserver somewhere for the ability to host the HTML file

## Layout

- each website will be its own module inherited from a parent Website class of some description
	- each module needs to record the following data
		- number of terms found
		- number of unique terms found
		- date published
		- date accessed
		- URL of article
		- URL of website
	- each module needs to have these properties
		- url of website & article
		- state
			- connection e.g., connecting, failed, success, etc.
		- search terms
		- website specific HTML scraping patterns
		- search term scraping patterns
			- regular expression
			- text/words/etc
	- each module should either be defined in...
		- a specific file
		- a config file (hotpluggable?)
- each module will need a reference to some database object
	- the database module needs...
		- CRUD functions
		- an init function if the DB doesn't exist
- need to respect linux perms
	- create a bot user (newsgrep)
	- create a group (newsgrep)
- configuration file for...
	- where database is stored
		- /var/local/newsgrep (subject to change)
	- where to output logging, warning, error info, etc
	- other things in the future

## Roadmap

1. [x] Get HTTP requests working
1. [ ] Get scraping working
1. [ ] Get load websites as modules
1. [ ] Get a database working
1. [ ] Get export to HTML working

