use log::info;

pub fn wait_for_debugger() {
    if cfg!(windows) {
        loop {
            if unsafe { windows::Win32::System::Diagnostics::Debug::IsDebuggerPresent().as_bool() }
            {
                info!("Debugger found.");
                break;
            } else {
                info!("Waiting for debugger.");
                std::thread::sleep(std::time::Duration::from_secs(5));
            }
        }
    }
}
