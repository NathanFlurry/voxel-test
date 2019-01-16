use glium::glutin;
use std::f32;
use crate::utils;
use vecmath::*;

pub struct CameraState {
    aspect_ratio: f32,
    position: Vector3<f32>,
    direction: Vector3<f32>,

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
    pub fn new(position: Vector3<f32>, direction: Vector3<f32>) -> CameraState {
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

    #[allow(dead_code)]  // TODO: Remove
    pub fn set_position(&mut self, pos: Vector3<f32>) {
        self.position = pos;
    }

    #[allow(dead_code)]  // TODO: Remove
    pub fn set_direction(&mut self, dir: Vector3<f32>) {
        self.direction = dir;
    }

    pub fn get_pitch_yaw(&self) -> (f32, f32) {  // See https://stackoverflow.com/a/33790309/2016800
        (
            self.direction[1].asin(),
            self.direction[2].atan2(self.direction[0])
        )
    }

    pub fn set_pitch_yaw(&mut self, pitch: f32, yaw: f32) {
        self.direction = [
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos()
        ];
    }

    pub fn get_perspective(&self) -> Matrix4<f32> {
        let fov = f32::consts::FRAC_PI_2 / 2.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [ f / self.aspect_ratio,  0.,                            0., 0.],
        [                        0.,  f,                            0., 0.],
        [                        0., 0.,     (zfar+znear)/(zfar-znear), 1.],
        [                        0., 0., -(2.*zfar*znear)/(zfar-znear), 0.],
        ]
    }

    pub fn get_view(&self) -> Matrix4<f32> {
        // See https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluLookAt.xml
        // Note that when they use `s` in the `M` matrix, `s` should be normalized

        let f = vec3_normalized(self.direction);

        let up = [0., 1., 0.];

        let s = vec3_normalized(vec3_cross(f, up));

        let u = vec3_cross(s, f);

        let p = [  // TODO: Make PR for https://github.com/glium/glium/blob/master/examples/support/camera.rs#L79 to replace `s` with `s_norm` (I just replace it here)
            -self.position[0] * s[0] - self.position[1] * s[1] - self.position[2] * s[2],
            -self.position[0] * u[0] - self.position[1] * u[1] - self.position[2] * u[2],
            -self.position[0] * f[0] - self.position[1] * f[1] - self.position[2] * f[2]
        ];

        // NOTE: Remember that this is column-major, so the lines of code are actually columns
        [
            [s[0], u[0], f[0], 0.0],
            [s[1], u[1], f[1], 0.0],
            [s[2], u[2], f[2], 0.0],
            [p[0], p[1],  p[2], 1.0],
        ]
    }

    pub fn update(&mut self, app: &mut utils::App, dt: f32) {
        let window = app.display.gl_window();

        // Grab/hide the mouse cursor
        window.grab_cursor(self.lock_cursor).unwrap();
        window.hide_cursor(self.lock_cursor);

        // Move the camera
        if self.lock_cursor {
            // Calculate move speed
            let move_speed = if self.moving_fast { 20. } else { 8.5 } * dt;

            // Normalize the direction
            let forward = vec3_normalized(self.direction);

            // Get up direction
            let up = [0.0, 1.0, 0.0];

            // Get cross product
            let side = vec3_normalized(vec3_cross(forward, up));

            // Get the up direction
            let up = vec3_cross(side, forward);

            if self.moving_up {
                self.position[0] += up[0] * move_speed;
                self.position[1] += up[1] * move_speed;
                self.position[2] += up[2] * move_speed;
            }

            if self.moving_left {
                self.position[0] -= side[0] * move_speed;
                self.position[1] -= side[1] * move_speed;
                self.position[2] -= side[2] * move_speed;
            }

            if self.moving_down {
                self.position[0] -= up[0] * move_speed;
                self.position[1] -= up[1] * move_speed;
                self.position[2] -= up[2] * move_speed;
            }

            if self.moving_right {
                self.position[0] += side[0] * move_speed;
                self.position[1] += side[1] * move_speed;
                self.position[2] += side[2] * move_speed;
            }

            if self.moving_forward {
                self.position[0] += forward[0] * move_speed;
                self.position[1] += forward[1] * move_speed;
                self.position[2] += forward[2] * move_speed;
            }

            if self.moving_backward {
                self.position[0] -= forward[0] * move_speed;
                self.position[1] -= forward[1] * move_speed;
                self.position[2] -= forward[2] * move_speed;
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
