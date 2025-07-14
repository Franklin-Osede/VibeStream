use std::path::Path;
use std::process::{Command, Stdio};
use std::io::{Read, Write};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::music::domain::value_objects::{FileFormat, AudioQuality};

/// Audio transcoding configuration
#[derive(Debug, Clone)]
pub struct TranscodeConfig {
    pub input_format: FileFormat,
    pub output_format: FileFormat,
    pub quality: AudioQuality,
    pub bitrate: Option<u32>,
    pub sample_rate: Option<u32>,
    pub channels: Option<u16>,
}

/// Audio transcoder service
pub struct AudioTranscoder;

impl AudioTranscoder {
    /// Transcode audio file from one format to another
    pub async fn transcode_file(
        input_path: &Path,
        output_path: &Path,
        config: &TranscodeConfig,
    ) -> Result<(), AppError> {
        // Check if FFmpeg is available
        if !Self::is_ffmpeg_available().await {
            return Err(AppError::InternalError(
                "FFmpeg is not available. Please install FFmpeg to enable audio transcoding.".to_string()
            ));
        }

        // Build FFmpeg command
        let mut command = Command::new("ffmpeg");
        
        // Input options
        command.arg("-i").arg(input_path);
        
        // Output options based on format and quality
        Self::add_output_options(&mut command, config);
        
        // Output file
        command.arg(output_path);
        
        // Suppress output
        command.stdout(Stdio::null()).stderr(Stdio::null());

        // Execute transcoding
        let status = command.status()
            .map_err(|e| AppError::InternalError(format!("Failed to execute FFmpeg: {}", e)))?;

        if !status.success() {
            return Err(AppError::InternalError("Audio transcoding failed".to_string()));
        }

        Ok(())
    }

    /// Transcode audio data from bytes
    pub async fn transcode_bytes(
        input_data: &[u8],
        input_format: FileFormat,
        output_format: FileFormat,
        quality: AudioQuality,
    ) -> Result<Vec<u8>, AppError> {
        // Create temporary files
        let temp_dir = std::env::temp_dir();
        let input_file = temp_dir.join(format!("input_{}.{}", uuid::Uuid::new_v4(), Self::format_extension(input_format.clone())));
        let output_file = temp_dir.join(format!("output_{}.{}", uuid::Uuid::new_v4(), Self::format_extension(output_format.clone())));

        // Write input data to temporary file
        fs::write(&input_file, input_data).await
            .map_err(|e| AppError::InternalError(format!("Failed to write input file: {}", e)))?;

        // Transcode configuration
        let config = TranscodeConfig {
            input_format,
            output_format,
            quality,
            bitrate: None,
            sample_rate: None,
            channels: None,
        };

        // Transcode file
        Self::transcode_file(&input_file, &output_file, &config).await?;

        // Read output data
        let output_data = fs::read(&output_file).await
            .map_err(|e| AppError::InternalError(format!("Failed to read output file: {}", e)))?;

        // Clean up temporary files
        let _ = fs::remove_file(&input_file).await;
        let _ = fs::remove_file(&output_file).await;

        Ok(output_data)
    }

