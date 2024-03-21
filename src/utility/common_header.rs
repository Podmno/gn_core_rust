/*

    Utility Common Header

*/


pub struct GUCommonFTPDevice {
    pub device_ftp_address: String,
    pub device_ftp_port: i32,
}

impl GUCommonFTPDevice {
    fn new() -> Self {
        Self {
            device_ftp_address: "".to_string(),
            device_ftp_port: 0
        }
    }
}
