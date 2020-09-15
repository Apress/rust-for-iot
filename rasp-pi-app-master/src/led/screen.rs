
// tag::led[]
use sensehat_screen::{font_to_frame, PixelColor, Screen, FrameLine, FONT_COLLECTION, Scroll}; // <1>

use std::thread;
use std::time::Duration;

const LED_DEV_PATH: &str = "/dev/fb1";  // <2>

pub struct LedControls {
    screen: Screen,             // <3>
    frame: FrameProcessor
}

// Clone is needed because ::  pub trait FlowDelegate: Clone {
// so its constituent parts have to be Clone 
// This is needed for our Authenticator
impl Clone for LedControls {    // <4>
    fn clone(&self) -> Self {
        LedControls {
            screen: Screen::open(LED_DEV_PATH).unwrap(),
            frame: FrameProcessor::new()
        }
    }
}

impl LedControls {
    pub fn new() -> LedControls {   // <5>
        LedControls {
            screen: Screen::open(LED_DEV_PATH).unwrap(),
            frame: FrameProcessor::new()
        }
    }
// end::led[]

    // tag::blank[]
    pub fn blank(&mut self) {
        self.screen.write_frame(&self.frame.off_frame); 
    }
    // end::blank[]

    /**
     * Will display a question mark for 3 seconds.
     */
    // tag::ques[]
    pub fn question(&mut self) {
        self.screen.write_frame(&self.frame.question_mark);
        thread::sleep(Duration::from_secs(3));
        self.screen.write_frame(&self.frame.off_frame);
    }
    // end::ques[]

    // tag::symbol[]
    pub fn display_symbol(&mut self, frame: &[u8; 128], length: u64) {
        let frame_line = FrameLine::from_slice(frame);
        self.screen.write_frame(&frame_line);
        thread::sleep(Duration::from_secs(length));
        self.screen.write_frame(&self.frame.off_frame);
    }
    // end::symbol[]

    // tag::proc[]
    pub fn processing(&mut self) {
        let sleep_time = 500;
        let yellow_squares = self.frame.yellow_squares;

        for x in 0..2 {
            for ys in &yellow_squares {
                    self.screen.write_frame(ys);
                thread::sleep(Duration::from_millis(sleep_time));
            }
        }
    }
    // end::proc[]

    // tag::display[]
    pub fn display(&mut self, word: &str) {
        // get the screen text
        // uses a macro tto get the font string
        let screen_text = FONT_COLLECTION.sanitize_str(word).unwrap(); // <1>
        let white_50_pct = PixelColor::WHITE.dim(0.5);              

        // Display the items
        for unicode in screen_text.chars() {
            if let Some(symbol) = FONT_COLLECTION.get(unicode) {            // <2>
                let frame = font_to_frame(&symbol.byte_array(), white_50_pct);  // <3>
                self.screen.write_frame(&frame);                // <4>
            }
            thread::sleep(Duration::from_millis(800));
        }
        // now turn the display back off
        self.screen.write_frame(&self.frame.off_frame);
    }
    // end::display[]

    // tag::scroll[]
    pub fn scroll_text(&mut self, word: &str) {
        let sanitized = FONT_COLLECTION.sanitize_str(word).unwrap();

        // Render the `FontString` as a vector of pixel frames, with
        // a stroke color of Blue and a BLACK background.
        let pixel_frames = sanitized.pixel_frames(PixelColor::BLUE, PixelColor::BLACK); // <1>

        // Create a `Scroll` from the pixel frame vector.
        // this will create some arrows to scroll over
        let scroll = Scroll::new(&pixel_frames);           // <2>

        // Consume the `FrameSequence` returned by the `left_to_right` method.
        scroll.left_to_right().for_each(|frame| {                   // <3>
            self.screen.write_frame(&frame.frame_line());
            thread::sleep(::std::time::Duration::from_millis(250));
        });        
    }
    // end::scroll[]
}

// tag::frame[]
struct FrameProcessor {
    off_frame: FrameLine,
    yellow_squares: [FrameLine; 4],
    question_mark: FrameLine,
}

impl FrameProcessor {
    fn new() -> FrameProcessor {
        let ys = [                  // <1>
            FrameLine::from_slice(&super::YELLOW_SMALL),
            FrameLine::from_slice(&super::YELLOW_MED),
            FrameLine::from_slice(&super::YELLOW_LARGE),
            FrameLine::from_slice(&super::YELLOW_XL),
        ];

        // Question Mark
        let white_50_pct = PixelColor::WHITE.dim(0.5);  // <2>
        let q_mark = FONT_COLLECTION.get('?').unwrap();

        FrameProcessor {
            off_frame: FrameLine::from_slice(&super::OFF),  // <3>
            yellow_squares: ys,
            question_mark: font_to_frame(&q_mark.byte_array(), white_50_pct),   // <4>
        }
    }
}
// end::frame[]