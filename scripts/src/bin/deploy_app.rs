use std::env;




use boot_core::{instantiate_daemon_env, networks::juno::UNI_5};

use interfaces::template::TemplateApp;
// use template_app::msg::ConfigResponse;

use semver::Version;
use template_app::contract::{APP_NAME, APP_NAMESPACE};

// To deploy the app we need to get the memory and then register it
// We can then deploy a test OS that uses that new app

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn deploy_app() -> anyhow::Result<()> {
    let network = UNI_5;
    let _app_version = Version::parse(APP_VERSION)?;

    // Setup the environment
    let (_, _sender, chain) = instantiate_daemon_env(network)?;

    // Load Abstract Version Control
    let _version_control_address: String =
        env::var("VERSION_CONTROL_ADDRESS").expect("VERSION_CONTROL_ADDRESS must be set");

    // let version_control = VersionControl::load(
    //     &chain,
    //     &Addr::unchecked(version_control_address),
    // );

    // Upload and register your module
    let app_name = format!("{}:{}", APP_NAMESPACE, APP_NAME);
    let _app = TemplateApp::new(&app_name, &chain);
    // version_control.upload_and_register_module(&mut app &app_version)?;

    // Example queries
    // app.query_base(BaseQueryMsg::Admin {})?;

    // let app_config: ConfigResponse = app.query_app(TemplateQueryMsg::Config {})?;

    // TODO: Attach to an OS

    Ok(())
}

fn main() {
    dotenv().ok();
    env_logger::init();

    use dotenv::dotenv;

    if let Err(ref err) = deploy_app() {
        log::error!("{}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| log::error!("because: {}", cause));

        // The backtrace is not always generated. Try to run this example
        // with `$env:RUST_BACKTRACE=1`.
        //    if let Some(backtrace) = e.backtrace() {
        //        log::debug!("backtrace: {:?}", backtrace);
        //    }

        ::std::process::exit(1);
    }
}
