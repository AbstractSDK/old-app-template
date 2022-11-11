use abstract_os::add_on::BaseQueryMsg;
use abstract_os::VERSION_CONTROL;
use boot_abstract::memory::Memory;
use boot_abstract::version_control::VersionControl;
use boot_core::{
    instantiate_daemon_env,
    networks::juno::{JUNO_DAEMON, UNI_5},
};
use cosmwasm_std::Coin;
use interfaces::{template, template::TemplateAddOn};
use template_addon::msg::{
    TemplateExecuteMsg, TemplateInstantiateMsg, TemplateMigrateMsg, TemplateQueryMsg,
};

use log::info;
use semver::Version;
use template_addon::contract::ADDON_NAME;

// To deploy the addon we need to get the memory and then register it
// We can then deploy a test OS that uses that new addon

const ADDON_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn deploy_addon() -> anyhow::Result<()> {
    let network = JUNO_DAEMON;
    let addon_version = Version::parse(ADDON_VERSION)?;

    // Setup the environment
    let (_, sender, chain) = instantiate_daemon_env(network)?;

    // Load Abstract Version Control
    // TODO: the address needs to be loaded from the json state
    let mut version_control = VersionControl::new(VERSION_CONTROL, &chain);

    // Upload and register your module
    let mut add_on = TemplateAddOn::new(&ADDON_NAME, &chain);
    version_control.upload_and_register_module(&mut add_on, &addon_version)?;

    // add_on.upload()?;
    //
    // add_on.instantiate(
    //     &TemplateInstantiateMsg {
    //         initial_counts: None,
    //         max_count: 1000u128.into(),
    //     },
    //     Some(&sender),
    //     None,
    // )?;
    //
    // // query the admin
    // add_on.query(&abstract_os::add_on::QueryMsg::Base(BaseQueryMsg::Admin {}))?;

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
