use super::provider::ProviderGrafana;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCloudStackData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    slug: PrimField<String>,
}
struct DataCloudStack_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudStackData>,
}
#[derive(Clone)]
pub struct DataCloudStack(Rc<DataCloudStack_>);
impl DataCloudStack {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGrafana) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Get a reference to the value of field `alertmanager_allowlist_url` after provisioning.\nAllowlist API endpoint that returns the source IP addresses to allow for the Alertmanager instances."]
    pub fn alertmanager_allowlist_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alertmanager_allowlist_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alertmanager_ip_allow_list_cname` after provisioning.\nComma-separated list of CNAMEs that can be whitelisted to access the Alertmanager instances (Optional)"]
    pub fn alertmanager_ip_allow_list_cname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alertmanager_ip_allow_list_cname", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alertmanager_name` after provisioning.\nName of the Alertmanager instance configured for this stack."]
    pub fn alertmanager_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alertmanager_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alertmanager_status` after provisioning.\nStatus of the Alertmanager instance configured for this stack."]
    pub fn alertmanager_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alertmanager_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alertmanager_url` after provisioning.\nBase URL of the Alertmanager instance configured for this stack."]
    pub fn alertmanager_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alertmanager_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alertmanager_user_id` after provisioning.\nUser ID of the Alertmanager instance configured for this stack."]
    pub fn alertmanager_user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alertmanager_user_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_provider_url` after provisioning.\nBase URL of the Cloud Provider API for this stack's cluster. This can be used with the `cloud_provider_url` provider config option to manage Cloud Provider resources for this stack."]
    pub fn cloud_provider_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_provider_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\nName of the cluster where this stack resides."]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_slug` after provisioning.\nSlug of the cluster where this stack resides."]
    pub fn cluster_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_slug", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connections_api_url` after provisioning.\nBase URL of the Connections API for this stack's cluster. This can be used with the `connections_api_url` provider config option to manage Connections resources for this stack."]
    pub fn connections_api_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connections_api_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_protection` after provisioning.\nWhether to enable delete protection for the stack, preventing accidental deletion."]
    pub fn delete_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of stack."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_management_allowlist_url` after provisioning.\nAllowlist API endpoint that returns the source IP addresses to allow for the Fleet Management instance."]
    pub fn fleet_management_allowlist_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fleet_management_allowlist_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_management_name` after provisioning.\nName of the Fleet Management instance configured for this stack."]
    pub fn fleet_management_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fleet_management_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_management_private_connectivity_info_availability_zone_ids` after provisioning.\nAvailability Zone IDs for Fleet Management when using AWS PrivateLink (only for AWS stacks)"]
    pub fn fleet_management_private_connectivity_info_availability_zone_ids(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.fleet_management_private_connectivity_info_availability_zone_ids",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_management_private_connectivity_info_availability_zones` after provisioning.\nAvailability Zones for Fleet Management when using AWS PrivateLink (only for AWS stacks)"]
    pub fn fleet_management_private_connectivity_info_availability_zones(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.fleet_management_private_connectivity_info_availability_zones",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_management_private_connectivity_info_private_dns` after provisioning.\nPrivate DNS for Fleet Management when using AWS PrivateLink (only for AWS stacks)"]
    pub fn fleet_management_private_connectivity_info_private_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.fleet_management_private_connectivity_info_private_dns",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_management_private_connectivity_info_regions` after provisioning.\nRegions for Fleet Management when using AWS PrivateLink (only for AWS stacks)"]
    pub fn fleet_management_private_connectivity_info_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.fleet_management_private_connectivity_info_regions",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_management_private_connectivity_info_service_name` after provisioning.\nService Name for Fleet Management when using AWS PrivateLink (only for AWS stacks)"]
    pub fn fleet_management_private_connectivity_info_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.fleet_management_private_connectivity_info_service_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_management_status` after provisioning.\nStatus of the Fleet Management instance configured for this stack."]
    pub fn fleet_management_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fleet_management_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_management_url` after provisioning.\nBase URL of the Fleet Management instance configured for this stack."]
    pub fn fleet_management_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fleet_management_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_management_user_id` after provisioning.\nUser ID of the Fleet Management instance configured for this stack."]
    pub fn fleet_management_user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fleet_management_user_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `grafanas_allowlist_url` after provisioning.\nAllowlist API endpoint that returns the source IP addresses to allow for the grafana instance."]
    pub fn grafanas_allowlist_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.grafanas_allowlist_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `grafanas_ip_allow_list_cname` after provisioning.\nComma-separated list of CNAMEs that can be whitelisted to access the grafana instance (Optional)"]
    pub fn grafanas_ip_allow_list_cname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.grafanas_ip_allow_list_cname", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `graphite_allowlist_url` after provisioning.\nAllowlist API endpoint that returns the source IP addresses to allow for the Graphite instance."]
    pub fn graphite_allowlist_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.graphite_allowlist_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `graphite_ip_allow_list_cname` after provisioning.\nComma-separated list of CNAMEs that can be whitelisted to access the Graphite instance (Optional)"]
    pub fn graphite_ip_allow_list_cname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.graphite_ip_allow_list_cname", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `graphite_name` after provisioning.\n"]
    pub fn graphite_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.graphite_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `graphite_private_connectivity_info_availability_zone_ids` after provisioning.\nAvailability Zone IDs for Graphite when using AWS PrivateLink (only for AWS stacks)"]
    pub fn graphite_private_connectivity_info_availability_zone_ids(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.graphite_private_connectivity_info_availability_zone_ids",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `graphite_private_connectivity_info_availability_zones` after provisioning.\nAvailability Zones for Graphite when using AWS PrivateLink (only for AWS stacks)"]
    pub fn graphite_private_connectivity_info_availability_zones(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.graphite_private_connectivity_info_availability_zones",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `graphite_private_connectivity_info_private_dns` after provisioning.\nPrivate DNS for Graphite when using AWS PrivateLink (only for AWS stacks)"]
    pub fn graphite_private_connectivity_info_private_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.graphite_private_connectivity_info_private_dns",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `graphite_private_connectivity_info_regions` after provisioning.\nRegions for Graphite when using AWS PrivateLink (only for AWS stacks)"]
    pub fn graphite_private_connectivity_info_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.graphite_private_connectivity_info_regions",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `graphite_private_connectivity_info_service_name` after provisioning.\nService Name for Graphite when using AWS PrivateLink (only for AWS stacks)"]
    pub fn graphite_private_connectivity_info_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.graphite_private_connectivity_info_service_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `graphite_status` after provisioning.\n"]
    pub fn graphite_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.graphite_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `graphite_url` after provisioning.\n"]
    pub fn graphite_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.graphite_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `graphite_user_id` after provisioning.\n"]
    pub fn graphite_user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.graphite_user_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe stack id assigned to this stack by Grafana."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `influx_url` after provisioning.\nBase URL of the InfluxDB instance configured for this stack. The username is the same as the metrics' (`prometheus_user_id` attribute of this resource). See https://grafana.com/docs/grafana-cloud/send-data/metrics/metrics-influxdb/push-from-telegraf/ for docs on how to use this."]
    pub fn influx_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.influx_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA map of labels to assign to the stack. Label keys and values must match the following regexp: \"^[a-zA-Z0-9/\\\\-._]+$\" and stacks cannot have more than 10 labels."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logs_allowlist_url` after provisioning.\nAllowlist API endpoint that returns the source IP addresses to allow for the Logs instance."]
    pub fn logs_allowlist_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logs_allowlist_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logs_ip_allow_list_cname` after provisioning.\nComma-separated list of CNAMEs that can be whitelisted to access the Logs instance (Optional)"]
    pub fn logs_ip_allow_list_cname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logs_ip_allow_list_cname", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logs_name` after provisioning.\n"]
    pub fn logs_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logs_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logs_private_connectivity_info_availability_zone_ids` after provisioning.\nAvailability Zone IDs for Logs when using AWS PrivateLink (only for AWS stacks)"]
    pub fn logs_private_connectivity_info_availability_zone_ids(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.logs_private_connectivity_info_availability_zone_ids",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `logs_private_connectivity_info_availability_zones` after provisioning.\nAvailability Zones for Logs when using AWS PrivateLink (only for AWS stacks)"]
    pub fn logs_private_connectivity_info_availability_zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.logs_private_connectivity_info_availability_zones",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `logs_private_connectivity_info_private_dns` after provisioning.\nPrivate DNS for Logs when using AWS PrivateLink (only for AWS stacks)"]
    pub fn logs_private_connectivity_info_private_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.logs_private_connectivity_info_private_dns",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `logs_private_connectivity_info_regions` after provisioning.\nRegions for Logs when using AWS PrivateLink (only for AWS stacks)"]
    pub fn logs_private_connectivity_info_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.logs_private_connectivity_info_regions",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `logs_private_connectivity_info_service_name` after provisioning.\nService Name for Logs when using AWS PrivateLink (only for AWS stacks)"]
    pub fn logs_private_connectivity_info_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.logs_private_connectivity_info_service_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `logs_status` after provisioning.\n"]
    pub fn logs_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logs_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logs_url` after provisioning.\n"]
    pub fn logs_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logs_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logs_user_id` after provisioning.\n"]
    pub fn logs_user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logs_user_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of stack. Conventionally matches the url of the instance (e.g. `<stack_slug>.grafana.net`)."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oncall_api_url` after provisioning.\nBase URL of the OnCall API instance configured for this stack."]
    pub fn oncall_api_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oncall_api_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nOrganization id to assign to this stack."]
    pub fn org_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_name` after provisioning.\nOrganization name to assign to this stack."]
    pub fn org_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_slug` after provisioning.\nOrganization slug to assign to this stack."]
    pub fn org_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_slug", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `otlp_private_connectivity_info_availability_zone_ids` after provisioning.\nAvailability Zone IDs for OTLP when using AWS PrivateLink (only for AWS stacks)"]
    pub fn otlp_private_connectivity_info_availability_zone_ids(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.otlp_private_connectivity_info_availability_zone_ids",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `otlp_private_connectivity_info_availability_zones` after provisioning.\nAvailability Zones for OTLP when using AWS PrivateLink (only for AWS stacks)"]
    pub fn otlp_private_connectivity_info_availability_zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.otlp_private_connectivity_info_availability_zones",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `otlp_private_connectivity_info_private_dns` after provisioning.\nPrivate DNS for OTLP when using AWS PrivateLink (only for AWS stacks)"]
    pub fn otlp_private_connectivity_info_private_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.otlp_private_connectivity_info_private_dns",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `otlp_private_connectivity_info_regions` after provisioning.\nRegions for OTLP when using AWS PrivateLink (only for AWS stacks)"]
    pub fn otlp_private_connectivity_info_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.otlp_private_connectivity_info_regions",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `otlp_private_connectivity_info_service_name` after provisioning.\nService Name for OTLP when using AWS PrivateLink (only for AWS stacks)"]
    pub fn otlp_private_connectivity_info_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.otlp_private_connectivity_info_service_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `otlp_url` after provisioning.\nBase URL of the OTLP instance configured for this stack. The username is the stack's ID (`id` attribute of this resource). See https://grafana.com/docs/grafana-cloud/send-data/otlp/send-data-otlp/ for docs on how to use this."]
    pub fn otlp_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.otlp_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pdc_api_private_connectivity_info_availability_zone_ids` after provisioning.\nAvailability Zone IDs for PDC's API when using AWS PrivateLink (only for AWS stacks)"]
    pub fn pdc_api_private_connectivity_info_availability_zone_ids(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.pdc_api_private_connectivity_info_availability_zone_ids",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `pdc_api_private_connectivity_info_availability_zones` after provisioning.\nAvailability Zones for PDC's API when using AWS PrivateLink (only for AWS stacks)"]
    pub fn pdc_api_private_connectivity_info_availability_zones(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.pdc_api_private_connectivity_info_availability_zones",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `pdc_api_private_connectivity_info_private_dns` after provisioning.\nPrivate DNS for PDC's API when using AWS PrivateLink (only for AWS stacks)"]
    pub fn pdc_api_private_connectivity_info_private_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.pdc_api_private_connectivity_info_private_dns",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `pdc_api_private_connectivity_info_regions` after provisioning.\nRegions for PDC's API when using AWS PrivateLink (only for AWS stacks)"]
    pub fn pdc_api_private_connectivity_info_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.pdc_api_private_connectivity_info_regions",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `pdc_api_private_connectivity_info_service_name` after provisioning.\nService Name for PDC's API when using AWS PrivateLink (only for AWS stacks)"]
    pub fn pdc_api_private_connectivity_info_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.pdc_api_private_connectivity_info_service_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `pdc_gateway_private_connectivity_info_availability_zone_ids` after provisioning.\nAvailability Zone IDs for PDC's Gateway when using AWS PrivateLink (only for AWS stacks)"]
    pub fn pdc_gateway_private_connectivity_info_availability_zone_ids(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.pdc_gateway_private_connectivity_info_availability_zone_ids",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `pdc_gateway_private_connectivity_info_availability_zones` after provisioning.\nAvailability Zones for PDC's Gateway when using AWS PrivateLink (only for AWS stacks)"]
    pub fn pdc_gateway_private_connectivity_info_availability_zones(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.pdc_gateway_private_connectivity_info_availability_zones",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `pdc_gateway_private_connectivity_info_private_dns` after provisioning.\nPrivate DNS for PDC's Gateway when using AWS PrivateLink (only for AWS stacks)"]
    pub fn pdc_gateway_private_connectivity_info_private_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.pdc_gateway_private_connectivity_info_private_dns",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `pdc_gateway_private_connectivity_info_regions` after provisioning.\nRegions for PDC's Gateway when using AWS PrivateLink (only for AWS stacks)"]
    pub fn pdc_gateway_private_connectivity_info_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.pdc_gateway_private_connectivity_info_regions",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `pdc_gateway_private_connectivity_info_service_name` after provisioning.\nService Name for PDC's Gateway when using AWS PrivateLink (only for AWS stacks)"]
    pub fn pdc_gateway_private_connectivity_info_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.pdc_gateway_private_connectivity_info_service_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `profiles_allowlist_url` after provisioning.\nAllowlist API endpoint that returns the source IP addresses to allow for the Profiles instance."]
    pub fn profiles_allowlist_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.profiles_allowlist_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `profiles_ip_allow_list_cname` after provisioning.\nComma-separated list of CNAMEs that can be whitelisted to access the Profiles instance (Optional)"]
    pub fn profiles_ip_allow_list_cname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.profiles_ip_allow_list_cname", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `profiles_name` after provisioning.\n"]
    pub fn profiles_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.profiles_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `profiles_private_connectivity_info_availability_zone_ids` after provisioning.\nAvailability Zone IDs for Profiles when using AWS PrivateLink (only for AWS stacks)"]
    pub fn profiles_private_connectivity_info_availability_zone_ids(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.profiles_private_connectivity_info_availability_zone_ids",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `profiles_private_connectivity_info_availability_zones` after provisioning.\nAvailability Zones for Profiles when using AWS PrivateLink (only for AWS stacks)"]
    pub fn profiles_private_connectivity_info_availability_zones(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.profiles_private_connectivity_info_availability_zones",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `profiles_private_connectivity_info_private_dns` after provisioning.\nPrivate DNS for Profiles when using AWS PrivateLink (only for AWS stacks)"]
    pub fn profiles_private_connectivity_info_private_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.profiles_private_connectivity_info_private_dns",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `profiles_private_connectivity_info_regions` after provisioning.\nRegions for Profiles when using AWS PrivateLink (only for AWS stacks)"]
    pub fn profiles_private_connectivity_info_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.profiles_private_connectivity_info_regions",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `profiles_private_connectivity_info_service_name` after provisioning.\nService Name for Profiles when using AWS PrivateLink (only for AWS stacks)"]
    pub fn profiles_private_connectivity_info_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.profiles_private_connectivity_info_service_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `profiles_status` after provisioning.\n"]
    pub fn profiles_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.profiles_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `profiles_url` after provisioning.\n"]
    pub fn profiles_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.profiles_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `profiles_user_id` after provisioning.\n"]
    pub fn profiles_user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.profiles_user_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_allowlist_url` after provisioning.\nAllowlist API endpoint that returns the source IP addresses to allow for the Prometheus instance."]
    pub fn prometheus_allowlist_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prometheus_allowlist_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_ip_allow_list_cname` after provisioning.\nComma-separated list of CNAMEs that can be whitelisted to access the Prometheus instance (Optional)"]
    pub fn prometheus_ip_allow_list_cname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prometheus_ip_allow_list_cname", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_name` after provisioning.\nPrometheus name for this instance."]
    pub fn prometheus_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prometheus_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_private_connectivity_info_availability_zone_ids` after provisioning.\nAvailability Zone IDs for Prometheus when using AWS PrivateLink (only for AWS stacks)"]
    pub fn prometheus_private_connectivity_info_availability_zone_ids(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.prometheus_private_connectivity_info_availability_zone_ids",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_private_connectivity_info_availability_zones` after provisioning.\nAvailability Zones for Prometheus when using AWS PrivateLink (only for AWS stacks)"]
    pub fn prometheus_private_connectivity_info_availability_zones(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.prometheus_private_connectivity_info_availability_zones",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_private_connectivity_info_private_dns` after provisioning.\nPrivate DNS for Prometheus when using AWS PrivateLink (only for AWS stacks)"]
    pub fn prometheus_private_connectivity_info_private_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.prometheus_private_connectivity_info_private_dns",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_private_connectivity_info_regions` after provisioning.\nRegions for Prometheus when using AWS PrivateLink (only for AWS stacks)"]
    pub fn prometheus_private_connectivity_info_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.prometheus_private_connectivity_info_regions",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_private_connectivity_info_service_name` after provisioning.\nService Name for Prometheus when using AWS PrivateLink (only for AWS stacks)"]
    pub fn prometheus_private_connectivity_info_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.prometheus_private_connectivity_info_service_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_remote_endpoint` after provisioning.\nUse this URL to query hosted metrics data e.g. Prometheus data source in Grafana"]
    pub fn prometheus_remote_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prometheus_remote_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_remote_write_endpoint` after provisioning.\nUse this URL to send prometheus metrics to Grafana cloud"]
    pub fn prometheus_remote_write_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prometheus_remote_write_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_status` after provisioning.\nPrometheus status for this instance."]
    pub fn prometheus_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prometheus_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_url` after provisioning.\nPrometheus url for this instance."]
    pub fn prometheus_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prometheus_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_user_id` after provisioning.\nPrometheus user ID. Used for e.g. remote_write."]
    pub fn prometheus_user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prometheus_user_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region_slug` after provisioning.\nThe region this stack is deployed to."]
    pub fn region_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region_slug", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `slug` after provisioning.\nSubdomain that the Grafana instance will be available at (i.e. setting slug to “<stack_slug>” will make the instance\navailable at “https://<stack_slug>.grafana.net\"."]
    pub fn slug(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.slug", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sm_url` after provisioning.\nBase URL of the Synthetic Monitoring API for this stack's region. This can be used with the `sm_url` provider config option. Note: Synthetic Monitoring requires activation either via the `grafana_synthetic_monitoring_installation` resource or manually in the Grafana Cloud UI before it can be used."]
    pub fn sm_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sm_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus of the stack."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `traces_allowlist_url` after provisioning.\nAllowlist API endpoint that returns the source IP addresses to allow for the Traces instance."]
    pub fn traces_allowlist_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.traces_allowlist_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `traces_ip_allow_list_cname` after provisioning.\nComma-separated list of CNAMEs that can be whitelisted to access the Traces instance (Optional)"]
    pub fn traces_ip_allow_list_cname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.traces_ip_allow_list_cname", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `traces_name` after provisioning.\n"]
    pub fn traces_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.traces_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `traces_private_connectivity_info_availability_zone_ids` after provisioning.\nAvailability Zone IDs for Traces when using AWS PrivateLink (only for AWS stacks)"]
    pub fn traces_private_connectivity_info_availability_zone_ids(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.traces_private_connectivity_info_availability_zone_ids",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `traces_private_connectivity_info_availability_zones` after provisioning.\nAvailability Zones for Traces when using AWS PrivateLink (only for AWS stacks)"]
    pub fn traces_private_connectivity_info_availability_zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.traces_private_connectivity_info_availability_zones",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `traces_private_connectivity_info_private_dns` after provisioning.\nPrivate DNS for Traces when using AWS PrivateLink (only for AWS stacks)"]
    pub fn traces_private_connectivity_info_private_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.traces_private_connectivity_info_private_dns",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `traces_private_connectivity_info_regions` after provisioning.\nRegions for Traces when using AWS PrivateLink (only for AWS stacks)"]
    pub fn traces_private_connectivity_info_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.traces_private_connectivity_info_regions",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `traces_private_connectivity_info_service_name` after provisioning.\nService Name for Traces when using AWS PrivateLink (only for AWS stacks)"]
    pub fn traces_private_connectivity_info_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.traces_private_connectivity_info_service_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `traces_status` after provisioning.\n"]
    pub fn traces_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.traces_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `traces_url` after provisioning.\nBase URL of the Traces instance configured for this stack. To use this in the Tempo data source in Grafana, append `/tempo` to the URL."]
    pub fn traces_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.traces_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `traces_user_id` after provisioning.\n"]
    pub fn traces_user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.traces_user_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nCustom URL for the Grafana instance. Must have a CNAME setup to point to `.grafana.net` before creating the stack"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}
