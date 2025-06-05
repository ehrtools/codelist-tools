# R Bindings

This package demonstrates how to call Rust functions from R using extendr.

## Prerequisites

- Install R (version 4.0 or higher recommended)
  - macOS: `brew install r`

#### Notes

- My R installation took a long time to complete, so be patient here. It seems
  like it was broken because for some reason installing R also means it installs
  the latest version of Python, Pytorch and a bunch of other things. I'm not
  super familiar with R or why it does this but 🤷‍♀️ it might just be a me
  problem.
- I think you need to have Xcode installed on macOS to get the C bridge to work.
- I had to add R to my path before I could get into the R console via the
  terminal:

```bash
R
```

- An alternative is to use RStudio, which is a popular IDE for R.
- You can exit the R console with `q()`. You then get this:

```bash
Save workspace image? [y/n/c]:
```

which i said no to.

## Building and Installing

1. First go into R folder and build the package.

```bash
cd bindings/r
```

You will need to go into R at this point and install required packages of
`rextendr`. If you get an error message when running this, you may also need to
install package dependencies for `devtools` first if not already installed:

```r
install.packages("devtools")
install.packages("rextendr")
```

I picked `64` as the mirror I wanted to use.

3. Build and load the package:

```r
rextendr::document()
```

4. Test that it works:

```r
hello()
```

Everytime you make changes to the Rust code, you will need to run the following
commands in R:

```r
rextendr::document()
library(codelist)
```

The `library(codelist)` command loads the package, so you can use the functions
and structs within in so it is important to run this after you have made changes
to the Rust code.

#### Tips for non-R users

- You can create R scripts by making a file that ends with `.R` and then running
  it in R. It is probably easier at this point to opne in RStudio and run in the
  console or by clicking at the script. Remember to save.
- Exit the R console with `q()` and then type `n` to not save the workspace
  image.
