reveal makes the following assumptions:
 - `reveal` will be run from the `dist` directory which contains the exam code and a `admin` directory
 - the sticky bit on `dist` and `admin` is set, and `hints` (and all files inside it) is owned by, and only readable/writable by `root`
 
# Setting up

To set up for a new exam environment:
 1. Change the appropriate `TODO`s in `src/main.rs` (notably, the password that's used for the zip archive, the relevant files to zip, the list of hint file names).
 2. install `reveal` on the test machines, making sure that it runs as a privileged user.
 3. Set up the exam directory with the directory structure
 ```
 dist/
  - exam/                # all exam code
  - admin/               # privileged directory with sticky bit set
  - - hints/             # privileged directory containing hint files
  - - hint.record        # privileged file with contents "0" * num_hints
  - - transactions.log   # privileged empty file
 ```
 4. Fill the `admin/hints` directory with the hint files. The name of each hint file should be the same as the same number of the exam question it corresponds to and should contain the text of the hint.
