use livekit::prelude::*;
use livekit_api::access_token;
use std::env;
use futures::StreamExt;
use livekit::webrtc::video_stream::native::NativeVideoStream;
use livekit::webrtc::encoded_video_frame_stream::native::NativeEncodedVideoFrameStream;
use livekit::webrtc::encoded_video_frame::EncodedVideoFrame;

use livekit::webrtc::audio_stream::native::NativeAudioStream;
use livekit::webrtc::encoded_audio_frame_stream::native::NativeEncodedAudioFrameStream;
use livekit::webrtc::encoded_audio_frame::EncodedAudioFrame;

// Basic demo to connect to a room using the specified env variables

#[tokio::main]
async fn main() {
    // let url = env::var("LIVEKIT_URL").expect("LIVEKIT_URL is not set");
    // let token = env::var("LIVEKIT_TOKEN").expect("LIVEKIT_TOKEN is not set");

    let url = env::var("LIVEKIT_URL").expect("LIVEKIT_URL is not set");
    let api_key = env::var("LIVEKIT_API_KEY").expect("LIVEKIT_API_KEY is not set");
    let api_secret = env::var("LIVEKIT_API_SECRET").expect("LIVEKIT_API_SECRET is not set");

    let token = access_token::AccessToken::with_api_key(&api_key, &api_secret)
        .with_identity("rust-bot")
        .with_name("Rust Bot")
        .with_grants(access_token::VideoGrants {
            room_join: true,
            room: "my-room".to_string(),
            ..Default::default()
        })
        .to_jwt()
        .unwrap();

    let (room, mut rx) = Room::connect(&url, &token, RoomOptions::default())
        .await
        .unwrap();
    log::info!("Connected to room: {} - {}", room.name(), room.sid());


    room.local_participant()
        .publish_data(DataPacket {
            payload: "Hello world".to_owned().into_bytes(),
            kind: DataPacketKind::Reliable,
            ..Default::default()
        })
        .await
        .unwrap();

    while let Some(msg) = rx.recv().await {
        println!("Event: {:?}", msg);
        match msg {
            RoomEvent::TrackSubscribed { track, publication, participant } => {
                if let RemoteTrack::Video(video_track) = &track {
                    match &video_track.receiver() {
                        Some(receiver) => {
                            let parameters = receiver.parameters();
                            println!("Parameters: {:?}", parameters);
                            let mut encoded_frame_stream = NativeEncodedVideoFrameStream::new(receiver);
                            while let Some(frame) = encoded_frame_stream.next().await {
                                println!("Got encoded frame - {}x{} type: {}", frame.width(), frame.height(), frame.payload_type());
                            //     let payload = frame.payload();
                            //     println!("payload:");
                            //     for b in payload {
                            //         print!("{:02x}", b);
                            //     }
                            //     println!();
                            }                            
                        },
                        None => {
                            println!("No receiver!");
                        },
                    }
                    
                    let rtc_track = video_track.rtc_track();
                    let mut video_stream = NativeVideoStream::new(rtc_track);
                    while let Some(frame) = video_stream.next().await {

                    }
                    // break;
                }
                else if let RemoteTrack::Audio(audio_track) = &track {
                    // match &audio_track.receiver() {
                    //     Some(receiver) => {
                    //         let parameters = receiver.parameters();
                    //         // println!("Parameters: {:?}", parameters);
                    //         let mut encoded_frame_stream = NativeEncodedAudioFrameStream::new(receiver);
                    //         while let Some(frame) = encoded_frame_stream.next().await {
                    //             println!("Got encoded audio frame type: {}", frame.payload_type());
                    //             let payload = frame.payload();
                    //             println!("payload:");
                    //             for b in payload {
                    //                 print!("{:02x}", b);
                    //             }
                    //             println!();
                    //         }
                    //         println!("Exited");
                    //     },
                    //     None => {
                    //         println!("No receiver!");
                    //     },
                    // }
                }
            },
            _ => {}
        }
    }
}
