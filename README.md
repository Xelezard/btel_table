# btel table

A Plugin that let's you easily create tables with the btel text editor.

## Installation
To install add this to your config.tr:
```
commands -> alt text
| t or table -> /path/to/the/executable
```
Note: Instead of using "t or table" you can use any other constellation of commands e.g. "_ or _____ or __"

## Usage
Note: the table plugin only works with empty files

To create a new table run:
```
table init "row" "row2" "row3"
```
**Row names must be in quotes!**

To add date to your table run:
```
table update "field" "field2" "field3" 
```
**It's important that you use quotes and cover every row!**

Currently the table is a mess to remove quotes and format everything use:
```
table finalize
```
**From now on the table will not recognise your table, so only do this when you are finished!**
