#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;
use dioxus_free_icons::{icons::io_icons::{IoCloudDownload, IoLogoGithub, IoLogoTwitter, IoOpen}, Icon};
use serde::{Deserialize, Serialize};
use qrcode::render::svg;
use qrcode::QrCode;
use base64::{engine::general_purpose, Engine as _};
use anyhow::Result;

mod env;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppState {
    isError: bool,
    dataUrl: Option<String>,
    topNavbarBurgerActive: bool,
    topNavbarBurgerClass: String,
    topNavbarMenuClass: String,
    termsOfUseModalActive: bool,
    termsOfUseModalClass: String,
    privacyPolicyModalActive: bool,
    privacyPolicyModalClass: String,
}

impl AppState {
    fn new() -> Self {
        Self {
            isError: false,
            dataUrl: None,
            topNavbarBurgerActive: false,
            topNavbarBurgerClass: "navbar-burger".to_string(),
            topNavbarMenuClass: "navbar-menu".to_string(),
            termsOfUseModalActive: false,
            termsOfUseModalClass: "modal".to_string(),
            privacyPolicyModalActive: false,
            privacyPolicyModalClass: "modal".to_string(),
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct MakeQrCodeProps {
    text: String,
    width: u32,
    height: u32,
    dark_color: String,
    light_color: String,
}

impl Default for MakeQrCodeProps {
    fn default() -> Self {
        Self {
            text: "".to_string(),
            width: 200,
            height: 200,
            dark_color: "#000000".to_string(),
            light_color: "#ffffff".to_string(),
        }
    }
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(AppState::new()));
    let mut app_state = consume_context::<Signal<AppState>>();

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        GoogleAnalytics {}
        Header {}
        div {
            class: "container px-3 py-3",
            form {
                class: "box",
                onsubmit: move |ev| {
                    let qrCode = makeQrCode(MakeQrCodeProps { text: ev.values().get("text").unwrap().as_value(), ..Default::default() });
                    match qrCode {
                        Ok(data_url) => {
                            app_state.write().isError = false;
                            app_state.write().dataUrl = Some(data_url);
                        }
                        Err(_) => {
                            app_state.write().isError = true;
                            app_state.write().dataUrl = None;
                        }
                    }
                },
                div {
                    class: "field",
                    div {
                        class: "control",
                        input {
                            id: "text",
                            name: "text",
                            class: "input",
                            r#type: "text",
                            required: true,
                            placeholder: "テキストを入力してください",
                        }
                    }
                }
                div {
                    class: "field is-grouped is-justify-content-center",
                    div {
                        class: "control",
                        button {
                            class: "button is-medium is-fullwidth is-link",
                            r#type: "submit",
                            "QR コードを生成"
                        }
                    }
                }
            }
        }
        ErrorMsg {}
        QrCode {}
        Footer{}
        TermsOfUseModal {}
        PrivacyPolicyModal {}
    }
}

#[component]
fn Header() -> Element {
    let mut app_state = consume_context::<Signal<AppState>>();

    rsx! {
        header {
            class: "container px-3 pt-3",
            nav { id: "top-navbar", class: "navbar",
                div { class: "navbar-brand",
                    section {
                        class: "navbar-item hero is-small",
                        div {
                            class: "hero-body",
                            p {
                                class: "title",
                                "ZEQRA",
                            }
                            p {
                                class: "subtitle",
                                "QR code generator",
                                span {
                                    class: "tag is-info is-light ml-3",
                                    "v{env::APP_VERSION}"
                                }
                            }
                        }
                    }
                    a {
                        id: "top-navbar-burger",
                        class: "{app_state.read().topNavbarBurgerClass}",
                        "aria-expanded": "false",
                        "data-target": "top-navbar-menu",
                        onclick: move |_| {
                            if app_state.read().topNavbarBurgerActive {
                                app_state.write().topNavbarBurgerActive = false;
                                app_state.write().topNavbarBurgerClass = "navbar-burger".to_string();
                                app_state.write().topNavbarMenuClass = "navbar-menu".to_string();
                            } else {
                                app_state.write().topNavbarBurgerActive = true;
                                app_state
                                    .write()
                                    .topNavbarBurgerClass = "navbar-burger is-active".to_string();
                                app_state.write().topNavbarMenuClass = "navbar-menu is-active".to_string();
                            }
                        },
                        span { "aria-hidden": "true" }
                        span { "aria-hidden": "true" }
                        span { "aria-hidden": "true" }
                        span { "aria-hidden": "true" }
                    }
                }
                div {
                    id: "top-navbar-menu",
                    class: "{app_state.read().topNavbarMenuClass}",
                    div { class: "navbar-start" }
                    div { class: "navbar-end",
                        div { class: "navbar-item",
                            div { class: "buttons",
                                a {
                                    href: "#",
                                    class: "Header-link",
                                    "data-target": "terms-of-use-modal",
                                    onclick: move |_| {
                                        if app_state.read().termsOfUseModalActive {
                                            app_state.write().termsOfUseModalActive = false;
                                            app_state.write().termsOfUseModalClass = "modal".to_string();
                                        } else {
                                            app_state.write().termsOfUseModalActive = true;
                                            app_state.write().termsOfUseModalClass = "modal is-active".to_string();
                                        }
                                    },
                                    "利用規約"
                                }
                                a {
                                    href: "#",
                                    class: "Header-link",
                                    "data-target": "privacy-policy-modal",
                                    onclick: move |_| {
                                        if app_state.read().privacyPolicyModalActive {
                                            app_state.write().privacyPolicyModalActive = false;
                                            app_state.write().privacyPolicyModalClass = "modal".to_string();
                                        } else {
                                            app_state.write().privacyPolicyModalActive = true;
                                            app_state.write().privacyPolicyModalClass = "modal is-active".to_string();
                                        }
                                    },
                                    "個人情報保護方針"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn GoogleAnalytics() -> Element {
    rsx! {
        script {
            r#async: true,
            src: "https://www.googletagmanager.com/gtag/js?id={env::APP_GA_TRACKING_ID}"
        }
        script {
            r#"
            window.dataLayer = window.dataLayer || [];
            function gtag(){{dataLayer.push(arguments);}}
            gtag('js', new Date());
            gtag('config', '{env::APP_GA_TRACKING_ID}');
            "#
        }
    }
}

#[component]
fn TermsOfUseModal() -> Element {
    let mut app_state = consume_context::<Signal<AppState>>();

    rsx! {
        div {
            id: "terms-of-use-modal",
            class: "{app_state.read().termsOfUseModalClass}",
            div { class: "modal-background" }
            div { class: "modal-card p-4",
                header { class: "modal-card-head",
                    p { class: "modal-card-title", "利用規約" }
                    button {
                        class: "modal-close is-large",
                        "aria-label": "close",
                        onclick: move |_| {
                            app_state.write().termsOfUseModalActive = false;
                            app_state.write().termsOfUseModalClass = "modal".to_string();
                        }
                    }
                }
                section { class: "modal-card-body", TermsOfUseContent {} }
                footer { class: "modal-card-foot",
                    div { class: "buttons",
                        button {
                            class: "button",
                            onclick: move |_| {
                                app_state.write().termsOfUseModalActive = false;
                                app_state.write().termsOfUseModalClass = "modal".to_string();
                            },
                            "閉じる"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TermsOfUseContent() -> Element {
    rsx! {
        div { class: "content",
            h3 { "1. 受諾" }
            p {
                "1.1 このウェブサービス（以下、「本サービス」といいます）を利用する場合、ユーザーは本利用規約に同意したものとみなされます。本サービスの利用は、本規約のすべての条件、および変更に同意することを含みます。"
            }
            h3 { "2. 定義" }
            p {
                "2.1 「本サービス」とは、本規約に基づき提供されるウェブサービスを指します。"
            }
            p {
                "2.2 「ユーザー」とは、本サービスを利用する個人または法人を指します。"
            }
            h3 { "3. サービスの提供" }
            p {
                "3.1 ユーザーは、本サービスの提供にあたり、合理的な努力を行いますが、本サービスの中断、遅延、またはエラーが生じる可能性があることを理解し、同意します。"
            }
            p {
                "3.2 ユーザーは、事前の通知なしに、本サービスの一部または全部を変更、中断、または終了する権利を留保します。"
            }
            h3 { "4. 利用条件" }
            p {
                "4.1 ユーザーは、本サービスを利用する際に、全ての適用される法律および規制を遵守する必要があります。"
            }
            p {
                "4.2 ユーザーは、本サービスを不正に使用し、または他のユーザーの利用を妨害する行為を行ってはなりません。"
            }
            p {
                "4.3 ユーザーは、本サービスを使用する際に、他のユーザーや本サービスの権利を侵害するような情報を提供してはなりません。"
            }
            h3 { "5. 個人情報の取り扱い" }
            p {
                "5.1 個人情報の収集、使用、および開示に関しては、個人情報保護方針が適用されます。"
            }
            h3 { "6. 責任の制限" }
            p {
                "6.1 当サービスの利用に関連して発生したいかなる損害についても、直接的、間接的、偶発的、特別、または重大な損害を含むがこれに限定されない、いかなる損害に対しても一切の責任を負いません。"
            }
            h3 { "7. 準拠法と管轄裁判所" }
            p { "7.1 本規約の解釈および適用は、日本法に従います。" }
            p {
                "7.2 本規約に関連するいかなる紛争も、東京地方裁判所を第一審の専属的な管轄裁判所とします。"
            }
        }
    }
}

#[component]
fn PrivacyPolicyModal() -> Element {
    let mut app_state = consume_context::<Signal<AppState>>();

    rsx! {
        div {
            id: "privacy-policy-modal",
            class: "{app_state.read().privacyPolicyModalClass}",
            div { class: "modal-background" }
            div { class: "modal-card p-4",
                header { class: "modal-card-head",
                    p { class: "modal-card-title", "個人情報保護方針" }
                    button {
                        class: "modal-close is-large",
                        "aria-label": "close",
                        onclick: move |_| {
                            app_state.write().privacyPolicyModalActive = false;
                            app_state.write().privacyPolicyModalClass = "modal".to_string();
                        }
                    }
                }
                section { class: "modal-card-body", PrivacyPolicyContent {} }
                footer { class: "modal-card-foot",
                    div { class: "buttons",
                        button {
                            class: "button",
                            onclick: move |_| {
                                app_state.write().privacyPolicyModalActive = false;
                                app_state.write().privacyPolicyModalClass = "modal".to_string();
                            },
                            "閉じる"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PrivacyPolicyContent() -> Element {
    rsx! {
        div { class: "content",
            p {
                "このウェブサイトは、Google Analytics を使用して、ウェブサイトのトラフィックとユーザーの行動に関する情報を収集しています。Google Analytics は Cookie を使用して、匿名の形式で情報を収集します。収集される情報には、ウェブサイトの利用者の IP アドレス、地理的位置、閲覧されたページ、利用されたブラウザやデバイスの種類などが含まれます。これらの情報は、個々のユーザーを特定するために使用されることはありません。"
            }
            p {
                "このウェブサイトは、Google Analytics の機能によって提供されるデータを収集、解析、報告するためにこれらの情報を使用します。これには、ウェブサイトの改善や、ユーザーのニーズに合わせたコンテンツの提供などが含まれます。"
            }
            p {
                "このウェブサイトを利用することにより、Google が収集したデータの処理に関して、Google の個人情報保護方針に同意したものとみなされます。Google の個人情報保護方針については、", a {
                    href: "https://policies.google.com/privacy",
                    target: "_blank",
                    "こちら",
                    span { class: "icon is-small mr-2", Icon { width: 16, height: 16, icon: IoOpen } }
                }
                "をご参照ください。"
            }
            p {
                "Cookie の使用に関する設定を変更したい場合は、ウェブブラウザの設定を変更して、Cookie の使用を管理することができます。ただし、Cookie の無効化または削除は、ウェブサイトの機能やサービスの一部を制限する可能性があります。"
            }
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        footer { class: "footer has-text-centered",
            div { class: "container",
                div { class: "columns",
                    div { class: "column",
                        p {
                            class: "buttons is-justify-content-center",
                            a {
                                class: "button",
                                href: "https://twitter.com/stlittle8",
                                target: "_blank",
                                span { class: "icon is-small", Icon { width: 16, height: 16, icon: IoLogoTwitter } }
                                span { "Twitter" }
                            }
                            a {
                                class: "button",
                                href: "https://github.com/st-little/zeqra",
                                target: "_blank",
                                span { class: "icon is-small", Icon { width: 16, height: 16, icon: IoLogoGithub } }
                                span { "GitHub" }
                            }
                        }
                    }
                }
                div { class: "has-text-centered", "© 2020 st-little" }
            }
        }
    }
}

#[component]
fn QrCode() -> Element {
    let app_state = consume_context::<Signal<AppState>>();
    let data_url = app_state.read().dataUrl.clone();

    match data_url {
        Some(data_url) => {
            rsx! {
                div {
                    class: "container pt-5 has-text-centered",
                    div {
                        img { src: "{data_url}", width: "150", height: "150" }
                    }
                    div {
                        class: "pt-3",
                        a {
                            class: "button is-medium is-success",
                            href: "{data_url}",
                            download: "QR-code.svg",
                            span {
                                class: "icon is-small mr-2",
                                Icon { width: 24, height: 24, icon: IoCloudDownload }
                            },
                            "Download"
                        }
                    }
                }
            }
        }
        None => None
    }
}

#[component]
fn ErrorMsg() -> Element {
    let app_state = consume_context::<Signal<AppState>>();
    let isError = app_state.read().isError;

    match isError {
        true => {
            rsx! {
                div {
                    class: "container px-3 py-3",
                    article {
                        class: "message is-danger",
                        div {
                            class: "message-body",
                            "QR コードの生成に失敗しました。"
                        }
                    }    
                }
            }
        }
        false => None
    }
}

fn makeQrCode(props: MakeQrCodeProps) -> Result<String> {
    let code: QrCode = QrCode::new(props.text.as_bytes())?;
    let image = code.render()
        .min_dimensions(props.width, props.height)
        .dark_color(svg::Color(&props.dark_color))
        .light_color(svg::Color(&props.light_color))
        .build();
    let data_url = format!("data:image/svg+xml;base64,{}", general_purpose::STANDARD.encode(&image));

    Ok(data_url)
}