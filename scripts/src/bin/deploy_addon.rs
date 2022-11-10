use abstract_os::add_on::BaseQueryMsg;
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

pub fn script() -> anyhow::Result<()> {
    let network = JUNO_DAEMON;
    let (_, sender, chain) = instantiate_daemon_env(network)?;

    let mut add_on = TemplateAddOn::new("{{ addon_name }}", &chain);

    add_on.upload()?;

    add_on.instantiate(
        &TemplateInstantiateMsg {
            initial_counts: None,
            max_count: 1000u128.into(),
        },
        Some(&sender),
        None,
    )?;

    // query the admin
    add_on.query(&abstract_os::add_on::QueryMsg::Base(BaseQueryMsg::Admin {}))?;

    Ok(())
}

fn main() {
    dotenv().ok();
    env_logger::init();

    use dotenv::dotenv;

    if let Err(ref err) = script() {
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
