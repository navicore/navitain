//! Generated types support documentation
#![deny(missing_docs)]
use anyhow::Ok;
use futures::{pin_mut, TryStreamExt};
use schemars::JsonSchema;

use apiexts::CustomResourceDefinition;
use k8s_openapi::{
    api::core::v1::Node, apiextensions_apiserver::pkg::apis::apiextensions::v1 as apiexts, serde,
};
use serde::{Deserialize, Serialize};
use tracing::*;

use kube::{
    api::{Api, ListParams, Patch, PatchParams, ResourceExt},
    runtime::wait::{await_condition, conditions},
    runtime::{watcher, WatchStreamExt},
    Client, CustomResource, CustomResourceExt,
};

// TopologyCRD
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(group = "navicore.tech", version = "v1", kind = "Topology", namespaced)]
#[kube(status = "TopologyStatus")]
#[kube(scale = r#"{"specReplicasPath":".spec.replicas", "statusReplicasPath":".status.replicas"}"#)]
struct TopologySpec {
    pub name: String,
    pub nodes: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
struct TopologyStatus {
    pub is_bad: bool,
}

const CRD_NAME: &str = "topologies.navicore.tech";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    // looks for a local kubeconfig first then tries ENV VARS
    let client = Client::try_default().await?;

    // Get the nodes current topology

    // Watch the topology resource

    //
    let ssapply = PatchParams::apply("topology_apply").force();

    // 0. Ensure the CRD is installed, could do this once
    let crd_client: Api<CustomResourceDefinition> = Api::all(client.clone());

    info!("Creating crd: {}", serde_yaml::to_string(&Topology::crd())?);
    crd_client
        .patch(CRD_NAME, &ssapply, &Patch::Apply(Topology::crd()))
        .await?;
    //
    info!("Waiting for the api-server to accept the CRD");

    let establish = await_condition(crd_client, CRD_NAME, conditions::is_crd_established());
    let _ = tokio::time::timeout(std::time::Duration::from_secs(10), establish).await?;

    // Let's get the current node topology
    let nodes: Api<Node> = Api::all(client.clone());

    // New client copy to inject our resource
    let topologys: Api<Topology> = Api::default_namespaced(client.clone());

    let spec = create_spec(nodes.clone()).await;

    let tt = topologys
        .patch(
            "default",
            &ssapply,
            &Patch::Apply(&Topology::new("default", spec)),
        )
        .await?;

    info!("Applied 1 {}: {:?}", tt.name_any(), tt.spec);

    let obs = watcher(nodes, watcher::Config::default()).applied_objects();
    pin_mut!(obs);
    while let Some(o) = obs.try_next().await? {
        info!("watch saw {:?}", o);
        match o {
            Node => {
                info!("watch matched Node");
                let nodes: Api<Node> = Api::all(client.clone());
                let spec = create_spec(nodes.clone()).await;
                let topologys: Api<Topology> = Api::default_namespaced(client.clone());

                let tt = topologys
                    .patch(
                        "default",
                        &ssapply,
                        &Patch::Apply(&Topology::new("default", spec)),
                    )
                    .await?;
            }
        }
    }

    Ok(())
}
async fn create_spec(nodes: Api<Node>) -> TopologySpec {
    let node_list = nodes.list(&ListParams::default()).await.unwrap();
    let mut node_names = Vec::new();
    for node in node_list {
        node_names.push(node.metadata.name.unwrap());
    }
    return TopologySpec {
        name: "default".to_string(),
        nodes: node_names,
    };
}
