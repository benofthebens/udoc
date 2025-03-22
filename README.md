# udoc
This project aims to apply software engineering practices and workflows and git, in addition to practice rust skills. This project aims to be an easy (and naive) way to create portable documentation for errors in systems.
## Table of Contents
- [Table of Contents](#table-of-contents)
- [Requirements](#requirements)
  - [Dependencies](#dependencies)
- [Usage Instructions](#usage-instructions)
  - [Commands](#commands)
    - [New](#new)
    - [Update](#update)
    - [Config](#config)
    - [Reset](#reset)
    - [Export](#export)
- [Project Roadmap](#project-roadmap)
## Requirements
- Cargo 1.83.0+ 
### Dependencies
- Clap 4.5.23
- Serde 1.0
- Serde Json 1.0
- Serde XML rs 0.6
- Chrono 0.4.39
- quick xml 
## Usage Instructions
build:
```powershell
cargo build
```
The udoc.exe file will be in the target/ directory.
### Commands

---
#### New

```powershell
$name = "example-error"
$description = "Example"

./target/debug/udoc.exe new -n $name -d $description
```
This command will create a repository with the format of: name-yyyy-mm-dd

With the subdirectories of: images, videos and .udoc and a file will bre created called log.md this is the main way to edit 
in the .udoc directory an exchange directory is created with a representation of log.md in a xml format called exchange.xml

The config is stored in the .udoc directory with the config.json file.
#### Update
Ensure that you are in the correct directory (inside the file you have created with the new command).
```powershell
$udoc_repo_name = "example-error-2025-03-22";
Set-Location $udoc_repo_name
```
To update the changes you have made to either the exchange.xml or log.md file execute:
```powershell
udoc.exe update
```
If you have not configured an email and name in the config.json this will not execute correctly please refer to [config](#config) commands.

Warning this will check the most recent edit if the most recent edit is on the exchange.xml file this will override the log.md file 

To prevent this from happening often please only change a single file.

#### Config
Ensure that you are in the correct directory (inside the file you have created with the new command).
```powershell
$udoc_repo_name = "example-error-2025-03-22";
Set-Location $udoc_repo_name
```
To update the config.json file from the cli 
```powershell
# Name
udoc config $config_property -n $config_value
# Email
udoc config $config_property -e $config_value
```
Note: __commands are set to change as the flags are redundant__
#### Reset
To reset the repository: 
```powershell
udoc reset
```
Warning this will reset to defaults.
#### Export
Currently, there is only one option to export to html.

__Note: This is set to change__

```powershell
udoc export
```

This will try and read a file called template.html (An example of this can be found on the git repo).

This can either be copied or you can create your own template.html however for the export to work:
- {{ title }}: is used to represent the title of the error.
- {{ sections }}: is used to represent the sections of the documentations. A section is defined as all text/images under a "###".

```powershell
udoc export
```
This command will create a html file called exchange.html, which will  represent the exchange.xml file Ensure you do an [update](#update) before you export to ensure all changed are commited.
## Scripts
## Project Roadmap
### Features
- Auto add images to exchange file when adding to the images directory
- Possible Diff algorithm and asking the user which version of the file you will want to keep rather than relying on time last edited time last edited.
- Change Config commands and update how the config works
- Global configuration (Default Path with exe and config etc)
### Refactor
- Refactor to have best practices
- Make some function instance methods
- Add functions to Utils modules
- Add more customisability.
### Tests 
- Better use of unit tests across modules.
- Integration tests using powershell.