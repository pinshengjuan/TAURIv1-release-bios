pub mod copy_files {
    use std::{
        fs,
        path::Path,
    };

    pub fn copy(from: String, destination: String) -> std::io::Result<()> {
        fs::copy(from, Path::new(&destination))?;
        Ok(())
    }
}

pub mod copy_cotents {
    use std::ffi::CString;
    use windows::{
            core::Error,
            Win32::{
                Foundation::{
                    HANDLE, HGLOBAL
                },
                System::{
                    DataExchange::{
                        OpenClipboard, EmptyClipboard, RegisterClipboardFormatW, SetClipboardData, CloseClipboard
                    },
                    Memory::{
                        GMEM_MOVEABLE,
                        GlobalAlloc, GlobalLock, GlobalUnlock
                    },
                }
            }
        };

    //
    // Refer: https://docs.microsoft.com/en-us/windows/win32/dataxchg/html-clipboard-format
    //
    pub fn set_clipboard_html(content: String) -> Result<HANDLE, Error> {
        let fragment = content;

        let dummy_zero = format!("{:>09}", 0);
        let marker_start = format!("Version:1.0\r\nStartHTML:{:>09}\r\nEndHTML:{:>09}\r\nStartFragment:{:>09}\r\nEndFragment:{:>09}\r\n", dummy_zero, dummy_zero, dummy_zero, dummy_zero);
        let html_start = marker_start.len();
        let mut html = String::new();
        html.push_str(r#"<!DOCTYPE><HTML><HEAD></HEAD><BODY><!-- StartFragment -->"#);
        html.push_str(&fragment);
        html.push_str(r#"<!-- EndFragment --></BODY></HTML>"#);

        let fragment_start = html.find(&fragment).unwrap_or(0);
        let fragment_end = fragment_start + fragment.len();
    
        let prefix = format!("Version:1.0\r\nStartHTML:{:>09}\r\nEndHTML:{:>09}\r\nStartFragment:{:>09}\r\nEndFragment:{:>09}\r\n", html_start, html_start+html.len(), html_start+fragment_start, html_start+fragment_end);

        let mut document = String::new();
        document.push_str(&format!("{}{}", prefix, html));

        let cstring = CString::new(document).unwrap();
        let cstring = cstring.as_bytes_with_nul();
    
        // 1. Open Clipboard
        // 2. Empty Clipboard
        // 3. Register Format
        // 4. Set Clipboard
        // 5. Close Clipboard
    
        let handle = unsafe {
            // Open Clipboard
            let open_clipboard = OpenClipboard(None);
            if open_clipboard.is_err() {
                return Err(open_clipboard.unwrap_err());
            }

            // Empty Clipboard
            let empty_clipboard = EmptyClipboard();
            if empty_clipboard.is_err() {
                return Err(empty_clipboard.unwrap_err());
            }

            // Register Format
            #[allow(non_snake_case)]
            let CF_HTML;
            let format_name: Vec<u16> = "HTML Format\0".encode_utf16().collect();
            let pcwstr = windows::core::PCWSTR(format_name.as_ptr() as *const u16);
            let register_clipboard_format = RegisterClipboardFormatW(pcwstr);
            CF_HTML = register_clipboard_format;

            // Set Clipboard
            let mem_alloc: HGLOBAL = GlobalAlloc(GMEM_MOVEABLE, cstring.len() * std::mem::size_of::<u16>())?;
            let mem_lock = GlobalLock(mem_alloc);
            std::ptr::copy_nonoverlapping(cstring.as_ptr(), mem_lock as *mut u8, cstring.len());
            let _ = GlobalUnlock(mem_alloc);
            let handle = HANDLE(mem_alloc.0 as isize);

            let set_clipboard_data = SetClipboardData(CF_HTML, handle);
            if set_clipboard_data.is_err() {
                return Err(set_clipboard_data.unwrap_err());
            }

            // Close Clipboard
            let close_clipboard = CloseClipboard();
            if close_clipboard.is_err() {
                return Err(close_clipboard.unwrap_err());
            }
            handle
        };

        Ok(handle)
    }
}