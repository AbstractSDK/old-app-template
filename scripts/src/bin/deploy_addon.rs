use std::env;
use abstract_os::add_on::BaseQueryMsg;
use abstract_os::{middleware, VERSION_CONTROL};
use boot_abstract::memory::Memory;
use boot_abstract::version_control::VersionControl;
use boot_core::{
    instantiate_daemon_env,
    networks::juno::{JUNO_DAEMON, UNI_5},
};
use cosmwasm_std::{Addr, Coin, Empty};
use interfaces::{template, template::{{addon_contract}}};
// use template_addon::msg::ConfigResponse;
use template_addon::msg::{{{addon_execute_msg}}, {{addon_instantiate_msg}}, {{addon_migrate_msg}}, {{addon_query_msg}}};

use log::info;
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
    let version_control_address: String = env::var("VERSION_CONTROL_ADDRESS").expect("VERSION_CONTROL_ADDRESS must be set");

    let version_control = VersionControl::new(VERSION_CONTROL, &chain, Some(&Addr::unchecked(version_control_address)));

    // Upload and register your module
    let addon_name = format!("{}:{}", ADDON_NAMESPACE, ADDON_NAME);
    let mut add_on = {{addon_contract}}::new(&addon_name, &chain);
    version_control.upload_and_register_module(&mut add_on, &addon_version)?;

    // Example queries
    // add_on.query_base(BaseQueryMsg::Admin {})?;

    // let add_on_config: ConfigResponse = add_on.query_addon({{addon_query_msg}}::Config {})?;

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
