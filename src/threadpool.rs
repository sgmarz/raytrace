
use crate::vector::Vec3;
use std::thread::{spawn, JoinHandle};
use std::sync::Arc;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::vec::Vec;
use crate::camera::Camera;

pub struct ControlPacket {
    pub row: u32,
    pub col: u32,
    pub u: f64,
    pub v: f64,
    pub camera: Arc<Camera>,
    pub done: bool
}

impl ControlPacket {
    pub const fn new(row: u32, col: u32, u: f64, v: f64, camera: Arc<Camera>) -> Self {
        Self {
            row,
            col,
            u,
            v,
            camera,
            done: false
        }
    }
    pub fn done() -> Self {
        Self {
            row: 0,
            col: 0,
            u: 0.0,
            v: 0.0,
            camera: Arc::new(Camera::default()),
            done: true,
        }
    }
}

pub struct DataPacket {
    pub row: u32,
    pub col: u32,
    pub color: Vec3,

}

impl DataPacket {
    pub const fn new(row: u32, col: u32, color: Vec3) -> Self {
        Self {
            row,
            col,
            color
        }
    }
}

pub struct Thread {
    pub thread: JoinHandle<()>,
    pub data: Receiver<DataPacket>,
    pub control: Sender<ControlPacket>,
    pub packets_sent: usize
}

pub struct ThreadPool {
    pub threads: Vec<Thread>,
    pub next: usize
}

impl ThreadPool {
    pub fn new(num_threads: usize) -> Self
    {
        assert!(num_threads > 0);
        let mut threads = Vec::with_capacity(num_threads);
        for _ in 0..num_threads {
            let (data_s, data_r): (Sender<DataPacket>, Receiver<_>) = channel();
            let (control_s, control_r): (Sender<ControlPacket>, Receiver<_>) = channel();
            let cws = data_s.clone();
            let t = Thread {
                thread: spawn(move || {
                    while let Ok(packet) = control_r.recv() {
                        if packet.done == true {
                            break;
                        }
                        let color = packet.camera.ray(packet.u, packet.v).trace();
                        let dp = DataPacket::new(packet.row, packet.col, color);
                        cws.send(dp).unwrap();
                    }
                }),
                data: data_r,
                control: control_s,
                packets_sent: 0
            };
            threads.push(t);
        }
        Self {
            threads,
            next: 0
        }
    }
    pub fn run(&mut self, control: ControlPacket) -> bool {
        let t = &mut self.threads[self.next];
        t.packets_sent += 1;
        let res = t.control.send(control);
        self.next += 1;
        if self.next >= self.threads.len() {
            self.next = 0;
        }
        res.is_ok()
    }
    pub fn run_c(&mut self, row: u32, col: u32, u: f64, v: f64, camera: Arc<Camera>) -> bool {
        let cp = ControlPacket::new(row, col, u, v, camera);
        self.run(cp)
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for t in self.threads.drain(..) {
            t.control.send(ControlPacket::done()).unwrap();
            t.thread.join().unwrap();
        }
    }
}