    /// Check if FFmpeg is available
    async fn is_ffmpeg_available() -> bool {
        Command::new("ffmpeg")
            .arg("-version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok()
    }

    /// Add output options to FFmpeg command based on format and quality
    fn add_output_options(command: &mut Command, config: &TranscodeConfig) {
        match config.output_format {
            FileFormat::Mp3 => {
                command.arg("-c:a").arg("libmp3lame");
                if let Some(bitrate) = config.bitrate {
                    command.arg("-b:a").arg(&format!("{}k", bitrate / 1000));
                } else {
                    let bitrate = match config.quality {
                        AudioQuality::High => "320",
                        AudioQuality::Medium => "192",
                        AudioQuality::Low => "128",
                        AudioQuality::Lossless => "320",
                    };
                    command.arg("-b:a").arg(&format!("{}k", bitrate));
                }
            }
            FileFormat::Aac => {
                command.arg("-c:a").arg("aac");
                if let Some(bitrate) = config.bitrate {
                    command.arg("-b:a").arg(&format!("{}k", bitrate / 1000));
                } else {
                    let bitrate = match config.quality {
                        AudioQuality::High => "256",
                        AudioQuality::Medium => "192",
                        AudioQuality::Low => "128",
                        AudioQuality::Lossless => "0",
                    };
                    command.arg("-b:a").arg(&format!("{}k", bitrate));
                }
            }
            FileFormat::Ogg => {
                command.arg("-c:a").arg("libvorbis");
                if let Some(bitrate) = config.bitrate {
                    command.arg("-b:a").arg(&format!("{}k", bitrate / 1000));
                } else {
                    let bitrate = match config.quality {
                        AudioQuality::High => "256",
                        AudioQuality::Medium => "192",
                        AudioQuality::Low => "128",
                        AudioQuality::Lossless => "0",
                    };
                    command.arg("-b:a").arg(&format!("{}k", bitrate));
                }
            }
            FileFormat::Flac => {
                command.arg("-c:a").arg("flac");
                command.arg("-compression_level").arg("8");
            }
            FileFormat::Wav => {
                command.arg("-c:a").arg("pcm_s16le");
            }
            FileFormat::M4a => {
                command.arg("-c:a").arg("aac");
                if let Some(bitrate) = config.bitrate {
                    command.arg("-b:a").arg(&format!("{}k", bitrate / 1000));
                } else {
                    let bitrate = match config.quality {
                        AudioQuality::High => "256",
                        AudioQuality::Medium => "192",
                        AudioQuality::Low => "128",
                        AudioQuality::Lossless => "0",
                    };
                    command.arg("-b:a").arg(&format!("{}k", bitrate));
                }
            }
        }

        // Add sample rate if specified
        if let Some(sample_rate) = config.sample_rate {
            command.arg("-ar").arg(&sample_rate.to_string());
        }

        // Add channels if specified
        if let Some(channels) = config.channels {
            command.arg("-ac").arg(&channels.to_string());
        }

        // Add quality settings
        match config.quality {
            AudioQuality::High => {
                command.arg("-q:a").arg("0"); // Best quality
            }
            AudioQuality::Medium => {
                command.arg("-q:a").arg("2"); // Good quality
            }
            AudioQuality::Low => {
                command.arg("-q:a").arg("5"); // Lower quality
            }
            AudioQuality::Lossless => {
                command.arg("-q:a").arg("0"); // Best quality for lossless
            }
        }
    }

    /// Get file extension for format
    fn format_extension(format: FileFormat) -> &'static str {
        match format {
            FileFormat::Mp3 => "mp3",
            FileFormat::Wav => "wav",
            FileFormat::Flac => "flac",
            FileFormat::Aac => "aac",
            FileFormat::Ogg => "ogg",
            FileFormat::M4a => "m4a",
        }
    }

    /// Get recommended bitrate for quality and format
    pub fn get_recommended_bitrate(quality: AudioQuality, format: FileFormat) -> u32 {
        match (quality, format) {
            (AudioQuality::High, FileFormat::Mp3) => 320000,
            (AudioQuality::High, FileFormat::Aac) => 256000,
            (AudioQuality::High, FileFormat::Ogg) => 256000,
            (AudioQuality::High, FileFormat::Flac) => 0, // Lossless
            (AudioQuality::High, FileFormat::Wav) => 0,  // Lossless
            (AudioQuality::Medium, FileFormat::Mp3) => 192000,
            (AudioQuality::Medium, FileFormat::Aac) => 192000,
            (AudioQuality::Medium, FileFormat::Ogg) => 192000,
            (AudioQuality::Medium, FileFormat::Flac) => 0, // Lossless
            (AudioQuality::Medium, FileFormat::Wav) => 0,  // Lossless
            (AudioQuality::Low, FileFormat::Mp3) => 128000,
            (AudioQuality::Low, FileFormat::Aac) => 128000,
            (AudioQuality::Low, FileFormat::Ogg) => 128000,
            (AudioQuality::Low, FileFormat::Flac) => 0, // Lossless
            (AudioQuality::Low, FileFormat::Wav) => 0,  // Lossless
            _ => 192000, // Default
        }
    }

    /// Get recommended sample rate for quality
    pub fn get_recommended_sample_rate(quality: AudioQuality) -> u32 {
        match quality {
            AudioQuality::High => 48000,
            AudioQuality::Medium => 44100,
            AudioQuality::Low => 22050,
            AudioQuality::Lossless => 48000,
        }
    }

    /// Get recommended channels for quality
    pub fn get_recommended_channels(quality: AudioQuality) -> u16 {
        match quality {
            AudioQuality::High => 2, // Stereo
            AudioQuality::Medium => 2, // Stereo
            AudioQuality::Low => 1,  // Mono
            AudioQuality::Lossless => 2, // Stereo
        }
    }

