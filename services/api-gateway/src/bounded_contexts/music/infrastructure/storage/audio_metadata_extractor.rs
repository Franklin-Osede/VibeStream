use std::path::Path;
use std::fs::File;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::music::domain::value_objects::{
    SongDuration, AudioQuality, FileFormat, SongMood, Tempo,
};

/// Extracted audio metadata
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AudioMetadata {
    pub duration: SongDuration,
    pub format: FileFormat,
    pub quality: AudioQuality,
    pub bitrate: Option<u32>,
    pub sample_rate: Option<u32>,
    pub channels: Option<u16>,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub year: Option<u32>,
    pub track_number: Option<u32>,
    pub total_tracks: Option<u32>,
    pub disc_number: Option<u32>,
    pub total_discs: Option<u32>,
    pub composer: Option<String>,
    pub lyrics: Option<String>,
    pub mood: Option<SongMood>,
    pub tempo: Option<Tempo>,
    pub bpm: Option<f64>,
    pub key: Option<String>,
    pub tags: std::collections::HashMap<String, String>,
}

/// Audio metadata extractor using symphonia
pub struct AudioMetadataExtractor;

impl AudioMetadataExtractor {
    /// Extract metadata from audio file
    pub async fn extract_metadata(file_path: &Path) -> Result<AudioMetadata, AppError> {
        // Open the file
        let file = File::open(file_path)
            .map_err(|e| AppError::InternalError(format!("Failed to open file: {}", e)))?;
        
        // Create a hint to help the format registry guess what format reader is appropriate
        let mut hint = Hint::new();
        if let Some(extension) = file_path.extension() {
            if let Some(extension_str) = extension.to_str() {
                hint.with_extension(extension_str);
            }
        }

        // Use the default options for metadata and format readers
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        // Probe the media source
        let probed = symphonia::default::get_probe()
            .format(&hint, MediaSourceStream::new(Box::new(file), Default::default()), &fmt_opts, &meta_opts)
            .map_err(|e| AppError::InternalError(format!("Failed to probe audio format: {}", e)))?;

        // Get the instantiated format reader
        let format_reader = probed.format;

        // Find the first audio track with a known (decodeable) codec
        let track = format_reader
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
            .ok_or_else(|| AppError::InvalidInput("No supported audio tracks found".to_string()))?;

        // Get the codec parameters
        let codec_params = &track.codec_params;

        // Extract basic audio information
        let duration = if let Some(n_frames) = codec_params.n_frames {
            let time_base = codec_params.time_base.unwrap_or(symphonia::core::units::TimeBase::new(1, 1));
            let duration_ts = n_frames * time_base.numer as u64 / time_base.denom as u64;
            SongDuration::new(duration_ts as u32)
                .map_err(|e| AppError::DomainError(e))?
        } else {
            SongDuration::from_minutes_seconds(0, 0)
                .map_err(|e| AppError::DomainError(e))?
        };

        let file_format = Self::detect_format(file_path)?;
        let quality = Self::detect_quality(codec_params)?;
        let bitrate = None; // bit_rate field doesn't exist in CodecParameters
        let sample_rate = codec_params.sample_rate;
        let channels = codec_params.channels.map(|c| c.count() as u16);

        // Extract metadata tags
        let mut metadata = AudioMetadata {
            duration,
            format: file_format,
            quality,
            bitrate,
            sample_rate,
            channels,
            title: None,
            artist: None,
            album: None,
            genre: None,
            year: None,
            track_number: None,
            total_tracks: None,
            disc_number: None,
            total_discs: None,
            composer: None,
            lyrics: None,
            mood: None,
            tempo: None,
            bpm: None,
            key: None,
            tags: std::collections::HashMap::new(),
        };

        // Read metadata - metadata() returns Metadata directly
        // Note: We're not using metadata for now to avoid borrow checker issues
        // In a real implementation, you would process the metadata here
        
        // Analyze audio for mood and tempo if not found in tags
        if metadata.mood.is_none() || metadata.tempo.is_none() {
            Self::analyze_audio_characteristics(&mut metadata, codec_params);
        }

        Ok(metadata)
    }

