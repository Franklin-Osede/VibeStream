use std::path::Path;
use std::process::Command;
use tokio::fs;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::bounded_contexts::p2p::domain::entities::video_stream::VideoQuality;

/// Video transcoder for P2P streaming (Mock implementation)
/// This uses external FFmpeg process instead of Rust bindings
pub struct VideoTranscoder {
    ffmpeg_path: String,
    output_dir: String,
    max_concurrent_jobs: usize,
    active_jobs: std::collections::HashMap<String, TranscodingJob>,
}

/// Transcoding job
#[derive(Debug, Clone)]
pub struct TranscodingJob {
    pub id: String,
    pub input_path: String,
    pub output_paths: Vec<String>,
    pub qualities: Vec<VideoQuality>,
    pub status: TranscodingStatus,
    pub progress: f32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Transcoding status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranscodingStatus {
    Pending,
    Processing,
    Completed,
    Failed(String),
}

/// Video format
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VideoFormat {
    MP4,
    WebM,
    HLS,
    DASH,
}

/// Transcoding configuration
#[derive(Debug, Clone)]
pub struct TranscodingConfig {
    pub input_path: String,
    pub output_dir: String,
    pub qualities: Vec<VideoQuality>,
    pub format: VideoFormat,
    pub enable_thumbnails: bool,
    pub enable_metadata: bool,
    pub segment_duration: u32, // seconds
}

impl VideoTranscoder {
    pub fn new(ffmpeg_path: String, output_dir: String) -> Self {
        Self {
            ffmpeg_path,
            output_dir,
            max_concurrent_jobs: 4,
            active_jobs: std::collections::HashMap::new(),
        }
    }

    /// Start transcoding a video
    pub async fn transcode_video(&mut self, config: TranscodingConfig) -> Result<String, String> {
        let job_id = Uuid::new_v4().to_string();
        
        // Validate input file
        if !Path::new(&config.input_path).exists() {
            return Err("Input file does not exist".to_string());
        }

        // Create output directory
        fs::create_dir_all(&config.output_dir).await
            .map_err(|e| format!("Failed to create output directory: {}", e))?;

        // Create transcoding job
        let job = TranscodingJob {
            id: job_id.clone(),
            input_path: config.input_path.clone(),
            output_paths: Vec::new(),
            qualities: config.qualities.clone(),
            status: TranscodingStatus::Pending,
            progress: 0.0,
            created_at: chrono::Utc::now(),
            started_at: None,
            completed_at: None,
        };

        self.active_jobs.insert(job_id.clone(), job);

        // Start transcoding in background
        let transcoder = self.clone_for_async();
        tokio::spawn(async move {
            let _ = transcoder.process_transcoding_job(job_id, config).await;
        });

        Ok(job_id)
    }

    /// Process a transcoding job
    async fn process_transcoding_job(self, job_id: String, config: TranscodingConfig) -> Result<(), String> {
        // Update job status
        if let Some(job) = self.active_jobs.get_mut(&job_id) {
            job.status = TranscodingStatus::Processing;
            job.started_at = Some(chrono::Utc::now());
        }

        println!("🎬 Starting transcoding job: {}", job_id);

        // Generate output paths for each quality
        let mut output_paths = Vec::new();
        for quality in &config.qualities {
            let output_path = self.generate_output_path(&config.output_dir, &job_id, quality, &config.format);
            output_paths.push(output_path.clone());
        }

        // Transcode for each quality
        for (i, quality) in config.qualities.iter().enumerate() {
            let output_path = &output_paths[i];
            
            match self.transcode_single_quality(&config.input_path, output_path, quality, &config.format).await {
                Ok(_) => {
                    println!("✅ Transcoding completed for quality: {:?}", quality);
                    
                    // Update progress
                    if let Some(job) = self.active_jobs.get_mut(&job_id) {
                        job.progress = (i + 1) as f32 / config.qualities.len() as f32;
                        job.output_paths.push(output_path.clone());
                    }
                }
                Err(e) => {
                    println!("❌ Transcoding failed for quality {:?}: {}", quality, e);
                    
                    if let Some(job) = self.active_jobs.get_mut(&job_id) {
                        job.status = TranscodingStatus::Failed(e);
                    }
                    return Err(format!("Transcoding failed for quality {:?}: {}", quality, e));
                }
            }
        }

        // Generate thumbnails if enabled
        if config.enable_thumbnails {
            if let Err(e) = self.generate_thumbnails(&config.input_path, &config.output_dir, &job_id).await {
                println!("⚠️ Failed to generate thumbnails: {}", e);
            }
        }

        // Update job status
        if let Some(job) = self.active_jobs.get_mut(&job_id) {
            job.status = TranscodingStatus::Completed;
            job.progress = 1.0;
            job.completed_at = Some(chrono::Utc::now());
        }

        println!("🎉 Transcoding job completed: {}", job_id);
        Ok(())
    }