    /// Create optimal transcoding configuration
    pub fn create_optimal_config(
        input_format: FileFormat,
        output_format: FileFormat,
        quality: AudioQuality,
    ) -> TranscodeConfig {
        TranscodeConfig {
            input_format,
            output_format: output_format.clone(),
            quality: quality.clone(),
            bitrate: Some(Self::get_recommended_bitrate(quality.clone(), output_format.clone())),
            sample_rate: Some(Self::get_recommended_sample_rate(quality.clone())),
            channels: Some(Self::get_recommended_channels(quality)),
        }
    }

    /// Validate if transcoding is supported
    pub async fn is_transcoding_supported(input_format: FileFormat, output_format: FileFormat) -> bool {
        // Check if FFmpeg is available
        if !Self::is_ffmpeg_available().await {
            return false;
        }

        // Check if format conversion is supported
        match (input_format, output_format) {
            (FileFormat::Mp3, FileFormat::Aac) => true,
            (FileFormat::Mp3, FileFormat::Ogg) => true,
            (FileFormat::Mp3, FileFormat::Flac) => true,
            (FileFormat::Mp3, FileFormat::Wav) => true,
            (FileFormat::Aac, FileFormat::Mp3) => true,
            (FileFormat::Aac, FileFormat::Ogg) => true,
            (FileFormat::Aac, FileFormat::Flac) => true,
            (FileFormat::Aac, FileFormat::Wav) => true,
            (FileFormat::Ogg, FileFormat::Mp3) => true,
            (FileFormat::Ogg, FileFormat::Aac) => true,
            (FileFormat::Ogg, FileFormat::Flac) => true,
            (FileFormat::Ogg, FileFormat::Wav) => true,
            (FileFormat::Flac, FileFormat::Mp3) => true,
            (FileFormat::Flac, FileFormat::Aac) => true,
            (FileFormat::Flac, FileFormat::Ogg) => true,
            (FileFormat::Flac, FileFormat::Wav) => true,
            (FileFormat::Wav, FileFormat::Mp3) => true,
            (FileFormat::Wav, FileFormat::Aac) => true,
            (FileFormat::Wav, FileFormat::Ogg) => true,
            (FileFormat::Wav, FileFormat::Flac) => true,
            _ => false, // Same format or unsupported conversion
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_format_extension() {
        assert_eq!(AudioTranscoder::format_extension(FileFormat::Mp3), "mp3");
        assert_eq!(AudioTranscoder::format_extension(FileFormat::Wav), "wav");
        assert_eq!(AudioTranscoder::format_extension(FileFormat::Flac), "flac");
    }

    #[test]
    fn test_recommended_bitrate() {
        assert_eq!(AudioTranscoder::get_recommended_bitrate(AudioQuality::High, FileFormat::Mp3), 320000);
        assert_eq!(AudioTranscoder::get_recommended_bitrate(AudioQuality::Medium, FileFormat::Mp3), 192000);
        assert_eq!(AudioTranscoder::get_recommended_bitrate(AudioQuality::Low, FileFormat::Mp3), 128000);
    }

    #[test]
    fn test_recommended_sample_rate() {
        assert_eq!(AudioTranscoder::get_recommended_sample_rate(AudioQuality::High), 48000);
        assert_eq!(AudioTranscoder::get_recommended_sample_rate(AudioQuality::Medium), 44100);
        assert_eq!(AudioTranscoder::get_recommended_sample_rate(AudioQuality::Low), 22050);
    }

    #[test]
    fn test_recommended_channels() {
        assert_eq!(AudioTranscoder::get_recommended_channels(AudioQuality::High), 2);
        assert_eq!(AudioTranscoder::get_recommended_channels(AudioQuality::Medium), 2);
        assert_eq!(AudioTranscoder::get_recommended_channels(AudioQuality::Low), 1);
    }

    #[tokio::test]
    async fn test_optimal_config() {
        let config = AudioTranscoder::create_optimal_config(
            FileFormat::Mp3,
            FileFormat::Aac,
            AudioQuality::High,
        );

        assert_eq!(config.input_format, FileFormat::Mp3);
        assert_eq!(config.output_format, FileFormat::Aac);
        assert_eq!(config.quality, AudioQuality::High);
        assert_eq!(config.bitrate, Some(256000));
        assert_eq!(config.sample_rate, Some(48000));
        assert_eq!(config.channels, Some(2));
    }
} 