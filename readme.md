# Output

Result of output `webpack.config.js`  
Default file will located to `dev/backend`

```js
var webpack = require("webpack");

const mode = "";
const out = "";
const entry = {};

module.exports = {
  mode: mode,
  module: {
    rules: [
      {
        test: /\.ejs$/,
        use: [
          {
            loader: "ejs-webpack-loader",
            options: { htmlmin: true },
          },
        ],
      },
    ],
  },
  entry: entry,
  output: {
    path: out,
    filename: "[name].js",
  },
};
```

# Usage

```
cargo run -- new
cargo run -- set_mode
cargo run -- show_mode
cargo run -- show_output
cargo run -- list_entry
cargo run -- add_entry
cargo run -- del_entry
cargo run -- build
cargo run -- help
```

# Install

1. Build release this repository
   `cargo build --release`
2. Add `target/release` path environment variable