    /// Transcode video for a single quality
    async fn transcode_single_quality(
        &self,
        input_path: &str,
        output_path: &str,
        quality: &VideoQuality,
        format: &VideoFormat,
    ) -> Result<(), String> {
        let (width, height) = quality.resolution();
        let bitrate = quality.bitrate();
        
        let mut command = Command::new(&self.ffmpeg_path);
        
        // Input
        command.arg("-i").arg(input_path);
        
        // Video codec and settings
        match format {
            VideoFormat::MP4 => {
                command
                    .arg("-c:v").arg("libx264")
                    .arg("-preset").arg("medium")
                    .arg("-crf").arg("23");
            }
            VideoFormat::WebM => {
                command
                    .arg("-c:v").arg("libvpx-vp9")
                    .arg("-crf").arg("30")
                    .arg("-b:v").arg("0");
            }
            VideoFormat::HLS => {
                command
                    .arg("-c:v").arg("libx264")
                    .arg("-preset").arg("medium")
                    .arg("-crf").arg("23")
                    .arg("-hls_time").arg("6")
                    .arg("-hls_list_size").arg("0")
                    .arg("-hls_segment_filename").arg(&format!("{}/segment_%03d.ts", output_path));
            }
            VideoFormat::DASH => {
                command
                    .arg("-c:v").arg("libx264")
                    .arg("-preset").arg("medium")
                    .arg("-crf").arg("23")
                    .arg("-f").arg("dash")
                    .arg("-seg_duration").arg("6");
            }
        }
        
        // Resolution and bitrate
        command
            .arg("-vf").arg(&format!("scale={}:{}", width, height))
            .arg("-b:v").arg(&format!("{}k", bitrate / 1000))
            .arg("-maxrate").arg(&format!("{}k", bitrate / 1000))
            .arg("-bufsize").arg(&format!("{}k", bitrate / 500));
        
        // Audio
        command
            .arg("-c:a").arg("aac")
            .arg("-b:a").arg("128k");
        
        // Output
        command.arg(output_path);
        
        // Execute command
        let output = command.output()
            .map_err(|e| format!("Failed to execute ffmpeg: {}", e))?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("FFmpeg failed: {}", error));
        }
        
        Ok(())
    }

    /// Generate thumbnails
    async fn generate_thumbnails(&self, input_path: &str, output_dir: &str, job_id: &str) -> Result<(), String> {
        let thumbnail_dir = format!("{}/thumbnails", output_dir);
        fs::create_dir_all(&thumbnail_dir).await
            .map_err(|e| format!("Failed to create thumbnail directory: {}", e))?;

        // Generate thumbnail at 10 seconds
        let thumbnail_path = format!("{}/{}_thumb.jpg", thumbnail_dir, job_id);
        
        let mut command = Command::new(&self.ffmpeg_path);
        command
            .arg("-i").arg(input_path)
            .arg("-ss").arg("10")
            .arg("-vframes").arg("1")
            .arg("-vf").arg("scale=320:180")
            .arg("-q:v").arg("2")
            .arg(&thumbnail_path);
        
        let output = command.output()
            .map_err(|e| format!("Failed to execute ffmpeg for thumbnail: {}", e))?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("FFmpeg thumbnail generation failed: {}", error));
        }
        
        Ok(())
    }

    /// Generate output path
    fn generate_output_path(&self, output_dir: &str, job_id: &str, quality: &VideoQuality, format: &VideoFormat) -> String {
        let (width, height) = quality.resolution();
        let extension = match format {
            VideoFormat::MP4 => "mp4",
            VideoFormat::WebM => "webm",
            VideoFormat::HLS => "m3u8",
            VideoFormat::DASH => "mpd",
        };
        
        format!("{}/{}_{}x{}.{}", output_dir, job_id, width, height, extension)
    }

    /// Get job status
    pub fn get_job_status(&self, job_id: &str) -> Option<&TranscodingJob> {
        self.active_jobs.get(job_id)
    }

    /// Get all active jobs
    pub fn get_active_jobs(&self) -> Vec<&TranscodingJob> {
        self.active_jobs.values().collect()
    }

    /// Cancel a job
    pub fn cancel_job(&mut self, job_id: &str) -> Result<(), String> {
        if let Some(job) = self.active_jobs.get_mut(job_id) {
            job.status = TranscodingStatus::Failed("Cancelled by user".to_string());
            Ok(())
        } else {
            Err("Job not found".to_string())
        }
    }

    /// Clone for async operations
    fn clone_for_async(&self) -> Self {
        Self {
            ffmpeg_path: self.ffmpeg_path.clone(),
            output_dir: self.output_dir.clone(),
            max_concurrent_jobs: self.max_concurrent_jobs,
            active_jobs: self.active_jobs.clone(),
        }
    }
}

