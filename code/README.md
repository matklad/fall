
# Visual Studio Code extension for the fall parser generator

This extensions uses Rust code (`native` directory), and so is a bit tricky to build and package. To build, check
that `.npmrc` uses the same settings as the target Code version. Then build extension with `npm install && vsce package`.

Install the resulting `.vsix` file into Code using **Install from VSIX** action. Beware, it might eat your laundry!
