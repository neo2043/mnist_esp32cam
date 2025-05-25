use anyhow::{bail, Result};

use esp_idf_hal::io::{EspIOError, Write};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::peripherals::Peripherals,
    http::{server::EspHttpServer, Method},
};
use esp32_cam::{config::get_config, espcam::Camera, wifi_handler::my_wifi};

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let sysloop = EspSystemEventLoop::take()?;

    let peripherals = Peripherals::take().unwrap();

    let config = get_config();

    let _wifi = match my_wifi(
        config.wifi_ssid,
        config.wifi_psk,
        peripherals.modem,
        sysloop,
    ) {
        Ok(inner) => inner,
        Err(err) => {
            bail!("Could not connect to Wi-Fi network: {:?}", err)
        }
    };

    let camera = Camera::new(
        peripherals.pins.gpio32,
        peripherals.pins.gpio0,
        peripherals.pins.gpio5,
        peripherals.pins.gpio18,
        peripherals.pins.gpio19,
        peripherals.pins.gpio21,
        peripherals.pins.gpio36,
        peripherals.pins.gpio39,
        peripherals.pins.gpio34,
        peripherals.pins.gpio35,
        peripherals.pins.gpio25,
        peripherals.pins.gpio23,
        peripherals.pins.gpio22,
        peripherals.pins.gpio26,
        peripherals.pins.gpio27,
        esp_idf_sys::camera::pixformat_t_PIXFORMAT_JPEG,
        esp_idf_sys::camera::framesize_t_FRAMESIZE_VGA,
    )
    .unwrap();

    let mut server = EspHttpServer::new(&esp_idf_svc::http::server::Configuration::default())?;

    // server.fn_handler("/camera.jpg", Method::Get, move |request| {
    //     camera.get_framebuffer();
    //     // take two frames to get a fresh one
    //     let framebuffer = camera.get_framebuffer();

    //     if let Some(framebuffer) = framebuffer {
    //         let data = framebuffer.data();

    //         let headers = [
    //             ("Content-Type", "image/jpeg"),
    //             ("Content-Length", &data.len().to_string()),
    //         ];
    //         let mut response = request.into_response(200, Some("OK"), &headers).unwrap();
    //         response.write_all(data)?;
    //     } else {
    //         let mut response = request.into_ok_response()?;
    //         response.write_all("no framebuffer".as_bytes())?;
    //     }

    //     Ok::<(), EspIOError>(())
    // })?;

    server.fn_handler("/", Method::Get, |request| {
        let mut response = request.into_ok_response()?;
        response.write_all("ok".as_bytes())?;
        Ok::<(), EspIOError>(())
    })?;

    server.fn_handler("/stream", Method::Get, move |req|{
        let headers = [
                ("Content-Type", "multipart/x-mixed-replace;boundary=123456789000000000000987654321"),
            ];
        if let Ok(mut resp) = req.into_response(200,Some("ok"),&headers){
            loop{
                // if let Some(fb) = c2.lock().unwrap().get_framebuffer(){
                if let Some(fb) = camera.get_framebuffer(){
                    resp.write("\r\n--123456789000000000000987654321\r\n".as_bytes())?;
                    resp.write(format!("Content-Type: image/jpeg\r\nContent-Length: {}\r\n\r\n",fb.data().len()).as_bytes())?;
                    resp.write(fb.data())?;
                }
            }
        };
        Ok::<(), EspIOError>(())
    })?;

    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
