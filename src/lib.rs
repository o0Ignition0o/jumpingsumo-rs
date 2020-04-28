use anyhow::Result as AnyResult;
use arsdk_rs::Drone;
use std::net::IpAddr;

use arsdk_rs::command::Feature::JumpingSumo as JumpingSumoFeature;
use arsdk_rs::frame::{Type as FrameType, BufferID, Frame};
use arsdk_rs::jumping_sumo::Class::*;
use arsdk_rs::jumping_sumo::PilotState;
use arsdk_rs::jumping_sumo::PilotingID::*;

pub struct JumpingSumo {
    drone: Drone,
}

impl JumpingSumo {
    pub fn new(addr: IpAddr) -> AnyResult<Self> {
        Ok(Self {
            drone: Drone::new(addr)?,
        })
    }

    pub fn forward(&self) -> AnyResult<()> {
        self.drive(PilotState {
            flag: true,
            speed: i8::MAX,
            turn: 0,
        })
    }

    pub fn backwards(&self) -> AnyResult<()> {
        self.drive(PilotState {
            flag: true,
            speed: i8::MIN,
            turn: 0,
        })
    }

    pub fn turn_left(&self) -> AnyResult<()> {
        self.drive(PilotState {
            flag: true,
            speed: 0,
            turn: i8::MIN,
        })
    }

    pub fn turn_right(&self) -> AnyResult<()> {
        self.drive(PilotState {
            flag: true,
            speed: 0,
            turn: i8::MAX,
        })
    }

    pub fn drive(&self, state: PilotState) -> AnyResult<()> {
        let feature = JumpingSumoFeature(Piloting(Pilot(state)));
        let frame = Frame::for_drone(&self.drone, FrameType::Data, BufferID::CDNonAck, feature);

        self.drone.send_frame(frame)
    }
}
