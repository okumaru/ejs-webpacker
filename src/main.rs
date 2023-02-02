
use std::fs::File;
use std::io::{Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Webpacker {
  webpack_file: String,
  webpack_path: String,
  mode: String,
  output: String,
  entrys: Vec<String>
}

impl Webpacker {
  pub fn new() -> Webpacker {

    let config_file = std::fs::OpenOptions::new()
      .write(true)
      .create(true)
      .read(true)
      .open("config.json")
      .expect("Failed to get config file");

    match serde_json::from_reader::<File, Webpacker>(config_file) {
        
      // File json decoded successfully
      Ok( config ) => config,
      
      // File json unsuccess to decoded
      Err(e) if e.is_eof() => Webpacker { 
        webpack_file: "webpack.config.js".to_string(),
        webpack_path: "dev\\backend\\".to_string(),
        mode: "development".to_string(),
        output: "html/assets/js/app".to_string(),
        entrys: Vec::new(),
      },
      
      // Other error
      _ => panic!("error while reading config json"),
    }
  }

  pub fn save(&self) -> std::io::Result<()> {
    let config_file = std::fs::OpenOptions::new()
      .write(true)
      .create(true)
      .read(true)
      .truncate(true)
      .open("config.json")
      .expect("Failed to save config file");

    serde_json::to_writer_pretty(config_file, self)
      .expect("Failed to write config file");

    Ok(())
  }

  pub fn add_entry(&mut self, item: &str ) -> std::io::Result<()> {
    self.entrys.push(item.to_string());
    self.save().expect("Failed to save entry");

    Ok(())
  }

  pub fn del_entry(&mut self, new_item: &str ) -> std::io::Result<()> {
    let mut new_entry: Vec<String> = Vec::new();
    for entry in &self.entrys {
      if entry != new_item {
        new_entry.push(entry.to_string());
      }
    }
    self.entrys = new_entry;
    self.save().expect("Failed to save entry");

    Ok(())
  }

  pub fn list_entries(&self) -> std::io::Result<()>  {
    for entry in &self.entrys {
      println!("- {}", entry.to_string());
    }

    Ok(())
  }

  pub fn show_output(&self) -> std::io::Result<()>  {
    let cur_dir = std::env::current_dir().expect("failed to get current directory");
    let full_path = &cur_dir.display().to_string().replace("\\", "/");
    let clear_dir = full_path.split(":").last().expect("failed to get clear path");
    println!("{}/{}", clear_dir, &self.output.to_string());

    Ok(())
  }

  // pub fn set_output(&mut self, _output: &str) -> std::io::Result<()> {
  //   println!("this function not ready!");

  //   Ok(())
  // }

  pub fn show_mode(&self) -> std::io::Result<()> {
    println!("{}", self.mode);

    Ok(())
  }

  pub fn set_mode(&mut self, mode: &str) -> std::io::Result<()> {
    self.mode = mode.to_string();
    self.save().expect("Failed to save entry");

    Ok(())
  }

  pub fn help(&self) -> std::io::Result<()> {
    println!("{: <15} - {}", "new", "Create config.js to save config");
    println!("{: <15} - {}", "set_mode", "Set mode that will saved at webpack.config.js");
    println!("{: <15} - {}", "show_mode", "Show mode that will saved at webpack.config.js");
    println!("{: <15} - {}", "show_output", "Show output result of compiled file by webpack by config on webpack.config.js");
    println!("{: <15} - {}", "list_entry", "Show file's that will compiled by webpack by config on webpack.config.js");
    println!("{: <15} - {}", "add_entry", "Add file's that will compiled by webpack by config on webpack.config.js");
    println!("{: <15} - {}", "del_entry", "Delete file's that will compiled by webpack by config on webpack.config.js");
    println!("{: <15} - {}", "build", "Build file webpack.config.js");
    println!("{: <15} - {}", "help", "Show this manual information");

    Ok(())
  }

  pub fn build(&mut self) -> std::io::Result<()> {

    let cur_dir = std::env::current_dir().expect("failed to get current directory");
    let full_path = &cur_dir.display().to_string().replace("\\", "/");
    let clear_dir = full_path.split(":").last().expect("failed to get clear path");
    let webpack_file = self.webpack_path.clone() + &self.webpack_file;
    let mut file = File::create(webpack_file)
      .expect("failed to create webpack file");

    file.write_all(b"var webpack = require(\"webpack\");\n\n")
      .expect("failed to write required webpack");

    file.write_all(format!("const mode = '{}';\nconst out = '{}/{}';\n", self.mode, clear_dir, self.output).as_bytes())
      .expect("failed to write mode webpack");

    file.write_all(b"const entry = {\n")
      .expect("failed to write output webpack");

    for entry in self.entrys.iter() {
      file.write_all(format!("  '{0}': __dirname + '/apps/{0}.js',\n", entry).as_bytes())
        .expect("failed to write item entry webpack");
    }
    
    file.write_all(b"}\n\n")
      .expect("failed to write output webpack");

    file.write_all(b"module.exports = {
      mode: mode,
      module: {
        rules: [{
          test: /\\.ejs$/,
          use: [{
            loader: 'ejs-webpack-loader',
            options: { htmlmin: true },
          }],
        }],
      },
      entry: entry,
      output: {
        path: out,
        filename: '[name].js',
      },
    };\n").expect("failed to write output webpack");

    Ok(())
  }
}

fn main() {
  let action: String = std::env::args().nth(1).expect("Please specify an action");
  let _item = std::env::args().nth(2);

  let mut webpacker = Webpacker::new();

  std::fs::create_dir_all(webpacker.webpack_path.clone())
    .expect("failed to create folder webpack");

  match action.as_str() {
    "new" => webpacker.save().expect("Create config failed!"),
    "set_mode" => webpacker.set_mode( &_item.expect("item cannot be empty").to_string() ).expect("Set mode failed!"),
    "show_mode" => webpacker.show_mode().expect("Show mode failed!"),
    // "setoutput" => webpacker.set_output( &_item.expect("item cannot be empty").to_string() ).expect("Set output failed!"),
    "show_output" => webpacker.show_output().expect("Show output failed!"),
    "list_entry" => webpacker.list_entries().expect("Show list entries failed!"),
    "add_entry" => webpacker.add_entry( &_item.expect("item cannot be empty").to_string() ).expect("Add entry failed!"),
    "del_entry" => webpacker.del_entry( &_item.expect("item cannot be empty").to_string() ).expect("Delete entry failed!"),
    "build" => webpacker.build().expect("Build failed!"),
    "help" => webpacker.help().expect("Failed show help information"),
    _ => println!("Does not found action!,\nplease use `help` to show list of actions command."),
  }
}