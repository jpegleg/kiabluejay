use std::collections::HashMap;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;
use std::env;
use std::fs::File;
use std::io::{BufReader, Write};

use actix_files::NamedFile;
use actix_session::{
    config::PersistentSession, storage::CookieSessionStore, Session, SessionMiddleware,
};
use actix_web::{
    cookie::{self, Key},
    get,
    middleware,
    web,
    App, HttpRequest, HttpServer,
};
use actix_web_lab::header::StrictTransportSecurity;
use actix_web_lab::middleware::RedirectHttps;
use chrono::prelude::*;
use rustls::crypto::{aws_lc_rs as provider, CryptoProvider};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::server::WebPkiClientVerifier;
use rustls::{self};
use log::LevelFilter;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    workers: Option<usize>,
    web: WebConfig,
    listeners: Vec<ListenerConfig>,
}

#[derive(Deserialize, Clone)]
struct WebConfig {
    static_dir: String,
    #[serde(default)]
    rewrites: HashMap<String, String>,
    pages: PageConfig,
}

#[derive(Deserialize, Clone)]
struct PageConfig {
    index_first_visit: String,
    index_returning_visit: String,
    session_age_gt_20: String,
    session_age_lte_20: String,
}

#[derive(Deserialize)]
struct ListenerConfig {
    port: u16,
    tls: Option<TlsConfig>,
}

#[derive(Deserialize)]
struct TlsConfig {
    cert_path: String,
    key_path: String,
}

#[derive(serde::Deserialize)]
struct Age {
    pub fage: i32,
}

#[derive(Clone)]
struct AppState {
    static_dir: PathBuf,
    rewrites: HashMap<String, String>,
    pages: PageConfig,
}

#[get("/session")]
async fn newcook(
    info: web::Query<Age>,
    session: Session,
    state: web::Data<Arc<AppState>>,
) -> actix_web::Result<NamedFile> {
    let id = info.fage;
    let mut counter = 0;

    if id > 20 {
        if let Ok(Some(count)) = session.get::<i32>("counter") {
            counter = count + 1;
            let _ = session.insert("counter", counter);
        } else {
            let _ = session.insert("counter", counter);
        }

        open_configured_file(
            &state.static_dir,
            &state.pages.session_age_gt_20,
        )
        .await
    } else {
        open_configured_file(
            &state.static_dir,
            &state.pages.session_age_lte_20,
        )
        .await
    }
}

#[get("/")]
async fn index(
    session: Session,
    state: web::Data<Arc<AppState>>,
) -> actix_web::Result<NamedFile> {
    let mut counter = 0;

    if let Ok(Some(_count)) = session.get::<i32>("counter") {
        if let Ok(Some(count)) = session.get::<i32>("counter") {
            counter = count + 1;
            let _ = session.insert("counter", counter);
        } else {
            let _ = session.insert("counter", counter);
        }

        open_configured_file(
            &state.static_dir,
            &state.pages.index_returning_visit,
        )
        .await
    } else {
        open_configured_file(
            &state.static_dir,
            &state.pages.index_first_visit,
        )
        .await
    }
}

async fn static_with_rewrites(
    req: HttpRequest,
    state: web::Data<Arc<AppState>>,
) -> actix_web::Result<NamedFile> {
    let request_path = req.path();

    let rewritten = state
        .rewrites
        .get(request_path)
        .map(String::as_str)
        .unwrap_or(request_path);

    let full_path = state.static_dir.join(rewritten);

    NamedFile::open_async(full_path)
        .await
        .map_err(|_| actix_web::error::ErrorNotFound("file not found"))
}

fn sanitize_relative_path(input: &str) -> Option<PathBuf> {
    let trimmed = input.trim_start_matches('/');
    let path = Path::new(trimmed);

    let mut clean = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => clean.push(part),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => return None,
        }
    }

    Some(clean)
}

async fn open_configured_file(
    static_dir: &Path,
    relative_path: &str,
) -> actix_web::Result<NamedFile> {
    let safe_rel_path = sanitize_relative_path(relative_path)
        .ok_or_else(|| actix_web::error::ErrorBadRequest("invalid configured path"))?;

    let full_path = static_dir.join(safe_rel_path);

    NamedFile::open_async(full_path)
        .await
        .map_err(|_| actix_web::error::ErrorNotFound("file not found"))
}

fn load_certs(filename: &str) -> Vec<CertificateDer<'static>> {
    let certfile = File::open(filename).expect("cannot open certificate file");
    let mut reader = BufReader::new(certfile);
    rustls_pemfile::certs(&mut reader)
        .map(|result| result.unwrap())
        .collect()
}

