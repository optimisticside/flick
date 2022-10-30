<div align="center">
    <img src="https://github.com/optimisticside/flick/blob/master/assets/logo-wide.png" alt="Flick Logo">
</div>

Open-source rocket flight computer: logs IMU data, detects rocket apogee & freefall, provides in-flight stabilization through thrust vector control.

# About Flick
Flick is an open-source flight controller for model rockets for use on the Teensy line of microcontrollers with a MPU 6050 IMU, and BMP280 barometer. The controller and communication-protocol are written in Rust, and the ground control system is written in Python. Flick will eventually control automatic apogee detection in flight and parachute deployment.

# Installation
The flight controller can be installed through `cargo`, but the target for the Teensy controller will need to be added (which you can do through `rustup`). Once the target has been added, running `cargo build` will create the binary. To download the program onto the Teensy microcontroller, you'll need either a build of [`teensy_loader_cli`](https://github.com/PaulStoffregen/teensy_loader_cli), or the [Teensy Loader Application](https://www.pjrc.com/teensy/loader.html). The latter is available with the Teensyduino add-ons.
