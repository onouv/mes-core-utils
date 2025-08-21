//! # MES Core Utils
//!
//! The MES Core utils contain general types and utilities to support implementing an MES system.
//!
//! MES stands for Manufacturng execution system, referring to a system to do any of the following:
//! - plan and track manufacturing jobs
//! - track overall equipment efficiency (OEE)
//! - gather equipment, process quality and job data
//! - provide insight to optimize OEE
//!

mod id;
use crate::id::*;
/*
// Demo
//
//

pub fn try_it() {
    let f1 = FunctionId::default();
    println!("{}", f1);

    match FunctionId::new(ID_SEGMENT_DELIMITER_DEFAULT, "=100.010.25") {
        Ok(f2) => {
            println!("{}", f2);
        }
        Err(err) => {
            println!("{}", err);
        }
    }

    match EquipmentId::new("/", "-100/010/25") {
        Ok(f3) => {
            println!("{}", f3);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}
*/
