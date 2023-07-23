#[derive(Clone, Copy, Debug, PartialEq)]
struct RingBuffer<T, const S: usize> {
    buf: [T; S],
    wp: usize,
    rp: usize,
}

impl<T: Copy, const S: usize> RingBuffer<T, S> {
    pub fn new(init: T) -> Self {
        Self {
            buf: [init; S],
            wp: 0,
            rp: 0,
        }
    }

    pub fn read(&mut self) -> T {
        let val = self.buf[self.rp];
        self.rp += 1;
        self.rp %= S;
        val
    }

    pub fn write(&mut self, val: T) {
        self.buf[self.wp] = val;
        self.wp += 1;
        self.wp %= S;
    }

    pub fn write_all(&mut self, values: &[T]) {
        for val in values {
            self.write(*val);
        }
    }
}

fn main() {
    let mut rb = RingBuffer::<u8, 4>::new(0);
    rb.write_all(&[0xfe, 0xbe]);
    let val = rb.read();
    println!("val: {}", val);
    dbg!(&rb);

    rb.write(0xde);
    rb.write(0xad);

    rb.write(0xca);
    rb.write(0xfe);

    dbg!(&rb);
}
