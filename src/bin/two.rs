use std::{
    net::{IpAddr, Ipv4Addr},
    num::{NonZeroU32, NonZeroU8},
};

use anyhow::anyhow;
use mediasoup::prelude::*;

async fn do_mediasoup_stuff(worker: Worker) -> anyhow::Result<()> {
    let media_codecs = vec![RtpCodecCapability::Audio {
        mime_type: MimeTypeAudio::Opus,
        preferred_payload_type: None,
        clock_rate: NonZeroU32::new(48000).unwrap(),
        channels: NonZeroU8::new(2).unwrap(),
        parameters: RtpCodecParametersParameters::from([("useinbandfec", 1_u32.into())]),
        rtcp_feedback: vec![],
    }];

    let router1 = worker
        .create_router(RouterOptions::new(media_codecs.clone()))
        // .await?;
        .await
        .map_err(|e| anyhow!(e.to_string()))?;

    let router2 = worker
        .create_router(RouterOptions::new(media_codecs))
        // .await?;
        .await
        .map_err(|e| anyhow!(e.to_string()))?;

    let transport1 = router1
        .create_webrtc_transport(WebRtcTransportOptions::new(
            WebRtcTransportListenInfos::new(ListenInfo {
                protocol: Protocol::Udp,
                ip: IpAddr::V4(Ipv4Addr::LOCALHOST),
                announced_ip: Some("9.9.9.1".parse().unwrap()),
                port: None,
                send_buffer_size: None,
                recv_buffer_size: None,
            }),
        ))
        // .await?;
        .await
        .map_err(|e| anyhow!(e.to_string()))?;

    let producer1 = transport1
        .produce(ProducerOptions::new(
            MediaKind::Audio,
            RtpParameters {
                mid: Some("AUDIO".to_string()),
                codecs: vec![RtpCodecParameters::Audio {
                    mime_type: MimeTypeAudio::Opus,
                    payload_type: 0,
                    clock_rate: NonZeroU32::new(48000).unwrap(),
                    channels: NonZeroU8::new(2).unwrap(),
                    parameters: RtpCodecParametersParameters::from([
                        ("useinbandfec", 1_u32.into()),
                        ("usedtx", 1_u32.into()),
                    ]),
                    rtcp_feedback: vec![],
                }],
                ..RtpParameters::default()
            },
        ))
        // .await?;
        .await
        .map_err(|e| anyhow!(e.to_string()))?;

    // Pipe producer1 into router2.
    router1
        .pipe_producer_to_router(producer1.id(), PipeToRouterOptions::new(router2.clone()))
        // .await?;
        .await
        .map_err(|e| anyhow!(e.to_string()))?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let wm = WorkerManager::new();
    let worker = wm.create_worker(WorkerSettings::default()).await?;

    let mut tasks = Vec::new();

    for _ in 0..5 {
        let w = worker.clone();
        tasks.push(tokio::spawn(do_mediasoup_stuff(w)));
    }

    for task in tasks {
        let result = task.await.unwrap();
        if let Err(e) = result {
            println!("mediasoup_stuff failed");
        } else {
            println!("mediasoup_stuff succeeded");
        }
    }

    Ok(())
}
