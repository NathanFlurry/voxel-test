use std::time::Duration;
use std::time::Instant;
use std::thread;
use glium::glutin;
use glium::Surface;

pub trait AppState {
    fn update(&mut self, app: &mut App);
    fn render(&mut self,  app: &mut App);
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
    pub fn new() -> App {
        // Build events loop
        let mut events_loop = glutin::EventsLoop::new();

        // Build display
        let window = glutin::WindowBuilder::new();
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
            accumulator += now - previous_clock;
            previous_clock = now;

            // Update the accumulator
            let fixed_time_stamp = Duration::new(0, 16666667);
            while accumulator >= fixed_time_stamp {
                accumulator -= fixed_time_stamp;

                // Update the game state
                state.update(&mut self);
            }

            // Render
            state.render(&mut self);

            // Wait for update
            thread::sleep(fixed_time_stamp - accumulator);
        }
    }
}
