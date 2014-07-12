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
    afapi::afGetFileFormat(file, &mut 0) as i32
  }
}

fn get_track_ids(file: afapi::AFfilehandle) -> i32 {
  let mut track_ids = 0;
  unsafe {
    let err = afapi::afGetTrackIDs(file, &mut track_ids);
  }
  return track_ids;
}

fn get_sample_format(file: afapi::AFfilehandle) -> (i32, i32) {
  let (mut format, mut width) = (0, 0);
  unsafe {
    let err = afapi::afGetSampleFormat(file,
        afapi::AF_DEFAULT_TRACK as ::libc::c_int, &mut format, &mut width);
  }
  return (format, width);
}

fn get_channels(file: afapi::AFfilehandle) -> i32 {
  unsafe {
    afapi::afGetChannels(file, ::afapi::AF_DEFAULT_TRACK) as i32
  }
}

fn with_readonly(path: &str, block: |handle: afapi::AFfilehandle|) {

  let file = open(path, "r");

  block(file);

  close(file); // TODO: make sure this closes it on failure too
}

#[cfg(test)]
mod test {

  static path: [&'static str, ..2] = [
    "imports/audiofile-test-media/media/dpwelib/testR22C1Fs.au",
    "imports/audiofile-test-media/media/dobson/stereofl.aifc",
  ];

  #[test]
  fn open_and_get_basic_info() {
    ::with_readonly(path[0], |file| {
      assert!(::get_format(file)
              == ::afapi::AF_FILE_NEXTSND);
      assert!(::get_track_ids(file)
              != 0);
      assert!(::get_channels(file)
              == 1);

      let (format, width) = ::get_sample_format(file);
      // TODO:: replace this with pattern matching
      assert!(format
              == ::afapi::AF_SAMPFMT_TWOSCOMP);
      assert!(width
              == 16);
    });
    ::with_readonly(path[1], |file| {
      assert!(::get_format(file)
              == ::afapi::AF_FILE_AIFFC);
      assert!(::get_channels(file)
              == 2);
      assert!(::get_track_ids(file)
              != 0);

      let (format, width) = ::get_sample_format(file);
      // TODO:: replace this with pattern matching
      assert!(format
              == ::afapi::AF_SAMPFMT_FLOAT);
      assert!(width
              == 32);
    });
  }
}
