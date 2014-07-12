extern crate libc;
extern crate std;

use std::result::Result;

#[allow(dead_code,
        unused_attribute,
        uppercase_variables,
        non_camel_case_types,
        non_snake_case_functions)]

mod afapi;


fn do_open(name: &str, mode: &str) -> afapi::AFfilehandle {
  name.with_c_str(|name|
    mode.with_c_str(|mode|
      unsafe {
        afapi::afOpenFile(name, mode, 0 as afapi::AFfilesetup)
      }
  ))
}

fn do_close(file: afapi::AFfilehandle) -> i32 {
  unsafe {
    afapi::afCloseFile(file)
  }
}

fn get_format(file: afapi::AFfilehandle) -> i32 {
  unsafe {
    afapi::afGetFileFormat(file, &mut 0)
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
        afapi::AF_DEFAULT_TRACK, &mut format, &mut width);
  }
  return (format, width);
}

fn get_byte_order(file: afapi::AFfilehandle) -> i32 {
  unsafe {
    afapi::afGetByteOrder(file, afapi::AF_DEFAULT_TRACK)
  }
}

fn get_compression(file: afapi::AFfilehandle) -> i32 {
  unsafe {
    afapi::afGetCompression(file, afapi::AF_DEFAULT_TRACK)
  }
}

fn get_channels(file: afapi::AFfilehandle) -> i32 {
  unsafe {
    afapi::afGetChannels(file, ::afapi::AF_DEFAULT_TRACK)
  }
}

fn get_rate(file: afapi::AFfilehandle) -> f64 {
  unsafe {
    afapi::afGetRate(file, ::afapi::AF_DEFAULT_TRACK)
  }
}

fn get_frame_count(file: afapi::AFfilehandle) -> i64 {
  unsafe {
    afapi::afGetFrameCount(file, ::afapi::AF_DEFAULT_TRACK)
  }
}

fn with_readonly(path: &str, block: |handle: afapi::AFfilehandle|) {

  let file = do_open(path, "r");

  block(file);

  do_close(file); // TODO: make sure this closes it on failure too
}

pub struct AudioFile {
  handle: afapi::AFfilehandle,
}

impl AudioFile {
  pub fn open(path: &str, mode: &str) -> Result<AudioFile, ()> {
    let file = do_open(path, mode);
    //let null:afapi::AFfilehandle = 0 as afapi::AFfilehandle;
    match file {
      _  => Ok(AudioFile{ handle: file }),
    }
  }
  pub fn close(&self) -> Result<(), ()> {
    match do_close(self.handle) {
      0 => Ok(()),
      _ => Err(()),
    }
  }
}

#[cfg(test)]
mod test {

  // This is not everything but should be plenty
  static path: [&'static str, ..7] = [
    "imports/audiofile-test-media/media/dpwelib/testR22C1Fs.au",
      // Sun/NeXT audio data: 16-bit linear PCM, mono, 22050 Hz
    "imports/audiofile-test-media/media/dobson/stereofl.aifc",
      // IFF data, AIFF-C compressed audio
    "imports/audiofile-test-media/media/alac/sine24-4channel.caf",
      // CoreAudio Format audio file version 1
    "imports/audiofile-test-media/media/afsp/ircam/manna-mips-le-f.sf",
      // IRCAM file (MIPS little-endian)
    "imports/audiofile-test-media/media/afsp/next/M1F1-Alaw-AFsp.au",
      // Sun/NeXT audio data: 8-bit A-law (CCITT G.711), stereo, 8000 Hz
    "imports/audiofile-test-media/media/afsp/next/M1F1-float64-AFsp.au",
      // Sun/NeXT audio data: 64-bit IEEE floating point, stereo, 8000 Hz
    "imports/audiofile-test-media/media/regression/traveling.wav",
      // RIFF (little-endian) data, WAVE audio, Microsoft ADPCM, mono 22050 Hz
  ];

