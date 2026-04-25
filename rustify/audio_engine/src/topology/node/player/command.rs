/// Enum representing commands for controlling audio playback.
pub enum AudioCommand {
    /// Play a new audio file specified by the path.
    Play(String),
    /// Pause the current audio playback.
    Pause,
    /// Resume the paused audio playback.
    Resume,
    /// Stop the current audio playback.
    Stop,
    /// Set the playback volume (0.0 to 1.0).
    SetVolume(f32),
}