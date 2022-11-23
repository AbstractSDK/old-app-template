use std::env;

use abstract_boot::version_control::VersionControl;
use abstract_os::app;
use abstract_os::app::BaseQueryMsg;
use boot_core::networks;
use boot_core::prelude::{instantiate_daemon_env, ContractInstance};
use cosmwasm_std::Addr;

// use template_app::msg::ConfigResponse;

use semver::Version;
use interfaces::template::TemplateApp;
use template_app::contract::{MODULE_NAME, MODULE_NAMESPACE};

// use template_app::msg::ConfigResponse;

// To deploy the app we need to get the memory and then register it
// We can then deploy a test OS that uses that new app

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn deploy_app() -> anyhow::Result<()> {
    let network = networks::UNI_5;

    // Setup the environment
    let (_, _sender, chain) = instantiate_daemon_env(network)?;

    // Load Abstract Version Control
    let version_control_address: String =
        env::var("VERSION_CONTROL_ADDRESS").expect("VERSION_CONTROL_ADDRESS must be set");

    let version_control = VersionControl::load(
        &chain,
        &Addr::unchecked(version_control_address),
    );

    // Upload and register your module
    let app_name = format!("{}:{}", MODULE_NAMESPACE, MODULE_NAME);
    let mut app =  TemplateApp::new(&app_name, &chain);
    let app_version = Version::parse(APP_VERSION)?;
    version_control.upload_and_register_module(&mut app.as_instance_mut(), &app_version)?;

    // Example queries
    // app.query_base(BaseQueryMsg::Admin {})?;

    // let app_config: ConfigResponse = app.query_app( {{app_query_msg}}::Config {})?;

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
