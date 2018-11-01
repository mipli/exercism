use std::io::{Read, Result, Write};

struct Stats {
    ops: usize,
    throughput: usize
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            ops: 0,
            throughput: 0
        }
    }
}

pub struct ReadStats<R> {
    inner: R,
    stats: Stats
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            inner: wrapped,
            stats: Stats::default()
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.inner
    }

    pub fn bytes_through(&self) -> usize {
        self.stats.throughput
    }

    pub fn reads(&self) -> usize {
        self.stats.ops
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.stats.ops += 1;
        self.inner.read(buf).map(|len| {
            self.stats.throughput += len;
            len
        })
    }
}

pub struct WriteStats<W: Write> {
    inner: W,
    stats: Stats
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            inner: wrapped,
            stats: Stats::default()
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.inner
    }

    pub fn bytes_through(&self) -> usize {
        self.stats.throughput
    }

    pub fn writes(&self) -> usize {
        self.stats.ops
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.stats.ops += 1;
        self.inner.write(buf).map(|len| {
            self.stats.throughput += len;
            len
        })
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}
