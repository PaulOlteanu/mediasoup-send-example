use std::num::{NonZeroU32, NonZeroU8};

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

    let _router1 = worker
        .create_router(RouterOptions::new(media_codecs.clone()))
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
            println!("mediasoup_stuff failed {}", e);
        } else {
            println!("mediasoup_stuff succeeded");
        }
    }

    Ok(())
}
