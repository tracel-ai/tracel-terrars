use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataContainerClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataContainerCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataContainerClusterData>,
}
#[derive(Clone)]
pub struct DataContainerCluster(Rc<DataContainerCluster_>);
impl DataContainerCluster {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe location (region or zone) in which the cluster master will be created, as well as the default node location. If you specify a zone (such as us-central1-a), the cluster will be a zonal cluster with a single cluster master. If you specify a region (such as us-west1), the cluster will be a regional cluster with multiple masters spread across zones in the region, and with default node locations in those zones as well."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `addons_config` after provisioning.\nThe configuration for addons supported by GKE."]
    pub fn addons_config(&self) -> ListRef<DataContainerClusterAddonsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.addons_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allow_net_admin` after provisioning.\nEnable NET_ADMIN for this cluster."]
    pub fn allow_net_admin(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_net_admin", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `anonymous_authentication_config` after provisioning.\nAnonymousAuthenticationConfig allows users to restrict or enable anonymous access to the cluster."]
    pub fn anonymous_authentication_config(
        &self,
    ) -> ListRef<DataContainerClusterAnonymousAuthenticationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.anonymous_authentication_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authenticator_groups_config` after provisioning.\nConfiguration for the Google Groups for GKE feature."]
    pub fn authenticator_groups_config(
        &self,
    ) -> ListRef<DataContainerClusterAuthenticatorGroupsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authenticator_groups_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `autopilot_cluster_policy_config` after provisioning.\nConfiguration for the cluster policy."]
    pub fn autopilot_cluster_policy_config(
        &self,
    ) -> ListRef<DataContainerClusterAutopilotClusterPolicyConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autopilot_cluster_policy_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `autopilot_privileged_admission` after provisioning.\nThe customer allowlist Cloud Storage paths for the cluster. These paths are used with the `--autopilot-privileged-admission` flag to authorize privileged workloads in Autopilot clusters. To allow default partner allowlists, set to []. To allow no allowlists, set to [\"\"]."]
    pub fn autopilot_privileged_admission(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autopilot_privileged_admission", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\nConfiguration options for the Binary Authorization feature."]
    pub fn binary_authorization(&self) -> ListRef<DataContainerClusterBinaryAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.binary_authorization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_autoscaling` after provisioning.\nPer-cluster configuration of Node Auto-Provisioning with Cluster Autoscaler to automatically adjust the size of the cluster and create/delete node pools based on the current needs of the cluster's workload. See the guide to using Node Auto-Provisioning for more details."]
    pub fn cluster_autoscaling(&self) -> ListRef<DataContainerClusterClusterAutoscalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_autoscaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_ipv4_cidr` after provisioning.\nThe IP address range of the Kubernetes pods in this cluster in CIDR notation (e.g. 10.96.0.0/14). Leave blank to have one automatically chosen or specify a /14 block in 10.0.0.0/8. This field will only work for routes-based clusters, where ip_allocation_policy is not defined."]
    pub fn cluster_ipv4_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_ipv4_cidr", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `confidential_nodes` after provisioning.\nConfiguration for the confidential nodes feature, which makes nodes run on confidential VMs. Warning: This configuration can't be changed (or added/removed) after cluster creation without deleting and recreating the entire cluster."]
    pub fn confidential_nodes(&self) -> ListRef<DataContainerClusterConfidentialNodesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.confidential_nodes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane_endpoints_config` after provisioning.\nConfiguration for all of the cluster's control plane endpoints. Currently supports only DNS endpoint configuration and disable IP endpoint. Other IP endpoint configurations are available in private_cluster_config."]
    pub fn control_plane_endpoints_config(
        &self,
    ) -> ListRef<DataContainerClusterControlPlaneEndpointsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane_endpoints_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cost_management_config` after provisioning.\nCost management configuration for the cluster."]
    pub fn cost_management_config(&self) -> ListRef<DataContainerClusterCostManagementConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cost_management_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_encryption` after provisioning.\nApplication-layer Secrets Encryption settings. The object format is {state = string, key_name = string}. Valid values of state are: \"ENCRYPTED\"; \"DECRYPTED\". key_name is the name of a CloudKMS key."]
    pub fn database_encryption(&self) -> ListRef<DataContainerClusterDatabaseEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.database_encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `datapath_provider` after provisioning.\nThe desired datapath provider for this cluster. By default, uses the IPTables-based kube-proxy implementation."]
    pub fn datapath_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.datapath_provider", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_max_pods_per_node` after provisioning.\nThe default maximum number of pods per node in this cluster. This doesn't work on \"routes-based\" clusters, clusters that don't have IP Aliasing enabled."]
    pub fn default_max_pods_per_node(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_max_pods_per_node", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_snat_status` after provisioning.\nWhether the cluster disables default in-node sNAT rules. In-node sNAT rules will be disabled when defaultSnatStatus is disabled."]
    pub fn default_snat_status(&self) -> ListRef<DataContainerClusterDefaultSnatStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_snat_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhen the field is set to true or unset in Terraform state, a terraform apply or terraform destroy that would delete the cluster will fail. When the field is set to false, deleting the cluster is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n Description of the cluster."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disable_l4_lb_firewall_reconciliation` after provisioning.\nDisable L4 load balancer VPC firewalls to enable firewall policies."]
    pub fn disable_l4_lb_firewall_reconciliation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.disable_l4_lb_firewall_reconciliation",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `dns_config` after provisioning.\nConfiguration for Cloud DNS for Kubernetes Engine."]
    pub fn dns_config(&self) -> ListRef<DataContainerClusterDnsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dns_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_autopilot` after provisioning.\nEnable Autopilot for this cluster."]
    pub fn enable_autopilot(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_autopilot", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_cilium_clusterwide_network_policy` after provisioning.\nWhether Cilium cluster-wide network policy is enabled on this cluster."]
    pub fn enable_cilium_clusterwide_network_policy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.enable_cilium_clusterwide_network_policy",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `enable_fqdn_network_policy` after provisioning.\nWhether FQDN Network Policy is enabled on this cluster."]
    pub fn enable_fqdn_network_policy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_fqdn_network_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_intranode_visibility` after provisioning.\nWhether Intra-node visibility is enabled for this cluster. This makes same node pod to pod traffic visible for VPC network."]
    pub fn enable_intranode_visibility(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_intranode_visibility", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_k8s_beta_apis` after provisioning.\nConfiguration for Kubernetes Beta APIs."]
    pub fn enable_k8s_beta_apis(&self) -> ListRef<DataContainerClusterEnableK8sBetaApisElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enable_k8s_beta_apis", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_kubernetes_alpha` after provisioning.\nWhether to enable Kubernetes Alpha features for this cluster. Note that when this option is enabled, the cluster cannot be upgraded and will be automatically deleted after 30 days."]
    pub fn enable_kubernetes_alpha(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_kubernetes_alpha", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_l4_ilb_subsetting` after provisioning.\nWhether L4ILB Subsetting is enabled for this cluster."]
    pub fn enable_l4_ilb_subsetting(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_l4_ilb_subsetting", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_legacy_abac` after provisioning.\nWhether the ABAC authorizer is enabled for this cluster. When enabled, identities in the system, including service accounts, nodes, and controllers, will have statically granted permissions beyond those provided by the RBAC configuration or IAM. Defaults to false."]
    pub fn enable_legacy_abac(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_legacy_abac", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_multi_networking` after provisioning.\nWhether multi-networking is enabled for this cluster."]
    pub fn enable_multi_networking(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_multi_networking", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_shielded_nodes` after provisioning.\nEnable Shielded Nodes features on all nodes in this cluster. Defaults to true."]
    pub fn enable_shielded_nodes(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_shielded_nodes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_tpu` after provisioning.\nWhether to enable Cloud TPU resources in this cluster."]
    pub fn enable_tpu(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_tpu", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nThe IP address of this cluster's Kubernetes master."]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enterprise_config` after provisioning.\nDefines the config needed to enable/disable GKE Enterprise"]
    pub fn enterprise_config(&self) -> ListRef<DataContainerClusterEnterpriseConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enterprise_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet` after provisioning.\nFleet configuration of the cluster."]
    pub fn fleet(&self) -> ListRef<DataContainerClusterFleetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gateway_api_config` after provisioning.\nConfiguration for GKE Gateway API controller."]
    pub fn gateway_api_config(&self) -> ListRef<DataContainerClusterGatewayApiConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gateway_api_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gke_auto_upgrade_config` after provisioning.\nConfiguration options for the auto-upgrade patch type feature, which provide more control over the speed of automatic upgrades of your GKE clusters."]
    pub fn gke_auto_upgrade_config(
        &self,
    ) -> ListRef<DataContainerClusterGkeAutoUpgradeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gke_auto_upgrade_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `identity_service_config` after provisioning.\nConfiguration for Identity Service which allows customers to use external identity providers with the K8S API."]
    pub fn identity_service_config(
        &self,
    ) -> ListRef<DataContainerClusterIdentityServiceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.identity_service_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `in_transit_encryption_config` after provisioning.\nDefines the config of in-transit encryption"]
    pub fn in_transit_encryption_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.in_transit_encryption_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `initial_node_count` after provisioning.\nThe number of nodes to create in this cluster's default node pool. In regional or multi-zonal clusters, this is the number of nodes per zone. Must be set if node_pool is not set. If you're using google_container_node_pool objects with no default node pool, you'll need to set this to a value of at least 1, alongside setting remove_default_node_pool to true."]
    pub fn initial_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_node_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_allocation_policy` after provisioning.\nConfiguration of cluster IP allocation for VPC-native clusters. Adding this block enables IP aliasing, making the cluster VPC-native instead of routes-based."]
    pub fn ip_allocation_policy(&self) -> ListRef<DataContainerClusterIpAllocationPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ip_allocation_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint of the set of labels for this cluster."]
    pub fn label_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location (region or zone) in which the cluster master will be created, as well as the default node location. If you specify a zone (such as us-central1-a), the cluster will be a zonal cluster with a single cluster master. If you specify a region (such as us-west1), the cluster will be a regional cluster with multiple masters spread across zones in the region, and with default node locations in those zones as well."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\nLogging configuration for the cluster."]
    pub fn logging_config(&self) -> ListRef<DataContainerClusterLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_service` after provisioning.\nThe logging service that the cluster should write logs to. Available options include logging.googleapis.com(Legacy Stackdriver), logging.googleapis.com/kubernetes(Stackdriver Kubernetes Engine Logging), and none. Defaults to logging.googleapis.com/kubernetes."]
    pub fn logging_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logging_service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\nThe maintenance policy to use for the cluster."]
    pub fn maintenance_policy(&self) -> ListRef<DataContainerClusterMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `master_auth` after provisioning.\nThe authentication information for accessing the Kubernetes master. Some values in this block are only returned by the API if your service account has permission to get credentials for your GKE cluster. If you see an unexpected diff unsetting your client cert, ensure you have the container.clusters.getCredentials permission."]
    pub fn master_auth(&self) -> ListRef<DataContainerClusterMasterAuthElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.master_auth", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `master_authorized_networks_config` after provisioning.\nThe desired configuration options for master authorized networks. Omit the nested cidr_blocks attribute to disallow external access (except the cluster node IPs, which GKE automatically whitelists)."]
    pub fn master_authorized_networks_config(
        &self,
    ) -> ListRef<DataContainerClusterMasterAuthorizedNetworksConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.master_authorized_networks_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `master_version` after provisioning.\nThe current version of the master in the cluster. This may be different than the min_master_version set in the config if the master has been updated by GKE."]
    pub fn master_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mesh_certificates` after provisioning.\nIf set, and enable_certificates=true, the GKE Workload Identity Certificates controller and node agent will be deployed in the cluster."]
    pub fn mesh_certificates(&self) -> ListRef<DataContainerClusterMeshCertificatesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mesh_certificates", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `min_master_version` after provisioning.\nThe minimum version of the master. GKE will auto-update the master to new versions, so this does not guarantee the current master version--use the read-only master_version field to obtain that. If unset, the cluster's version will be set by GKE to the version of the most recent official release (which is not necessarily the latest version)."]
    pub fn min_master_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_master_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monitoring_config` after provisioning.\nMonitoring configuration for the cluster."]
    pub fn monitoring_config(&self) -> ListRef<DataContainerClusterMonitoringConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.monitoring_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monitoring_service` after provisioning.\nThe monitoring service that the cluster should write metrics to. Automatically send metrics from pods in the cluster to the Google Cloud Monitoring API. VM metrics will be collected by Google Compute Engine regardless of this setting Available options include monitoring.googleapis.com(Legacy Stackdriver), monitoring.googleapis.com/kubernetes(Stackdriver Kubernetes Engine Monitoring), and none. Defaults to monitoring.googleapis.com/kubernetes."]
    pub fn monitoring_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monitoring_service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the cluster, unique within the project and location."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe name or self_link of the Google Compute Engine network to which the cluster is connected. For Shared VPC, set this to the self link of the shared network."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_performance_config` after provisioning.\nNetwork bandwidth tier configuration."]
    pub fn network_performance_config(
        &self,
    ) -> ListRef<DataContainerClusterNetworkPerformanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_performance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_policy` after provisioning.\nConfiguration options for the NetworkPolicy feature."]
    pub fn network_policy(&self) -> ListRef<DataContainerClusterNetworkPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `networking_mode` after provisioning.\nDetermines whether alias IPs or routes will be used for pod IPs in the cluster. Defaults to VPC_NATIVE for new clusters."]
    pub fn networking_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.networking_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_config` after provisioning.\nThe configuration of the nodepool"]
    pub fn node_config(&self) -> ListRef<DataContainerClusterNodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_locations` after provisioning.\nThe list of zones in which the cluster's nodes are located. Nodes must be in the region of their regional cluster or in the same region as their cluster's zone for zonal clusters. If this is specified for a zonal cluster, omit the cluster's zone."]
    pub fn node_locations(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.node_locations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_pool` after provisioning.\nList of node pools associated with this cluster. See google_container_node_pool for schema. Warning: node pools defined inside a cluster can't be changed (or added/removed) after cluster creation without deleting and recreating the entire cluster. Unless you absolutely need the ability to say \"these are the only node pools associated with this cluster\", use the google_container_node_pool resource instead of this property."]
    pub fn node_pool(&self) -> ListRef<DataContainerClusterNodePoolElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_pool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_pool_auto_config` after provisioning.\nNode pool configs that apply to all auto-provisioned node pools in autopilot clusters and node auto-provisioning enabled clusters."]
    pub fn node_pool_auto_config(&self) -> ListRef<DataContainerClusterNodePoolAutoConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_pool_auto_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_pool_defaults` after provisioning.\nThe default nodel pool settings for the entire cluster."]
    pub fn node_pool_defaults(&self) -> ListRef<DataContainerClusterNodePoolDefaultsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_pool_defaults", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_version` after provisioning.\nThe Kubernetes version on the nodes. Must either be unset or set to the same value as min_master_version on create. Defaults to the default version set by GKE which is not necessarily the latest version. This only affects nodes in the default node pool. While a fuzzy version can be specified, it's recommended that you specify explicit versions as Terraform will see spurious diffs when fuzzy versions are used. See the google_container_engine_versions data source's version_prefix field to approximate fuzzy versions in a Terraform-compatible way. To update nodes in other node pools, use the version attribute on the node pool."]
    pub fn node_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\nThe notification config for sending cluster upgrade notifications"]
    pub fn notification_config(&self) -> ListRef<DataContainerClusterNotificationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pod_autoscaling` after provisioning.\nPodAutoscaling is used for configuration of parameters for workload autoscaling"]
    pub fn pod_autoscaling(&self) -> ListRef<DataContainerClusterPodAutoscalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pod_autoscaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_cluster_config` after provisioning.\nConfiguration for private clusters, clusters with private nodes."]
    pub fn private_cluster_config(&self) -> ListRef<DataContainerClusterPrivateClusterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_cluster_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_ipv6_google_access` after provisioning.\nThe desired state of IPv6 connectivity to Google Services. By default, no private IPv6 access to or from Google Services (all access will be via IPv4)."]
    pub fn private_ipv6_google_access(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_ipv6_google_access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rbac_binding_config` after provisioning.\nRBACBindingConfig allows user to restrict ClusterRoleBindings an RoleBindings that can be created."]
    pub fn rbac_binding_config(&self) -> ListRef<DataContainerClusterRbacBindingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rbac_binding_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `release_channel` after provisioning.\nConfiguration options for the Release channel feature, which provide more control over automatic upgrades of your GKE clusters. Note that removing this field from your config will not unenroll it. Instead, use the \"UNSPECIFIED\" channel."]
    pub fn release_channel(&self) -> ListRef<DataContainerClusterReleaseChannelElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.release_channel", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `remove_default_node_pool` after provisioning.\nIf true, deletes the default node pool upon cluster creation. If you're using google_container_node_pool resources with no default node pool, this should be set to true, alongside setting initial_node_count to at least 1."]
    pub fn remove_default_node_pool(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.remove_default_node_pool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_labels` after provisioning.\nThe GCE resource labels (a map of key/value pairs) to be applied to the cluster.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn resource_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_usage_export_config` after provisioning.\nConfiguration for the ResourceUsageExportConfig feature."]
    pub fn resource_usage_export_config(
        &self,
    ) -> ListRef<DataContainerClusterResourceUsageExportConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_usage_export_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secret_manager_config` after provisioning.\nConfiguration for the Secret Manager feature."]
    pub fn secret_manager_config(&self) -> ListRef<DataContainerClusterSecretManagerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secret_manager_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secret_sync_config` after provisioning.\nConfiguration for the Sync as k8s secrets feature."]
    pub fn secret_sync_config(&self) -> ListRef<DataContainerClusterSecretSyncConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secret_sync_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_posture_config` after provisioning.\nDefines the config needed to enable/disable features for the Security Posture API"]
    pub fn security_posture_config(
        &self,
    ) -> ListRef<DataContainerClusterSecurityPostureConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_posture_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL for the resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_external_ips_config` after provisioning.\nIf set, and enabled=true, services with external ips field will not be blocked"]
    pub fn service_external_ips_config(
        &self,
    ) -> ListRef<DataContainerClusterServiceExternalIpsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_external_ips_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `services_ipv4_cidr` after provisioning.\nThe IP address range of the Kubernetes services in this cluster, in CIDR notation (e.g. 1.2.3.4/29). Service addresses are typically put in the last /16 from the container CIDR."]
    pub fn services_ipv4_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.services_ipv4_cidr", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nThe name or self_link of the Google Compute Engine subnetwork in which the cluster's instances are launched."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnetwork", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tpu_ipv4_cidr_block` after provisioning.\nThe IP address range of the Cloud TPUs in this cluster, in CIDR notation (e.g. 1.2.3.4/29)."]
    pub fn tpu_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tpu_ipv4_cidr_block", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_managed_keys_config` after provisioning.\nThe custom keys configuration of the cluster."]
    pub fn user_managed_keys_config(
        &self,
    ) -> ListRef<DataContainerClusterUserManagedKeysConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_managed_keys_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vertical_pod_autoscaling` after provisioning.\nVertical Pod Autoscaling automatically adjusts the resources of pods controlled by it."]
    pub fn vertical_pod_autoscaling(
        &self,
    ) -> ListRef<DataContainerClusterVerticalPodAutoscalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vertical_pod_autoscaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_identity_config` after provisioning.\nConfiguration for the use of Kubernetes Service Accounts in GCP IAM policies."]
    pub fn workload_identity_config(
        &self,
    ) -> ListRef<DataContainerClusterWorkloadIdentityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload_identity_config", self.extract_ref()),
        )
    }
}
impl Referable for DataContainerCluster {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataContainerCluster {}
impl ToListMappable for DataContainerCluster {
    type O = ListRef<DataContainerClusterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataContainerCluster_ {
    fn extract_datasource_type(&self) -> String {
        "google_container_cluster".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataContainerCluster {
    pub tf_id: String,
    #[doc = "The name of the cluster, unique within the project and location."]
    pub name: PrimField<String>,
}
impl BuildDataContainerCluster {
    pub fn build(self, stack: &mut Stack) -> DataContainerCluster {
        let out = DataContainerCluster(Rc::new(DataContainerCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataContainerClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataContainerClusterRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataContainerClusterRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `addons_config` after provisioning.\nThe configuration for addons supported by GKE."]
    pub fn addons_config(&self) -> ListRef<DataContainerClusterAddonsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.addons_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allow_net_admin` after provisioning.\nEnable NET_ADMIN for this cluster."]
    pub fn allow_net_admin(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_net_admin", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `anonymous_authentication_config` after provisioning.\nAnonymousAuthenticationConfig allows users to restrict or enable anonymous access to the cluster."]
    pub fn anonymous_authentication_config(
        &self,
    ) -> ListRef<DataContainerClusterAnonymousAuthenticationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.anonymous_authentication_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authenticator_groups_config` after provisioning.\nConfiguration for the Google Groups for GKE feature."]
    pub fn authenticator_groups_config(
        &self,
    ) -> ListRef<DataContainerClusterAuthenticatorGroupsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authenticator_groups_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `autopilot_cluster_policy_config` after provisioning.\nConfiguration for the cluster policy."]
    pub fn autopilot_cluster_policy_config(
        &self,
    ) -> ListRef<DataContainerClusterAutopilotClusterPolicyConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autopilot_cluster_policy_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `autopilot_privileged_admission` after provisioning.\nThe customer allowlist Cloud Storage paths for the cluster. These paths are used with the `--autopilot-privileged-admission` flag to authorize privileged workloads in Autopilot clusters. To allow default partner allowlists, set to []. To allow no allowlists, set to [\"\"]."]
    pub fn autopilot_privileged_admission(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autopilot_privileged_admission", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `binary_authorization` after provisioning.\nConfiguration options for the Binary Authorization feature."]
    pub fn binary_authorization(&self) -> ListRef<DataContainerClusterBinaryAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.binary_authorization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_autoscaling` after provisioning.\nPer-cluster configuration of Node Auto-Provisioning with Cluster Autoscaler to automatically adjust the size of the cluster and create/delete node pools based on the current needs of the cluster's workload. See the guide to using Node Auto-Provisioning for more details."]
    pub fn cluster_autoscaling(&self) -> ListRef<DataContainerClusterClusterAutoscalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_autoscaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_ipv4_cidr` after provisioning.\nThe IP address range of the Kubernetes pods in this cluster in CIDR notation (e.g. 10.96.0.0/14). Leave blank to have one automatically chosen or specify a /14 block in 10.0.0.0/8. This field will only work for routes-based clusters, where ip_allocation_policy is not defined."]
    pub fn cluster_ipv4_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_ipv4_cidr", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `confidential_nodes` after provisioning.\nConfiguration for the confidential nodes feature, which makes nodes run on confidential VMs. Warning: This configuration can't be changed (or added/removed) after cluster creation without deleting and recreating the entire cluster."]
    pub fn confidential_nodes(&self) -> ListRef<DataContainerClusterConfidentialNodesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.confidential_nodes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane_endpoints_config` after provisioning.\nConfiguration for all of the cluster's control plane endpoints. Currently supports only DNS endpoint configuration and disable IP endpoint. Other IP endpoint configurations are available in private_cluster_config."]
    pub fn control_plane_endpoints_config(
        &self,
    ) -> ListRef<DataContainerClusterControlPlaneEndpointsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane_endpoints_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cost_management_config` after provisioning.\nCost management configuration for the cluster."]
    pub fn cost_management_config(&self) -> ListRef<DataContainerClusterCostManagementConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cost_management_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_encryption` after provisioning.\nApplication-layer Secrets Encryption settings. The object format is {state = string, key_name = string}. Valid values of state are: \"ENCRYPTED\"; \"DECRYPTED\". key_name is the name of a CloudKMS key."]
    pub fn database_encryption(&self) -> ListRef<DataContainerClusterDatabaseEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.database_encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `datapath_provider` after provisioning.\nThe desired datapath provider for this cluster. By default, uses the IPTables-based kube-proxy implementation."]
    pub fn datapath_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.datapath_provider", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_max_pods_per_node` after provisioning.\nThe default maximum number of pods per node in this cluster. This doesn't work on \"routes-based\" clusters, clusters that don't have IP Aliasing enabled."]
    pub fn default_max_pods_per_node(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_max_pods_per_node", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_snat_status` after provisioning.\nWhether the cluster disables default in-node sNAT rules. In-node sNAT rules will be disabled when defaultSnatStatus is disabled."]
    pub fn default_snat_status(&self) -> ListRef<DataContainerClusterDefaultSnatStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_snat_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhen the field is set to true or unset in Terraform state, a terraform apply or terraform destroy that would delete the cluster will fail. When the field is set to false, deleting the cluster is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n Description of the cluster."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disable_l4_lb_firewall_reconciliation` after provisioning.\nDisable L4 load balancer VPC firewalls to enable firewall policies."]
    pub fn disable_l4_lb_firewall_reconciliation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.disable_l4_lb_firewall_reconciliation",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `dns_config` after provisioning.\nConfiguration for Cloud DNS for Kubernetes Engine."]
    pub fn dns_config(&self) -> ListRef<DataContainerClusterDnsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dns_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_autopilot` after provisioning.\nEnable Autopilot for this cluster."]
    pub fn enable_autopilot(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_autopilot", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_cilium_clusterwide_network_policy` after provisioning.\nWhether Cilium cluster-wide network policy is enabled on this cluster."]
    pub fn enable_cilium_clusterwide_network_policy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.enable_cilium_clusterwide_network_policy",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `enable_fqdn_network_policy` after provisioning.\nWhether FQDN Network Policy is enabled on this cluster."]
    pub fn enable_fqdn_network_policy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_fqdn_network_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_intranode_visibility` after provisioning.\nWhether Intra-node visibility is enabled for this cluster. This makes same node pod to pod traffic visible for VPC network."]
    pub fn enable_intranode_visibility(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_intranode_visibility", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_k8s_beta_apis` after provisioning.\nConfiguration for Kubernetes Beta APIs."]
    pub fn enable_k8s_beta_apis(&self) -> ListRef<DataContainerClusterEnableK8sBetaApisElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enable_k8s_beta_apis", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_kubernetes_alpha` after provisioning.\nWhether to enable Kubernetes Alpha features for this cluster. Note that when this option is enabled, the cluster cannot be upgraded and will be automatically deleted after 30 days."]
    pub fn enable_kubernetes_alpha(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_kubernetes_alpha", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_l4_ilb_subsetting` after provisioning.\nWhether L4ILB Subsetting is enabled for this cluster."]
    pub fn enable_l4_ilb_subsetting(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_l4_ilb_subsetting", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_legacy_abac` after provisioning.\nWhether the ABAC authorizer is enabled for this cluster. When enabled, identities in the system, including service accounts, nodes, and controllers, will have statically granted permissions beyond those provided by the RBAC configuration or IAM. Defaults to false."]
    pub fn enable_legacy_abac(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_legacy_abac", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_multi_networking` after provisioning.\nWhether multi-networking is enabled for this cluster."]
    pub fn enable_multi_networking(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_multi_networking", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_shielded_nodes` after provisioning.\nEnable Shielded Nodes features on all nodes in this cluster. Defaults to true."]
    pub fn enable_shielded_nodes(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_shielded_nodes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_tpu` after provisioning.\nWhether to enable Cloud TPU resources in this cluster."]
    pub fn enable_tpu(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_tpu", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nThe IP address of this cluster's Kubernetes master."]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enterprise_config` after provisioning.\nDefines the config needed to enable/disable GKE Enterprise"]
    pub fn enterprise_config(&self) -> ListRef<DataContainerClusterEnterpriseConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enterprise_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet` after provisioning.\nFleet configuration of the cluster."]
    pub fn fleet(&self) -> ListRef<DataContainerClusterFleetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gateway_api_config` after provisioning.\nConfiguration for GKE Gateway API controller."]
    pub fn gateway_api_config(&self) -> ListRef<DataContainerClusterGatewayApiConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gateway_api_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gke_auto_upgrade_config` after provisioning.\nConfiguration options for the auto-upgrade patch type feature, which provide more control over the speed of automatic upgrades of your GKE clusters."]
    pub fn gke_auto_upgrade_config(
        &self,
    ) -> ListRef<DataContainerClusterGkeAutoUpgradeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gke_auto_upgrade_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `identity_service_config` after provisioning.\nConfiguration for Identity Service which allows customers to use external identity providers with the K8S API."]
    pub fn identity_service_config(
        &self,
    ) -> ListRef<DataContainerClusterIdentityServiceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.identity_service_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `in_transit_encryption_config` after provisioning.\nDefines the config of in-transit encryption"]
    pub fn in_transit_encryption_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.in_transit_encryption_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `initial_node_count` after provisioning.\nThe number of nodes to create in this cluster's default node pool. In regional or multi-zonal clusters, this is the number of nodes per zone. Must be set if node_pool is not set. If you're using google_container_node_pool objects with no default node pool, you'll need to set this to a value of at least 1, alongside setting remove_default_node_pool to true."]
    pub fn initial_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_node_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_allocation_policy` after provisioning.\nConfiguration of cluster IP allocation for VPC-native clusters. Adding this block enables IP aliasing, making the cluster VPC-native instead of routes-based."]
    pub fn ip_allocation_policy(&self) -> ListRef<DataContainerClusterIpAllocationPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ip_allocation_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint of the set of labels for this cluster."]
    pub fn label_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location (region or zone) in which the cluster master will be created, as well as the default node location. If you specify a zone (such as us-central1-a), the cluster will be a zonal cluster with a single cluster master. If you specify a region (such as us-west1), the cluster will be a regional cluster with multiple masters spread across zones in the region, and with default node locations in those zones as well."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\nLogging configuration for the cluster."]
    pub fn logging_config(&self) -> ListRef<DataContainerClusterLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_service` after provisioning.\nThe logging service that the cluster should write logs to. Available options include logging.googleapis.com(Legacy Stackdriver), logging.googleapis.com/kubernetes(Stackdriver Kubernetes Engine Logging), and none. Defaults to logging.googleapis.com/kubernetes."]
    pub fn logging_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logging_service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\nThe maintenance policy to use for the cluster."]
    pub fn maintenance_policy(&self) -> ListRef<DataContainerClusterMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `master_auth` after provisioning.\nThe authentication information for accessing the Kubernetes master. Some values in this block are only returned by the API if your service account has permission to get credentials for your GKE cluster. If you see an unexpected diff unsetting your client cert, ensure you have the container.clusters.getCredentials permission."]
    pub fn master_auth(&self) -> ListRef<DataContainerClusterMasterAuthElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.master_auth", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `master_authorized_networks_config` after provisioning.\nThe desired configuration options for master authorized networks. Omit the nested cidr_blocks attribute to disallow external access (except the cluster node IPs, which GKE automatically whitelists)."]
    pub fn master_authorized_networks_config(
        &self,
    ) -> ListRef<DataContainerClusterMasterAuthorizedNetworksConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.master_authorized_networks_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `master_version` after provisioning.\nThe current version of the master in the cluster. This may be different than the min_master_version set in the config if the master has been updated by GKE."]
    pub fn master_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mesh_certificates` after provisioning.\nIf set, and enable_certificates=true, the GKE Workload Identity Certificates controller and node agent will be deployed in the cluster."]
    pub fn mesh_certificates(&self) -> ListRef<DataContainerClusterMeshCertificatesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mesh_certificates", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `min_master_version` after provisioning.\nThe minimum version of the master. GKE will auto-update the master to new versions, so this does not guarantee the current master version--use the read-only master_version field to obtain that. If unset, the cluster's version will be set by GKE to the version of the most recent official release (which is not necessarily the latest version)."]
    pub fn min_master_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_master_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monitoring_config` after provisioning.\nMonitoring configuration for the cluster."]
    pub fn monitoring_config(&self) -> ListRef<DataContainerClusterMonitoringConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.monitoring_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monitoring_service` after provisioning.\nThe monitoring service that the cluster should write metrics to. Automatically send metrics from pods in the cluster to the Google Cloud Monitoring API. VM metrics will be collected by Google Compute Engine regardless of this setting Available options include monitoring.googleapis.com(Legacy Stackdriver), monitoring.googleapis.com/kubernetes(Stackdriver Kubernetes Engine Monitoring), and none. Defaults to monitoring.googleapis.com/kubernetes."]
    pub fn monitoring_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monitoring_service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the cluster, unique within the project and location."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe name or self_link of the Google Compute Engine network to which the cluster is connected. For Shared VPC, set this to the self link of the shared network."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_performance_config` after provisioning.\nNetwork bandwidth tier configuration."]
    pub fn network_performance_config(
        &self,
    ) -> ListRef<DataContainerClusterNetworkPerformanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_performance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_policy` after provisioning.\nConfiguration options for the NetworkPolicy feature."]
    pub fn network_policy(&self) -> ListRef<DataContainerClusterNetworkPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `networking_mode` after provisioning.\nDetermines whether alias IPs or routes will be used for pod IPs in the cluster. Defaults to VPC_NATIVE for new clusters."]
    pub fn networking_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.networking_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_config` after provisioning.\nThe configuration of the nodepool"]
    pub fn node_config(&self) -> ListRef<DataContainerClusterNodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_locations` after provisioning.\nThe list of zones in which the cluster's nodes are located. Nodes must be in the region of their regional cluster or in the same region as their cluster's zone for zonal clusters. If this is specified for a zonal cluster, omit the cluster's zone."]
    pub fn node_locations(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.node_locations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_pool` after provisioning.\nList of node pools associated with this cluster. See google_container_node_pool for schema. Warning: node pools defined inside a cluster can't be changed (or added/removed) after cluster creation without deleting and recreating the entire cluster. Unless you absolutely need the ability to say \"these are the only node pools associated with this cluster\", use the google_container_node_pool resource instead of this property."]
    pub fn node_pool(&self) -> ListRef<DataContainerClusterNodePoolElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_pool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_pool_auto_config` after provisioning.\nNode pool configs that apply to all auto-provisioned node pools in autopilot clusters and node auto-provisioning enabled clusters."]
    pub fn node_pool_auto_config(&self) -> ListRef<DataContainerClusterNodePoolAutoConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_pool_auto_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_pool_defaults` after provisioning.\nThe default nodel pool settings for the entire cluster."]
    pub fn node_pool_defaults(&self) -> ListRef<DataContainerClusterNodePoolDefaultsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_pool_defaults", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_version` after provisioning.\nThe Kubernetes version on the nodes. Must either be unset or set to the same value as min_master_version on create. Defaults to the default version set by GKE which is not necessarily the latest version. This only affects nodes in the default node pool. While a fuzzy version can be specified, it's recommended that you specify explicit versions as Terraform will see spurious diffs when fuzzy versions are used. See the google_container_engine_versions data source's version_prefix field to approximate fuzzy versions in a Terraform-compatible way. To update nodes in other node pools, use the version attribute on the node pool."]
    pub fn node_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\nThe notification config for sending cluster upgrade notifications"]
    pub fn notification_config(&self) -> ListRef<DataContainerClusterNotificationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pod_autoscaling` after provisioning.\nPodAutoscaling is used for configuration of parameters for workload autoscaling"]
    pub fn pod_autoscaling(&self) -> ListRef<DataContainerClusterPodAutoscalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pod_autoscaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_cluster_config` after provisioning.\nConfiguration for private clusters, clusters with private nodes."]
    pub fn private_cluster_config(&self) -> ListRef<DataContainerClusterPrivateClusterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_cluster_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_ipv6_google_access` after provisioning.\nThe desired state of IPv6 connectivity to Google Services. By default, no private IPv6 access to or from Google Services (all access will be via IPv4)."]
    pub fn private_ipv6_google_access(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_ipv6_google_access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rbac_binding_config` after provisioning.\nRBACBindingConfig allows user to restrict ClusterRoleBindings an RoleBindings that can be created."]
    pub fn rbac_binding_config(&self) -> ListRef<DataContainerClusterRbacBindingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rbac_binding_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `release_channel` after provisioning.\nConfiguration options for the Release channel feature, which provide more control over automatic upgrades of your GKE clusters. Note that removing this field from your config will not unenroll it. Instead, use the \"UNSPECIFIED\" channel."]
    pub fn release_channel(&self) -> ListRef<DataContainerClusterReleaseChannelElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.release_channel", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `remove_default_node_pool` after provisioning.\nIf true, deletes the default node pool upon cluster creation. If you're using google_container_node_pool resources with no default node pool, this should be set to true, alongside setting initial_node_count to at least 1."]
    pub fn remove_default_node_pool(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.remove_default_node_pool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_labels` after provisioning.\nThe GCE resource labels (a map of key/value pairs) to be applied to the cluster.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn resource_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_usage_export_config` after provisioning.\nConfiguration for the ResourceUsageExportConfig feature."]
    pub fn resource_usage_export_config(
        &self,
    ) -> ListRef<DataContainerClusterResourceUsageExportConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_usage_export_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secret_manager_config` after provisioning.\nConfiguration for the Secret Manager feature."]
    pub fn secret_manager_config(&self) -> ListRef<DataContainerClusterSecretManagerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secret_manager_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secret_sync_config` after provisioning.\nConfiguration for the Sync as k8s secrets feature."]
    pub fn secret_sync_config(&self) -> ListRef<DataContainerClusterSecretSyncConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secret_sync_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_posture_config` after provisioning.\nDefines the config needed to enable/disable features for the Security Posture API"]
    pub fn security_posture_config(
        &self,
    ) -> ListRef<DataContainerClusterSecurityPostureConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_posture_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL for the resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_external_ips_config` after provisioning.\nIf set, and enabled=true, services with external ips field will not be blocked"]
    pub fn service_external_ips_config(
        &self,
    ) -> ListRef<DataContainerClusterServiceExternalIpsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_external_ips_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `services_ipv4_cidr` after provisioning.\nThe IP address range of the Kubernetes services in this cluster, in CIDR notation (e.g. 1.2.3.4/29). Service addresses are typically put in the last /16 from the container CIDR."]
    pub fn services_ipv4_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.services_ipv4_cidr", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nThe name or self_link of the Google Compute Engine subnetwork in which the cluster's instances are launched."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnetwork", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tpu_ipv4_cidr_block` after provisioning.\nThe IP address range of the Cloud TPUs in this cluster, in CIDR notation (e.g. 1.2.3.4/29)."]
    pub fn tpu_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tpu_ipv4_cidr_block", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_managed_keys_config` after provisioning.\nThe custom keys configuration of the cluster."]
    pub fn user_managed_keys_config(
        &self,
    ) -> ListRef<DataContainerClusterUserManagedKeysConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_managed_keys_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vertical_pod_autoscaling` after provisioning.\nVertical Pod Autoscaling automatically adjusts the resources of pods controlled by it."]
    pub fn vertical_pod_autoscaling(
        &self,
    ) -> ListRef<DataContainerClusterVerticalPodAutoscalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vertical_pod_autoscaling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_identity_config` after provisioning.\nConfiguration for the use of Kubernetes Service Accounts in GCP IAM policies."]
    pub fn workload_identity_config(
        &self,
    ) -> ListRef<DataContainerClusterWorkloadIdentityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload_identity_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElCloudrunConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer_type: Option<PrimField<String>>,
}
impl DataContainerClusterAddonsConfigElCloudrunConfigEl {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `load_balancer_type`.\n"]
    pub fn set_load_balancer_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.load_balancer_type = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterAddonsConfigElCloudrunConfigEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElCloudrunConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterAddonsConfigElCloudrunConfigEl {}
impl BuildDataContainerClusterAddonsConfigElCloudrunConfigEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElCloudrunConfigEl {
        DataContainerClusterAddonsConfigElCloudrunConfigEl {
            disabled: core::default::Default::default(),
            load_balancer_type: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterAddonsConfigElCloudrunConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterAddonsConfigElCloudrunConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterAddonsConfigElCloudrunConfigElRef {
        DataContainerClusterAddonsConfigElCloudrunConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterAddonsConfigElCloudrunConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `load_balancer_type` after provisioning.\n"]
    pub fn load_balancer_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancer_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElConfigConnectorConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterAddonsConfigElConfigConnectorConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterAddonsConfigElConfigConnectorConfigEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElConfigConnectorConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterAddonsConfigElConfigConnectorConfigEl {}
impl BuildDataContainerClusterAddonsConfigElConfigConnectorConfigEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElConfigConnectorConfigEl {
        DataContainerClusterAddonsConfigElConfigConnectorConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterAddonsConfigElConfigConnectorConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterAddonsConfigElConfigConnectorConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterAddonsConfigElConfigConnectorConfigElRef {
        DataContainerClusterAddonsConfigElConfigConnectorConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterAddonsConfigElConfigConnectorConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElDnsCacheConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterAddonsConfigElDnsCacheConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterAddonsConfigElDnsCacheConfigEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElDnsCacheConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterAddonsConfigElDnsCacheConfigEl {}
impl BuildDataContainerClusterAddonsConfigElDnsCacheConfigEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElDnsCacheConfigEl {
        DataContainerClusterAddonsConfigElDnsCacheConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterAddonsConfigElDnsCacheConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterAddonsConfigElDnsCacheConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterAddonsConfigElDnsCacheConfigElRef {
        DataContainerClusterAddonsConfigElDnsCacheConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterAddonsConfigElDnsCacheConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl {}
impl BuildDataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl {
        DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigElRef {
        DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl {}
impl BuildDataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl {
        DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigElRef {
        DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl {}
impl BuildDataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl {
        DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigElRef {
        DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElGkeBackupAgentConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterAddonsConfigElGkeBackupAgentConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterAddonsConfigElGkeBackupAgentConfigEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElGkeBackupAgentConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterAddonsConfigElGkeBackupAgentConfigEl {}
impl BuildDataContainerClusterAddonsConfigElGkeBackupAgentConfigEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElGkeBackupAgentConfigEl {
        DataContainerClusterAddonsConfigElGkeBackupAgentConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterAddonsConfigElGkeBackupAgentConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterAddonsConfigElGkeBackupAgentConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterAddonsConfigElGkeBackupAgentConfigElRef {
        DataContainerClusterAddonsConfigElGkeBackupAgentConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterAddonsConfigElGkeBackupAgentConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElHorizontalPodAutoscalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
}
impl DataContainerClusterAddonsConfigElHorizontalPodAutoscalingEl {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterAddonsConfigElHorizontalPodAutoscalingEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElHorizontalPodAutoscalingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterAddonsConfigElHorizontalPodAutoscalingEl {}
impl BuildDataContainerClusterAddonsConfigElHorizontalPodAutoscalingEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElHorizontalPodAutoscalingEl {
        DataContainerClusterAddonsConfigElHorizontalPodAutoscalingEl {
            disabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterAddonsConfigElHorizontalPodAutoscalingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterAddonsConfigElHorizontalPodAutoscalingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterAddonsConfigElHorizontalPodAutoscalingElRef {
        DataContainerClusterAddonsConfigElHorizontalPodAutoscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterAddonsConfigElHorizontalPodAutoscalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElHttpLoadBalancingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
}
impl DataContainerClusterAddonsConfigElHttpLoadBalancingEl {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterAddonsConfigElHttpLoadBalancingEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElHttpLoadBalancingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterAddonsConfigElHttpLoadBalancingEl {}
impl BuildDataContainerClusterAddonsConfigElHttpLoadBalancingEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElHttpLoadBalancingEl {
        DataContainerClusterAddonsConfigElHttpLoadBalancingEl {
            disabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterAddonsConfigElHttpLoadBalancingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterAddonsConfigElHttpLoadBalancingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterAddonsConfigElHttpLoadBalancingElRef {
        DataContainerClusterAddonsConfigElHttpLoadBalancingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterAddonsConfigElHttpLoadBalancingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElLustreCsiDriverConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_multi_nic: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_legacy_lustre_port: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterAddonsConfigElLustreCsiDriverConfigEl {
    #[doc = "Set the field `disable_multi_nic`.\n"]
    pub fn set_disable_multi_nic(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_multi_nic = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_legacy_lustre_port`.\n"]
    pub fn set_enable_legacy_lustre_port(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_legacy_lustre_port = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterAddonsConfigElLustreCsiDriverConfigEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElLustreCsiDriverConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterAddonsConfigElLustreCsiDriverConfigEl {}
impl BuildDataContainerClusterAddonsConfigElLustreCsiDriverConfigEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElLustreCsiDriverConfigEl {
        DataContainerClusterAddonsConfigElLustreCsiDriverConfigEl {
            disable_multi_nic: core::default::Default::default(),
            enable_legacy_lustre_port: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterAddonsConfigElLustreCsiDriverConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterAddonsConfigElLustreCsiDriverConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterAddonsConfigElLustreCsiDriverConfigElRef {
        DataContainerClusterAddonsConfigElLustreCsiDriverConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterAddonsConfigElLustreCsiDriverConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_multi_nic` after provisioning.\n"]
    pub fn disable_multi_nic(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_multi_nic", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_legacy_lustre_port` after provisioning.\n"]
    pub fn enable_legacy_lustre_port(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_legacy_lustre_port", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElNetworkPolicyConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
}
impl DataContainerClusterAddonsConfigElNetworkPolicyConfigEl {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterAddonsConfigElNetworkPolicyConfigEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElNetworkPolicyConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterAddonsConfigElNetworkPolicyConfigEl {}
impl BuildDataContainerClusterAddonsConfigElNetworkPolicyConfigEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElNetworkPolicyConfigEl {
        DataContainerClusterAddonsConfigElNetworkPolicyConfigEl {
            disabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterAddonsConfigElNetworkPolicyConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterAddonsConfigElNetworkPolicyConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterAddonsConfigElNetworkPolicyConfigElRef {
        DataContainerClusterAddonsConfigElNetworkPolicyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterAddonsConfigElNetworkPolicyConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElParallelstoreCsiDriverConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterAddonsConfigElParallelstoreCsiDriverConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterAddonsConfigElParallelstoreCsiDriverConfigEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElParallelstoreCsiDriverConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterAddonsConfigElParallelstoreCsiDriverConfigEl {}
impl BuildDataContainerClusterAddonsConfigElParallelstoreCsiDriverConfigEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElParallelstoreCsiDriverConfigEl {
        DataContainerClusterAddonsConfigElParallelstoreCsiDriverConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterAddonsConfigElParallelstoreCsiDriverConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterAddonsConfigElParallelstoreCsiDriverConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterAddonsConfigElParallelstoreCsiDriverConfigElRef {
        DataContainerClusterAddonsConfigElParallelstoreCsiDriverConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterAddonsConfigElParallelstoreCsiDriverConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElPodSnapshotConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterAddonsConfigElPodSnapshotConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterAddonsConfigElPodSnapshotConfigEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElPodSnapshotConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterAddonsConfigElPodSnapshotConfigEl {}
impl BuildDataContainerClusterAddonsConfigElPodSnapshotConfigEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElPodSnapshotConfigEl {
        DataContainerClusterAddonsConfigElPodSnapshotConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterAddonsConfigElPodSnapshotConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterAddonsConfigElPodSnapshotConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterAddonsConfigElPodSnapshotConfigElRef {
        DataContainerClusterAddonsConfigElPodSnapshotConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterAddonsConfigElPodSnapshotConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterLoggingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterLoggingConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterLoggingConfigEl
{
    type O = BlockAssignable<
        DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterLoggingConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterLoggingConfigEl {}
impl BuildDataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterLoggingConfigEl {
    pub fn build(
        self,
    ) -> DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterLoggingConfigEl {
        DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterLoggingConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterLoggingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterLoggingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterLoggingConfigElRef {
        DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterLoggingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterMonitoringConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterMonitoringConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterMonitoringConfigEl
{
    type O = BlockAssignable<
        DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterMonitoringConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterMonitoringConfigEl {
}
impl BuildDataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterMonitoringConfigEl {
    pub fn build(
        self,
    ) -> DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterMonitoringConfigEl {
        DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterMonitoringConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterMonitoringConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterMonitoringConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterMonitoringConfigElRef {
        DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterMonitoringConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterMonitoringConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElRayOperatorConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ray_cluster_logging_config: Option<
        ListField<DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterLoggingConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    ray_cluster_monitoring_config: Option<
        ListField<
            DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterMonitoringConfigEl,
        >,
    >,
}
impl DataContainerClusterAddonsConfigElRayOperatorConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `ray_cluster_logging_config`.\n"]
    pub fn set_ray_cluster_logging_config(
        mut self,
        v: impl Into<
            ListField<
                DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterLoggingConfigEl,
            >,
        >,
    ) -> Self {
        self.ray_cluster_logging_config = Some(v.into());
        self
    }
    #[doc = "Set the field `ray_cluster_monitoring_config`.\n"]
    pub fn set_ray_cluster_monitoring_config(
        mut self,
        v: impl Into<
            ListField<
                DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterMonitoringConfigEl,
            >,
        >,
    ) -> Self {
        self.ray_cluster_monitoring_config = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterAddonsConfigElRayOperatorConfigEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElRayOperatorConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterAddonsConfigElRayOperatorConfigEl {}
impl BuildDataContainerClusterAddonsConfigElRayOperatorConfigEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElRayOperatorConfigEl {
        DataContainerClusterAddonsConfigElRayOperatorConfigEl {
            enabled: core::default::Default::default(),
            ray_cluster_logging_config: core::default::Default::default(),
            ray_cluster_monitoring_config: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterAddonsConfigElRayOperatorConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterAddonsConfigElRayOperatorConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterAddonsConfigElRayOperatorConfigElRef {
        DataContainerClusterAddonsConfigElRayOperatorConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterAddonsConfigElRayOperatorConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `ray_cluster_logging_config` after provisioning.\n"]
    pub fn ray_cluster_logging_config(
        &self,
    ) -> ListRef<DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterLoggingConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ray_cluster_logging_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ray_cluster_monitoring_config` after provisioning.\n"]
    pub fn ray_cluster_monitoring_config(
        &self,
    ) -> ListRef<DataContainerClusterAddonsConfigElRayOperatorConfigElRayClusterMonitoringConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ray_cluster_monitoring_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElSliceControllerConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterAddonsConfigElSliceControllerConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterAddonsConfigElSliceControllerConfigEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElSliceControllerConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterAddonsConfigElSliceControllerConfigEl {}
impl BuildDataContainerClusterAddonsConfigElSliceControllerConfigEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElSliceControllerConfigEl {
        DataContainerClusterAddonsConfigElSliceControllerConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterAddonsConfigElSliceControllerConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterAddonsConfigElSliceControllerConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterAddonsConfigElSliceControllerConfigElRef {
        DataContainerClusterAddonsConfigElSliceControllerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterAddonsConfigElSliceControllerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigElStatefulHaConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterAddonsConfigElStatefulHaConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterAddonsConfigElStatefulHaConfigEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigElStatefulHaConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterAddonsConfigElStatefulHaConfigEl {}
impl BuildDataContainerClusterAddonsConfigElStatefulHaConfigEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigElStatefulHaConfigEl {
        DataContainerClusterAddonsConfigElStatefulHaConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterAddonsConfigElStatefulHaConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterAddonsConfigElStatefulHaConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterAddonsConfigElStatefulHaConfigElRef {
        DataContainerClusterAddonsConfigElStatefulHaConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterAddonsConfigElStatefulHaConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterAddonsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudrun_config: Option<ListField<DataContainerClusterAddonsConfigElCloudrunConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config_connector_config:
        Option<ListField<DataContainerClusterAddonsConfigElConfigConnectorConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_cache_config: Option<ListField<DataContainerClusterAddonsConfigElDnsCacheConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gce_persistent_disk_csi_driver_config:
        Option<ListField<DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_filestore_csi_driver_config:
        Option<ListField<DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_fuse_csi_driver_config:
        Option<ListField<DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gke_backup_agent_config:
        Option<ListField<DataContainerClusterAddonsConfigElGkeBackupAgentConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_pod_autoscaling:
        Option<ListField<DataContainerClusterAddonsConfigElHorizontalPodAutoscalingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_load_balancing: Option<ListField<DataContainerClusterAddonsConfigElHttpLoadBalancingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lustre_csi_driver_config:
        Option<ListField<DataContainerClusterAddonsConfigElLustreCsiDriverConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_policy_config:
        Option<ListField<DataContainerClusterAddonsConfigElNetworkPolicyConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parallelstore_csi_driver_config:
        Option<ListField<DataContainerClusterAddonsConfigElParallelstoreCsiDriverConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_snapshot_config: Option<ListField<DataContainerClusterAddonsConfigElPodSnapshotConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ray_operator_config: Option<ListField<DataContainerClusterAddonsConfigElRayOperatorConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slice_controller_config:
        Option<ListField<DataContainerClusterAddonsConfigElSliceControllerConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stateful_ha_config: Option<ListField<DataContainerClusterAddonsConfigElStatefulHaConfigEl>>,
}
impl DataContainerClusterAddonsConfigEl {
    #[doc = "Set the field `cloudrun_config`.\n"]
    pub fn set_cloudrun_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElCloudrunConfigEl>>,
    ) -> Self {
        self.cloudrun_config = Some(v.into());
        self
    }
    #[doc = "Set the field `config_connector_config`.\n"]
    pub fn set_config_connector_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElConfigConnectorConfigEl>>,
    ) -> Self {
        self.config_connector_config = Some(v.into());
        self
    }
    #[doc = "Set the field `dns_cache_config`.\n"]
    pub fn set_dns_cache_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElDnsCacheConfigEl>>,
    ) -> Self {
        self.dns_cache_config = Some(v.into());
        self
    }
    #[doc = "Set the field `gce_persistent_disk_csi_driver_config`.\n"]
    pub fn set_gce_persistent_disk_csi_driver_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigEl>>,
    ) -> Self {
        self.gce_persistent_disk_csi_driver_config = Some(v.into());
        self
    }
    #[doc = "Set the field `gcp_filestore_csi_driver_config`.\n"]
    pub fn set_gcp_filestore_csi_driver_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigEl>>,
    ) -> Self {
        self.gcp_filestore_csi_driver_config = Some(v.into());
        self
    }
    #[doc = "Set the field `gcs_fuse_csi_driver_config`.\n"]
    pub fn set_gcs_fuse_csi_driver_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigEl>>,
    ) -> Self {
        self.gcs_fuse_csi_driver_config = Some(v.into());
        self
    }
    #[doc = "Set the field `gke_backup_agent_config`.\n"]
    pub fn set_gke_backup_agent_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElGkeBackupAgentConfigEl>>,
    ) -> Self {
        self.gke_backup_agent_config = Some(v.into());
        self
    }
    #[doc = "Set the field `horizontal_pod_autoscaling`.\n"]
    pub fn set_horizontal_pod_autoscaling(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElHorizontalPodAutoscalingEl>>,
    ) -> Self {
        self.horizontal_pod_autoscaling = Some(v.into());
        self
    }
    #[doc = "Set the field `http_load_balancing`.\n"]
    pub fn set_http_load_balancing(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElHttpLoadBalancingEl>>,
    ) -> Self {
        self.http_load_balancing = Some(v.into());
        self
    }
    #[doc = "Set the field `lustre_csi_driver_config`.\n"]
    pub fn set_lustre_csi_driver_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElLustreCsiDriverConfigEl>>,
    ) -> Self {
        self.lustre_csi_driver_config = Some(v.into());
        self
    }
    #[doc = "Set the field `network_policy_config`.\n"]
    pub fn set_network_policy_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElNetworkPolicyConfigEl>>,
    ) -> Self {
        self.network_policy_config = Some(v.into());
        self
    }
    #[doc = "Set the field `parallelstore_csi_driver_config`.\n"]
    pub fn set_parallelstore_csi_driver_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElParallelstoreCsiDriverConfigEl>>,
    ) -> Self {
        self.parallelstore_csi_driver_config = Some(v.into());
        self
    }
    #[doc = "Set the field `pod_snapshot_config`.\n"]
    pub fn set_pod_snapshot_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElPodSnapshotConfigEl>>,
    ) -> Self {
        self.pod_snapshot_config = Some(v.into());
        self
    }
    #[doc = "Set the field `ray_operator_config`.\n"]
    pub fn set_ray_operator_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElRayOperatorConfigEl>>,
    ) -> Self {
        self.ray_operator_config = Some(v.into());
        self
    }
    #[doc = "Set the field `slice_controller_config`.\n"]
    pub fn set_slice_controller_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElSliceControllerConfigEl>>,
    ) -> Self {
        self.slice_controller_config = Some(v.into());
        self
    }
    #[doc = "Set the field `stateful_ha_config`.\n"]
    pub fn set_stateful_ha_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterAddonsConfigElStatefulHaConfigEl>>,
    ) -> Self {
        self.stateful_ha_config = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterAddonsConfigEl {
    type O = BlockAssignable<DataContainerClusterAddonsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterAddonsConfigEl {}
impl BuildDataContainerClusterAddonsConfigEl {
    pub fn build(self) -> DataContainerClusterAddonsConfigEl {
        DataContainerClusterAddonsConfigEl {
            cloudrun_config: core::default::Default::default(),
            config_connector_config: core::default::Default::default(),
            dns_cache_config: core::default::Default::default(),
            gce_persistent_disk_csi_driver_config: core::default::Default::default(),
            gcp_filestore_csi_driver_config: core::default::Default::default(),
            gcs_fuse_csi_driver_config: core::default::Default::default(),
            gke_backup_agent_config: core::default::Default::default(),
            horizontal_pod_autoscaling: core::default::Default::default(),
            http_load_balancing: core::default::Default::default(),
            lustre_csi_driver_config: core::default::Default::default(),
            network_policy_config: core::default::Default::default(),
            parallelstore_csi_driver_config: core::default::Default::default(),
            pod_snapshot_config: core::default::Default::default(),
            ray_operator_config: core::default::Default::default(),
            slice_controller_config: core::default::Default::default(),
            stateful_ha_config: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterAddonsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterAddonsConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterAddonsConfigElRef {
        DataContainerClusterAddonsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterAddonsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloudrun_config` after provisioning.\n"]
    pub fn cloudrun_config(
        &self,
    ) -> ListRef<DataContainerClusterAddonsConfigElCloudrunConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloudrun_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `config_connector_config` after provisioning.\n"]
    pub fn config_connector_config(
        &self,
    ) -> ListRef<DataContainerClusterAddonsConfigElConfigConnectorConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.config_connector_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dns_cache_config` after provisioning.\n"]
    pub fn dns_cache_config(
        &self,
    ) -> ListRef<DataContainerClusterAddonsConfigElDnsCacheConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dns_cache_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gce_persistent_disk_csi_driver_config` after provisioning.\n"]
    pub fn gce_persistent_disk_csi_driver_config(
        &self,
    ) -> ListRef<DataContainerClusterAddonsConfigElGcePersistentDiskCsiDriverConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gce_persistent_disk_csi_driver_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gcp_filestore_csi_driver_config` after provisioning.\n"]
    pub fn gcp_filestore_csi_driver_config(
        &self,
    ) -> ListRef<DataContainerClusterAddonsConfigElGcpFilestoreCsiDriverConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcp_filestore_csi_driver_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_fuse_csi_driver_config` after provisioning.\n"]
    pub fn gcs_fuse_csi_driver_config(
        &self,
    ) -> ListRef<DataContainerClusterAddonsConfigElGcsFuseCsiDriverConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcs_fuse_csi_driver_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gke_backup_agent_config` after provisioning.\n"]
    pub fn gke_backup_agent_config(
        &self,
    ) -> ListRef<DataContainerClusterAddonsConfigElGkeBackupAgentConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gke_backup_agent_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `horizontal_pod_autoscaling` after provisioning.\n"]
    pub fn horizontal_pod_autoscaling(
        &self,
    ) -> ListRef<DataContainerClusterAddonsConfigElHorizontalPodAutoscalingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.horizontal_pod_autoscaling", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `http_load_balancing` after provisioning.\n"]
    pub fn http_load_balancing(
        &self,
    ) -> ListRef<DataContainerClusterAddonsConfigElHttpLoadBalancingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http_load_balancing", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lustre_csi_driver_config` after provisioning.\n"]
    pub fn lustre_csi_driver_config(
        &self,
    ) -> ListRef<DataContainerClusterAddonsConfigElLustreCsiDriverConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lustre_csi_driver_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_policy_config` after provisioning.\n"]
    pub fn network_policy_config(
        &self,
    ) -> ListRef<DataContainerClusterAddonsConfigElNetworkPolicyConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_policy_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `parallelstore_csi_driver_config` after provisioning.\n"]
    pub fn parallelstore_csi_driver_config(
        &self,
    ) -> ListRef<DataContainerClusterAddonsConfigElParallelstoreCsiDriverConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.parallelstore_csi_driver_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pod_snapshot_config` after provisioning.\n"]
    pub fn pod_snapshot_config(
        &self,
    ) -> ListRef<DataContainerClusterAddonsConfigElPodSnapshotConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pod_snapshot_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ray_operator_config` after provisioning.\n"]
    pub fn ray_operator_config(
        &self,
    ) -> ListRef<DataContainerClusterAddonsConfigElRayOperatorConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ray_operator_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `slice_controller_config` after provisioning.\n"]
    pub fn slice_controller_config(
        &self,
    ) -> ListRef<DataContainerClusterAddonsConfigElSliceControllerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.slice_controller_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `stateful_ha_config` after provisioning.\n"]
    pub fn stateful_ha_config(
        &self,
    ) -> ListRef<DataContainerClusterAddonsConfigElStatefulHaConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stateful_ha_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterAnonymousAuthenticationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}
impl DataContainerClusterAnonymousAuthenticationConfigEl {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterAnonymousAuthenticationConfigEl {
    type O = BlockAssignable<DataContainerClusterAnonymousAuthenticationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterAnonymousAuthenticationConfigEl {}
impl BuildDataContainerClusterAnonymousAuthenticationConfigEl {
    pub fn build(self) -> DataContainerClusterAnonymousAuthenticationConfigEl {
        DataContainerClusterAnonymousAuthenticationConfigEl {
            mode: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterAnonymousAuthenticationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterAnonymousAuthenticationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterAnonymousAuthenticationConfigElRef {
        DataContainerClusterAnonymousAuthenticationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterAnonymousAuthenticationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterAuthenticatorGroupsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group: Option<PrimField<String>>,
}
impl DataContainerClusterAuthenticatorGroupsConfigEl {
    #[doc = "Set the field `security_group`.\n"]
    pub fn set_security_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.security_group = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterAuthenticatorGroupsConfigEl {
    type O = BlockAssignable<DataContainerClusterAuthenticatorGroupsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterAuthenticatorGroupsConfigEl {}
impl BuildDataContainerClusterAuthenticatorGroupsConfigEl {
    pub fn build(self) -> DataContainerClusterAuthenticatorGroupsConfigEl {
        DataContainerClusterAuthenticatorGroupsConfigEl {
            security_group: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterAuthenticatorGroupsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterAuthenticatorGroupsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterAuthenticatorGroupsConfigElRef {
        DataContainerClusterAuthenticatorGroupsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterAuthenticatorGroupsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `security_group` after provisioning.\n"]
    pub fn security_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_group", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterAutopilotClusterPolicyConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    no_standard_node_pools: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_system_impersonation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_system_mutation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_unsafe_webhooks: Option<PrimField<bool>>,
}
impl DataContainerClusterAutopilotClusterPolicyConfigEl {
    #[doc = "Set the field `no_standard_node_pools`.\n"]
    pub fn set_no_standard_node_pools(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.no_standard_node_pools = Some(v.into());
        self
    }
    #[doc = "Set the field `no_system_impersonation`.\n"]
    pub fn set_no_system_impersonation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.no_system_impersonation = Some(v.into());
        self
    }
    #[doc = "Set the field `no_system_mutation`.\n"]
    pub fn set_no_system_mutation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.no_system_mutation = Some(v.into());
        self
    }
    #[doc = "Set the field `no_unsafe_webhooks`.\n"]
    pub fn set_no_unsafe_webhooks(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.no_unsafe_webhooks = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterAutopilotClusterPolicyConfigEl {
    type O = BlockAssignable<DataContainerClusterAutopilotClusterPolicyConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterAutopilotClusterPolicyConfigEl {}
impl BuildDataContainerClusterAutopilotClusterPolicyConfigEl {
    pub fn build(self) -> DataContainerClusterAutopilotClusterPolicyConfigEl {
        DataContainerClusterAutopilotClusterPolicyConfigEl {
            no_standard_node_pools: core::default::Default::default(),
            no_system_impersonation: core::default::Default::default(),
            no_system_mutation: core::default::Default::default(),
            no_unsafe_webhooks: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterAutopilotClusterPolicyConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterAutopilotClusterPolicyConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterAutopilotClusterPolicyConfigElRef {
        DataContainerClusterAutopilotClusterPolicyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterAutopilotClusterPolicyConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `no_standard_node_pools` after provisioning.\n"]
    pub fn no_standard_node_pools(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.no_standard_node_pools", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `no_system_impersonation` after provisioning.\n"]
    pub fn no_system_impersonation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.no_system_impersonation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `no_system_mutation` after provisioning.\n"]
    pub fn no_system_mutation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.no_system_mutation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `no_unsafe_webhooks` after provisioning.\n"]
    pub fn no_unsafe_webhooks(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.no_unsafe_webhooks", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterBinaryAuthorizationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluation_mode: Option<PrimField<String>>,
}
impl DataContainerClusterBinaryAuthorizationEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `evaluation_mode`.\n"]
    pub fn set_evaluation_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.evaluation_mode = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterBinaryAuthorizationEl {
    type O = BlockAssignable<DataContainerClusterBinaryAuthorizationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterBinaryAuthorizationEl {}
impl BuildDataContainerClusterBinaryAuthorizationEl {
    pub fn build(self) -> DataContainerClusterBinaryAuthorizationEl {
        DataContainerClusterBinaryAuthorizationEl {
            enabled: core::default::Default::default(),
            evaluation_mode: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterBinaryAuthorizationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterBinaryAuthorizationElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterBinaryAuthorizationElRef {
        DataContainerClusterBinaryAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterBinaryAuthorizationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `evaluation_mode` after provisioning.\n"]
    pub fn evaluation_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.evaluation_mode", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_upgrade_start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
}
impl
    DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl
{
    #[doc = "Set the field `auto_upgrade_start_time`.\n"]
    pub fn set_auto_upgrade_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auto_upgrade_start_time = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl { type O = BlockAssignable < DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl
{}
impl BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl { pub fn build (self) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl { DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl { auto_upgrade_start_time : core :: default :: Default :: default () , description : core :: default :: Default :: default () , } } }
pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsElRef { fn new (shared : StackShared , base : String) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsElRef { DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsElRef { shared : shared , base : base . to_string () , } } }
impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `auto_upgrade_start_time` after provisioning.\n"] pub fn auto_upgrade_start_time (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.auto_upgrade_start_time" , self . base)) } # [doc = "Get a reference to the value of field `description` after provisioning.\n"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } }
#[derive(Serialize)]
pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl { # [serde (skip_serializing_if = "Option::is_none")] auto_repair : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] auto_upgrade : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] upgrade_options : Option < ListField < DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl > > , }
impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl {
    #[doc = "Set the field `auto_repair`.\n"]
    pub fn set_auto_repair(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_repair = Some(v.into());
        self
    }
    #[doc = "Set the field `auto_upgrade`.\n"]
    pub fn set_auto_upgrade(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_upgrade = Some(v.into());
        self
    }
    #[doc = "Set the field `upgrade_options`.\n"]
    pub fn set_upgrade_options(
        mut self,
        v : impl Into < ListField < DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsEl > >,
    ) -> Self {
        self.upgrade_options = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl
{
    type O = BlockAssignable<
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl {}
impl BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl {
    pub fn build(
        self,
    ) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl {
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl {
            auto_repair: core::default::Default::default(),
            auto_upgrade: core::default::Default::default(),
            upgrade_options: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElRef {
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_repair` after provisioning.\n"]
    pub fn auto_repair(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_repair", self.base))
    }
    #[doc = "Get a reference to the value of field `auto_upgrade` after provisioning.\n"]
    pub fn auto_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_upgrade", self.base))
    }
    #[doc = "Get a reference to the value of field `upgrade_options` after provisioning.\n"]    pub fn upgrade_options (& self) -> ListRef < DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElUpgradeOptionsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.upgrade_options", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_integrity_monitoring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_secure_boot: Option<PrimField<bool>>,
}
impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl {
    #[doc = "Set the field `enable_integrity_monitoring`.\n"]
    pub fn set_enable_integrity_monitoring(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_secure_boot`.\n"]
    pub fn set_enable_secure_boot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_secure_boot = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl
{
    type O = BlockAssignable<
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl
{}
impl
    BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl
{
    pub fn build(
        self,
    ) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl
    {
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigElRef { fn new (shared : StackShared , base : String) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigElRef { DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigElRef { shared : shared , base : base . to_string () , } } }
impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\n"]
    pub fn enable_integrity_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_integrity_monitoring", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_secure_boot` after provisioning.\n"]
    pub fn enable_secure_boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_secure_boot", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_soak_duration: Option<PrimField<String>>,
}
impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl { # [doc = "Set the field `batch_node_count`.\n"] pub fn set_batch_node_count (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . batch_node_count = Some (v . into ()) ; self } # [doc = "Set the field `batch_percentage`.\n"] pub fn set_batch_percentage (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . batch_percentage = Some (v . into ()) ; self } # [doc = "Set the field `batch_soak_duration`.\n"] pub fn set_batch_soak_duration (mut self , v : impl Into < PrimField < String > >) -> Self { self . batch_soak_duration = Some (v . into ()) ; self } }
impl ToListMappable for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl { type O = BlockAssignable < DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl
{}
impl BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl { pub fn build (self) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl { DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl { batch_node_count : core :: default :: Default :: default () , batch_percentage : core :: default :: Default :: default () , batch_soak_duration : core :: default :: Default :: default () , } } }
pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef { fn new (shared : StackShared , base : String) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef { DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef { shared : shared , base : base . to_string () , } } }
impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `batch_node_count` after provisioning.\n"] pub fn batch_node_count (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.batch_node_count" , self . base)) } # [doc = "Get a reference to the value of field `batch_percentage` after provisioning.\n"] pub fn batch_percentage (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.batch_percentage" , self . base)) } # [doc = "Get a reference to the value of field `batch_soak_duration` after provisioning.\n"] pub fn batch_soak_duration (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.batch_soak_duration" , self . base)) } }
#[derive(Serialize)]
pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] node_pool_soak_duration : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] standard_rollout_policy : Option < ListField < DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl > > , }
impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl { # [doc = "Set the field `node_pool_soak_duration`.\n"] pub fn set_node_pool_soak_duration (mut self , v : impl Into < PrimField < String > >) -> Self { self . node_pool_soak_duration = Some (v . into ()) ; self } # [doc = "Set the field `standard_rollout_policy`.\n"] pub fn set_standard_rollout_policy (mut self , v : impl Into < ListField < DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl > >) -> Self { self . standard_rollout_policy = Some (v . into ()) ; self } }
impl ToListMappable for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl { type O = BlockAssignable < DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl
{}
impl BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl { pub fn build (self) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl { DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl { node_pool_soak_duration : core :: default :: Default :: default () , standard_rollout_policy : core :: default :: Default :: default () , } } }
pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElRef { fn new (shared : StackShared , base : String) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElRef { DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElRef { shared : shared , base : base . to_string () , } } }
impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `node_pool_soak_duration` after provisioning.\n"] pub fn node_pool_soak_duration (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.node_pool_soak_duration" , self . base)) } # [doc = "Get a reference to the value of field `standard_rollout_policy` after provisioning.\n"] pub fn standard_rollout_policy (& self) -> ListRef < DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.standard_rollout_policy" , self . base)) } }
#[derive(Serialize)]
pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] blue_green_settings : Option < ListField < DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] max_surge : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] max_unavailable : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] strategy : Option < PrimField < String > > , }
impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl {
    #[doc = "Set the field `blue_green_settings`.\n"]
    pub fn set_blue_green_settings(
        mut self,
        v : impl Into < ListField < DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsEl > >,
    ) -> Self {
        self.blue_green_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `max_surge`.\n"]
    pub fn set_max_surge(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_surge = Some(v.into());
        self
    }
    #[doc = "Set the field `max_unavailable`.\n"]
    pub fn set_max_unavailable(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_unavailable = Some(v.into());
        self
    }
    #[doc = "Set the field `strategy`.\n"]
    pub fn set_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.strategy = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl
{
    type O = BlockAssignable<
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl
{}
impl BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl {
    pub fn build(
        self,
    ) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl {
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl {
            blue_green_settings: core::default::Default::default(),
            max_surge: core::default::Default::default(),
            max_unavailable: core::default::Default::default(),
            strategy: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElRef
    {
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `blue_green_settings` after provisioning.\n"]    pub fn blue_green_settings (& self) -> ListRef < DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElBlueGreenSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.blue_green_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_surge` after provisioning.\n"]
    pub fn max_surge(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_surge", self.base))
    }
    #[doc = "Get a reference to the value of field `max_unavailable` after provisioning.\n"]
    pub fn max_unavailable(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_unavailable", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `strategy` after provisioning.\n"]
    pub fn strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.strategy", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl { # [serde (skip_serializing_if = "Option::is_none")] boot_disk_kms_key : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] disk_size : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] disk_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] image_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] management : Option < ListField < DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl > > , # [serde (skip_serializing_if = "Option::is_none")] min_cpu_platform : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] oauth_scopes : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] service_account : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] shielded_instance_config : Option < ListField < DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] upgrade_settings : Option < ListField < DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl > > , }
impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl {
    #[doc = "Set the field `boot_disk_kms_key`.\n"]
    pub fn set_boot_disk_kms_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.boot_disk_kms_key = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_size`.\n"]
    pub fn set_disk_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_size = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_type`.\n"]
    pub fn set_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_type = Some(v.into());
        self
    }
    #[doc = "Set the field `image_type`.\n"]
    pub fn set_image_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_type = Some(v.into());
        self
    }
    #[doc = "Set the field `management`.\n"]
    pub fn set_management(
        mut self,
        v: impl Into<
            ListField<
                DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementEl,
            >,
        >,
    ) -> Self {
        self.management = Some(v.into());
        self
    }
    #[doc = "Set the field `min_cpu_platform`.\n"]
    pub fn set_min_cpu_platform(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_cpu_platform = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth_scopes`.\n"]
    pub fn set_oauth_scopes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.oauth_scopes = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        mut self,
        v : impl Into < ListField < DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigEl > >,
    ) -> Self {
        self.shielded_instance_config = Some(v.into());
        self
    }
    #[doc = "Set the field `upgrade_settings`.\n"]
    pub fn set_upgrade_settings(
        mut self,
        v: impl Into<
            ListField<
                DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsEl,
            >,
        >,
    ) -> Self {
        self.upgrade_settings = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl {
    type O = BlockAssignable<DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl {}
impl BuildDataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl {
    pub fn build(self) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl {
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl {
            boot_disk_kms_key: core::default::Default::default(),
            disk_size: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            image_type: core::default::Default::default(),
            management: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            oauth_scopes: core::default::Default::default(),
            service_account: core::default::Default::default(),
            shielded_instance_config: core::default::Default::default(),
            upgrade_settings: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElRef {
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `boot_disk_kms_key` after provisioning.\n"]
    pub fn boot_disk_kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_kms_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disk_size` after provisioning.\n"]
    pub fn disk_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\n"]
    pub fn disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }
    #[doc = "Get a reference to the value of field `image_type` after provisioning.\n"]
    pub fn image_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_type", self.base))
    }
    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(
        &self,
    ) -> ListRef<DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElManagementElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.management", self.base))
    }
    #[doc = "Get a reference to the value of field `min_cpu_platform` after provisioning.\n"]
    pub fn min_cpu_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_cpu_platform", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_scopes` after provisioning.\n"]
    pub fn oauth_scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.oauth_scopes", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]    pub fn shielded_instance_config (& self) -> ListRef < DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElShieldedInstanceConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.shielded_instance_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `upgrade_settings` after provisioning.\n"]
    pub fn upgrade_settings(
        &self,
    ) -> ListRef<
        DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElUpgradeSettingsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upgrade_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterClusterAutoscalingElResourceLimitsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type: Option<PrimField<String>>,
}
impl DataContainerClusterClusterAutoscalingElResourceLimitsEl {
    #[doc = "Set the field `maximum`.\n"]
    pub fn set_maximum(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum = Some(v.into());
        self
    }
    #[doc = "Set the field `minimum`.\n"]
    pub fn set_minimum(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minimum = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_type = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterClusterAutoscalingElResourceLimitsEl {
    type O = BlockAssignable<DataContainerClusterClusterAutoscalingElResourceLimitsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterClusterAutoscalingElResourceLimitsEl {}
impl BuildDataContainerClusterClusterAutoscalingElResourceLimitsEl {
    pub fn build(self) -> DataContainerClusterClusterAutoscalingElResourceLimitsEl {
        DataContainerClusterClusterAutoscalingElResourceLimitsEl {
            maximum: core::default::Default::default(),
            minimum: core::default::Default::default(),
            resource_type: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterClusterAutoscalingElResourceLimitsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterClusterAutoscalingElResourceLimitsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterClusterAutoscalingElResourceLimitsElRef {
        DataContainerClusterClusterAutoscalingElResourceLimitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterClusterAutoscalingElResourceLimitsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `maximum` after provisioning.\n"]
    pub fn maximum(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum", self.base))
    }
    #[doc = "Get a reference to the value of field `minimum` after provisioning.\n"]
    pub fn minimum(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterClusterAutoscalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_provisioning_defaults:
        Option<ListField<DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_provisioning_locations: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_profile: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_compute_class_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_limits: Option<ListField<DataContainerClusterClusterAutoscalingElResourceLimitsEl>>,
}
impl DataContainerClusterClusterAutoscalingEl {
    #[doc = "Set the field `auto_provisioning_defaults`.\n"]
    pub fn set_auto_provisioning_defaults(
        mut self,
        v: impl Into<ListField<DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsEl>>,
    ) -> Self {
        self.auto_provisioning_defaults = Some(v.into());
        self
    }
    #[doc = "Set the field `auto_provisioning_locations`.\n"]
    pub fn set_auto_provisioning_locations(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.auto_provisioning_locations = Some(v.into());
        self
    }
    #[doc = "Set the field `autoscaling_profile`.\n"]
    pub fn set_autoscaling_profile(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.autoscaling_profile = Some(v.into());
        self
    }
    #[doc = "Set the field `default_compute_class_enabled`.\n"]
    pub fn set_default_compute_class_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.default_compute_class_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_limits`.\n"]
    pub fn set_resource_limits(
        mut self,
        v: impl Into<ListField<DataContainerClusterClusterAutoscalingElResourceLimitsEl>>,
    ) -> Self {
        self.resource_limits = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterClusterAutoscalingEl {
    type O = BlockAssignable<DataContainerClusterClusterAutoscalingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterClusterAutoscalingEl {}
impl BuildDataContainerClusterClusterAutoscalingEl {
    pub fn build(self) -> DataContainerClusterClusterAutoscalingEl {
        DataContainerClusterClusterAutoscalingEl {
            auto_provisioning_defaults: core::default::Default::default(),
            auto_provisioning_locations: core::default::Default::default(),
            autoscaling_profile: core::default::Default::default(),
            default_compute_class_enabled: core::default::Default::default(),
            enabled: core::default::Default::default(),
            resource_limits: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterClusterAutoscalingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterClusterAutoscalingElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterClusterAutoscalingElRef {
        DataContainerClusterClusterAutoscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterClusterAutoscalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_provisioning_defaults` after provisioning.\n"]
    pub fn auto_provisioning_defaults(
        &self,
    ) -> ListRef<DataContainerClusterClusterAutoscalingElAutoProvisioningDefaultsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_provisioning_defaults", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `auto_provisioning_locations` after provisioning.\n"]
    pub fn auto_provisioning_locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_provisioning_locations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `autoscaling_profile` after provisioning.\n"]
    pub fn autoscaling_profile(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.autoscaling_profile", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_compute_class_enabled` after provisioning.\n"]
    pub fn default_compute_class_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_compute_class_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_limits` after provisioning.\n"]
    pub fn resource_limits(
        &self,
    ) -> ListRef<DataContainerClusterClusterAutoscalingElResourceLimitsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_limits", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterConfidentialNodesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential_instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterConfidentialNodesEl {
    #[doc = "Set the field `confidential_instance_type`.\n"]
    pub fn set_confidential_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.confidential_instance_type = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterConfidentialNodesEl {
    type O = BlockAssignable<DataContainerClusterConfidentialNodesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterConfidentialNodesEl {}
impl BuildDataContainerClusterConfidentialNodesEl {
    pub fn build(self) -> DataContainerClusterConfidentialNodesEl {
        DataContainerClusterConfidentialNodesEl {
            confidential_instance_type: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterConfidentialNodesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterConfidentialNodesElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterConfidentialNodesElRef {
        DataContainerClusterConfidentialNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterConfidentialNodesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `confidential_instance_type` after provisioning.\n"]
    pub fn confidential_instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.confidential_instance_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterControlPlaneEndpointsConfigElDnsEndpointConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_external_traffic: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_k8s_certs_via_dns: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_k8s_tokens_via_dns: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<PrimField<String>>,
}
impl DataContainerClusterControlPlaneEndpointsConfigElDnsEndpointConfigEl {
    #[doc = "Set the field `allow_external_traffic`.\n"]
    pub fn set_allow_external_traffic(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_external_traffic = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_k8s_certs_via_dns`.\n"]
    pub fn set_enable_k8s_certs_via_dns(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_k8s_certs_via_dns = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_k8s_tokens_via_dns`.\n"]
    pub fn set_enable_k8s_tokens_via_dns(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_k8s_tokens_via_dns = Some(v.into());
        self
    }
    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterControlPlaneEndpointsConfigElDnsEndpointConfigEl {
    type O = BlockAssignable<DataContainerClusterControlPlaneEndpointsConfigElDnsEndpointConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterControlPlaneEndpointsConfigElDnsEndpointConfigEl {}
impl BuildDataContainerClusterControlPlaneEndpointsConfigElDnsEndpointConfigEl {
    pub fn build(self) -> DataContainerClusterControlPlaneEndpointsConfigElDnsEndpointConfigEl {
        DataContainerClusterControlPlaneEndpointsConfigElDnsEndpointConfigEl {
            allow_external_traffic: core::default::Default::default(),
            enable_k8s_certs_via_dns: core::default::Default::default(),
            enable_k8s_tokens_via_dns: core::default::Default::default(),
            endpoint: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterControlPlaneEndpointsConfigElDnsEndpointConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterControlPlaneEndpointsConfigElDnsEndpointConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterControlPlaneEndpointsConfigElDnsEndpointConfigElRef {
        DataContainerClusterControlPlaneEndpointsConfigElDnsEndpointConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterControlPlaneEndpointsConfigElDnsEndpointConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_external_traffic` after provisioning.\n"]
    pub fn allow_external_traffic(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_external_traffic", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_k8s_certs_via_dns` after provisioning.\n"]
    pub fn enable_k8s_certs_via_dns(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_k8s_certs_via_dns", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_k8s_tokens_via_dns` after provisioning.\n"]
    pub fn enable_k8s_tokens_via_dns(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_k8s_tokens_via_dns", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterControlPlaneEndpointsConfigElIpEndpointsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterControlPlaneEndpointsConfigElIpEndpointsConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterControlPlaneEndpointsConfigElIpEndpointsConfigEl {
    type O = BlockAssignable<DataContainerClusterControlPlaneEndpointsConfigElIpEndpointsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterControlPlaneEndpointsConfigElIpEndpointsConfigEl {}
impl BuildDataContainerClusterControlPlaneEndpointsConfigElIpEndpointsConfigEl {
    pub fn build(self) -> DataContainerClusterControlPlaneEndpointsConfigElIpEndpointsConfigEl {
        DataContainerClusterControlPlaneEndpointsConfigElIpEndpointsConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterControlPlaneEndpointsConfigElIpEndpointsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterControlPlaneEndpointsConfigElIpEndpointsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterControlPlaneEndpointsConfigElIpEndpointsConfigElRef {
        DataContainerClusterControlPlaneEndpointsConfigElIpEndpointsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterControlPlaneEndpointsConfigElIpEndpointsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterControlPlaneEndpointsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_endpoint_config:
        Option<ListField<DataContainerClusterControlPlaneEndpointsConfigElDnsEndpointConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_endpoints_config:
        Option<ListField<DataContainerClusterControlPlaneEndpointsConfigElIpEndpointsConfigEl>>,
}
impl DataContainerClusterControlPlaneEndpointsConfigEl {
    #[doc = "Set the field `dns_endpoint_config`.\n"]
    pub fn set_dns_endpoint_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterControlPlaneEndpointsConfigElDnsEndpointConfigEl>>,
    ) -> Self {
        self.dns_endpoint_config = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_endpoints_config`.\n"]
    pub fn set_ip_endpoints_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterControlPlaneEndpointsConfigElIpEndpointsConfigEl>>,
    ) -> Self {
        self.ip_endpoints_config = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterControlPlaneEndpointsConfigEl {
    type O = BlockAssignable<DataContainerClusterControlPlaneEndpointsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterControlPlaneEndpointsConfigEl {}
impl BuildDataContainerClusterControlPlaneEndpointsConfigEl {
    pub fn build(self) -> DataContainerClusterControlPlaneEndpointsConfigEl {
        DataContainerClusterControlPlaneEndpointsConfigEl {
            dns_endpoint_config: core::default::Default::default(),
            ip_endpoints_config: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterControlPlaneEndpointsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterControlPlaneEndpointsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterControlPlaneEndpointsConfigElRef {
        DataContainerClusterControlPlaneEndpointsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterControlPlaneEndpointsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dns_endpoint_config` after provisioning.\n"]
    pub fn dns_endpoint_config(
        &self,
    ) -> ListRef<DataContainerClusterControlPlaneEndpointsConfigElDnsEndpointConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dns_endpoint_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ip_endpoints_config` after provisioning.\n"]
    pub fn ip_endpoints_config(
        &self,
    ) -> ListRef<DataContainerClusterControlPlaneEndpointsConfigElIpEndpointsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ip_endpoints_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterCostManagementConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterCostManagementConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterCostManagementConfigEl {
    type O = BlockAssignable<DataContainerClusterCostManagementConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterCostManagementConfigEl {}
impl BuildDataContainerClusterCostManagementConfigEl {
    pub fn build(self) -> DataContainerClusterCostManagementConfigEl {
        DataContainerClusterCostManagementConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterCostManagementConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterCostManagementConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterCostManagementConfigElRef {
        DataContainerClusterCostManagementConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterCostManagementConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterDatabaseEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl DataContainerClusterDatabaseEncryptionEl {
    #[doc = "Set the field `key_name`.\n"]
    pub fn set_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterDatabaseEncryptionEl {
    type O = BlockAssignable<DataContainerClusterDatabaseEncryptionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterDatabaseEncryptionEl {}
impl BuildDataContainerClusterDatabaseEncryptionEl {
    pub fn build(self) -> DataContainerClusterDatabaseEncryptionEl {
        DataContainerClusterDatabaseEncryptionEl {
            key_name: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterDatabaseEncryptionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterDatabaseEncryptionElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterDatabaseEncryptionElRef {
        DataContainerClusterDatabaseEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterDatabaseEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterDefaultSnatStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
}
impl DataContainerClusterDefaultSnatStatusEl {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterDefaultSnatStatusEl {
    type O = BlockAssignable<DataContainerClusterDefaultSnatStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterDefaultSnatStatusEl {}
impl BuildDataContainerClusterDefaultSnatStatusEl {
    pub fn build(self) -> DataContainerClusterDefaultSnatStatusEl {
        DataContainerClusterDefaultSnatStatusEl {
            disabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterDefaultSnatStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterDefaultSnatStatusElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterDefaultSnatStatusElRef {
        DataContainerClusterDefaultSnatStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterDefaultSnatStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterDnsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additive_vpc_scope_dns_domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_dns: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_dns_domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_dns_scope: Option<PrimField<String>>,
}
impl DataContainerClusterDnsConfigEl {
    #[doc = "Set the field `additive_vpc_scope_dns_domain`.\n"]
    pub fn set_additive_vpc_scope_dns_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.additive_vpc_scope_dns_domain = Some(v.into());
        self
    }
    #[doc = "Set the field `cluster_dns`.\n"]
    pub fn set_cluster_dns(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_dns = Some(v.into());
        self
    }
    #[doc = "Set the field `cluster_dns_domain`.\n"]
    pub fn set_cluster_dns_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_dns_domain = Some(v.into());
        self
    }
    #[doc = "Set the field `cluster_dns_scope`.\n"]
    pub fn set_cluster_dns_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_dns_scope = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterDnsConfigEl {
    type O = BlockAssignable<DataContainerClusterDnsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterDnsConfigEl {}
impl BuildDataContainerClusterDnsConfigEl {
    pub fn build(self) -> DataContainerClusterDnsConfigEl {
        DataContainerClusterDnsConfigEl {
            additive_vpc_scope_dns_domain: core::default::Default::default(),
            cluster_dns: core::default::Default::default(),
            cluster_dns_domain: core::default::Default::default(),
            cluster_dns_scope: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterDnsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterDnsConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterDnsConfigElRef {
        DataContainerClusterDnsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterDnsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additive_vpc_scope_dns_domain` after provisioning.\n"]
    pub fn additive_vpc_scope_dns_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.additive_vpc_scope_dns_domain", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_dns` after provisioning.\n"]
    pub fn cluster_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_dns", self.base))
    }
    #[doc = "Get a reference to the value of field `cluster_dns_domain` after provisioning.\n"]
    pub fn cluster_dns_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_dns_domain", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_dns_scope` after provisioning.\n"]
    pub fn cluster_dns_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_dns_scope", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterEnableK8sBetaApisEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled_apis: Option<SetField<PrimField<String>>>,
}
impl DataContainerClusterEnableK8sBetaApisEl {
    #[doc = "Set the field `enabled_apis`.\n"]
    pub fn set_enabled_apis(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.enabled_apis = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterEnableK8sBetaApisEl {
    type O = BlockAssignable<DataContainerClusterEnableK8sBetaApisEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterEnableK8sBetaApisEl {}
impl BuildDataContainerClusterEnableK8sBetaApisEl {
    pub fn build(self) -> DataContainerClusterEnableK8sBetaApisEl {
        DataContainerClusterEnableK8sBetaApisEl {
            enabled_apis: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterEnableK8sBetaApisElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterEnableK8sBetaApisElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterEnableK8sBetaApisElRef {
        DataContainerClusterEnableK8sBetaApisElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterEnableK8sBetaApisElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled_apis` after provisioning.\n"]
    pub fn enabled_apis(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.enabled_apis", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterEnterpriseConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_tier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_tier: Option<PrimField<String>>,
}
impl DataContainerClusterEnterpriseConfigEl {
    #[doc = "Set the field `cluster_tier`.\n"]
    pub fn set_cluster_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_tier = Some(v.into());
        self
    }
    #[doc = "Set the field `desired_tier`.\n"]
    pub fn set_desired_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.desired_tier = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterEnterpriseConfigEl {
    type O = BlockAssignable<DataContainerClusterEnterpriseConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterEnterpriseConfigEl {}
impl BuildDataContainerClusterEnterpriseConfigEl {
    pub fn build(self) -> DataContainerClusterEnterpriseConfigEl {
        DataContainerClusterEnterpriseConfigEl {
            cluster_tier: core::default::Default::default(),
            desired_tier: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterEnterpriseConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterEnterpriseConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterEnterpriseConfigElRef {
        DataContainerClusterEnterpriseConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterEnterpriseConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster_tier` after provisioning.\n"]
    pub fn cluster_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_tier", self.base))
    }
    #[doc = "Get a reference to the value of field `desired_tier` after provisioning.\n"]
    pub fn desired_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.desired_tier", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterFleetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    membership: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    membership_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    membership_location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    membership_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pre_registered: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
impl DataContainerClusterFleetEl {
    #[doc = "Set the field `membership`.\n"]
    pub fn set_membership(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.membership = Some(v.into());
        self
    }
    #[doc = "Set the field `membership_id`.\n"]
    pub fn set_membership_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.membership_id = Some(v.into());
        self
    }
    #[doc = "Set the field `membership_location`.\n"]
    pub fn set_membership_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.membership_location = Some(v.into());
        self
    }
    #[doc = "Set the field `membership_type`.\n"]
    pub fn set_membership_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.membership_type = Some(v.into());
        self
    }
    #[doc = "Set the field `pre_registered`.\n"]
    pub fn set_pre_registered(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.pre_registered = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterFleetEl {
    type O = BlockAssignable<DataContainerClusterFleetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterFleetEl {}
impl BuildDataContainerClusterFleetEl {
    pub fn build(self) -> DataContainerClusterFleetEl {
        DataContainerClusterFleetEl {
            membership: core::default::Default::default(),
            membership_id: core::default::Default::default(),
            membership_location: core::default::Default::default(),
            membership_type: core::default::Default::default(),
            pre_registered: core::default::Default::default(),
            project: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterFleetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterFleetElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterFleetElRef {
        DataContainerClusterFleetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterFleetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `membership` after provisioning.\n"]
    pub fn membership(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.membership", self.base))
    }
    #[doc = "Get a reference to the value of field `membership_id` after provisioning.\n"]
    pub fn membership_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.membership_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `membership_location` after provisioning.\n"]
    pub fn membership_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.membership_location", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `membership_type` after provisioning.\n"]
    pub fn membership_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.membership_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pre_registered` after provisioning.\n"]
    pub fn pre_registered(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pre_registered", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterGatewayApiConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<PrimField<String>>,
}
impl DataContainerClusterGatewayApiConfigEl {
    #[doc = "Set the field `channel`.\n"]
    pub fn set_channel(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.channel = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterGatewayApiConfigEl {
    type O = BlockAssignable<DataContainerClusterGatewayApiConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterGatewayApiConfigEl {}
impl BuildDataContainerClusterGatewayApiConfigEl {
    pub fn build(self) -> DataContainerClusterGatewayApiConfigEl {
        DataContainerClusterGatewayApiConfigEl {
            channel: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterGatewayApiConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterGatewayApiConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterGatewayApiConfigElRef {
        DataContainerClusterGatewayApiConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterGatewayApiConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `channel` after provisioning.\n"]
    pub fn channel(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterGkeAutoUpgradeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    patch_mode: Option<PrimField<String>>,
}
impl DataContainerClusterGkeAutoUpgradeConfigEl {
    #[doc = "Set the field `patch_mode`.\n"]
    pub fn set_patch_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.patch_mode = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterGkeAutoUpgradeConfigEl {
    type O = BlockAssignable<DataContainerClusterGkeAutoUpgradeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterGkeAutoUpgradeConfigEl {}
impl BuildDataContainerClusterGkeAutoUpgradeConfigEl {
    pub fn build(self) -> DataContainerClusterGkeAutoUpgradeConfigEl {
        DataContainerClusterGkeAutoUpgradeConfigEl {
            patch_mode: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterGkeAutoUpgradeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterGkeAutoUpgradeConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterGkeAutoUpgradeConfigElRef {
        DataContainerClusterGkeAutoUpgradeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterGkeAutoUpgradeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `patch_mode` after provisioning.\n"]
    pub fn patch_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.patch_mode", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterIdentityServiceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterIdentityServiceConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterIdentityServiceConfigEl {
    type O = BlockAssignable<DataContainerClusterIdentityServiceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterIdentityServiceConfigEl {}
impl BuildDataContainerClusterIdentityServiceConfigEl {
    pub fn build(self) -> DataContainerClusterIdentityServiceConfigEl {
        DataContainerClusterIdentityServiceConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterIdentityServiceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterIdentityServiceConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterIdentityServiceConfigElRef {
        DataContainerClusterIdentityServiceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterIdentityServiceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterIpAllocationPolicyElAdditionalIpRangesConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_ipv4_range_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
}
impl DataContainerClusterIpAllocationPolicyElAdditionalIpRangesConfigEl {
    #[doc = "Set the field `pod_ipv4_range_names`.\n"]
    pub fn set_pod_ipv4_range_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.pod_ipv4_range_names = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\n"]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterIpAllocationPolicyElAdditionalIpRangesConfigEl {
    type O = BlockAssignable<DataContainerClusterIpAllocationPolicyElAdditionalIpRangesConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterIpAllocationPolicyElAdditionalIpRangesConfigEl {}
impl BuildDataContainerClusterIpAllocationPolicyElAdditionalIpRangesConfigEl {
    pub fn build(self) -> DataContainerClusterIpAllocationPolicyElAdditionalIpRangesConfigEl {
        DataContainerClusterIpAllocationPolicyElAdditionalIpRangesConfigEl {
            pod_ipv4_range_names: core::default::Default::default(),
            status: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterIpAllocationPolicyElAdditionalIpRangesConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterIpAllocationPolicyElAdditionalIpRangesConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterIpAllocationPolicyElAdditionalIpRangesConfigElRef {
        DataContainerClusterIpAllocationPolicyElAdditionalIpRangesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterIpAllocationPolicyElAdditionalIpRangesConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pod_ipv4_range_names` after provisioning.\n"]
    pub fn pod_ipv4_range_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pod_ipv4_range_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\n"]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_range_names: Option<SetField<PrimField<String>>>,
}
impl DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl {
    #[doc = "Set the field `pod_range_names`.\n"]
    pub fn set_pod_range_names(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.pod_range_names = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl {
    type O = BlockAssignable<DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl {}
impl BuildDataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl {
    pub fn build(self) -> DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl {
        DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl {
            pod_range_names: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigElRef {
        DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pod_range_names` after provisioning.\n"]
    pub fn pod_range_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.pod_range_names", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterIpAllocationPolicyElAutoIpamConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterIpAllocationPolicyElAutoIpamConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterIpAllocationPolicyElAutoIpamConfigEl {
    type O = BlockAssignable<DataContainerClusterIpAllocationPolicyElAutoIpamConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterIpAllocationPolicyElAutoIpamConfigEl {}
impl BuildDataContainerClusterIpAllocationPolicyElAutoIpamConfigEl {
    pub fn build(self) -> DataContainerClusterIpAllocationPolicyElAutoIpamConfigEl {
        DataContainerClusterIpAllocationPolicyElAutoIpamConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterIpAllocationPolicyElAutoIpamConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterIpAllocationPolicyElAutoIpamConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterIpAllocationPolicyElAutoIpamConfigElRef {
        DataContainerClusterIpAllocationPolicyElAutoIpamConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterIpAllocationPolicyElAutoIpamConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterIpAllocationPolicyElNetworkTierConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network_tier: Option<PrimField<String>>,
}
impl DataContainerClusterIpAllocationPolicyElNetworkTierConfigEl {
    #[doc = "Set the field `network_tier`.\n"]
    pub fn set_network_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_tier = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterIpAllocationPolicyElNetworkTierConfigEl {
    type O = BlockAssignable<DataContainerClusterIpAllocationPolicyElNetworkTierConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterIpAllocationPolicyElNetworkTierConfigEl {}
impl BuildDataContainerClusterIpAllocationPolicyElNetworkTierConfigEl {
    pub fn build(self) -> DataContainerClusterIpAllocationPolicyElNetworkTierConfigEl {
        DataContainerClusterIpAllocationPolicyElNetworkTierConfigEl {
            network_tier: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterIpAllocationPolicyElNetworkTierConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterIpAllocationPolicyElNetworkTierConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterIpAllocationPolicyElNetworkTierConfigElRef {
        DataContainerClusterIpAllocationPolicyElNetworkTierConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterIpAllocationPolicyElNetworkTierConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network_tier` after provisioning.\n"]
    pub fn network_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_tier", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
}
impl DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl {
    type O = BlockAssignable<DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl {}
impl BuildDataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl {
    pub fn build(self) -> DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl {
        DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl {
            disabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigElRef {
        DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterIpAllocationPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_ip_ranges_config:
        Option<ListField<DataContainerClusterIpAllocationPolicyElAdditionalIpRangesConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_pod_ranges_config:
        Option<ListField<DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_ipam_config: Option<ListField<DataContainerClusterIpAllocationPolicyElAutoIpamConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_ipv4_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_secondary_range_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_tier_config:
        Option<ListField<DataContainerClusterIpAllocationPolicyElNetworkTierConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_cidr_overprovision_config:
        Option<ListField<DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    services_ipv4_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    services_secondary_range_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stack_type: Option<PrimField<String>>,
}
impl DataContainerClusterIpAllocationPolicyEl {
    #[doc = "Set the field `additional_ip_ranges_config`.\n"]
    pub fn set_additional_ip_ranges_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterIpAllocationPolicyElAdditionalIpRangesConfigEl>>,
    ) -> Self {
        self.additional_ip_ranges_config = Some(v.into());
        self
    }
    #[doc = "Set the field `additional_pod_ranges_config`.\n"]
    pub fn set_additional_pod_ranges_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigEl>>,
    ) -> Self {
        self.additional_pod_ranges_config = Some(v.into());
        self
    }
    #[doc = "Set the field `auto_ipam_config`.\n"]
    pub fn set_auto_ipam_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterIpAllocationPolicyElAutoIpamConfigEl>>,
    ) -> Self {
        self.auto_ipam_config = Some(v.into());
        self
    }
    #[doc = "Set the field `cluster_ipv4_cidr_block`.\n"]
    pub fn set_cluster_ipv4_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_ipv4_cidr_block = Some(v.into());
        self
    }
    #[doc = "Set the field `cluster_secondary_range_name`.\n"]
    pub fn set_cluster_secondary_range_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_secondary_range_name = Some(v.into());
        self
    }
    #[doc = "Set the field `network_tier_config`.\n"]
    pub fn set_network_tier_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterIpAllocationPolicyElNetworkTierConfigEl>>,
    ) -> Self {
        self.network_tier_config = Some(v.into());
        self
    }
    #[doc = "Set the field `pod_cidr_overprovision_config`.\n"]
    pub fn set_pod_cidr_overprovision_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigEl>>,
    ) -> Self {
        self.pod_cidr_overprovision_config = Some(v.into());
        self
    }
    #[doc = "Set the field `services_ipv4_cidr_block`.\n"]
    pub fn set_services_ipv4_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.services_ipv4_cidr_block = Some(v.into());
        self
    }
    #[doc = "Set the field `services_secondary_range_name`.\n"]
    pub fn set_services_secondary_range_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.services_secondary_range_name = Some(v.into());
        self
    }
    #[doc = "Set the field `stack_type`.\n"]
    pub fn set_stack_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stack_type = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterIpAllocationPolicyEl {
    type O = BlockAssignable<DataContainerClusterIpAllocationPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterIpAllocationPolicyEl {}
impl BuildDataContainerClusterIpAllocationPolicyEl {
    pub fn build(self) -> DataContainerClusterIpAllocationPolicyEl {
        DataContainerClusterIpAllocationPolicyEl {
            additional_ip_ranges_config: core::default::Default::default(),
            additional_pod_ranges_config: core::default::Default::default(),
            auto_ipam_config: core::default::Default::default(),
            cluster_ipv4_cidr_block: core::default::Default::default(),
            cluster_secondary_range_name: core::default::Default::default(),
            network_tier_config: core::default::Default::default(),
            pod_cidr_overprovision_config: core::default::Default::default(),
            services_ipv4_cidr_block: core::default::Default::default(),
            services_secondary_range_name: core::default::Default::default(),
            stack_type: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterIpAllocationPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterIpAllocationPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterIpAllocationPolicyElRef {
        DataContainerClusterIpAllocationPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterIpAllocationPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additional_ip_ranges_config` after provisioning.\n"]
    pub fn additional_ip_ranges_config(
        &self,
    ) -> ListRef<DataContainerClusterIpAllocationPolicyElAdditionalIpRangesConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_ip_ranges_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `additional_pod_ranges_config` after provisioning.\n"]
    pub fn additional_pod_ranges_config(
        &self,
    ) -> ListRef<DataContainerClusterIpAllocationPolicyElAdditionalPodRangesConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_pod_ranges_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `auto_ipam_config` after provisioning.\n"]
    pub fn auto_ipam_config(
        &self,
    ) -> ListRef<DataContainerClusterIpAllocationPolicyElAutoIpamConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_ipam_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_ipv4_cidr_block` after provisioning.\n"]
    pub fn cluster_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_ipv4_cidr_block", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_secondary_range_name` after provisioning.\n"]
    pub fn cluster_secondary_range_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_secondary_range_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_tier_config` after provisioning.\n"]
    pub fn network_tier_config(
        &self,
    ) -> ListRef<DataContainerClusterIpAllocationPolicyElNetworkTierConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_tier_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pod_cidr_overprovision_config` after provisioning.\n"]
    pub fn pod_cidr_overprovision_config(
        &self,
    ) -> ListRef<DataContainerClusterIpAllocationPolicyElPodCidrOverprovisionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pod_cidr_overprovision_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `services_ipv4_cidr_block` after provisioning.\n"]
    pub fn services_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.services_ipv4_cidr_block", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `services_secondary_range_name` after provisioning.\n"]
    pub fn services_secondary_range_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.services_secondary_range_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `stack_type` after provisioning.\n"]
    pub fn stack_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterLoggingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_components: Option<ListField<PrimField<String>>>,
}
impl DataContainerClusterLoggingConfigEl {
    #[doc = "Set the field `enable_components`.\n"]
    pub fn set_enable_components(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.enable_components = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterLoggingConfigEl {
    type O = BlockAssignable<DataContainerClusterLoggingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterLoggingConfigEl {}
impl BuildDataContainerClusterLoggingConfigEl {
    pub fn build(self) -> DataContainerClusterLoggingConfigEl {
        DataContainerClusterLoggingConfigEl {
            enable_components: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterLoggingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterLoggingConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterLoggingConfigElRef {
        DataContainerClusterLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterLoggingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_components` after provisioning.\n"]
    pub fn enable_components(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enable_components", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl {
    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.duration = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl {
    type O = BlockAssignable<DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl {}
impl BuildDataContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl {
    pub fn build(self) -> DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl {
        DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl {
            duration: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowElRef {
        DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterMaintenancePolicyElDisruptionBudgetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    last_disruption_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_minor_version_disruption_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minor_version_disruption_interval: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    patch_version_disruption_interval: Option<PrimField<String>>,
}
impl DataContainerClusterMaintenancePolicyElDisruptionBudgetEl {
    #[doc = "Set the field `last_disruption_time`.\n"]
    pub fn set_last_disruption_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_disruption_time = Some(v.into());
        self
    }
    #[doc = "Set the field `last_minor_version_disruption_time`.\n"]
    pub fn set_last_minor_version_disruption_time(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.last_minor_version_disruption_time = Some(v.into());
        self
    }
    #[doc = "Set the field `minor_version_disruption_interval`.\n"]
    pub fn set_minor_version_disruption_interval(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.minor_version_disruption_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `patch_version_disruption_interval`.\n"]
    pub fn set_patch_version_disruption_interval(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.patch_version_disruption_interval = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterMaintenancePolicyElDisruptionBudgetEl {
    type O = BlockAssignable<DataContainerClusterMaintenancePolicyElDisruptionBudgetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterMaintenancePolicyElDisruptionBudgetEl {}
impl BuildDataContainerClusterMaintenancePolicyElDisruptionBudgetEl {
    pub fn build(self) -> DataContainerClusterMaintenancePolicyElDisruptionBudgetEl {
        DataContainerClusterMaintenancePolicyElDisruptionBudgetEl {
            last_disruption_time: core::default::Default::default(),
            last_minor_version_disruption_time: core::default::Default::default(),
            minor_version_disruption_interval: core::default::Default::default(),
            patch_version_disruption_interval: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterMaintenancePolicyElDisruptionBudgetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterMaintenancePolicyElDisruptionBudgetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterMaintenancePolicyElDisruptionBudgetElRef {
        DataContainerClusterMaintenancePolicyElDisruptionBudgetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterMaintenancePolicyElDisruptionBudgetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `last_disruption_time` after provisioning.\n"]
    pub fn last_disruption_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_disruption_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `last_minor_version_disruption_time` after provisioning.\n"]
    pub fn last_minor_version_disruption_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_minor_version_disruption_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `minor_version_disruption_interval` after provisioning.\n"]
    pub fn minor_version_disruption_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minor_version_disruption_interval", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `patch_version_disruption_interval` after provisioning.\n"]
    pub fn patch_version_disruption_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.patch_version_disruption_interval", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
}
impl DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl {
    #[doc = "Set the field `end_time_behavior`.\n"]
    pub fn set_end_time_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time_behavior = Some(v.into());
        self
    }
    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl
{
    type O = BlockAssignable<
        DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl {}
impl BuildDataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl {
    pub fn build(
        self,
    ) -> DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl {
        DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl {
            end_time_behavior: core::default::Default::default(),
            scope: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsElRef {
        DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_time_behavior` after provisioning.\n"]
    pub fn end_time_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.end_time_behavior", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterMaintenancePolicyElMaintenanceExclusionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusion_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusion_options: Option<
        ListField<DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl DataContainerClusterMaintenancePolicyElMaintenanceExclusionEl {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `exclusion_name`.\n"]
    pub fn set_exclusion_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exclusion_name = Some(v.into());
        self
    }
    #[doc = "Set the field `exclusion_options`.\n"]
    pub fn set_exclusion_options(
        mut self,
        v: impl Into<
            ListField<
                DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsEl,
            >,
        >,
    ) -> Self {
        self.exclusion_options = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterMaintenancePolicyElMaintenanceExclusionEl {
    type O = BlockAssignable<DataContainerClusterMaintenancePolicyElMaintenanceExclusionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterMaintenancePolicyElMaintenanceExclusionEl {}
impl BuildDataContainerClusterMaintenancePolicyElMaintenanceExclusionEl {
    pub fn build(self) -> DataContainerClusterMaintenancePolicyElMaintenanceExclusionEl {
        DataContainerClusterMaintenancePolicyElMaintenanceExclusionEl {
            end_time: core::default::Default::default(),
            exclusion_name: core::default::Default::default(),
            exclusion_options: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterMaintenancePolicyElMaintenanceExclusionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterMaintenancePolicyElMaintenanceExclusionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterMaintenancePolicyElMaintenanceExclusionElRef {
        DataContainerClusterMaintenancePolicyElMaintenanceExclusionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterMaintenancePolicyElMaintenanceExclusionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `exclusion_name` after provisioning.\n"]
    pub fn exclusion_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exclusion_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exclusion_options` after provisioning.\n"]
    pub fn exclusion_options(
        &self,
    ) -> ListRef<DataContainerClusterMaintenancePolicyElMaintenanceExclusionElExclusionOptionsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclusion_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterMaintenancePolicyElRecurringWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recurrence: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl DataContainerClusterMaintenancePolicyElRecurringWindowEl {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `recurrence`.\n"]
    pub fn set_recurrence(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.recurrence = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterMaintenancePolicyElRecurringWindowEl {
    type O = BlockAssignable<DataContainerClusterMaintenancePolicyElRecurringWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterMaintenancePolicyElRecurringWindowEl {}
impl BuildDataContainerClusterMaintenancePolicyElRecurringWindowEl {
    pub fn build(self) -> DataContainerClusterMaintenancePolicyElRecurringWindowEl {
        DataContainerClusterMaintenancePolicyElRecurringWindowEl {
            end_time: core::default::Default::default(),
            recurrence: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterMaintenancePolicyElRecurringWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterMaintenancePolicyElRecurringWindowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterMaintenancePolicyElRecurringWindowElRef {
        DataContainerClusterMaintenancePolicyElRecurringWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterMaintenancePolicyElRecurringWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `recurrence` after provisioning.\n"]
    pub fn recurrence(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.recurrence", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterMaintenancePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    daily_maintenance_window:
        Option<ListField<DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disruption_budget: Option<ListField<DataContainerClusterMaintenancePolicyElDisruptionBudgetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_exclusion:
        Option<SetField<DataContainerClusterMaintenancePolicyElMaintenanceExclusionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recurring_window: Option<ListField<DataContainerClusterMaintenancePolicyElRecurringWindowEl>>,
}
impl DataContainerClusterMaintenancePolicyEl {
    #[doc = "Set the field `daily_maintenance_window`.\n"]
    pub fn set_daily_maintenance_window(
        mut self,
        v: impl Into<ListField<DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowEl>>,
    ) -> Self {
        self.daily_maintenance_window = Some(v.into());
        self
    }
    #[doc = "Set the field `disruption_budget`.\n"]
    pub fn set_disruption_budget(
        mut self,
        v: impl Into<ListField<DataContainerClusterMaintenancePolicyElDisruptionBudgetEl>>,
    ) -> Self {
        self.disruption_budget = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_exclusion`.\n"]
    pub fn set_maintenance_exclusion(
        mut self,
        v: impl Into<SetField<DataContainerClusterMaintenancePolicyElMaintenanceExclusionEl>>,
    ) -> Self {
        self.maintenance_exclusion = Some(v.into());
        self
    }
    #[doc = "Set the field `recurring_window`.\n"]
    pub fn set_recurring_window(
        mut self,
        v: impl Into<ListField<DataContainerClusterMaintenancePolicyElRecurringWindowEl>>,
    ) -> Self {
        self.recurring_window = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterMaintenancePolicyEl {
    type O = BlockAssignable<DataContainerClusterMaintenancePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterMaintenancePolicyEl {}
impl BuildDataContainerClusterMaintenancePolicyEl {
    pub fn build(self) -> DataContainerClusterMaintenancePolicyEl {
        DataContainerClusterMaintenancePolicyEl {
            daily_maintenance_window: core::default::Default::default(),
            disruption_budget: core::default::Default::default(),
            maintenance_exclusion: core::default::Default::default(),
            recurring_window: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterMaintenancePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterMaintenancePolicyElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterMaintenancePolicyElRef {
        DataContainerClusterMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterMaintenancePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `daily_maintenance_window` after provisioning.\n"]
    pub fn daily_maintenance_window(
        &self,
    ) -> ListRef<DataContainerClusterMaintenancePolicyElDailyMaintenanceWindowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.daily_maintenance_window", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disruption_budget` after provisioning.\n"]
    pub fn disruption_budget(
        &self,
    ) -> ListRef<DataContainerClusterMaintenancePolicyElDisruptionBudgetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disruption_budget", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_exclusion` after provisioning.\n"]
    pub fn maintenance_exclusion(
        &self,
    ) -> SetRef<DataContainerClusterMaintenancePolicyElMaintenanceExclusionElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.maintenance_exclusion", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `recurring_window` after provisioning.\n"]
    pub fn recurring_window(
        &self,
    ) -> ListRef<DataContainerClusterMaintenancePolicyElRecurringWindowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.recurring_window", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterMasterAuthElClientCertificateConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    issue_client_certificate: Option<PrimField<bool>>,
}
impl DataContainerClusterMasterAuthElClientCertificateConfigEl {
    #[doc = "Set the field `issue_client_certificate`.\n"]
    pub fn set_issue_client_certificate(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.issue_client_certificate = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterMasterAuthElClientCertificateConfigEl {
    type O = BlockAssignable<DataContainerClusterMasterAuthElClientCertificateConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterMasterAuthElClientCertificateConfigEl {}
impl BuildDataContainerClusterMasterAuthElClientCertificateConfigEl {
    pub fn build(self) -> DataContainerClusterMasterAuthElClientCertificateConfigEl {
        DataContainerClusterMasterAuthElClientCertificateConfigEl {
            issue_client_certificate: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterMasterAuthElClientCertificateConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterMasterAuthElClientCertificateConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterMasterAuthElClientCertificateConfigElRef {
        DataContainerClusterMasterAuthElClientCertificateConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterMasterAuthElClientCertificateConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `issue_client_certificate` after provisioning.\n"]
    pub fn issue_client_certificate(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.issue_client_certificate", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterMasterAuthEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_certificate_config:
        Option<ListField<DataContainerClusterMasterAuthElClientCertificateConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_ca_certificate: Option<PrimField<String>>,
}
impl DataContainerClusterMasterAuthEl {
    #[doc = "Set the field `client_certificate`.\n"]
    pub fn set_client_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_certificate = Some(v.into());
        self
    }
    #[doc = "Set the field `client_certificate_config`.\n"]
    pub fn set_client_certificate_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterMasterAuthElClientCertificateConfigEl>>,
    ) -> Self {
        self.client_certificate_config = Some(v.into());
        self
    }
    #[doc = "Set the field `client_key`.\n"]
    pub fn set_client_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_key = Some(v.into());
        self
    }
    #[doc = "Set the field `cluster_ca_certificate`.\n"]
    pub fn set_cluster_ca_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_ca_certificate = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterMasterAuthEl {
    type O = BlockAssignable<DataContainerClusterMasterAuthEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterMasterAuthEl {}
impl BuildDataContainerClusterMasterAuthEl {
    pub fn build(self) -> DataContainerClusterMasterAuthEl {
        DataContainerClusterMasterAuthEl {
            client_certificate: core::default::Default::default(),
            client_certificate_config: core::default::Default::default(),
            client_key: core::default::Default::default(),
            cluster_ca_certificate: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterMasterAuthElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterMasterAuthElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterMasterAuthElRef {
        DataContainerClusterMasterAuthElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterMasterAuthElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_certificate` after provisioning.\n"]
    pub fn client_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_certificate_config` after provisioning.\n"]
    pub fn client_certificate_config(
        &self,
    ) -> ListRef<DataContainerClusterMasterAuthElClientCertificateConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_certificate_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_key` after provisioning.\n"]
    pub fn client_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.base))
    }
    #[doc = "Get a reference to the value of field `cluster_ca_certificate` after provisioning.\n"]
    pub fn cluster_ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_ca_certificate", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
}
impl DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl {
    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cidr_block = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl {
    type O = BlockAssignable<DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl {}
impl BuildDataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl {
    pub fn build(self) -> DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl {
        DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl {
            cidr_block: core::default::Default::default(),
            display_name: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksElRef {
        DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterMasterAuthorizedNetworksConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_blocks: Option<SetField<DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_public_cidrs_access_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_endpoint_enforcement_enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterMasterAuthorizedNetworksConfigEl {
    #[doc = "Set the field `cidr_blocks`.\n"]
    pub fn set_cidr_blocks(
        mut self,
        v: impl Into<SetField<DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksEl>>,
    ) -> Self {
        self.cidr_blocks = Some(v.into());
        self
    }
    #[doc = "Set the field `gcp_public_cidrs_access_enabled`.\n"]
    pub fn set_gcp_public_cidrs_access_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.gcp_public_cidrs_access_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `private_endpoint_enforcement_enabled`.\n"]
    pub fn set_private_endpoint_enforcement_enabled(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.private_endpoint_enforcement_enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterMasterAuthorizedNetworksConfigEl {
    type O = BlockAssignable<DataContainerClusterMasterAuthorizedNetworksConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterMasterAuthorizedNetworksConfigEl {}
impl BuildDataContainerClusterMasterAuthorizedNetworksConfigEl {
    pub fn build(self) -> DataContainerClusterMasterAuthorizedNetworksConfigEl {
        DataContainerClusterMasterAuthorizedNetworksConfigEl {
            cidr_blocks: core::default::Default::default(),
            gcp_public_cidrs_access_enabled: core::default::Default::default(),
            private_endpoint_enforcement_enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterMasterAuthorizedNetworksConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterMasterAuthorizedNetworksConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterMasterAuthorizedNetworksConfigElRef {
        DataContainerClusterMasterAuthorizedNetworksConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterMasterAuthorizedNetworksConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cidr_blocks` after provisioning.\n"]
    pub fn cidr_blocks(
        &self,
    ) -> SetRef<DataContainerClusterMasterAuthorizedNetworksConfigElCidrBlocksElRef> {
        SetRef::new(self.shared().clone(), format!("{}.cidr_blocks", self.base))
    }
    #[doc = "Get a reference to the value of field `gcp_public_cidrs_access_enabled` after provisioning.\n"]
    pub fn gcp_public_cidrs_access_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_public_cidrs_access_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `private_endpoint_enforcement_enabled` after provisioning.\n"]
    pub fn private_endpoint_enforcement_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_endpoint_enforcement_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterMeshCertificatesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_certificates: Option<PrimField<bool>>,
}
impl DataContainerClusterMeshCertificatesEl {
    #[doc = "Set the field `enable_certificates`.\n"]
    pub fn set_enable_certificates(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_certificates = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterMeshCertificatesEl {
    type O = BlockAssignable<DataContainerClusterMeshCertificatesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterMeshCertificatesEl {}
impl BuildDataContainerClusterMeshCertificatesEl {
    pub fn build(self) -> DataContainerClusterMeshCertificatesEl {
        DataContainerClusterMeshCertificatesEl {
            enable_certificates: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterMeshCertificatesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterMeshCertificatesElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterMeshCertificatesElRef {
        DataContainerClusterMeshCertificatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterMeshCertificatesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_certificates` after provisioning.\n"]
    pub fn enable_certificates(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_certificates", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_metrics: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_relay: Option<PrimField<bool>>,
}
impl DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl {
    #[doc = "Set the field `enable_metrics`.\n"]
    pub fn set_enable_metrics(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_metrics = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_relay`.\n"]
    pub fn set_enable_relay(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_relay = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl
{
    type O = BlockAssignable<
        DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl {}
impl BuildDataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl {
    pub fn build(
        self,
    ) -> DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl {
        DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl {
            enable_metrics: core::default::Default::default(),
            enable_relay: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigElRef {
        DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_metrics` after provisioning.\n"]
    pub fn enable_metrics(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_metrics", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_relay` after provisioning.\n"]
    pub fn enable_relay(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_relay", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterMonitoringConfigElManagedPrometheusElAutoMonitoringConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
}
impl DataContainerClusterMonitoringConfigElManagedPrometheusElAutoMonitoringConfigEl {
    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterMonitoringConfigElManagedPrometheusElAutoMonitoringConfigEl
{
    type O = BlockAssignable<
        DataContainerClusterMonitoringConfigElManagedPrometheusElAutoMonitoringConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterMonitoringConfigElManagedPrometheusElAutoMonitoringConfigEl {}
impl BuildDataContainerClusterMonitoringConfigElManagedPrometheusElAutoMonitoringConfigEl {
    pub fn build(
        self,
    ) -> DataContainerClusterMonitoringConfigElManagedPrometheusElAutoMonitoringConfigEl {
        DataContainerClusterMonitoringConfigElManagedPrometheusElAutoMonitoringConfigEl {
            scope: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterMonitoringConfigElManagedPrometheusElAutoMonitoringConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterMonitoringConfigElManagedPrometheusElAutoMonitoringConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterMonitoringConfigElManagedPrometheusElAutoMonitoringConfigElRef {
        DataContainerClusterMonitoringConfigElManagedPrometheusElAutoMonitoringConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterMonitoringConfigElManagedPrometheusElAutoMonitoringConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterMonitoringConfigElManagedPrometheusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_monitoring_config: Option<
        ListField<DataContainerClusterMonitoringConfigElManagedPrometheusElAutoMonitoringConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterMonitoringConfigElManagedPrometheusEl {
    #[doc = "Set the field `auto_monitoring_config`.\n"]
    pub fn set_auto_monitoring_config(
        mut self,
        v: impl Into<
            ListField<
                DataContainerClusterMonitoringConfigElManagedPrometheusElAutoMonitoringConfigEl,
            >,
        >,
    ) -> Self {
        self.auto_monitoring_config = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterMonitoringConfigElManagedPrometheusEl {
    type O = BlockAssignable<DataContainerClusterMonitoringConfigElManagedPrometheusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterMonitoringConfigElManagedPrometheusEl {}
impl BuildDataContainerClusterMonitoringConfigElManagedPrometheusEl {
    pub fn build(self) -> DataContainerClusterMonitoringConfigElManagedPrometheusEl {
        DataContainerClusterMonitoringConfigElManagedPrometheusEl {
            auto_monitoring_config: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterMonitoringConfigElManagedPrometheusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterMonitoringConfigElManagedPrometheusElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterMonitoringConfigElManagedPrometheusElRef {
        DataContainerClusterMonitoringConfigElManagedPrometheusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterMonitoringConfigElManagedPrometheusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_monitoring_config` after provisioning.\n"]
    pub fn auto_monitoring_config(
        &self,
    ) -> ListRef<DataContainerClusterMonitoringConfigElManagedPrometheusElAutoMonitoringConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_monitoring_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterMonitoringConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_datapath_observability_config: Option<
        ListField<DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_components: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_prometheus:
        Option<ListField<DataContainerClusterMonitoringConfigElManagedPrometheusEl>>,
}
impl DataContainerClusterMonitoringConfigEl {
    #[doc = "Set the field `advanced_datapath_observability_config`.\n"]
    pub fn set_advanced_datapath_observability_config(
        mut self,
        v: impl Into<
            ListField<DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigEl>,
        >,
    ) -> Self {
        self.advanced_datapath_observability_config = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_components`.\n"]
    pub fn set_enable_components(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.enable_components = Some(v.into());
        self
    }
    #[doc = "Set the field `managed_prometheus`.\n"]
    pub fn set_managed_prometheus(
        mut self,
        v: impl Into<ListField<DataContainerClusterMonitoringConfigElManagedPrometheusEl>>,
    ) -> Self {
        self.managed_prometheus = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterMonitoringConfigEl {
    type O = BlockAssignable<DataContainerClusterMonitoringConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterMonitoringConfigEl {}
impl BuildDataContainerClusterMonitoringConfigEl {
    pub fn build(self) -> DataContainerClusterMonitoringConfigEl {
        DataContainerClusterMonitoringConfigEl {
            advanced_datapath_observability_config: core::default::Default::default(),
            enable_components: core::default::Default::default(),
            managed_prometheus: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterMonitoringConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterMonitoringConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterMonitoringConfigElRef {
        DataContainerClusterMonitoringConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterMonitoringConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `advanced_datapath_observability_config` after provisioning.\n"]
    pub fn advanced_datapath_observability_config(
        &self,
    ) -> ListRef<DataContainerClusterMonitoringConfigElAdvancedDatapathObservabilityConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_datapath_observability_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_components` after provisioning.\n"]
    pub fn enable_components(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enable_components", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `managed_prometheus` after provisioning.\n"]
    pub fn managed_prometheus(
        &self,
    ) -> ListRef<DataContainerClusterMonitoringConfigElManagedPrometheusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_prometheus", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNetworkPerformanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    total_egress_bandwidth_tier: Option<PrimField<String>>,
}
impl DataContainerClusterNetworkPerformanceConfigEl {
    #[doc = "Set the field `total_egress_bandwidth_tier`.\n"]
    pub fn set_total_egress_bandwidth_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.total_egress_bandwidth_tier = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNetworkPerformanceConfigEl {
    type O = BlockAssignable<DataContainerClusterNetworkPerformanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNetworkPerformanceConfigEl {}
impl BuildDataContainerClusterNetworkPerformanceConfigEl {
    pub fn build(self) -> DataContainerClusterNetworkPerformanceConfigEl {
        DataContainerClusterNetworkPerformanceConfigEl {
            total_egress_bandwidth_tier: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNetworkPerformanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNetworkPerformanceConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNetworkPerformanceConfigElRef {
        DataContainerClusterNetworkPerformanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNetworkPerformanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `total_egress_bandwidth_tier` after provisioning.\n"]
    pub fn total_egress_bandwidth_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_egress_bandwidth_tier", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNetworkPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<PrimField<String>>,
}
impl DataContainerClusterNetworkPolicyEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `provider`.\n"]
    pub fn set_provider(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.provider = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNetworkPolicyEl {
    type O = BlockAssignable<DataContainerClusterNetworkPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNetworkPolicyEl {}
impl BuildDataContainerClusterNetworkPolicyEl {
    pub fn build(self) -> DataContainerClusterNetworkPolicyEl {
        DataContainerClusterNetworkPolicyEl {
            enabled: core::default::Default::default(),
            provider: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNetworkPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNetworkPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNetworkPolicyElRef {
        DataContainerClusterNetworkPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNetworkPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElAdvancedMachineFeaturesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_nested_virtualization: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performance_monitoring_unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threads_per_core: Option<PrimField<f64>>,
}
impl DataContainerClusterNodeConfigElAdvancedMachineFeaturesEl {
    #[doc = "Set the field `enable_nested_virtualization`.\n"]
    pub fn set_enable_nested_virtualization(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_nested_virtualization = Some(v.into());
        self
    }
    #[doc = "Set the field `performance_monitoring_unit`.\n"]
    pub fn set_performance_monitoring_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.performance_monitoring_unit = Some(v.into());
        self
    }
    #[doc = "Set the field `threads_per_core`.\n"]
    pub fn set_threads_per_core(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.threads_per_core = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElAdvancedMachineFeaturesEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElAdvancedMachineFeaturesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElAdvancedMachineFeaturesEl {}
impl BuildDataContainerClusterNodeConfigElAdvancedMachineFeaturesEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElAdvancedMachineFeaturesEl {
        DataContainerClusterNodeConfigElAdvancedMachineFeaturesEl {
            enable_nested_virtualization: core::default::Default::default(),
            performance_monitoring_unit: core::default::Default::default(),
            threads_per_core: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElAdvancedMachineFeaturesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElAdvancedMachineFeaturesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElAdvancedMachineFeaturesElRef {
        DataContainerClusterNodeConfigElAdvancedMachineFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElAdvancedMachineFeaturesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_nested_virtualization` after provisioning.\n"]
    pub fn enable_nested_virtualization(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_nested_virtualization", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `performance_monitoring_unit` after provisioning.\n"]
    pub fn performance_monitoring_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.performance_monitoring_unit", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `threads_per_core` after provisioning.\n"]
    pub fn threads_per_core(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.threads_per_core", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElBootDiskEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioned_iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioned_throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size_gb: Option<PrimField<f64>>,
}
impl DataContainerClusterNodeConfigElBootDiskEl {
    #[doc = "Set the field `disk_type`.\n"]
    pub fn set_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_type = Some(v.into());
        self
    }
    #[doc = "Set the field `provisioned_iops`.\n"]
    pub fn set_provisioned_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.provisioned_iops = Some(v.into());
        self
    }
    #[doc = "Set the field `provisioned_throughput`.\n"]
    pub fn set_provisioned_throughput(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.provisioned_throughput = Some(v.into());
        self
    }
    #[doc = "Set the field `size_gb`.\n"]
    pub fn set_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size_gb = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElBootDiskEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElBootDiskEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElBootDiskEl {}
impl BuildDataContainerClusterNodeConfigElBootDiskEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElBootDiskEl {
        DataContainerClusterNodeConfigElBootDiskEl {
            disk_type: core::default::Default::default(),
            provisioned_iops: core::default::Default::default(),
            provisioned_throughput: core::default::Default::default(),
            size_gb: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElBootDiskElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElBootDiskElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodeConfigElBootDiskElRef {
        DataContainerClusterNodeConfigElBootDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElBootDiskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\n"]
    pub fn disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }
    #[doc = "Get a reference to the value of field `provisioned_iops` after provisioning.\n"]
    pub fn provisioned_iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_iops", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `provisioned_throughput` after provisioning.\n"]
    pub fn provisioned_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_throughput", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `size_gb` after provisioning.\n"]
    pub fn size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_gb", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElConfidentialNodesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential_instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterNodeConfigElConfidentialNodesEl {
    #[doc = "Set the field `confidential_instance_type`.\n"]
    pub fn set_confidential_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.confidential_instance_type = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElConfidentialNodesEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElConfidentialNodesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElConfidentialNodesEl {}
impl BuildDataContainerClusterNodeConfigElConfidentialNodesEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElConfidentialNodesEl {
        DataContainerClusterNodeConfigElConfidentialNodesEl {
            confidential_instance_type: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElConfidentialNodesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElConfidentialNodesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElConfidentialNodesElRef {
        DataContainerClusterNodeConfigElConfidentialNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElConfidentialNodesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `confidential_instance_type` after provisioning.\n"]
    pub fn confidential_instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.confidential_instance_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_uri: Option<PrimField<String>>,
}
impl DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl { # [doc = "Set the field `secret_uri`.\n"] pub fn set_secret_uri (mut self , v : impl Into < PrimField < String > >) -> Self { self . secret_uri = Some (v . into ()) ; self } }
impl ToListMappable for DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl { type O = BlockAssignable < DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl
{}
impl BuildDataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl { pub fn build (self) -> DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl { DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl { secret_uri : core :: default :: Default :: default () , } } }
pub struct DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigElRef { fn new (shared : StackShared , base : String) -> DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigElRef { DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigElRef { shared : shared , base : base . to_string () , } } }
impl DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `secret_uri` after provisioning.\n"] pub fn secret_uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.secret_uri" , self . base)) } }
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl { # [serde (skip_serializing_if = "Option::is_none")] fqdns : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] gcp_secret_manager_certificate_config : Option < ListField < DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl > > , }
impl DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl { # [doc = "Set the field `fqdns`.\n"] pub fn set_fqdns (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . fqdns = Some (v . into ()) ; self } # [doc = "Set the field `gcp_secret_manager_certificate_config`.\n"] pub fn set_gcp_secret_manager_certificate_config (mut self , v : impl Into < ListField < DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl > >) -> Self { self . gcp_secret_manager_certificate_config = Some (v . into ()) ; self } }
impl ToListMappable for DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl { type O = BlockAssignable < DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl
{}
impl BuildDataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl { pub fn build (self) -> DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl { DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl { fqdns : core :: default :: Default :: default () , gcp_secret_manager_certificate_config : core :: default :: Default :: default () , } } }
pub struct DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElRef { fn new (shared : StackShared , base : String) -> DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElRef { DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElRef { shared : shared , base : base . to_string () , } } }
impl DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `fqdns` after provisioning.\n"] pub fn fqdns (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.fqdns" , self . base)) } # [doc = "Get a reference to the value of field `gcp_secret_manager_certificate_config` after provisioning.\n"] pub fn gcp_secret_manager_certificate_config (& self) -> ListRef < DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.gcp_secret_manager_certificate_config" , self . base)) } }
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl { # [serde (skip_serializing_if = "Option::is_none")] certificate_authority_domain_config : Option < ListField < DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] enabled : Option < PrimField < bool > > , }
impl DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl {
    #[doc = "Set the field `certificate_authority_domain_config`.\n"]
    pub fn set_certificate_authority_domain_config(
        mut self,
        v : impl Into < ListField < DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl > >,
    ) -> Self {
        self.certificate_authority_domain_config = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl
{
    type O = BlockAssignable<
        DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl {}
impl BuildDataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl {
        DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl {
            certificate_authority_domain_config: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElRef {
        DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificate_authority_domain_config` after provisioning.\n"]    pub fn certificate_authority_domain_config (& self) -> ListRef < DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.certificate_authority_domain_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_secret_manager_secret_uri: Option<PrimField<String>>,
}
impl DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl {
    #[doc = "Set the field `gcp_secret_manager_secret_uri`.\n"]
    pub fn set_gcp_secret_manager_secret_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_secret_manager_secret_uri = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl
{
    type O = BlockAssignable<
        DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl {}
impl BuildDataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl {
        DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl {
            gcp_secret_manager_secret_uri: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElCaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElCaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElCaElRef {
        DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElCaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElCaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gcp_secret_manager_secret_uri` after provisioning.\n"]
    pub fn gcp_secret_manager_secret_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_secret_manager_secret_uri", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_secret_manager_secret_uri: Option<PrimField<String>>,
}
impl DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl {
    #[doc = "Set the field `gcp_secret_manager_secret_uri`.\n"]
    pub fn set_gcp_secret_manager_secret_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_secret_manager_secret_uri = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl
{
    type O = BlockAssignable<
        DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl
{}
impl BuildDataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl
    {
        DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl {
            gcp_secret_manager_secret_uri: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertElRef
    {
        DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gcp_secret_manager_secret_uri` after provisioning.\n"]
    pub fn gcp_secret_manager_secret_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_secret_manager_secret_uri", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_secret_manager_secret_uri: Option<PrimField<String>>,
}
impl DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl {
    #[doc = "Set the field `gcp_secret_manager_secret_uri`.\n"]
    pub fn set_gcp_secret_manager_secret_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_secret_manager_secret_uri = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl
{
    type O = BlockAssignable<
        DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl
{}
impl BuildDataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl {
        DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl {
            gcp_secret_manager_secret_uri: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyElRef
    {
        DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gcp_secret_manager_secret_uri` after provisioning.\n"]
    pub fn gcp_secret_manager_secret_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_secret_manager_secret_uri", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cert: Option<
        ListField<
            DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<
        ListField<
            DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl,
        >,
    >,
}
impl DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl {
    #[doc = "Set the field `cert`.\n"]
    pub fn set_cert(
        mut self,
        v : impl Into < ListField < DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl > >,
    ) -> Self {
        self.cert = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(
        mut self,
        v : impl Into < ListField < DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl > >,
    ) -> Self {
        self.key = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl
{
    type O = BlockAssignable<
        DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl {
}
impl BuildDataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl {
        DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl {
            cert: core::default::Default::default(),
            key: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElRef {
        DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cert` after provisioning.\n"]
    pub fn cert(
        &self,
    ) -> ListRef<
        DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.cert", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(
        &self,
    ) -> ListRef<
        DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.key", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<ListField<PrimField<String>>>,
}
impl DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl
{
    type O = BlockAssignable<
        DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl {
}
impl BuildDataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl {
        DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderElRef {
        DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ca: Option<
        ListField<DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    capabilities: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client: Option<
        ListField<DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    dial_timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<
        ListField<DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    override_path: Option<PrimField<bool>>,
}
impl DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsEl {
    #[doc = "Set the field `ca`.\n"]
    pub fn set_ca(
        mut self,
        v: impl Into<
            ListField<DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl>,
        >,
    ) -> Self {
        self.ca = Some(v.into());
        self
    }
    #[doc = "Set the field `capabilities`.\n"]
    pub fn set_capabilities(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.capabilities = Some(v.into());
        self
    }
    #[doc = "Set the field `client`.\n"]
    pub fn set_client(
        mut self,
        v: impl Into<
            ListField<
                DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl,
            >,
        >,
    ) -> Self {
        self.client = Some(v.into());
        self
    }
    #[doc = "Set the field `dial_timeout`.\n"]
    pub fn set_dial_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dial_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl Into<
            ListField<
                DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl,
            >,
        >,
    ) -> Self {
        self.header = Some(v.into());
        self
    }
    #[doc = "Set the field `host`.\n"]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }
    #[doc = "Set the field `override_path`.\n"]
    pub fn set_override_path(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.override_path = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsEl {
    type O =
        BlockAssignable<DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsEl {}
impl BuildDataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsEl {
        DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsEl {
            ca: core::default::Default::default(),
            capabilities: core::default::Default::default(),
            client: core::default::Default::default(),
            dial_timeout: core::default::Default::default(),
            header: core::default::Default::default(),
            host: core::default::Default::default(),
            override_path: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElRef {
        DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca` after provisioning.\n"]
    pub fn ca(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElCaElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.ca", self.base))
    }
    #[doc = "Get a reference to the value of field `capabilities` after provisioning.\n"]
    pub fn capabilities(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.capabilities", self.base))
    }
    #[doc = "Get a reference to the value of field `client` after provisioning.\n"]
    pub fn client(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.client", self.base))
    }
    #[doc = "Get a reference to the value of field `dial_timeout` after provisioning.\n"]
    pub fn dial_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dial_timeout", self.base))
    }
    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `override_path` after provisioning.\n"]
    pub fn override_path(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.override_path", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hosts:
        Option<ListField<DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server: Option<PrimField<String>>,
}
impl DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsEl {
    #[doc = "Set the field `hosts`.\n"]
    pub fn set_hosts(
        mut self,
        v: impl Into<
            ListField<DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsEl>,
        >,
    ) -> Self {
        self.hosts = Some(v.into());
        self
    }
    #[doc = "Set the field `server`.\n"]
    pub fn set_server(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.server = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElContainerdConfigElRegistryHostsEl {}
impl BuildDataContainerClusterNodeConfigElContainerdConfigElRegistryHostsEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsEl {
        DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsEl {
            hosts: core::default::Default::default(),
            server: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElRef {
        DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hosts` after provisioning.\n"]
    pub fn hosts(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElHostsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hosts", self.base))
    }
    #[doc = "Get a reference to the value of field `server` after provisioning.\n"]
    pub fn server(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElContainerdConfigElWritableCgroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterNodeConfigElContainerdConfigElWritableCgroupsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElContainerdConfigElWritableCgroupsEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElContainerdConfigElWritableCgroupsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElContainerdConfigElWritableCgroupsEl {}
impl BuildDataContainerClusterNodeConfigElContainerdConfigElWritableCgroupsEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElContainerdConfigElWritableCgroupsEl {
        DataContainerClusterNodeConfigElContainerdConfigElWritableCgroupsEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElContainerdConfigElWritableCgroupsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElContainerdConfigElWritableCgroupsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElContainerdConfigElWritableCgroupsElRef {
        DataContainerClusterNodeConfigElContainerdConfigElWritableCgroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElContainerdConfigElWritableCgroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElContainerdConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    private_registry_access_config: Option<
        ListField<DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    registry_hosts:
        Option<ListField<DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    writable_cgroups:
        Option<ListField<DataContainerClusterNodeConfigElContainerdConfigElWritableCgroupsEl>>,
}
impl DataContainerClusterNodeConfigElContainerdConfigEl {
    #[doc = "Set the field `private_registry_access_config`.\n"]
    pub fn set_private_registry_access_config(
        mut self,
        v: impl Into<
            ListField<
                DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl,
            >,
        >,
    ) -> Self {
        self.private_registry_access_config = Some(v.into());
        self
    }
    #[doc = "Set the field `registry_hosts`.\n"]
    pub fn set_registry_hosts(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsEl>>,
    ) -> Self {
        self.registry_hosts = Some(v.into());
        self
    }
    #[doc = "Set the field `writable_cgroups`.\n"]
    pub fn set_writable_cgroups(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElContainerdConfigElWritableCgroupsEl>>,
    ) -> Self {
        self.writable_cgroups = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElContainerdConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElContainerdConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElContainerdConfigEl {}
impl BuildDataContainerClusterNodeConfigElContainerdConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElContainerdConfigEl {
        DataContainerClusterNodeConfigElContainerdConfigEl {
            private_registry_access_config: core::default::Default::default(),
            registry_hosts: core::default::Default::default(),
            writable_cgroups: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElContainerdConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElContainerdConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElContainerdConfigElRef {
        DataContainerClusterNodeConfigElContainerdConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElContainerdConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `private_registry_access_config` after provisioning.\n"]
    pub fn private_registry_access_config(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_registry_access_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `registry_hosts` after provisioning.\n"]
    pub fn registry_hosts(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElContainerdConfigElRegistryHostsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.registry_hosts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `writable_cgroups` after provisioning.\n"]
    pub fn writable_cgroups(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElContainerdConfigElWritableCgroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.writable_cgroups", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElEffectiveTaintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataContainerClusterNodeConfigElEffectiveTaintsEl {
    #[doc = "Set the field `effect`.\n"]
    pub fn set_effect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.effect = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElEffectiveTaintsEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElEffectiveTaintsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElEffectiveTaintsEl {}
impl BuildDataContainerClusterNodeConfigElEffectiveTaintsEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElEffectiveTaintsEl {
        DataContainerClusterNodeConfigElEffectiveTaintsEl {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElEffectiveTaintsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElEffectiveTaintsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElEffectiveTaintsElRef {
        DataContainerClusterNodeConfigElEffectiveTaintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElEffectiveTaintsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `effect` after provisioning.\n"]
    pub fn effect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_cache_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssd_count: Option<PrimField<f64>>,
}
impl DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl {
    #[doc = "Set the field `data_cache_count`.\n"]
    pub fn set_data_cache_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.data_cache_count = Some(v.into());
        self
    }
    #[doc = "Set the field `local_ssd_count`.\n"]
    pub fn set_local_ssd_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.local_ssd_count = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl {}
impl BuildDataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl {
        DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl {
            data_cache_count: core::default::Default::default(),
            local_ssd_count: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigElRef {
        DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_cache_count` after provisioning.\n"]
    pub fn data_cache_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_cache_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_ssd_count` after provisioning.\n"]
    pub fn local_ssd_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_ssd_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElFastSocketEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterNodeConfigElFastSocketEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElFastSocketEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElFastSocketEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElFastSocketEl {}
impl BuildDataContainerClusterNodeConfigElFastSocketEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElFastSocketEl {
        DataContainerClusterNodeConfigElFastSocketEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElFastSocketElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElFastSocketElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodeConfigElFastSocketElRef {
        DataContainerClusterNodeConfigElFastSocketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElFastSocketElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElGcfsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterNodeConfigElGcfsConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElGcfsConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElGcfsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElGcfsConfigEl {}
impl BuildDataContainerClusterNodeConfigElGcfsConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElGcfsConfigEl {
        DataContainerClusterNodeConfigElGcfsConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElGcfsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElGcfsConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodeConfigElGcfsConfigElRef {
        DataContainerClusterNodeConfigElGcfsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElGcfsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_driver_version: Option<PrimField<String>>,
}
impl DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    #[doc = "Set the field `gpu_driver_version`.\n"]
    pub fn set_gpu_driver_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gpu_driver_version = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl
{
    type O = BlockAssignable<
        DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {}
impl BuildDataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
        DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
            gpu_driver_version: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
        DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gpu_driver_version` after provisioning.\n"]
    pub fn gpu_driver_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gpu_driver_version", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_sharing_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_shared_clients_per_gpu: Option<PrimField<f64>>,
}
impl DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    #[doc = "Set the field `gpu_sharing_strategy`.\n"]
    pub fn set_gpu_sharing_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gpu_sharing_strategy = Some(v.into());
        self
    }
    #[doc = "Set the field `max_shared_clients_per_gpu`.\n"]
    pub fn set_max_shared_clients_per_gpu(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_shared_clients_per_gpu = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl {}
impl BuildDataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
        DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
            gpu_sharing_strategy: core::default::Default::default(),
            max_shared_clients_per_gpu: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
        DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gpu_sharing_strategy` after provisioning.\n"]
    pub fn gpu_sharing_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gpu_sharing_strategy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_shared_clients_per_gpu` after provisioning.\n"]
    pub fn max_shared_clients_per_gpu(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_shared_clients_per_gpu", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElGuestAcceleratorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_driver_installation_config: Option<
        ListField<DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_partition_size: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_sharing_config:
        Option<ListField<DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataContainerClusterNodeConfigElGuestAcceleratorEl {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }
    #[doc = "Set the field `gpu_driver_installation_config`.\n"]
    pub fn set_gpu_driver_installation_config(
        mut self,
        v: impl Into<
            ListField<
                DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl,
            >,
        >,
    ) -> Self {
        self.gpu_driver_installation_config = Some(v.into());
        self
    }
    #[doc = "Set the field `gpu_partition_size`.\n"]
    pub fn set_gpu_partition_size(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gpu_partition_size = Some(v.into());
        self
    }
    #[doc = "Set the field `gpu_sharing_config`.\n"]
    pub fn set_gpu_sharing_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigEl>>,
    ) -> Self {
        self.gpu_sharing_config = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElGuestAcceleratorEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElGuestAcceleratorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElGuestAcceleratorEl {}
impl BuildDataContainerClusterNodeConfigElGuestAcceleratorEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElGuestAcceleratorEl {
        DataContainerClusterNodeConfigElGuestAcceleratorEl {
            count: core::default::Default::default(),
            gpu_driver_installation_config: core::default::Default::default(),
            gpu_partition_size: core::default::Default::default(),
            gpu_sharing_config: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElGuestAcceleratorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElGuestAcceleratorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElGuestAcceleratorElRef {
        DataContainerClusterNodeConfigElGuestAcceleratorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElGuestAcceleratorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `gpu_driver_installation_config` after provisioning.\n"]
    pub fn gpu_driver_installation_config(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gpu_driver_installation_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gpu_partition_size` after provisioning.\n"]
    pub fn gpu_partition_size(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gpu_partition_size", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gpu_sharing_config` after provisioning.\n"]
    pub fn gpu_sharing_config(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElGuestAcceleratorElGpuSharingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gpu_sharing_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElGvnicEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterNodeConfigElGvnicEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElGvnicEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElGvnicEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElGvnicEl {}
impl BuildDataContainerClusterNodeConfigElGvnicEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElGvnicEl {
        DataContainerClusterNodeConfigElGvnicEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElGvnicElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElGvnicElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodeConfigElGvnicElRef {
        DataContainerClusterNodeConfigElGvnicElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElGvnicElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElHostMaintenancePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_interval: Option<PrimField<String>>,
}
impl DataContainerClusterNodeConfigElHostMaintenancePolicyEl {
    #[doc = "Set the field `maintenance_interval`.\n"]
    pub fn set_maintenance_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.maintenance_interval = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElHostMaintenancePolicyEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElHostMaintenancePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElHostMaintenancePolicyEl {}
impl BuildDataContainerClusterNodeConfigElHostMaintenancePolicyEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElHostMaintenancePolicyEl {
        DataContainerClusterNodeConfigElHostMaintenancePolicyEl {
            maintenance_interval: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElHostMaintenancePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElHostMaintenancePolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElHostMaintenancePolicyElRef {
        DataContainerClusterNodeConfigElHostMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElHostMaintenancePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `maintenance_interval` after provisioning.\n"]
    pub fn maintenance_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_interval", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElKubeletConfigElEvictionMinimumReclaimEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    imagefs_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    imagefs_inodes_free: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodefs_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodefs_inodes_free: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pid_available: Option<PrimField<String>>,
}
impl DataContainerClusterNodeConfigElKubeletConfigElEvictionMinimumReclaimEl {
    #[doc = "Set the field `imagefs_available`.\n"]
    pub fn set_imagefs_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.imagefs_available = Some(v.into());
        self
    }
    #[doc = "Set the field `imagefs_inodes_free`.\n"]
    pub fn set_imagefs_inodes_free(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.imagefs_inodes_free = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_available`.\n"]
    pub fn set_memory_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.memory_available = Some(v.into());
        self
    }
    #[doc = "Set the field `nodefs_available`.\n"]
    pub fn set_nodefs_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nodefs_available = Some(v.into());
        self
    }
    #[doc = "Set the field `nodefs_inodes_free`.\n"]
    pub fn set_nodefs_inodes_free(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nodefs_inodes_free = Some(v.into());
        self
    }
    #[doc = "Set the field `pid_available`.\n"]
    pub fn set_pid_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pid_available = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElKubeletConfigElEvictionMinimumReclaimEl {
    type O =
        BlockAssignable<DataContainerClusterNodeConfigElKubeletConfigElEvictionMinimumReclaimEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElKubeletConfigElEvictionMinimumReclaimEl {}
impl BuildDataContainerClusterNodeConfigElKubeletConfigElEvictionMinimumReclaimEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElKubeletConfigElEvictionMinimumReclaimEl {
        DataContainerClusterNodeConfigElKubeletConfigElEvictionMinimumReclaimEl {
            imagefs_available: core::default::Default::default(),
            imagefs_inodes_free: core::default::Default::default(),
            memory_available: core::default::Default::default(),
            nodefs_available: core::default::Default::default(),
            nodefs_inodes_free: core::default::Default::default(),
            pid_available: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElKubeletConfigElEvictionMinimumReclaimElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElKubeletConfigElEvictionMinimumReclaimElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElKubeletConfigElEvictionMinimumReclaimElRef {
        DataContainerClusterNodeConfigElKubeletConfigElEvictionMinimumReclaimElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElKubeletConfigElEvictionMinimumReclaimElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `imagefs_available` after provisioning.\n"]
    pub fn imagefs_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.imagefs_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `imagefs_inodes_free` after provisioning.\n"]
    pub fn imagefs_inodes_free(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.imagefs_inodes_free", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_available` after provisioning.\n"]
    pub fn memory_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nodefs_available` after provisioning.\n"]
    pub fn nodefs_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nodefs_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nodefs_inodes_free` after provisioning.\n"]
    pub fn nodefs_inodes_free(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nodefs_inodes_free", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pid_available` after provisioning.\n"]
    pub fn pid_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pid_available", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    imagefs_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    imagefs_inodes_free: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodefs_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodefs_inodes_free: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pid_available: Option<PrimField<String>>,
}
impl DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftEl {
    #[doc = "Set the field `imagefs_available`.\n"]
    pub fn set_imagefs_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.imagefs_available = Some(v.into());
        self
    }
    #[doc = "Set the field `imagefs_inodes_free`.\n"]
    pub fn set_imagefs_inodes_free(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.imagefs_inodes_free = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_available`.\n"]
    pub fn set_memory_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.memory_available = Some(v.into());
        self
    }
    #[doc = "Set the field `nodefs_available`.\n"]
    pub fn set_nodefs_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nodefs_available = Some(v.into());
        self
    }
    #[doc = "Set the field `nodefs_inodes_free`.\n"]
    pub fn set_nodefs_inodes_free(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nodefs_inodes_free = Some(v.into());
        self
    }
    #[doc = "Set the field `pid_available`.\n"]
    pub fn set_pid_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pid_available = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElKubeletConfigElEvictionSoftEl {}
impl BuildDataContainerClusterNodeConfigElKubeletConfigElEvictionSoftEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftEl {
        DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftEl {
            imagefs_available: core::default::Default::default(),
            imagefs_inodes_free: core::default::Default::default(),
            memory_available: core::default::Default::default(),
            nodefs_available: core::default::Default::default(),
            nodefs_inodes_free: core::default::Default::default(),
            pid_available: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftElRef {
        DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `imagefs_available` after provisioning.\n"]
    pub fn imagefs_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.imagefs_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `imagefs_inodes_free` after provisioning.\n"]
    pub fn imagefs_inodes_free(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.imagefs_inodes_free", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_available` after provisioning.\n"]
    pub fn memory_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nodefs_available` after provisioning.\n"]
    pub fn nodefs_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nodefs_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nodefs_inodes_free` after provisioning.\n"]
    pub fn nodefs_inodes_free(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nodefs_inodes_free", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pid_available` after provisioning.\n"]
    pub fn pid_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pid_available", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    imagefs_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    imagefs_inodes_free: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodefs_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodefs_inodes_free: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pid_available: Option<PrimField<String>>,
}
impl DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl {
    #[doc = "Set the field `imagefs_available`.\n"]
    pub fn set_imagefs_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.imagefs_available = Some(v.into());
        self
    }
    #[doc = "Set the field `imagefs_inodes_free`.\n"]
    pub fn set_imagefs_inodes_free(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.imagefs_inodes_free = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_available`.\n"]
    pub fn set_memory_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.memory_available = Some(v.into());
        self
    }
    #[doc = "Set the field `nodefs_available`.\n"]
    pub fn set_nodefs_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nodefs_available = Some(v.into());
        self
    }
    #[doc = "Set the field `nodefs_inodes_free`.\n"]
    pub fn set_nodefs_inodes_free(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nodefs_inodes_free = Some(v.into());
        self
    }
    #[doc = "Set the field `pid_available`.\n"]
    pub fn set_pid_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pid_available = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl {
    type O =
        BlockAssignable<DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl {}
impl BuildDataContainerClusterNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl {
        DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl {
            imagefs_available: core::default::Default::default(),
            imagefs_inodes_free: core::default::Default::default(),
            memory_available: core::default::Default::default(),
            nodefs_available: core::default::Default::default(),
            nodefs_inodes_free: core::default::Default::default(),
            pid_available: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftGracePeriodElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftGracePeriodElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftGracePeriodElRef {
        DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftGracePeriodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftGracePeriodElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `imagefs_available` after provisioning.\n"]
    pub fn imagefs_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.imagefs_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `imagefs_inodes_free` after provisioning.\n"]
    pub fn imagefs_inodes_free(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.imagefs_inodes_free", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_available` after provisioning.\n"]
    pub fn memory_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nodefs_available` after provisioning.\n"]
    pub fn nodefs_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nodefs_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nodefs_inodes_free` after provisioning.\n"]
    pub fn nodefs_inodes_free(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nodefs_inodes_free", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pid_available` after provisioning.\n"]
    pub fn pid_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pid_available", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElKubeletConfigElMemoryManagerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
}
impl DataContainerClusterNodeConfigElKubeletConfigElMemoryManagerEl {
    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElKubeletConfigElMemoryManagerEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElKubeletConfigElMemoryManagerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElKubeletConfigElMemoryManagerEl {}
impl BuildDataContainerClusterNodeConfigElKubeletConfigElMemoryManagerEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElKubeletConfigElMemoryManagerEl {
        DataContainerClusterNodeConfigElKubeletConfigElMemoryManagerEl {
            policy: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElKubeletConfigElMemoryManagerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElKubeletConfigElMemoryManagerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElKubeletConfigElMemoryManagerElRef {
        DataContainerClusterNodeConfigElKubeletConfigElMemoryManagerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElKubeletConfigElMemoryManagerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElKubeletConfigElTopologyManagerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
}
impl DataContainerClusterNodeConfigElKubeletConfigElTopologyManagerEl {
    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy = Some(v.into());
        self
    }
    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElKubeletConfigElTopologyManagerEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElKubeletConfigElTopologyManagerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElKubeletConfigElTopologyManagerEl {}
impl BuildDataContainerClusterNodeConfigElKubeletConfigElTopologyManagerEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElKubeletConfigElTopologyManagerEl {
        DataContainerClusterNodeConfigElKubeletConfigElTopologyManagerEl {
            policy: core::default::Default::default(),
            scope: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElKubeletConfigElTopologyManagerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElKubeletConfigElTopologyManagerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElKubeletConfigElTopologyManagerElRef {
        DataContainerClusterNodeConfigElKubeletConfigElTopologyManagerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElKubeletConfigElTopologyManagerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElKubeletConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_unsafe_sysctls: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_log_max_files: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_log_max_size: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_cfs_quota: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_cfs_quota_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_manager_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eviction_max_pod_grace_period_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eviction_minimum_reclaim:
        Option<ListField<DataContainerClusterNodeConfigElKubeletConfigElEvictionMinimumReclaimEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eviction_soft: Option<ListField<DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eviction_soft_grace_period:
        Option<ListField<DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_gc_high_threshold_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_gc_low_threshold_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_maximum_gc_age: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_minimum_gc_age: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insecure_kubelet_readonly_port_enabled: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_parallel_image_pulls: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_manager:
        Option<ListField<DataContainerClusterNodeConfigElKubeletConfigElMemoryManagerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_pids_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    single_process_oom_kill: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topology_manager:
        Option<ListField<DataContainerClusterNodeConfigElKubeletConfigElTopologyManagerEl>>,
}
impl DataContainerClusterNodeConfigElKubeletConfigEl {
    #[doc = "Set the field `allowed_unsafe_sysctls`.\n"]
    pub fn set_allowed_unsafe_sysctls(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.allowed_unsafe_sysctls = Some(v.into());
        self
    }
    #[doc = "Set the field `container_log_max_files`.\n"]
    pub fn set_container_log_max_files(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.container_log_max_files = Some(v.into());
        self
    }
    #[doc = "Set the field `container_log_max_size`.\n"]
    pub fn set_container_log_max_size(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container_log_max_size = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu_cfs_quota`.\n"]
    pub fn set_cpu_cfs_quota(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cpu_cfs_quota = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu_cfs_quota_period`.\n"]
    pub fn set_cpu_cfs_quota_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cpu_cfs_quota_period = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu_manager_policy`.\n"]
    pub fn set_cpu_manager_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cpu_manager_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `eviction_max_pod_grace_period_seconds`.\n"]
    pub fn set_eviction_max_pod_grace_period_seconds(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.eviction_max_pod_grace_period_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `eviction_minimum_reclaim`.\n"]
    pub fn set_eviction_minimum_reclaim(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElKubeletConfigElEvictionMinimumReclaimEl>>,
    ) -> Self {
        self.eviction_minimum_reclaim = Some(v.into());
        self
    }
    #[doc = "Set the field `eviction_soft`.\n"]
    pub fn set_eviction_soft(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftEl>>,
    ) -> Self {
        self.eviction_soft = Some(v.into());
        self
    }
    #[doc = "Set the field `eviction_soft_grace_period`.\n"]
    pub fn set_eviction_soft_grace_period(
        mut self,
        v: impl Into<
            ListField<DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl>,
        >,
    ) -> Self {
        self.eviction_soft_grace_period = Some(v.into());
        self
    }
    #[doc = "Set the field `image_gc_high_threshold_percent`.\n"]
    pub fn set_image_gc_high_threshold_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.image_gc_high_threshold_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `image_gc_low_threshold_percent`.\n"]
    pub fn set_image_gc_low_threshold_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.image_gc_low_threshold_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `image_maximum_gc_age`.\n"]
    pub fn set_image_maximum_gc_age(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_maximum_gc_age = Some(v.into());
        self
    }
    #[doc = "Set the field `image_minimum_gc_age`.\n"]
    pub fn set_image_minimum_gc_age(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_minimum_gc_age = Some(v.into());
        self
    }
    #[doc = "Set the field `insecure_kubelet_readonly_port_enabled`.\n"]
    pub fn set_insecure_kubelet_readonly_port_enabled(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.insecure_kubelet_readonly_port_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `max_parallel_image_pulls`.\n"]
    pub fn set_max_parallel_image_pulls(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_parallel_image_pulls = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_manager`.\n"]
    pub fn set_memory_manager(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElKubeletConfigElMemoryManagerEl>>,
    ) -> Self {
        self.memory_manager = Some(v.into());
        self
    }
    #[doc = "Set the field `pod_pids_limit`.\n"]
    pub fn set_pod_pids_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.pod_pids_limit = Some(v.into());
        self
    }
    #[doc = "Set the field `single_process_oom_kill`.\n"]
    pub fn set_single_process_oom_kill(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.single_process_oom_kill = Some(v.into());
        self
    }
    #[doc = "Set the field `topology_manager`.\n"]
    pub fn set_topology_manager(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElKubeletConfigElTopologyManagerEl>>,
    ) -> Self {
        self.topology_manager = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElKubeletConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElKubeletConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElKubeletConfigEl {}
impl BuildDataContainerClusterNodeConfigElKubeletConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElKubeletConfigEl {
        DataContainerClusterNodeConfigElKubeletConfigEl {
            allowed_unsafe_sysctls: core::default::Default::default(),
            container_log_max_files: core::default::Default::default(),
            container_log_max_size: core::default::Default::default(),
            cpu_cfs_quota: core::default::Default::default(),
            cpu_cfs_quota_period: core::default::Default::default(),
            cpu_manager_policy: core::default::Default::default(),
            eviction_max_pod_grace_period_seconds: core::default::Default::default(),
            eviction_minimum_reclaim: core::default::Default::default(),
            eviction_soft: core::default::Default::default(),
            eviction_soft_grace_period: core::default::Default::default(),
            image_gc_high_threshold_percent: core::default::Default::default(),
            image_gc_low_threshold_percent: core::default::Default::default(),
            image_maximum_gc_age: core::default::Default::default(),
            image_minimum_gc_age: core::default::Default::default(),
            insecure_kubelet_readonly_port_enabled: core::default::Default::default(),
            max_parallel_image_pulls: core::default::Default::default(),
            memory_manager: core::default::Default::default(),
            pod_pids_limit: core::default::Default::default(),
            single_process_oom_kill: core::default::Default::default(),
            topology_manager: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElKubeletConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElKubeletConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElKubeletConfigElRef {
        DataContainerClusterNodeConfigElKubeletConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElKubeletConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_unsafe_sysctls` after provisioning.\n"]
    pub fn allowed_unsafe_sysctls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_unsafe_sysctls", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `container_log_max_files` after provisioning.\n"]
    pub fn container_log_max_files(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_log_max_files", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `container_log_max_size` after provisioning.\n"]
    pub fn container_log_max_size(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_log_max_size", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_cfs_quota` after provisioning.\n"]
    pub fn cpu_cfs_quota(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_cfs_quota", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_cfs_quota_period` after provisioning.\n"]
    pub fn cpu_cfs_quota_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_cfs_quota_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_manager_policy` after provisioning.\n"]
    pub fn cpu_manager_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_manager_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `eviction_max_pod_grace_period_seconds` after provisioning.\n"]
    pub fn eviction_max_pod_grace_period_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.eviction_max_pod_grace_period_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `eviction_minimum_reclaim` after provisioning.\n"]
    pub fn eviction_minimum_reclaim(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElKubeletConfigElEvictionMinimumReclaimElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.eviction_minimum_reclaim", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `eviction_soft` after provisioning.\n"]
    pub fn eviction_soft(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.eviction_soft", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `eviction_soft_grace_period` after provisioning.\n"]
    pub fn eviction_soft_grace_period(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElKubeletConfigElEvictionSoftGracePeriodElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.eviction_soft_grace_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_gc_high_threshold_percent` after provisioning.\n"]
    pub fn image_gc_high_threshold_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_gc_high_threshold_percent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_gc_low_threshold_percent` after provisioning.\n"]
    pub fn image_gc_low_threshold_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_gc_low_threshold_percent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_maximum_gc_age` after provisioning.\n"]
    pub fn image_maximum_gc_age(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_maximum_gc_age", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_minimum_gc_age` after provisioning.\n"]
    pub fn image_minimum_gc_age(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_minimum_gc_age", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `insecure_kubelet_readonly_port_enabled` after provisioning.\n"]
    pub fn insecure_kubelet_readonly_port_enabled(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.insecure_kubelet_readonly_port_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_parallel_image_pulls` after provisioning.\n"]
    pub fn max_parallel_image_pulls(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_parallel_image_pulls", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_manager` after provisioning.\n"]
    pub fn memory_manager(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElKubeletConfigElMemoryManagerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.memory_manager", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pod_pids_limit` after provisioning.\n"]
    pub fn pod_pids_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pod_pids_limit", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `single_process_oom_kill` after provisioning.\n"]
    pub fn single_process_oom_kill(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.single_process_oom_kill", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `topology_manager` after provisioning.\n"]
    pub fn topology_manager(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElKubeletConfigElTopologyManagerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.topology_manager", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_ptp_kvm_time_sync: Option<PrimField<bool>>,
}
impl DataContainerClusterNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl {
    #[doc = "Set the field `enable_ptp_kvm_time_sync`.\n"]
    pub fn set_enable_ptp_kvm_time_sync(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_ptp_kvm_time_sync = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl {}
impl BuildDataContainerClusterNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl {
        DataContainerClusterNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl {
            enable_ptp_kvm_time_sync: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElLinuxNodeConfigElAccurateTimeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElLinuxNodeConfigElAccurateTimeConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElLinuxNodeConfigElAccurateTimeConfigElRef {
        DataContainerClusterNodeConfigElLinuxNodeConfigElAccurateTimeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElLinuxNodeConfigElAccurateTimeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_ptp_kvm_time_sync` after provisioning.\n"]
    pub fn enable_ptp_kvm_time_sync(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_ptp_kvm_time_sync", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElLinuxNodeConfigElHugepagesConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hugepage_size_1g: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hugepage_size_2m: Option<PrimField<f64>>,
}
impl DataContainerClusterNodeConfigElLinuxNodeConfigElHugepagesConfigEl {
    #[doc = "Set the field `hugepage_size_1g`.\n"]
    pub fn set_hugepage_size_1g(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hugepage_size_1g = Some(v.into());
        self
    }
    #[doc = "Set the field `hugepage_size_2m`.\n"]
    pub fn set_hugepage_size_2m(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hugepage_size_2m = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElLinuxNodeConfigElHugepagesConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElLinuxNodeConfigElHugepagesConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElLinuxNodeConfigElHugepagesConfigEl {}
impl BuildDataContainerClusterNodeConfigElLinuxNodeConfigElHugepagesConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElLinuxNodeConfigElHugepagesConfigEl {
        DataContainerClusterNodeConfigElLinuxNodeConfigElHugepagesConfigEl {
            hugepage_size_1g: core::default::Default::default(),
            hugepage_size_2m: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElLinuxNodeConfigElHugepagesConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElLinuxNodeConfigElHugepagesConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElLinuxNodeConfigElHugepagesConfigElRef {
        DataContainerClusterNodeConfigElLinuxNodeConfigElHugepagesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElLinuxNodeConfigElHugepagesConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hugepage_size_1g` after provisioning.\n"]
    pub fn hugepage_size_1g(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hugepage_size_1g", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hugepage_size_2m` after provisioning.\n"]
    pub fn hugepage_size_2m(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hugepage_size_2m", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
}
impl DataContainerClusterNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl {
    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl {
    type O =
        BlockAssignable<DataContainerClusterNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl {}
impl BuildDataContainerClusterNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl {
        DataContainerClusterNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl {
            policy: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingElRef {
        DataContainerClusterNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    swap_size_gib: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    swap_size_percent: Option<PrimField<f64>>,
}
impl DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl {
    #[doc = "Set the field `swap_size_gib`.\n"]
    pub fn set_swap_size_gib(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.swap_size_gib = Some(v.into());
        self
    }
    #[doc = "Set the field `swap_size_percent`.\n"]
    pub fn set_swap_size_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.swap_size_percent = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl
{
    type O = BlockAssignable<
        DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl {}
impl BuildDataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl {
        DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl {
            swap_size_gib: core::default::Default::default(),
            swap_size_percent: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileElRef {
        DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `swap_size_gib` after provisioning.\n"]
    pub fn swap_size_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.swap_size_gib", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `swap_size_percent` after provisioning.\n"]
    pub fn swap_size_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.swap_size_percent", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_count: Option<PrimField<f64>>,
}
impl DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl {
    #[doc = "Set the field `disk_count`.\n"]
    pub fn set_disk_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_count = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl
{
    type O = BlockAssignable<
        DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl
{}
impl BuildDataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl
    {
        DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl {
            disk_count: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileElRef
    {
        DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk_count` after provisioning.\n"]
    pub fn disk_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_count", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
}
impl DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl
{
    type O = BlockAssignable<
        DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl {}
impl BuildDataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl {
        DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl {
            disabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigElRef {
        DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    swap_size_gib: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    swap_size_percent: Option<PrimField<f64>>,
}
impl DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl {
    #[doc = "Set the field `swap_size_gib`.\n"]
    pub fn set_swap_size_gib(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.swap_size_gib = Some(v.into());
        self
    }
    #[doc = "Set the field `swap_size_percent`.\n"]
    pub fn set_swap_size_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.swap_size_percent = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl
{
    type O = BlockAssignable<
        DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl
{}
impl BuildDataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl
    {
        DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl {
            swap_size_gib: core::default::Default::default(),
            swap_size_percent: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileElRef
    {
        DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `swap_size_gib` after provisioning.\n"]
    pub fn swap_size_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.swap_size_gib", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `swap_size_percent` after provisioning.\n"]
    pub fn swap_size_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.swap_size_percent", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_profile: Option<
        ListField<DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    dedicated_local_ssd_profile: Option<
        ListField<
            DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_config: Option<
        ListField<DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    ephemeral_local_ssd_profile: Option<
        ListField<
            DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl,
        >,
    >,
}
impl DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigEl {
    #[doc = "Set the field `boot_disk_profile`.\n"]
    pub fn set_boot_disk_profile(
        mut self,
        v: impl Into<
            ListField<
                DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl,
            >,
        >,
    ) -> Self {
        self.boot_disk_profile = Some(v.into());
        self
    }
    #[doc = "Set the field `dedicated_local_ssd_profile`.\n"]
    pub fn set_dedicated_local_ssd_profile(
        mut self,
        v : impl Into < ListField < DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl > >,
    ) -> Self {
        self.dedicated_local_ssd_profile = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v: impl Into<
            ListField<
                DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl,
            >,
        >,
    ) -> Self {
        self.encryption_config = Some(v.into());
        self
    }
    #[doc = "Set the field `ephemeral_local_ssd_profile`.\n"]
    pub fn set_ephemeral_local_ssd_profile(
        mut self,
        v : impl Into < ListField < DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl > >,
    ) -> Self {
        self.ephemeral_local_ssd_profile = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigEl {}
impl BuildDataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigEl {
        DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigEl {
            boot_disk_profile: core::default::Default::default(),
            dedicated_local_ssd_profile: core::default::Default::default(),
            enabled: core::default::Default::default(),
            encryption_config: core::default::Default::default(),
            ephemeral_local_ssd_profile: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElRef {
        DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `boot_disk_profile` after provisioning.\n"]
    pub fn boot_disk_profile(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.boot_disk_profile", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dedicated_local_ssd_profile` after provisioning.\n"]
    pub fn dedicated_local_ssd_profile(
        &self,
    ) -> ListRef<
        DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dedicated_local_ssd_profile", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ephemeral_local_ssd_profile` after provisioning.\n"]
    pub fn ephemeral_local_ssd_profile(
        &self,
    ) -> ListRef<
        DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ephemeral_local_ssd_profile", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElLinuxNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accurate_time_config:
        Option<ListField<DataContainerClusterNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cgroup_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hugepages_config:
        Option<ListField<DataContainerClusterNodeConfigElLinuxNodeConfigElHugepagesConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_kernel_module_loading: Option<
        ListField<DataContainerClusterNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    swap_config: Option<ListField<DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sysctls: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transparent_hugepage_defrag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transparent_hugepage_enabled: Option<PrimField<String>>,
}
impl DataContainerClusterNodeConfigElLinuxNodeConfigEl {
    #[doc = "Set the field `accurate_time_config`.\n"]
    pub fn set_accurate_time_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl>>,
    ) -> Self {
        self.accurate_time_config = Some(v.into());
        self
    }
    #[doc = "Set the field `cgroup_mode`.\n"]
    pub fn set_cgroup_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cgroup_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `hugepages_config`.\n"]
    pub fn set_hugepages_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElLinuxNodeConfigElHugepagesConfigEl>>,
    ) -> Self {
        self.hugepages_config = Some(v.into());
        self
    }
    #[doc = "Set the field `node_kernel_module_loading`.\n"]
    pub fn set_node_kernel_module_loading(
        mut self,
        v: impl Into<
            ListField<DataContainerClusterNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl>,
        >,
    ) -> Self {
        self.node_kernel_module_loading = Some(v.into());
        self
    }
    #[doc = "Set the field `swap_config`.\n"]
    pub fn set_swap_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigEl>>,
    ) -> Self {
        self.swap_config = Some(v.into());
        self
    }
    #[doc = "Set the field `sysctls`.\n"]
    pub fn set_sysctls(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.sysctls = Some(v.into());
        self
    }
    #[doc = "Set the field `transparent_hugepage_defrag`.\n"]
    pub fn set_transparent_hugepage_defrag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.transparent_hugepage_defrag = Some(v.into());
        self
    }
    #[doc = "Set the field `transparent_hugepage_enabled`.\n"]
    pub fn set_transparent_hugepage_enabled(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.transparent_hugepage_enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElLinuxNodeConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElLinuxNodeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElLinuxNodeConfigEl {}
impl BuildDataContainerClusterNodeConfigElLinuxNodeConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElLinuxNodeConfigEl {
        DataContainerClusterNodeConfigElLinuxNodeConfigEl {
            accurate_time_config: core::default::Default::default(),
            cgroup_mode: core::default::Default::default(),
            hugepages_config: core::default::Default::default(),
            node_kernel_module_loading: core::default::Default::default(),
            swap_config: core::default::Default::default(),
            sysctls: core::default::Default::default(),
            transparent_hugepage_defrag: core::default::Default::default(),
            transparent_hugepage_enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElLinuxNodeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElLinuxNodeConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElLinuxNodeConfigElRef {
        DataContainerClusterNodeConfigElLinuxNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElLinuxNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accurate_time_config` after provisioning.\n"]
    pub fn accurate_time_config(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElLinuxNodeConfigElAccurateTimeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.accurate_time_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cgroup_mode` after provisioning.\n"]
    pub fn cgroup_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cgroup_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `hugepages_config` after provisioning.\n"]
    pub fn hugepages_config(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElLinuxNodeConfigElHugepagesConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hugepages_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_kernel_module_loading` after provisioning.\n"]
    pub fn node_kernel_module_loading(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_kernel_module_loading", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `swap_config` after provisioning.\n"]
    pub fn swap_config(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElLinuxNodeConfigElSwapConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.swap_config", self.base))
    }
    #[doc = "Get a reference to the value of field `sysctls` after provisioning.\n"]
    pub fn sysctls(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.sysctls", self.base))
    }
    #[doc = "Get a reference to the value of field `transparent_hugepage_defrag` after provisioning.\n"]
    pub fn transparent_hugepage_defrag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transparent_hugepage_defrag", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `transparent_hugepage_enabled` after provisioning.\n"]
    pub fn transparent_hugepage_enabled(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transparent_hugepage_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssd_count: Option<PrimField<f64>>,
}
impl DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl {
    #[doc = "Set the field `local_ssd_count`.\n"]
    pub fn set_local_ssd_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.local_ssd_count = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl {}
impl BuildDataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl {
        DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl {
            local_ssd_count: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigElRef {
        DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `local_ssd_count` after provisioning.\n"]
    pub fn local_ssd_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_ssd_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElReservationAffinityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    consume_reservation_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}
impl DataContainerClusterNodeConfigElReservationAffinityEl {
    #[doc = "Set the field `consume_reservation_type`.\n"]
    pub fn set_consume_reservation_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.consume_reservation_type = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElReservationAffinityEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElReservationAffinityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElReservationAffinityEl {}
impl BuildDataContainerClusterNodeConfigElReservationAffinityEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElReservationAffinityEl {
        DataContainerClusterNodeConfigElReservationAffinityEl {
            consume_reservation_type: core::default::Default::default(),
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElReservationAffinityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElReservationAffinityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElReservationAffinityElRef {
        DataContainerClusterNodeConfigElReservationAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElReservationAffinityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `consume_reservation_type` after provisioning.\n"]
    pub fn consume_reservation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consume_reservation_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElSandboxConfigEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataContainerClusterNodeConfigElSandboxConfigEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElSandboxConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElSandboxConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElSandboxConfigEl {}
impl BuildDataContainerClusterNodeConfigElSandboxConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElSandboxConfigEl {
        DataContainerClusterNodeConfigElSandboxConfigEl {
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElSandboxConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElSandboxConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElSandboxConfigElRef {
        DataContainerClusterNodeConfigElSandboxConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElSandboxConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElSecondaryBootDisksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}
impl DataContainerClusterNodeConfigElSecondaryBootDisksEl {
    #[doc = "Set the field `disk_image`.\n"]
    pub fn set_disk_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_image = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElSecondaryBootDisksEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElSecondaryBootDisksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElSecondaryBootDisksEl {}
impl BuildDataContainerClusterNodeConfigElSecondaryBootDisksEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElSecondaryBootDisksEl {
        DataContainerClusterNodeConfigElSecondaryBootDisksEl {
            disk_image: core::default::Default::default(),
            mode: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElSecondaryBootDisksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElSecondaryBootDisksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElSecondaryBootDisksElRef {
        DataContainerClusterNodeConfigElSecondaryBootDisksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElSecondaryBootDisksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk_image` after provisioning.\n"]
    pub fn disk_image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_image", self.base))
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElShieldedInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_integrity_monitoring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_secure_boot: Option<PrimField<bool>>,
}
impl DataContainerClusterNodeConfigElShieldedInstanceConfigEl {
    #[doc = "Set the field `enable_integrity_monitoring`.\n"]
    pub fn set_enable_integrity_monitoring(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_secure_boot`.\n"]
    pub fn set_enable_secure_boot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_secure_boot = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElShieldedInstanceConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElShieldedInstanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElShieldedInstanceConfigEl {}
impl BuildDataContainerClusterNodeConfigElShieldedInstanceConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElShieldedInstanceConfigEl {
        DataContainerClusterNodeConfigElShieldedInstanceConfigEl {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElShieldedInstanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElShieldedInstanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElShieldedInstanceConfigElRef {
        DataContainerClusterNodeConfigElShieldedInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElShieldedInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\n"]
    pub fn enable_integrity_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_integrity_monitoring", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_secure_boot` after provisioning.\n"]
    pub fn enable_secure_boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_secure_boot", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<ListField<PrimField<String>>>,
}
impl DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operator = Some(v.into());
        self
    }
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl {}
impl BuildDataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl {
        DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl {
            key: core::default::Default::default(),
            operator: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityElRef {
        DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElSoleTenantConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    min_node_cpus: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_affinity:
        Option<SetField<DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl>>,
}
impl DataContainerClusterNodeConfigElSoleTenantConfigEl {
    #[doc = "Set the field `min_node_cpus`.\n"]
    pub fn set_min_node_cpus(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_node_cpus = Some(v.into());
        self
    }
    #[doc = "Set the field `node_affinity`.\n"]
    pub fn set_node_affinity(
        mut self,
        v: impl Into<SetField<DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityEl>>,
    ) -> Self {
        self.node_affinity = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElSoleTenantConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElSoleTenantConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElSoleTenantConfigEl {}
impl BuildDataContainerClusterNodeConfigElSoleTenantConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElSoleTenantConfigEl {
        DataContainerClusterNodeConfigElSoleTenantConfigEl {
            min_node_cpus: core::default::Default::default(),
            node_affinity: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElSoleTenantConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElSoleTenantConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElSoleTenantConfigElRef {
        DataContainerClusterNodeConfigElSoleTenantConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElSoleTenantConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `min_node_cpus` after provisioning.\n"]
    pub fn min_node_cpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_node_cpus", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_affinity` after provisioning.\n"]
    pub fn node_affinity(
        &self,
    ) -> SetRef<DataContainerClusterNodeConfigElSoleTenantConfigElNodeAffinityElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.node_affinity", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElTaintEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataContainerClusterNodeConfigElTaintEl {
    #[doc = "Set the field `effect`.\n"]
    pub fn set_effect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.effect = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElTaintEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElTaintEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElTaintEl {}
impl BuildDataContainerClusterNodeConfigElTaintEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElTaintEl {
        DataContainerClusterNodeConfigElTaintEl {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElTaintElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElTaintElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodeConfigElTaintElRef {
        DataContainerClusterNodeConfigElTaintElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElTaintElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `effect` after provisioning.\n"]
    pub fn effect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElWindowsNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    osversion: Option<PrimField<String>>,
}
impl DataContainerClusterNodeConfigElWindowsNodeConfigEl {
    #[doc = "Set the field `osversion`.\n"]
    pub fn set_osversion(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.osversion = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElWindowsNodeConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElWindowsNodeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElWindowsNodeConfigEl {}
impl BuildDataContainerClusterNodeConfigElWindowsNodeConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElWindowsNodeConfigEl {
        DataContainerClusterNodeConfigElWindowsNodeConfigEl {
            osversion: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElWindowsNodeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElWindowsNodeConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElWindowsNodeConfigElRef {
        DataContainerClusterNodeConfigElWindowsNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElWindowsNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `osversion` after provisioning.\n"]
    pub fn osversion(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.osversion", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigElWorkloadMetadataConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}
impl DataContainerClusterNodeConfigElWorkloadMetadataConfigEl {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigElWorkloadMetadataConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigElWorkloadMetadataConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigElWorkloadMetadataConfigEl {}
impl BuildDataContainerClusterNodeConfigElWorkloadMetadataConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigElWorkloadMetadataConfigEl {
        DataContainerClusterNodeConfigElWorkloadMetadataConfigEl {
            mode: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElWorkloadMetadataConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElWorkloadMetadataConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodeConfigElWorkloadMetadataConfigElRef {
        DataContainerClusterNodeConfigElWorkloadMetadataConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElWorkloadMetadataConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_machine_features:
        Option<ListField<DataContainerClusterNodeConfigElAdvancedMachineFeaturesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk: Option<ListField<DataContainerClusterNodeConfigElBootDiskEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_kms_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential_nodes: Option<ListField<DataContainerClusterNodeConfigElConfidentialNodesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    containerd_config: Option<ListField<DataContainerClusterNodeConfigElContainerdConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_taints: Option<ListField<DataContainerClusterNodeConfigElEffectiveTaintsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_confidential_storage: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ephemeral_storage_local_ssd_config:
        Option<ListField<DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fast_socket: Option<ListField<DataContainerClusterNodeConfigElFastSocketEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flex_start: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcfs_config: Option<ListField<DataContainerClusterNodeConfigElGcfsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_accelerator: Option<ListField<DataContainerClusterNodeConfigElGuestAcceleratorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gvnic: Option<ListField<DataContainerClusterNodeConfigElGvnicEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_maintenance_policy:
        Option<ListField<DataContainerClusterNodeConfigElHostMaintenancePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kubelet_config: Option<ListField<DataContainerClusterNodeConfigElKubeletConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    linux_node_config: Option<ListField<DataContainerClusterNodeConfigElLinuxNodeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_nvme_ssd_block_config:
        Option<ListField<DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssd_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssd_encryption_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_variant: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_run_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_cpu_platform: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth_scopes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preemptible: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reservation_affinity: Option<ListField<DataContainerClusterNodeConfigElReservationAffinityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sandbox_config: Option<ListField<DataContainerClusterNodeConfigElSandboxConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_boot_disks: Option<ListField<DataContainerClusterNodeConfigElSecondaryBootDisksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shielded_instance_config:
        Option<ListField<DataContainerClusterNodeConfigElShieldedInstanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sole_tenant_config: Option<ListField<DataContainerClusterNodeConfigElSoleTenantConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_pools: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    taint: Option<ListField<DataContainerClusterNodeConfigElTaintEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    windows_node_config: Option<ListField<DataContainerClusterNodeConfigElWindowsNodeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workload_metadata_config:
        Option<ListField<DataContainerClusterNodeConfigElWorkloadMetadataConfigEl>>,
}
impl DataContainerClusterNodeConfigEl {
    #[doc = "Set the field `advanced_machine_features`.\n"]
    pub fn set_advanced_machine_features(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElAdvancedMachineFeaturesEl>>,
    ) -> Self {
        self.advanced_machine_features = Some(v.into());
        self
    }
    #[doc = "Set the field `boot_disk`.\n"]
    pub fn set_boot_disk(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElBootDiskEl>>,
    ) -> Self {
        self.boot_disk = Some(v.into());
        self
    }
    #[doc = "Set the field `boot_disk_kms_key`.\n"]
    pub fn set_boot_disk_kms_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.boot_disk_kms_key = Some(v.into());
        self
    }
    #[doc = "Set the field `confidential_nodes`.\n"]
    pub fn set_confidential_nodes(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElConfidentialNodesEl>>,
    ) -> Self {
        self.confidential_nodes = Some(v.into());
        self
    }
    #[doc = "Set the field `containerd_config`.\n"]
    pub fn set_containerd_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElContainerdConfigEl>>,
    ) -> Self {
        self.containerd_config = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_size_gb`.\n"]
    pub fn set_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_type`.\n"]
    pub fn set_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_type = Some(v.into());
        self
    }
    #[doc = "Set the field `effective_taints`.\n"]
    pub fn set_effective_taints(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElEffectiveTaintsEl>>,
    ) -> Self {
        self.effective_taints = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_confidential_storage`.\n"]
    pub fn set_enable_confidential_storage(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_confidential_storage = Some(v.into());
        self
    }
    #[doc = "Set the field `ephemeral_storage_local_ssd_config`.\n"]
    pub fn set_ephemeral_storage_local_ssd_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigEl>>,
    ) -> Self {
        self.ephemeral_storage_local_ssd_config = Some(v.into());
        self
    }
    #[doc = "Set the field `fast_socket`.\n"]
    pub fn set_fast_socket(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElFastSocketEl>>,
    ) -> Self {
        self.fast_socket = Some(v.into());
        self
    }
    #[doc = "Set the field `flex_start`.\n"]
    pub fn set_flex_start(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.flex_start = Some(v.into());
        self
    }
    #[doc = "Set the field `gcfs_config`.\n"]
    pub fn set_gcfs_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElGcfsConfigEl>>,
    ) -> Self {
        self.gcfs_config = Some(v.into());
        self
    }
    #[doc = "Set the field `guest_accelerator`.\n"]
    pub fn set_guest_accelerator(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElGuestAcceleratorEl>>,
    ) -> Self {
        self.guest_accelerator = Some(v.into());
        self
    }
    #[doc = "Set the field `gvnic`.\n"]
    pub fn set_gvnic(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElGvnicEl>>,
    ) -> Self {
        self.gvnic = Some(v.into());
        self
    }
    #[doc = "Set the field `host_maintenance_policy`.\n"]
    pub fn set_host_maintenance_policy(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElHostMaintenancePolicyEl>>,
    ) -> Self {
        self.host_maintenance_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `image_type`.\n"]
    pub fn set_image_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_type = Some(v.into());
        self
    }
    #[doc = "Set the field `kubelet_config`.\n"]
    pub fn set_kubelet_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElKubeletConfigEl>>,
    ) -> Self {
        self.kubelet_config = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `linux_node_config`.\n"]
    pub fn set_linux_node_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElLinuxNodeConfigEl>>,
    ) -> Self {
        self.linux_node_config = Some(v.into());
        self
    }
    #[doc = "Set the field `local_nvme_ssd_block_config`.\n"]
    pub fn set_local_nvme_ssd_block_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigEl>>,
    ) -> Self {
        self.local_nvme_ssd_block_config = Some(v.into());
        self
    }
    #[doc = "Set the field `local_ssd_count`.\n"]
    pub fn set_local_ssd_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.local_ssd_count = Some(v.into());
        self
    }
    #[doc = "Set the field `local_ssd_encryption_mode`.\n"]
    pub fn set_local_ssd_encryption_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_ssd_encryption_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `logging_variant`.\n"]
    pub fn set_logging_variant(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.logging_variant = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
    #[doc = "Set the field `max_run_duration`.\n"]
    pub fn set_max_run_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_run_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `min_cpu_platform`.\n"]
    pub fn set_min_cpu_platform(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_cpu_platform = Some(v.into());
        self
    }
    #[doc = "Set the field `node_group`.\n"]
    pub fn set_node_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_group = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth_scopes`.\n"]
    pub fn set_oauth_scopes(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.oauth_scopes = Some(v.into());
        self
    }
    #[doc = "Set the field `preemptible`.\n"]
    pub fn set_preemptible(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preemptible = Some(v.into());
        self
    }
    #[doc = "Set the field `reservation_affinity`.\n"]
    pub fn set_reservation_affinity(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElReservationAffinityEl>>,
    ) -> Self {
        self.reservation_affinity = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_labels`.\n"]
    pub fn set_resource_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_manager_tags`.\n"]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `sandbox_config`.\n"]
    pub fn set_sandbox_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElSandboxConfigEl>>,
    ) -> Self {
        self.sandbox_config = Some(v.into());
        self
    }
    #[doc = "Set the field `secondary_boot_disks`.\n"]
    pub fn set_secondary_boot_disks(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElSecondaryBootDisksEl>>,
    ) -> Self {
        self.secondary_boot_disks = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElShieldedInstanceConfigEl>>,
    ) -> Self {
        self.shielded_instance_config = Some(v.into());
        self
    }
    #[doc = "Set the field `sole_tenant_config`.\n"]
    pub fn set_sole_tenant_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElSoleTenantConfigEl>>,
    ) -> Self {
        self.sole_tenant_config = Some(v.into());
        self
    }
    #[doc = "Set the field `spot`.\n"]
    pub fn set_spot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.spot = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_pools`.\n"]
    pub fn set_storage_pools(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.storage_pools = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
    #[doc = "Set the field `taint`.\n"]
    pub fn set_taint(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElTaintEl>>,
    ) -> Self {
        self.taint = Some(v.into());
        self
    }
    #[doc = "Set the field `windows_node_config`.\n"]
    pub fn set_windows_node_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElWindowsNodeConfigEl>>,
    ) -> Self {
        self.windows_node_config = Some(v.into());
        self
    }
    #[doc = "Set the field `workload_metadata_config`.\n"]
    pub fn set_workload_metadata_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodeConfigElWorkloadMetadataConfigEl>>,
    ) -> Self {
        self.workload_metadata_config = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodeConfigEl {
    type O = BlockAssignable<DataContainerClusterNodeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodeConfigEl {}
impl BuildDataContainerClusterNodeConfigEl {
    pub fn build(self) -> DataContainerClusterNodeConfigEl {
        DataContainerClusterNodeConfigEl {
            advanced_machine_features: core::default::Default::default(),
            boot_disk: core::default::Default::default(),
            boot_disk_kms_key: core::default::Default::default(),
            confidential_nodes: core::default::Default::default(),
            containerd_config: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            effective_taints: core::default::Default::default(),
            enable_confidential_storage: core::default::Default::default(),
            ephemeral_storage_local_ssd_config: core::default::Default::default(),
            fast_socket: core::default::Default::default(),
            flex_start: core::default::Default::default(),
            gcfs_config: core::default::Default::default(),
            guest_accelerator: core::default::Default::default(),
            gvnic: core::default::Default::default(),
            host_maintenance_policy: core::default::Default::default(),
            image_type: core::default::Default::default(),
            kubelet_config: core::default::Default::default(),
            labels: core::default::Default::default(),
            linux_node_config: core::default::Default::default(),
            local_nvme_ssd_block_config: core::default::Default::default(),
            local_ssd_count: core::default::Default::default(),
            local_ssd_encryption_mode: core::default::Default::default(),
            logging_variant: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            max_run_duration: core::default::Default::default(),
            metadata: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            node_group: core::default::Default::default(),
            oauth_scopes: core::default::Default::default(),
            preemptible: core::default::Default::default(),
            reservation_affinity: core::default::Default::default(),
            resource_labels: core::default::Default::default(),
            resource_manager_tags: core::default::Default::default(),
            sandbox_config: core::default::Default::default(),
            secondary_boot_disks: core::default::Default::default(),
            service_account: core::default::Default::default(),
            shielded_instance_config: core::default::Default::default(),
            sole_tenant_config: core::default::Default::default(),
            spot: core::default::Default::default(),
            storage_pools: core::default::Default::default(),
            tags: core::default::Default::default(),
            taint: core::default::Default::default(),
            windows_node_config: core::default::Default::default(),
            workload_metadata_config: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodeConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodeConfigElRef {
        DataContainerClusterNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `advanced_machine_features` after provisioning.\n"]
    pub fn advanced_machine_features(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElAdvancedMachineFeaturesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_machine_features", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `boot_disk` after provisioning.\n"]
    pub fn boot_disk(&self) -> ListRef<DataContainerClusterNodeConfigElBootDiskElRef> {
        ListRef::new(self.shared().clone(), format!("{}.boot_disk", self.base))
    }
    #[doc = "Get a reference to the value of field `boot_disk_kms_key` after provisioning.\n"]
    pub fn boot_disk_kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_kms_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `confidential_nodes` after provisioning.\n"]
    pub fn confidential_nodes(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElConfidentialNodesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.confidential_nodes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `containerd_config` after provisioning.\n"]
    pub fn containerd_config(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElContainerdConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.containerd_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\n"]
    pub fn disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\n"]
    pub fn disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }
    #[doc = "Get a reference to the value of field `effective_taints` after provisioning.\n"]
    pub fn effective_taints(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElEffectiveTaintsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.effective_taints", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_confidential_storage` after provisioning.\n"]
    pub fn enable_confidential_storage(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_confidential_storage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ephemeral_storage_local_ssd_config` after provisioning.\n"]
    pub fn ephemeral_storage_local_ssd_config(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElEphemeralStorageLocalSsdConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ephemeral_storage_local_ssd_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fast_socket` after provisioning.\n"]
    pub fn fast_socket(&self) -> ListRef<DataContainerClusterNodeConfigElFastSocketElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fast_socket", self.base))
    }
    #[doc = "Get a reference to the value of field `flex_start` after provisioning.\n"]
    pub fn flex_start(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.flex_start", self.base))
    }
    #[doc = "Get a reference to the value of field `gcfs_config` after provisioning.\n"]
    pub fn gcfs_config(&self) -> ListRef<DataContainerClusterNodeConfigElGcfsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gcfs_config", self.base))
    }
    #[doc = "Get a reference to the value of field `guest_accelerator` after provisioning.\n"]
    pub fn guest_accelerator(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElGuestAcceleratorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guest_accelerator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gvnic` after provisioning.\n"]
    pub fn gvnic(&self) -> ListRef<DataContainerClusterNodeConfigElGvnicElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gvnic", self.base))
    }
    #[doc = "Get a reference to the value of field `host_maintenance_policy` after provisioning.\n"]
    pub fn host_maintenance_policy(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElHostMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.host_maintenance_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_type` after provisioning.\n"]
    pub fn image_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_type", self.base))
    }
    #[doc = "Get a reference to the value of field `kubelet_config` after provisioning.\n"]
    pub fn kubelet_config(&self) -> ListRef<DataContainerClusterNodeConfigElKubeletConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kubelet_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `linux_node_config` after provisioning.\n"]
    pub fn linux_node_config(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElLinuxNodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.linux_node_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_nvme_ssd_block_config` after provisioning.\n"]
    pub fn local_nvme_ssd_block_config(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElLocalNvmeSsdBlockConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.local_nvme_ssd_block_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_ssd_count` after provisioning.\n"]
    pub fn local_ssd_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_ssd_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_ssd_encryption_mode` after provisioning.\n"]
    pub fn local_ssd_encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_ssd_encryption_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `logging_variant` after provisioning.\n"]
    pub fn logging_variant(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logging_variant", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `max_run_duration` after provisioning.\n"]
    pub fn max_run_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_run_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
    #[doc = "Get a reference to the value of field `min_cpu_platform` after provisioning.\n"]
    pub fn min_cpu_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_cpu_platform", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_group` after provisioning.\n"]
    pub fn node_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_group", self.base))
    }
    #[doc = "Get a reference to the value of field `oauth_scopes` after provisioning.\n"]
    pub fn oauth_scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.oauth_scopes", self.base))
    }
    #[doc = "Get a reference to the value of field `preemptible` after provisioning.\n"]
    pub fn preemptible(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preemptible", self.base))
    }
    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElReservationAffinityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reservation_affinity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_labels` after provisioning.\n"]
    pub fn resource_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_labels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\n"]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_manager_tags", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sandbox_config` after provisioning.\n"]
    pub fn sandbox_config(&self) -> ListRef<DataContainerClusterNodeConfigElSandboxConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sandbox_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_boot_disks` after provisioning.\n"]
    pub fn secondary_boot_disks(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElSecondaryBootDisksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secondary_boot_disks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElShieldedInstanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.shielded_instance_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sole_tenant_config` after provisioning.\n"]
    pub fn sole_tenant_config(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElSoleTenantConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sole_tenant_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `spot` after provisioning.\n"]
    pub fn spot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.spot", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_pools` after provisioning.\n"]
    pub fn storage_pools(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_pools", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
    #[doc = "Get a reference to the value of field `taint` after provisioning.\n"]
    pub fn taint(&self) -> ListRef<DataContainerClusterNodeConfigElTaintElRef> {
        ListRef::new(self.shared().clone(), format!("{}.taint", self.base))
    }
    #[doc = "Get a reference to the value of field `windows_node_config` after provisioning.\n"]
    pub fn windows_node_config(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElWindowsNodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.windows_node_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `workload_metadata_config` after provisioning.\n"]
    pub fn workload_metadata_config(
        &self,
    ) -> ListRef<DataContainerClusterNodeConfigElWorkloadMetadataConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload_metadata_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElAutoscalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    location_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_max_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_min_node_count: Option<PrimField<f64>>,
}
impl DataContainerClusterNodePoolElAutoscalingEl {
    #[doc = "Set the field `location_policy`.\n"]
    pub fn set_location_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `max_node_count`.\n"]
    pub fn set_max_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `min_node_count`.\n"]
    pub fn set_min_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `total_max_node_count`.\n"]
    pub fn set_total_max_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.total_max_node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `total_min_node_count`.\n"]
    pub fn set_total_min_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.total_min_node_count = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElAutoscalingEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElAutoscalingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElAutoscalingEl {}
impl BuildDataContainerClusterNodePoolElAutoscalingEl {
    pub fn build(self) -> DataContainerClusterNodePoolElAutoscalingEl {
        DataContainerClusterNodePoolElAutoscalingEl {
            location_policy: core::default::Default::default(),
            max_node_count: core::default::Default::default(),
            min_node_count: core::default::Default::default(),
            total_max_node_count: core::default::Default::default(),
            total_min_node_count: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElAutoscalingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElAutoscalingElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolElAutoscalingElRef {
        DataContainerClusterNodePoolElAutoscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElAutoscalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `location_policy` after provisioning.\n"]
    pub fn location_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_node_count` after provisioning.\n"]
    pub fn max_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_node_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_node_count` after provisioning.\n"]
    pub fn min_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_node_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_max_node_count` after provisioning.\n"]
    pub fn total_max_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_max_node_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_min_node_count` after provisioning.\n"]
    pub fn total_min_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_min_node_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElManagementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_repair: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_upgrade: Option<PrimField<bool>>,
}
impl DataContainerClusterNodePoolElManagementEl {
    #[doc = "Set the field `auto_repair`.\n"]
    pub fn set_auto_repair(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_repair = Some(v.into());
        self
    }
    #[doc = "Set the field `auto_upgrade`.\n"]
    pub fn set_auto_upgrade(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_upgrade = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElManagementEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElManagementEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElManagementEl {}
impl BuildDataContainerClusterNodePoolElManagementEl {
    pub fn build(self) -> DataContainerClusterNodePoolElManagementEl {
        DataContainerClusterNodePoolElManagementEl {
            auto_repair: core::default::Default::default(),
            auto_upgrade: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElManagementElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElManagementElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolElManagementElRef {
        DataContainerClusterNodePoolElManagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElManagementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_repair` after provisioning.\n"]
    pub fn auto_repair(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_repair", self.base))
    }
    #[doc = "Get a reference to the value of field `auto_upgrade` after provisioning.\n"]
    pub fn auto_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_upgrade", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNetworkConfigElAdditionalNodeNetworkConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolElNetworkConfigElAdditionalNodeNetworkConfigsEl {
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\n"]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodePoolElNetworkConfigElAdditionalNodeNetworkConfigsEl
{
    type O = BlockAssignable<
        DataContainerClusterNodePoolElNetworkConfigElAdditionalNodeNetworkConfigsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNetworkConfigElAdditionalNodeNetworkConfigsEl {}
impl BuildDataContainerClusterNodePoolElNetworkConfigElAdditionalNodeNetworkConfigsEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolElNetworkConfigElAdditionalNodeNetworkConfigsEl {
        DataContainerClusterNodePoolElNetworkConfigElAdditionalNodeNetworkConfigsEl {
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNetworkConfigElAdditionalNodeNetworkConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNetworkConfigElAdditionalNodeNetworkConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNetworkConfigElAdditionalNodeNetworkConfigsElRef {
        DataContainerClusterNodePoolElNetworkConfigElAdditionalNodeNetworkConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNetworkConfigElAdditionalNodeNetworkConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\n"]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNetworkConfigElAdditionalPodNetworkConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_pods_per_node: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_pod_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolElNetworkConfigElAdditionalPodNetworkConfigsEl {
    #[doc = "Set the field `max_pods_per_node`.\n"]
    pub fn set_max_pods_per_node(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_pods_per_node = Some(v.into());
        self
    }
    #[doc = "Set the field `secondary_pod_range`.\n"]
    pub fn set_secondary_pod_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secondary_pod_range = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\n"]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNetworkConfigElAdditionalPodNetworkConfigsEl {
    type O =
        BlockAssignable<DataContainerClusterNodePoolElNetworkConfigElAdditionalPodNetworkConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNetworkConfigElAdditionalPodNetworkConfigsEl {}
impl BuildDataContainerClusterNodePoolElNetworkConfigElAdditionalPodNetworkConfigsEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolElNetworkConfigElAdditionalPodNetworkConfigsEl {
        DataContainerClusterNodePoolElNetworkConfigElAdditionalPodNetworkConfigsEl {
            max_pods_per_node: core::default::Default::default(),
            secondary_pod_range: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNetworkConfigElAdditionalPodNetworkConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNetworkConfigElAdditionalPodNetworkConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNetworkConfigElAdditionalPodNetworkConfigsElRef {
        DataContainerClusterNodePoolElNetworkConfigElAdditionalPodNetworkConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNetworkConfigElAdditionalPodNetworkConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_pods_per_node` after provisioning.\n"]
    pub fn max_pods_per_node(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_pods_per_node", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_pod_range` after provisioning.\n"]
    pub fn secondary_pod_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secondary_pod_range", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\n"]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    total_egress_bandwidth_tier: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl {
    #[doc = "Set the field `total_egress_bandwidth_tier`.\n"]
    pub fn set_total_egress_bandwidth_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.total_egress_bandwidth_tier = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl {
    type O =
        BlockAssignable<DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl {}
impl BuildDataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl {
        DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl {
            total_egress_bandwidth_tier: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigElRef {
        DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `total_egress_bandwidth_tier` after provisioning.\n"]
    pub fn total_egress_bandwidth_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_egress_bandwidth_tier", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
}
impl DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl {
    type O =
        BlockAssignable<DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl {}
impl BuildDataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl {
        DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl {
            disabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigElRef {
        DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNetworkConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_network_profile: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_node_network_configs: Option<
        ListField<DataContainerClusterNodePoolElNetworkConfigElAdditionalNodeNetworkConfigsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_pod_network_configs: Option<
        ListField<DataContainerClusterNodePoolElNetworkConfigElAdditionalPodNetworkConfigsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_pod_range: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_private_nodes: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_performance_config:
        Option<ListField<DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_cidr_overprovision_config: Option<
        ListField<DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_ipv4_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolElNetworkConfigEl {
    #[doc = "Set the field `accelerator_network_profile`.\n"]
    pub fn set_accelerator_network_profile(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accelerator_network_profile = Some(v.into());
        self
    }
    #[doc = "Set the field `additional_node_network_configs`.\n"]
    pub fn set_additional_node_network_configs(
        mut self,
        v: impl Into<
            ListField<DataContainerClusterNodePoolElNetworkConfigElAdditionalNodeNetworkConfigsEl>,
        >,
    ) -> Self {
        self.additional_node_network_configs = Some(v.into());
        self
    }
    #[doc = "Set the field `additional_pod_network_configs`.\n"]
    pub fn set_additional_pod_network_configs(
        mut self,
        v: impl Into<
            ListField<DataContainerClusterNodePoolElNetworkConfigElAdditionalPodNetworkConfigsEl>,
        >,
    ) -> Self {
        self.additional_pod_network_configs = Some(v.into());
        self
    }
    #[doc = "Set the field `create_pod_range`.\n"]
    pub fn set_create_pod_range(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.create_pod_range = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_private_nodes`.\n"]
    pub fn set_enable_private_nodes(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_private_nodes = Some(v.into());
        self
    }
    #[doc = "Set the field `network_performance_config`.\n"]
    pub fn set_network_performance_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigEl>>,
    ) -> Self {
        self.network_performance_config = Some(v.into());
        self
    }
    #[doc = "Set the field `pod_cidr_overprovision_config`.\n"]
    pub fn set_pod_cidr_overprovision_config(
        mut self,
        v: impl Into<
            ListField<DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigEl>,
        >,
    ) -> Self {
        self.pod_cidr_overprovision_config = Some(v.into());
        self
    }
    #[doc = "Set the field `pod_ipv4_cidr_block`.\n"]
    pub fn set_pod_ipv4_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pod_ipv4_cidr_block = Some(v.into());
        self
    }
    #[doc = "Set the field `pod_range`.\n"]
    pub fn set_pod_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pod_range = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\n"]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNetworkConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNetworkConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNetworkConfigEl {}
impl BuildDataContainerClusterNodePoolElNetworkConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNetworkConfigEl {
        DataContainerClusterNodePoolElNetworkConfigEl {
            accelerator_network_profile: core::default::Default::default(),
            additional_node_network_configs: core::default::Default::default(),
            additional_pod_network_configs: core::default::Default::default(),
            create_pod_range: core::default::Default::default(),
            enable_private_nodes: core::default::Default::default(),
            network_performance_config: core::default::Default::default(),
            pod_cidr_overprovision_config: core::default::Default::default(),
            pod_ipv4_cidr_block: core::default::Default::default(),
            pod_range: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNetworkConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNetworkConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolElNetworkConfigElRef {
        DataContainerClusterNodePoolElNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNetworkConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator_network_profile` after provisioning.\n"]
    pub fn accelerator_network_profile(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_network_profile", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `additional_node_network_configs` after provisioning.\n"]
    pub fn additional_node_network_configs(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNetworkConfigElAdditionalNodeNetworkConfigsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_node_network_configs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `additional_pod_network_configs` after provisioning.\n"]
    pub fn additional_pod_network_configs(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNetworkConfigElAdditionalPodNetworkConfigsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_pod_network_configs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_pod_range` after provisioning.\n"]
    pub fn create_pod_range(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_pod_range", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_private_nodes` after provisioning.\n"]
    pub fn enable_private_nodes(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_private_nodes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_performance_config` after provisioning.\n"]
    pub fn network_performance_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNetworkConfigElNetworkPerformanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_performance_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pod_cidr_overprovision_config` after provisioning.\n"]
    pub fn pod_cidr_overprovision_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNetworkConfigElPodCidrOverprovisionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pod_cidr_overprovision_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pod_ipv4_cidr_block` after provisioning.\n"]
    pub fn pod_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pod_ipv4_cidr_block", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pod_range` after provisioning.\n"]
    pub fn pod_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pod_range", self.base))
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\n"]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_nested_virtualization: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performance_monitoring_unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threads_per_core: Option<PrimField<f64>>,
}
impl DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl {
    #[doc = "Set the field `enable_nested_virtualization`.\n"]
    pub fn set_enable_nested_virtualization(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_nested_virtualization = Some(v.into());
        self
    }
    #[doc = "Set the field `performance_monitoring_unit`.\n"]
    pub fn set_performance_monitoring_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.performance_monitoring_unit = Some(v.into());
        self
    }
    #[doc = "Set the field `threads_per_core`.\n"]
    pub fn set_threads_per_core(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.threads_per_core = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl {
        DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl {
            enable_nested_virtualization: core::default::Default::default(),
            performance_monitoring_unit: core::default::Default::default(),
            threads_per_core: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesElRef {
        DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_nested_virtualization` after provisioning.\n"]
    pub fn enable_nested_virtualization(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_nested_virtualization", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `performance_monitoring_unit` after provisioning.\n"]
    pub fn performance_monitoring_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.performance_monitoring_unit", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `threads_per_core` after provisioning.\n"]
    pub fn threads_per_core(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.threads_per_core", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElBootDiskEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioned_iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioned_throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size_gb: Option<PrimField<f64>>,
}
impl DataContainerClusterNodePoolElNodeConfigElBootDiskEl {
    #[doc = "Set the field `disk_type`.\n"]
    pub fn set_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_type = Some(v.into());
        self
    }
    #[doc = "Set the field `provisioned_iops`.\n"]
    pub fn set_provisioned_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.provisioned_iops = Some(v.into());
        self
    }
    #[doc = "Set the field `provisioned_throughput`.\n"]
    pub fn set_provisioned_throughput(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.provisioned_throughput = Some(v.into());
        self
    }
    #[doc = "Set the field `size_gb`.\n"]
    pub fn set_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size_gb = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElBootDiskEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElBootDiskEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElBootDiskEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElBootDiskEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElBootDiskEl {
        DataContainerClusterNodePoolElNodeConfigElBootDiskEl {
            disk_type: core::default::Default::default(),
            provisioned_iops: core::default::Default::default(),
            provisioned_throughput: core::default::Default::default(),
            size_gb: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElBootDiskElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElBootDiskElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElBootDiskElRef {
        DataContainerClusterNodePoolElNodeConfigElBootDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElBootDiskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\n"]
    pub fn disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }
    #[doc = "Get a reference to the value of field `provisioned_iops` after provisioning.\n"]
    pub fn provisioned_iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_iops", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `provisioned_throughput` after provisioning.\n"]
    pub fn provisioned_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_throughput", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `size_gb` after provisioning.\n"]
    pub fn size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_gb", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElConfidentialNodesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential_instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterNodePoolElNodeConfigElConfidentialNodesEl {
    #[doc = "Set the field `confidential_instance_type`.\n"]
    pub fn set_confidential_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.confidential_instance_type = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElConfidentialNodesEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElConfidentialNodesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElConfidentialNodesEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElConfidentialNodesEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElConfidentialNodesEl {
        DataContainerClusterNodePoolElNodeConfigElConfidentialNodesEl {
            confidential_instance_type: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElConfidentialNodesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElConfidentialNodesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElConfidentialNodesElRef {
        DataContainerClusterNodePoolElNodeConfigElConfidentialNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElConfidentialNodesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `confidential_instance_type` after provisioning.\n"]
    pub fn confidential_instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.confidential_instance_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_uri: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl { # [doc = "Set the field `secret_uri`.\n"] pub fn set_secret_uri (mut self , v : impl Into < PrimField < String > >) -> Self { self . secret_uri = Some (v . into ()) ; self } }
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl { type O = BlockAssignable < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl
{}
impl BuildDataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl { pub fn build (self) -> DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl { DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl { secret_uri : core :: default :: Default :: default () , } } }
pub struct DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigElRef { fn new (shared : StackShared , base : String) -> DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigElRef { DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigElRef { shared : shared , base : base . to_string () , } } }
impl DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `secret_uri` after provisioning.\n"] pub fn secret_uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.secret_uri" , self . base)) } }
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl { # [serde (skip_serializing_if = "Option::is_none")] fqdns : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] gcp_secret_manager_certificate_config : Option < ListField < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl > > , }
impl DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl { # [doc = "Set the field `fqdns`.\n"] pub fn set_fqdns (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . fqdns = Some (v . into ()) ; self } # [doc = "Set the field `gcp_secret_manager_certificate_config`.\n"] pub fn set_gcp_secret_manager_certificate_config (mut self , v : impl Into < ListField < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl > >) -> Self { self . gcp_secret_manager_certificate_config = Some (v . into ()) ; self } }
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl { type O = BlockAssignable < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl
{}
impl BuildDataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl { pub fn build (self) -> DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl { DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl { fqdns : core :: default :: Default :: default () , gcp_secret_manager_certificate_config : core :: default :: Default :: default () , } } }
pub struct DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElRef { fn new (shared : StackShared , base : String) -> DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElRef { DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElRef { shared : shared , base : base . to_string () , } } }
impl DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `fqdns` after provisioning.\n"] pub fn fqdns (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.fqdns" , self . base)) } # [doc = "Get a reference to the value of field `gcp_secret_manager_certificate_config` after provisioning.\n"] pub fn gcp_secret_manager_certificate_config (& self) -> ListRef < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.gcp_secret_manager_certificate_config" , self . base)) } }
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl { # [serde (skip_serializing_if = "Option::is_none")] certificate_authority_domain_config : Option < ListField < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] enabled : Option < PrimField < bool > > , }
impl DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl {
    #[doc = "Set the field `certificate_authority_domain_config`.\n"]
    pub fn set_certificate_authority_domain_config(
        mut self,
        v : impl Into < ListField < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl > >,
    ) -> Self {
        self.certificate_authority_domain_config = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl
{
    type O = BlockAssignable<
        DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl
{}
impl
    BuildDataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl
{
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl
    {
        DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl {
            certificate_authority_domain_config: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElRef
    {
        DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElRef { shared : shared , base : base . to_string () , }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificate_authority_domain_config` after provisioning.\n"]    pub fn certificate_authority_domain_config (& self) -> ListRef < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.certificate_authority_domain_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_secret_manager_secret_uri: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl {
    #[doc = "Set the field `gcp_secret_manager_secret_uri`.\n"]
    pub fn set_gcp_secret_manager_secret_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_secret_manager_secret_uri = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl
{
    type O = BlockAssignable<
        DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl
{}
impl BuildDataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl
    {
        DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl {
            gcp_secret_manager_secret_uri: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElCaElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElCaElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElCaElRef
    {
        DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElCaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElCaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gcp_secret_manager_secret_uri` after provisioning.\n"]
    pub fn gcp_secret_manager_secret_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_secret_manager_secret_uri", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_secret_manager_secret_uri: Option<PrimField<String>>,
}
impl
    DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl
{
    #[doc = "Set the field `gcp_secret_manager_secret_uri`.\n"]
    pub fn set_gcp_secret_manager_secret_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_secret_manager_secret_uri = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl { type O = BlockAssignable < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl
{}
impl BuildDataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl { pub fn build (self) -> DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl { DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl { gcp_secret_manager_secret_uri : core :: default :: Default :: default () , } } }
pub struct DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertElRef { fn new (shared : StackShared , base : String) -> DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertElRef { DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertElRef { shared : shared , base : base . to_string () , } } }
impl DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `gcp_secret_manager_secret_uri` after provisioning.\n"] pub fn gcp_secret_manager_secret_uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.gcp_secret_manager_secret_uri" , self . base)) } }
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_secret_manager_secret_uri: Option<PrimField<String>>,
}
impl
    DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl
{
    #[doc = "Set the field `gcp_secret_manager_secret_uri`.\n"]
    pub fn set_gcp_secret_manager_secret_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_secret_manager_secret_uri = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl { type O = BlockAssignable < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl
{}
impl BuildDataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl { pub fn build (self) -> DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl { DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl { gcp_secret_manager_secret_uri : core :: default :: Default :: default () , } } }
pub struct DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyElRef { fn new (shared : StackShared , base : String) -> DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyElRef { DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyElRef { shared : shared , base : base . to_string () , } } }
impl DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `gcp_secret_manager_secret_uri` after provisioning.\n"] pub fn gcp_secret_manager_secret_uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.gcp_secret_manager_secret_uri" , self . base)) } }
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl { # [serde (skip_serializing_if = "Option::is_none")] cert : Option < ListField < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl > > , # [serde (skip_serializing_if = "Option::is_none")] key : Option < ListField < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl > > , }
impl DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl {
    #[doc = "Set the field `cert`.\n"]
    pub fn set_cert(
        mut self,
        v : impl Into < ListField < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertEl > >,
    ) -> Self {
        self.cert = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(
        mut self,
        v : impl Into < ListField < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyEl > >,
    ) -> Self {
        self.key = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl
{
    type O = BlockAssignable<
        DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl
{}
impl
    BuildDataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl
{
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl
    {
        DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl {
            cert: core::default::Default::default(),
            key: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElRef { fn new (shared : StackShared , base : String) -> DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElRef { DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElRef { shared : shared , base : base . to_string () , } } }
impl DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cert` after provisioning.\n"]    pub fn cert (& self) -> ListRef < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElCertElRef >{
        ListRef::new(self.shared().clone(), format!("{}.cert", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]    pub fn key (& self) -> ListRef < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElKeyElRef >{
        ListRef::new(self.shared().clone(), format!("{}.key", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<ListField<PrimField<String>>>,
}
impl DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl
{
    type O = BlockAssignable<
        DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl
{}
impl
    BuildDataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl
{
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl
    {
        DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderElRef { fn new (shared : StackShared , base : String) -> DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderElRef { DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderElRef { shared : shared , base : base . to_string () , } } }
impl DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsEl { # [serde (skip_serializing_if = "Option::is_none")] ca : Option < ListField < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl > > , # [serde (skip_serializing_if = "Option::is_none")] capabilities : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] client : Option < ListField < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl > > , # [serde (skip_serializing_if = "Option::is_none")] dial_timeout : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] header : Option < ListField < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl > > , # [serde (skip_serializing_if = "Option::is_none")] host : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] override_path : Option < PrimField < bool > > , }
impl DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsEl {
    #[doc = "Set the field `ca`.\n"]
    pub fn set_ca(
        mut self,
        v : impl Into < ListField < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElCaEl > >,
    ) -> Self {
        self.ca = Some(v.into());
        self
    }
    #[doc = "Set the field `capabilities`.\n"]
    pub fn set_capabilities(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.capabilities = Some(v.into());
        self
    }
    #[doc = "Set the field `client`.\n"]
    pub fn set_client(
        mut self,
        v : impl Into < ListField < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientEl > >,
    ) -> Self {
        self.client = Some(v.into());
        self
    }
    #[doc = "Set the field `dial_timeout`.\n"]
    pub fn set_dial_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dial_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v : impl Into < ListField < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderEl > >,
    ) -> Self {
        self.header = Some(v.into());
        self
    }
    #[doc = "Set the field `host`.\n"]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }
    #[doc = "Set the field `override_path`.\n"]
    pub fn set_override_path(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.override_path = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsEl
{
    type O = BlockAssignable<
        DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsEl
{}
impl BuildDataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsEl {
        DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsEl {
            ca: core::default::Default::default(),
            capabilities: core::default::Default::default(),
            client: core::default::Default::default(),
            dial_timeout: core::default::Default::default(),
            header: core::default::Default::default(),
            host: core::default::Default::default(),
            override_path: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElRef {
        DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca` after provisioning.\n"]
    pub fn ca(
        &self,
    ) -> ListRef<
        DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElCaElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.ca", self.base))
    }
    #[doc = "Get a reference to the value of field `capabilities` after provisioning.\n"]
    pub fn capabilities(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.capabilities", self.base))
    }
    #[doc = "Get a reference to the value of field `client` after provisioning.\n"]    pub fn client (& self) -> ListRef < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElClientElRef >{
        ListRef::new(self.shared().clone(), format!("{}.client", self.base))
    }
    #[doc = "Get a reference to the value of field `dial_timeout` after provisioning.\n"]
    pub fn dial_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dial_timeout", self.base))
    }
    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]    pub fn header (& self) -> ListRef < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElHeaderElRef >{
        ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `override_path` after provisioning.\n"]
    pub fn override_path(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.override_path", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hosts: Option<
        ListField<
            DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    server: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsEl {
    #[doc = "Set the field `hosts`.\n"]
    pub fn set_hosts(
        mut self,
        v: impl Into<
            ListField<
                DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsEl,
            >,
        >,
    ) -> Self {
        self.hosts = Some(v.into());
        self
    }
    #[doc = "Set the field `server`.\n"]
    pub fn set_server(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.server = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsEl
{
    type O = BlockAssignable<
        DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsEl {
        DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsEl {
            hosts: core::default::Default::default(),
            server: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElRef {
        DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hosts` after provisioning.\n"]
    pub fn hosts(
        &self,
    ) -> ListRef<
        DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElHostsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.hosts", self.base))
    }
    #[doc = "Get a reference to the value of field `server` after provisioning.\n"]
    pub fn server(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElContainerdConfigElWritableCgroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterNodePoolElNodeConfigElContainerdConfigElWritableCgroupsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodePoolElNodeConfigElContainerdConfigElWritableCgroupsEl
{
    type O = BlockAssignable<
        DataContainerClusterNodePoolElNodeConfigElContainerdConfigElWritableCgroupsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElContainerdConfigElWritableCgroupsEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElContainerdConfigElWritableCgroupsEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolElNodeConfigElContainerdConfigElWritableCgroupsEl {
        DataContainerClusterNodePoolElNodeConfigElContainerdConfigElWritableCgroupsEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElContainerdConfigElWritableCgroupsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElContainerdConfigElWritableCgroupsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElContainerdConfigElWritableCgroupsElRef {
        DataContainerClusterNodePoolElNodeConfigElContainerdConfigElWritableCgroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElContainerdConfigElWritableCgroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElContainerdConfigEl { # [serde (skip_serializing_if = "Option::is_none")] private_registry_access_config : Option < ListField < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] registry_hosts : Option < ListField < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsEl > > , # [serde (skip_serializing_if = "Option::is_none")] writable_cgroups : Option < ListField < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElWritableCgroupsEl > > , }
impl DataContainerClusterNodePoolElNodeConfigElContainerdConfigEl {
    #[doc = "Set the field `private_registry_access_config`.\n"]
    pub fn set_private_registry_access_config(
        mut self,
        v : impl Into < ListField < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigEl > >,
    ) -> Self {
        self.private_registry_access_config = Some(v.into());
        self
    }
    #[doc = "Set the field `registry_hosts`.\n"]
    pub fn set_registry_hosts(
        mut self,
        v: impl Into<
            ListField<DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsEl>,
        >,
    ) -> Self {
        self.registry_hosts = Some(v.into());
        self
    }
    #[doc = "Set the field `writable_cgroups`.\n"]
    pub fn set_writable_cgroups(
        mut self,
        v: impl Into<
            ListField<
                DataContainerClusterNodePoolElNodeConfigElContainerdConfigElWritableCgroupsEl,
            >,
        >,
    ) -> Self {
        self.writable_cgroups = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElContainerdConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElContainerdConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElContainerdConfigEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElContainerdConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElContainerdConfigEl {
        DataContainerClusterNodePoolElNodeConfigElContainerdConfigEl {
            private_registry_access_config: core::default::Default::default(),
            registry_hosts: core::default::Default::default(),
            writable_cgroups: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `private_registry_access_config` after provisioning.\n"]    pub fn private_registry_access_config (& self) -> ListRef < DataContainerClusterNodePoolElNodeConfigElContainerdConfigElPrivateRegistryAccessConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_registry_access_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `registry_hosts` after provisioning.\n"]
    pub fn registry_hosts(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRegistryHostsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.registry_hosts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `writable_cgroups` after provisioning.\n"]
    pub fn writable_cgroups(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElContainerdConfigElWritableCgroupsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.writable_cgroups", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl {
    #[doc = "Set the field `effect`.\n"]
    pub fn set_effect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.effect = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl {
        DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsElRef {
        DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `effect` after provisioning.\n"]
    pub fn effect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_cache_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssd_count: Option<PrimField<f64>>,
}
impl DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl {
    #[doc = "Set the field `data_cache_count`.\n"]
    pub fn set_data_cache_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.data_cache_count = Some(v.into());
        self
    }
    #[doc = "Set the field `local_ssd_count`.\n"]
    pub fn set_local_ssd_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.local_ssd_count = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl {
    type O =
        BlockAssignable<DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl {
        DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl {
            data_cache_count: core::default::Default::default(),
            local_ssd_count: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_cache_count` after provisioning.\n"]
    pub fn data_cache_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_cache_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_ssd_count` after provisioning.\n"]
    pub fn local_ssd_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_ssd_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElFastSocketEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterNodePoolElNodeConfigElFastSocketEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElFastSocketEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElFastSocketEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElFastSocketEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElFastSocketEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElFastSocketEl {
        DataContainerClusterNodePoolElNodeConfigElFastSocketEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElFastSocketElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElFastSocketElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElFastSocketElRef {
        DataContainerClusterNodePoolElNodeConfigElFastSocketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElFastSocketElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElGcfsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterNodePoolElNodeConfigElGcfsConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElGcfsConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElGcfsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElGcfsConfigEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElGcfsConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElGcfsConfigEl {
        DataContainerClusterNodePoolElNodeConfigElGcfsConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElGcfsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElGcfsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElGcfsConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElGcfsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElGcfsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_driver_version: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
    #[doc = "Set the field `gpu_driver_version`.\n"]
    pub fn set_gpu_driver_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gpu_driver_version = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl
{
    type O = BlockAssignable<
        DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl
{}
impl
    BuildDataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl
{
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl
    {
        DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl {
            gpu_driver_version: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef
    {
        DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef { shared : shared , base : base . to_string () , }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gpu_driver_version` after provisioning.\n"]
    pub fn gpu_driver_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gpu_driver_version", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_sharing_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_shared_clients_per_gpu: Option<PrimField<f64>>,
}
impl DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    #[doc = "Set the field `gpu_sharing_strategy`.\n"]
    pub fn set_gpu_sharing_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gpu_sharing_strategy = Some(v.into());
        self
    }
    #[doc = "Set the field `max_shared_clients_per_gpu`.\n"]
    pub fn set_max_shared_clients_per_gpu(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_shared_clients_per_gpu = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl
{
    type O = BlockAssignable<
        DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
        DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl {
            gpu_sharing_strategy: core::default::Default::default(),
            max_shared_clients_per_gpu: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gpu_sharing_strategy` after provisioning.\n"]
    pub fn gpu_sharing_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gpu_sharing_strategy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_shared_clients_per_gpu` after provisioning.\n"]
    pub fn max_shared_clients_per_gpu(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_shared_clients_per_gpu", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl { # [serde (skip_serializing_if = "Option::is_none")] count : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] gpu_driver_installation_config : Option < ListField < DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] gpu_partition_size : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] gpu_sharing_config : Option < ListField < DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl > > , # [serde (rename = "type" , skip_serializing_if = "Option::is_none")] type_ : Option < PrimField < String > > , }
impl DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }
    #[doc = "Set the field `gpu_driver_installation_config`.\n"]
    pub fn set_gpu_driver_installation_config(
        mut self,
        v : impl Into < ListField < DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigEl > >,
    ) -> Self {
        self.gpu_driver_installation_config = Some(v.into());
        self
    }
    #[doc = "Set the field `gpu_partition_size`.\n"]
    pub fn set_gpu_partition_size(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gpu_partition_size = Some(v.into());
        self
    }
    #[doc = "Set the field `gpu_sharing_config`.\n"]
    pub fn set_gpu_sharing_config(
        mut self,
        v: impl Into<
            ListField<
                DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigEl,
            >,
        >,
    ) -> Self {
        self.gpu_sharing_config = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl {
        DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl {
            count: core::default::Default::default(),
            gpu_driver_installation_config: core::default::Default::default(),
            gpu_partition_size: core::default::Default::default(),
            gpu_sharing_config: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElRef {
        DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `gpu_driver_installation_config` after provisioning.\n"]    pub fn gpu_driver_installation_config (& self) -> ListRef < DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuDriverInstallationConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.gpu_driver_installation_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gpu_partition_size` after provisioning.\n"]
    pub fn gpu_partition_size(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gpu_partition_size", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gpu_sharing_config` after provisioning.\n"]
    pub fn gpu_sharing_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElGpuSharingConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gpu_sharing_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElGvnicEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterNodePoolElNodeConfigElGvnicEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElGvnicEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElGvnicEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElGvnicEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElGvnicEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElGvnicEl {
        DataContainerClusterNodePoolElNodeConfigElGvnicEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElGvnicElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElGvnicElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElGvnicElRef {
        DataContainerClusterNodePoolElNodeConfigElGvnicElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElGvnicElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_interval: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl {
    #[doc = "Set the field `maintenance_interval`.\n"]
    pub fn set_maintenance_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.maintenance_interval = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl {
        DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl {
            maintenance_interval: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyElRef {
        DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `maintenance_interval` after provisioning.\n"]
    pub fn maintenance_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_interval", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionMinimumReclaimEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    imagefs_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    imagefs_inodes_free: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodefs_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodefs_inodes_free: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pid_available: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionMinimumReclaimEl {
    #[doc = "Set the field `imagefs_available`.\n"]
    pub fn set_imagefs_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.imagefs_available = Some(v.into());
        self
    }
    #[doc = "Set the field `imagefs_inodes_free`.\n"]
    pub fn set_imagefs_inodes_free(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.imagefs_inodes_free = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_available`.\n"]
    pub fn set_memory_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.memory_available = Some(v.into());
        self
    }
    #[doc = "Set the field `nodefs_available`.\n"]
    pub fn set_nodefs_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nodefs_available = Some(v.into());
        self
    }
    #[doc = "Set the field `nodefs_inodes_free`.\n"]
    pub fn set_nodefs_inodes_free(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nodefs_inodes_free = Some(v.into());
        self
    }
    #[doc = "Set the field `pid_available`.\n"]
    pub fn set_pid_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pid_available = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionMinimumReclaimEl
{
    type O = BlockAssignable<
        DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionMinimumReclaimEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionMinimumReclaimEl {
}
impl BuildDataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionMinimumReclaimEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionMinimumReclaimEl {
        DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionMinimumReclaimEl {
            imagefs_available: core::default::Default::default(),
            imagefs_inodes_free: core::default::Default::default(),
            memory_available: core::default::Default::default(),
            nodefs_available: core::default::Default::default(),
            nodefs_inodes_free: core::default::Default::default(),
            pid_available: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionMinimumReclaimElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionMinimumReclaimElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionMinimumReclaimElRef {
        DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionMinimumReclaimElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionMinimumReclaimElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `imagefs_available` after provisioning.\n"]
    pub fn imagefs_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.imagefs_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `imagefs_inodes_free` after provisioning.\n"]
    pub fn imagefs_inodes_free(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.imagefs_inodes_free", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_available` after provisioning.\n"]
    pub fn memory_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nodefs_available` after provisioning.\n"]
    pub fn nodefs_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nodefs_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nodefs_inodes_free` after provisioning.\n"]
    pub fn nodefs_inodes_free(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nodefs_inodes_free", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pid_available` after provisioning.\n"]
    pub fn pid_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pid_available", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    imagefs_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    imagefs_inodes_free: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodefs_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodefs_inodes_free: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pid_available: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftEl {
    #[doc = "Set the field `imagefs_available`.\n"]
    pub fn set_imagefs_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.imagefs_available = Some(v.into());
        self
    }
    #[doc = "Set the field `imagefs_inodes_free`.\n"]
    pub fn set_imagefs_inodes_free(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.imagefs_inodes_free = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_available`.\n"]
    pub fn set_memory_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.memory_available = Some(v.into());
        self
    }
    #[doc = "Set the field `nodefs_available`.\n"]
    pub fn set_nodefs_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nodefs_available = Some(v.into());
        self
    }
    #[doc = "Set the field `nodefs_inodes_free`.\n"]
    pub fn set_nodefs_inodes_free(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nodefs_inodes_free = Some(v.into());
        self
    }
    #[doc = "Set the field `pid_available`.\n"]
    pub fn set_pid_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pid_available = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftEl {
    type O =
        BlockAssignable<DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftEl {
        DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftEl {
            imagefs_available: core::default::Default::default(),
            imagefs_inodes_free: core::default::Default::default(),
            memory_available: core::default::Default::default(),
            nodefs_available: core::default::Default::default(),
            nodefs_inodes_free: core::default::Default::default(),
            pid_available: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftElRef {
        DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `imagefs_available` after provisioning.\n"]
    pub fn imagefs_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.imagefs_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `imagefs_inodes_free` after provisioning.\n"]
    pub fn imagefs_inodes_free(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.imagefs_inodes_free", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_available` after provisioning.\n"]
    pub fn memory_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nodefs_available` after provisioning.\n"]
    pub fn nodefs_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nodefs_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nodefs_inodes_free` after provisioning.\n"]
    pub fn nodefs_inodes_free(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nodefs_inodes_free", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pid_available` after provisioning.\n"]
    pub fn pid_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pid_available", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    imagefs_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    imagefs_inodes_free: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodefs_available: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodefs_inodes_free: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pid_available: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl {
    #[doc = "Set the field `imagefs_available`.\n"]
    pub fn set_imagefs_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.imagefs_available = Some(v.into());
        self
    }
    #[doc = "Set the field `imagefs_inodes_free`.\n"]
    pub fn set_imagefs_inodes_free(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.imagefs_inodes_free = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_available`.\n"]
    pub fn set_memory_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.memory_available = Some(v.into());
        self
    }
    #[doc = "Set the field `nodefs_available`.\n"]
    pub fn set_nodefs_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nodefs_available = Some(v.into());
        self
    }
    #[doc = "Set the field `nodefs_inodes_free`.\n"]
    pub fn set_nodefs_inodes_free(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nodefs_inodes_free = Some(v.into());
        self
    }
    #[doc = "Set the field `pid_available`.\n"]
    pub fn set_pid_available(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pid_available = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl
{
    type O = BlockAssignable<
        DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl
{}
impl BuildDataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl {
        DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl {
            imagefs_available: core::default::Default::default(),
            imagefs_inodes_free: core::default::Default::default(),
            memory_available: core::default::Default::default(),
            nodefs_available: core::default::Default::default(),
            nodefs_inodes_free: core::default::Default::default(),
            pid_available: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftGracePeriodElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftGracePeriodElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftGracePeriodElRef {
        DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftGracePeriodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftGracePeriodElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `imagefs_available` after provisioning.\n"]
    pub fn imagefs_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.imagefs_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `imagefs_inodes_free` after provisioning.\n"]
    pub fn imagefs_inodes_free(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.imagefs_inodes_free", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_available` after provisioning.\n"]
    pub fn memory_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nodefs_available` after provisioning.\n"]
    pub fn nodefs_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nodefs_available", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nodefs_inodes_free` after provisioning.\n"]
    pub fn nodefs_inodes_free(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nodefs_inodes_free", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pid_available` after provisioning.\n"]
    pub fn pid_available(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pid_available", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElKubeletConfigElMemoryManagerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolElNodeConfigElKubeletConfigElMemoryManagerEl {
    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElKubeletConfigElMemoryManagerEl {
    type O =
        BlockAssignable<DataContainerClusterNodePoolElNodeConfigElKubeletConfigElMemoryManagerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElKubeletConfigElMemoryManagerEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElKubeletConfigElMemoryManagerEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElKubeletConfigElMemoryManagerEl {
        DataContainerClusterNodePoolElNodeConfigElKubeletConfigElMemoryManagerEl {
            policy: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElKubeletConfigElMemoryManagerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElKubeletConfigElMemoryManagerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElKubeletConfigElMemoryManagerElRef {
        DataContainerClusterNodePoolElNodeConfigElKubeletConfigElMemoryManagerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElKubeletConfigElMemoryManagerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElKubeletConfigElTopologyManagerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolElNodeConfigElKubeletConfigElTopologyManagerEl {
    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy = Some(v.into());
        self
    }
    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElKubeletConfigElTopologyManagerEl {
    type O =
        BlockAssignable<DataContainerClusterNodePoolElNodeConfigElKubeletConfigElTopologyManagerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElKubeletConfigElTopologyManagerEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElKubeletConfigElTopologyManagerEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolElNodeConfigElKubeletConfigElTopologyManagerEl {
        DataContainerClusterNodePoolElNodeConfigElKubeletConfigElTopologyManagerEl {
            policy: core::default::Default::default(),
            scope: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElKubeletConfigElTopologyManagerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElKubeletConfigElTopologyManagerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElKubeletConfigElTopologyManagerElRef {
        DataContainerClusterNodePoolElNodeConfigElKubeletConfigElTopologyManagerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElKubeletConfigElTopologyManagerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElKubeletConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_unsafe_sysctls: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_log_max_files: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_log_max_size: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_cfs_quota: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_cfs_quota_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_manager_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eviction_max_pod_grace_period_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eviction_minimum_reclaim: Option<
        ListField<
            DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionMinimumReclaimEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    eviction_soft:
        Option<ListField<DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eviction_soft_grace_period: Option<
        ListField<
            DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_gc_high_threshold_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_gc_low_threshold_percent: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_maximum_gc_age: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_minimum_gc_age: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insecure_kubelet_readonly_port_enabled: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_parallel_image_pulls: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_manager:
        Option<ListField<DataContainerClusterNodePoolElNodeConfigElKubeletConfigElMemoryManagerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_pids_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    single_process_oom_kill: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topology_manager: Option<
        ListField<DataContainerClusterNodePoolElNodeConfigElKubeletConfigElTopologyManagerEl>,
    >,
}
impl DataContainerClusterNodePoolElNodeConfigElKubeletConfigEl {
    #[doc = "Set the field `allowed_unsafe_sysctls`.\n"]
    pub fn set_allowed_unsafe_sysctls(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.allowed_unsafe_sysctls = Some(v.into());
        self
    }
    #[doc = "Set the field `container_log_max_files`.\n"]
    pub fn set_container_log_max_files(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.container_log_max_files = Some(v.into());
        self
    }
    #[doc = "Set the field `container_log_max_size`.\n"]
    pub fn set_container_log_max_size(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container_log_max_size = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu_cfs_quota`.\n"]
    pub fn set_cpu_cfs_quota(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.cpu_cfs_quota = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu_cfs_quota_period`.\n"]
    pub fn set_cpu_cfs_quota_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cpu_cfs_quota_period = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu_manager_policy`.\n"]
    pub fn set_cpu_manager_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cpu_manager_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `eviction_max_pod_grace_period_seconds`.\n"]
    pub fn set_eviction_max_pod_grace_period_seconds(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.eviction_max_pod_grace_period_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `eviction_minimum_reclaim`.\n"]
    pub fn set_eviction_minimum_reclaim(
        mut self,
        v: impl Into<
            ListField<
                DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionMinimumReclaimEl,
            >,
        >,
    ) -> Self {
        self.eviction_minimum_reclaim = Some(v.into());
        self
    }
    #[doc = "Set the field `eviction_soft`.\n"]
    pub fn set_eviction_soft(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftEl>>,
    ) -> Self {
        self.eviction_soft = Some(v.into());
        self
    }
    #[doc = "Set the field `eviction_soft_grace_period`.\n"]
    pub fn set_eviction_soft_grace_period(
        mut self,
        v: impl Into<
            ListField<
                DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftGracePeriodEl,
            >,
        >,
    ) -> Self {
        self.eviction_soft_grace_period = Some(v.into());
        self
    }
    #[doc = "Set the field `image_gc_high_threshold_percent`.\n"]
    pub fn set_image_gc_high_threshold_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.image_gc_high_threshold_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `image_gc_low_threshold_percent`.\n"]
    pub fn set_image_gc_low_threshold_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.image_gc_low_threshold_percent = Some(v.into());
        self
    }
    #[doc = "Set the field `image_maximum_gc_age`.\n"]
    pub fn set_image_maximum_gc_age(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_maximum_gc_age = Some(v.into());
        self
    }
    #[doc = "Set the field `image_minimum_gc_age`.\n"]
    pub fn set_image_minimum_gc_age(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_minimum_gc_age = Some(v.into());
        self
    }
    #[doc = "Set the field `insecure_kubelet_readonly_port_enabled`.\n"]
    pub fn set_insecure_kubelet_readonly_port_enabled(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.insecure_kubelet_readonly_port_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `max_parallel_image_pulls`.\n"]
    pub fn set_max_parallel_image_pulls(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_parallel_image_pulls = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_manager`.\n"]
    pub fn set_memory_manager(
        mut self,
        v: impl Into<
            ListField<DataContainerClusterNodePoolElNodeConfigElKubeletConfigElMemoryManagerEl>,
        >,
    ) -> Self {
        self.memory_manager = Some(v.into());
        self
    }
    #[doc = "Set the field `pod_pids_limit`.\n"]
    pub fn set_pod_pids_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.pod_pids_limit = Some(v.into());
        self
    }
    #[doc = "Set the field `single_process_oom_kill`.\n"]
    pub fn set_single_process_oom_kill(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.single_process_oom_kill = Some(v.into());
        self
    }
    #[doc = "Set the field `topology_manager`.\n"]
    pub fn set_topology_manager(
        mut self,
        v: impl Into<
            ListField<DataContainerClusterNodePoolElNodeConfigElKubeletConfigElTopologyManagerEl>,
        >,
    ) -> Self {
        self.topology_manager = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElKubeletConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElKubeletConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElKubeletConfigEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElKubeletConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElKubeletConfigEl {
        DataContainerClusterNodePoolElNodeConfigElKubeletConfigEl {
            allowed_unsafe_sysctls: core::default::Default::default(),
            container_log_max_files: core::default::Default::default(),
            container_log_max_size: core::default::Default::default(),
            cpu_cfs_quota: core::default::Default::default(),
            cpu_cfs_quota_period: core::default::Default::default(),
            cpu_manager_policy: core::default::Default::default(),
            eviction_max_pod_grace_period_seconds: core::default::Default::default(),
            eviction_minimum_reclaim: core::default::Default::default(),
            eviction_soft: core::default::Default::default(),
            eviction_soft_grace_period: core::default::Default::default(),
            image_gc_high_threshold_percent: core::default::Default::default(),
            image_gc_low_threshold_percent: core::default::Default::default(),
            image_maximum_gc_age: core::default::Default::default(),
            image_minimum_gc_age: core::default::Default::default(),
            insecure_kubelet_readonly_port_enabled: core::default::Default::default(),
            max_parallel_image_pulls: core::default::Default::default(),
            memory_manager: core::default::Default::default(),
            pod_pids_limit: core::default::Default::default(),
            single_process_oom_kill: core::default::Default::default(),
            topology_manager: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElKubeletConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElKubeletConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElKubeletConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElKubeletConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElKubeletConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_unsafe_sysctls` after provisioning.\n"]
    pub fn allowed_unsafe_sysctls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_unsafe_sysctls", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `container_log_max_files` after provisioning.\n"]
    pub fn container_log_max_files(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_log_max_files", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `container_log_max_size` after provisioning.\n"]
    pub fn container_log_max_size(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_log_max_size", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_cfs_quota` after provisioning.\n"]
    pub fn cpu_cfs_quota(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_cfs_quota", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_cfs_quota_period` after provisioning.\n"]
    pub fn cpu_cfs_quota_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_cfs_quota_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_manager_policy` after provisioning.\n"]
    pub fn cpu_manager_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_manager_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `eviction_max_pod_grace_period_seconds` after provisioning.\n"]
    pub fn eviction_max_pod_grace_period_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.eviction_max_pod_grace_period_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `eviction_minimum_reclaim` after provisioning.\n"]
    pub fn eviction_minimum_reclaim(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionMinimumReclaimElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.eviction_minimum_reclaim", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `eviction_soft` after provisioning.\n"]
    pub fn eviction_soft(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.eviction_soft", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `eviction_soft_grace_period` after provisioning.\n"]
    pub fn eviction_soft_grace_period(
        &self,
    ) -> ListRef<
        DataContainerClusterNodePoolElNodeConfigElKubeletConfigElEvictionSoftGracePeriodElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.eviction_soft_grace_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_gc_high_threshold_percent` after provisioning.\n"]
    pub fn image_gc_high_threshold_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_gc_high_threshold_percent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_gc_low_threshold_percent` after provisioning.\n"]
    pub fn image_gc_low_threshold_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_gc_low_threshold_percent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_maximum_gc_age` after provisioning.\n"]
    pub fn image_maximum_gc_age(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_maximum_gc_age", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_minimum_gc_age` after provisioning.\n"]
    pub fn image_minimum_gc_age(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_minimum_gc_age", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `insecure_kubelet_readonly_port_enabled` after provisioning.\n"]
    pub fn insecure_kubelet_readonly_port_enabled(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.insecure_kubelet_readonly_port_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_parallel_image_pulls` after provisioning.\n"]
    pub fn max_parallel_image_pulls(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_parallel_image_pulls", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_manager` after provisioning.\n"]
    pub fn memory_manager(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElKubeletConfigElMemoryManagerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.memory_manager", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pod_pids_limit` after provisioning.\n"]
    pub fn pod_pids_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pod_pids_limit", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `single_process_oom_kill` after provisioning.\n"]
    pub fn single_process_oom_kill(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.single_process_oom_kill", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `topology_manager` after provisioning.\n"]
    pub fn topology_manager(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElKubeletConfigElTopologyManagerElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.topology_manager", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_ptp_kvm_time_sync: Option<PrimField<bool>>,
}
impl DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl {
    #[doc = "Set the field `enable_ptp_kvm_time_sync`.\n"]
    pub fn set_enable_ptp_kvm_time_sync(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_ptp_kvm_time_sync = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl
{
    type O = BlockAssignable<
        DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl {
        DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl {
            enable_ptp_kvm_time_sync: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElAccurateTimeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElAccurateTimeConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElAccurateTimeConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElAccurateTimeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElAccurateTimeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_ptp_kvm_time_sync` after provisioning.\n"]
    pub fn enable_ptp_kvm_time_sync(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_ptp_kvm_time_sync", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElHugepagesConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hugepage_size_1g: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hugepage_size_2m: Option<PrimField<f64>>,
}
impl DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElHugepagesConfigEl {
    #[doc = "Set the field `hugepage_size_1g`.\n"]
    pub fn set_hugepage_size_1g(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hugepage_size_1g = Some(v.into());
        self
    }
    #[doc = "Set the field `hugepage_size_2m`.\n"]
    pub fn set_hugepage_size_2m(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hugepage_size_2m = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElHugepagesConfigEl
{
    type O = BlockAssignable<
        DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElHugepagesConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElHugepagesConfigEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElHugepagesConfigEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElHugepagesConfigEl {
        DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElHugepagesConfigEl {
            hugepage_size_1g: core::default::Default::default(),
            hugepage_size_2m: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElHugepagesConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElHugepagesConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElHugepagesConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElHugepagesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElHugepagesConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hugepage_size_1g` after provisioning.\n"]
    pub fn hugepage_size_1g(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hugepage_size_1g", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hugepage_size_2m` after provisioning.\n"]
    pub fn hugepage_size_2m(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hugepage_size_2m", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl {
    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl
{
    type O = BlockAssignable<
        DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl
{}
impl BuildDataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl {
        DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl {
            policy: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingElRef
    {
        DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    swap_size_gib: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    swap_size_percent: Option<PrimField<f64>>,
}
impl DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl {
    #[doc = "Set the field `swap_size_gib`.\n"]
    pub fn set_swap_size_gib(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.swap_size_gib = Some(v.into());
        self
    }
    #[doc = "Set the field `swap_size_percent`.\n"]
    pub fn set_swap_size_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.swap_size_percent = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl
{
    type O = BlockAssignable<
        DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl
{}
impl BuildDataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl
    {
        DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl {
            swap_size_gib: core::default::Default::default(),
            swap_size_percent: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileElRef
    {
        DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileElRef { shared : shared , base : base . to_string () , }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `swap_size_gib` after provisioning.\n"]
    pub fn swap_size_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.swap_size_gib", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `swap_size_percent` after provisioning.\n"]
    pub fn swap_size_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.swap_size_percent", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_count: Option<PrimField<f64>>,
}
impl DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl { # [doc = "Set the field `disk_count`.\n"] pub fn set_disk_count (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . disk_count = Some (v . into ()) ; self } }
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl { type O = BlockAssignable < DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl
{}
impl BuildDataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl { pub fn build (self) -> DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl { DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl { disk_count : core :: default :: Default :: default () , } } }
pub struct DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileElRef { fn new (shared : StackShared , base : String) -> DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileElRef { DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileElRef { shared : shared , base : base . to_string () , } } }
impl DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `disk_count` after provisioning.\n"] pub fn disk_count (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.disk_count" , self . base)) } }
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
}
impl DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl
{
    type O = BlockAssignable<
        DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl
{}
impl
    BuildDataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl
{
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl
    {
        DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl {
            disabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigElRef
    {
        DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigElRef { shared : shared , base : base . to_string () , }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    swap_size_gib: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    swap_size_percent: Option<PrimField<f64>>,
}
impl DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl { # [doc = "Set the field `swap_size_gib`.\n"] pub fn set_swap_size_gib (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . swap_size_gib = Some (v . into ()) ; self } # [doc = "Set the field `swap_size_percent`.\n"] pub fn set_swap_size_percent (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . swap_size_percent = Some (v . into ()) ; self } }
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl { type O = BlockAssignable < DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl
{}
impl BuildDataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl { pub fn build (self) -> DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl { DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl { swap_size_gib : core :: default :: Default :: default () , swap_size_percent : core :: default :: Default :: default () , } } }
pub struct DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileElRef { fn new (shared : StackShared , base : String) -> DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileElRef { DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileElRef { shared : shared , base : base . to_string () , } } }
impl DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `swap_size_gib` after provisioning.\n"] pub fn swap_size_gib (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.swap_size_gib" , self . base)) } # [doc = "Get a reference to the value of field `swap_size_percent` after provisioning.\n"] pub fn swap_size_percent (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.swap_size_percent" , self . base)) } }
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigEl { # [serde (skip_serializing_if = "Option::is_none")] boot_disk_profile : Option < ListField < DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl > > , # [serde (skip_serializing_if = "Option::is_none")] dedicated_local_ssd_profile : Option < ListField < DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl > > , # [serde (skip_serializing_if = "Option::is_none")] enabled : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] encryption_config : Option < ListField < DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] ephemeral_local_ssd_profile : Option < ListField < DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl > > , }
impl DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigEl {
    #[doc = "Set the field `boot_disk_profile`.\n"]
    pub fn set_boot_disk_profile(
        mut self,
        v : impl Into < ListField < DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileEl > >,
    ) -> Self {
        self.boot_disk_profile = Some(v.into());
        self
    }
    #[doc = "Set the field `dedicated_local_ssd_profile`.\n"]
    pub fn set_dedicated_local_ssd_profile(
        mut self,
        v : impl Into < ListField < DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileEl > >,
    ) -> Self {
        self.dedicated_local_ssd_profile = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v : impl Into < ListField < DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigEl > >,
    ) -> Self {
        self.encryption_config = Some(v.into());
        self
    }
    #[doc = "Set the field `ephemeral_local_ssd_profile`.\n"]
    pub fn set_ephemeral_local_ssd_profile(
        mut self,
        v : impl Into < ListField < DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileEl > >,
    ) -> Self {
        self.ephemeral_local_ssd_profile = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigEl {
    type O =
        BlockAssignable<DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigEl {
        DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigEl {
            boot_disk_profile: core::default::Default::default(),
            dedicated_local_ssd_profile: core::default::Default::default(),
            enabled: core::default::Default::default(),
            encryption_config: core::default::Default::default(),
            ephemeral_local_ssd_profile: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `boot_disk_profile` after provisioning.\n"]
    pub fn boot_disk_profile(
        &self,
    ) -> ListRef<
        DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElBootDiskProfileElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.boot_disk_profile", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dedicated_local_ssd_profile` after provisioning.\n"]    pub fn dedicated_local_ssd_profile (& self) -> ListRef < DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElDedicatedLocalSsdProfileElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.dedicated_local_ssd_profile", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]    pub fn encryption_config (& self) -> ListRef < DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEncryptionConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ephemeral_local_ssd_profile` after provisioning.\n"]    pub fn ephemeral_local_ssd_profile (& self) -> ListRef < DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElEphemeralLocalSsdProfileElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.ephemeral_local_ssd_profile", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accurate_time_config: Option<
        ListField<DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    cgroup_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hugepages_config: Option<
        ListField<DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElHugepagesConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_kernel_module_loading: Option<
        ListField<
            DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    swap_config:
        Option<ListField<DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sysctls: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transparent_hugepage_defrag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transparent_hugepage_enabled: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl {
    #[doc = "Set the field `accurate_time_config`.\n"]
    pub fn set_accurate_time_config(
        mut self,
        v: impl Into<
            ListField<
                DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElAccurateTimeConfigEl,
            >,
        >,
    ) -> Self {
        self.accurate_time_config = Some(v.into());
        self
    }
    #[doc = "Set the field `cgroup_mode`.\n"]
    pub fn set_cgroup_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cgroup_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `hugepages_config`.\n"]
    pub fn set_hugepages_config(
        mut self,
        v: impl Into<
            ListField<DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElHugepagesConfigEl>,
        >,
    ) -> Self {
        self.hugepages_config = Some(v.into());
        self
    }
    #[doc = "Set the field `node_kernel_module_loading`.\n"]
    pub fn set_node_kernel_module_loading(
        mut self,
        v : impl Into < ListField < DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl > >,
    ) -> Self {
        self.node_kernel_module_loading = Some(v.into());
        self
    }
    #[doc = "Set the field `swap_config`.\n"]
    pub fn set_swap_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigEl>>,
    ) -> Self {
        self.swap_config = Some(v.into());
        self
    }
    #[doc = "Set the field `sysctls`.\n"]
    pub fn set_sysctls(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.sysctls = Some(v.into());
        self
    }
    #[doc = "Set the field `transparent_hugepage_defrag`.\n"]
    pub fn set_transparent_hugepage_defrag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.transparent_hugepage_defrag = Some(v.into());
        self
    }
    #[doc = "Set the field `transparent_hugepage_enabled`.\n"]
    pub fn set_transparent_hugepage_enabled(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.transparent_hugepage_enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl {
        DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl {
            accurate_time_config: core::default::Default::default(),
            cgroup_mode: core::default::Default::default(),
            hugepages_config: core::default::Default::default(),
            node_kernel_module_loading: core::default::Default::default(),
            swap_config: core::default::Default::default(),
            sysctls: core::default::Default::default(),
            transparent_hugepage_defrag: core::default::Default::default(),
            transparent_hugepage_enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accurate_time_config` after provisioning.\n"]
    pub fn accurate_time_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElAccurateTimeConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.accurate_time_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cgroup_mode` after provisioning.\n"]
    pub fn cgroup_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cgroup_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `hugepages_config` after provisioning.\n"]
    pub fn hugepages_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElHugepagesConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hugepages_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_kernel_module_loading` after provisioning.\n"]
    pub fn node_kernel_module_loading(
        &self,
    ) -> ListRef<
        DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElNodeKernelModuleLoadingElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_kernel_module_loading", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `swap_config` after provisioning.\n"]
    pub fn swap_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElSwapConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.swap_config", self.base))
    }
    #[doc = "Get a reference to the value of field `sysctls` after provisioning.\n"]
    pub fn sysctls(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.sysctls", self.base))
    }
    #[doc = "Get a reference to the value of field `transparent_hugepage_defrag` after provisioning.\n"]
    pub fn transparent_hugepage_defrag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transparent_hugepage_defrag", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `transparent_hugepage_enabled` after provisioning.\n"]
    pub fn transparent_hugepage_enabled(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transparent_hugepage_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssd_count: Option<PrimField<f64>>,
}
impl DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl {
    #[doc = "Set the field `local_ssd_count`.\n"]
    pub fn set_local_ssd_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.local_ssd_count = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl {
        DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl {
            local_ssd_count: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `local_ssd_count` after provisioning.\n"]
    pub fn local_ssd_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_ssd_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElReservationAffinityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    consume_reservation_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}
impl DataContainerClusterNodePoolElNodeConfigElReservationAffinityEl {
    #[doc = "Set the field `consume_reservation_type`.\n"]
    pub fn set_consume_reservation_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.consume_reservation_type = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElReservationAffinityEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElReservationAffinityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElReservationAffinityEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElReservationAffinityEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElReservationAffinityEl {
        DataContainerClusterNodePoolElNodeConfigElReservationAffinityEl {
            consume_reservation_type: core::default::Default::default(),
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElReservationAffinityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElReservationAffinityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElReservationAffinityElRef {
        DataContainerClusterNodePoolElNodeConfigElReservationAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElReservationAffinityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `consume_reservation_type` after provisioning.\n"]
    pub fn consume_reservation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consume_reservation_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElSandboxConfigEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolElNodeConfigElSandboxConfigEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElSandboxConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElSandboxConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElSandboxConfigEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElSandboxConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElSandboxConfigEl {
        DataContainerClusterNodePoolElNodeConfigElSandboxConfigEl {
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElSandboxConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElSandboxConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElSandboxConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElSandboxConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElSandboxConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElSecondaryBootDisksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolElNodeConfigElSecondaryBootDisksEl {
    #[doc = "Set the field `disk_image`.\n"]
    pub fn set_disk_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_image = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElSecondaryBootDisksEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElSecondaryBootDisksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElSecondaryBootDisksEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElSecondaryBootDisksEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElSecondaryBootDisksEl {
        DataContainerClusterNodePoolElNodeConfigElSecondaryBootDisksEl {
            disk_image: core::default::Default::default(),
            mode: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElSecondaryBootDisksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElSecondaryBootDisksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElSecondaryBootDisksElRef {
        DataContainerClusterNodePoolElNodeConfigElSecondaryBootDisksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElSecondaryBootDisksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk_image` after provisioning.\n"]
    pub fn disk_image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_image", self.base))
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_integrity_monitoring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_secure_boot: Option<PrimField<bool>>,
}
impl DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl {
    #[doc = "Set the field `enable_integrity_monitoring`.\n"]
    pub fn set_enable_integrity_monitoring(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_secure_boot`.\n"]
    pub fn set_enable_secure_boot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_secure_boot = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl {
        DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\n"]
    pub fn enable_integrity_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_integrity_monitoring", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_secure_boot` after provisioning.\n"]
    pub fn enable_secure_boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_secure_boot", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<ListField<PrimField<String>>>,
}
impl DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operator = Some(v.into());
        self
    }
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl {
    type O =
        BlockAssignable<DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl {
        DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl {
            key: core::default::Default::default(),
            operator: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityElRef {
        DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    min_node_cpus: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_affinity: Option<
        SetField<DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl>,
    >,
}
impl DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl {
    #[doc = "Set the field `min_node_cpus`.\n"]
    pub fn set_min_node_cpus(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_node_cpus = Some(v.into());
        self
    }
    #[doc = "Set the field `node_affinity`.\n"]
    pub fn set_node_affinity(
        mut self,
        v: impl Into<
            SetField<DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityEl>,
        >,
    ) -> Self {
        self.node_affinity = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl {
        DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl {
            min_node_cpus: core::default::Default::default(),
            node_affinity: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `min_node_cpus` after provisioning.\n"]
    pub fn min_node_cpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_node_cpus", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_affinity` after provisioning.\n"]
    pub fn node_affinity(
        &self,
    ) -> SetRef<DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElNodeAffinityElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.node_affinity", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElTaintEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolElNodeConfigElTaintEl {
    #[doc = "Set the field `effect`.\n"]
    pub fn set_effect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.effect = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElTaintEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElTaintEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElTaintEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElTaintEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElTaintEl {
        DataContainerClusterNodePoolElNodeConfigElTaintEl {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElTaintElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElTaintElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElTaintElRef {
        DataContainerClusterNodePoolElNodeConfigElTaintElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElTaintElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `effect` after provisioning.\n"]
    pub fn effect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElWindowsNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    osversion: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolElNodeConfigElWindowsNodeConfigEl {
    #[doc = "Set the field `osversion`.\n"]
    pub fn set_osversion(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.osversion = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElWindowsNodeConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElWindowsNodeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElWindowsNodeConfigEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElWindowsNodeConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElWindowsNodeConfigEl {
        DataContainerClusterNodePoolElNodeConfigElWindowsNodeConfigEl {
            osversion: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElWindowsNodeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElWindowsNodeConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElWindowsNodeConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElWindowsNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElWindowsNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `osversion` after provisioning.\n"]
    pub fn osversion(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.osversion", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl {
        DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl {
            mode: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_machine_features:
        Option<ListField<DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk: Option<ListField<DataContainerClusterNodePoolElNodeConfigElBootDiskEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_kms_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential_nodes:
        Option<ListField<DataContainerClusterNodePoolElNodeConfigElConfidentialNodesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    containerd_config:
        Option<ListField<DataContainerClusterNodePoolElNodeConfigElContainerdConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_taints:
        Option<ListField<DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_confidential_storage: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ephemeral_storage_local_ssd_config: Option<
        ListField<DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    fast_socket: Option<ListField<DataContainerClusterNodePoolElNodeConfigElFastSocketEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flex_start: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcfs_config: Option<ListField<DataContainerClusterNodePoolElNodeConfigElGcfsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_accelerator:
        Option<ListField<DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gvnic: Option<ListField<DataContainerClusterNodePoolElNodeConfigElGvnicEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_maintenance_policy:
        Option<ListField<DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kubelet_config: Option<ListField<DataContainerClusterNodePoolElNodeConfigElKubeletConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    linux_node_config:
        Option<ListField<DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_nvme_ssd_block_config:
        Option<ListField<DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssd_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssd_encryption_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_variant: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_run_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_cpu_platform: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth_scopes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preemptible: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reservation_affinity:
        Option<ListField<DataContainerClusterNodePoolElNodeConfigElReservationAffinityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sandbox_config: Option<ListField<DataContainerClusterNodePoolElNodeConfigElSandboxConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_boot_disks:
        Option<ListField<DataContainerClusterNodePoolElNodeConfigElSecondaryBootDisksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shielded_instance_config:
        Option<ListField<DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sole_tenant_config:
        Option<ListField<DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_pools: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    taint: Option<ListField<DataContainerClusterNodePoolElNodeConfigElTaintEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    windows_node_config:
        Option<ListField<DataContainerClusterNodePoolElNodeConfigElWindowsNodeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workload_metadata_config:
        Option<ListField<DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl>>,
}
impl DataContainerClusterNodePoolElNodeConfigEl {
    #[doc = "Set the field `advanced_machine_features`.\n"]
    pub fn set_advanced_machine_features(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesEl>>,
    ) -> Self {
        self.advanced_machine_features = Some(v.into());
        self
    }
    #[doc = "Set the field `boot_disk`.\n"]
    pub fn set_boot_disk(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElBootDiskEl>>,
    ) -> Self {
        self.boot_disk = Some(v.into());
        self
    }
    #[doc = "Set the field `boot_disk_kms_key`.\n"]
    pub fn set_boot_disk_kms_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.boot_disk_kms_key = Some(v.into());
        self
    }
    #[doc = "Set the field `confidential_nodes`.\n"]
    pub fn set_confidential_nodes(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElConfidentialNodesEl>>,
    ) -> Self {
        self.confidential_nodes = Some(v.into());
        self
    }
    #[doc = "Set the field `containerd_config`.\n"]
    pub fn set_containerd_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElContainerdConfigEl>>,
    ) -> Self {
        self.containerd_config = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_size_gb`.\n"]
    pub fn set_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_type`.\n"]
    pub fn set_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_type = Some(v.into());
        self
    }
    #[doc = "Set the field `effective_taints`.\n"]
    pub fn set_effective_taints(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsEl>>,
    ) -> Self {
        self.effective_taints = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_confidential_storage`.\n"]
    pub fn set_enable_confidential_storage(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_confidential_storage = Some(v.into());
        self
    }
    #[doc = "Set the field `ephemeral_storage_local_ssd_config`.\n"]
    pub fn set_ephemeral_storage_local_ssd_config(
        mut self,
        v: impl Into<
            ListField<DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigEl>,
        >,
    ) -> Self {
        self.ephemeral_storage_local_ssd_config = Some(v.into());
        self
    }
    #[doc = "Set the field `fast_socket`.\n"]
    pub fn set_fast_socket(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElFastSocketEl>>,
    ) -> Self {
        self.fast_socket = Some(v.into());
        self
    }
    #[doc = "Set the field `flex_start`.\n"]
    pub fn set_flex_start(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.flex_start = Some(v.into());
        self
    }
    #[doc = "Set the field `gcfs_config`.\n"]
    pub fn set_gcfs_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElGcfsConfigEl>>,
    ) -> Self {
        self.gcfs_config = Some(v.into());
        self
    }
    #[doc = "Set the field `guest_accelerator`.\n"]
    pub fn set_guest_accelerator(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorEl>>,
    ) -> Self {
        self.guest_accelerator = Some(v.into());
        self
    }
    #[doc = "Set the field `gvnic`.\n"]
    pub fn set_gvnic(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElGvnicEl>>,
    ) -> Self {
        self.gvnic = Some(v.into());
        self
    }
    #[doc = "Set the field `host_maintenance_policy`.\n"]
    pub fn set_host_maintenance_policy(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyEl>>,
    ) -> Self {
        self.host_maintenance_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `image_type`.\n"]
    pub fn set_image_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_type = Some(v.into());
        self
    }
    #[doc = "Set the field `kubelet_config`.\n"]
    pub fn set_kubelet_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElKubeletConfigEl>>,
    ) -> Self {
        self.kubelet_config = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `linux_node_config`.\n"]
    pub fn set_linux_node_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigEl>>,
    ) -> Self {
        self.linux_node_config = Some(v.into());
        self
    }
    #[doc = "Set the field `local_nvme_ssd_block_config`.\n"]
    pub fn set_local_nvme_ssd_block_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigEl>>,
    ) -> Self {
        self.local_nvme_ssd_block_config = Some(v.into());
        self
    }
    #[doc = "Set the field `local_ssd_count`.\n"]
    pub fn set_local_ssd_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.local_ssd_count = Some(v.into());
        self
    }
    #[doc = "Set the field `local_ssd_encryption_mode`.\n"]
    pub fn set_local_ssd_encryption_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_ssd_encryption_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `logging_variant`.\n"]
    pub fn set_logging_variant(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.logging_variant = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
    #[doc = "Set the field `max_run_duration`.\n"]
    pub fn set_max_run_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_run_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `min_cpu_platform`.\n"]
    pub fn set_min_cpu_platform(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_cpu_platform = Some(v.into());
        self
    }
    #[doc = "Set the field `node_group`.\n"]
    pub fn set_node_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_group = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth_scopes`.\n"]
    pub fn set_oauth_scopes(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.oauth_scopes = Some(v.into());
        self
    }
    #[doc = "Set the field `preemptible`.\n"]
    pub fn set_preemptible(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preemptible = Some(v.into());
        self
    }
    #[doc = "Set the field `reservation_affinity`.\n"]
    pub fn set_reservation_affinity(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElReservationAffinityEl>>,
    ) -> Self {
        self.reservation_affinity = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_labels`.\n"]
    pub fn set_resource_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_manager_tags`.\n"]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `sandbox_config`.\n"]
    pub fn set_sandbox_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElSandboxConfigEl>>,
    ) -> Self {
        self.sandbox_config = Some(v.into());
        self
    }
    #[doc = "Set the field `secondary_boot_disks`.\n"]
    pub fn set_secondary_boot_disks(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElSecondaryBootDisksEl>>,
    ) -> Self {
        self.secondary_boot_disks = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigEl>>,
    ) -> Self {
        self.shielded_instance_config = Some(v.into());
        self
    }
    #[doc = "Set the field `sole_tenant_config`.\n"]
    pub fn set_sole_tenant_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigEl>>,
    ) -> Self {
        self.sole_tenant_config = Some(v.into());
        self
    }
    #[doc = "Set the field `spot`.\n"]
    pub fn set_spot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.spot = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_pools`.\n"]
    pub fn set_storage_pools(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.storage_pools = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
    #[doc = "Set the field `taint`.\n"]
    pub fn set_taint(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElTaintEl>>,
    ) -> Self {
        self.taint = Some(v.into());
        self
    }
    #[doc = "Set the field `windows_node_config`.\n"]
    pub fn set_windows_node_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElWindowsNodeConfigEl>>,
    ) -> Self {
        self.windows_node_config = Some(v.into());
        self
    }
    #[doc = "Set the field `workload_metadata_config`.\n"]
    pub fn set_workload_metadata_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigEl>>,
    ) -> Self {
        self.workload_metadata_config = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeConfigEl {}
impl BuildDataContainerClusterNodePoolElNodeConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeConfigEl {
        DataContainerClusterNodePoolElNodeConfigEl {
            advanced_machine_features: core::default::Default::default(),
            boot_disk: core::default::Default::default(),
            boot_disk_kms_key: core::default::Default::default(),
            confidential_nodes: core::default::Default::default(),
            containerd_config: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            effective_taints: core::default::Default::default(),
            enable_confidential_storage: core::default::Default::default(),
            ephemeral_storage_local_ssd_config: core::default::Default::default(),
            fast_socket: core::default::Default::default(),
            flex_start: core::default::Default::default(),
            gcfs_config: core::default::Default::default(),
            guest_accelerator: core::default::Default::default(),
            gvnic: core::default::Default::default(),
            host_maintenance_policy: core::default::Default::default(),
            image_type: core::default::Default::default(),
            kubelet_config: core::default::Default::default(),
            labels: core::default::Default::default(),
            linux_node_config: core::default::Default::default(),
            local_nvme_ssd_block_config: core::default::Default::default(),
            local_ssd_count: core::default::Default::default(),
            local_ssd_encryption_mode: core::default::Default::default(),
            logging_variant: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            max_run_duration: core::default::Default::default(),
            metadata: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            node_group: core::default::Default::default(),
            oauth_scopes: core::default::Default::default(),
            preemptible: core::default::Default::default(),
            reservation_affinity: core::default::Default::default(),
            resource_labels: core::default::Default::default(),
            resource_manager_tags: core::default::Default::default(),
            sandbox_config: core::default::Default::default(),
            secondary_boot_disks: core::default::Default::default(),
            service_account: core::default::Default::default(),
            shielded_instance_config: core::default::Default::default(),
            sole_tenant_config: core::default::Default::default(),
            spot: core::default::Default::default(),
            storage_pools: core::default::Default::default(),
            tags: core::default::Default::default(),
            taint: core::default::Default::default(),
            windows_node_config: core::default::Default::default(),
            workload_metadata_config: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolElNodeConfigElRef {
        DataContainerClusterNodePoolElNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `advanced_machine_features` after provisioning.\n"]
    pub fn advanced_machine_features(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElAdvancedMachineFeaturesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_machine_features", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `boot_disk` after provisioning.\n"]
    pub fn boot_disk(&self) -> ListRef<DataContainerClusterNodePoolElNodeConfigElBootDiskElRef> {
        ListRef::new(self.shared().clone(), format!("{}.boot_disk", self.base))
    }
    #[doc = "Get a reference to the value of field `boot_disk_kms_key` after provisioning.\n"]
    pub fn boot_disk_kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_kms_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `confidential_nodes` after provisioning.\n"]
    pub fn confidential_nodes(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElConfidentialNodesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.confidential_nodes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `containerd_config` after provisioning.\n"]
    pub fn containerd_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElContainerdConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.containerd_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\n"]
    pub fn disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\n"]
    pub fn disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }
    #[doc = "Get a reference to the value of field `effective_taints` after provisioning.\n"]
    pub fn effective_taints(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElEffectiveTaintsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.effective_taints", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_confidential_storage` after provisioning.\n"]
    pub fn enable_confidential_storage(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_confidential_storage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ephemeral_storage_local_ssd_config` after provisioning.\n"]
    pub fn ephemeral_storage_local_ssd_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElEphemeralStorageLocalSsdConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ephemeral_storage_local_ssd_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fast_socket` after provisioning.\n"]
    pub fn fast_socket(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElFastSocketElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fast_socket", self.base))
    }
    #[doc = "Get a reference to the value of field `flex_start` after provisioning.\n"]
    pub fn flex_start(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.flex_start", self.base))
    }
    #[doc = "Get a reference to the value of field `gcfs_config` after provisioning.\n"]
    pub fn gcfs_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElGcfsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gcfs_config", self.base))
    }
    #[doc = "Get a reference to the value of field `guest_accelerator` after provisioning.\n"]
    pub fn guest_accelerator(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElGuestAcceleratorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guest_accelerator", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gvnic` after provisioning.\n"]
    pub fn gvnic(&self) -> ListRef<DataContainerClusterNodePoolElNodeConfigElGvnicElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gvnic", self.base))
    }
    #[doc = "Get a reference to the value of field `host_maintenance_policy` after provisioning.\n"]
    pub fn host_maintenance_policy(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElHostMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.host_maintenance_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_type` after provisioning.\n"]
    pub fn image_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_type", self.base))
    }
    #[doc = "Get a reference to the value of field `kubelet_config` after provisioning.\n"]
    pub fn kubelet_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElKubeletConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kubelet_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `linux_node_config` after provisioning.\n"]
    pub fn linux_node_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElLinuxNodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.linux_node_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_nvme_ssd_block_config` after provisioning.\n"]
    pub fn local_nvme_ssd_block_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElLocalNvmeSsdBlockConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.local_nvme_ssd_block_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_ssd_count` after provisioning.\n"]
    pub fn local_ssd_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_ssd_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_ssd_encryption_mode` after provisioning.\n"]
    pub fn local_ssd_encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_ssd_encryption_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `logging_variant` after provisioning.\n"]
    pub fn logging_variant(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logging_variant", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `max_run_duration` after provisioning.\n"]
    pub fn max_run_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_run_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
    #[doc = "Get a reference to the value of field `min_cpu_platform` after provisioning.\n"]
    pub fn min_cpu_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_cpu_platform", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_group` after provisioning.\n"]
    pub fn node_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_group", self.base))
    }
    #[doc = "Get a reference to the value of field `oauth_scopes` after provisioning.\n"]
    pub fn oauth_scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.oauth_scopes", self.base))
    }
    #[doc = "Get a reference to the value of field `preemptible` after provisioning.\n"]
    pub fn preemptible(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preemptible", self.base))
    }
    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElReservationAffinityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reservation_affinity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_labels` after provisioning.\n"]
    pub fn resource_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_labels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\n"]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_manager_tags", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sandbox_config` after provisioning.\n"]
    pub fn sandbox_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElSandboxConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sandbox_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_boot_disks` after provisioning.\n"]
    pub fn secondary_boot_disks(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElSecondaryBootDisksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secondary_boot_disks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElShieldedInstanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.shielded_instance_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sole_tenant_config` after provisioning.\n"]
    pub fn sole_tenant_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElSoleTenantConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sole_tenant_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `spot` after provisioning.\n"]
    pub fn spot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.spot", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_pools` after provisioning.\n"]
    pub fn storage_pools(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_pools", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
    #[doc = "Get a reference to the value of field `taint` after provisioning.\n"]
    pub fn taint(&self) -> ListRef<DataContainerClusterNodePoolElNodeConfigElTaintElRef> {
        ListRef::new(self.shared().clone(), format!("{}.taint", self.base))
    }
    #[doc = "Get a reference to the value of field `windows_node_config` after provisioning.\n"]
    pub fn windows_node_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElWindowsNodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.windows_node_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `workload_metadata_config` after provisioning.\n"]
    pub fn workload_metadata_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElNodeConfigElWorkloadMetadataConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload_metadata_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElNodeDrainConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    respect_pdb_during_node_pool_deletion: Option<PrimField<bool>>,
}
impl DataContainerClusterNodePoolElNodeDrainConfigEl {
    #[doc = "Set the field `respect_pdb_during_node_pool_deletion`.\n"]
    pub fn set_respect_pdb_during_node_pool_deletion(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.respect_pdb_during_node_pool_deletion = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElNodeDrainConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElNodeDrainConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElNodeDrainConfigEl {}
impl BuildDataContainerClusterNodePoolElNodeDrainConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolElNodeDrainConfigEl {
        DataContainerClusterNodePoolElNodeDrainConfigEl {
            respect_pdb_during_node_pool_deletion: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElNodeDrainConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElNodeDrainConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElNodeDrainConfigElRef {
        DataContainerClusterNodePoolElNodeDrainConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElNodeDrainConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `respect_pdb_during_node_pool_deletion` after provisioning.\n"]
    pub fn respect_pdb_during_node_pool_deletion(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.respect_pdb_during_node_pool_deletion", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElPlacementPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tpu_topology: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolElPlacementPolicyEl {
    #[doc = "Set the field `policy_name`.\n"]
    pub fn set_policy_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy_name = Some(v.into());
        self
    }
    #[doc = "Set the field `tpu_topology`.\n"]
    pub fn set_tpu_topology(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tpu_topology = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElPlacementPolicyEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElPlacementPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElPlacementPolicyEl {}
impl BuildDataContainerClusterNodePoolElPlacementPolicyEl {
    pub fn build(self) -> DataContainerClusterNodePoolElPlacementPolicyEl {
        DataContainerClusterNodePoolElPlacementPolicyEl {
            policy_name: core::default::Default::default(),
            tpu_topology: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElPlacementPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElPlacementPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElPlacementPolicyElRef {
        DataContainerClusterNodePoolElPlacementPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElPlacementPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `policy_name` after provisioning.\n"]
    pub fn policy_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_name", self.base))
    }
    #[doc = "Get a reference to the value of field `tpu_topology` after provisioning.\n"]
    pub fn tpu_topology(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tpu_topology", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElQueuedProvisioningEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterNodePoolElQueuedProvisioningEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElQueuedProvisioningEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElQueuedProvisioningEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElQueuedProvisioningEl {}
impl BuildDataContainerClusterNodePoolElQueuedProvisioningEl {
    pub fn build(self) -> DataContainerClusterNodePoolElQueuedProvisioningEl {
        DataContainerClusterNodePoolElQueuedProvisioningEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElQueuedProvisioningElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElQueuedProvisioningElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElQueuedProvisioningElRef {
        DataContainerClusterNodePoolElQueuedProvisioningElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElQueuedProvisioningElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_soak_duration: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
    #[doc = "Set the field `batch_node_count`.\n"]
    pub fn set_batch_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.batch_node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `batch_percentage`.\n"]
    pub fn set_batch_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.batch_percentage = Some(v.into());
        self
    }
    #[doc = "Set the field `batch_soak_duration`.\n"]
    pub fn set_batch_soak_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.batch_soak_duration = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl
{
    type O = BlockAssignable<
        DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl
{}
impl
    BuildDataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl
{
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl
    {
        DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl {
            batch_node_count: core::default::Default::default(),
            batch_percentage: core::default::Default::default(),
            batch_soak_duration: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef
    {
        DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef { shared : shared , base : base . to_string () , }
    }
}
impl DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `batch_node_count` after provisioning.\n"]
    pub fn batch_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.batch_node_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `batch_percentage` after provisioning.\n"]
    pub fn batch_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.batch_percentage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `batch_soak_duration` after provisioning.\n"]
    pub fn batch_soak_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.batch_soak_duration", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl { # [serde (skip_serializing_if = "Option::is_none")] node_pool_soak_duration : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] standard_rollout_policy : Option < ListField < DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl > > , }
impl DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl {
    #[doc = "Set the field `node_pool_soak_duration`.\n"]
    pub fn set_node_pool_soak_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_pool_soak_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `standard_rollout_policy`.\n"]
    pub fn set_standard_rollout_policy(
        mut self,
        v : impl Into < ListField < DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyEl > >,
    ) -> Self {
        self.standard_rollout_policy = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl {}
impl BuildDataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl {
    pub fn build(self) -> DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl {
        DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl {
            node_pool_soak_duration: core::default::Default::default(),
            standard_rollout_policy: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElRef {
        DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `node_pool_soak_duration` after provisioning.\n"]
    pub fn node_pool_soak_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_pool_soak_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `standard_rollout_policy` after provisioning.\n"]    pub fn standard_rollout_policy (& self) -> ListRef < DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElStandardRolloutPolicyElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.standard_rollout_policy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolElUpgradeSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    blue_green_settings:
        Option<ListField<DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_surge: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_unavailable: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strategy: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolElUpgradeSettingsEl {
    #[doc = "Set the field `blue_green_settings`.\n"]
    pub fn set_blue_green_settings(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsEl>>,
    ) -> Self {
        self.blue_green_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `max_surge`.\n"]
    pub fn set_max_surge(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_surge = Some(v.into());
        self
    }
    #[doc = "Set the field `max_unavailable`.\n"]
    pub fn set_max_unavailable(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_unavailable = Some(v.into());
        self
    }
    #[doc = "Set the field `strategy`.\n"]
    pub fn set_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.strategy = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolElUpgradeSettingsEl {
    type O = BlockAssignable<DataContainerClusterNodePoolElUpgradeSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolElUpgradeSettingsEl {}
impl BuildDataContainerClusterNodePoolElUpgradeSettingsEl {
    pub fn build(self) -> DataContainerClusterNodePoolElUpgradeSettingsEl {
        DataContainerClusterNodePoolElUpgradeSettingsEl {
            blue_green_settings: core::default::Default::default(),
            max_surge: core::default::Default::default(),
            max_unavailable: core::default::Default::default(),
            strategy: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElUpgradeSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElUpgradeSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolElUpgradeSettingsElRef {
        DataContainerClusterNodePoolElUpgradeSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElUpgradeSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `blue_green_settings` after provisioning.\n"]
    pub fn blue_green_settings(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElUpgradeSettingsElBlueGreenSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.blue_green_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_surge` after provisioning.\n"]
    pub fn max_surge(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_surge", self.base))
    }
    #[doc = "Get a reference to the value of field `max_unavailable` after provisioning.\n"]
    pub fn max_unavailable(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_unavailable", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `strategy` after provisioning.\n"]
    pub fn strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.strategy", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling: Option<ListField<DataContainerClusterNodePoolElAutoscalingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_group_urls: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_instance_group_urls: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    management: Option<ListField<DataContainerClusterNodePoolElManagementEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_pods_per_node: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_config: Option<ListField<DataContainerClusterNodePoolElNetworkConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_config: Option<ListField<DataContainerClusterNodePoolElNodeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_drain_config: Option<ListField<DataContainerClusterNodePoolElNodeDrainConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_locations: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement_policy: Option<ListField<DataContainerClusterNodePoolElPlacementPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queued_provisioning: Option<ListField<DataContainerClusterNodePoolElQueuedProvisioningEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upgrade_settings: Option<ListField<DataContainerClusterNodePoolElUpgradeSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolEl {
    #[doc = "Set the field `autoscaling`.\n"]
    pub fn set_autoscaling(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElAutoscalingEl>>,
    ) -> Self {
        self.autoscaling = Some(v.into());
        self
    }
    #[doc = "Set the field `initial_node_count`.\n"]
    pub fn set_initial_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.initial_node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_group_urls`.\n"]
    pub fn set_instance_group_urls(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.instance_group_urls = Some(v.into());
        self
    }
    #[doc = "Set the field `managed_instance_group_urls`.\n"]
    pub fn set_managed_instance_group_urls(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.managed_instance_group_urls = Some(v.into());
        self
    }
    #[doc = "Set the field `management`.\n"]
    pub fn set_management(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElManagementEl>>,
    ) -> Self {
        self.management = Some(v.into());
        self
    }
    #[doc = "Set the field `max_pods_per_node`.\n"]
    pub fn set_max_pods_per_node(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_pods_per_node = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNetworkConfigEl>>,
    ) -> Self {
        self.network_config = Some(v.into());
        self
    }
    #[doc = "Set the field `node_config`.\n"]
    pub fn set_node_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeConfigEl>>,
    ) -> Self {
        self.node_config = Some(v.into());
        self
    }
    #[doc = "Set the field `node_count`.\n"]
    pub fn set_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `node_drain_config`.\n"]
    pub fn set_node_drain_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElNodeDrainConfigEl>>,
    ) -> Self {
        self.node_drain_config = Some(v.into());
        self
    }
    #[doc = "Set the field `node_locations`.\n"]
    pub fn set_node_locations(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.node_locations = Some(v.into());
        self
    }
    #[doc = "Set the field `placement_policy`.\n"]
    pub fn set_placement_policy(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElPlacementPolicyEl>>,
    ) -> Self {
        self.placement_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `queued_provisioning`.\n"]
    pub fn set_queued_provisioning(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElQueuedProvisioningEl>>,
    ) -> Self {
        self.queued_provisioning = Some(v.into());
        self
    }
    #[doc = "Set the field `upgrade_settings`.\n"]
    pub fn set_upgrade_settings(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolElUpgradeSettingsEl>>,
    ) -> Self {
        self.upgrade_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolEl {
    type O = BlockAssignable<DataContainerClusterNodePoolEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolEl {}
impl BuildDataContainerClusterNodePoolEl {
    pub fn build(self) -> DataContainerClusterNodePoolEl {
        DataContainerClusterNodePoolEl {
            autoscaling: core::default::Default::default(),
            initial_node_count: core::default::Default::default(),
            instance_group_urls: core::default::Default::default(),
            managed_instance_group_urls: core::default::Default::default(),
            management: core::default::Default::default(),
            max_pods_per_node: core::default::Default::default(),
            name: core::default::Default::default(),
            name_prefix: core::default::Default::default(),
            network_config: core::default::Default::default(),
            node_config: core::default::Default::default(),
            node_count: core::default::Default::default(),
            node_drain_config: core::default::Default::default(),
            node_locations: core::default::Default::default(),
            placement_policy: core::default::Default::default(),
            queued_provisioning: core::default::Default::default(),
            upgrade_settings: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolElRef {
        DataContainerClusterNodePoolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(&self) -> ListRef<DataContainerClusterNodePoolElAutoscalingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.autoscaling", self.base))
    }
    #[doc = "Get a reference to the value of field `initial_node_count` after provisioning.\n"]
    pub fn initial_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_node_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance_group_urls` after provisioning.\n"]
    pub fn instance_group_urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_group_urls", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `managed_instance_group_urls` after provisioning.\n"]
    pub fn managed_instance_group_urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_instance_group_urls", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> ListRef<DataContainerClusterNodePoolElManagementElRef> {
        ListRef::new(self.shared().clone(), format!("{}.management", self.base))
    }
    #[doc = "Get a reference to the value of field `max_pods_per_node` after provisioning.\n"]
    pub fn max_pods_per_node(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_pods_per_node", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<DataContainerClusterNodePoolElNetworkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> ListRef<DataContainerClusterNodePoolElNodeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_config", self.base))
    }
    #[doc = "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }
    #[doc = "Get a reference to the value of field `node_drain_config` after provisioning.\n"]
    pub fn node_drain_config(&self) -> ListRef<DataContainerClusterNodePoolElNodeDrainConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_drain_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_locations` after provisioning.\n"]
    pub fn node_locations(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.node_locations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `placement_policy` after provisioning.\n"]
    pub fn placement_policy(&self) -> ListRef<DataContainerClusterNodePoolElPlacementPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.placement_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `queued_provisioning` after provisioning.\n"]
    pub fn queued_provisioning(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolElQueuedProvisioningElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.queued_provisioning", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `upgrade_settings` after provisioning.\n"]
    pub fn upgrade_settings(&self) -> ListRef<DataContainerClusterNodePoolElUpgradeSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upgrade_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl {
    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl
{
    type O = BlockAssignable<
        DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolAutoConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl
{}
impl BuildDataContainerClusterNodePoolAutoConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl {
        DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl {
            policy: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigElNodeKernelModuleLoadingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigElNodeKernelModuleLoadingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigElNodeKernelModuleLoadingElRef {
        DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigElNodeKernelModuleLoadingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigElNodeKernelModuleLoadingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cgroup_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_kernel_module_loading: Option<
        ListField<
            DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl,
        >,
    >,
}
impl DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigEl {
    #[doc = "Set the field `cgroup_mode`.\n"]
    pub fn set_cgroup_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cgroup_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `node_kernel_module_loading`.\n"]
    pub fn set_node_kernel_module_loading(
        mut self,
        v: impl Into<
            ListField<
                DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigElNodeKernelModuleLoadingEl,
            >,
        >,
    ) -> Self {
        self.node_kernel_module_loading = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolAutoConfigElLinuxNodeConfigEl {}
impl BuildDataContainerClusterNodePoolAutoConfigElLinuxNodeConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigEl {
        DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigEl {
            cgroup_mode: core::default::Default::default(),
            node_kernel_module_loading: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigElRef {
        DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cgroup_mode` after provisioning.\n"]
    pub fn cgroup_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cgroup_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `node_kernel_module_loading` after provisioning.\n"]
    pub fn node_kernel_module_loading(
        &self,
    ) -> ListRef<
        DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigElNodeKernelModuleLoadingElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_kernel_module_loading", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolAutoConfigElNetworkTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
}
impl DataContainerClusterNodePoolAutoConfigElNetworkTagsEl {
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolAutoConfigElNetworkTagsEl {
    type O = BlockAssignable<DataContainerClusterNodePoolAutoConfigElNetworkTagsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolAutoConfigElNetworkTagsEl {}
impl BuildDataContainerClusterNodePoolAutoConfigElNetworkTagsEl {
    pub fn build(self) -> DataContainerClusterNodePoolAutoConfigElNetworkTagsEl {
        DataContainerClusterNodePoolAutoConfigElNetworkTagsEl {
            tags: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolAutoConfigElNetworkTagsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolAutoConfigElNetworkTagsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolAutoConfigElNetworkTagsElRef {
        DataContainerClusterNodePoolAutoConfigElNetworkTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolAutoConfigElNetworkTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolAutoConfigElNodeKubeletConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    insecure_kubelet_readonly_port_enabled: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolAutoConfigElNodeKubeletConfigEl {
    #[doc = "Set the field `insecure_kubelet_readonly_port_enabled`.\n"]
    pub fn set_insecure_kubelet_readonly_port_enabled(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.insecure_kubelet_readonly_port_enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolAutoConfigElNodeKubeletConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolAutoConfigElNodeKubeletConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolAutoConfigElNodeKubeletConfigEl {}
impl BuildDataContainerClusterNodePoolAutoConfigElNodeKubeletConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolAutoConfigElNodeKubeletConfigEl {
        DataContainerClusterNodePoolAutoConfigElNodeKubeletConfigEl {
            insecure_kubelet_readonly_port_enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolAutoConfigElNodeKubeletConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolAutoConfigElNodeKubeletConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolAutoConfigElNodeKubeletConfigElRef {
        DataContainerClusterNodePoolAutoConfigElNodeKubeletConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolAutoConfigElNodeKubeletConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `insecure_kubelet_readonly_port_enabled` after provisioning.\n"]
    pub fn insecure_kubelet_readonly_port_enabled(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.insecure_kubelet_readonly_port_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolAutoConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    linux_node_config: Option<ListField<DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_tags: Option<ListField<DataContainerClusterNodePoolAutoConfigElNetworkTagsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_kubelet_config:
        Option<ListField<DataContainerClusterNodePoolAutoConfigElNodeKubeletConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
}
impl DataContainerClusterNodePoolAutoConfigEl {
    #[doc = "Set the field `linux_node_config`.\n"]
    pub fn set_linux_node_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigEl>>,
    ) -> Self {
        self.linux_node_config = Some(v.into());
        self
    }
    #[doc = "Set the field `network_tags`.\n"]
    pub fn set_network_tags(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolAutoConfigElNetworkTagsEl>>,
    ) -> Self {
        self.network_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `node_kubelet_config`.\n"]
    pub fn set_node_kubelet_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolAutoConfigElNodeKubeletConfigEl>>,
    ) -> Self {
        self.node_kubelet_config = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_manager_tags`.\n"]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolAutoConfigEl {
    type O = BlockAssignable<DataContainerClusterNodePoolAutoConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolAutoConfigEl {}
impl BuildDataContainerClusterNodePoolAutoConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolAutoConfigEl {
        DataContainerClusterNodePoolAutoConfigEl {
            linux_node_config: core::default::Default::default(),
            network_tags: core::default::Default::default(),
            node_kubelet_config: core::default::Default::default(),
            resource_manager_tags: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolAutoConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolAutoConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolAutoConfigElRef {
        DataContainerClusterNodePoolAutoConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolAutoConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `linux_node_config` after provisioning.\n"]
    pub fn linux_node_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolAutoConfigElLinuxNodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.linux_node_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_tags` after provisioning.\n"]
    pub fn network_tags(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolAutoConfigElNetworkTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_tags", self.base))
    }
    #[doc = "Get a reference to the value of field `node_kubelet_config` after provisioning.\n"]
    pub fn node_kubelet_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolAutoConfigElNodeKubeletConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_kubelet_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\n"]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_manager_tags", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_uri: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl { # [doc = "Set the field `secret_uri`.\n"] pub fn set_secret_uri (mut self , v : impl Into < PrimField < String > >) -> Self { self . secret_uri = Some (v . into ()) ; self } }
impl ToListMappable for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl { type O = BlockAssignable < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl
{}
impl BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl { pub fn build (self) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl { DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl { secret_uri : core :: default :: Default :: default () , } } }
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigElRef { fn new (shared : StackShared , base : String) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigElRef { DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigElRef { shared : shared , base : base . to_string () , } } }
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `secret_uri` after provisioning.\n"] pub fn secret_uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.secret_uri" , self . base)) } }
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl { # [serde (skip_serializing_if = "Option::is_none")] fqdns : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] gcp_secret_manager_certificate_config : Option < ListField < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl > > , }
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl { # [doc = "Set the field `fqdns`.\n"] pub fn set_fqdns (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . fqdns = Some (v . into ()) ; self } # [doc = "Set the field `gcp_secret_manager_certificate_config`.\n"] pub fn set_gcp_secret_manager_certificate_config (mut self , v : impl Into < ListField < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigEl > >) -> Self { self . gcp_secret_manager_certificate_config = Some (v . into ()) ; self } }
impl ToListMappable for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl { type O = BlockAssignable < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl
{}
impl BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl { pub fn build (self) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl { DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl { fqdns : core :: default :: Default :: default () , gcp_secret_manager_certificate_config : core :: default :: Default :: default () , } } }
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElRef { fn new (shared : StackShared , base : String) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElRef { DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElRef { shared : shared , base : base . to_string () , } } }
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `fqdns` after provisioning.\n"] pub fn fqdns (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.fqdns" , self . base)) } # [doc = "Get a reference to the value of field `gcp_secret_manager_certificate_config` after provisioning.\n"] pub fn gcp_secret_manager_certificate_config (& self) -> ListRef < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElGcpSecretManagerCertificateConfigElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.gcp_secret_manager_certificate_config" , self . base)) } }
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigEl { # [serde (skip_serializing_if = "Option::is_none")] certificate_authority_domain_config : Option < ListField < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] enabled : Option < PrimField < bool > > , }
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigEl { # [doc = "Set the field `certificate_authority_domain_config`.\n"] pub fn set_certificate_authority_domain_config (mut self , v : impl Into < ListField < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigEl > >) -> Self { self . certificate_authority_domain_config = Some (v . into ()) ; self } # [doc = "Set the field `enabled`.\n"] pub fn set_enabled (mut self , v : impl Into < PrimField < bool > >) -> Self { self . enabled = Some (v . into ()) ; self } }
impl ToListMappable for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigEl { type O = BlockAssignable < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigEl
{}
impl BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigEl { pub fn build (self) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigEl { DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigEl { certificate_authority_domain_config : core :: default :: Default :: default () , enabled : core :: default :: Default :: default () , } } }
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElRef { fn new (shared : StackShared , base : String) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElRef { DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElRef { shared : shared , base : base . to_string () , } } }
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `certificate_authority_domain_config` after provisioning.\n"] pub fn certificate_authority_domain_config (& self) -> ListRef < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElCertificateAuthorityDomainConfigElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.certificate_authority_domain_config" , self . base)) } # [doc = "Get a reference to the value of field `enabled` after provisioning.\n"] pub fn enabled (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.enabled" , self . base)) } }
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElCaEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_secret_manager_secret_uri: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElCaEl { # [doc = "Set the field `gcp_secret_manager_secret_uri`.\n"] pub fn set_gcp_secret_manager_secret_uri (mut self , v : impl Into < PrimField < String > >) -> Self { self . gcp_secret_manager_secret_uri = Some (v . into ()) ; self } }
impl ToListMappable for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElCaEl { type O = BlockAssignable < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElCaEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElCaEl
{}
impl BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElCaEl { pub fn build (self) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElCaEl { DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElCaEl { gcp_secret_manager_secret_uri : core :: default :: Default :: default () , } } }
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElCaElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElCaElRef { fn new (shared : StackShared , base : String) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElCaElRef { DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElCaElRef { shared : shared , base : base . to_string () , } } }
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElCaElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `gcp_secret_manager_secret_uri` after provisioning.\n"] pub fn gcp_secret_manager_secret_uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.gcp_secret_manager_secret_uri" , self . base)) } }
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElCertEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_secret_manager_secret_uri: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElCertEl { # [doc = "Set the field `gcp_secret_manager_secret_uri`.\n"] pub fn set_gcp_secret_manager_secret_uri (mut self , v : impl Into < PrimField < String > >) -> Self { self . gcp_secret_manager_secret_uri = Some (v . into ()) ; self } }
impl ToListMappable for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElCertEl { type O = BlockAssignable < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElCertEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElCertEl
{}
impl BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElCertEl { pub fn build (self) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElCertEl { DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElCertEl { gcp_secret_manager_secret_uri : core :: default :: Default :: default () , } } }
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElCertElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElCertElRef { fn new (shared : StackShared , base : String) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElCertElRef { DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElCertElRef { shared : shared , base : base . to_string () , } } }
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElCertElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `gcp_secret_manager_secret_uri` after provisioning.\n"] pub fn gcp_secret_manager_secret_uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.gcp_secret_manager_secret_uri" , self . base)) } }
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElKeyEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_secret_manager_secret_uri: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElKeyEl { # [doc = "Set the field `gcp_secret_manager_secret_uri`.\n"] pub fn set_gcp_secret_manager_secret_uri (mut self , v : impl Into < PrimField < String > >) -> Self { self . gcp_secret_manager_secret_uri = Some (v . into ()) ; self } }
impl ToListMappable for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElKeyEl { type O = BlockAssignable < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElKeyEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElKeyEl
{}
impl BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElKeyEl { pub fn build (self) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElKeyEl { DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElKeyEl { gcp_secret_manager_secret_uri : core :: default :: Default :: default () , } } }
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElKeyElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElKeyElRef { fn new (shared : StackShared , base : String) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElKeyElRef { DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElKeyElRef { shared : shared , base : base . to_string () , } } }
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElKeyElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `gcp_secret_manager_secret_uri` after provisioning.\n"] pub fn gcp_secret_manager_secret_uri (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.gcp_secret_manager_secret_uri" , self . base)) } }
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientEl { # [serde (skip_serializing_if = "Option::is_none")] cert : Option < ListField < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElCertEl > > , # [serde (skip_serializing_if = "Option::is_none")] key : Option < ListField < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElKeyEl > > , }
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientEl { # [doc = "Set the field `cert`.\n"] pub fn set_cert (mut self , v : impl Into < ListField < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElCertEl > >) -> Self { self . cert = Some (v . into ()) ; self } # [doc = "Set the field `key`.\n"] pub fn set_key (mut self , v : impl Into < ListField < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElKeyEl > >) -> Self { self . key = Some (v . into ()) ; self } }
impl ToListMappable for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientEl { type O = BlockAssignable < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientEl
{}
impl BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientEl { pub fn build (self) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientEl { DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientEl { cert : core :: default :: Default :: default () , key : core :: default :: Default :: default () , } } }
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElRef { fn new (shared : StackShared , base : String) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElRef { DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElRef { shared : shared , base : base . to_string () , } } }
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `cert` after provisioning.\n"] pub fn cert (& self) -> ListRef < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElCertElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.cert" , self . base)) } # [doc = "Get a reference to the value of field `key` after provisioning.\n"] pub fn key (& self) -> ListRef < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElKeyElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.key" , self . base)) } }
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElHeaderEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<ListField<PrimField<String>>>,
}
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElHeaderEl { # [doc = "Set the field `key`.\n"] pub fn set_key (mut self , v : impl Into < PrimField < String > >) -> Self { self . key = Some (v . into ()) ; self } # [doc = "Set the field `value`.\n"] pub fn set_value (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . value = Some (v . into ()) ; self } }
impl ToListMappable for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElHeaderEl { type O = BlockAssignable < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElHeaderEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElHeaderEl
{}
impl BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElHeaderEl { pub fn build (self) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElHeaderEl { DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElHeaderEl { key : core :: default :: Default :: default () , value : core :: default :: Default :: default () , } } }
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElHeaderElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElHeaderElRef { fn new (shared : StackShared , base : String) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElHeaderElRef { DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElHeaderElRef { shared : shared , base : base . to_string () , } } }
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElHeaderElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `key` after provisioning.\n"] pub fn key (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.key" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsEl { # [serde (skip_serializing_if = "Option::is_none")] ca : Option < ListField < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElCaEl > > , # [serde (skip_serializing_if = "Option::is_none")] capabilities : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] client : Option < ListField < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientEl > > , # [serde (skip_serializing_if = "Option::is_none")] dial_timeout : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] header : Option < ListField < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElHeaderEl > > , # [serde (skip_serializing_if = "Option::is_none")] host : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] override_path : Option < PrimField < bool > > , }
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsEl { # [doc = "Set the field `ca`.\n"] pub fn set_ca (mut self , v : impl Into < ListField < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElCaEl > >) -> Self { self . ca = Some (v . into ()) ; self } # [doc = "Set the field `capabilities`.\n"] pub fn set_capabilities (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . capabilities = Some (v . into ()) ; self } # [doc = "Set the field `client`.\n"] pub fn set_client (mut self , v : impl Into < ListField < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientEl > >) -> Self { self . client = Some (v . into ()) ; self } # [doc = "Set the field `dial_timeout`.\n"] pub fn set_dial_timeout (mut self , v : impl Into < PrimField < String > >) -> Self { self . dial_timeout = Some (v . into ()) ; self } # [doc = "Set the field `header`.\n"] pub fn set_header (mut self , v : impl Into < ListField < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElHeaderEl > >) -> Self { self . header = Some (v . into ()) ; self } # [doc = "Set the field `host`.\n"] pub fn set_host (mut self , v : impl Into < PrimField < String > >) -> Self { self . host = Some (v . into ()) ; self } # [doc = "Set the field `override_path`.\n"] pub fn set_override_path (mut self , v : impl Into < PrimField < bool > >) -> Self { self . override_path = Some (v . into ()) ; self } }
impl ToListMappable for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsEl { type O = BlockAssignable < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsEl
{}
impl BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsEl { pub fn build (self) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsEl { DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsEl { ca : core :: default :: Default :: default () , capabilities : core :: default :: Default :: default () , client : core :: default :: Default :: default () , dial_timeout : core :: default :: Default :: default () , header : core :: default :: Default :: default () , host : core :: default :: Default :: default () , override_path : core :: default :: Default :: default () , } } }
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElRef { fn new (shared : StackShared , base : String) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElRef { DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElRef { shared : shared , base : base . to_string () , } } }
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `ca` after provisioning.\n"] pub fn ca (& self) -> ListRef < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElCaElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.ca" , self . base)) } # [doc = "Get a reference to the value of field `capabilities` after provisioning.\n"] pub fn capabilities (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.capabilities" , self . base)) } # [doc = "Get a reference to the value of field `client` after provisioning.\n"] pub fn client (& self) -> ListRef < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElClientElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.client" , self . base)) } # [doc = "Get a reference to the value of field `dial_timeout` after provisioning.\n"] pub fn dial_timeout (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.dial_timeout" , self . base)) } # [doc = "Get a reference to the value of field `header` after provisioning.\n"] pub fn header (& self) -> ListRef < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElHeaderElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.header" , self . base)) } # [doc = "Get a reference to the value of field `host` after provisioning.\n"] pub fn host (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.host" , self . base)) } # [doc = "Get a reference to the value of field `override_path` after provisioning.\n"] pub fn override_path (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.override_path" , self . base)) } }
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsEl { # [serde (skip_serializing_if = "Option::is_none")] hosts : Option < ListField < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsEl > > , # [serde (skip_serializing_if = "Option::is_none")] server : Option < PrimField < String > > , }
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsEl {
    #[doc = "Set the field `hosts`.\n"]
    pub fn set_hosts(
        mut self,
        v : impl Into < ListField < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsEl > >,
    ) -> Self {
        self.hosts = Some(v.into());
        self
    }
    #[doc = "Set the field `server`.\n"]
    pub fn set_server(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.server = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsEl
{
    type O = BlockAssignable<
        DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsEl
{}
impl
    BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsEl
{
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsEl
    {
        DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsEl { hosts : core :: default :: Default :: default () , server : core :: default :: Default :: default () , }
    }
}
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElRef { fn new (shared : StackShared , base : String) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElRef { DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElRef { shared : shared , base : base . to_string () , } } }
impl
    DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hosts` after provisioning.\n"]    pub fn hosts (& self) -> ListRef < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElHostsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.hosts", self.base))
    }
    #[doc = "Get a reference to the value of field `server` after provisioning.\n"]
    pub fn server(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElWritableCgroupsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElWritableCgroupsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElWritableCgroupsEl { type O = BlockAssignable < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElWritableCgroupsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElWritableCgroupsEl
{}
impl BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElWritableCgroupsEl { pub fn build (self) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElWritableCgroupsEl { DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElWritableCgroupsEl { enabled : core :: default :: Default :: default () , } } }
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElWritableCgroupsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElWritableCgroupsElRef { fn new (shared : StackShared , base : String) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElWritableCgroupsElRef { DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElWritableCgroupsElRef { shared : shared , base : base . to_string () , } } }
impl
    DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElWritableCgroupsElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigEl { # [serde (skip_serializing_if = "Option::is_none")] private_registry_access_config : Option < ListField < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] registry_hosts : Option < ListField < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsEl > > , # [serde (skip_serializing_if = "Option::is_none")] writable_cgroups : Option < ListField < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElWritableCgroupsEl > > , }
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigEl {
    #[doc = "Set the field `private_registry_access_config`.\n"]
    pub fn set_private_registry_access_config(
        mut self,
        v : impl Into < ListField < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigEl > >,
    ) -> Self {
        self.private_registry_access_config = Some(v.into());
        self
    }
    #[doc = "Set the field `registry_hosts`.\n"]
    pub fn set_registry_hosts(
        mut self,
        v : impl Into < ListField < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsEl > >,
    ) -> Self {
        self.registry_hosts = Some(v.into());
        self
    }
    #[doc = "Set the field `writable_cgroups`.\n"]
    pub fn set_writable_cgroups(
        mut self,
        v : impl Into < ListField < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElWritableCgroupsEl > >,
    ) -> Self {
        self.writable_cgroups = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigEl
{
    type O = BlockAssignable<
        DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigEl {}
impl BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigEl {
    pub fn build(
        self,
    ) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigEl {
        DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigEl {
            private_registry_access_config: core::default::Default::default(),
            registry_hosts: core::default::Default::default(),
            writable_cgroups: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRef {
        DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `private_registry_access_config` after provisioning.\n"]    pub fn private_registry_access_config (& self) -> ListRef < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElPrivateRegistryAccessConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_registry_access_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `registry_hosts` after provisioning.\n"]    pub fn registry_hosts (& self) -> ListRef < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRegistryHostsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.registry_hosts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `writable_cgroups` after provisioning.\n"]    pub fn writable_cgroups (& self) -> ListRef < DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElWritableCgroupsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.writable_cgroups", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElGcfsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElGcfsConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElGcfsConfigEl {
    type O =
        BlockAssignable<DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElGcfsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElGcfsConfigEl {}
impl BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElGcfsConfigEl {
    pub fn build(self) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElGcfsConfigEl {
        DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElGcfsConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElGcfsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElGcfsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElGcfsConfigElRef {
        DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElGcfsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElGcfsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    containerd_config: Option<
        ListField<DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcfs_config:
        Option<ListField<DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElGcfsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insecure_kubelet_readonly_port_enabled: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_variant: Option<PrimField<String>>,
}
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl {
    #[doc = "Set the field `containerd_config`.\n"]
    pub fn set_containerd_config(
        mut self,
        v: impl Into<
            ListField<DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigEl>,
        >,
    ) -> Self {
        self.containerd_config = Some(v.into());
        self
    }
    #[doc = "Set the field `gcfs_config`.\n"]
    pub fn set_gcfs_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElGcfsConfigEl>>,
    ) -> Self {
        self.gcfs_config = Some(v.into());
        self
    }
    #[doc = "Set the field `insecure_kubelet_readonly_port_enabled`.\n"]
    pub fn set_insecure_kubelet_readonly_port_enabled(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.insecure_kubelet_readonly_port_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `logging_variant`.\n"]
    pub fn set_logging_variant(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.logging_variant = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl {
    type O = BlockAssignable<DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl {}
impl BuildDataContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl {
    pub fn build(self) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl {
        DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl {
            containerd_config: core::default::Default::default(),
            gcfs_config: core::default::Default::default(),
            insecure_kubelet_readonly_port_enabled: core::default::Default::default(),
            logging_variant: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElRef {
        DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `containerd_config` after provisioning.\n"]
    pub fn containerd_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElContainerdConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.containerd_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gcfs_config` after provisioning.\n"]
    pub fn gcfs_config(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElGcfsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gcfs_config", self.base))
    }
    #[doc = "Get a reference to the value of field `insecure_kubelet_readonly_port_enabled` after provisioning.\n"]
    pub fn insecure_kubelet_readonly_port_enabled(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.insecure_kubelet_readonly_port_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `logging_variant` after provisioning.\n"]
    pub fn logging_variant(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logging_variant", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNodePoolDefaultsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    node_config_defaults:
        Option<ListField<DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl>>,
}
impl DataContainerClusterNodePoolDefaultsEl {
    #[doc = "Set the field `node_config_defaults`.\n"]
    pub fn set_node_config_defaults(
        mut self,
        v: impl Into<ListField<DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsEl>>,
    ) -> Self {
        self.node_config_defaults = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNodePoolDefaultsEl {
    type O = BlockAssignable<DataContainerClusterNodePoolDefaultsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNodePoolDefaultsEl {}
impl BuildDataContainerClusterNodePoolDefaultsEl {
    pub fn build(self) -> DataContainerClusterNodePoolDefaultsEl {
        DataContainerClusterNodePoolDefaultsEl {
            node_config_defaults: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNodePoolDefaultsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNodePoolDefaultsElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNodePoolDefaultsElRef {
        DataContainerClusterNodePoolDefaultsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNodePoolDefaultsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `node_config_defaults` after provisioning.\n"]
    pub fn node_config_defaults(
        &self,
    ) -> ListRef<DataContainerClusterNodePoolDefaultsElNodeConfigDefaultsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_config_defaults", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNotificationConfigElPubsubElFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    event_type: Option<ListField<PrimField<String>>>,
}
impl DataContainerClusterNotificationConfigElPubsubElFilterEl {
    #[doc = "Set the field `event_type`.\n"]
    pub fn set_event_type(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.event_type = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNotificationConfigElPubsubElFilterEl {
    type O = BlockAssignable<DataContainerClusterNotificationConfigElPubsubElFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNotificationConfigElPubsubElFilterEl {}
impl BuildDataContainerClusterNotificationConfigElPubsubElFilterEl {
    pub fn build(self) -> DataContainerClusterNotificationConfigElPubsubElFilterEl {
        DataContainerClusterNotificationConfigElPubsubElFilterEl {
            event_type: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNotificationConfigElPubsubElFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNotificationConfigElPubsubElFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNotificationConfigElPubsubElFilterElRef {
        DataContainerClusterNotificationConfigElPubsubElFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNotificationConfigElPubsubElFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `event_type` after provisioning.\n"]
    pub fn event_type(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.event_type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNotificationConfigElPubsubEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<ListField<DataContainerClusterNotificationConfigElPubsubElFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<PrimField<String>>,
}
impl DataContainerClusterNotificationConfigElPubsubEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl Into<ListField<DataContainerClusterNotificationConfigElPubsubElFilterEl>>,
    ) -> Self {
        self.filter = Some(v.into());
        self
    }
    #[doc = "Set the field `topic`.\n"]
    pub fn set_topic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.topic = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNotificationConfigElPubsubEl {
    type O = BlockAssignable<DataContainerClusterNotificationConfigElPubsubEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNotificationConfigElPubsubEl {}
impl BuildDataContainerClusterNotificationConfigElPubsubEl {
    pub fn build(self) -> DataContainerClusterNotificationConfigElPubsubEl {
        DataContainerClusterNotificationConfigElPubsubEl {
            enabled: core::default::Default::default(),
            filter: core::default::Default::default(),
            topic: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNotificationConfigElPubsubElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNotificationConfigElPubsubElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterNotificationConfigElPubsubElRef {
        DataContainerClusterNotificationConfigElPubsubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNotificationConfigElPubsubElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<DataContainerClusterNotificationConfigElPubsubElFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterNotificationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pubsub: Option<ListField<DataContainerClusterNotificationConfigElPubsubEl>>,
}
impl DataContainerClusterNotificationConfigEl {
    #[doc = "Set the field `pubsub`.\n"]
    pub fn set_pubsub(
        mut self,
        v: impl Into<ListField<DataContainerClusterNotificationConfigElPubsubEl>>,
    ) -> Self {
        self.pubsub = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterNotificationConfigEl {
    type O = BlockAssignable<DataContainerClusterNotificationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterNotificationConfigEl {}
impl BuildDataContainerClusterNotificationConfigEl {
    pub fn build(self) -> DataContainerClusterNotificationConfigEl {
        DataContainerClusterNotificationConfigEl {
            pubsub: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterNotificationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterNotificationConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterNotificationConfigElRef {
        DataContainerClusterNotificationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterNotificationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pubsub` after provisioning.\n"]
    pub fn pubsub(&self) -> ListRef<DataContainerClusterNotificationConfigElPubsubElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pubsub", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterPodAutoscalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hpa_profile: Option<PrimField<String>>,
}
impl DataContainerClusterPodAutoscalingEl {
    #[doc = "Set the field `hpa_profile`.\n"]
    pub fn set_hpa_profile(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hpa_profile = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterPodAutoscalingEl {
    type O = BlockAssignable<DataContainerClusterPodAutoscalingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterPodAutoscalingEl {}
impl BuildDataContainerClusterPodAutoscalingEl {
    pub fn build(self) -> DataContainerClusterPodAutoscalingEl {
        DataContainerClusterPodAutoscalingEl {
            hpa_profile: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterPodAutoscalingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterPodAutoscalingElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterPodAutoscalingElRef {
        DataContainerClusterPodAutoscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterPodAutoscalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hpa_profile` after provisioning.\n"]
    pub fn hpa_profile(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hpa_profile", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl {
    type O = BlockAssignable<DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl {}
impl BuildDataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl {
    pub fn build(self) -> DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl {
        DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigElRef {
        DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterPrivateClusterConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_private_endpoint: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_private_nodes: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_global_access_config:
        Option<ListField<DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_ipv4_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peering_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_endpoint_subnetwork: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_endpoint: Option<PrimField<String>>,
}
impl DataContainerClusterPrivateClusterConfigEl {
    #[doc = "Set the field `enable_private_endpoint`.\n"]
    pub fn set_enable_private_endpoint(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_private_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_private_nodes`.\n"]
    pub fn set_enable_private_nodes(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_private_nodes = Some(v.into());
        self
    }
    #[doc = "Set the field `master_global_access_config`.\n"]
    pub fn set_master_global_access_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigEl>>,
    ) -> Self {
        self.master_global_access_config = Some(v.into());
        self
    }
    #[doc = "Set the field `master_ipv4_cidr_block`.\n"]
    pub fn set_master_ipv4_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.master_ipv4_cidr_block = Some(v.into());
        self
    }
    #[doc = "Set the field `peering_name`.\n"]
    pub fn set_peering_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.peering_name = Some(v.into());
        self
    }
    #[doc = "Set the field `private_endpoint`.\n"]
    pub fn set_private_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `private_endpoint_subnetwork`.\n"]
    pub fn set_private_endpoint_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_endpoint_subnetwork = Some(v.into());
        self
    }
    #[doc = "Set the field `public_endpoint`.\n"]
    pub fn set_public_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_endpoint = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterPrivateClusterConfigEl {
    type O = BlockAssignable<DataContainerClusterPrivateClusterConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterPrivateClusterConfigEl {}
impl BuildDataContainerClusterPrivateClusterConfigEl {
    pub fn build(self) -> DataContainerClusterPrivateClusterConfigEl {
        DataContainerClusterPrivateClusterConfigEl {
            enable_private_endpoint: core::default::Default::default(),
            enable_private_nodes: core::default::Default::default(),
            master_global_access_config: core::default::Default::default(),
            master_ipv4_cidr_block: core::default::Default::default(),
            peering_name: core::default::Default::default(),
            private_endpoint: core::default::Default::default(),
            private_endpoint_subnetwork: core::default::Default::default(),
            public_endpoint: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterPrivateClusterConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterPrivateClusterConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterPrivateClusterConfigElRef {
        DataContainerClusterPrivateClusterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterPrivateClusterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_private_endpoint` after provisioning.\n"]
    pub fn enable_private_endpoint(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_private_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_private_nodes` after provisioning.\n"]
    pub fn enable_private_nodes(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_private_nodes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `master_global_access_config` after provisioning.\n"]
    pub fn master_global_access_config(
        &self,
    ) -> ListRef<DataContainerClusterPrivateClusterConfigElMasterGlobalAccessConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.master_global_access_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `master_ipv4_cidr_block` after provisioning.\n"]
    pub fn master_ipv4_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_ipv4_cidr_block", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `peering_name` after provisioning.\n"]
    pub fn peering_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.peering_name", self.base))
    }
    #[doc = "Get a reference to the value of field `private_endpoint` after provisioning.\n"]
    pub fn private_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `private_endpoint_subnetwork` after provisioning.\n"]
    pub fn private_endpoint_subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_endpoint_subnetwork", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `public_endpoint` after provisioning.\n"]
    pub fn public_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_endpoint", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterRbacBindingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_insecure_binding_system_authenticated: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_insecure_binding_system_unauthenticated: Option<PrimField<bool>>,
}
impl DataContainerClusterRbacBindingConfigEl {
    #[doc = "Set the field `enable_insecure_binding_system_authenticated`.\n"]
    pub fn set_enable_insecure_binding_system_authenticated(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.enable_insecure_binding_system_authenticated = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_insecure_binding_system_unauthenticated`.\n"]
    pub fn set_enable_insecure_binding_system_unauthenticated(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.enable_insecure_binding_system_unauthenticated = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterRbacBindingConfigEl {
    type O = BlockAssignable<DataContainerClusterRbacBindingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterRbacBindingConfigEl {}
impl BuildDataContainerClusterRbacBindingConfigEl {
    pub fn build(self) -> DataContainerClusterRbacBindingConfigEl {
        DataContainerClusterRbacBindingConfigEl {
            enable_insecure_binding_system_authenticated: core::default::Default::default(),
            enable_insecure_binding_system_unauthenticated: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterRbacBindingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterRbacBindingConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterRbacBindingConfigElRef {
        DataContainerClusterRbacBindingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterRbacBindingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_insecure_binding_system_authenticated` after provisioning.\n"]
    pub fn enable_insecure_binding_system_authenticated(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_insecure_binding_system_authenticated", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_insecure_binding_system_unauthenticated` after provisioning.\n"]
    pub fn enable_insecure_binding_system_unauthenticated(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.enable_insecure_binding_system_unauthenticated",
                self.base
            ),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterReleaseChannelEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<PrimField<String>>,
}
impl DataContainerClusterReleaseChannelEl {
    #[doc = "Set the field `channel`.\n"]
    pub fn set_channel(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.channel = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterReleaseChannelEl {
    type O = BlockAssignable<DataContainerClusterReleaseChannelEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterReleaseChannelEl {}
impl BuildDataContainerClusterReleaseChannelEl {
    pub fn build(self) -> DataContainerClusterReleaseChannelEl {
        DataContainerClusterReleaseChannelEl {
            channel: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterReleaseChannelElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterReleaseChannelElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterReleaseChannelElRef {
        DataContainerClusterReleaseChannelElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterReleaseChannelElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `channel` after provisioning.\n"]
    pub fn channel(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterResourceUsageExportConfigElBigqueryDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_id: Option<PrimField<String>>,
}
impl DataContainerClusterResourceUsageExportConfigElBigqueryDestinationEl {
    #[doc = "Set the field `dataset_id`.\n"]
    pub fn set_dataset_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dataset_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterResourceUsageExportConfigElBigqueryDestinationEl {
    type O = BlockAssignable<DataContainerClusterResourceUsageExportConfigElBigqueryDestinationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterResourceUsageExportConfigElBigqueryDestinationEl {}
impl BuildDataContainerClusterResourceUsageExportConfigElBigqueryDestinationEl {
    pub fn build(self) -> DataContainerClusterResourceUsageExportConfigElBigqueryDestinationEl {
        DataContainerClusterResourceUsageExportConfigElBigqueryDestinationEl {
            dataset_id: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterResourceUsageExportConfigElBigqueryDestinationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterResourceUsageExportConfigElBigqueryDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterResourceUsageExportConfigElBigqueryDestinationElRef {
        DataContainerClusterResourceUsageExportConfigElBigqueryDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterResourceUsageExportConfigElBigqueryDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\n"]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterResourceUsageExportConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bigquery_destination:
        Option<ListField<DataContainerClusterResourceUsageExportConfigElBigqueryDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_network_egress_metering: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_resource_consumption_metering: Option<PrimField<bool>>,
}
impl DataContainerClusterResourceUsageExportConfigEl {
    #[doc = "Set the field `bigquery_destination`.\n"]
    pub fn set_bigquery_destination(
        mut self,
        v: impl Into<ListField<DataContainerClusterResourceUsageExportConfigElBigqueryDestinationEl>>,
    ) -> Self {
        self.bigquery_destination = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_network_egress_metering`.\n"]
    pub fn set_enable_network_egress_metering(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_network_egress_metering = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_resource_consumption_metering`.\n"]
    pub fn set_enable_resource_consumption_metering(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.enable_resource_consumption_metering = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterResourceUsageExportConfigEl {
    type O = BlockAssignable<DataContainerClusterResourceUsageExportConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterResourceUsageExportConfigEl {}
impl BuildDataContainerClusterResourceUsageExportConfigEl {
    pub fn build(self) -> DataContainerClusterResourceUsageExportConfigEl {
        DataContainerClusterResourceUsageExportConfigEl {
            bigquery_destination: core::default::Default::default(),
            enable_network_egress_metering: core::default::Default::default(),
            enable_resource_consumption_metering: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterResourceUsageExportConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterResourceUsageExportConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterResourceUsageExportConfigElRef {
        DataContainerClusterResourceUsageExportConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterResourceUsageExportConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bigquery_destination` after provisioning.\n"]
    pub fn bigquery_destination(
        &self,
    ) -> ListRef<DataContainerClusterResourceUsageExportConfigElBigqueryDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigquery_destination", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_network_egress_metering` after provisioning.\n"]
    pub fn enable_network_egress_metering(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_network_egress_metering", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_resource_consumption_metering` after provisioning.\n"]
    pub fn enable_resource_consumption_metering(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_resource_consumption_metering", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterSecretManagerConfigElRotationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation_interval: Option<PrimField<String>>,
}
impl DataContainerClusterSecretManagerConfigElRotationConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `rotation_interval`.\n"]
    pub fn set_rotation_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rotation_interval = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterSecretManagerConfigElRotationConfigEl {
    type O = BlockAssignable<DataContainerClusterSecretManagerConfigElRotationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterSecretManagerConfigElRotationConfigEl {}
impl BuildDataContainerClusterSecretManagerConfigElRotationConfigEl {
    pub fn build(self) -> DataContainerClusterSecretManagerConfigElRotationConfigEl {
        DataContainerClusterSecretManagerConfigElRotationConfigEl {
            enabled: core::default::Default::default(),
            rotation_interval: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterSecretManagerConfigElRotationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterSecretManagerConfigElRotationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterSecretManagerConfigElRotationConfigElRef {
        DataContainerClusterSecretManagerConfigElRotationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterSecretManagerConfigElRotationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `rotation_interval` after provisioning.\n"]
    pub fn rotation_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rotation_interval", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterSecretManagerConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation_config: Option<ListField<DataContainerClusterSecretManagerConfigElRotationConfigEl>>,
}
impl DataContainerClusterSecretManagerConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `rotation_config`.\n"]
    pub fn set_rotation_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterSecretManagerConfigElRotationConfigEl>>,
    ) -> Self {
        self.rotation_config = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterSecretManagerConfigEl {
    type O = BlockAssignable<DataContainerClusterSecretManagerConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterSecretManagerConfigEl {}
impl BuildDataContainerClusterSecretManagerConfigEl {
    pub fn build(self) -> DataContainerClusterSecretManagerConfigEl {
        DataContainerClusterSecretManagerConfigEl {
            enabled: core::default::Default::default(),
            rotation_config: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterSecretManagerConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterSecretManagerConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterSecretManagerConfigElRef {
        DataContainerClusterSecretManagerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterSecretManagerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `rotation_config` after provisioning.\n"]
    pub fn rotation_config(
        &self,
    ) -> ListRef<DataContainerClusterSecretManagerConfigElRotationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rotation_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterSecretSyncConfigElRotationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation_interval: Option<PrimField<String>>,
}
impl DataContainerClusterSecretSyncConfigElRotationConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `rotation_interval`.\n"]
    pub fn set_rotation_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rotation_interval = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterSecretSyncConfigElRotationConfigEl {
    type O = BlockAssignable<DataContainerClusterSecretSyncConfigElRotationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterSecretSyncConfigElRotationConfigEl {}
impl BuildDataContainerClusterSecretSyncConfigElRotationConfigEl {
    pub fn build(self) -> DataContainerClusterSecretSyncConfigElRotationConfigEl {
        DataContainerClusterSecretSyncConfigElRotationConfigEl {
            enabled: core::default::Default::default(),
            rotation_interval: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterSecretSyncConfigElRotationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterSecretSyncConfigElRotationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataContainerClusterSecretSyncConfigElRotationConfigElRef {
        DataContainerClusterSecretSyncConfigElRotationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterSecretSyncConfigElRotationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `rotation_interval` after provisioning.\n"]
    pub fn rotation_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rotation_interval", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterSecretSyncConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation_config: Option<ListField<DataContainerClusterSecretSyncConfigElRotationConfigEl>>,
}
impl DataContainerClusterSecretSyncConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `rotation_config`.\n"]
    pub fn set_rotation_config(
        mut self,
        v: impl Into<ListField<DataContainerClusterSecretSyncConfigElRotationConfigEl>>,
    ) -> Self {
        self.rotation_config = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterSecretSyncConfigEl {
    type O = BlockAssignable<DataContainerClusterSecretSyncConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterSecretSyncConfigEl {}
impl BuildDataContainerClusterSecretSyncConfigEl {
    pub fn build(self) -> DataContainerClusterSecretSyncConfigEl {
        DataContainerClusterSecretSyncConfigEl {
            enabled: core::default::Default::default(),
            rotation_config: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterSecretSyncConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterSecretSyncConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterSecretSyncConfigElRef {
        DataContainerClusterSecretSyncConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterSecretSyncConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `rotation_config` after provisioning.\n"]
    pub fn rotation_config(
        &self,
    ) -> ListRef<DataContainerClusterSecretSyncConfigElRotationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rotation_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterSecurityPostureConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vulnerability_mode: Option<PrimField<String>>,
}
impl DataContainerClusterSecurityPostureConfigEl {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `vulnerability_mode`.\n"]
    pub fn set_vulnerability_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vulnerability_mode = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterSecurityPostureConfigEl {
    type O = BlockAssignable<DataContainerClusterSecurityPostureConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterSecurityPostureConfigEl {}
impl BuildDataContainerClusterSecurityPostureConfigEl {
    pub fn build(self) -> DataContainerClusterSecurityPostureConfigEl {
        DataContainerClusterSecurityPostureConfigEl {
            mode: core::default::Default::default(),
            vulnerability_mode: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterSecurityPostureConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterSecurityPostureConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterSecurityPostureConfigElRef {
        DataContainerClusterSecurityPostureConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterSecurityPostureConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `vulnerability_mode` after provisioning.\n"]
    pub fn vulnerability_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vulnerability_mode", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterServiceExternalIpsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterServiceExternalIpsConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterServiceExternalIpsConfigEl {
    type O = BlockAssignable<DataContainerClusterServiceExternalIpsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterServiceExternalIpsConfigEl {}
impl BuildDataContainerClusterServiceExternalIpsConfigEl {
    pub fn build(self) -> DataContainerClusterServiceExternalIpsConfigEl {
        DataContainerClusterServiceExternalIpsConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterServiceExternalIpsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterServiceExternalIpsConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterServiceExternalIpsConfigElRef {
        DataContainerClusterServiceExternalIpsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterServiceExternalIpsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterUserManagedKeysConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aggregation_ca: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_ca: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane_disk_encryption_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane_disk_encryption_key_versions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    etcd_api_ca: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    etcd_peer_ca: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gkeops_etcd_backup_encryption_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_signing_keys: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_verification_keys: Option<SetField<PrimField<String>>>,
}
impl DataContainerClusterUserManagedKeysConfigEl {
    #[doc = "Set the field `aggregation_ca`.\n"]
    pub fn set_aggregation_ca(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.aggregation_ca = Some(v.into());
        self
    }
    #[doc = "Set the field `cluster_ca`.\n"]
    pub fn set_cluster_ca(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_ca = Some(v.into());
        self
    }
    #[doc = "Set the field `control_plane_disk_encryption_key`.\n"]
    pub fn set_control_plane_disk_encryption_key(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.control_plane_disk_encryption_key = Some(v.into());
        self
    }
    #[doc = "Set the field `control_plane_disk_encryption_key_versions`.\n"]
    pub fn set_control_plane_disk_encryption_key_versions(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.control_plane_disk_encryption_key_versions = Some(v.into());
        self
    }
    #[doc = "Set the field `etcd_api_ca`.\n"]
    pub fn set_etcd_api_ca(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.etcd_api_ca = Some(v.into());
        self
    }
    #[doc = "Set the field `etcd_peer_ca`.\n"]
    pub fn set_etcd_peer_ca(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.etcd_peer_ca = Some(v.into());
        self
    }
    #[doc = "Set the field `gkeops_etcd_backup_encryption_key`.\n"]
    pub fn set_gkeops_etcd_backup_encryption_key(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.gkeops_etcd_backup_encryption_key = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account_signing_keys`.\n"]
    pub fn set_service_account_signing_keys(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.service_account_signing_keys = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account_verification_keys`.\n"]
    pub fn set_service_account_verification_keys(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.service_account_verification_keys = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterUserManagedKeysConfigEl {
    type O = BlockAssignable<DataContainerClusterUserManagedKeysConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterUserManagedKeysConfigEl {}
impl BuildDataContainerClusterUserManagedKeysConfigEl {
    pub fn build(self) -> DataContainerClusterUserManagedKeysConfigEl {
        DataContainerClusterUserManagedKeysConfigEl {
            aggregation_ca: core::default::Default::default(),
            cluster_ca: core::default::Default::default(),
            control_plane_disk_encryption_key: core::default::Default::default(),
            control_plane_disk_encryption_key_versions: core::default::Default::default(),
            etcd_api_ca: core::default::Default::default(),
            etcd_peer_ca: core::default::Default::default(),
            gkeops_etcd_backup_encryption_key: core::default::Default::default(),
            service_account_signing_keys: core::default::Default::default(),
            service_account_verification_keys: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterUserManagedKeysConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterUserManagedKeysConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterUserManagedKeysConfigElRef {
        DataContainerClusterUserManagedKeysConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterUserManagedKeysConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aggregation_ca` after provisioning.\n"]
    pub fn aggregation_ca(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aggregation_ca", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_ca` after provisioning.\n"]
    pub fn cluster_ca(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_ca", self.base))
    }
    #[doc = "Get a reference to the value of field `control_plane_disk_encryption_key` after provisioning.\n"]
    pub fn control_plane_disk_encryption_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.control_plane_disk_encryption_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane_disk_encryption_key_versions` after provisioning.\n"]
    pub fn control_plane_disk_encryption_key_versions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.control_plane_disk_encryption_key_versions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `etcd_api_ca` after provisioning.\n"]
    pub fn etcd_api_ca(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etcd_api_ca", self.base))
    }
    #[doc = "Get a reference to the value of field `etcd_peer_ca` after provisioning.\n"]
    pub fn etcd_peer_ca(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etcd_peer_ca", self.base))
    }
    #[doc = "Get a reference to the value of field `gkeops_etcd_backup_encryption_key` after provisioning.\n"]
    pub fn gkeops_etcd_backup_encryption_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gkeops_etcd_backup_encryption_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_account_signing_keys` after provisioning.\n"]
    pub fn service_account_signing_keys(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.service_account_signing_keys", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_account_verification_keys` after provisioning.\n"]
    pub fn service_account_verification_keys(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.service_account_verification_keys", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterVerticalPodAutoscalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataContainerClusterVerticalPodAutoscalingEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterVerticalPodAutoscalingEl {
    type O = BlockAssignable<DataContainerClusterVerticalPodAutoscalingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterVerticalPodAutoscalingEl {}
impl BuildDataContainerClusterVerticalPodAutoscalingEl {
    pub fn build(self) -> DataContainerClusterVerticalPodAutoscalingEl {
        DataContainerClusterVerticalPodAutoscalingEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterVerticalPodAutoscalingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterVerticalPodAutoscalingElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterVerticalPodAutoscalingElRef {
        DataContainerClusterVerticalPodAutoscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterVerticalPodAutoscalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataContainerClusterWorkloadIdentityConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    workload_pool: Option<PrimField<String>>,
}
impl DataContainerClusterWorkloadIdentityConfigEl {
    #[doc = "Set the field `workload_pool`.\n"]
    pub fn set_workload_pool(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.workload_pool = Some(v.into());
        self
    }
}
impl ToListMappable for DataContainerClusterWorkloadIdentityConfigEl {
    type O = BlockAssignable<DataContainerClusterWorkloadIdentityConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataContainerClusterWorkloadIdentityConfigEl {}
impl BuildDataContainerClusterWorkloadIdentityConfigEl {
    pub fn build(self) -> DataContainerClusterWorkloadIdentityConfigEl {
        DataContainerClusterWorkloadIdentityConfigEl {
            workload_pool: core::default::Default::default(),
        }
    }
}
pub struct DataContainerClusterWorkloadIdentityConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerClusterWorkloadIdentityConfigElRef {
    fn new(shared: StackShared, base: String) -> DataContainerClusterWorkloadIdentityConfigElRef {
        DataContainerClusterWorkloadIdentityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataContainerClusterWorkloadIdentityConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `workload_pool` after provisioning.\n"]
    pub fn workload_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_pool", self.base),
        )
    }
}
