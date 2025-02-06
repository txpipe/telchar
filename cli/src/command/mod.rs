mod codegen;
mod publish;

pub async fn execute(matches: clap::ArgMatches) {
    match matches.subcommand() {
        Some(("codegen", sub_matches)) => {
            codegen::run(sub_matches).await;
        }
        Some(("publish", _)) => {
            publish::run().await;
        }
        _ => unreachable!()
    }
}