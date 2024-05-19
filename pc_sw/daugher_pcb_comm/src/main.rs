
pub mod agw;
use agw::audio::AudioCfgSettings;
use custom_display_format::CDMIsoTp;
use futures_util::StreamExt;
use w211_can::{canb::EZS_A1, canbus::{frame_to_u64, CanBus}, tokio_socketcan::CANFilter};
pub mod custom_display_format;

#[tokio::main]
async fn main() {
    env_logger::init();
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
    let can_name = CanBus::B.get_net_name().to_string(); // Runs on bus B
    let vlad = CDMIsoTp::new(&rt, can_name.clone());

    let audio_page_settings = AudioCfgSettings {
        auto_scroll: true,
    };
    let agw: agw::AgwEmulator = agw::AgwEmulator::new(&rt, can_name, vlad, audio_page_settings);
    let _next_down = false;
    let _prev_down = false;

    let mut ezs_can = CanBus::B.create_can_socket().unwrap();
    let _ = ezs_can.set_filter(&[CANFilter::new(EZS_A1::get_canid() as u32, 0xFFF).unwrap()]);

    let mut key_in_ezs = true;

    loop {
        if let Some(Ok(frame)) = ezs_can.next().await {
            let wrapped = EZS_A1::new(frame_to_u64(&frame).0);
            if wrapped.get_KL_15R_EIN() {
                if !key_in_ezs {
                    agw.wakeup();
                }
                key_in_ezs = true;
            } else {
                key_in_ezs = false
            }
        }
    }
}
