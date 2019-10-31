# Usage instructions

[Download the correct release for your platform](https://github.com/louisdewar/tutorial_web/releases).

### What's included in the release?

- These instructions
- A compiled version of `tutorial_web`
- A folder named `static` with the default versions of the static files

## Running the binary

Open up a terminal and navigate to the folder where the binary named `tutorial_web` is.

Run `./tutorial_web --help`.

If you get an error on mac saying that it can't be checked for malware then navigate to the folder in finder, right click on the binary then manually click open. Once you do this a terminal window will pop up, you can safely dismiss this and go back to your original terminal and re-run `./tutorial_web --help`. You only have to do this once.

## Folder structure

You need to create a folder for all of the courses. It can be called anything you want. I'm going to refer to it as `COURSE_FOLDER`.

The structure for the course folder is `COURSE_FOLDER/LANGUAGE/COURSE_NAME.yml`.
For an example see the folder `courses` in the repo for this project.

## Starting the server

Run `./tutorial_web start-test-server --help` to see the options.
Essentially you need to specify the input files (`COURSE_FOLDER`) and the static files.
The static files are those files in the folder `static` that came with the project.

An example is: `./tutorial_web start-test-server -i COURSE_FOLDER -s static`

## Building the files

Once you are ready to deploy you can build all the static files.
Run `./tutorial_web build --help` to see the options.
The options are like the web-server except it also takes an output dir where all the files will be put.

`./tutorial_web build -i COURSE_FOLDER -s static -o OUTPUT_DIR`