fn load_private_key(filename: &str) -> PrivateKeyDer<'static> {
    let keyfile = File::open(filename).expect("cannot open private key file");
    let mut reader = BufReader::new(keyfile);

    loop {
        match rustls_pemfile::read_one(&mut reader).expect("cannot parse private key .pem file") {
            Some(rustls_pemfile::Item::Pkcs1Key(key)) => return key.into(),
            Some(rustls_pemfile::Item::Pkcs8Key(key)) => return key.into(),
            Some(rustls_pemfile::Item::Sec1Key(key)) => return key.into(),
            None => break,
            _ => {}
        }
    }

    panic!(
        "no keys found in {:?} (encrypted keys not supported)",
        filename
    );
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> eyre::Result<()> {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .filter_module("actix_server", LevelFilter::Warn)
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();
    let readi: String = Utc::now().to_rfc3339();
    let runid = env::var("RUN_ID").unwrap_or("kiabluejay".to_string());
    log::info!(
        "{{\"event\":\"initialized version 0.1.5\",\"time\":\"{}\",\"run_id\":\"{}\"}}",
        readi,
        runid
    );
    let config_file = File::open("morph.yaml").expect("Failed to open morph.yaml");
    let config: Config = serde_yml::from_reader(config_file).expect("failed to read morph.yaml");
    let state = Arc::new(AppState {
        static_dir: PathBuf::from(config.web.static_dir.clone()),
        rewrites: config.web.rewrites.clone(),
        pages: config.web.pages.clone(),
    });

    loop {
        let readi: String = Utc::now().to_rfc3339();
        log::info!(
            "{{\"event\":\"configuration_loaded\",\"workers\":\"{}\",\"listeners\":\"{}\",\"timestamp\":\"{}\",\"run_id\":\"{}\"}}",
            config.workers.unwrap_or(2),
            config.listeners.len(),
            readi,
            runid
        );

        let mut server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(state.clone()))
                .wrap(RedirectHttps::default())
                .wrap(RedirectHttps::with_hsts(StrictTransportSecurity::recommended()))
                .wrap(middleware::DefaultHeaders::new().add(("x-content-type-options", "nosniff")))
                .wrap(middleware::DefaultHeaders::new().add(("x-frame-options", "SAMEORIGIN")))
                .wrap(middleware::DefaultHeaders::new().add(("x-xss-protection", "1; mode=block")))
                .wrap(middleware::Logger::new(
                    "{\"event\":\"ingress_http\",\"client_address\":\"%a\",\"request_start_time\":\"%t\",\"HTTP\":\"%s\",\"http_request_first_line\":\"%r\",\"size\":\"%b\",\"server_time\":\"%T\",\"referer\":\"%{Referer}i\",\"user_agent\":\"%{User-Agent}i\",\"run_id\":\"%{RUN_ID}e\"}"
                ))
                .wrap(
                    SessionMiddleware::builder(
                        CookieSessionStore::default(),
                        Key::from(&[0; 64]),
                    )
                    .cookie_secure(false)
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl(cookie::time::Duration::hours(2)),
                    )
                    .build(),
                )
                .service(index)
                .service(newcook)
                .route("/{tail:.*}", web::get().to(static_with_rewrites))
        });

        for listener in config.listeners {
            let addr = format!("0.0.0.0:{}", listener.port);

            match listener.tls {
                Some(tls) => {
                    let cert = load_certs(&tls.cert_path);
                    let key = load_private_key(&tls.key_path);
                    let versions = rustls::ALL_VERSIONS.to_vec();
                    // ocsp here is required but unused, empty vector
                    let ocsp = Vec::new();
                    let client_auth = WebPkiClientVerifier::no_client_auth();

                    let tls_config = rustls::ServerConfig::builder_with_provider(
                        CryptoProvider {
                            cipher_suites: provider::ALL_CIPHER_SUITES.to_vec(),
                            ..provider::default_provider()
                        }
                        .into(),
                    )
                    .with_protocol_versions(&versions)
                    .expect("inconsistent cipher-suites/versions specified")
                    .with_client_cert_verifier(client_auth)
                    .with_single_cert_with_ocsp(cert, key, ocsp)
                    .expect("bad certificates/private key");

                    server = server.bind_rustls_0_23(&addr, tls_config)?;
                    let listeni: String = Utc::now().to_rfc3339();
                    log::info!(
                        "{{\"event\":\"server_listening_https\",\"addr\":\"{}\",\"time\":\"{}\",\"run_id\":\"{}\"}}",
                        addr,
                        listeni,
                        runid
                    );
                }
                None => {
                    server = server.bind(&addr)?;
                    let listeni: String = Utc::now().to_rfc3339();
                    log::info!(
                        "{{\"event\":\"server_listening_http\",\"addr\":\"{}\",\"time\":\"{}\",\"run_id\":\"{}\"}}",
                        addr,
                        listeni,
                        runid
                    );
                }
            }
        }

        let workers = config.workers.unwrap_or(2);
        server = server.workers(workers);
        let mut server = server.run();
        let _ = server.handle();

        tokio::select! {
            res = &mut server => {
                let stopi: String = Utc::now().to_rfc3339();
                log::info!(
                    "{{\"event\":\"server_shutdown_arrived\",\"time\":\"{}\",\"run_id\":\"{}\"}}",
                    stopi,
                    runid
                );
                res?;
                break;
            }
        }
    }

    Ok(())
}
