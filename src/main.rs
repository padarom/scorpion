use anyhow::ensure;
use anyhow::Result;

fn main() -> Result<()> {
    println!("Looking for RealSense devices");
    let ctx = realsense_rust::Context::new()?;
    let devices = ctx.query_devices(None)?;
    let mut devices_found: bool = false;

    for device in devices {
        let device = device.unwrap();
        let name = device.name().unwrap().unwrap();
        let sn = device.serial_number().unwrap().unwrap();
        println!("Found {} SN {}", name, sn);
        devices_found = true;
        device.hardware_reset()?;
    }
    
    ensure!(devices_found, "No devices found");
    Ok(())
}

/*
extern crate glium;

use realsense_rust::{Error as RsError, Format as RsFormat, StreamKind};
use std::time::Duration;
use futures::executor::block_on;

async fn idunno() -> Result<(), RsError> {
    let ctx = realsense_rust::Context::new().unwrap();
    let devices = ctx.query_devices(None).expect("No devices found.");

    println!("Devices? {:?}", devices.len());
    for device in devices {
        println!("Device found: {:?}", device);
    }

    let pipeline = realsense_rust::Pipeline::new()?;
    let config = realsense_rust::Config::new()?;
    let config = config.enable_stream(StreamKind::Pose, 0, 0, 0, RsFormat::_6Dof, 200)?;
    let mut pipeline = pipeline.start(Some(config))?;

    let profile = pipeline.profile();
    for (idx, stream_result) in profile.streams()?.try_into_iter()?.enumerate() {
        let stream = stream_result?;
        println!("stream data {}: {:#?}", idx, stream.get_data()?);
    }

    loop {
        let timeout = Duration::from_millis(1000);
        let frames_result = pipeline.wait_async(Some(timeout)).await;
        let frames = match frames_result {
            Err(RsError::Timeout(..)) => {
                println!("timeout error");
                continue;
            }
            result => result?,
        };
        let poseframe = frames.pose_frame()?.unwrap();
        let pose = poseframe.pose()?;
        println!("posedata: {:?}", pose);
    }
}

fn main() {
    std::thread::spawn(move || {
        block_on(idunno()).expect("OH YEAH");
    });
    return;
    
    let event_loop = glium::glutin::event_loop::EventLoop::new();

    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Scorpion North Star");

    let cb = glium::glutin::ContextBuilder::new();

    let _display = glium::Display::new(wb, cb, &event_loop).unwrap();
}
*/