/// Video metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub duration: f64,
    pub width: u32,
    pub height: u32,
    pub bitrate: u32,
    pub frame_rate: f32,
    pub codec: String,
    pub format: String,
}

impl VideoTranscoder {
    /// Extract video metadata
    pub async fn extract_metadata(&self, input_path: &str) -> Result<VideoMetadata, String> {
        let mut command = Command::new(&self.ffmpeg_path);
        command
            .arg("-i").arg(input_path)
            .arg("-f").arg("null")
            .arg("-");
        
        let output = command.output()
            .map_err(|e| format!("Failed to execute ffmpeg: {}", e))?;
        
        // Parse ffmpeg output to extract metadata
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        // This is a simplified parser - in production you'd want a more robust solution
        let duration = self.extract_duration(&stderr)?;
        let (width, height) = self.extract_resolution(&stderr)?;
        let bitrate = self.extract_bitrate(&stderr)?;
        let frame_rate = self.extract_frame_rate(&stderr)?;
        
        Ok(VideoMetadata {
            duration,
            width,
            height,
            bitrate,
            frame_rate,
            codec: "h264".to_string(), // Default assumption
            format: "mp4".to_string(), // Default assumption
        })
    }

    fn extract_duration(&self, output: &str) -> Result<f64, String> {
        // Look for "Duration: 00:01:23.45" pattern
        if let Some(duration_str) = output.lines()
            .find(|line| line.contains("Duration:"))
            .and_then(|line| line.split("Duration:").nth(1))
        {
            let duration_str = duration_str.trim().split_whitespace().next().unwrap_or("");
            // Parse HH:MM:SS.ms format
            let parts: Vec<&str> = duration_str.split(':').collect();
            if parts.len() >= 3 {
                let hours: f64 = parts[0].parse().unwrap_or(0.0);
                let minutes: f64 = parts[1].parse().unwrap_or(0.0);
                let seconds: f64 = parts[2].parse().unwrap_or(0.0);
                return Ok(hours * 3600.0 + minutes * 60.0 + seconds);
            }
        }
        Err("Could not extract duration".to_string())
    }

    fn extract_resolution(&self, output: &str) -> Result<(u32, u32), String> {
        // Look for "1920x1080" pattern
        if let Some(resolution_str) = output.lines()
            .find(|line| line.contains("x") && line.chars().any(|c| c.is_digit(10)))
            .and_then(|line| line.split_whitespace().find(|word| word.contains("x")))
        {
            let parts: Vec<&str> = resolution_str.split('x').collect();
            if parts.len() == 2 {
                let width: u32 = parts[0].parse().unwrap_or(0);
                let height: u32 = parts[1].parse().unwrap_or(0);
                return Ok((width, height));
            }
        }
        Err("Could not extract resolution".to_string())
    }

    fn extract_bitrate(&self, output: &str) -> Result<u32, String> {
        // Look for bitrate in kb/s
        if let Some(bitrate_str) = output.lines()
            .find(|line| line.contains("bitrate:"))
            .and_then(|line| line.split("bitrate:").nth(1))
        {
            if let Some(kbps_str) = bitrate_str.split_whitespace().find(|word| word.ends_with("kb/s")) {
                if let Ok(kbps) = kbps_str.trim_end_matches("kb/s").parse::<u32>() {
                    return Ok(kbps * 1000); // Convert to bps
                }
            }
        }
        Ok(0) // Default to 0 if not found
    }

    fn extract_frame_rate(&self, output: &str) -> Result<f32, String> {
        // Look for fps pattern
        if let Some(fps_str) = output.lines()
            .find(|line| line.contains("fps"))
            .and_then(|line| line.split_whitespace().find(|word| word.ends_with("fps")))
        {
            if let Ok(fps) = fps_str.trim_end_matches("fps").parse::<f32>() {
                return Ok(fps);
            }
        }
        Ok(30.0) // Default to 30 fps
    }
} 