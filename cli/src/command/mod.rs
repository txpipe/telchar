mod install;
mod codegen;
mod publish;

pub async fn execute(matches: clap::ArgMatches) {
    match matches.subcommand() {
        Some(("install", sub_matches)) => {
            install::run(sub_matches).await;
        }
        Some(("codegen", sub_matches)) => {
            codegen::run(sub_matches).await;
        }
        Some(("publish", _)) => {
            publish::run().await;
        }
        _ => unreachable!()
    }
}