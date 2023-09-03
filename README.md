# mecab-rs [![Build Status](https://travis-ci.org/tsurai/mecab-rs.svg?branch=master)](https://travis-ci.org/tsurai/mecab-rs)

Safe Rust wrapper for [mecab](https://taku910.github.io/mecab/) a japanese language part-of-speech and morphological analyzer library.

## Usage

The wrapper is almost identical to the C++ interface of mecab with the addition of various iterator for comfortable data access. It is build with the latest version of mecab v0.996.

## Windows

Both Windows Rust versions (MSVC ABI and gcc toolchain) can just use the [prebuilt 32bit library](https://mecab.googlecode.com/svn/trunk/mecab/doc/index.html#download).
Using mecab-rs with the Windows commandline is not recommended and can cause [undefined behavior](https://github.com/tsurai/mecab-rs/issues/3#issuecomment-182297235) if you are not using the correct codepage and a font that supports japanese character.

CMake & 64bit library:
* [CMake config for mecab](https://drive.google.com/file/d/0B7w3ZGc8CTgqRVo0Snp2ZzBTNkk/view?usp=sharing)
* [64bit MSVC library and binaries](https://drive.google.com/file/d/0B7w3ZGc8CTgqSmtrM2JCd3VXaVk/view?usp=sharing)
* [64bit GCC library and binaries](https://drive.google.com/file/d/0B7w3ZGc8CTgqUjJweENpa2dvcG8/view?usp=sharing)

Thanks to [@DoumanAsh](https://github.com/DoumanAsh) for providing these files

## Examples

Include this in your Cargo.toml to add mecab to your project:
```toml
[dependencies]
mecab = "*"
```
### Single-threaded environment

```Rust
extern crate mecab;

use mecab::Tagger;

fn main() {
  let input = "太郎は次郎が持っている本を花子に渡した。";
  println!("INPUT: {}", input);

  let mut tagger = Tagger::new("");

  // gets tagged result as String
  let mut result = tagger.parse_str(input);
  println!("RESULT: {}", result);

  // gets N best results as String
  result = tagger.parse_nbest(3, input);
  println!("NBEST:\n{}", result);

  // gets N best in sequence
  tagger.parse_nbest_init(input);
  for i in 0..3 {
    if let Some(res) = tagger.next() {
      println!("{}:\n{}", i, res);
    }
  }

  // gets Node object
  for node in tagger.parse_to_node(input).iter_next() {
    match node.stat as i32 {
      mecab::MECAB_BOS_NODE => {
        print!("{} BOS ", node.id);
      },
      mecab::MECAB_EOS_NODE => {
        print!("{} EOS ", node.id);
      },
      _ => {
        print!("{} {} ", node.id, &(node.surface)[..(node.length as usize)]);
      }
    }

    println!("{} {} {} {} {} {} {} {} {} {} {} {} {}",
      node.feature,
      input.len() as isize - node.surface.len() as isize,
      input.len() as isize - node.surface.len() as isize  + node.length as isize,
      node.rcattr,
      node.lcattr,
      node.posid,
      node.char_type,
      node.stat,
      node.isbest,
      node.alpha,
      node.beta,
      node.prob,
      node.cost);
  }

  // dictionary info
  for dict in tagger.dictionary_info().iter() {
    println!("\nfilename: {}", dict.filename);
    println!("charset: {}", dict.charset);
    println!("size: {}", dict.size);
    println!("type: {}", dict.dict_type);
    println!("lsize: {}", dict.lsize);
    println!("rsize: {}", dict.rsize);
    println!("version: {}", dict.version);
  }
}
```
## Multithreaded environment
See the [multithreaded example](examples/multithreaded.rs)

## License

The MIT License (MIT)

Copyright (c) 2015-2016 Cristian Kubis

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
