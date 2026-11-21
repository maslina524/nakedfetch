use winapi::win32::processthreadsapi;

pub fn terminate_self(code: i32) -> ! {
    processthreadsapi::ExitProcess(code as u32)
}
