use glium::glutin;
use std::f32;
use std::cell::Ref;
use crate::utils;

type Point = (f32, f32, f32);

pub struct CameraState {
    aspect_ratio: f32,
    position: Point,
    direction: Point,

    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,
    moving_fast: bool,

    lock_cursor: bool
}

impl CameraState {
    pub fn new(position: Point, direction: Point) -> CameraState {
        CameraState {
            aspect_ratio: 1024.0 / 768.0,
            position,
            direction,

            moving_up: false,
            moving_left: false,
            moving_down: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,
            moving_fast: false,  // TODO: This

            lock_cursor: false
        }
    }

    pub fn set_position(&mut self, pos: (f32, f32, f32)) {
        self.position = pos;
    }

    pub fn set_direction(&mut self, dir: (f32, f32, f32)) {
        self.direction = dir;
    }

    pub fn get_pitch_yaw(&self) -> (f32, f32) {  // See https://stackoverflow.com/a/33790309/2016800
        (
            self.direction.1.asin(),
            self.direction.2.atan2(self.direction.0)
        )
    }

    pub fn set_pitch_yaw(&mut self, pitch: f32, yaw: f32) {
        self.direction = (
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos()
        );
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        let fov = f32::consts::FRAC_PI_2 / 2.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [f / self.aspect_ratio,    0.0,              0.0              ,   0.0],
            [         0.0         ,     f ,              0.0              ,   0.0],
            [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
            [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
        ]
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        let f = {
            let f = self.direction;
            let len = f.0 * f.0 + f.1 * f.1 + f.2 * f.2;
            let len = len.sqrt();
            (f.0 / len, f.1 / len, f.2 / len)
        };

        let up = (0.0, 1.0, 0.0);

        let s = (f.1 * up.2 - f.2 * up.1,
                 f.2 * up.0 - f.0 * up.2,
                 f.0 * up.1 - f.1 * up.0);

        let s_norm = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };

        let u = (s_norm.1 * f.2 - s_norm.2 * f.1,
                 s_norm.2 * f.0 - s_norm.0 * f.2,
                 s_norm.0 * f.1 - s_norm.1 * f.0);

        let p = (-self.position.0 * s.0 - self.position.1 * s.1 - self.position.2 * s.2,
                 -self.position.0 * u.0 - self.position.1 * u.1 - self.position.2 * u.2,
                 -self.position.0 * f.0 - self.position.1 * f.1 - self.position.2 * f.2);

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [s_norm.0, u.0, f.0, 0.0],
            [s_norm.1, u.1, f.1, 0.0],
            [s_norm.2, u.2, f.2, 0.0],
            [p.0, p.1,  p.2, 1.0],
        ]
    }

    pub fn update(&mut self, app: &mut utils::App, dt: f32) {
        let window = app.display.gl_window();

        // Grab/hide the mouse cursor
        window.grab_cursor(self.lock_cursor);
        window.hide_cursor(self.lock_cursor);

        // Move the camera
        if self.lock_cursor {
            // Calculate move speed
            let move_speed = if self.moving_fast { 15. } else { 7.5 } * dt;

            // Normalize the direction
            let forward = {
                let f = self.direction;
                let len = f.0 * f.0 + f.1 * f.1 + f.2 * f.2;
                let len = len.sqrt();
                (f.0 / len, f.1 / len, f.2 / len)
            };

            // Get up direction
            let up = (0.0, 1.0, 0.0);

            // Get cross product
            let side = (forward.1 * up.2 - forward.2 * up.1,
                     forward.2 * up.0 - forward.0 * up.2,
                     forward.0 * up.1 - forward.1 * up.0);

            // Normalize result
            let side = {
                let len = side.0 * side.0 + side.1 * side.1 + side.2 * side.2;
                let len = len.sqrt();
                (side.0 / len, side.1 / len, side.2 / len)
            };

            // Get the up direction
            let up = (side.1 * forward.2 - side.2 * forward.1,
                     side.2 * forward.0 - side.0 * forward.2,
                     side.0 * forward.1 - side.1 * forward.0);

            if self.moving_up {
                self.position.0 += up.0 * move_speed;
                self.position.1 += up.1 * move_speed;
                self.position.2 += up.2 * move_speed;
            }

            if self.moving_left {
                self.position.0 -= side.0 * move_speed;
                self.position.1 -= side.1 * move_speed;
                self.position.2 -= side.2 * move_speed;
            }

            if self.moving_down {
                self.position.0 -= up.0 * move_speed;
                self.position.1 -= up.1 * move_speed;
                self.position.2 -= up.2 * move_speed;
            }

            if self.moving_right {
                self.position.0 += side.0 * move_speed;
                self.position.1 += side.1 * move_speed;
                self.position.2 += side.2 * move_speed;
            }

            if self.moving_forward {
                self.position.0 += forward.0 * move_speed;
                self.position.1 += forward.1 * move_speed;
                self.position.2 += forward.2 * move_speed;
            }

            if self.moving_backward {
                self.position.0 -= forward.0 * move_speed;
                self.position.1 -= forward.1 * move_speed;
                self.position.2 -= forward.2 * move_speed;
            }
        }
    }

    pub fn process_input(&mut self, event: &glutin::Event) {
        match *event {
            glutin::Event::WindowEvent { ref event, .. } => match *event {
                glutin::WindowEvent::Resized(size) => {
                    // Update aspect ratio
                    self.aspect_ratio = (size.width / size.height) as f32
                },

                glutin::WindowEvent::KeyboardInput { ref input, .. } => {
                    // Get key state
                    let pressed = input.state == glutin::ElementState::Pressed;
                    let key = match input.virtual_keycode {
                        Some(key) => key,
                        None => return,
                    };

                    // Move camera
                    match key {
                        glutin::VirtualKeyCode::E => self.moving_up = pressed,
                        glutin::VirtualKeyCode::Q => self.moving_down = pressed,
                        glutin::VirtualKeyCode::A => self.moving_left = pressed,
                        glutin::VirtualKeyCode::D => self.moving_right = pressed,
                        glutin::VirtualKeyCode::W => self.moving_forward = pressed,
                        glutin::VirtualKeyCode::S => self.moving_backward = pressed,
                        glutin::VirtualKeyCode::LShift | glutin::VirtualKeyCode::RShift => self.moving_fast = pressed,

                        glutin::VirtualKeyCode::Escape if pressed => self.lock_cursor = false,

                        _ => { },
                    };
                },

                glutin::WindowEvent::Focused(focused) => {
                    // Change the locked state of the cursor
                    self.lock_cursor = focused
                },

                glutin::WindowEvent::MouseInput { .. } => self.lock_cursor = true,

                _ => { }
            },

            glutin::Event::DeviceEvent { ref event, .. } => match *event {
                glutin::DeviceEvent::MouseMotion { delta } => {
                    if !self.lock_cursor { return }

                    let sensitivity = 0.0025;

                    // Calculate pitch and yaw
                    let (mut pitch, mut yaw) = self.get_pitch_yaw();

                    // Add the new movement
                    yaw += delta.0 as f32 * sensitivity;
                    pitch -= delta.1 as f32 * sensitivity;

                    // Cap the pitch
                    let pitch_boundary= f32::consts::FRAC_PI_2 - 0.0001;
                    if pitch > pitch_boundary {
                        pitch = pitch_boundary;
                    }
                    if pitch < -pitch_boundary {
                        pitch = -pitch_boundary;
                    }

                    // Set new direction
                    self.set_pitch_yaw(pitch, yaw);
                },

                _ => { }
            },

            _ => { }
        }
    }
}
