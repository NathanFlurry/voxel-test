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

pub struct App {
    events_loop: glutin::EventsLoop,
    pub display: glium::Display
}

impl App {
    pub fn new() -> App {
        // Build events loop
        let mut events_loop = glutin::EventsLoop::new();

        // Build display
        let window = glutin::WindowBuilder::new();
        let context = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        App {
            events_loop,
            display
        }
    }

    pub fn start(mut self, mut state: Box<AppState>) {
        // Create clocks
        let mut accumulator = Duration::new(0, 0);
        let mut previous_clock = Instant::now();

        loop {
            // Process events
            let mut should_stop = false;
            self.events_loop.poll_events(|event| {
                match event {
                    glutin::Event::WindowEvent { event: glutin::WindowEvent::CloseRequested, .. } => should_stop = true,
                    _ => state.process_event(event)
                }
            });

            // Stop if needed
            if should_stop {
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
