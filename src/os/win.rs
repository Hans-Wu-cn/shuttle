
use winapi::um::winuser::GetSystemMetrics;

pub fn get_system_metrics() -> (i32,i32){
    use winapi::um::winuser::{SM_CXSCREEN, SM_CYSCREEN};
    unsafe {
        (
            GetSystemMetrics(SM_CXSCREEN)
            ,GetSystemMetrics(SM_CYSCREEN)
        )
    }
}


#[cfg(test)]
mod tests {
    use super::get_system_metrics;

    #[test]
    fn test_get_system_metrics(){
        let (x,y)=get_system_metrics();
        println!("{x}:{y}")
    }
}