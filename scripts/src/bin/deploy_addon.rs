use std::env;

use abstract_os::VERSION_CONTROL;

use boot_abstract::version_control::VersionControl;
use boot_core::{instantiate_daemon_env, networks::juno::JUNO_DAEMON};
use cosmwasm_std::Addr;
use interfaces::template::TemplateApp;
// use template_addon::msg::ConfigResponse;

use semver::Version;
use template_addon::contract::{ADDON_NAME, ADDON_NAMESPACE};

// To deploy the addon we need to get the memory and then register it
// We can then deploy a test OS that uses that new addon

const ADDON_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn deploy_addon() -> anyhow::Result<()> {
    let network = JUNO_DAEMON;
    let addon_version = Version::parse(ADDON_VERSION)?;

    // Setup the environment
    let (_, _sender, chain) = instantiate_daemon_env(network)?;

    // Load Abstract Version Control
    let version_control_address: String =
        env::var("VERSION_CONTROL_ADDRESS").expect("VERSION_CONTROL_ADDRESS must be set");

    // let version_control = VersionControl::load(
    //     &chain,
    //     &Addr::unchecked(version_control_address),
    // );

    // Upload and register your module
    let addon_name = format!("{}:{}", ADDON_NAMESPACE, ADDON_NAME);
    let mut app = TemplateApp::new(&addon_name, &chain);
    // version_control.upload_and_register_module(&mut app &addon_version)?;

    // Example queries
    // app.query_base(BaseQueryMsg::Admin {})?;

    // let app_config: ConfigResponse = app.query_addon(TemplateQueryMsg::Config {})?;

    // TODO: Attach to an OS

    Ok(())
}

fn main() {
    dotenv().ok();
    env_logger::init();

    use dotenv::dotenv;

    if let Err(ref err) = deploy_addon() {
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
