![cdlogo](https://carefuldata.com/images/cdlogo.png)

# kiabluejay

Kiabluejay is fast and security focused, leveraging Actix for extremely fast HTTP framework and Tokio industry leading performance. Whether it is mission critical content serving, large scale content delivery, or for your personal project or small business website, kiabluejay handles heavy load reliably, and is easy to use with a single YAML config with code and docs that provide many secure defaults and clear instructions. Kiabluejay uses aws-lc-rs with RusTLS for SSL/TLS cryptography. It enables web serving with hybrid PQC via RusTLS with aws-lc-rs. Kiabluejay has JSON HTTP event logging, configurable headers, as well as session cookies.

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

The config structure changed again with v0.2.6, now "cookie_forbidden" page is a required config value when sessions are enabled.

Here is an example config that enables many good defaults and strict transport security, and uses the session features with secure session cookies. Using secure mode isn't always required for session cookies as they don't always represent authentication and may not be a problem if they are forged or tampered with. But if the cookie is for a security or authenticated content use, then a secure cookie should likely be used, and likely the cache-control value lowered or set to `no-cache` like this next example. It is up to the web code, which we'll demonstrate later in this document, to how the sessions feature is actually used.


```
workers: 2

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
    cache-control: "no-cache"
    referrer-policy: "strict-origin-when-cross-origin"
    cross-origin-opener-policy: "same-origin"
    cross-origin-embedder-policy: "require-corp"
    permissions-policy: "geolocation=(),camera=(),microphone=(),payment=(),usb=(),fullscreen=(self)"
    strict-transport-security: "max-age=63072000; includeSubDomains; preload"
  session:
    enabled: true
    pages:
      index_first_visit: "index.html"
      index_returning_visit: "index2.html"
      session_age_gt_value: "index2.html"
      session_age_lte_value: "index3.html"
      cookie_forbidden: "403.html"
    ttl_hours: 2
    value: 20
    secure_cookie: true
    key_path: /opt/kiabluejay/crypt/cookie_signer.pem
listeners:
  - port: 443
    tls:
      cert_path: /opt/morpho/cert.pem
      key_path: /opt/morpho/key.pem
```

As of version `0.2.0` we can also make specific headers or specific headers with specific values required to access the `/session` context that issues cookies.


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
    cache-control: "no-cache"
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
      cookie_forbidden: "403.html"
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
configure these as required in kiabluejay, as of `0.2.1`. 

Let's also change our cache example header to cache for 1200 seconds for this one, maybe because the failed cookie fetching caching condition isn't a factor for how the web code uses it and we can start using the cache more.

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
      cookie_forbidden: "403.html"
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

As of version `0.2.3` we can configure specific URI contexts to protect with sessions rather than using the default.

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
    contexts:
      - "index2.html"
      - "materials"
      - "shop"
      - "materials.html"
      - "shop.html"
      - "js/factory.js"
      - "images/00001.png'
      - "images/00002.png'
      - "css/depth.css"
    pages:
      index_first_visit: "index.html"
      index_returning_visit: "index2.html"
      session_age_gt_value: "index2.html"
      session_age_lte_value: "index3.html"
      cookie_forbidden: "403.html"
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

The "contexts" feature has been updated in `0.2.4` to glob match on configured rules rather than exact match. This is an important fix, `0.2.3` should not be used, use `0.2.4+` instead when using the "contexts" feature.

Version `0.2.5` adds a DELETE method to `/session` to enable logout functionality.

Example javascript that performs a logout (purge the cookie):

```
fetch('https://example.com:443/session?fage=1024', { method: 'DELETE', headers: { 'grpid00a': groupIdValue } })
```

The requirements on /session apply to both the creation of cookies and the deletion (purging) of them. So if we require specific headers and a fage value of greater than 1000 to get a cookie, then we'll also require those to purge the cookie.

There is a configured "cookie_forbidden" page that is required with "sessions" is enabled, as of `0.2.6`. This change enables users to define the page loaded when `/session` access is denied because of missed "required" requirements, such as wrong header value or unauthorized IP address. This doesn't change the "age" denied page that already exists as "session_age_lte_value". In previous versions, the message for these client requirements failures was "Forbidden, your request was not authorized.". Instead of that fixed message for those client error paths, in `0.2.6+` we configure the web page to display when a cookie request is rejected.

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
    contexts:
      - "index2.html"
      - "materials"
      - "shop"
      - "materials.html"
      - "shop.html"
      - "js/factory.js"
      - "images/00001.png'
      - "images/00002.png'
      - "css/depth.css"
    pages:
      index_first_visit: "index.html"
      index_returning_visit: "index2.html"
      session_age_gt_value: "index2.html"
      session_age_lte_value: "index3.html"
      cookie_forbidden: "status_codes/400_series/http_403.html"
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

Here is a realistic looking example config for sessions, like for online liquor sales that requires some specific values to be set to get to the 21+ content of the site. This version updates the headers to allow geolocation and payments features in the browser's permissions_policy.

```
workers: 8

web:
  static_dir: /var/www/html/
  rewrites:
    /docs: /docs/index.html
    /about: /about.html
    /shop:  /shop.html
    /materials: /materials.html
    /policies: /policies.html
    /security: /security.html
    /:      /index.html
  headers:
    cache-control: "max-age: '5'"
    referrer-policy: "strict-origin-when-cross-origin"
    cross-origin-opener-policy: "same-origin"
    cross-origin-embedder-policy: "require-corp"
    content-security-policy: "style-src 'self' 'unsafe-inline' https:;img-src 'self' data: https:;font-src 'self' https:;connect-src 'self' https:;object-src 'none';base-uri 'none';frame-ancestors 'none';form-action 'self';upgrade-insecure-requests;"
    permissions-policy: "camera=(),microphone=(),usb=(),fullscreen=(self)"
    strict-transport-security: "max-age=63072000; includeSubDomains; preload"
  session:
    enabled: true
    contexts:
      - "index2.html"
      - "materials"
      - "shop"
      - "materials.html"
      - "shop.html"
      - "js/factory.js"
      - "images/products_00001.png'
      - "images/products_00002.png'
      - "images/wine_of_the_month.png'
      - "images/products_00003.png'
      - "images/products_00004.png'
      - "images/products_00005.png'
      - "images/products_00006.png'
      - "images/products_00007.png'
      - "images/products_00008.png'
      - "images/products_00009.png'
      - "css/depth.css"
    pages:
      index_first_visit: "index.html"
      index_returning_visit: "index2.html"
      session_age_gt_value: "index2.html"
      session_age_lte_value: "index3.html"
      cookie_forbidden: "http_403.html"
    ttl_hours: 6
    value: 20
    secure_cookie: true
    key_path: 2026_cookie_hmac_03.bin
    required:
      header:
        name: "promocode"
listeners:
  - port: 443
    tls:
      cert_path: cert.pem
      key_path: key.pem
  - port: 80

```

_Notice how this example has a low cache value in the headers,`cache-control: "max-age: '5'"`. This is often a good choice when using "sessions" as it avoids the situations where invalid auth or post auth remained cached on the client side for longer than desired but still provides a small amount of caching for user flows. Having a longer cache life optimizes performance and utilization, but having a shorter cache life helps when auth state changes to the same pages/contexts might occur within the cache window. Having a short cache duration also helps if content frequently changes, so visitors get the updates quickly rather than waiting potentially hours or days to see a page update if the cache duration is long. Web browsers can get "stuck" with cached content, preventing successful auth after failed auth, or preventing newly updated content fixes from displaying._

In many cases we can just let the cookie expire, but if we want to include some "logout" functionality in the web app, the session purge feature from `0.2.5+` can be used for purging the "id" session cookie created by the GET on /session.

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
      cookie_forbidden: "403.html"
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
This means that any CSS, javascript, etc must be inside that "index_first_visit" file. The exception to this is if the "contexts" feature is used, then only the specified protected contexts matches will require the session cookie. This can enable resources outside of the login page to be loaded without a session cookie. By enabling "contexts" configuration, you are explicitly defining the protected content with each file (URI context pattern) that is protected behind the "sessions" feature.</b>

If we disable "sessions" by setting "enabled: false" then we can skip the  requirements on the content, otherwise requests without a session cookie are sent back to our "index_first_visit" page.


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
