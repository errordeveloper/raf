/* automatically generated by rust-bindgen */

pub type Enum_Unnamed1 = ::libc::c_int;
pub static AU_PVTYPE_LONG: ::libc::c_int = 1;
pub static AU_PVTYPE_DOUBLE: ::libc::c_int = 2;
pub static AU_PVTYPE_PTR: ::libc::c_int = 3;
pub enum Struct__AUpvlist { }
pub type AUpvlist = *mut Struct__AUpvlist;
pub enum Struct__AFvirtualfile { }
pub type AFvirtualfile = Struct__AFvirtualfile;
pub enum Struct__AFfilesetup { }
pub type AFfilesetup = *mut Struct__AFfilesetup;
pub enum Struct__AFfilehandle { }
pub type AFfilehandle = *mut Struct__AFfilehandle;
pub type AFerrfunc =
    ::std::option::Option<extern "C" fn
                              (arg1: ::libc::c_long,
                               arg2: *const ::libc::c_char)>;
pub type AFframecount = ::libc::off_t;
pub type AFfileoffset = ::libc::off_t;
pub type Enum_Unnamed2 = ::libc::c_int;
pub static AF_DEFAULT_TRACK: ::libc::c_int = 1001;
pub type Enum_Unnamed3 = ::libc::c_int;
pub static AF_DEFAULT_INST: ::libc::c_int = 2001;
pub type Enum_Unnamed4 = ::libc::c_int;
pub static AF_NUM_UNLIMITED: ::libc::c_int = 99999;
pub type Enum_Unnamed5 = ::libc::c_int;
pub static AF_BYTEORDER_BIGENDIAN: ::libc::c_int = 501;
pub static AF_BYTEORDER_LITTLEENDIAN: ::libc::c_int = 502;
pub type Enum_Unnamed6 = ::libc::c_int;
pub static AF_FILE_UNKNOWN: ::libc::c_int = -1;
pub static AF_FILE_RAWDATA: ::libc::c_int = 0;
pub static AF_FILE_AIFFC: ::libc::c_int = 1;
pub static AF_FILE_AIFF: ::libc::c_int = 2;
pub static AF_FILE_NEXTSND: ::libc::c_int = 3;
pub static AF_FILE_WAVE: ::libc::c_int = 4;
pub static AF_FILE_BICSF: ::libc::c_int = 5;
pub static AF_FILE_IRCAM: ::libc::c_int = 5;
pub static AF_FILE_MPEG1BITSTREAM: ::libc::c_int = 6;
pub static AF_FILE_SOUNDDESIGNER1: ::libc::c_int = 7;
pub static AF_FILE_SOUNDDESIGNER2: ::libc::c_int = 8;
pub static AF_FILE_AVR: ::libc::c_int = 9;
pub static AF_FILE_IFF_8SVX: ::libc::c_int = 10;
pub static AF_FILE_SAMPLEVISION: ::libc::c_int = 11;
pub static AF_FILE_VOC: ::libc::c_int = 12;
pub static AF_FILE_NIST_SPHERE: ::libc::c_int = 13;
pub static AF_FILE_SOUNDFONT2: ::libc::c_int = 14;
pub static AF_FILE_CAF: ::libc::c_int = 15;
pub static AF_FILE_FLAC: ::libc::c_int = 16;
pub type Enum_Unnamed7 = ::libc::c_int;
pub static AF_LOOP_MODE_NOLOOP: ::libc::c_int = 0;
pub static AF_LOOP_MODE_FORW: ::libc::c_int = 1;
pub static AF_LOOP_MODE_FORWBAKW: ::libc::c_int = 2;
pub type Enum_Unnamed8 = ::libc::c_int;
pub static AF_SAMPFMT_TWOSCOMP: ::libc::c_int = 401;
pub static AF_SAMPFMT_UNSIGNED: ::libc::c_int = 402;
pub static AF_SAMPFMT_FLOAT: ::libc::c_int = 403;
pub static AF_SAMPFMT_DOUBLE: ::libc::c_int = 404;
pub type Enum_Unnamed9 = ::libc::c_int;
pub static AF_INST_LOOP_OFF: ::libc::c_int = 0;
pub static AF_INST_LOOP_CONTINUOUS: ::libc::c_int = 1;
pub static AF_INST_LOOP_SUSTAIN: ::libc::c_int = 3;
pub type Enum_Unnamed10 = ::libc::c_int;
pub static AF_INST_MIDI_BASENOTE: ::libc::c_int = 301;
pub static AF_INST_NUMCENTS_DETUNE: ::libc::c_int = 302;
pub static AF_INST_MIDI_LONOTE: ::libc::c_int = 303;
pub static AF_INST_MIDI_HINOTE: ::libc::c_int = 304;
pub static AF_INST_MIDI_LOVELOCITY: ::libc::c_int = 305;
pub static AF_INST_MIDI_HIVELOCITY: ::libc::c_int = 306;
pub static AF_INST_NUMDBS_GAIN: ::libc::c_int = 307;
pub static AF_INST_SUSLOOPID: ::libc::c_int = 308;
pub static AF_INST_RELLOOPID: ::libc::c_int = 309;
pub static AF_INST_SAMP_STARTFRAME: ::libc::c_int = 310;
pub static AF_INST_SAMP_ENDFRAME: ::libc::c_int = 311;
pub static AF_INST_SAMP_MODE: ::libc::c_int = 312;
pub static AF_INST_TRACKID: ::libc::c_int = 313;
pub static AF_INST_NAME: ::libc::c_int = 314;
pub static AF_INST_SAMP_RATE: ::libc::c_int = 315;
pub static AF_INST_PRESETID: ::libc::c_int = 316;
pub static AF_INST_PRESET_NAME: ::libc::c_int = 317;
pub type Enum_Unnamed11 = ::libc::c_int;
pub static AF_MISC_UNRECOGNIZED: ::libc::c_int = 0;
pub static AF_MISC_COPY: ::libc::c_int = 201;
pub static AF_MISC_AUTH: ::libc::c_int = 202;
pub static AF_MISC_NAME: ::libc::c_int = 203;
pub static AF_MISC_ANNO: ::libc::c_int = 204;
pub static AF_MISC_APPL: ::libc::c_int = 205;
pub static AF_MISC_MIDI: ::libc::c_int = 206;
pub static AF_MISC_PCMMAP: ::libc::c_int = 207;
pub static AF_MISC_NeXT: ::libc::c_int = 208;
pub static AF_MISC_IRCAM_PEAKAMP: ::libc::c_int = 209;
pub static AF_MISC_IRCAM_COMMENT: ::libc::c_int = 210;
pub static AF_MISC_COMMENT: ::libc::c_int = 210;
pub static AF_MISC_ICMT: ::libc::c_int = 210;
pub static AF_MISC_ICRD: ::libc::c_int = 211;
pub static AF_MISC_ISFT: ::libc::c_int = 212;
pub type Enum_Unnamed12 = ::libc::c_int;
pub static AF_COMPRESSION_UNKNOWN: ::libc::c_int = -1;
pub static AF_COMPRESSION_NONE: ::libc::c_int = 0;
pub static AF_COMPRESSION_G722: ::libc::c_int = 501;
pub static AF_COMPRESSION_G711_ULAW: ::libc::c_int = 502;
pub static AF_COMPRESSION_G711_ALAW: ::libc::c_int = 503;
pub static AF_COMPRESSION_APPLE_ACE2: ::libc::c_int = 504;
pub static AF_COMPRESSION_APPLE_ACE8: ::libc::c_int = 505;
pub static AF_COMPRESSION_APPLE_MAC3: ::libc::c_int = 506;
pub static AF_COMPRESSION_APPLE_MAC6: ::libc::c_int = 507;
pub static AF_COMPRESSION_G726: ::libc::c_int = 517;
pub static AF_COMPRESSION_G728: ::libc::c_int = 518;
pub static AF_COMPRESSION_DVI_AUDIO: ::libc::c_int = 519;
pub static AF_COMPRESSION_IMA: ::libc::c_int = 519;
pub static AF_COMPRESSION_GSM: ::libc::c_int = 520;
pub static AF_COMPRESSION_FS1016: ::libc::c_int = 521;
pub static AF_COMPRESSION_DV: ::libc::c_int = 522;
pub static AF_COMPRESSION_MS_ADPCM: ::libc::c_int = 523;
pub static AF_COMPRESSION_FLAC: ::libc::c_int = 530;
pub static AF_COMPRESSION_ALAC: ::libc::c_int = 540;
pub type Enum_Unnamed13 = ::libc::c_int;
pub static AF_QUERYTYPE_INSTPARAM: ::libc::c_int = 500;
pub static AF_QUERYTYPE_FILEFMT: ::libc::c_int = 501;
pub static AF_QUERYTYPE_COMPRESSION: ::libc::c_int = 502;
pub static AF_QUERYTYPE_COMPRESSIONPARAM: ::libc::c_int = 503;
pub static AF_QUERYTYPE_MISC: ::libc::c_int = 504;
pub static AF_QUERYTYPE_INST: ::libc::c_int = 505;
pub static AF_QUERYTYPE_MARK: ::libc::c_int = 506;
pub static AF_QUERYTYPE_LOOP: ::libc::c_int = 507;
pub type Enum_Unnamed14 = ::libc::c_int;
pub static AF_QUERY_NAME: ::libc::c_int = 600;
pub static AF_QUERY_DESC: ::libc::c_int = 601;
pub static AF_QUERY_LABEL: ::libc::c_int = 602;
pub static AF_QUERY_TYPE: ::libc::c_int = 603;
pub static AF_QUERY_DEFAULT: ::libc::c_int = 604;
pub static AF_QUERY_ID_COUNT: ::libc::c_int = 605;
pub static AF_QUERY_IDS: ::libc::c_int = 606;
pub static AF_QUERY_IMPLEMENTED: ::libc::c_int = 613;
pub static AF_QUERY_TYPE_COUNT: ::libc::c_int = 607;
pub static AF_QUERY_TYPES: ::libc::c_int = 608;
pub static AF_QUERY_NATIVE_SAMPFMT: ::libc::c_int = 609;
pub static AF_QUERY_NATIVE_SAMPWIDTH: ::libc::c_int = 610;
pub static AF_QUERY_SQUISHFAC: ::libc::c_int = 611;
pub static AF_QUERY_MAX_NUMBER: ::libc::c_int = 612;
pub static AF_QUERY_SUPPORTED: ::libc::c_int = 613;
pub type Enum_Unnamed15 = ::libc::c_int;
pub static AF_QUERY_TRACKS: ::libc::c_int = 620;
pub static AF_QUERY_CHANNELS: ::libc::c_int = 621;
pub static AF_QUERY_SAMPLE_SIZES: ::libc::c_int = 622;
pub static AF_QUERY_SAMPLE_FORMATS: ::libc::c_int = 623;
pub static AF_QUERY_COMPRESSION_TYPES: ::libc::c_int = 624;
pub type Enum_Unnamed16 = ::libc::c_int;
pub static AF_QUERY_VALUE_COUNT: ::libc::c_int = 650;
pub static AF_QUERY_VALUES: ::libc::c_int = 651;
pub type Enum_Unnamed17 = ::libc::c_int;
pub static AF_BAD_NOT_IMPLEMENTED: ::libc::c_int = 0;
pub static AF_BAD_FILEHANDLE: ::libc::c_int = 1;
pub static AF_BAD_OPEN: ::libc::c_int = 3;
pub static AF_BAD_CLOSE: ::libc::c_int = 4;
pub static AF_BAD_READ: ::libc::c_int = 5;
pub static AF_BAD_WRITE: ::libc::c_int = 6;
pub static AF_BAD_LSEEK: ::libc::c_int = 7;
pub static AF_BAD_NO_FILEHANDLE: ::libc::c_int = 8;
pub static AF_BAD_ACCMODE: ::libc::c_int = 10;
pub static AF_BAD_NOWRITEACC: ::libc::c_int = 11;
pub static AF_BAD_NOREADACC: ::libc::c_int = 12;
pub static AF_BAD_FILEFMT: ::libc::c_int = 13;
pub static AF_BAD_RATE: ::libc::c_int = 14;
pub static AF_BAD_CHANNELS: ::libc::c_int = 15;
pub static AF_BAD_SAMPCNT: ::libc::c_int = 16;
pub static AF_BAD_WIDTH: ::libc::c_int = 17;
pub static AF_BAD_SEEKMODE: ::libc::c_int = 18;
pub static AF_BAD_NO_LOOPDATA: ::libc::c_int = 19;
pub static AF_BAD_MALLOC: ::libc::c_int = 20;
pub static AF_BAD_LOOPID: ::libc::c_int = 21;
pub static AF_BAD_SAMPFMT: ::libc::c_int = 22;
pub static AF_BAD_FILESETUP: ::libc::c_int = 23;
pub static AF_BAD_TRACKID: ::libc::c_int = 24;
pub static AF_BAD_NUMTRACKS: ::libc::c_int = 25;
pub static AF_BAD_NO_FILESETUP: ::libc::c_int = 26;
pub static AF_BAD_LOOPMODE: ::libc::c_int = 27;
pub static AF_BAD_INSTID: ::libc::c_int = 28;
pub static AF_BAD_NUMLOOPS: ::libc::c_int = 29;
pub static AF_BAD_NUMMARKS: ::libc::c_int = 30;
pub static AF_BAD_MARKID: ::libc::c_int = 31;
pub static AF_BAD_MARKPOS: ::libc::c_int = 32;
pub static AF_BAD_NUMINSTS: ::libc::c_int = 33;
pub static AF_BAD_NOAESDATA: ::libc::c_int = 34;
pub static AF_BAD_MISCID: ::libc::c_int = 35;
pub static AF_BAD_NUMMISC: ::libc::c_int = 36;
pub static AF_BAD_MISCSIZE: ::libc::c_int = 37;
pub static AF_BAD_MISCTYPE: ::libc::c_int = 38;
pub static AF_BAD_MISCSEEK: ::libc::c_int = 39;
pub static AF_BAD_STRLEN: ::libc::c_int = 40;
pub static AF_BAD_RATECONV: ::libc::c_int = 45;
pub static AF_BAD_SYNCFILE: ::libc::c_int = 46;
pub static AF_BAD_CODEC_CONFIG: ::libc::c_int = 47;
pub static AF_BAD_CODEC_STATE: ::libc::c_int = 48;
pub static AF_BAD_CODEC_LICENSE: ::libc::c_int = 49;
pub static AF_BAD_CODEC_TYPE: ::libc::c_int = 50;
pub static AF_BAD_COMPRESSION: ::libc::c_int = 47;
pub static AF_BAD_COMPTYPE: ::libc::c_int = 50;
pub static AF_BAD_INSTPTYPE: ::libc::c_int = 51;
pub static AF_BAD_INSTPID: ::libc::c_int = 52;
pub static AF_BAD_BYTEORDER: ::libc::c_int = 53;
pub static AF_BAD_FILEFMT_PARAM: ::libc::c_int = 54;
pub static AF_BAD_COMP_PARAM: ::libc::c_int = 55;
pub static AF_BAD_DATAOFFSET: ::libc::c_int = 56;
pub static AF_BAD_FRAMECNT: ::libc::c_int = 57;
pub static AF_BAD_QUERYTYPE: ::libc::c_int = 58;
pub static AF_BAD_QUERY: ::libc::c_int = 59;
pub static AF_WARNING_CODEC_RATE: ::libc::c_int = 60;
pub static AF_WARNING_RATECVT: ::libc::c_int = 61;
pub static AF_BAD_HEADER: ::libc::c_int = 62;
pub static AF_BAD_FRAME: ::libc::c_int = 63;
pub static AF_BAD_LOOPCOUNT: ::libc::c_int = 64;
pub static AF_BAD_DMEDIA_CALL: ::libc::c_int = 65;
pub static AF_BAD_AIFF_HEADER: ::libc::c_int = 108;
pub static AF_BAD_AIFF_FORM: ::libc::c_int = 109;
pub static AF_BAD_AIFF_SSND: ::libc::c_int = 110;
pub static AF_BAD_AIFF_CHUNKID: ::libc::c_int = 111;
pub static AF_BAD_AIFF_COMM: ::libc::c_int = 112;
pub static AF_BAD_AIFF_INST: ::libc::c_int = 113;
pub static AF_BAD_AIFF_MARK: ::libc::c_int = 114;
pub static AF_BAD_AIFF_SKIP: ::libc::c_int = 115;
pub static AF_BAD_AIFF_LOOPMODE: ::libc::c_int = 116;
pub type Enum_Unnamed18 = ::libc::c_int;
pub static AF_ERR_NOT_IMPLEMENTED: ::libc::c_int = 3000;
pub static AF_ERR_BAD_FILEHANDLE: ::libc::c_int = 3001;
pub static AF_ERR_BAD_READ: ::libc::c_int = 3005;
pub static AF_ERR_BAD_WRITE: ::libc::c_int = 3006;
pub static AF_ERR_BAD_LSEEK: ::libc::c_int = 3007;
pub static AF_ERR_BAD_ACCMODE: ::libc::c_int = 3010;
pub static AF_ERR_NO_WRITEACC: ::libc::c_int = 3011;
pub static AF_ERR_NO_READACC: ::libc::c_int = 3012;
pub static AF_ERR_BAD_FILEFMT: ::libc::c_int = 3013;
pub static AF_ERR_BAD_RATE: ::libc::c_int = 3014;
pub static AF_ERR_BAD_CHANNELS: ::libc::c_int = 3015;
pub static AF_ERR_BAD_SAMPCNT: ::libc::c_int = 3016;
pub static AF_ERR_BAD_WIDTH: ::libc::c_int = 3017;
pub static AF_ERR_BAD_SEEKMODE: ::libc::c_int = 3018;
pub static AF_ERR_BAD_LOOPID: ::libc::c_int = 3021;
pub static AF_ERR_BAD_SAMPFMT: ::libc::c_int = 3022;
pub static AF_ERR_BAD_FILESETUP: ::libc::c_int = 3023;
pub static AF_ERR_BAD_TRACKID: ::libc::c_int = 3024;
pub static AF_ERR_BAD_NUMTRACKS: ::libc::c_int = 3025;
pub static AF_ERR_BAD_LOOPMODE: ::libc::c_int = 3027;
pub static AF_ERR_BAD_INSTID: ::libc::c_int = 3028;
pub static AF_ERR_BAD_NUMLOOPS: ::libc::c_int = 3029;
pub static AF_ERR_BAD_NUMMARKS: ::libc::c_int = 3030;
pub static AF_ERR_BAD_MARKID: ::libc::c_int = 3031;
pub static AF_ERR_BAD_MARKPOS: ::libc::c_int = 3032;
pub static AF_ERR_BAD_NUMINSTS: ::libc::c_int = 3033;
pub static AF_ERR_BAD_NOAESDATA: ::libc::c_int = 3034;
pub static AF_ERR_BAD_MISCID: ::libc::c_int = 3035;
pub static AF_ERR_BAD_NUMMISC: ::libc::c_int = 3036;
pub static AF_ERR_BAD_MISCSIZE: ::libc::c_int = 3037;
pub static AF_ERR_BAD_MISCTYPE: ::libc::c_int = 3038;
pub static AF_ERR_BAD_MISCSEEK: ::libc::c_int = 3039;
pub static AF_ERR_BAD_STRLEN: ::libc::c_int = 3040;
pub static AF_ERR_BAD_RATECONV: ::libc::c_int = 3045;
pub static AF_ERR_BAD_SYNCFILE: ::libc::c_int = 3046;
pub static AF_ERR_BAD_CODEC_CONFIG: ::libc::c_int = 3047;
pub static AF_ERR_BAD_CODEC_TYPE: ::libc::c_int = 3050;
pub static AF_ERR_BAD_INSTPTYPE: ::libc::c_int = 3051;
pub static AF_ERR_BAD_INSTPID: ::libc::c_int = 3052;
pub static AF_ERR_BAD_BYTEORDER: ::libc::c_int = 3053;
pub static AF_ERR_BAD_FILEFMT_PARAM: ::libc::c_int = 3054;
pub static AF_ERR_BAD_COMP_PARAM: ::libc::c_int = 3055;
pub static AF_ERR_BAD_DATAOFFSET: ::libc::c_int = 3056;
pub static AF_ERR_BAD_FRAMECNT: ::libc::c_int = 3057;
pub static AF_ERR_BAD_QUERYTYPE: ::libc::c_int = 3058;
pub static AF_ERR_BAD_QUERY: ::libc::c_int = 3059;
pub static AF_ERR_BAD_HEADER: ::libc::c_int = 3062;
pub static AF_ERR_BAD_FRAME: ::libc::c_int = 3063;
pub static AF_ERR_BAD_LOOPCOUNT: ::libc::c_int = 3064;
pub static AF_ERR_BAD_AIFF_HEADER: ::libc::c_int = 3066;
pub static AF_ERR_BAD_AIFF_FORM: ::libc::c_int = 3067;
pub static AF_ERR_BAD_AIFF_SSND: ::libc::c_int = 3068;
pub static AF_ERR_BAD_AIFF_CHUNKID: ::libc::c_int = 3069;
pub static AF_ERR_BAD_AIFF_COMM: ::libc::c_int = 3070;
pub static AF_ERR_BAD_AIFF_INST: ::libc::c_int = 3071;
pub static AF_ERR_BAD_AIFF_MARK: ::libc::c_int = 3072;
pub static AF_ERR_BAD_AIFF_SKIP: ::libc::c_int = 3073;
pub static AF_ERR_BAD_AIFF_LOOPMODE: ::libc::c_int = 3074;
extern "C" {
    pub fn AUpvnew(maxItems: ::libc::c_int) -> AUpvlist;
    pub fn AUpvgetmaxitems(arg1: AUpvlist) -> ::libc::c_int;
    pub fn AUpvfree(arg1: AUpvlist) -> ::libc::c_int;
    pub fn AUpvsetparam(arg1: AUpvlist, item: ::libc::c_int,
                        param: ::libc::c_int) -> ::libc::c_int;
    pub fn AUpvsetvaltype(arg1: AUpvlist, item: ::libc::c_int,
                          _type: ::libc::c_int) -> ::libc::c_int;
    pub fn AUpvsetval(arg1: AUpvlist, item: ::libc::c_int,
                      val: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn AUpvgetparam(arg1: AUpvlist, item: ::libc::c_int,
                        param: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn AUpvgetvaltype(arg1: AUpvlist, item: ::libc::c_int,
                          _type: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn AUpvgetval(arg1: AUpvlist, item: ::libc::c_int,
                      val: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn afSetErrorHandler(efunc: AFerrfunc) -> AFerrfunc;
    pub fn afQuery(querytype: ::libc::c_int, arg1: ::libc::c_int,
                   arg2: ::libc::c_int, arg3: ::libc::c_int,
                   arg4: ::libc::c_int) -> AUpvlist;
    pub fn afQueryLong(querytype: ::libc::c_int, arg1: ::libc::c_int,
                       arg2: ::libc::c_int, arg3: ::libc::c_int,
                       arg4: ::libc::c_int) -> ::libc::c_long;
    pub fn afQueryDouble(querytype: ::libc::c_int, arg1: ::libc::c_int,
                         arg2: ::libc::c_int, arg3: ::libc::c_int,
                         arg4: ::libc::c_int) -> ::libc::c_double;
    pub fn afQueryPointer(querytype: ::libc::c_int, arg1: ::libc::c_int,
                          arg2: ::libc::c_int, arg3: ::libc::c_int,
                          arg4: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn afNewFileSetup() -> AFfilesetup;
    pub fn afFreeFileSetup(arg1: AFfilesetup);
    pub fn afIdentifyFD(arg1: ::libc::c_int) -> ::libc::c_int;
    pub fn afIdentifyNamedFD(arg1: ::libc::c_int,
                             filename: *const ::libc::c_char,
                             implemented: *mut ::libc::c_int) ->
     ::libc::c_int;
    pub fn afOpenFile(filename: *const ::libc::c_char,
                      mode: *const ::libc::c_char, setup: AFfilesetup) ->
     AFfilehandle;
    pub fn afOpenVirtualFile(vfile: *mut AFvirtualfile,
                             mode: *const ::libc::c_char, setup: AFfilesetup)
     -> AFfilehandle;
    pub fn afOpenFD(fd: ::libc::c_int, mode: *const ::libc::c_char,
                    setup: AFfilesetup) -> AFfilehandle;
    pub fn afOpenNamedFD(fd: ::libc::c_int, mode: *const ::libc::c_char,
                         setup: AFfilesetup, filename: *const ::libc::c_char)
     -> AFfilehandle;
    pub fn afSaveFilePosition(file: AFfilehandle);
    pub fn afRestoreFilePosition(file: AFfilehandle);
    pub fn afSyncFile(file: AFfilehandle) -> ::libc::c_int;
    pub fn afCloseFile(file: AFfilehandle) -> ::libc::c_int;
    pub fn afInitFileFormat(arg1: AFfilesetup, format: ::libc::c_int);
    pub fn afGetFileFormat(arg1: AFfilehandle, version: *mut ::libc::c_int) ->
     ::libc::c_int;
    pub fn afInitTrackIDs(arg1: AFfilesetup, trackids: *const ::libc::c_int,
                          trackCount: ::libc::c_int);
    pub fn afGetTrackIDs(arg1: AFfilehandle, trackids: *mut ::libc::c_int) ->
     ::libc::c_int;
    pub fn afReadFrames(arg1: AFfilehandle, track: ::libc::c_int,
                        buffer: *mut ::libc::c_void,
                        frameCount: ::libc::c_int) -> ::libc::c_int;
    pub fn afWriteFrames(arg1: AFfilehandle, track: ::libc::c_int,
                         buffer: *const ::libc::c_void,
                         frameCount: ::libc::c_int) -> ::libc::c_int;
    pub fn afSeekFrame(arg1: AFfilehandle, track: ::libc::c_int,
                       frameoffset: AFframecount) -> AFframecount;
    pub fn afTellFrame(arg1: AFfilehandle, track: ::libc::c_int) ->
     AFframecount;
    pub fn afGetTrackBytes(arg1: AFfilehandle, track: ::libc::c_int) ->
     AFfileoffset;
    pub fn afGetFrameSize(arg1: AFfilehandle, track: ::libc::c_int,
                          expand3to4: ::libc::c_int) -> ::libc::c_float;
    pub fn afGetVirtualFrameSize(arg1: AFfilehandle, track: ::libc::c_int,
                                 expand3to4: ::libc::c_int) ->
     ::libc::c_float;
    pub fn afInitAESChannelData(arg1: AFfilesetup, track: ::libc::c_int);
    pub fn afInitAESChannelDataTo(arg1: AFfilesetup, track: ::libc::c_int,
                                  willBeData: ::libc::c_int);
    pub fn afGetAESChannelData(arg1: AFfilehandle, track: ::libc::c_int,
                               buf: [::libc::c_uchar, ..24u]) ->
     ::libc::c_int;
    pub fn afSetAESChannelData(arg1: AFfilehandle, track: ::libc::c_int,
                               buf: [::libc::c_uchar, ..24u]);
    pub fn afInitByteOrder(arg1: AFfilesetup, track: ::libc::c_int,
                           byteOrder: ::libc::c_int);
    pub fn afGetByteOrder(arg1: AFfilehandle, track: ::libc::c_int) ->
     ::libc::c_int;
    pub fn afSetVirtualByteOrder(arg1: AFfilehandle, track: ::libc::c_int,
                                 byteOrder: ::libc::c_int) -> ::libc::c_int;
    pub fn afGetVirtualByteOrder(arg1: AFfilehandle, track: ::libc::c_int) ->
     ::libc::c_int;
    pub fn afInitChannels(arg1: AFfilesetup, track: ::libc::c_int,
                          nchannels: ::libc::c_int);
    pub fn afGetChannels(arg1: AFfilehandle, track: ::libc::c_int) ->
     ::libc::c_int;
    pub fn afSetVirtualChannels(arg1: AFfilehandle, track: ::libc::c_int,
                                channelCount: ::libc::c_int) -> ::libc::c_int;
    pub fn afGetVirtualChannels(arg1: AFfilehandle, track: ::libc::c_int) ->
     ::libc::c_int;
    pub fn afSetChannelMatrix(arg1: AFfilehandle, track: ::libc::c_int,
                              matrix: *mut ::libc::c_double);
    pub fn afInitSampleFormat(arg1: AFfilesetup, track: ::libc::c_int,
                              sampleFormat: ::libc::c_int,
                              sampleWidth: ::libc::c_int);
    pub fn afGetSampleFormat(file: AFfilehandle, track: ::libc::c_int,
                             sampleFormat: *mut ::libc::c_int,
                             sampleWidth: *mut ::libc::c_int);
    pub fn afSetVirtualSampleFormat(arg1: AFfilehandle, track: ::libc::c_int,
                                    sampleFormat: ::libc::c_int,
                                    sampleWidth: ::libc::c_int) ->
     ::libc::c_int;
    pub fn afGetVirtualSampleFormat(arg1: AFfilehandle, track: ::libc::c_int,
                                    sampleFormat: *mut ::libc::c_int,
                                    sampleWidth: *mut ::libc::c_int);
    pub fn afInitRate(arg1: AFfilesetup, track: ::libc::c_int,
                      rate: ::libc::c_double);
    pub fn afGetRate(arg1: AFfilehandle, track: ::libc::c_int) ->
     ::libc::c_double;
    pub fn afInitCompression(arg1: AFfilesetup, track: ::libc::c_int,
                             compression: ::libc::c_int);
    pub fn afGetCompression(arg1: AFfilehandle, track: ::libc::c_int) ->
     ::libc::c_int;
    pub fn afInitPCMMapping(filesetup: AFfilesetup, track: ::libc::c_int,
                            slope: ::libc::c_double,
                            intercept: ::libc::c_double,
                            minClip: ::libc::c_double,
                            maxClip: ::libc::c_double);
    pub fn afGetPCMMapping(file: AFfilehandle, track: ::libc::c_int,
                           slope: *mut ::libc::c_double,
                           intercept: *mut ::libc::c_double,
                           minClip: *mut ::libc::c_double,
                           maxClip: *mut ::libc::c_double);
    pub fn afSetTrackPCMMapping(file: AFfilehandle, track: ::libc::c_int,
                                slope: ::libc::c_double,
                                intercept: ::libc::c_double,
                                minClip: ::libc::c_double,
                                maxClip: ::libc::c_double) -> ::libc::c_int;
    pub fn afSetVirtualPCMMapping(file: AFfilehandle, track: ::libc::c_int,
                                  slope: ::libc::c_double,
                                  intercept: ::libc::c_double,
                                  minClip: ::libc::c_double,
                                  maxClip: ::libc::c_double) -> ::libc::c_int;
    pub fn afGetVirtualPCMMapping(file: AFfilehandle, track: ::libc::c_int,
                                  slope: *mut ::libc::c_double,
                                  intercept: *mut ::libc::c_double,
                                  minClip: *mut ::libc::c_double,
                                  maxClip: *mut ::libc::c_double);
    pub fn afInitDataOffset(arg1: AFfilesetup, track: ::libc::c_int,
                            offset: AFfileoffset);
    pub fn afGetDataOffset(arg1: AFfilehandle, track: ::libc::c_int) ->
     AFfileoffset;
    pub fn afInitFrameCount(arg1: AFfilesetup, track: ::libc::c_int,
                            frameCount: AFframecount);
    pub fn afGetFrameCount(file: AFfilehandle, track: ::libc::c_int) ->
     AFframecount;
    pub fn afInitLoopIDs(arg1: AFfilesetup, instid: ::libc::c_int,
                         ids: *const ::libc::c_int, nids: ::libc::c_int);
    pub fn afGetLoopIDs(arg1: AFfilehandle, instid: ::libc::c_int,
                        loopids: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn afSetLoopMode(arg1: AFfilehandle, instid: ::libc::c_int,
                         _loop: ::libc::c_int, mode: ::libc::c_int);
    pub fn afGetLoopMode(arg1: AFfilehandle, instid: ::libc::c_int,
                         loopid: ::libc::c_int) -> ::libc::c_int;
    pub fn afSetLoopCount(arg1: AFfilehandle, instid: ::libc::c_int,
                          _loop: ::libc::c_int, count: ::libc::c_int) ->
     ::libc::c_int;
    pub fn afGetLoopCount(arg1: AFfilehandle, instid: ::libc::c_int,
                          loopid: ::libc::c_int) -> ::libc::c_int;
    pub fn afSetLoopStart(arg1: AFfilehandle, instid: ::libc::c_int,
                          loopid: ::libc::c_int, markerid: ::libc::c_int);
    pub fn afGetLoopStart(arg1: AFfilehandle, instid: ::libc::c_int,
                          loopid: ::libc::c_int) -> ::libc::c_int;
    pub fn afSetLoopEnd(arg1: AFfilehandle, instid: ::libc::c_int,
                        loopid: ::libc::c_int, markerid: ::libc::c_int);
    pub fn afGetLoopEnd(arg1: AFfilehandle, instid: ::libc::c_int,
                        loopid: ::libc::c_int) -> ::libc::c_int;
    pub fn afSetLoopStartFrame(arg1: AFfilehandle, instid: ::libc::c_int,
                               _loop: ::libc::c_int, startFrame: AFframecount)
     -> ::libc::c_int;
    pub fn afGetLoopStartFrame(arg1: AFfilehandle, instid: ::libc::c_int,
                               _loop: ::libc::c_int) -> AFframecount;
    pub fn afSetLoopEndFrame(arg1: AFfilehandle, instid: ::libc::c_int,
                             _loop: ::libc::c_int, startFrame: AFframecount)
     -> ::libc::c_int;
    pub fn afGetLoopEndFrame(arg1: AFfilehandle, instid: ::libc::c_int,
                             _loop: ::libc::c_int) -> AFframecount;
    pub fn afSetLoopTrack(arg1: AFfilehandle, instid: ::libc::c_int,
                          loopid: ::libc::c_int, trackid: ::libc::c_int);
    pub fn afGetLoopTrack(arg1: AFfilehandle, instid: ::libc::c_int,
                          loopid: ::libc::c_int) -> ::libc::c_int;
    pub fn afInitMarkIDs(arg1: AFfilesetup, trackid: ::libc::c_int,
                         ids: *const ::libc::c_int, nids: ::libc::c_int);
    pub fn afGetMarkIDs(file: AFfilehandle, trackid: ::libc::c_int,
                        markids: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn afSetMarkPosition(file: AFfilehandle, trackid: ::libc::c_int,
                             markid: ::libc::c_int, markpos: AFframecount);
    pub fn afGetMarkPosition(file: AFfilehandle, trackid: ::libc::c_int,
                             markid: ::libc::c_int) -> AFframecount;
    pub fn afInitMarkName(arg1: AFfilesetup, trackid: ::libc::c_int,
                          marker: ::libc::c_int, name: *const ::libc::c_char);
    pub fn afInitMarkComment(arg1: AFfilesetup, trackid: ::libc::c_int,
                             marker: ::libc::c_int,
                             comment: *const ::libc::c_char);
    pub fn afGetMarkName(file: AFfilehandle, trackid: ::libc::c_int,
                         markid: ::libc::c_int) -> *mut ::libc::c_char;
    pub fn afGetMarkComment(file: AFfilehandle, trackid: ::libc::c_int,
                            markid: ::libc::c_int) -> *mut ::libc::c_char;
    pub fn afInitInstIDs(arg1: AFfilesetup, ids: *const ::libc::c_int,
                         nids: ::libc::c_int);
    pub fn afGetInstIDs(file: AFfilehandle, instids: *mut ::libc::c_int) ->
     ::libc::c_int;
    pub fn afGetInstParams(file: AFfilehandle, instid: ::libc::c_int,
                           pvlist: AUpvlist, nparams: ::libc::c_int);
    pub fn afSetInstParams(file: AFfilehandle, instid: ::libc::c_int,
                           pvlist: AUpvlist, nparams: ::libc::c_int);
    pub fn afGetInstParamLong(file: AFfilehandle, instid: ::libc::c_int,
                              param: ::libc::c_int) -> ::libc::c_long;
    pub fn afSetInstParamLong(file: AFfilehandle, instid: ::libc::c_int,
                              param: ::libc::c_int, value: ::libc::c_long);
    pub fn afInitMiscIDs(arg1: AFfilesetup, ids: *const ::libc::c_int,
                         nids: ::libc::c_int);
    pub fn afGetMiscIDs(arg1: AFfilehandle, ids: *mut ::libc::c_int) ->
     ::libc::c_int;
    pub fn afInitMiscType(arg1: AFfilesetup, miscellaneousid: ::libc::c_int,
                          _type: ::libc::c_int);
    pub fn afGetMiscType(arg1: AFfilehandle, miscellaneousid: ::libc::c_int)
     -> ::libc::c_int;
    pub fn afInitMiscSize(arg1: AFfilesetup, miscellaneousid: ::libc::c_int,
                          size: ::libc::c_int);
    pub fn afGetMiscSize(arg1: AFfilehandle, miscellaneousid: ::libc::c_int)
     -> ::libc::c_int;
    pub fn afWriteMisc(arg1: AFfilehandle, miscellaneousid: ::libc::c_int,
                       buf: *const ::libc::c_void, bytes: ::libc::c_int) ->
     ::libc::c_int;
    pub fn afReadMisc(arg1: AFfilehandle, miscellaneousid: ::libc::c_int,
                      buf: *mut ::libc::c_void, bytes: ::libc::c_int) ->
     ::libc::c_int;
    pub fn afSeekMisc(arg1: AFfilehandle, miscellaneousid: ::libc::c_int,
                      offset: ::libc::c_int) -> ::libc::c_int;
}
