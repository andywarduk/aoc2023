use std::{borrow::Cow, cmp::max, error::Error, fs::File};

use gif::{Encoder, Frame, Repeat};

use self::region::Region;

mod region;

pub struct Gif {
    width: u16,
    height: u16,
    x_scale: u16,
    y_scale: u16,
    gif_width: u16,
    gif_height: u16,
    encoder: Encoder<File>,
    last_frame: Option<Vec<Vec<u8>>>,
}

impl Gif {
    /// Creates a new GIF with a given palette, size and scale
    pub fn new(
        file: &str,
        palette: &[[u8; 3]],
        width: u16,
        height: u16,
        x_scale: u16,
        y_scale: u16,
    ) -> Result<Self, Box<dyn Error>> {
        let gif_width = width * x_scale;
        let gif_height = height * y_scale;

        // Create the flattened palette
        let flat_pal = palette.iter().flatten().cloned().collect::<Vec<_>>();

        // Create the encoder
        let mut encoder = Encoder::new(File::create(file)?, gif_width, gif_height, &flat_pal)?;

        // Ininitely repeat
        encoder.set_repeat(Repeat::Infinite)?;

        Ok(Self {
            width,
            height,
            x_scale,
            y_scale,
            gif_width,
            gif_height,
            encoder,
            last_frame: None,
        })
    }

    /// Output a frame to the GIF
    pub fn draw_frame(
        &mut self,
        frame_data: Vec<Vec<u8>>,
        delay: u16,
    ) -> Result<(), Box<dyn Error>> {
        self.draw_frame_identical_check(frame_data, delay, IdenticalAction::Ignore)
    }

    /// Output a frame to the GIF and takes a given action if the frame is identical to the last
    pub fn draw_frame_identical_check(
        &mut self,
        frame_data: Vec<Vec<u8>>,
        delay: u16,
        identical_action: IdenticalAction,
    ) -> Result<(), Box<dyn Error>> {
        // Make sure the frame looks like the correct size
        assert_eq!(frame_data.len(), self.height as usize);
        assert_eq!(frame_data[0].len(), self.width as usize);

        // Calculate the difference between this frame and the last
        match self.frame_difference(&frame_data) {
            None => {
                // No difference
                match identical_action {
                    IdenticalAction::Delay => self.delay(delay)?,
                    IdenticalAction::Ignore => (),
                }
            }
            Some(difference) => {
                // Scale the frame up
                let out_section = frame_data
                    .iter()
                    .enumerate()
                    .filter_map(|(y, l)| {
                        if difference.contains_y(y as u16) {
                            Some(&l[difference.x_range()])
                        } else {
                            None
                        }
                    })
                    .fold(
                        Vec::with_capacity(self.gif_height as usize * self.gif_width as usize),
                        |mut acc: Vec<u8>, line| {
                            let expanded_line: Vec<u8> = line
                                .iter()
                                .flat_map(|pix| vec![*pix; self.x_scale as usize])
                                .collect();

                            for _ in 0..self.y_scale {
                                acc.extend(&expanded_line);
                            }

                            acc
                        },
                    );

                // Create the next frame
                let frame = Frame {
                    top: difference.top() * self.y_scale,
                    left: difference.left() * self.x_scale,
                    width: difference.width() * self.x_scale,
                    height: difference.height() * self.y_scale,
                    buffer: Cow::Borrowed(&*out_section),
                    delay: max(2, delay),
                    ..Default::default()
                };

                // Write out the frame
                self.encoder.write_frame(&frame)?;

                // Save the last frame
                self.last_frame = Some(frame_data);
            }
        }

        Ok(())
    }

    /// Creates an empty delay frame
    pub fn delay(&mut self, delay: u16) -> Result<(), Box<dyn Error>> {
        // Create the next frame
        let frame = Frame {
            delay: max(2, delay),
            width: 1,
            height: 1,
            transparent: Some(0),
            buffer: Cow::Owned(vec![0]),
            ..Default::default()
        };

        // Write out the frame
        self.encoder.write_frame(&frame)?;

        Ok(())
    }

    /// Returns dimensions of the image
    pub fn dimensions(&self) -> (u16, u16) {
        (self.gif_width, self.gif_height)
    }

    /// Returns a new empty fram for the image
    pub fn empty_frame(&self) -> Vec<Vec<u8>> {
        vec![vec![0; self.gif_width as usize]; self.gif_height as usize]
    }

    /// Calculates the difference between a given frame and the last frame output
    /// Returns None if there is no difference
    /// If there is no previous frame then the whole frame is considered changed
    fn frame_difference(&self, frame_data: &[Vec<u8>]) -> Option<Region> {
        if let Some(last_frame) = &self.last_frame {
            let mut region = Region::max_init();

            // Process each row
            for (y, (l1, l2)) in last_frame.iter().zip(frame_data.iter()).enumerate() {
                // Process each pixel
                for (x, (_, _)) in l1
                    .iter()
                    .zip(l2.iter())
                    .enumerate()
                    .filter(|(_, (p1, p2))| *p1 != *p2)
                {
                    region.max_add(x as u16, y as u16)
                }
            }

            if region.max_valid() {
                Some(region)
            } else {
                None
            }
        } else {
            // No previous frame
            Some(Region::new(0, 0, self.height - 1, self.width - 1))
        }
    }
}

/// Action to take if the next frame is identical to the last
pub enum IdenticalAction {
    Ignore,
    Delay,
}
