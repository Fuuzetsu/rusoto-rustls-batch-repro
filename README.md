To reproduce.

Firsts, check that everything works with native TLS. You'll need
RegisterJobDefinition and Deregister job definition credentials or role. Role is
optional.

```console
cargo run -- --role arn:aws:iam::YOURACCOUNTID:role/YOURROLENAME
```

Once confirmed that this works (no output, clean exit), use `rustls` instead:

```console
cargo run --no-default-features --features rustls -- --role arn:aws:iam::YOURACCOUNTID:role/YOURROLENAME
```

You will probably see something like this:

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: HttpDispatch(HttpDispatchError { message: "Error during dispatch: channel closed" })', src/main.rs:86:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

If you run with `RUST_LOG=trace`, we can see a difference between the two.

In native tls case:
```
[2021-02-04T00:40:16Z TRACE mio::poll] registering event source with poller: token=Token(2), interests=READABLE | WRITABLE
[2021-02-04T00:40:16Z TRACE want] signal: Want
[2021-02-04T00:40:16Z TRACE want] signal found waiting giver, notifying
[2021-02-04T00:40:16Z TRACE want] poll_want: taker wants!
[2021-02-04T00:40:16Z TRACE want] signal: Want
[2021-02-04T00:40:16Z TRACE want] poll_want: taker wants!
[2021-02-04T00:40:16Z TRACE want] signal: Want
[2021-02-04T00:40:16Z DEBUG rusoto_core::proto::json::payload] Response body: b"{\"jobDefinitionName\":\"rustls-failure-repro\",\"jobDefinitionArn\":\"arn:aws:batch:ap-northeast-1:822646120884:job-definition/rustls-failure-repro:3\",\"revision\":3}"
[2021-02-04T00:40:16Z DEBUG rusoto_core::proto::json::payload] Response status: 200 OK
[2021-02-04T00:40:16Z TRACE want] signal: Want
```

In `rustls` case:

```
[2021-02-04T00:39:09Z TRACE mio::poll] registering event source with poller: token=Token(2), interests=READABLE | WRITABLE
[2021-02-04T00:39:09Z DEBUG rustls::client::hs] No cached session for DNSNameRef("batch.ap-northeast-1.amazonaws.com")
[2021-02-04T00:39:09Z DEBUG rustls::client::hs] Not resuming any session
[2021-02-04T00:39:09Z TRACE rustls::client::hs] Sending ClientHello Message { <snip>
…
[2021-02-04T00:39:09Z DEBUG rustls::client::hs] ALPN protocol is Some(b"h2")
[2021-02-04T00:39:09Z DEBUG rustls::client::hs] Using ciphersuite TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256
[2021-02-04T00:39:09Z DEBUG rustls::client::hs] Server supports tickets
[2021-02-04T00:39:09Z DEBUG rustls::client::tls12] ECDHE curve is ECParameters { curve_type: NamedCurve, named_group: secp256r1 }
[2021-02-04T00:39:09Z TRACE rustls::client::tls12] Server cert is <snip>
…
[2021-02-04T00:39:09Z DEBUG rustls::client::tls12] Server DNS name is DNSName("batch.ap-northeast-1.amazonaws.com")
[2021-02-04T00:39:10Z DEBUG rustls::client::tls12] Session saved
[2021-02-04T00:39:10Z TRACE want] signal: Closed
[2021-02-04T00:39:10Z TRACE want] signal found waiting giver, notifying
[2021-02-04T00:39:10Z TRACE mio::poll] deregistering event source from poller
[2021-02-04T00:39:10Z TRACE want] poll_want: closed
[2021-02-04T00:39:10Z TRACE want] signal: Closed
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: HttpDispatch(HttpDispatchError { message: "Error during dispatch: channel closed" })', src/main.rs:87:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2021-02-04T00:39:10Z DEBUG rustls::session] Sending warning alert CloseNotify
[2021-02-04T00:39:10Z TRACE mio::poll] deregistering event source from poller
[2021-02-04T00:39:10Z TRACE want] signal: Closed
```
