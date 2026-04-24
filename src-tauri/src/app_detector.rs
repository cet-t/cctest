pub fn get_active_window() -> Result<String, Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        Ok("windows_app".to_string())
    }

    #[cfg(target_os = "macos")]
    {
        Ok("macos_app".to_string())
    }

    #[cfg(target_os = "linux")]
    {
        Ok("linux_app".to_string())
    }
}
