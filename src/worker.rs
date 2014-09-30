#![feature(macro_rules)]
#![allow(dead_code)]

// test harness access
#[cfg(test)]
extern crate test;

extern crate time;

use time::Timespec;


pub type UuidBytes = i64;

macro_rules! max_by_bits( ($inp:expr) => ( -1i64 ^ (-1i64 << $inp)); )

static NUM_WORKER_ID_BITS: uint = 5;
static NUM_DATACENTER_ID_BITS: uint = 5;
static NUM_SEQUENCE_ID_BITS: uint = 12;
static NUM_TIMESTAMP_BITS: uint = 41;
static MAX_WORKER_ID: i64 = max_by_bits!(NUM_WORKER_ID_BITS);
static MAX_SEQUENCE_ID: i64 = max_by_bits!(NUM_SEQUENCE_ID_BITS);
static MAX_DATACENTER_ID: i64 = max_by_bits!(NUM_DATACENTER_ID_BITS);
static MAX_TIMESTAMP: i64 = max_by_bits!(NUM_TIMESTAMP_BITS);
static SEQUENCE_ID_SHIFT: uint = 0;
static WORKER_ID_SHIFT: uint = NUM_SEQUENCE_ID_BITS;
static DATACENTER_ID_SHIFT: uint = NUM_SEQUENCE_ID_BITS + NUM_WORKER_ID_BITS;
static TIMESTAMP_SHIFT: uint = NUM_SEQUENCE_ID_BITS + NUM_WORKER_ID_BITS + NUM_DATACENTER_ID_BITS;

// A Universally Unique Identifier (UUID)
pub struct Uuid {
    pub bytes: UuidBytes
}

#[deriving(Clone)]
pub struct IdWorker {
    worker_id: i64,
    datacenter_id: i64,
    sequence_id: i64,
    last_time: Timespec
}

impl Uuid {
    pub fn new(bytes: UuidBytes) -> Uuid {
        Uuid { bytes: bytes }
    }

    pub fn get_timestamp(self) -> i64 {
        (self.bytes >> TIMESTAMP_SHIFT) & MAX_TIMESTAMP
    }

    pub fn get_worker_id(self) -> i64 {
        (self.bytes >> WORKER_ID_SHIFT) & MAX_WORKER_ID
    }

    pub fn get_datacenter_id(self) -> i64 {
        (self.bytes >> DATACENTER_ID_SHIFT) & MAX_DATACENTER_ID
    }

    pub fn get_sequence_id(self) -> i64 {
        (self.bytes >> SEQUENCE_ID_SHIFT) & MAX_SEQUENCE_ID
    }
}

impl IdWorker {
    pub fn new(worker_id: i64, datacenter_id: i64) -> IdWorker {
        if worker_id > MAX_WORKER_ID { fail!("CMON!") }
        if datacenter_id > MAX_DATACENTER_ID { fail!("CMON!") }

        IdWorker {
            worker_id: worker_id,
            datacenter_id: datacenter_id,
            sequence_id: 0i64,
            last_time: Timespec { sec: -1, nsec: 0}
        }
    }

    // TODO(tal): figure out the whole immutable thing for middleware
    pub fn next(self) -> Uuid {
        // Tue, 21 Mar 2006 20:50:14.000 GMT (first tweet)
        let twepoch:Timespec = Timespec { sec: 1288834974657, nsec: 0 };


        let mut curr_time: Timespec = time::now_utc().to_timespec();

        // check time is not going backwards
        if curr_time < self.last_time {
            fail!("fix your time, yo!");
        }

        /*
        if curr_time == self.last_time {
            self.sequence_id = (self.sequence_id + 1) & MAX_SEQUENCE_ID;
            if self.sequence_id == 0 {
                curr_time = self.until_next_time();
            }
        } else {
            self.sequence_id = 0;
        }

        self.last_time = curr_time;
        */

        let bytes: UuidBytes = (curr_time - twepoch).num_milliseconds() << TIMESTAMP_SHIFT |
            (self.datacenter_id << DATACENTER_ID_SHIFT) |
            (self.worker_id << WORKER_ID_SHIFT) |
            self.sequence_id;

        Uuid::new(bytes)
    }

    fn until_next_time(self) -> Timespec {
        let mut t = time::now_utc().to_timespec();
        while t <= self.last_time {
            t = time::now_utc().to_timespec();
        }
        return t;
    }
}

#[cfg(test)]
mod tests {
    use super::{Uuid, IdWorker};

    #[test]
    fn test_next() {
        let (worker_id, datacenter_id): (i64, i64) = (31, 31);
        let mut worker = IdWorker::new(worker_id, datacenter_id);
        let id: Uuid = worker.next();
        assert_eq!(id.get_worker_id(), worker_id);
        assert_eq!(id.get_datacenter_id(), datacenter_id);
    }
}