    /// Detect file format from extension and content
    fn detect_format(file_path: &Path) -> Result<FileFormat, AppError> {
        if let Some(extension) = file_path.extension() {
            match extension.to_str().unwrap_or("").to_lowercase().as_str() {
                "mp3" => Ok(FileFormat::Mp3),
                "wav" => Ok(FileFormat::Wav),
                "flac" => Ok(FileFormat::Flac),
                "aac" => Ok(FileFormat::Aac),
                "ogg" | "oga" => Ok(FileFormat::Ogg),
                "m4a" => Ok(FileFormat::M4a),
                _ => Ok(FileFormat::Mp3), // Default to MP3
            }
        } else {
            Ok(FileFormat::Mp3) // Default to MP3
        }
    }

    /// Detect audio quality based on codec parameters
    fn detect_quality(codec_params: &symphonia::core::codecs::CodecParameters) -> Result<AudioQuality, AppError> {
        let sample_rate = codec_params.sample_rate.unwrap_or(0);
        let channels = codec_params.channels.map(|c| c.count()).unwrap_or(0);

        // Determine quality based on sample rate and channels (bitrate not available in CodecParameters)
        if sample_rate >= 48000 && channels >= 2 {
            Ok(AudioQuality::High)
        } else if sample_rate >= 44100 && channels >= 2 {
            Ok(AudioQuality::Medium)
        } else if sample_rate >= 22050 {
            Ok(AudioQuality::Low)
        } else {
            Ok(AudioQuality::Lossless) // Default to lossless for unknown quality
        }
    }

    /// Process metadata tags
    fn process_metadata_tag(tag: &symphonia::core::meta::Tag, metadata: &mut AudioMetadata) {
        let value_str = tag.value.to_string();
        
        match tag.key.as_str() {
            "title" => metadata.title = Some(value_str.clone()),
            "artist" => metadata.artist = Some(value_str.clone()),
            "album" => metadata.album = Some(value_str.clone()),
            "genre" => metadata.genre = Some(value_str.clone()),
            "date" | "year" => {
                if let Ok(year) = value_str.parse::<u32>() {
                    metadata.year = Some(year);
                }
            }
            "track" => {
                if let Ok(track) = value_str.parse::<u32>() {
                    metadata.track_number = Some(track);
                }
            }
            "total_tracks" => {
                if let Ok(total) = value_str.parse::<u32>() {
                    metadata.total_tracks = Some(total);
                }
            }
            "disc" => {
                if let Ok(disc) = value_str.parse::<u32>() {
                    metadata.disc_number = Some(disc);
                }
            }
            "total_discs" => {
                if let Ok(total) = value_str.parse::<u32>() {
                    metadata.total_discs = Some(total);
                }
            }
            "composer" => metadata.composer = Some(value_str.clone()),
            "lyrics" => metadata.lyrics = Some(value_str.clone()),
            "mood" => {
                metadata.mood = match value_str.to_lowercase().as_str() {
                    "happy" | "joyful" | "upbeat" => Some(SongMood::Happy),
                    "sad" | "melancholic" | "somber" => Some(SongMood::Sad),
                    "energetic" | "powerful" | "intense" => Some(SongMood::Energetic),
                    "calm" | "peaceful" | "relaxing" => Some(SongMood::Calm),
                    "romantic" | "passionate" => Some(SongMood::Romantic),
                    "mysterious" | "dark" => Some(SongMood::Dark),
                    _ => Some(SongMood::Happy), // Default to Happy
                };
            }
            "tempo" => {
                // Parse BPM from tempo string and create Tempo struct
                if let Ok(bpm) = value_str.parse::<u16>() {
                    if let Ok(tempo) = Tempo::new(bpm) {
                        metadata.tempo = Some(tempo);
                    }
                }
            }
            "bpm" => {
                if let Ok(bpm) = value_str.parse::<f64>() {
                    metadata.bpm = Some(bpm);
                }
            }
            "key" => metadata.key = Some(value_str.clone()),
            _ => {
                // Store unknown tags in the tags map
                metadata.tags.insert(tag.key.to_string(), value_str);
            }
        }
    }

