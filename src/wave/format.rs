//! Information related to the format of waveform-audio data.
use crate::util::BinaryRead as _;
use std::convert::TryInto;
use std::io::{self, Read, Seek, SeekFrom};
use winapi::shared::mmreg::*;

enum_with_try_from!(
/// Waveform-audio format type.
pub enum Tag(u16) {
    /// Format used for For one- or two-channel PCM data.
    Pcm = WAVE_FORMAT_PCM,
    /// Microsoft Corporation.
    Unknown = WAVE_FORMAT_UNKNOWN,
    /// Microsoft Corporation.
    AdPcm = WAVE_FORMAT_ADPCM,
    /// Microsoft Corporation.
    IeeeFloat = WAVE_FORMAT_IEEE_FLOAT,
    /// Compaq Computer Corp.
    Vselp = WAVE_FORMAT_VSELP,
    /// IBM Corporation.
    IbmCvsd = WAVE_FORMAT_IBM_CVSD,
    /// Microsoft Corporation.
    Alaw = WAVE_FORMAT_ALAW,
    /// Microsoft Corporation.
    Mulaw = WAVE_FORMAT_MULAW,
    /// Microsoft Corporation.
    Dts = WAVE_FORMAT_DTS,
    /// Microsoft Corporation.
    Drm = WAVE_FORMAT_DRM,
    /// Microsoft Corporation.
    WmaVoice9 = WAVE_FORMAT_WMAVOICE9,
    /// Microsoft Corporation.
    WmaVoice10 = WAVE_FORMAT_WMAVOICE10,
    /// OKI.
    OkiAdPcm = WAVE_FORMAT_OKI_ADPCM,
    /// Intel Corporation. DVI and IMA formats.
    DviImaAdPcm = WAVE_FORMAT_DVI_ADPCM,
    /// Videologic.
    MediaspaceAdPcm = WAVE_FORMAT_MEDIASPACE_ADPCM,
    /// Sierra Semiconductor Corp.
    SierraAdPcm = WAVE_FORMAT_SIERRA_ADPCM,
    /// Antex Electronics Corporation.
    G723AdPcm = WAVE_FORMAT_G723_ADPCM,
    /// DSP Solutions, Inc.
    DigiStd = WAVE_FORMAT_DIGISTD,
    /// DSP Solutions, Inc.
    DigiFix = WAVE_FORMAT_DIGIFIX,
    /// Dialogic Corporation.
    DialogicOkiAdPcm = WAVE_FORMAT_DIALOGIC_OKI_ADPCM,
    /// Media Vision, Inc.
    MediavisionAdPcm = WAVE_FORMAT_MEDIAVISION_ADPCM,
    /// Hewlett-Packard Company.
    CuCodec = WAVE_FORMAT_CU_CODEC,
    /// Hewlett-Packard Company.
    HpDynVoice = WAVE_FORMAT_HP_DYN_VOICE,
    /// Yamaha Corporation of America.
    YamahaAdPcm = WAVE_FORMAT_YAMAHA_ADPCM,
    /// Speech Compression.
    Sonarc = WAVE_FORMAT_SONARC,
    /// DSP Group, Inc.
    DspGroupTrueSpeech = WAVE_FORMAT_DSPGROUP_TRUESPEECH,
    /// Echo Speech Corporation.
    EchoSc1 = WAVE_FORMAT_ECHOSC1,
    /// Virtual Music, Inc.
    AudiofileAf36 = WAVE_FORMAT_AUDIOFILE_AF36,
    /// Audio Processing Technology.
    Aptx = WAVE_FORMAT_APTX,
    /// Virtual Music, Inc.
    AudiofileAf10 = WAVE_FORMAT_AUDIOFILE_AF10,
    /// Aculab plc.
    Prosody1612 = WAVE_FORMAT_PROSODY_1612,
    /// Merging Technologies S.A.
    Lrc = WAVE_FORMAT_LRC,
    /// Dolby Laboratories.
    DolbyAc2 = WAVE_FORMAT_DOLBY_AC2,
    /// Microsoft Corporation.
    MsGsm610 = WAVE_FORMAT_GSM610,
    /// Microsoft Corporation.
    MsnAudio = WAVE_FORMAT_MSNAUDIO,
    /// Antex Electronics Corporation.
    AntexAdpcme = WAVE_FORMAT_ANTEX_ADPCME,
    /// Control Resources Limited.
    ControlResVqlpc = WAVE_FORMAT_CONTROL_RES_VQLPC,
    /// DSP Solutions, Inc.
    Digireal = WAVE_FORMAT_DIGIREAL,
    /// DSP Solutions, Inc.
    DigiAdPcm = WAVE_FORMAT_DIGIADPCM,
    /// Control Resources Limited.
    ControlResCr10 = WAVE_FORMAT_CONTROL_RES_CR10,
    /// Natural MicroSystems.
    NmsVbxAdPcm = WAVE_FORMAT_NMS_VBXADPCM,
    /// Crystal Semiconductor IMA ADPCM.
    CsImaAdPcm = WAVE_FORMAT_CS_IMAADPCM,
    /// Echo Speech Corporation.
    EchoSc3 = WAVE_FORMAT_ECHOSC3,
    /// Rockwell International.
    RockwellAdPcm = WAVE_FORMAT_ROCKWELL_ADPCM,
    /// Rockwell International.
    RockwellDigitalk = WAVE_FORMAT_ROCKWELL_DIGITALK,
    /// Xebec Multimedia Solutions Limited.
    Xebec = WAVE_FORMAT_XEBEC,
    /// Antex Electronics Corporation.
    G721AdPcm = WAVE_FORMAT_G721_ADPCM,
    /// Antex Electronics Corporation.
    G728Celp = WAVE_FORMAT_G728_CELP,
    /// Microsoft Corporation.
    Msg723 = WAVE_FORMAT_MSG723,
    /// Intel Corp.
    IntelG7231 = WAVE_FORMAT_INTEL_G723_1,
    /// Intel Corp.
    IntelG729 = WAVE_FORMAT_INTEL_G729,
    /// Sharp.
    SharpG726 = WAVE_FORMAT_SHARP_G726,
    /// Microsoft Corporation.
    Mpeg = WAVE_FORMAT_MPEG,
    /// InSoft, Inc.
    Rt24 = WAVE_FORMAT_RT24,
    /// InSoft, Inc.
    Pac = WAVE_FORMAT_PAC,
    /// ISO/MPEG Layer3 Format Tag.
    MpegLayer3 = WAVE_FORMAT_MPEGLAYER3,
    /// Lucent Technologies.
    LucentG723 = WAVE_FORMAT_LUCENT_G723,
    /// Cirrus Logic.
    Cirrus = WAVE_FORMAT_CIRRUS,
    /// ESS Technology.
    EsPcm = WAVE_FORMAT_ESPCM,
    /// Voxware Inc.
    Voxware = WAVE_FORMAT_VOXWARE,
    /// Canopus, co., Ltd.
    CanopusAtrac = WAVE_FORMAT_CANOPUS_ATRAC,
    /// APICOM.
    G726AdPcm = WAVE_FORMAT_G726_ADPCM,
    /// APICOM.
    G722AdPcm = WAVE_FORMAT_G722_ADPCM,
    /// Microsoft Corporation.
    Dsat = WAVE_FORMAT_DSAT,
    /// Microsoft Corporation.
    DsatDisplay = WAVE_FORMAT_DSAT_DISPLAY,
    /// Voxware Inc.
    VoxwareByteAligned = WAVE_FORMAT_VOXWARE_BYTE_ALIGNED,
    /// Voxware Inc.
    VoxwareAc8 = WAVE_FORMAT_VOXWARE_AC8,
    /// Voxware Inc.
    VoxwareAc10 = WAVE_FORMAT_VOXWARE_AC10,
    /// Voxware Inc.
    VoxwareAc16 = WAVE_FORMAT_VOXWARE_AC16,
    /// Voxware Inc.
    VoxwareAc20 = WAVE_FORMAT_VOXWARE_AC20,
    /// Voxware Inc.
    VoxwareRt24 = WAVE_FORMAT_VOXWARE_RT24,
    /// Voxware Inc.
    VoxwareRt29 = WAVE_FORMAT_VOXWARE_RT29,
    /// Voxware Inc.
    VoxwareRt29HW = WAVE_FORMAT_VOXWARE_RT29HW,
    /// Voxware Inc.
    VoxwareVr12 = WAVE_FORMAT_VOXWARE_VR12,
    /// Voxware Inc.
    VoxwareVr18 = WAVE_FORMAT_VOXWARE_VR18,
    /// Voxware Inc.
    VoxwareTq40 = WAVE_FORMAT_VOXWARE_TQ40,
    /// Voxware Inc.
    VoxwareSc3 = WAVE_FORMAT_VOXWARE_SC3,
    /// Voxware Inc.
    VoxwareSc31 = WAVE_FORMAT_VOXWARE_SC3_1,
    /// Softsound, Ltd.
    Softsound = WAVE_FORMAT_SOFTSOUND,
    /// Voxware Inc.
    VoxwareTq60 = WAVE_FORMAT_VOXWARE_TQ60,
    /// Microsoft Corporation.
    Msrt24 = WAVE_FORMAT_MSRT24,
    /// AT&T Labs, Inc.
    G729A = WAVE_FORMAT_G729A,
    /// Motion Pixels.
    Mvi2 = WAVE_FORMAT_MVI_MVI2,
    /// DataFusion Systems (Pty) (Ltd).
    DfG726 = WAVE_FORMAT_DF_G726,
    /// DataFusion Systems (Pty) (Ltd).
    DfGsm610 = WAVE_FORMAT_DF_GSM610,
    /// Iterated Systems, Inc.
    IsiAudio = WAVE_FORMAT_ISIAUDIO,
    /// OnLive! Technologies, Inc.
    OnLive = WAVE_FORMAT_ONLIVE,
    /// Multitude Inc.
    MultitudeFtSx20 = WAVE_FORMAT_MULTITUDE_FT_SX20,
    /// Infocom.
    InfocomItsG721AdPcm = WAVE_FORMAT_INFOCOM_ITS_G721_ADPCM,
    /// Convedia Corp.
    ConvediaG729 = WAVE_FORMAT_CONVEDIA_G729,
    /// Congruency Inc.
    Congruency = WAVE_FORMAT_CONGRUENCY,
    /// Siemens Business Communications Sys.
    Sbc24 = WAVE_FORMAT_SBC24,
    /// Sonic Foundry.
    DolbyAc3Spdif = WAVE_FORMAT_DOLBY_AC3_SPDIF,
    /// MediaSonic.
    MediasonicG723 = WAVE_FORMAT_MEDIASONIC_G723,
    /// Aculab plc.
    Prosody8Kbps = WAVE_FORMAT_PROSODY_8KBPS,
    /// ZyXEL Communications, Inc.
    ZyxelAdPcm = WAVE_FORMAT_ZYXEL_ADPCM,
    /// Philips Speech Processing.
    PhilipsLpcbb = WAVE_FORMAT_PHILIPS_LPCBB,
    /// Studer Professional Audio AG.
    Packed = WAVE_FORMAT_PACKED,
    /// Malden Electronics Ltd.
    MaldenPhonytalk = WAVE_FORMAT_MALDEN_PHONYTALK,
    /// Racal recorders.
    RacalRecorderGsm = WAVE_FORMAT_RACAL_RECORDER_GSM,
    /// Racal recorders.
    RacalRecorderG720A = WAVE_FORMAT_RACAL_RECORDER_G720_A,
    /// Racal recorders.
    RacalRecorderG7231 = WAVE_FORMAT_RACAL_RECORDER_G723_1,
    /// Racal recorders.
    RacalRecorderTetraAcelp = WAVE_FORMAT_RACAL_RECORDER_TETRA_ACELP,
    /// NEC Corp.
    NecAac = WAVE_FORMAT_NEC_AAC,
    /// For Raw AAC, with format block of "audio specific config" (as defined by MPEG-4), that
    /// follows a wave format structure.
    RawAac1 = WAVE_FORMAT_RAW_AAC1,
    /// Rhetorex Inc.
    RhetorexAdPcm = WAVE_FORMAT_RHETOREX_ADPCM,
    /// BeCubed Software Inc.
    Irat = WAVE_FORMAT_IRAT,
    /// Vivo Software.
    VivoG723 = WAVE_FORMAT_VIVO_G723,
    /// Vivo Software.
    VivoSiren = WAVE_FORMAT_VIVO_SIREN,
    /// Philips Speech Processing.
    PhilipsCelp = WAVE_FORMAT_PHILIPS_CELP,
    /// Philips Speech Processing.
    PhilipsGrundig = WAVE_FORMAT_PHILIPS_GRUNDIG,
    /// Digital Equipment Corporation.
    DigitalG723 = WAVE_FORMAT_DIGITAL_G723,
    /// Sanyo Electric Co., Ltd.
    SanyoLdAdPcm = WAVE_FORMAT_SANYO_LD_ADPCM,
    /// Sipro Lab Telecom Inc.
    SiprolabAceplnet = WAVE_FORMAT_SIPROLAB_ACEPLNET,
    /// Sipro Lab Telecom Inc.
    SiprolabAcelp4800 = WAVE_FORMAT_SIPROLAB_ACELP4800,
    /// Sipro Lab Telecom Inc.
    SiprolabAcelp8V3 = WAVE_FORMAT_SIPROLAB_ACELP8V3,
    /// Sipro Lab Telecom Inc.
    SiprolabG729 = WAVE_FORMAT_SIPROLAB_G729,
    /// Sipro Lab Telecom Inc.
    SiprolabG729A = WAVE_FORMAT_SIPROLAB_G729A,
    /// Sipro Lab Telecom Inc.
    SiprolabKelvin = WAVE_FORMAT_SIPROLAB_KELVIN,
    /// VoiceAge Corp.
    VoiceageAmr = WAVE_FORMAT_VOICEAGE_AMR,
    /// Dictaphone Corporation.
    G726Adpcm = WAVE_FORMAT_G726ADPCM,
    /// Dictaphone Corporation.
    DictaphoneCelp68 = WAVE_FORMAT_DICTAPHONE_CELP68,
    /// Dictaphone Corporation.
    DictaphoneCelp54 = WAVE_FORMAT_DICTAPHONE_CELP54,
    /// Qualcomm, Inc.
    QualcommPurevoice = WAVE_FORMAT_QUALCOMM_PUREVOICE,
    /// Qualcomm, Inc.
    QualcommHalfrate = WAVE_FORMAT_QUALCOMM_HALFRATE,
    /// Ring Zero Systems, Inc.
    Tubgsm = WAVE_FORMAT_TUBGSM,
    /// Microsoft Corporation.
    Msaudio1 = WAVE_FORMAT_MSAUDIO1,
    /// Microsoft Corporation.
    Wmaudio2 = WAVE_FORMAT_WMAUDIO2,
    /// Microsoft Corporation.
    Wmaudio3 = WAVE_FORMAT_WMAUDIO3,
    /// Microsoft Corporation.
    WmaudioLossless = WAVE_FORMAT_WMAUDIO_LOSSLESS,
    /// Microsoft Corporation.
    Wmaspdif = WAVE_FORMAT_WMASPDIF,
    /// Unisys Corp.
    UnisysNapAdPcm = WAVE_FORMAT_UNISYS_NAP_ADPCM,
    /// Unisys Corp.
    UnisysNapUlaw = WAVE_FORMAT_UNISYS_NAP_ULAW,
    /// Unisys Corp.
    UnisysNapAlaw = WAVE_FORMAT_UNISYS_NAP_ALAW,
    /// Unisys Corp.
    UnisysNap16K = WAVE_FORMAT_UNISYS_NAP_16K,
    /// SyCom Technologies.
    SycomAcmSyc008 = WAVE_FORMAT_SYCOM_ACM_SYC008,
    /// SyCom Technologies.
    SycomAcmSyc701G726L = WAVE_FORMAT_SYCOM_ACM_SYC701_G726L,
    /// SyCom Technologies.
    SycomAcmSyc701Celp54 = WAVE_FORMAT_SYCOM_ACM_SYC701_CELP54,
    /// SyCom Technologies.
    SycomAcmSyc701Celp68 = WAVE_FORMAT_SYCOM_ACM_SYC701_CELP68,
    /// Knowledge Adventure, Inc.
    KnowledgeAdventureAdPcm = WAVE_FORMAT_KNOWLEDGE_ADVENTURE_ADPCM,
    /// Fraunhofer IIS.
    FraunhoferIisMpeg2Aac = WAVE_FORMAT_FRAUNHOFER_IIS_MPEG2_AAC,
    /// Digital Theatre Systems, Inc.
    DtsDs = WAVE_FORMAT_DTS_DS,
    /// Creative Labs, Inc.
    CreativeAdPcm = WAVE_FORMAT_CREATIVE_ADPCM,
    /// Creative Labs, Inc.
    CreativeFastspeech8 = WAVE_FORMAT_CREATIVE_FASTSPEECH8,
    /// Creative Labs, Inc.
    CreativeFastspeech10 = WAVE_FORMAT_CREATIVE_FASTSPEECH10,
    /// UHER informatic GmbH.
    UherAdPcm = WAVE_FORMAT_UHER_ADPCM,
    /// Ulead Systems, Inc.
    UleadDvAudio = WAVE_FORMAT_ULEAD_DV_AUDIO,
    /// Ulead Systems, Inc.
    UleadDvAudio1 = WAVE_FORMAT_ULEAD_DV_AUDIO_1,
    /// Quarterdeck Corporation.
    Quarterdeck = WAVE_FORMAT_QUARTERDECK,
    /// I-link Worldwide.
    IlinkVc = WAVE_FORMAT_ILINK_VC,
    /// Aureal Semiconductor.
    RawSport = WAVE_FORMAT_RAW_SPORT,
    /// ESS Technology, Inc.
    EsstAc3 = WAVE_FORMAT_ESST_AC3,
    /// Generic format.
    GenericPassthru = WAVE_FORMAT_GENERIC_PASSTHRU,
    /// Interactive Products, Inc.
    IpiHsx = WAVE_FORMAT_IPI_HSX,
    /// Interactive Products, Inc.
    IpiRpelp = WAVE_FORMAT_IPI_RPELP,
    /// Consistent Software.
    Cs2 = WAVE_FORMAT_CS2,
    /// Sony Corp.
    SonyScx = WAVE_FORMAT_SONY_SCX,
    /// Sony Corp.
    SonyScy = WAVE_FORMAT_SONY_SCY,
    /// Sony Corp.
    SonyAtrac3 = WAVE_FORMAT_SONY_ATRAC3,
    /// Sony Corp.
    SonySpc = WAVE_FORMAT_SONY_SPC,
    /// Telum Inc.
    TelumAudio = WAVE_FORMAT_TELUM_AUDIO,
    /// Telum Inc.
    TelumIaAudio = WAVE_FORMAT_TELUM_IA_AUDIO,
    /// Norcom Electronics Corp.
    NorcomVoiceSystemsAdPcm = WAVE_FORMAT_NORCOM_VOICE_SYSTEMS_ADPCM,
    /// Fujitsu Corp.
    FmTownsSnd = WAVE_FORMAT_FM_TOWNS_SND,
    /// Micronas Semiconductors, Inc.
    Micronas = WAVE_FORMAT_MICRONAS,
    /// Micronas Semiconductors, Inc.
    MicronasCelp833 = WAVE_FORMAT_MICRONAS_CELP833,
    /// Brooktree Corporation.
    BtvDigital = WAVE_FORMAT_BTV_DIGITAL,
    /// Intel Corp.
    IntelMusicCoder = WAVE_FORMAT_INTEL_MUSIC_CODER,
    /// Ligos.
    IndeoAudio = WAVE_FORMAT_INDEO_AUDIO,
    /// QDesign Corporation.
    QdesignMusic = WAVE_FORMAT_QDESIGN_MUSIC,
    /// On2 Technologies.
    On2Vp7Audio = WAVE_FORMAT_ON2_VP7_AUDIO,
    /// On2 Technologies.
    On2Vp6Audio = WAVE_FORMAT_ON2_VP6_AUDIO,
    /// AT&T Labs, Inc.
    VmeVmpcm = WAVE_FORMAT_VME_VMPCM,
    /// AT&T Labs, Inc.
    Tpc = WAVE_FORMAT_TPC,
    /// Clearjump.
    LightwaveLossless = WAVE_FORMAT_LIGHTWAVE_LOSSLESS,
    /// Ing C. Olivetti & C., S.p.A.
    Oligsm = WAVE_FORMAT_OLIGSM,
    /// Ing C. Olivetti & C., S.p.A.
    Oliadpcm = WAVE_FORMAT_OLIADPCM,
    /// Ing C. Olivetti & C., S.p.A.
    Olicelp = WAVE_FORMAT_OLICELP,
    /// Ing C. Olivetti & C., S.p.A.
    Olisbc = WAVE_FORMAT_OLISBC,
    /// Ing C. Olivetti & C., S.p.A.
    Oliopr = WAVE_FORMAT_OLIOPR,
    /// Lernout & Hauspie.
    LhCodec = WAVE_FORMAT_LH_CODEC,
    /// Lernout & Hauspie.
    LhCodecCelp = WAVE_FORMAT_LH_CODEC_CELP,
    /// Lernout & Hauspie.
    LhCodecSbc8 = WAVE_FORMAT_LH_CODEC_SBC8,
    /// Lernout & Hauspie.
    LhCodecSbc12 = WAVE_FORMAT_LH_CODEC_SBC12,
    /// Lernout & Hauspie.
    LhCodecSbc16 = WAVE_FORMAT_LH_CODEC_SBC16,
    /// Norris Communications, Inc.
    Norris = WAVE_FORMAT_NORRIS,
    /// ISIAudio.
    Isiaudio2 = WAVE_FORMAT_ISIAUDIO_2,
    /// AT&T Labs, Inc.
    SoundspaceMusicompress = WAVE_FORMAT_SOUNDSPACE_MUSICOMPRESS,
    /// Microsoft Corporation.
    MpegAdtsAac = WAVE_FORMAT_MPEG_ADTS_AAC,
    /// Microsoft Corporation.
    MpegRawAac = WAVE_FORMAT_MPEG_RAW_AAC,
    /// Microsoft Corporation (MPEG-4 Audio Transport Streams (LOAS/LATM).
    MpegLoas = WAVE_FORMAT_MPEG_LOAS,
    /// Microsoft Corporation.
    NokiaMpegAdtsAac = WAVE_FORMAT_NOKIA_MPEG_ADTS_AAC,
    /// Microsoft Corporation.
    NokiaMpegRawAac = WAVE_FORMAT_NOKIA_MPEG_RAW_AAC,
    /// Microsoft Corporation.
    VodafoneMpegAdtsAac = WAVE_FORMAT_VODAFONE_MPEG_ADTS_AAC,
    /// Microsoft Corporation.
    VodafoneMpegRawAac = WAVE_FORMAT_VODAFONE_MPEG_RAW_AAC,
    /// Microsoft Corporation (MPEG-2 AAC or MPEG-4 HE-AAC v1/v2 streams with any payload (ADTS,
    /// ADIF, LOAS/LATM, RAW). Format block includes MP4 "audio specific config".
    MpegHeaac = WAVE_FORMAT_MPEG_HEAAC,
    /// Voxware Inc.
    Voxwarert24Speech = WAVE_FORMAT_VOXWARE_RT24_SPEECH,
    /// Sonic Foundry.
    SonicfoundryLossless = WAVE_FORMAT_SONICFOUNDRY_LOSSLESS,
    /// Innings Telecom Inc.
    InningsTelecomAdPcm = WAVE_FORMAT_INNINGS_TELECOM_ADPCM,
    /// Lucent Technologies.
    LucentSx8300P = WAVE_FORMAT_LUCENT_SX8300P,
    /// Lucent Technologies.
    LucentSx5363S = WAVE_FORMAT_LUCENT_SX5363S,
    /// CUSeeMe.
    Cuseeme = WAVE_FORMAT_CUSEEME,
    /// NTCSoft.
    NtcsoftAlf2CmAcm = WAVE_FORMAT_NTCSOFT_ALF2CM_ACM,
    /// FAST Multimedia AG.
    Dvm = WAVE_FORMAT_DVM,
    /// DTS 2 format.
    Dts2 = WAVE_FORMAT_DTS2,
    /// Make AVIs format.
    Makeavis = WAVE_FORMAT_MAKEAVIS,
    /// Divio, Inc.
    DivioMpeg4Aac = WAVE_FORMAT_DIVIO_MPEG4_AAC,
    /// Nokia.
    NokiaAdaptiveMultirate = WAVE_FORMAT_NOKIA_ADAPTIVE_MULTIRATE,
    /// Divio, Inc.
    DivioG726 = WAVE_FORMAT_DIVIO_G726,
    /// LEAD Technologies.
    LeadSpeech = WAVE_FORMAT_LEAD_SPEECH,
    /// LEAD Technologies.
    LeadVorbis = WAVE_FORMAT_LEAD_VORBIS,
    /// xiph.org.
    WavpackAudio = WAVE_FORMAT_WAVPACK_AUDIO,
    /// Apple Lossless.
    Alac = 0x6C61,
    /// Ogg Vorbis.
    OggVorbisMode1 = WAVE_FORMAT_OGG_VORBIS_MODE_1,
    /// Ogg Vorbis.
    OggVorbisMode2 = WAVE_FORMAT_OGG_VORBIS_MODE_2,
    /// Ogg Vorbis.
    OggVorbisMode3 = WAVE_FORMAT_OGG_VORBIS_MODE_3,
    /// Ogg Vorbis.
    OggVorbisMode1Plus = WAVE_FORMAT_OGG_VORBIS_MODE_1_PLUS,
    /// Ogg Vorbis.
    OggVorbisMode2Plus = WAVE_FORMAT_OGG_VORBIS_MODE_2_PLUS,
    /// Ogg Vorbis.
    OggVorbisMode3Plus = WAVE_FORMAT_OGG_VORBIS_MODE_3_PLUS,
    /// 3COM Corp.
    ThreeComNbx = WAVE_FORMAT_3COM_NBX,
    /// Opus.
    Opus = 0x704F,
    /// FAAD AAC format.
    FaadAac = WAVE_FORMAT_FAAD_AAC,
    /// AMR Narrowband.
    AmrNb = WAVE_FORMAT_AMR_NB,
    /// AMR Wideband.
    AmrWb = WAVE_FORMAT_AMR_WB,
    /// AMR Wideband Plus.
    AmrWp = WAVE_FORMAT_AMR_WP,
    /// GSMA/3GPP.
    GsmAmrCbr = WAVE_FORMAT_GSM_AMR_CBR,
    /// GSMA/3GPP.
    GsmAmrVbrSid = WAVE_FORMAT_GSM_AMR_VBR_SID,
    /// Comverse Infosys.
    ComverseInfosysG7231 = WAVE_FORMAT_COMVERSE_INFOSYS_G723_1,
    /// Comverse Infosys.
    ComverseInfosysAvqsbc = WAVE_FORMAT_COMVERSE_INFOSYS_AVQSBC,
    /// Comverse Infosys.
    ComverseInfosysSbc = WAVE_FORMAT_COMVERSE_INFOSYS_SBC,
    /// Symbol Technologies.
    SymbolG729A = WAVE_FORMAT_SYMBOL_G729_A,
    /// VoiceAge Corp.
    VoiceageAmrWb = WAVE_FORMAT_VOICEAGE_AMR_WB,
    /// Ingenient Technologies, Inc.
    IngenientG726 = WAVE_FORMAT_INGENIENT_G726,
    /// ISO/MPEG-4.
    Mpeg4Aac = WAVE_FORMAT_MPEG4_AAC,
    /// Encore Software.
    EncoreG726 = WAVE_FORMAT_ENCORE_G726,
    /// ZOLL Medical Corp.
    ZollAsao = WAVE_FORMAT_ZOLL_ASAO,
    /// xiph.org.
    SpeexVoice = WAVE_FORMAT_SPEEX_VOICE,
    /// Vianix LLC.
    VianixMasc = WAVE_FORMAT_VIANIX_MASC,
    /// Microsoft.
    Wm9SpectrumAnalyzer = WAVE_FORMAT_WM9_SPECTRUM_ANALYZER,
    /// Microsoft.
    WmfSpectrumAnayzer = WAVE_FORMAT_WMF_SPECTRUM_ANAYZER,
    /// GSM 610 format.
    Gsm610 = WAVE_FORMAT_GSM_610,
    /// GSM 620 format .
    Gsm620 = WAVE_FORMAT_GSM_620,
    /// GSM 660 format .
    Gsm660 = WAVE_FORMAT_GSM_660,
    /// GSM 690 format .
    Gsm690 = WAVE_FORMAT_GSM_690,
    /// GSM Adaptive Multirate format .
    GsmAdaptiveMultirateWb = WAVE_FORMAT_GSM_ADAPTIVE_MULTIRATE_WB,
    /// Polycom.
    PolycomG722 = WAVE_FORMAT_POLYCOM_G722,
    /// Polycom.
    PolycomG728 = WAVE_FORMAT_POLYCOM_G728,
    /// Polycom.
    PolycomG729A = WAVE_FORMAT_POLYCOM_G729_A,
    /// Polycom.
    PolycomSiren = WAVE_FORMAT_POLYCOM_SIREN,
    /// Global IP.
    GlobalIpIlbc = WAVE_FORMAT_GLOBAL_IP_ILBC,
    /// RadioTime.
    RadiotimeTimeShiftRadio = WAVE_FORMAT_RADIOTIME_TIME_SHIFT_RADIO,
    /// Nice Systems.
    NiceAca = WAVE_FORMAT_NICE_ACA,
    /// Nice Systems.
    NiceAdPcm = WAVE_FORMAT_NICE_ADPCM,
    /// Vocord Telecom.
    VocordG721 = WAVE_FORMAT_VOCORD_G721,
    /// Vocord Telecom.
    VocordG726 = WAVE_FORMAT_VOCORD_G726,
    /// Vocord Telecom.
    VocordG7221 = WAVE_FORMAT_VOCORD_G722_1,
    /// Vocord Telecom.
    VocordG728 = WAVE_FORMAT_VOCORD_G728,
    /// Vocord Telecom.
    VocordG729 = WAVE_FORMAT_VOCORD_G729,
    /// Vocord Telecom.
    VocordG729A = WAVE_FORMAT_VOCORD_G729_A,
    /// Vocord Telecom.
    VocordG7231 = WAVE_FORMAT_VOCORD_G723_1,
    /// Vocord Telecom.
    VocordLbc = WAVE_FORMAT_VOCORD_LBC,
    /// Nice Systems.
    NiceG728 = WAVE_FORMAT_NICE_G728,
    /// France Telecom.
    FraceTelecomG729 = WAVE_FORMAT_FRACE_TELECOM_G729,
    /// CODIAN.
    Codian = WAVE_FORMAT_CODIAN,
    /// flac.sourceforge.net.
    Flac = WAVE_FORMAT_FLAC,
});

