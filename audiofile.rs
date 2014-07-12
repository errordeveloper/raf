extern crate libc;
extern crate std;

#[allow(dead_code,
        unused_attribute,
        uppercase_variables,
        non_camel_case_types,
        non_snake_case_functions)]

mod afapi;

fn open(name: &str, mode: &str) -> afapi::AFfilehandle {
  name.with_c_str(|name|
    mode.with_c_str(|mode|
      unsafe {
        afapi::afOpenFile(name, mode, 0 as afapi::AFfilesetup)
      }
  ))
}

fn close(file: afapi::AFfilehandle) -> i32 {
  unsafe {
    afapi::afCloseFile(file) as i32
  }
}

fn get_format(file: afapi::AFfilehandle) -> i32 {
  unsafe {
    afapi::afGetFileFormat(file, 0 as *mut i32) as i32
  }
}

fn with_readonly(path: &str, block: |handle: afapi::AFfilehandle|) {

  let file = open(path, "r");

  block(file);

  close(file);
}

#[cfg(test)]
mod test {

  static path: [&'static str, ..1] = [
    "imports/audiofile-test-media/media/dpwelib/testR22C1Fs.au",
  ];

  #[test]
  fn open_and_get_basic_info() {
    ::with_readonly(path[0], |file| {
      assert!(3 == ::get_format(file));
    });
  }
}
