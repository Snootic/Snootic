#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::config::get_configuration;
    use leptos::prelude::*;
    use leptos_meta::{HashedStylesheet, MetaTags};
    use portfolio::app::App;
    use portfolio::three::ImportMap;

    any_spawner::Executor::init_tokio().ok();

    let conf = get_configuration(None).map_err(std::io::Error::other)?;
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options.clone();

    println!("listening on http://{addr}");
    HttpServer::new(move || {
        let options = leptos_options.clone();
        let site_root = options.site_root.clone().to_string();

        App::new()
            .service(
                web::scope("/pkg")
                    .wrap(middleware::DefaultHeaders::new().add(("Cache-Control", "no-cache")))
                    .service(Files::new("", format!("{site_root}/pkg"))),
            )
            .service(Files::new("/assets", &site_root))
            .service(favicon)
            .route(
                "/{tail:.*}",
                leptos_actix::render_app_to_stream(
                    {
                        let options = options.clone();
                        move || {
                            let options = options.clone();
                            view! {
                                <!DOCTYPE html>
                                <html lang="en" data-motion="plot">
                                    <head>
                                        <meta charset="utf-8"/>
                                        <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                        <script>{r#"(function(){document.documentElement.dataset.js='1';var m=null;try{m=new URLSearchParams(location.search).get('lang')}catch(e){}var s=null;try{s=localStorage.getItem('locale')}catch(e){}var t=(m||s||(navigator.languages&&navigator.languages[0])||navigator.language||'en').toString().toLowerCase();var l=t.indexOf('pt')===0?'pt-BR':'en';document.documentElement.lang=l;document.documentElement.dataset.locale=(l==='pt-BR')?'pt-br':'en'})()"#}</script>
                                        <link rel="preconnect" href="https://fonts.googleapis.com"/>
                                        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>
                                        <link
                                            rel="stylesheet"
                                            href="https://fonts.googleapis.com/css2?family=Archivo+Black&family=Archivo:wght@400;600&family=JetBrains+Mono:wght@400;700&display=swap"
                                        />
                                        <ImportMap/>

                                        <AutoReload options=options.clone() />
                                        <HydrationScripts options=options.clone()/>
                                        <HashedStylesheet id="leptos" options=options.clone()/>
                                        <MetaTags/>
                                    </head>
                                    <body>
                                        <App/>
                                    </body>
                                </html>
                            }
                        }
                    },
                    leptos_router::Method::Get,
                ),
            )
            .app_data(web::Data::new(options))
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    use portfolio::app::App;

    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(App);
}
