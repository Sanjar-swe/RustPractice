Build a CLI tool that manages Markdown files as a personal To-do list. 
The main goal of this tool is to allow you to view your todo list and the status of each todo. 
Yeah can use it as your project tracking of what is next and what is done.

The tool has an init subcommand to create a journal directory where the files are saved on it. 
The whole state is on the file itself. 
For example if the file name ends with . completed.md that means this todo is completed. 
The CLI allows for listing todos, toggle the completion of each one. 
And allows you to edit each todo in your favorite editor.