pub struct CircularBuffer {
    buffer: Vec<f32>,
    buffer_size: usize,
    read_position: usize,
    write_position: usize,
    sample_rate: i32,
}

impl CircularBuffer {
    pub fn new(size_of_buffer: usize) -> Self {
        CircularBuffer {
            buffer: vec![0.0; size_of_buffer],
            buffer_size: size_of_buffer,
            read_position: size_of_buffer - 1,
            write_position: 0,
            sample_rate: 44100, // Set a default sample rate (change as needed)
        }
    }

    pub fn resize(&mut self, new_size: usize) {
        self.buffer = vec![0.0; new_size];
        self.buffer_size = new_size;
    }

    pub fn set_sample_rate(&mut self, new_sample_rate: i32) {
        self.sample_rate = new_sample_rate;
    }

    pub fn set_read_position_from_milliseconds(&mut self, milliseconds: i32) {
        let offset = ((milliseconds as f32 / 1000.0) * self.sample_rate as f32 + 0.5) as usize;
        let temp = self.write_position.wrapping_sub(offset);
        self.read_position = if temp >= self.buffer_size {
            temp % self.buffer_size
        } else {
            temp
        };
    }

    pub fn read(&mut self) -> f32 {
        let ret = self.buffer[self.read_position];
        self.read_position = self.read_position.wrapping_add(1);
        if self.read_position >= self.buffer_size {
            self.read_position = 0;
        }
        ret
    }

    pub fn write(&mut self, val: f32) {
        self.buffer[self.write_position] = val;
        self.write_position = self.write_position.wrapping_add(1);
        if self.write_position >= self.buffer_size {
            self.write_position = 0;
        }
    }

    pub fn reset(&mut self) {
        self.buffer.iter_mut().for_each(|x| *x = 0.0);
        self.write_position = 0;
        self.read_position = self.buffer_size - 1;
    }

    pub fn get_sample_rate(&self) -> i32 {
        self.sample_rate
    }

    pub fn get_buffer_size(&self) -> usize {
        self.buffer_size
    }

    pub fn get_read_position(&self) -> usize {
        self.read_position
    }

    pub fn get_write_position(&self) -> usize {
        self.write_position
    }
}