impl Referable for DataCloudStack {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataCloudStack {}
impl ToListMappable for DataCloudStack {
    type O = ListRef<DataCloudStackRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataCloudStack_ {
    fn extract_datasource_type(&self) -> String {
        "grafana_cloud_stack".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCloudStack {
    pub tf_id: String,
    #[doc = "Subdomain that the Grafana instance will be available at (i.e. setting slug to “<stack_slug>” will make the instance\navailable at “https://<stack_slug>.grafana.net\"."]
    pub slug: PrimField<String>,
}
impl BuildDataCloudStack {
    pub fn build(self, stack: &mut Stack) -> DataCloudStack {
        let out = DataCloudStack(Rc::new(DataCloudStack_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCloudStackData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                slug: self.slug,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataCloudStackRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudStackRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCloudStackRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `alertmanager_allowlist_url` after provisioning.\nAllowlist API endpoint that returns the source IP addresses to allow for the Alertmanager instances."]
    pub fn alertmanager_allowlist_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alertmanager_allowlist_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alertmanager_ip_allow_list_cname` after provisioning.\nComma-separated list of CNAMEs that can be whitelisted to access the Alertmanager instances (Optional)"]
    pub fn alertmanager_ip_allow_list_cname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alertmanager_ip_allow_list_cname", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alertmanager_name` after provisioning.\nName of the Alertmanager instance configured for this stack."]
    pub fn alertmanager_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alertmanager_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alertmanager_status` after provisioning.\nStatus of the Alertmanager instance configured for this stack."]
    pub fn alertmanager_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alertmanager_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alertmanager_url` after provisioning.\nBase URL of the Alertmanager instance configured for this stack."]
    pub fn alertmanager_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alertmanager_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `alertmanager_user_id` after provisioning.\nUser ID of the Alertmanager instance configured for this stack."]
    pub fn alertmanager_user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alertmanager_user_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_provider_url` after provisioning.\nBase URL of the Cloud Provider API for this stack's cluster. This can be used with the `cloud_provider_url` provider config option to manage Cloud Provider resources for this stack."]
    pub fn cloud_provider_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_provider_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\nName of the cluster where this stack resides."]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_slug` after provisioning.\nSlug of the cluster where this stack resides."]
    pub fn cluster_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_slug", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connections_api_url` after provisioning.\nBase URL of the Connections API for this stack's cluster. This can be used with the `connections_api_url` provider config option to manage Connections resources for this stack."]
    pub fn connections_api_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connections_api_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_protection` after provisioning.\nWhether to enable delete protection for the stack, preventing accidental deletion."]
    pub fn delete_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of stack."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_management_allowlist_url` after provisioning.\nAllowlist API endpoint that returns the source IP addresses to allow for the Fleet Management instance."]
    pub fn fleet_management_allowlist_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fleet_management_allowlist_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_management_name` after provisioning.\nName of the Fleet Management instance configured for this stack."]
    pub fn fleet_management_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fleet_management_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_management_private_connectivity_info_availability_zone_ids` after provisioning.\nAvailability Zone IDs for Fleet Management when using AWS PrivateLink (only for AWS stacks)"]
    pub fn fleet_management_private_connectivity_info_availability_zone_ids(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.fleet_management_private_connectivity_info_availability_zone_ids",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_management_private_connectivity_info_availability_zones` after provisioning.\nAvailability Zones for Fleet Management when using AWS PrivateLink (only for AWS stacks)"]
    pub fn fleet_management_private_connectivity_info_availability_zones(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.fleet_management_private_connectivity_info_availability_zones",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_management_private_connectivity_info_private_dns` after provisioning.\nPrivate DNS for Fleet Management when using AWS PrivateLink (only for AWS stacks)"]
    pub fn fleet_management_private_connectivity_info_private_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.fleet_management_private_connectivity_info_private_dns",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_management_private_connectivity_info_regions` after provisioning.\nRegions for Fleet Management when using AWS PrivateLink (only for AWS stacks)"]
    pub fn fleet_management_private_connectivity_info_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.fleet_management_private_connectivity_info_regions",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_management_private_connectivity_info_service_name` after provisioning.\nService Name for Fleet Management when using AWS PrivateLink (only for AWS stacks)"]
    pub fn fleet_management_private_connectivity_info_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.fleet_management_private_connectivity_info_service_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_management_status` after provisioning.\nStatus of the Fleet Management instance configured for this stack."]
    pub fn fleet_management_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fleet_management_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_management_url` after provisioning.\nBase URL of the Fleet Management instance configured for this stack."]
    pub fn fleet_management_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fleet_management_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet_management_user_id` after provisioning.\nUser ID of the Fleet Management instance configured for this stack."]
    pub fn fleet_management_user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fleet_management_user_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `grafanas_allowlist_url` after provisioning.\nAllowlist API endpoint that returns the source IP addresses to allow for the grafana instance."]
    pub fn grafanas_allowlist_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.grafanas_allowlist_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `grafanas_ip_allow_list_cname` after provisioning.\nComma-separated list of CNAMEs that can be whitelisted to access the grafana instance (Optional)"]
    pub fn grafanas_ip_allow_list_cname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.grafanas_ip_allow_list_cname", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `graphite_allowlist_url` after provisioning.\nAllowlist API endpoint that returns the source IP addresses to allow for the Graphite instance."]
    pub fn graphite_allowlist_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.graphite_allowlist_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `graphite_ip_allow_list_cname` after provisioning.\nComma-separated list of CNAMEs that can be whitelisted to access the Graphite instance (Optional)"]
    pub fn graphite_ip_allow_list_cname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.graphite_ip_allow_list_cname", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `graphite_name` after provisioning.\n"]
    pub fn graphite_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.graphite_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `graphite_private_connectivity_info_availability_zone_ids` after provisioning.\nAvailability Zone IDs for Graphite when using AWS PrivateLink (only for AWS stacks)"]
    pub fn graphite_private_connectivity_info_availability_zone_ids(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.graphite_private_connectivity_info_availability_zone_ids",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `graphite_private_connectivity_info_availability_zones` after provisioning.\nAvailability Zones for Graphite when using AWS PrivateLink (only for AWS stacks)"]
    pub fn graphite_private_connectivity_info_availability_zones(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.graphite_private_connectivity_info_availability_zones",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `graphite_private_connectivity_info_private_dns` after provisioning.\nPrivate DNS for Graphite when using AWS PrivateLink (only for AWS stacks)"]
    pub fn graphite_private_connectivity_info_private_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.graphite_private_connectivity_info_private_dns",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `graphite_private_connectivity_info_regions` after provisioning.\nRegions for Graphite when using AWS PrivateLink (only for AWS stacks)"]
    pub fn graphite_private_connectivity_info_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.graphite_private_connectivity_info_regions",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `graphite_private_connectivity_info_service_name` after provisioning.\nService Name for Graphite when using AWS PrivateLink (only for AWS stacks)"]
    pub fn graphite_private_connectivity_info_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.graphite_private_connectivity_info_service_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `graphite_status` after provisioning.\n"]
    pub fn graphite_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.graphite_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `graphite_url` after provisioning.\n"]
    pub fn graphite_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.graphite_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `graphite_user_id` after provisioning.\n"]
    pub fn graphite_user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.graphite_user_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe stack id assigned to this stack by Grafana."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `influx_url` after provisioning.\nBase URL of the InfluxDB instance configured for this stack. The username is the same as the metrics' (`prometheus_user_id` attribute of this resource). See https://grafana.com/docs/grafana-cloud/send-data/metrics/metrics-influxdb/push-from-telegraf/ for docs on how to use this."]
    pub fn influx_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.influx_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA map of labels to assign to the stack. Label keys and values must match the following regexp: \"^[a-zA-Z0-9/\\\\-._]+$\" and stacks cannot have more than 10 labels."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logs_allowlist_url` after provisioning.\nAllowlist API endpoint that returns the source IP addresses to allow for the Logs instance."]
    pub fn logs_allowlist_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logs_allowlist_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logs_ip_allow_list_cname` after provisioning.\nComma-separated list of CNAMEs that can be whitelisted to access the Logs instance (Optional)"]
    pub fn logs_ip_allow_list_cname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logs_ip_allow_list_cname", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logs_name` after provisioning.\n"]
    pub fn logs_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logs_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logs_private_connectivity_info_availability_zone_ids` after provisioning.\nAvailability Zone IDs for Logs when using AWS PrivateLink (only for AWS stacks)"]
    pub fn logs_private_connectivity_info_availability_zone_ids(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.logs_private_connectivity_info_availability_zone_ids",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `logs_private_connectivity_info_availability_zones` after provisioning.\nAvailability Zones for Logs when using AWS PrivateLink (only for AWS stacks)"]
    pub fn logs_private_connectivity_info_availability_zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.logs_private_connectivity_info_availability_zones",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `logs_private_connectivity_info_private_dns` after provisioning.\nPrivate DNS for Logs when using AWS PrivateLink (only for AWS stacks)"]
    pub fn logs_private_connectivity_info_private_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.logs_private_connectivity_info_private_dns",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `logs_private_connectivity_info_regions` after provisioning.\nRegions for Logs when using AWS PrivateLink (only for AWS stacks)"]
    pub fn logs_private_connectivity_info_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.logs_private_connectivity_info_regions",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `logs_private_connectivity_info_service_name` after provisioning.\nService Name for Logs when using AWS PrivateLink (only for AWS stacks)"]
    pub fn logs_private_connectivity_info_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.logs_private_connectivity_info_service_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `logs_status` after provisioning.\n"]
    pub fn logs_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logs_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logs_url` after provisioning.\n"]
    pub fn logs_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logs_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logs_user_id` after provisioning.\n"]
    pub fn logs_user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logs_user_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of stack. Conventionally matches the url of the instance (e.g. `<stack_slug>.grafana.net`)."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oncall_api_url` after provisioning.\nBase URL of the OnCall API instance configured for this stack."]
    pub fn oncall_api_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oncall_api_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nOrganization id to assign to this stack."]
    pub fn org_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_name` after provisioning.\nOrganization name to assign to this stack."]
    pub fn org_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_slug` after provisioning.\nOrganization slug to assign to this stack."]
    pub fn org_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_slug", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `otlp_private_connectivity_info_availability_zone_ids` after provisioning.\nAvailability Zone IDs for OTLP when using AWS PrivateLink (only for AWS stacks)"]
    pub fn otlp_private_connectivity_info_availability_zone_ids(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.otlp_private_connectivity_info_availability_zone_ids",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `otlp_private_connectivity_info_availability_zones` after provisioning.\nAvailability Zones for OTLP when using AWS PrivateLink (only for AWS stacks)"]
    pub fn otlp_private_connectivity_info_availability_zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.otlp_private_connectivity_info_availability_zones",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `otlp_private_connectivity_info_private_dns` after provisioning.\nPrivate DNS for OTLP when using AWS PrivateLink (only for AWS stacks)"]
    pub fn otlp_private_connectivity_info_private_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.otlp_private_connectivity_info_private_dns",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `otlp_private_connectivity_info_regions` after provisioning.\nRegions for OTLP when using AWS PrivateLink (only for AWS stacks)"]
    pub fn otlp_private_connectivity_info_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.otlp_private_connectivity_info_regions",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `otlp_private_connectivity_info_service_name` after provisioning.\nService Name for OTLP when using AWS PrivateLink (only for AWS stacks)"]
    pub fn otlp_private_connectivity_info_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.otlp_private_connectivity_info_service_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `otlp_url` after provisioning.\nBase URL of the OTLP instance configured for this stack. The username is the stack's ID (`id` attribute of this resource). See https://grafana.com/docs/grafana-cloud/send-data/otlp/send-data-otlp/ for docs on how to use this."]
    pub fn otlp_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.otlp_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pdc_api_private_connectivity_info_availability_zone_ids` after provisioning.\nAvailability Zone IDs for PDC's API when using AWS PrivateLink (only for AWS stacks)"]
    pub fn pdc_api_private_connectivity_info_availability_zone_ids(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.pdc_api_private_connectivity_info_availability_zone_ids",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `pdc_api_private_connectivity_info_availability_zones` after provisioning.\nAvailability Zones for PDC's API when using AWS PrivateLink (only for AWS stacks)"]
    pub fn pdc_api_private_connectivity_info_availability_zones(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.pdc_api_private_connectivity_info_availability_zones",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `pdc_api_private_connectivity_info_private_dns` after provisioning.\nPrivate DNS for PDC's API when using AWS PrivateLink (only for AWS stacks)"]
    pub fn pdc_api_private_connectivity_info_private_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.pdc_api_private_connectivity_info_private_dns",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `pdc_api_private_connectivity_info_regions` after provisioning.\nRegions for PDC's API when using AWS PrivateLink (only for AWS stacks)"]
    pub fn pdc_api_private_connectivity_info_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.pdc_api_private_connectivity_info_regions",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `pdc_api_private_connectivity_info_service_name` after provisioning.\nService Name for PDC's API when using AWS PrivateLink (only for AWS stacks)"]
    pub fn pdc_api_private_connectivity_info_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.pdc_api_private_connectivity_info_service_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `pdc_gateway_private_connectivity_info_availability_zone_ids` after provisioning.\nAvailability Zone IDs for PDC's Gateway when using AWS PrivateLink (only for AWS stacks)"]
    pub fn pdc_gateway_private_connectivity_info_availability_zone_ids(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.pdc_gateway_private_connectivity_info_availability_zone_ids",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `pdc_gateway_private_connectivity_info_availability_zones` after provisioning.\nAvailability Zones for PDC's Gateway when using AWS PrivateLink (only for AWS stacks)"]
    pub fn pdc_gateway_private_connectivity_info_availability_zones(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.pdc_gateway_private_connectivity_info_availability_zones",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `pdc_gateway_private_connectivity_info_private_dns` after provisioning.\nPrivate DNS for PDC's Gateway when using AWS PrivateLink (only for AWS stacks)"]
    pub fn pdc_gateway_private_connectivity_info_private_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.pdc_gateway_private_connectivity_info_private_dns",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `pdc_gateway_private_connectivity_info_regions` after provisioning.\nRegions for PDC's Gateway when using AWS PrivateLink (only for AWS stacks)"]
    pub fn pdc_gateway_private_connectivity_info_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.pdc_gateway_private_connectivity_info_regions",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `pdc_gateway_private_connectivity_info_service_name` after provisioning.\nService Name for PDC's Gateway when using AWS PrivateLink (only for AWS stacks)"]
    pub fn pdc_gateway_private_connectivity_info_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.pdc_gateway_private_connectivity_info_service_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `profiles_allowlist_url` after provisioning.\nAllowlist API endpoint that returns the source IP addresses to allow for the Profiles instance."]
    pub fn profiles_allowlist_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.profiles_allowlist_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `profiles_ip_allow_list_cname` after provisioning.\nComma-separated list of CNAMEs that can be whitelisted to access the Profiles instance (Optional)"]
    pub fn profiles_ip_allow_list_cname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.profiles_ip_allow_list_cname", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `profiles_name` after provisioning.\n"]
    pub fn profiles_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.profiles_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `profiles_private_connectivity_info_availability_zone_ids` after provisioning.\nAvailability Zone IDs for Profiles when using AWS PrivateLink (only for AWS stacks)"]
    pub fn profiles_private_connectivity_info_availability_zone_ids(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.profiles_private_connectivity_info_availability_zone_ids",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `profiles_private_connectivity_info_availability_zones` after provisioning.\nAvailability Zones for Profiles when using AWS PrivateLink (only for AWS stacks)"]
    pub fn profiles_private_connectivity_info_availability_zones(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.profiles_private_connectivity_info_availability_zones",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `profiles_private_connectivity_info_private_dns` after provisioning.\nPrivate DNS for Profiles when using AWS PrivateLink (only for AWS stacks)"]
    pub fn profiles_private_connectivity_info_private_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.profiles_private_connectivity_info_private_dns",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `profiles_private_connectivity_info_regions` after provisioning.\nRegions for Profiles when using AWS PrivateLink (only for AWS stacks)"]
    pub fn profiles_private_connectivity_info_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.profiles_private_connectivity_info_regions",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `profiles_private_connectivity_info_service_name` after provisioning.\nService Name for Profiles when using AWS PrivateLink (only for AWS stacks)"]
    pub fn profiles_private_connectivity_info_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.profiles_private_connectivity_info_service_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `profiles_status` after provisioning.\n"]
    pub fn profiles_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.profiles_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `profiles_url` after provisioning.\n"]
    pub fn profiles_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.profiles_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `profiles_user_id` after provisioning.\n"]
    pub fn profiles_user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.profiles_user_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_allowlist_url` after provisioning.\nAllowlist API endpoint that returns the source IP addresses to allow for the Prometheus instance."]
    pub fn prometheus_allowlist_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prometheus_allowlist_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_ip_allow_list_cname` after provisioning.\nComma-separated list of CNAMEs that can be whitelisted to access the Prometheus instance (Optional)"]
    pub fn prometheus_ip_allow_list_cname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prometheus_ip_allow_list_cname", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_name` after provisioning.\nPrometheus name for this instance."]
    pub fn prometheus_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prometheus_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_private_connectivity_info_availability_zone_ids` after provisioning.\nAvailability Zone IDs for Prometheus when using AWS PrivateLink (only for AWS stacks)"]
    pub fn prometheus_private_connectivity_info_availability_zone_ids(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.prometheus_private_connectivity_info_availability_zone_ids",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_private_connectivity_info_availability_zones` after provisioning.\nAvailability Zones for Prometheus when using AWS PrivateLink (only for AWS stacks)"]
    pub fn prometheus_private_connectivity_info_availability_zones(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.prometheus_private_connectivity_info_availability_zones",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_private_connectivity_info_private_dns` after provisioning.\nPrivate DNS for Prometheus when using AWS PrivateLink (only for AWS stacks)"]
    pub fn prometheus_private_connectivity_info_private_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.prometheus_private_connectivity_info_private_dns",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_private_connectivity_info_regions` after provisioning.\nRegions for Prometheus when using AWS PrivateLink (only for AWS stacks)"]
    pub fn prometheus_private_connectivity_info_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.prometheus_private_connectivity_info_regions",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_private_connectivity_info_service_name` after provisioning.\nService Name for Prometheus when using AWS PrivateLink (only for AWS stacks)"]
    pub fn prometheus_private_connectivity_info_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.prometheus_private_connectivity_info_service_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_remote_endpoint` after provisioning.\nUse this URL to query hosted metrics data e.g. Prometheus data source in Grafana"]
    pub fn prometheus_remote_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prometheus_remote_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_remote_write_endpoint` after provisioning.\nUse this URL to send prometheus metrics to Grafana cloud"]
    pub fn prometheus_remote_write_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prometheus_remote_write_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_status` after provisioning.\nPrometheus status for this instance."]
    pub fn prometheus_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prometheus_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_url` after provisioning.\nPrometheus url for this instance."]
    pub fn prometheus_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prometheus_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prometheus_user_id` after provisioning.\nPrometheus user ID. Used for e.g. remote_write."]
    pub fn prometheus_user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prometheus_user_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region_slug` after provisioning.\nThe region this stack is deployed to."]
    pub fn region_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region_slug", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `slug` after provisioning.\nSubdomain that the Grafana instance will be available at (i.e. setting slug to “<stack_slug>” will make the instance\navailable at “https://<stack_slug>.grafana.net\"."]
    pub fn slug(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.slug", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sm_url` after provisioning.\nBase URL of the Synthetic Monitoring API for this stack's region. This can be used with the `sm_url` provider config option. Note: Synthetic Monitoring requires activation either via the `grafana_synthetic_monitoring_installation` resource or manually in the Grafana Cloud UI before it can be used."]
    pub fn sm_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sm_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus of the stack."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `traces_allowlist_url` after provisioning.\nAllowlist API endpoint that returns the source IP addresses to allow for the Traces instance."]
    pub fn traces_allowlist_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.traces_allowlist_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `traces_ip_allow_list_cname` after provisioning.\nComma-separated list of CNAMEs that can be whitelisted to access the Traces instance (Optional)"]
    pub fn traces_ip_allow_list_cname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.traces_ip_allow_list_cname", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `traces_name` after provisioning.\n"]
    pub fn traces_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.traces_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `traces_private_connectivity_info_availability_zone_ids` after provisioning.\nAvailability Zone IDs for Traces when using AWS PrivateLink (only for AWS stacks)"]
    pub fn traces_private_connectivity_info_availability_zone_ids(
        &self,
    ) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.traces_private_connectivity_info_availability_zone_ids",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `traces_private_connectivity_info_availability_zones` after provisioning.\nAvailability Zones for Traces when using AWS PrivateLink (only for AWS stacks)"]
    pub fn traces_private_connectivity_info_availability_zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.traces_private_connectivity_info_availability_zones",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `traces_private_connectivity_info_private_dns` after provisioning.\nPrivate DNS for Traces when using AWS PrivateLink (only for AWS stacks)"]
    pub fn traces_private_connectivity_info_private_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.traces_private_connectivity_info_private_dns",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `traces_private_connectivity_info_regions` after provisioning.\nRegions for Traces when using AWS PrivateLink (only for AWS stacks)"]
    pub fn traces_private_connectivity_info_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.traces_private_connectivity_info_regions",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `traces_private_connectivity_info_service_name` after provisioning.\nService Name for Traces when using AWS PrivateLink (only for AWS stacks)"]
    pub fn traces_private_connectivity_info_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.traces_private_connectivity_info_service_name",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `traces_status` after provisioning.\n"]
    pub fn traces_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.traces_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `traces_url` after provisioning.\nBase URL of the Traces instance configured for this stack. To use this in the Tempo data source in Grafana, append `/tempo` to the URL."]
    pub fn traces_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.traces_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `traces_user_id` after provisioning.\n"]
    pub fn traces_user_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.traces_user_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nCustom URL for the Grafana instance. Must have a CNAME setup to point to `.grafana.net` before creating the stack"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}
