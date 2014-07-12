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

fn get_track_ids(file: afapi::AFfilehandle) -> i32 {
  let track_ids: ::libc::c_int = 0;
  let track_ids_ptr = track_ids as *mut ::libc::c_int;
  unsafe {
    let err = afapi::afGetTrackIDs(file, track_ids_ptr);
  }
  return track_ids;
}

fn get_sample_format(file: afapi::AFfilehandle) -> (i32, i32) {

  let (format, width):
    (::libc::c_int, ::libc::c_int) = (0, 0);
  let (format_ptr, width_ptr) =
    (format as *mut ::libc::c_int, width as *mut ::libc::c_int);

  unsafe {
    let err = afapi::afGetSampleFormat(file,
        afapi::AF_DEFAULT_TRACK as ::libc::c_int, format_ptr, width_ptr);
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

  close(file);
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
              == 0);
      assert!(::get_channels(file)
              == 1);
      //let (format, width) = ::get_sample_format(file);
      ////16-bit integer (2's complement, big endian)
      //fail!("format={} width={}", format, width);
      //assert!(format
      //        == ::afapi::AF_SAMPFMT_TWOSCOMP);
      //assert!(width
      //        == ::afapi::AF_BYTEORDER_BIGENDIAN);
    });
    ::with_readonly(path[1], |file| {
      assert!(::get_channels(file) == 2);
      //let (format, width) = ::get_sample_format(file);
      //fail!("format={} width={}", format, width);
    });
  }
}
