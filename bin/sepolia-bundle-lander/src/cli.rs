use url::Url;
#[derive(Debug, Clone, Default, clap::Args)]
pub struct BundleLander {
    #[clap(short)]
    node_endpoint: Url
}
