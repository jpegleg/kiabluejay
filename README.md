![cdlogo](https://carefuldata.com/images/cdlogo.png)

# kiabluejay

Kiabluejay is fast and security focused, leveraging Actix for extremely fast HTTP framework and Tokio industry leading performance. Kiabluejay uses aws-lc-rs with RusTLS for SSL/TLS cryptography. It enables web serving with hybrid PQC via RusTLS with aws-lc-rs. Kiabluejay has JSON HTTP event logging, configurable headers, as well as simple session cookies.

The use of the cookies is optional, but they are an available configurable content gate feature. The "session" feature enables secure cookies, regular cookies, the ability to set required headers to get cookies, the ability to set required client source IP addresses to get cookies, and protected content that requires a cookie to access.

Some headers are not configurable and required, set automatically by kiabluejay for security reasons:

```
    "x-content-type-options": "nosniff"
    "x-frame-options": "SAMEORIGIN"
    "x-xss-protection": "1; mode=block"
```

Any other headers can be set via the `morph.yaml` headers section. See the example YAML below.

Configure an Actix async IO server for one or more listeners for a single set of web files by using the `morph.yaml` file.

## Here is a config example that doesn't use the special session cookie features

### This is the "The Normal Style" that is recommended for general use

This is a standard reference config. Your javascript, html, css, images, videos, and so on goes within "static_dir" directory (folder), which might be
mounted into the container, added into the container, or used directly from the filesystem. When running in a container, we can ConfigMap all of the web content,
or mount it from whatever storage system is needed, such as the local file system or a remote distributed storage system.

Customize the rewrites and directories (folders) to whatever is needed.
Adjust the content-security-policy to match the needs of the web code or remove it. Tune the cache-control
to meet the needs of the site, this reference enables visitor browsers to cache for 600 seconds.

```
workers: 1

web:
  static_dir: /var/www/html/
  rewrites:
    /docs: /docs/index.html
    /about: /about.html
    /shows: /shows.html
    /art:   /art.html
    /music: /music.html
    /:      /index.html
  headers:
    cache-control: "max-age: '600'"
    referrer-policy: "strict-origin-when-cross-origin"
    cross-origin-opener-policy: "same-origin"
    cross-origin-embedder-policy: "require-corp"
    content-security-policy: "style-src 'self' 'unsafe-inline' https:;img-src 'self' data: https:;font-src 'self' https:;connect-src 'self' https:;object-src 'none';base-uri 'none';frame-ancestors 'none';form-action 'self';upgrade-insecure-requests;"
    permissions-policy: "geolocation=(),camera=(),microphone=(),payment=(),usb=(),fullscreen=(self)"
    strict-transport-security: "max-age=63072000; includeSubDomains; preload"
  session:
    enabled: false
listeners:
  - port: 443
    tls:
      cert_path: /opt/morpho/cert.pem
      key_path: /opt/morpho/key.pem
  - port: 80
```

<i>Note: In version of kiabluejay prior to v0.2.0, the "pages" section of the config was always required, even if sessions were disabled.</i>

A listener is created for each configured port. TLS (HTTPS) can be enabled on any port by supplying `tls:` and the `key_path` and `cert_path` pointing to PEM files
for the web server to use. The cert.pem is likely the leaf and intermediate. Redirection to TLS and strict transport security are enabled.

The `static_dir` points to the web root on the file system.

The `rewrites` section enables configurable rewrites.

The `worker` count sets the number of worker threads to spawn during initialization. You might do 1 or 2 workers per vCPU. When in doubt, use 2.

The TLS ciphers are automatically set (via RusTLS), so there is no need to configure them. They are set to optimal secure defaults that get A+ reports. The reference config also has A+ industry standard headers as of June 2026. Headers and TLS change with the industry over time, and kiabluejay intends to keep up with the latest and greatest, within reason. The code is actively maintained as much as time permits.

### old, new, dev - different workflows and approaches available

For an "old classic" approach, the web content can be packaged in a .ZIP archive and unpacked on a server into the directory (folder) we configure for "static_dir" on the server.
For a "new cloud native" approach, the web content can be managed via Kubernetes API so that each container is spawned with the required content mounted into it.

For a web developer working locally, we can run kiabluejay on the CLI and use it while we develop our javascript and web content. While not strictly required, developing with kiabluejay locally enables us to write the morph.yaml most effectively, especially for aspects like content security policy which is specific to the web code requirements. Once established, the morph.yaml and (zip or tar archive, etc) of the web code and content can be used to deploy to the server or cloud system, or handed off to other teams for adoption, etc etc. If we don't want to run our development kiabluejay as root, then change the port to something else like 3443.

```
...CUT...
listeners:
  - port: 3443
    tls:
      cert_path: cert.pem
      key_path: key.pem

```
When developing with self-signed certificates, you will of course need to skip trust checks. That is okay for local dev tests. Then we can open the web browser to "https://localhost:3443" and skip through the browser warnings for the self-signed untrusted cert, and view our dev version of the site.

Then our code editor can edit the files and we can simply refresh the web browser to see the changes, kiabluejay hot reloads content so it can run while we change the code and files.


## features of kiabluejay: sessions, cookie requirements, and the morph.yaml

The `morph.yaml` file is read from the working directory of the kiabluejay process, the `$(pwd)`. This enables multiple processes to `cd` into different directories and have different `morph.yaml` config files. This technique is used more heavily by the OpenBSD fork. The container style kiabluejay doesn't need this since the filesystem is that mounted within the container.

Kiabluejay is a TLS and security focused server, HTTP listeners on any port will try to redirect to HTTPS 443. Any port can be used and configured as a TLS endpoint.

The config structure has changed with v0.2.0, now "pages" is nested within "session" and is only used if session is "enabled: true". Version 0.2.0 also expands the session features significantly, enabling secure cookies, header requirements for getting a cookie, and a configurable integer "value". This "value" integer is the value below what is required for `/session?fage=value` to issue a cookie. The max value for `fage` is 32767.

Here is an example config that enables many secure defaults regarding content security policy and strict transport security, and uses the session features.


```
workers: 1

web:
  static_dir: /var/www/html/
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
    pages:
      index_first_visit: "index.html"
      index_returning_visit: "index2.html"
      session_age_gt_value: "index2.html"
      session_age_lte_value: "index3.html"
    ttl_hours: 2
    value: 20
    secure_cookie: true
    key_path: /opt/kiabluejay/crypt/cookie_signer.pem
listeners:
  - port: 443
    tls:
      cert_path: /opt/morpho/cert.pem
      key_path: /opt/morpho/key.pem
  - port: 80
```

As of version 0.2.0 we can also make specific headers or specific headers with specific values required to access the `/session` context that issues cookies.


```
workers: 1

web:
  static_dir: /var/www/html/
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
    pages:
      index_first_visit: "index.html"
      index_returning_visit: "index2.html"
      session_age_gt_value: "index2.html"
      session_age_lte_value: "index3.html"
    ttl_hours: 2
    value: 20
    secure_cookie: true
    key_path: /opt/kiabluejay/crypt/cookie_signer.pem
    required:
      header:
        name: "wesetthisforreasons"
        value: "flavoroftheweek"
listeners:
  - port: 443
    tls:
      cert_path: /opt/morpho/cert.pem
      key_path: /opt/morpho/key.pem
  - port: 80
```

With version `0.2.1` and onward, we also have "ipv4" and "ipv6" options for "required", to limit access based on IP to `/session` cookies.

While this isn't how a regular public website should be, if the system is designed so that specific IP addresses are used, then we can
configure these as required in kiabluejay, as of v0.2.1.

```
workers: 1

web:
  static_dir: /var/www/html/
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
    pages:
      index_first_visit: "index.html"
      index_returning_visit: "index2.html"
      session_age_gt_value: "index2.html"
      session_age_lte_value: "index3.html"
    ttl_hours: 2
    value: 20
    secure_cookie: true
    key_path: /opt/kiabluejay/crypt/cookie_signer.pem
    required:
      header:
        name: "wesetthisforreasons"
        value: "flavoroftheweek"
      ipv4:
        addresses:
        - 127.0.0.1
        - 192.168.1.0/24
listeners:
  - port: 443
    tls:
      cert_path: /opt/morpho/cert.pem
      key_path: /opt/morpho/key.pem
  - port: 80
```

#### About the web code, the HTML and javascript and how it can use kiabluejay

The most simple and normal way to use kiabluejay is to serve up a "web root" of files from a "directory" (folder).
The files can be any javascript, HTML, CSS, images, videos, etc, etc. And kiabluejay has "session" disabled, no restrictions.
In this case the content is still served up with strong hybrid PQC TLS, enabling transport security and application security features with security headers.

For those that need protected content, there are additional features that can be enabled, but the web code must also work with it.

The index.html could then use `<form action="/session">` and submit session information `?fage=55` to submit an age of 55. If we have "required" with a "header" configured, the configured header name and/or header name with a specific value must be present in the request to `/session`, otherwise the session is denied with HTTP 403. This header feature can be used as a layer to slow down crawlers, bots, and those attempting to access protected content. If only the header name is configured, then the check is for the existence of that header, no matter what the value is set to. If we add a configured value, then that header name with that value must be used to access `/session`.

The web code HTML form can submit a required header like this:

```
<script>
document.getElementById('groupidForm').addEventListener('submit', function (e) {
  e.preventDefault();
  var groupIdValue = document.getElementById('groupid').value;
  var encoder = new TextEncoder();
  if (encoder.encode(groupIdValue).length > 512) {
    alert('Group ID must be 512 bytes or fewer.');
    return;
  }
  var url = this.action + '?fage=99';
  fetch(url, {
    method: 'GET',
    headers: {
      'grpid00a': groupIdValue
    }
  })
  .then(function (response) {
    if (response.redirected) {
      window.location.href = response.url;
    } else {
      return response.text().then(function (text) {
        document.open();
        document.write(text);
        document.close();
      });
    }
  })
  .catch(function (err) {
    console.error('Submission failed:', err);
  });
});
</script>
```

And have the header be provided by a user in the `index.html` file. In this example, the "age" is simply
sent as 99 for all requests, and it is the required header that is used in the check before providing a cookie.

```
    <form id="groupidForm" action="/session" method="get">
      <h1>Group ID Required</h1>
      <div class="rule"></div>
      <label for="groupid">Enter your Group ID</label>
      <input type="password" required="required" id="groupid" name="groupid" maxlength="512" autocomplete="off">
      <input type="hidden" id="fage" name="fage" value="99">
      <div class="submit-row">
        <input type="submit" value="Enter">
      </div>
    </form>
```

And then in our `morph.yaml` we would have a stanza for requiring that header:

```
...CUT...
  session:
    enabled: true
    pages:
      index_first_visit: "index.html"
      index_returning_visit: "index2.html"
      session_age_gt_value: "index2.html"
      session_age_lte_value: "index3.html"
    ttl_hours: 2
    value: 20
    required:
      header:
        name: "grpid00a"
        value: "Every owl rides the moon cow express to dairy town."
...CUT...
```


The "value" config options within sessions is the number one less than the required number to get a cookie. So when we use "20" for "value", that sets the value required submitted value to be 21 or greater to get a cookie issued.

The cookie signing key is to be any sufficiently strong 64 bytes or larger. The raw bytes from the file are used as seed into the transform to the secret used in HMAC for the secure cookies feature. The cookie middlware is entirely provided by Actix.

<b>Important note: when using "sessions", the "index_first_visit" page must be self contained because assets outside of that file will not load without a session cookie.
This means that any CSS, javascript, etc must be inside that "index_first_visit" file.</b>

If we disable "sessions" by setting "enabled: false" then we can skip the cookie requirements on the content, otherwise requests without a session cookie are sent back to our "index_first_visit" page.


### The "kiastack"

The "kiastack" is a collection of free software used together to manage application networking, transport security, network protocol support, fail over, security, and ultimately content serving.
The kiastack services are primarily written in Rust but there is one in Go.

<b>The full kiastack</b>
```
- kiagateway: SNI and Host header application gateway
- kiaproxy: failover provider
- kiabluejay: actix web server [ protected content, security and performance focus ]
- kiamagpie: go web server [ QUIC protocol, plain HTTP, remote caching, and multi-domain features ]
- redirectrix: actix web server [ ACME HTTP-01 enablement ]
```

See [kiaproxy](https://github.com/jpegleg/kiaproxy) and [kiagateway](https://github.com/jpegleg/kiagateway/) for networking support.
Also see kiabluejay's cousin [kiamagpie](https://github.com/jpegleg/kiamagpie) which does caching, QUIC, and cert hot-reloading.
All together they are the kiastack and can handle domain routing, failover, and many different kinds of web serving needs while having a strong security posture and being high performance.

Kiamagpie has different features and is more flexible in configuration, focusing on hot reloading of certificates and keys, content caching, multi-protocol (including QUIC), and multi-domain support.
Kiabluejay is focused speed and security, cookie session enablement, and hot content reloading (not hot key and cert reloading).
Both have configurable redirects, have JSON event logs, and are multi-listener. The kiamagpie event logs are more comprehensive, while kiabluejay does have some non-JSON output for some
error conditions and is only tracking JSON events at the HTTP level, where as kiamagpie tracks events at the TCP level.

Kiabluejay will quickly panic if a certificate or key could not be loaded during initialization, this is intentional. Read output if the behavior is not what you expect and double check the mounts, morph.yaml values, file and directory names and permissions.

One of the design choices of kiabluejay is to isolate a web root to the scope of a process, where as kiamagpie supports multiplexed domains and handling of many separate web roots and websites within the same process. The kiabluejay approach in this regard helps with security, where as the kiamagpie approach helps with centralization and ease.

#### using the kiastack

Select any or all of the services and deploy in a Kubernetes cluster, in Docker containers, in VMs, or on baremetal.

The dockerhub releases are prebuilt containers for kiagateway, kiaproxy, kiabluejay, and kiamagpie.

In linux we are more likely to use the OCI container style, although running via systemd or rc works great too.

The OpenBSD style of use has examples in the [paludification_toad project](https://github.com/jpegleg/paludification_toad/) which diverge from the linux version (container versions) with additional OpenBSD security integrations and different cryptographic software for TLS.

<b>Generally we have kiagateway as internet facing first hop. The kiagateway routes domain specific traffic to kiaproxy instances which route traffic to kiabluejay instances.
</br>
Redirectrix is added as internet facing if we want to have a separate ACME HTTP-01 integration for PKI automation. Kiamagpie might be added as internet facing if we need to support QUIC protocol.
We'll generally want to use kiabluejay over kiamagpie, but kiamagpie has uses too.
</b>

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

The OpenBSD version starts at 0.1.700 (based on the 0.1.7 version) and uses this project as an upstream source.

The OpenBSD version does not use aws-lc-rs, and instead uses (libressl) via openssl integration for the TLS cryptography.

The OpenBSD fork also has PEM decryption for the TLS identity private key PEM file, via libressl (openssl).


## Project promises

This project will never use AI-slop. All code is reviewed, tested, and implemented by a human expert. This repository and the crates.io repository are carefully managed and protected.

This project will be maintained as best as is reasonable.
