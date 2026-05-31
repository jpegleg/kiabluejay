![cdlogo](https://carefuldata.com/images/cdlogo.png)

# kiabluejay

Kiabluejay is fast and security focused, leveraging Actix for extremely fast HTTP framework and Tokio industry leading performance. Kiabluejay uses aws-lc-rs with RusTLS for SSL/TLS cryptography. It enables web serving with hybrid PQC via RusTLS with aws-lc-rs. Kiabluejay has JSON HTTP event logging, configurable headers, as well as simple session cookies.

The use of the cookies is optional, but they are an available content age gate (age 21) feature, that could be used for some other purposes, too. 

Some headers are not configurable and required, set automatically by kiabluejay for security reasons:

```
    "x-content-type-options": "nosniff"
    "x-frame-options": "SAMEORIGIN"
    "x-xss-protection": "1; mode=block"
```

Any other headers can be set via the `morph.yaml` headers section. See the example YAML below.

Configure an Actix async IO server for one or more listeners for a single set of web files by using the `morph.yaml` file.

Kiabluejay is a TLS and security focused server, HTTP listeners on any port will try to redirect to HTTPS 443.

```
workers: 1

web:
  static_dir: /var/www/html/
  pages:
    index_first_visit: "index.html"
    index_returning_visit: "index2.html"
    session_age_gt_20: "index2.html"
    session_age_lte_20: "index3.html"
  rewrites:
    /docs: /docs/index.html
    /about: /about.html
    /shows: /shows.html
    /art:   /art.html
    /music: /music.html
    /:      /index.html
  headers:
    cache-control: "max-age: '1200'"
    referrer-policy: "strict-origin-when-cross-origin"
    cross-origin-opener-policy: "same-origin"
    cross-origin-embedder-policy: "require-corp"
    content-security-policy: "style-src 'self' 'unsafe-inline' https:;img-src 'self' data: https:;font-src 'self' https:;connect-src 'self' https:;object-src 'none';base-uri 'none';frame-ancestors 'none';form-action 'self';upgrade-insecure-requests;"
    permissions-policy: "geolocation=(),camera=(),microphone=(),payment=(),usb=(),fullscreen=(self)"
    strict-transport-security: "max-age=63072000; includeSubDomains; preload"
  session:
    enabled: true
    ttl_hours: 2
listeners:
  - port: 443
    tls:
      cert_path: /opt/morpho/cert.pem
      key_path: /opt/morpho/key.pem
  - port: 80
```
The index.html could then use `<form action="/session">` and submit session information `?fage=55` to submit an age of 55.

<b>Important note: when using "sessions", the "index_first_visit" page must be self contained because assets outside of that file will not load without a session cookie.
This means that any CSS, javascript, etc must be inside that "index_first_visit" file.</b>

If we disable "sessions" by setting "enabled: false" then we can skip the cookie requirements on the content, otherwise requests without a session cookie are sent back to our "index_first_visit" page.



A listener is created for each configured port. TLS (HTTPS) can be enabled on any port by supplying `tls:` and the `key_path` and `cert_path` pointing to PEM files
for the web server to use. The cert.pem is likely the leaf and intermediate. Redirection to TLS and strict transport security are enabled.

The `static_dir` points to the web root on the file system.

The `rewrites` section enables configurable rewrites.

The `worker` count sets the number of worker threads to spawn during initialization. You might do 1 or 2 workers per vCPU. When in doubt, use 2.

Also see [kiaproxy](https://github.com/jpegleg/kiaproxy) and [kiagateway](https://github.com/jpegleg/kiagateway/) for networking support.
Also see kiabluejay's cousin [kiamagpie](https://github.com/jpegleg/kiamagpie) which does caching, QUIC, and cert hot-reloading.
All together they are the kiastack and can handle domain routing, failover, and many different kinds of web serving needs while having a strong security posture and being high performance.

Kiamagpie has more features and is more flexible in configuration, focusing on hot reloading of certificates and keys, content caching, multi-protocol (including QUIC), and multi-domain support.
Kiabluejay is focused speed and security, cookie enablement for simple number logic in the web forms, and hot content reloading (not hot key and cert reloading).
Both have configurable redirects, have JSON event logs, and are multi-listener. The kiamagpie event logs are more comprehensive, while kiabluejay does have some non-JSON output for some
error conditions and is only tracking JSON events at the HTTP level, where as kiamagpie tracks events at the TCP level.

Kiabluejay will quickly panic if a certificate or key could not be loaded during initialization, this is intentional. Read output if the behavior is not what you expect and double check the mounts, morph.yaml values, file and directory names and permissions.

## Installation

Kiabluejay is available on [github](https://github.com/jpegleg/kiabluejay), [crates.io](https://crates.io/crates/kiabluejay), and [docker hub](https://hub.docker.com/r/carefuldata/kiabluejay).

The container image is very small and hardened, with only a single statically linked Rust binary added to a minimized container "scratch" image.

Here is an example of pulling the image from docker hub and running via Podman or Docker:

```
podman pull docker.io/carefuldata/kiabluejay:latest

export RUN_ID="$(cat /dev/random | head -n2 | base64 | tr -d '\n' | cut -c 1-12)"# pass whatever value you want to be logged as "run_id" which each log event
# the default run_id is "kiabluejay" but if default will only log at init, not http event, default run_id of an http event is "-".

podman run -d -it --network=host \
  --env RUN_ID \
  -v /opt/kiamagpie_live/morph.yaml:/morph.yaml \
  -v /var/www/html/:/var/www/html/
  -v /opt/kiamagpie_crypt/:/opt/crypt/ \
  carefuldata/kiabluejay:latest

```

Installing via Cargo:

```
cargo install kiabluejay
```

Kiabluejay can also be compiled from source or installed from precompiled release binaries via github.

Kiabluejay works well in Kubernetes, too, just specify the `morph.yaml` config and file mounts in the manifest, etc etc.

#### Kiabluejay on OpenBSD

There is a version of Kiabluejay that integrates with Pledge and Unveil security:

https://github.com/jpegleg/paludification_toad/tree/main/morphobsd

The OpenBSD version starts at 0.1.700 and uses this project as an upstream source.


## Project promises

This project will never use AI-slop. All code is reviewed, tested, and implemented by a human expert. This repository and the crates.io repository are carefully managed and protected.

This project will be maintained as best as is reasonable.
