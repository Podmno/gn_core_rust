/*

    Common FTP Function

*/

use crate::utility::common_header::GUCommonFTPDevice;

pub struct GUCommonFTPClient {

    server_info: Option<GUCommonFTPDevice>
}

impl GUCommonFTPClient {
    fn new() -> Self {
        Self {
            server_info: None
        }
    }

    /// stup with GUCommonFTPDevice struct
    fn setup_server_info(&mut self,device_info: GUCommonFTPDevice) {
        print!("Common FTP > setup_server_info : target FTP Address > {}",device_info.device_ftp_address);
        self.server_info = Some(device_info);
    }

    fn test_connection(&self) {

    }

    fn get_file_list() {

    }

    fn get_file_info() {
        
    }
}
