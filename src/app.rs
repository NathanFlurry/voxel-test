use std::time::Duration;
use std::time::Instant;
use std::thread;
use glium::glutin;
use glium::Surface;
use crate::utils::AsFloatSecs;

pub trait AppState {
    fn update(&mut self, app: &mut App, dt: f32);
    fn render(&mut self,  app: &mut App, dt: f32);
    fn process_event(&mut self, event: glutin::Event);
}

struct AppInitData {
    events_loop: glutin::EventsLoop
}

pub struct WindowState {
    pub is_focused: bool,
    pub is_closing: bool
}

impl WindowState {
    fn process_event(&mut self, event: &glutin::Event) {
        match *event {
            glutin::Event::WindowEvent { ref event, .. } => match event {
                glutin::WindowEvent::CloseRequested => self.is_closing = true,
                glutin::WindowEvent::Focused(focused) => self.is_focused = *focused,
                _ => { }
            },
            _ => { }
        }
    }
}

impl Default for WindowState {
    fn default() -> WindowState {
        WindowState {
            is_focused: false,
            is_closing: false
        }
    }
}

pub struct App {
    init_data: Option<AppInitData>,
    pub display: glium::Display,
    pub window_state: WindowState
}

impl App {
    pub fn new(title: &str) -> App {
        // Build events loop
        let mut events_loop = glutin::EventsLoop::new();

        // Build display
        let window = glutin::WindowBuilder::new().with_title(title);
        let context = glutin::ContextBuilder::new().with_depth_buffer(24).with_vsync(true);
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        App {
            init_data: Some(AppInitData { events_loop }),
            display,
            window_state: WindowState::default()
        }
    }

    pub fn start(mut self, mut state: Box<AppState>) {
        // Extract the init data from the app
        let AppInitData { mut events_loop } = self.init_data.expect("Failed to get init data for app");
        self.init_data = None;

        // Create clocks
        let mut accumulator = Duration::new(0, 0);
        let mut previous_clock = Instant::now();

        loop {
            // Process events
            events_loop.poll_events(|event| {
                // Process the event in the
                self.window_state.process_event(&event);

                // Send event to the state
                state.process_event(event);
            });

            // Stop if needed
            if self.window_state.is_closing {
                break;
            }

            // Update the clock
            let now = Instant::now();
            let dt = now - previous_clock;
            accumulator += dt;
            previous_clock = now;

            // Start timer to see how long the update operations take
            let before_update = Instant::now();

            // Update the accumulator
            let fixed_time_stamp = Duration::new(0, 16666667);
            let fixed_time_stamp_float = fixed_time_stamp.as_float_secs() as f32;
            while accumulator >= fixed_time_stamp {
                accumulator -= fixed_time_stamp;

                // Update the game state
                state.update(&mut self, fixed_time_stamp_float);
            }

//            println!("dt {}", 1. / dt.as_float_secs());

            // Render
            state.render(&mut self, dt.as_float_secs() as f32);

            // Calculate time the update took
            let now = Instant::now();
            let update_duration = now - before_update;

            // Wait for update
            let time_remaining = fixed_time_stamp - accumulator;
            if time_remaining > update_duration {
                // Sleep the amount of time until the next frame should be scheduled
                thread::sleep(time_remaining - update_duration);
            } else {
                // The update took too long, so sleep for 0 ms to let the CPU do other things
                thread::sleep_ms(0);
            }
        }
    }
}