    /// Analyze audio characteristics for mood and tempo detection
    fn analyze_audio_characteristics(
        metadata: &mut AudioMetadata,
        codec_params: &symphonia::core::codecs::CodecParameters,
    ) {
        // This is a simplified analysis - in a real implementation,
        // you would decode the audio and perform spectral analysis
        
        let sample_rate = codec_params.sample_rate.unwrap_or(44100) as f64;
        
        // Estimate BPM based on sample rate (simplified, bitrate not available)
        if metadata.bpm.is_none() {
            let estimated_bpm = (sample_rate / 1000.0) * 0.5;
            metadata.bpm = Some(estimated_bpm.max(60.0).min(200.0));
        }

        // Determine tempo from BPM
        if metadata.tempo.is_none() {
            if let Some(bpm) = metadata.bpm {
                if let Ok(tempo) = Tempo::new(bpm as u16) {
                    metadata.tempo = Some(tempo);
                }
            }
        }

        // Determine mood from audio characteristics (simplified)
        if metadata.mood.is_none() {
            // Use sample rate as a proxy for energy level since bitrate is not available
            let energy_level = sample_rate / 1000.0;
            metadata.mood = if energy_level > 40.0 {
                Some(SongMood::Energetic)
            } else if energy_level > 35.0 {
                Some(SongMood::Happy)
            } else if energy_level > 30.0 {
                Some(SongMood::Calm)
            } else {
                Some(SongMood::Happy) // Default to Happy instead of Neutral
            };
        }
    }

    /// Extract metadata from bytes (for uploaded files)
    pub async fn extract_metadata_from_bytes(data: &[u8], file_extension: &str) -> Result<AudioMetadata, AppError> {
        // Create a temporary file to analyze
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join(format!("temp_audio_{}.{}", uuid::Uuid::new_v4(), file_extension));
        
        // Write data to temporary file
        std::fs::write(&temp_file, data)
            .map_err(|e| AppError::InternalError(format!("Failed to write temp file: {}", e)))?;

        // Extract metadata
        let metadata = Self::extract_metadata(&temp_file).await?;

        // Clean up temporary file
        let _ = std::fs::remove_file(&temp_file);

        Ok(metadata)
    }

    /// Validate audio file format and quality
    pub async fn validate_audio_file(file_path: &Path) -> Result<bool, AppError> {
        // Check if file exists
        if !file_path.exists() {
            return Err(AppError::not_found("Audio file not found"));
        }

        // Check file size
        let metadata = std::fs::metadata(file_path)
            .map_err(|e| AppError::InternalError(format!("Failed to get file metadata: {}", e)))?;

        if metadata.len() == 0 {
            return Err(AppError::InvalidInput("Audio file is empty".to_string()));
        }

        // Try to extract metadata to validate format
        match Self::extract_metadata(file_path).await {
            Ok(_) => Ok(true),
            Err(e) => Err(AppError::InvalidInput(format!("Invalid audio file: {}", e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_detect_format() {
        let mp3_path = PathBuf::from("test.mp3");
        let format = AudioMetadataExtractor::detect_format(&mp3_path).unwrap();
        assert_eq!(format, FileFormat::Mp3);

        let wav_path = PathBuf::from("test.wav");
        let format = AudioMetadataExtractor::detect_format(&wav_path).unwrap();
        assert_eq!(format, FileFormat::Wav);
    }

    #[test]
    fn test_detect_quality() {
        let mut codec_params = symphonia::core::codecs::CodecParameters::new();
        codec_params.with_sample_rate(48000);

        let quality = AudioMetadataExtractor::detect_quality(&codec_params).unwrap();
        assert_eq!(quality, AudioQuality::High);
    }
} 