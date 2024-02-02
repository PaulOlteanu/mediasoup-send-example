### Error from example 1

```
  error[E0277]: `(dyn std::error::Error + 'static)` cannot be sent between threads safely
   --> src/bin/one.rs:18:15
    |
18  |         .await?;
    |               ^ `(dyn std::error::Error + 'static)` cannot be sent between threads safely
    |
    = help: the trait `std::marker::Send` is not implemented for `(dyn std::error::Error + 'static)`
    = help: the following other types implement trait `std::ops::FromResidual<R>`:
              <std::result::Result<T, F> as std::ops::FromResidual<std::ops::Yeet<E>>>
              <std::result::Result<T, F> as std::ops::FromResidual<std::result::Result<std::convert::Infallible, E>>>
    = note: required for `std::ptr::Unique<(dyn std::error::Error + 'static)>` to implement `std::marker::Send`
note: required because it appears within the type `Box<dyn Error>`
   --> /home/paul/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:195:12
    |
195 | pub struct Box<
    |            ^^^
note: required because it appears within the type `RequestError`
   --> /home/paul/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mediasoup-0.15.0/src/worker.rs:46:10
    |
46  | pub enum RequestError {
    |          ^^^^^^^^^^^^
note: required because it appears within the type `CreateRouterError`
   --> /home/paul/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mediasoup-0.15.0/src/worker.rs:312:10
    |
312 | pub enum CreateRouterError {
    |          ^^^^^^^^^^^^^^^^^
    = note: required for `anyhow::Error` to implement `std::convert::From<mediasoup::worker::CreateRouterError>`
    = note: required for `std::result::Result<(), anyhow::Error>` to implement `std::ops::FromResidual<std::result::Result<std::convert::Infallible, mediasoup::worker::CreateRouterError>>`

error[E0277]: `(dyn std::error::Error + 'static)` cannot be shared between threads safely
   --> src/bin/one.rs:18:15
    |
18  |         .await?;
    |               ^ `(dyn std::error::Error + 'static)` cannot be shared between threads safely
    |
    = help: the trait `std::marker::Sync` is not implemented for `(dyn std::error::Error + 'static)`
    = help: the following other types implement trait `std::ops::FromResidual<R>`:
              <std::result::Result<T, F> as std::ops::FromResidual<std::ops::Yeet<E>>>
              <std::result::Result<T, F> as std::ops::FromResidual<std::result::Result<std::convert::Infallible, E>>>
    = note: required for `std::ptr::Unique<(dyn std::error::Error + 'static)>` to implement `std::marker::Sync`
note: required because it appears within the type `Box<dyn Error>`
   --> /home/paul/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:195:12
    |
195 | pub struct Box<
    |            ^^^
note: required because it appears within the type `RequestError`
   --> /home/paul/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mediasoup-0.15.0/src/worker.rs:46:10
    |
46  | pub enum RequestError {
    |          ^^^^^^^^^^^^
note: required because it appears within the type `CreateRouterError`
   --> /home/paul/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mediasoup-0.15.0/src/worker.rs:312:10
    |
312 | pub enum CreateRouterError {
    |          ^^^^^^^^^^^^^^^^^
    = note: required for `anyhow::Error` to implement `std::convert::From<mediasoup::worker::CreateRouterError>`
    = note: required for `std::result::Result<(), anyhow::Error>` to implement `std::ops::FromResidual<std::result::Result<std::convert::Infallible, mediasoup::worker::CreateRouterError>>`

For more information about this error, try `rustc --explain E0277`.
warning: `mediasoup-send` (bin "one") generated 1 warning
error: could not compile `mediasoup-send` (bin "one") due to 2 previous errors; 1 warning emitted

```

### Error from example 2
```
error[E0277]: `(dyn std::error::Error + 'static)` cannot be sent between threads safely
    --> src/bin/two.rs:87:33
     |
87   |         tasks.push(tokio::spawn(do_mediasoup_stuff(w)));
     |                    ------------ ^^^^^^^^^^^^^^^^^^^^^ `(dyn std::error::Error + 'static)` cannot be sent between threads safely
     |                    |
     |                    required by a bound introduced by this call
     |
     = help: the trait `std::marker::Send` is not implemented for `(dyn std::error::Error + 'static)`
     = note: required for `std::ptr::Unique<(dyn std::error::Error + 'static)>` to implement `std::marker::Send`
note: required because it appears within the type `Box<dyn Error>`
    --> /home/paul/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:195:12
     |
195  | pub struct Box<
     |            ^^^
note: required because it appears within the type `RequestError`
    --> /home/paul/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mediasoup-0.15.0/src/worker.rs:46:10
     |
46   | pub enum RequestError {
     |          ^^^^^^^^^^^^
note: required because it appears within the type `Result<PipeTransport, RequestError>`
    --> /home/paul/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:502:10
     |
502  | pub enum Result<T, E> {
     |          ^^^^^^
note: required because it appears within the type `Option<Result<PipeTransport, RequestError>>`
    --> /home/paul/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:569:10
     |
569  | pub enum Option<T> {
     |          ^^^^^^
note: required because it appears within the type `TryZip<impl Future<Output = Result<PipeTransport, RequestError>>, impl Future<Output = Result<PipeTransport, RequestError>>>`
    --> /home/paul/.cargo/registry/src/index.crates.io-6f17d22bba15001f/futures-lite-1.13.0/src/future.rs:424:16
     |
424  |     pub struct TryZip<F1, F2>
     |                ^^^^^^
     = note: required because it captures the following types: `&mediasoup::prelude::Router`, `mediasoup::prelude::PipeToRouterOptions`, `mediasoup::prelude::Router`, `mediasoup::router::RouterId`, `mediasoup::prelude::PipeTransportOptions`, `mediasoup::prelude::PipeTransport`, `mediasoup::prelude::PipeTransport`, `futures_lite::future::TryZip<impl std::future::Future<Output = std::result::Result<mediasoup::prelude::PipeTransport, mediasoup::worker::RequestError>>, impl std::future::Future<Output = std::result::Result<mediasoup::prelude::PipeTransport, mediasoup::worker::RequestError>>>`, `futures_lite::future::TryZip<impl std::future::Future<Output = std::result::Result<(), mediasoup::worker::RequestError>>, impl std::future::Future<Output = std::result::Result<(), mediasoup::worker::RequestError>>>`
note: required because it's used within this `async` fn body
    --> /home/paul/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mediasoup-0.15.0/src/router.rs:1446:50
     |
1446 |       ) -> Result<PipeTransportPair, RequestError> {
     |  __________________________________________________^
1447 | |         let PipeToRouterOptions {
1448 | |             router,
1449 | |             listen_info,
...    |
1518 | |         })
1519 | |     }
     | |_____^
     = note: required because it captures the following types: `&mediasoup::prelude::Router`, `mediasoup::prelude::PipeToRouterOptions`, `std::sync::Arc<async_lock::mutex::Mutex<std::option::Option<mediasoup::router::WeakPipeTransportPair>>>`, `async_lock::mutex::MutexGuard<'_, std::option::Option<mediasoup::router::WeakPipeTransportPair>>`, `async_lock::mutex::Lock<'_, std::option::Option<mediasoup::router::WeakPipeTransportPair>>`, `std::option::Option<mediasoup::router::PipeTransportPair>`, `impl std::future::Future<Output = std::result::Result<mediasoup::router::PipeTransportPair, mediasoup::worker::RequestError>>`
note: required because it's used within this `async` fn body
    --> /home/paul/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mediasoup-0.15.0/src/router.rs:1407:50
     |
1407 |       ) -> Result<PipeTransportPair, RequestError> {
     |  __________________________________________________^
1408 | |         // Here we may have to create a new PipeTransport pair to connect source and
1409 | |         // destination Routers. We just want to keep a PipeTransport pair for each
1410 | |         // pair of Routers. Since this operation is async, it may happen that two
...    |
1440 | |         Ok(pipe_transport_pair)
1441 | |     }
     | |_____^
     = note: required because it captures the following types: `&mediasoup::prelude::Router`, `mediasoup::prelude::ProducerId`, `mediasoup::prelude::PipeToRouterOptions`, `mediasoup::prelude::Producer`, `mediasoup::router::PipeTransportPair`, `impl std::future::Future<Output = std::result::Result<mediasoup::router::PipeTransportPair, mediasoup::worker::RequestError>>`, `mediasoup::prelude::Consumer`, `std::pin::Pin<std::boxed::Box<dyn std::future::Future<Output = std::result::Result<mediasoup::prelude::Consumer, mediasoup::prelude::ConsumeError>> + std::marker::Send>>`, `std::pin::Pin<std::boxed::Box<dyn std::future::Future<Output = std::result::Result<mediasoup::prelude::Producer, mediasoup::prelude::ProduceError>> + std::marker::Send>>`
note: required because it's used within this `async` fn body
    --> /home/paul/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mediasoup-0.15.0/src/router.rs:1047:70
     |
1047 |       ) -> Result<PipeProducerToRouterPair, PipeProducerToRouterError> {
     |  ______________________________________________________________________^
1048 | |         debug!("pipe_producer_to_router()");
1049 | |
1050 | |         if pipe_to_router_options.router.id() == self.id() {
...    |
1172 | |         })
1173 | |     }
     | |_____^
     = note: required because it captures the following types: `mediasoup::prelude::Worker`, `std::vec::Vec<mediasoup::prelude::RtpCodecCapability>`, `mediasoup::prelude::Router`, `impl std::future::Future<Output = std::result::Result<mediasoup::prelude::Router, mediasoup::worker::CreateRouterError>>`, `mediasoup::prelude::Router`, `impl std::future::Future<Output = std::result::Result<mediasoup::prelude::Router, mediasoup::worker::CreateRouterError>>`, `mediasoup::prelude::WebRtcTransport`, `impl std::future::Future<Output = std::result::Result<mediasoup::prelude::WebRtcTransport, mediasoup::worker::RequestError>>`, `mediasoup::prelude::Producer`, `mediasoup::prelude::RtpParameters`, `std::pin::Pin<std::boxed::Box<dyn std::future::Future<Output = std::result::Result<mediasoup::prelude::Producer, mediasoup::prelude::ProduceError>> + std::marker::Send>>`, `impl std::future::Future<Output = std::result::Result<mediasoup::prelude::PipeProducerToRouterPair, mediasoup::prelude::PipeProducerToRouterError>>`
note: required because it's used within this `async` fn body
    --> src/bin/two.rs:9:67
     |
9    |   async fn do_mediasoup_stuff(worker: Worker) -> anyhow::Result<()> {
     |  ___________________________________________________________________^
10   | |     let media_codecs = vec![RtpCodecCapability::Audio {
11   | |         mime_type: MimeTypeAudio::Opus,
12   | |         preferred_payload_type: None,
...    |
75   | |     Ok(())
76   | | }
     | |_^
note: required by a bound in `tokio::spawn`
    --> /home/paul/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.35.1/src/task/spawn.rs:166:21
     |
164  |     pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
     |            ----- required by a bound in this function
165  |     where
166  |         F: Future + Send + 'static,
     |                     ^^^^ required by this bound in `spawn`

error: could not compile `mediasoup-send` (bin "two") due to previous error
```
