use crate::sbi::shutdown;
use core::panic::PanicInfo;

/// 错误处理
/// 
/// 发生 panic 是进行结果处理
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("\x1b[1;31mpanic: '{}'\x1b[0m", info.message().unwrap());
    println!("!TEST FINISH!");
    shutdown()
}

/// 终止程序
/// 
/// abort
#[no_mangle]
extern "C" fn abort() -> ! {
    panic!("abort()")
}
