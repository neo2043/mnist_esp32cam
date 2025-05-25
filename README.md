# MNIST ESP32-CAM

This project enables real-time digit classification using an **ESP32-CAM** streaming JPEG images over HTTP to a **browser-based interface** powered by a neural network. It allows handwritten digits or similar content to be detected and predicted using a lightweight web application and a machine learning inference engine running in WebAssembly.

---

## Project Overview

- **Hardware:** ESP32-CAM captures live video and streams JPEG frames over HTTP.
- **Client:** A Chromium-based browser receives these JPEG frames, decodes them, extracts pixel data, and feeds it into a trained MNIST digit classifier.
- **Inference:** The classifier is implemented using WebAssembly (compiled from Rust) and runs entirely in the browser.

---

## Getting Started

### 1. ESP32-CAM Setup

Flashing an ESP32-CAM board with a sketch that serves MJPEG stream.

  1. Connect the esp-cam to pc using ftdi and put esp-cam in flash mode
  2. run the command
      
        `cargo run`
  3. reboot esp-cam in normal mode

### 2. Web Interface

1. To run the Web Interface. Run the command below in web_interface directory
    
    `python -m http.server`

---

## Credits

#### [tracel-ai/burn](https://github.com/tracel-ai/burn)
#### [Kezii/esp32cam_rs](https://github.com/Kezii/esp32cam_rs)