  #[test]
  fn open_and_get_basic_info_with_low_level_calls() {
    ::with_readonly(path[0], |file| {
      assert_eq!(::get_format(file), ::afapi::AF_FILE_NEXTSND);
      assert_eq!(::get_channels(file), 1);
      assert!(::get_track_ids(file) != 0);

      let (format, width) = ::get_sample_format(file);
      // TODO:: replace this with pattern matching
      assert_eq!(format, ::afapi::AF_SAMPFMT_TWOSCOMP);
      assert_eq!(width, 16);

      assert_eq!(::get_compression(file),
                 ::afapi::AF_COMPRESSION_NONE);
      assert_eq!(::get_byte_order(file),
                ::afapi::AF_BYTEORDER_BIGENDIAN);

      assert_eq!(::get_rate(file), 22050.0)
      assert_eq!(::get_frame_count(file), 22406);
    });

    ::with_readonly(path[1], |file| {
      assert_eq!(::get_format(file), ::afapi::AF_FILE_AIFFC);
      assert_eq!(::get_channels(file), 2);
      assert!(::get_track_ids(file) != 0);

      let (format, width) = ::get_sample_format(file);
      // TODO:: replace this with pattern matching
      assert_eq!(format, ::afapi::AF_SAMPFMT_FLOAT);
      assert_eq!(width, 32);

      assert_eq!(::get_compression(file),
              ::afapi::AF_COMPRESSION_NONE);
      assert_eq!(::get_byte_order(file),
                 ::afapi::AF_BYTEORDER_BIGENDIAN);

      assert_eq!(::get_rate(file), 22050.0)
      assert_eq!(::get_frame_count(file), 29016);
    });

    ::with_readonly(path[2], |file| {
      assert_eq!(::get_format(file), ::afapi::AF_FILE_CAF);
      assert_eq!(::get_channels(file), 4);
      assert!(::get_track_ids(file) != 0);

      let (format, width) = ::get_sample_format(file);
      // TODO:: replace this with pattern matching
      assert_eq!(format, ::afapi::AF_SAMPFMT_TWOSCOMP);
      assert_eq!(width, 24);

      assert_eq!(::get_compression(file),
              ::afapi::AF_COMPRESSION_ALAC);
      assert_eq!(::get_byte_order(file),
                 ::afapi::AF_BYTEORDER_LITTLEENDIAN);

      assert_eq!(::get_rate(file), 44100.0)
      assert_eq!(::get_frame_count(file), 220500);
    });

    ::with_readonly(path[3], |file| {
      assert_eq!(::get_format(file), ::afapi::AF_FILE_IRCAM);
      assert_eq!(::get_channels(file), 1);
      assert!(::get_track_ids(file) != 0);

      let (format, width) = ::get_sample_format(file);
      // TODO:: replace this with pattern matching
      assert_eq!(format, ::afapi::AF_SAMPFMT_FLOAT);
      assert_eq!(width, 32);

      assert_eq!(::get_compression(file),
              ::afapi::AF_COMPRESSION_NONE);
      assert_eq!(::get_byte_order(file),
                 ::afapi::AF_BYTEORDER_LITTLEENDIAN);

      assert_eq!(::get_rate(file), 44100.0)
      assert_eq!(::get_frame_count(file), 23501);
    });

    ::with_readonly(path[4], |file| {
      assert_eq!(::get_format(file), ::afapi::AF_FILE_NEXTSND);
      assert_eq!(::get_channels(file), 2);
      assert!(::get_track_ids(file) != 0);

      let (format, width) = ::get_sample_format(file);
      // TODO:: replace this with pattern matching
      assert_eq!(format, ::afapi::AF_SAMPFMT_TWOSCOMP);
      assert_eq!(width, 16);

      assert_eq!(::get_compression(file),
                 ::afapi::AF_COMPRESSION_G711_ALAW);
      assert_eq!(::get_byte_order(file),
                ::afapi::AF_BYTEORDER_LITTLEENDIAN);

      assert_eq!(::get_rate(file), 8000.0)
      assert_eq!(::get_frame_count(file), 23493);
    });

    ::with_readonly(path[5], |file| {
      assert_eq!(::get_format(file), ::afapi::AF_FILE_NEXTSND);
      assert_eq!(::get_channels(file), 2);
      assert!(::get_track_ids(file) != 0);

      let (format, width) = ::get_sample_format(file);
      // TODO:: replace this with pattern matching
      assert_eq!(format, ::afapi::AF_SAMPFMT_DOUBLE);
      assert_eq!(width, 64);

      assert_eq!(::get_compression(file),
                 ::afapi::AF_COMPRESSION_NONE);
      assert_eq!(::get_byte_order(file),
                ::afapi::AF_BYTEORDER_BIGENDIAN);

      assert_eq!(::get_rate(file), 8000.0)
      assert_eq!(::get_frame_count(file), 23493);
    });

    ::with_readonly(path[6], |file| {
      assert_eq!(::get_format(file), ::afapi::AF_FILE_WAVE);
      assert_eq!(::get_channels(file), 1);
      assert!(::get_track_ids(file) != 0);

      let (format, width) = ::get_sample_format(file);
      // TODO:: replace this with pattern matching
      assert_eq!(format, ::afapi::AF_SAMPFMT_TWOSCOMP);
      assert_eq!(width, 16);

      assert_eq!(::get_compression(file),
                 ::afapi::AF_COMPRESSION_MS_ADPCM);
      assert_eq!(::get_byte_order(file),
                ::afapi::AF_BYTEORDER_LITTLEENDIAN);

      assert_eq!(::get_rate(file), 22050.0)
      assert_eq!(::get_frame_count(file), 102212);
    });
  }

  #[test]
  fn should_fail_to_open_dev_null() {
    assert_eq!(::do_open("/dev/null", "r"), 0 as ::afapi::AFfilehandle);
  }

  #[test]
  fn struct_version_open_and_clonse() {
    match ::AudioFile::open(path[2], "r") {
      Err(v) => fail!("Shouldn't have happened!"),
       Ok(v) => { v.close(); },
    }
    let f = ::AudioFile::open("/dev/null", "r");
  }

}
