![cdlogo](https://carefuldata.com/images/cdlogo.png)

# kiabluejay

Kiabluejay is fast and security focused. It enables web serving with hybrid PQC via RusTLS with aws-lc-rs, JSON HTTP event logging, as well as simple session cookies.

The use of the cookies is optional, but they are an available content age gate (age 21) feature that could be used for some other purposes, too. 

The related "page" config values are required in the YAML. This configuration is rather rigid in the current versions.

Use your intended index for "/" by configuring with index_first_visit as a "page" file within the "web" section of the `morph.yaml` file. 

The other three "pages" are only used if /session is used and thus cookies are created or denied.

The index_returning_visit is the page to display if a cookie is present.
The session_age_lte_20 is a configured page to display if a user supplies an age under 21.

Configure an Actix async IO server for one or more listeners for a single set of web files with the `morph.yaml` file.

Kiabluejay is a TLS and security focused server, HTTP listeners on any port will try to redirect to HTTPS 443.

```
workers: 1

web:
  static_dir: /var/www/html/
  pages:
    index_first_visit: "verify_age_landing_page.html"
    index_returning_visit: "index.html"
    session_age_gt_20: "index.html"
    session_age_lte_20: "notice.html"

listeners:
  - port: 443
    tls:
      cert_path: /opt/morpho/cert.pem
      key_path: /opt/morpho/key.pem
  - port: 80

rewrites:
  "/about": "/about.html"
```
The login.html could then use `<form action="/session">` and submit session information `?fage=55` to submit an age of 55.

A listener is created for each configured port. TLS (HTTPS) can be enabled on any port by supplying `tls:` and the `key_path` and `cert_path` pointing to PEM files
for the web server to use. The cert.pem is likely the leaf and intermediate. Redirection to TLS and strict transport security are enabled.

The `static_dir` points to the web root on the file system.

The `rewrites` section enables configurable rewrites.

The `worker` count sets the number of worker threads to spawn during initialization. You might do 1 or 2 workers per vCPU. When in doubt, use 2.

Also see [kiaproxy](https://github.com/jpegleg/kiaproxy) and [kiagateay](https://github.com/jpegleg/kiagateway/) for networking support.
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

export RUN_ID=$(cat /dev/urandom | base64 | head -n2 | tail -n1 | cut -c1-32) # pass whatever value you want to be logged as "run_id" which each log event
# the default run_id is "kiabluejay"

podman run -d -it --network=host \
  -v /opt/kiamagpie_live/morph.yaml:/morph.yaml \
  -v /var/www/html/:/var/www/html/
  -v /opt/kiamagpie_crypt/:/opt/crypt/ \
  carefuldata/kiabluejay:latest

```

Installing via Cargo:

```
cargo install kiabluejay
```

Kiagbluejay can also be compiled from source or installed from precompiled release binaries via github.

Kiabluejay works well in Kubernetes, too, just specify the `morph.yaml` config and file mounts in the manifest, etc etc.


## Project promises

This project will never use AI-slop. All code is reviewed, tested, and implemented by a human expert. This repository and the crates.io repository are carefully managed and protected.

This project will be maintained as best as is reasonable.
