/*

    Common FTP Function

*/

use crate::utility::common_header::GUCommonFTPDevice;

pub struct GUCommonFTPClient {

    server_info: GUCommonFTPDevice
}

impl GUCommonFTPClient {
    fn new() -> Self {
        Self {
            server_info: todo!(),
        }
    }

    /// stup with GUCommonFTPDevice struct
    fn setup_server_info(device_info: GUCommonFTPDevice) {
        print!("Common FTP > setup_server_info : target FTP Address > {}",device_info.device_ftp_address)


    }

    fn test_connection() {

    }
}
