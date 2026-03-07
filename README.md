![cdlogo](https://carefuldata.com/images/cdlogo.png)

# kiabluejay

Kiabluejay is very fast and secure, enabling hybrid PQC, as well as cheap session cookies.
The use of the cookies is optional, but they are an available age gate feature.

Configure an Actix async IO server for one or more listeners for a single set of web files with the `morph.yaml` file.

```
workers: 1

web:
  static_dir: /var/www/html/
  pages:
    index_first_visit: "login.html"
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
for the web server to use. The cert.pem is likely the leaf and intermediate.

The `static_dir` points to the web root on the file system.

The `rewrites` section enables configurable rewrites.

The `worker` count sets the number of worker threads to spawn during initialization. You might do 1 or 2 workers per vCPU. When in doubt, use 2.

Also see [kiaproxy](https://github.com/jpegleg/kiaproxy) and [kiagateay](https://github.com/jpegleg/kiagateway/) for networking support.
Also see kiabluejay's cousin [kiamagpie](https://github.com/jpegleg/kiagateway/) which does caching, QUIC, and cert hot-reloading.

## Installation

Kiabluejay is available on [github](https://github.com/jpegleg/kiabluejay), [crates.io](https://crates.io/crates/kiabluejay), and [docker hub](https://hub.docker.com/r/carefuldata/kiabluejay).

The container image is very small and hardened, with only a single statically linked Rust binary added to a minimized container "scratch" image.

Here is an example of pulling the image from docker hub and running via Podman or Docker:

```
podman pull docker.io/carefuldata/kiabluejay:latest

export RUN_ID=$(cat /dev/urandom | base64 | head -n2 | tail -n1 | cut -c1-32) # pass whatever value you want to be logged as "run_id" which each log event
# the default run_id is "kiabluejay"

podman run -d -it --network=host \
  -v /opt/kiamagpie_live:/morph.yaml \
  -v /var/www/html:/var/www/html/
  -v /opt/kiamagpie_crypt:/opt/crypt/ \
  carefuldata/kiabluejay

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
