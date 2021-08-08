// Copyright 2019-present, the HuggingFace Inc. team, The Google AI Language Team and Facebook, Inc.
// Copyright 2019 Guillaume Becquin
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//     http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.extern crate anyhow;
use std::fs;
use std::env;

fn main()  {
    //get command line arguments
    let cmd_args: Vec<String> = env::args().collect();

    if cmd_args.len() != 3{
        println!("Requires two arguments.\nUSAGE: berttagr_file input.txt output.txt");
    }
    else {

        println!("In file {}", cmd_args[1]);
        println!("Out file {}", cmd_args[2]);

        let in_path = cmd_args[1].as_str();
        let out_path = cmd_args[2].as_str();

        let contents = fs::read_to_string(in_path)
            .expect("Something went wrong reading the file");

        let result: String = rustlib::rusttagr::rust_tag_r(contents.as_str());

        //write to a file
        fs::write(out_path, result.as_str())
            .expect("Something went wrong reading the file");
    }
}