/// Defines the format of waveform-audio data. Only format information common to all
/// waveform-audio data formats is included in this structure.
pub struct Format {
    /// Waveform-audio format type.
    pub format_tag: Tag,
    /// Number of channels in the waveform-audio data. Monaural data uses one
    /// channel and stereo data uses two channels.
    pub channels: u16,
    /// Sample rate, in samples per second (hertz). If `format_tag` is
    /// `Format::Pcm`, then common values for `samples_per_sec` are 8.0 kHz,
    /// 11.025 kHz, 22.05 kHz, and 44.1 kHz. For non-PCM formats, this member
    /// must be computed according to the manufacturer's specification of the
    /// format tag.
    pub samples_per_sec: u32,
    /// Required average data-transfer rate, in bytes per second, for the
    /// format tag. If `format_tag` is `Format::Pcm`, `avg_bytes_per_sec`
    /// should be equal to the product of `samples_per_sec` and `block_align`.
    /// For non-PCM formats, this member must be computed according to the
    /// manufacturer's specification of the format tag.
    pub avg_bytes_per_sec: u32,
    /// Block alignment, in bytes. The block alignment is the minimum atomic
    /// unit of data for the `format_tag` format type. If `format_tag` is
    /// `Format::Pcm`, `block_align` must be equal to the product of `channels`
    /// and `bits_per_sample` divided by 8 (bits per byte). For non-PCM
    /// formats, this member must be computed according to the manufacturer's
    /// specification of the format tag.
    ///
    /// The software will process a multiple of `block_align` bytes of data at
    /// a time. Data written to and read from a device must always start at the
    /// beginning of a block.
    pub block_align: u16,
    /// Bits per sample for the `format_tag` format type. If `format_tag` is
    /// `Format::Pcm`, then `bits_per_sample` should be equal to 8 or 16. For
    /// non-PCM formats, this member must be set according to the
    /// manufacturer's specification of the format tag.
    pub bits_per_sample: u16,
}

impl Format {
    /// Fill the format structure from the stream of a `.wav` file.
    pub fn from_wav_stream<S: Read + Seek>(file: &mut S, offset: u64) -> io::Result<Self> {
        file.seek(SeekFrom::Start(offset))?;

        Ok(Self {
            format_tag: file.read_u16()?.try_into().map_err(|tag| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("unknown format tag: {}", tag),
                )
            })?,
            channels: file.read_u16()?,
            samples_per_sec: file.read_u32()?,
            avg_bytes_per_sec: file.read_u32()?,
            block_align: file.read_u16()?,
            bits_per_sample: file.read_u16()?,
        })
    }

    pub(crate) fn c_struct(&self) -> WAVEFORMATEX {
        WAVEFORMATEX {
            wFormatTag: self.format_tag as u16,
            nChannels: self.channels,
            nSamplesPerSec: self.samples_per_sec,
            nAvgBytesPerSec: self.avg_bytes_per_sec,
            nBlockAlign: self.block_align,
            wBitsPerSample: self.bits_per_sample,
            cbSize: 0,
        }
    }
}
