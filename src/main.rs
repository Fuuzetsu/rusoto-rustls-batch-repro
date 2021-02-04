use rusoto_batch::{
    Batch, BatchClient, ContainerProperties, DeregisterJobDefinitionRequest,
    RegisterJobDefinitionRequest, ResourceRequirement,
};
use rusoto_core::{HttpClient, Region};
use rusoto_credential::DefaultCredentialsProvider;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    // AWS region to use for Batch. STS uses default region.
    #[structopt(long)]
    region: Option<Region>,
    #[structopt(long)]
    role: Option<String>,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let opts = Opts::from_args();
    let region = opts.region.unwrap_or_default();

    // Establish provider for all the clients that we need.
    let batch_client = match opts.role {
        Some(role_arn) => {
            let sts = {
                // STS client uses its own HTTP client. We specify one explicitly
                // because we want to specify region.
                let http_client = HttpClient::new().unwrap();
                // Instead of aws_options.aws_region we use the default region. For
                // example if we run on EC2 instance in the US, it makes little
                // sense to run STS against the API endpoint in Japan. We do still
                // want to use the user-provider region for other requests as the
                // region may actually matter there, such as S3 bucket locations or
                // any other region-scoped resources.
                rusoto_sts::StsClient::new_with(
                    http_client,
                    DefaultCredentialsProvider::new().unwrap(),
                    Region::default(),
                )
            };
            // If we asked to use a specific role, we're going to do it
            // here. The role expires after a while.
            let provider = rusoto_credential::AutoRefreshingProvider::new(
                rusoto_sts::StsAssumeRoleSessionCredentialsProvider::new(
                    sts,
                    role_arn,
                    "batch_repro".to_owned(),
                    None,
                    None,
                    None,
                    None,
                ),
            )
            .unwrap();

            rusoto_batch::BatchClient::new_with(HttpClient::new().unwrap(), provider, region)
        }
        None => BatchClient::new(region),
    };

    let register_request = RegisterJobDefinitionRequest {
        job_definition_name: "rustls-failure-repro".into(),
        type_: "container".into(),
        container_properties: Some(ContainerProperties {
            command: Some(vec!["echo".into(), "test".into()]),
            image: Some("ubuntu:latest".into()),
            resource_requirements: Some(vec![
                ResourceRequirement {
                    type_: "MEMORY".into(),
                    value: "1024".into(),
                },
                ResourceRequirement {
                    type_: "VCPU".into(),
                    value: "1".into(),
                },
            ]),
            ..Default::default()
        }),
        ..Default::default()
    };

    let registered_job_definition = batch_client
        .register_job_definition(register_request)
        .await
        .unwrap();

    // We expected to have failed by now, but if not, delete the resource.
    let deregister_request = DeregisterJobDefinitionRequest {
        job_definition: format!(
            "{}:{}",
            registered_job_definition.job_definition_name, registered_job_definition.revision
        ),
    };

    batch_client
        .deregister_job_definition(deregister_request)
        .await
        .unwrap();
}
