# R Bindings

This package demonstrates how to call Rust functions from R using extendr. I
(@CarolineMorton) have not yet implemented this with our Codelist library but
rather with a simple "hello world" function.

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

1. First build the Rust library:

```bash
cd r
cargo build --release
```

This will create a shared library in `target/release/libcodelist.dylib` (macOS)
or `target/release/libcodelist.so` (Linux).

2. Open R with the terminal command `R` and install required packages. If you
   get an error message when running this, you may also need to install package
   dependencies for `devtools` first if not already installed:

```r
install.packages("devtools")
install.packages("rextendr")
```

I picked `64` as the mirror I wanted to use.

3. Build and load the package:

```r
rextendr::document()
devtools::document()
devtools::load_all()
```

4. Test that it works:

```r
hello()
```

## Installing Permanently

To install the package permanently:

```r
devtools::build()
devtools::install()
```

After installation, you can use it like any other R package:

```r
library(codelist)
hello()
```
