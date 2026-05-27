use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ProviderGoogleData {
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_approval_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_context_manager_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active_directory_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    add_terraform_attribution_label: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alloydb_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apigee_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apihub_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apikeys_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_engine_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apphub_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    artifact_registry_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assured_workloads_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_dr_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    beyondcorp_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    big_query_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    biglake_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    biglake_iceberg_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bigquery_analytics_hub_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bigquery_connection_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bigquery_data_transfer_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bigquery_datapolicy_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bigquery_datapolicyv2_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bigquery_reservation_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bigtable_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_budgets_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    binary_authorization_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blockchain_node_engine_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_manager_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ces_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chronicle_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_asset_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_billing_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_build_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_functions_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_identity_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_ids_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_quotas_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_resource_manager_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_run_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_run_v2_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_scheduler_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_security_compliance_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_tasks_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudbuildv2_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    clouddeploy_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    clouddomains_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudfunctions2_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    colab_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    composer_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_center_insights_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_analysis_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_attached_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_aws_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_azure_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    core_billing_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credentials: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_catalog_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_fusion_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_lineage_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_loss_prevention_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_pipeline_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_migration_service_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dataflow_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dataform_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dataplex_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dataproc_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dataproc_gdc_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dataproc_metastore_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    datastream_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_manager_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    developer_connect_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dialogflow_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dialogflow_cx_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discovery_engine_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    document_ai_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    document_ai_warehouse_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edgecontainer_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edgenetwork_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    essential_contacts_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eventarc_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filestore_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    firebase_app_check_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    firebase_app_hosting_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    firebase_data_connect_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    firebase_remote_config_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    firebaserules_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    firestore_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gemini_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gke_backup_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gke_hub2_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gke_hub_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gkeonprem_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    healthcare_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hypercomputecluster_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam2_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam3_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_beta_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_credentials_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_workforce_pool_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iap_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_platform_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    impersonate_service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    impersonate_service_account_delegates: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    integration_connectors_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    integrations_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    looker_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lustre_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_kafka_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memcache_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memorystore_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    migration_center_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ml_engine_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_armor_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_armor_global_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitoring_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    netapp_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_connectivity_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_connectivityv1_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_management_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_security_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_services_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notebooks_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    observability_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oracle_database_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    org_policy_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os_config_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os_config_v2_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os_login_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parallelstore_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter_manager_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter_manager_regional_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    poll_interval: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefer_global_endpoints: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefer_regional_endpoints: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    privateca_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    privileged_access_manager_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_ca_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pubsub_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pubsub_lite_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recaptcha_enterprise_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redis_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager3_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_v3_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scopes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_manager_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_manager_regional_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secure_source_manager_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_center_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_center_management_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_center_v2_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    securityposture_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_directory_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_management_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_networking_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_usage_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    site_verification_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_repo_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spanner_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sql_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_batch_operations_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_control_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_insights_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_transfer_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_location_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terraform_attribution_label_addition_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transcoder_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    universe_domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_project_override: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vector_search_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vertex_ai_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vmwareengine_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_access_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workbench_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workflows_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workload_identity_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workstations_custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
struct ProviderGoogle_ {
    data: RefCell<ProviderGoogleData>,
}
pub struct ProviderGoogle(Rc<ProviderGoogle_>);
impl ProviderGoogle {
    pub fn provider_ref(&self) -> String {
        let data = self.0.data.borrow();
        if let Some(alias) = &data.alias {
            format!("{}.{}", "google", alias)
        } else {
            "google".into()
        }
    }
    pub fn set_alias(self, alias: impl ToString) -> Self {
        self.0.data.borrow_mut().alias = Some(alias.to_string());
        self
    }
    #[doc = "Set the field `access_approval_custom_endpoint`.\n"]
    pub fn set_access_approval_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().access_approval_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `access_context_manager_custom_endpoint`.\n"]
    pub fn set_access_context_manager_custom_endpoint(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .access_context_manager_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `access_token`.\n"]
    pub fn set_access_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().access_token = Some(v.into());
        self
    }
    #[doc = "Set the field `active_directory_custom_endpoint`.\n"]
    pub fn set_active_directory_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().active_directory_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `add_terraform_attribution_label`.\n"]
    pub fn set_add_terraform_attribution_label(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().add_terraform_attribution_label = Some(v.into());
        self
    }
    #[doc = "Set the field `alloydb_custom_endpoint`.\n"]
    pub fn set_alloydb_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().alloydb_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `apigee_custom_endpoint`.\n"]
    pub fn set_apigee_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().apigee_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `apihub_custom_endpoint`.\n"]
    pub fn set_apihub_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().apihub_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `apikeys_custom_endpoint`.\n"]
    pub fn set_apikeys_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().apikeys_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `app_engine_custom_endpoint`.\n"]
    pub fn set_app_engine_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().app_engine_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `apphub_custom_endpoint`.\n"]
    pub fn set_apphub_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().apphub_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `artifact_registry_custom_endpoint`.\n"]
    pub fn set_artifact_registry_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().artifact_registry_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `assured_workloads_custom_endpoint`.\n"]
    pub fn set_assured_workloads_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().assured_workloads_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_dr_custom_endpoint`.\n"]
    pub fn set_backup_dr_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().backup_dr_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `beyondcorp_custom_endpoint`.\n"]
    pub fn set_beyondcorp_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().beyondcorp_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `big_query_custom_endpoint`.\n"]
    pub fn set_big_query_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().big_query_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `biglake_custom_endpoint`.\n"]
    pub fn set_biglake_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().biglake_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `biglake_iceberg_custom_endpoint`.\n"]
    pub fn set_biglake_iceberg_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().biglake_iceberg_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `bigquery_analytics_hub_custom_endpoint`.\n"]
    pub fn set_bigquery_analytics_hub_custom_endpoint(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .bigquery_analytics_hub_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `bigquery_connection_custom_endpoint`.\n"]
    pub fn set_bigquery_connection_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().bigquery_connection_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `bigquery_data_transfer_custom_endpoint`.\n"]
    pub fn set_bigquery_data_transfer_custom_endpoint(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .bigquery_data_transfer_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `bigquery_datapolicy_custom_endpoint`.\n"]
    pub fn set_bigquery_datapolicy_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().bigquery_datapolicy_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `bigquery_datapolicyv2_custom_endpoint`.\n"]
    pub fn set_bigquery_datapolicyv2_custom_endpoint(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .bigquery_datapolicyv2_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `bigquery_reservation_custom_endpoint`.\n"]
    pub fn set_bigquery_reservation_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0
            .data
            .borrow_mut()
            .bigquery_reservation_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `bigtable_custom_endpoint`.\n"]
    pub fn set_bigtable_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().bigtable_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `billing_budgets_custom_endpoint`.\n"]
    pub fn set_billing_budgets_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().billing_budgets_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `billing_custom_endpoint`.\n"]
    pub fn set_billing_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().billing_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `billing_project`.\n"]
    pub fn set_billing_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().billing_project = Some(v.into());
        self
    }
    #[doc = "Set the field `binary_authorization_custom_endpoint`.\n"]
    pub fn set_binary_authorization_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0
            .data
            .borrow_mut()
            .binary_authorization_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `blockchain_node_engine_custom_endpoint`.\n"]
    pub fn set_blockchain_node_engine_custom_endpoint(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .blockchain_node_engine_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `certificate_manager_custom_endpoint`.\n"]
    pub fn set_certificate_manager_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().certificate_manager_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `ces_custom_endpoint`.\n"]
    pub fn set_ces_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ces_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `chronicle_custom_endpoint`.\n"]
    pub fn set_chronicle_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().chronicle_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_asset_custom_endpoint`.\n"]
    pub fn set_cloud_asset_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloud_asset_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_billing_custom_endpoint`.\n"]
    pub fn set_cloud_billing_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloud_billing_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_build_custom_endpoint`.\n"]
    pub fn set_cloud_build_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloud_build_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_functions_custom_endpoint`.\n"]
    pub fn set_cloud_functions_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloud_functions_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_identity_custom_endpoint`.\n"]
    pub fn set_cloud_identity_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloud_identity_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_ids_custom_endpoint`.\n"]
    pub fn set_cloud_ids_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloud_ids_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_quotas_custom_endpoint`.\n"]
    pub fn set_cloud_quotas_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloud_quotas_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_resource_manager_custom_endpoint`.\n"]
    pub fn set_cloud_resource_manager_custom_endpoint(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .cloud_resource_manager_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_run_custom_endpoint`.\n"]
    pub fn set_cloud_run_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloud_run_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_run_v2_custom_endpoint`.\n"]
    pub fn set_cloud_run_v2_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloud_run_v2_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_scheduler_custom_endpoint`.\n"]
    pub fn set_cloud_scheduler_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloud_scheduler_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_security_compliance_custom_endpoint`.\n"]
    pub fn set_cloud_security_compliance_custom_endpoint(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .cloud_security_compliance_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_tasks_custom_endpoint`.\n"]
    pub fn set_cloud_tasks_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloud_tasks_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `cloudbuildv2_custom_endpoint`.\n"]
    pub fn set_cloudbuildv2_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloudbuildv2_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `clouddeploy_custom_endpoint`.\n"]
    pub fn set_clouddeploy_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().clouddeploy_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `clouddomains_custom_endpoint`.\n"]
    pub fn set_clouddomains_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().clouddomains_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `cloudfunctions2_custom_endpoint`.\n"]
    pub fn set_cloudfunctions2_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloudfunctions2_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `colab_custom_endpoint`.\n"]
    pub fn set_colab_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().colab_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `composer_custom_endpoint`.\n"]
    pub fn set_composer_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().composer_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `compute_custom_endpoint`.\n"]
    pub fn set_compute_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().compute_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `contact_center_insights_custom_endpoint`.\n"]
    pub fn set_contact_center_insights_custom_endpoint(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .contact_center_insights_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `container_analysis_custom_endpoint`.\n"]
    pub fn set_container_analysis_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().container_analysis_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `container_attached_custom_endpoint`.\n"]
    pub fn set_container_attached_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().container_attached_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `container_aws_custom_endpoint`.\n"]
    pub fn set_container_aws_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().container_aws_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `container_azure_custom_endpoint`.\n"]
    pub fn set_container_azure_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().container_azure_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `container_custom_endpoint`.\n"]
    pub fn set_container_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().container_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `core_billing_custom_endpoint`.\n"]
    pub fn set_core_billing_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().core_billing_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `credentials`.\n"]
    pub fn set_credentials(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().credentials = Some(v.into());
        self
    }
    #[doc = "Set the field `data_catalog_custom_endpoint`.\n"]
    pub fn set_data_catalog_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().data_catalog_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `data_fusion_custom_endpoint`.\n"]
    pub fn set_data_fusion_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().data_fusion_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `data_lineage_custom_endpoint`.\n"]
    pub fn set_data_lineage_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().data_lineage_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `data_loss_prevention_custom_endpoint`.\n"]
    pub fn set_data_loss_prevention_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0
            .data
            .borrow_mut()
            .data_loss_prevention_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `data_pipeline_custom_endpoint`.\n"]
    pub fn set_data_pipeline_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().data_pipeline_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `database_migration_service_custom_endpoint`.\n"]
    pub fn set_database_migration_service_custom_endpoint(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .database_migration_service_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `dataflow_custom_endpoint`.\n"]
    pub fn set_dataflow_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().dataflow_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `dataform_custom_endpoint`.\n"]
    pub fn set_dataform_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().dataform_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `dataplex_custom_endpoint`.\n"]
    pub fn set_dataplex_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().dataplex_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `dataproc_custom_endpoint`.\n"]
    pub fn set_dataproc_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().dataproc_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `dataproc_gdc_custom_endpoint`.\n"]
    pub fn set_dataproc_gdc_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().dataproc_gdc_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `dataproc_metastore_custom_endpoint`.\n"]
    pub fn set_dataproc_metastore_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().dataproc_metastore_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `datastream_custom_endpoint`.\n"]
    pub fn set_datastream_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().datastream_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `default_labels`.\n"]
    pub fn set_default_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().default_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deployment_manager_custom_endpoint`.\n"]
    pub fn set_deployment_manager_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deployment_manager_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `developer_connect_custom_endpoint`.\n"]
    pub fn set_developer_connect_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().developer_connect_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `dialogflow_custom_endpoint`.\n"]
    pub fn set_dialogflow_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().dialogflow_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `dialogflow_cx_custom_endpoint`.\n"]
    pub fn set_dialogflow_cx_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().dialogflow_cx_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `discovery_engine_custom_endpoint`.\n"]
    pub fn set_discovery_engine_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().discovery_engine_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `dns_custom_endpoint`.\n"]
    pub fn set_dns_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().dns_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `document_ai_custom_endpoint`.\n"]
    pub fn set_document_ai_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().document_ai_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `document_ai_warehouse_custom_endpoint`.\n"]
    pub fn set_document_ai_warehouse_custom_endpoint(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .document_ai_warehouse_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `edgecontainer_custom_endpoint`.\n"]
    pub fn set_edgecontainer_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().edgecontainer_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `edgenetwork_custom_endpoint`.\n"]
    pub fn set_edgenetwork_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().edgenetwork_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `essential_contacts_custom_endpoint`.\n"]
    pub fn set_essential_contacts_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().essential_contacts_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `eventarc_custom_endpoint`.\n"]
    pub fn set_eventarc_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().eventarc_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `filestore_custom_endpoint`.\n"]
    pub fn set_filestore_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filestore_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `firebase_app_check_custom_endpoint`.\n"]
    pub fn set_firebase_app_check_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().firebase_app_check_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `firebase_app_hosting_custom_endpoint`.\n"]
    pub fn set_firebase_app_hosting_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0
            .data
            .borrow_mut()
            .firebase_app_hosting_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `firebase_data_connect_custom_endpoint`.\n"]
    pub fn set_firebase_data_connect_custom_endpoint(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .firebase_data_connect_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `firebase_remote_config_custom_endpoint`.\n"]
    pub fn set_firebase_remote_config_custom_endpoint(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .firebase_remote_config_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `firebaserules_custom_endpoint`.\n"]
    pub fn set_firebaserules_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().firebaserules_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `firestore_custom_endpoint`.\n"]
    pub fn set_firestore_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().firestore_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `gemini_custom_endpoint`.\n"]
    pub fn set_gemini_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().gemini_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `gke_backup_custom_endpoint`.\n"]
    pub fn set_gke_backup_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().gke_backup_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `gke_hub2_custom_endpoint`.\n"]
    pub fn set_gke_hub2_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().gke_hub2_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `gke_hub_custom_endpoint`.\n"]
    pub fn set_gke_hub_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().gke_hub_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `gkeonprem_custom_endpoint`.\n"]
    pub fn set_gkeonprem_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().gkeonprem_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `healthcare_custom_endpoint`.\n"]
    pub fn set_healthcare_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().healthcare_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `hypercomputecluster_custom_endpoint`.\n"]
    pub fn set_hypercomputecluster_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().hypercomputecluster_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `iam2_custom_endpoint`.\n"]
    pub fn set_iam2_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().iam2_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `iam3_custom_endpoint`.\n"]
    pub fn set_iam3_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().iam3_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `iam_beta_custom_endpoint`.\n"]
    pub fn set_iam_beta_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().iam_beta_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `iam_credentials_custom_endpoint`.\n"]
    pub fn set_iam_credentials_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().iam_credentials_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `iam_custom_endpoint`.\n"]
    pub fn set_iam_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().iam_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `iam_workforce_pool_custom_endpoint`.\n"]
    pub fn set_iam_workforce_pool_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().iam_workforce_pool_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `iap_custom_endpoint`.\n"]
    pub fn set_iap_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().iap_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `identity_platform_custom_endpoint`.\n"]
    pub fn set_identity_platform_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().identity_platform_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `impersonate_service_account`.\n"]
    pub fn set_impersonate_service_account(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().impersonate_service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `impersonate_service_account_delegates`.\n"]
    pub fn set_impersonate_service_account_delegates(
        self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .impersonate_service_account_delegates = Some(v.into());
        self
    }
    #[doc = "Set the field `integration_connectors_custom_endpoint`.\n"]
    pub fn set_integration_connectors_custom_endpoint(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .integration_connectors_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `integrations_custom_endpoint`.\n"]
    pub fn set_integrations_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().integrations_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_custom_endpoint`.\n"]
    pub fn set_kms_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `logging_custom_endpoint`.\n"]
    pub fn set_logging_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().logging_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `looker_custom_endpoint`.\n"]
    pub fn set_looker_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().looker_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `lustre_custom_endpoint`.\n"]
    pub fn set_lustre_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().lustre_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `managed_kafka_custom_endpoint`.\n"]
    pub fn set_managed_kafka_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().managed_kafka_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `memcache_custom_endpoint`.\n"]
    pub fn set_memcache_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().memcache_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `memorystore_custom_endpoint`.\n"]
    pub fn set_memorystore_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().memorystore_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `migration_center_custom_endpoint`.\n"]
    pub fn set_migration_center_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().migration_center_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `ml_engine_custom_endpoint`.\n"]
    pub fn set_ml_engine_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ml_engine_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `model_armor_custom_endpoint`.\n"]
    pub fn set_model_armor_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().model_armor_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `model_armor_global_custom_endpoint`.\n"]
    pub fn set_model_armor_global_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().model_armor_global_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `monitoring_custom_endpoint`.\n"]
    pub fn set_monitoring_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().monitoring_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `netapp_custom_endpoint`.\n"]
    pub fn set_netapp_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().netapp_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `network_connectivity_custom_endpoint`.\n"]
    pub fn set_network_connectivity_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0
            .data
            .borrow_mut()
            .network_connectivity_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `network_connectivityv1_custom_endpoint`.\n"]
    pub fn set_network_connectivityv1_custom_endpoint(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .network_connectivityv1_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `network_management_custom_endpoint`.\n"]
    pub fn set_network_management_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network_management_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `network_security_custom_endpoint`.\n"]
    pub fn set_network_security_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network_security_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `network_services_custom_endpoint`.\n"]
    pub fn set_network_services_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network_services_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `notebooks_custom_endpoint`.\n"]
    pub fn set_notebooks_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().notebooks_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `observability_custom_endpoint`.\n"]
    pub fn set_observability_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().observability_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `oracle_database_custom_endpoint`.\n"]
    pub fn set_oracle_database_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().oracle_database_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `org_policy_custom_endpoint`.\n"]
    pub fn set_org_policy_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().org_policy_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `os_config_custom_endpoint`.\n"]
    pub fn set_os_config_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().os_config_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `os_config_v2_custom_endpoint`.\n"]
    pub fn set_os_config_v2_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().os_config_v2_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `os_login_custom_endpoint`.\n"]
    pub fn set_os_login_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().os_login_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `parallelstore_custom_endpoint`.\n"]
    pub fn set_parallelstore_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parallelstore_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `parameter_manager_custom_endpoint`.\n"]
    pub fn set_parameter_manager_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parameter_manager_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `parameter_manager_regional_custom_endpoint`.\n"]
    pub fn set_parameter_manager_regional_custom_endpoint(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .parameter_manager_regional_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `poll_interval`.\n"]
    pub fn set_poll_interval(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().poll_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `prefer_global_endpoints`.\n"]
    pub fn set_prefer_global_endpoints(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().prefer_global_endpoints = Some(v.into());
        self
    }
    #[doc = "Set the field `prefer_regional_endpoints`.\n"]
    pub fn set_prefer_regional_endpoints(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().prefer_regional_endpoints = Some(v.into());
        self
    }
    #[doc = "Set the field `privateca_custom_endpoint`.\n"]
    pub fn set_privateca_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().privateca_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `privileged_access_manager_custom_endpoint`.\n"]
    pub fn set_privileged_access_manager_custom_endpoint(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .privileged_access_manager_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `public_ca_custom_endpoint`.\n"]
    pub fn set_public_ca_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().public_ca_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `pubsub_custom_endpoint`.\n"]
    pub fn set_pubsub_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().pubsub_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `pubsub_lite_custom_endpoint`.\n"]
    pub fn set_pubsub_lite_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().pubsub_lite_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `recaptcha_enterprise_custom_endpoint`.\n"]
    pub fn set_recaptcha_enterprise_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0
            .data
            .borrow_mut()
            .recaptcha_enterprise_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `redis_custom_endpoint`.\n"]
    pub fn set_redis_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().redis_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `request_reason`.\n"]
    pub fn set_request_reason(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().request_reason = Some(v.into());
        self
    }
    #[doc = "Set the field `request_timeout`.\n"]
    pub fn set_request_timeout(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().request_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_manager3_custom_endpoint`.\n"]
    pub fn set_resource_manager3_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().resource_manager3_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_manager_custom_endpoint`.\n"]
    pub fn set_resource_manager_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().resource_manager_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_manager_v3_custom_endpoint`.\n"]
    pub fn set_resource_manager_v3_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().resource_manager_v3_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `scopes`.\n"]
    pub fn set_scopes(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().scopes = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_manager_custom_endpoint`.\n"]
    pub fn set_secret_manager_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().secret_manager_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_manager_regional_custom_endpoint`.\n"]
    pub fn set_secret_manager_regional_custom_endpoint(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .secret_manager_regional_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `secure_source_manager_custom_endpoint`.\n"]
    pub fn set_secure_source_manager_custom_endpoint(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .secure_source_manager_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `security_center_custom_endpoint`.\n"]
    pub fn set_security_center_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().security_center_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `security_center_management_custom_endpoint`.\n"]
    pub fn set_security_center_management_custom_endpoint(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .security_center_management_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `security_center_v2_custom_endpoint`.\n"]
    pub fn set_security_center_v2_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().security_center_v2_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `securityposture_custom_endpoint`.\n"]
    pub fn set_securityposture_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().securityposture_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `service_directory_custom_endpoint`.\n"]
    pub fn set_service_directory_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_directory_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `service_management_custom_endpoint`.\n"]
    pub fn set_service_management_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_management_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `service_networking_custom_endpoint`.\n"]
    pub fn set_service_networking_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_networking_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `service_usage_custom_endpoint`.\n"]
    pub fn set_service_usage_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_usage_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `site_verification_custom_endpoint`.\n"]
    pub fn set_site_verification_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().site_verification_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `source_repo_custom_endpoint`.\n"]
    pub fn set_source_repo_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_repo_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `spanner_custom_endpoint`.\n"]
    pub fn set_spanner_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().spanner_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `sql_custom_endpoint`.\n"]
    pub fn set_sql_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sql_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_batch_operations_custom_endpoint`.\n"]
    pub fn set_storage_batch_operations_custom_endpoint(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .storage_batch_operations_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_control_custom_endpoint`.\n"]
    pub fn set_storage_control_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().storage_control_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_custom_endpoint`.\n"]
    pub fn set_storage_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().storage_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_insights_custom_endpoint`.\n"]
    pub fn set_storage_insights_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().storage_insights_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_transfer_custom_endpoint`.\n"]
    pub fn set_storage_transfer_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().storage_transfer_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `tags_custom_endpoint`.\n"]
    pub fn set_tags_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tags_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `tags_location_custom_endpoint`.\n"]
    pub fn set_tags_location_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tags_location_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `terraform_attribution_label_addition_strategy`.\n"]
    pub fn set_terraform_attribution_label_addition_strategy(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .terraform_attribution_label_addition_strategy = Some(v.into());
        self
    }
    #[doc = "Set the field `transcoder_custom_endpoint`.\n"]
    pub fn set_transcoder_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().transcoder_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `universe_domain`.\n"]
    pub fn set_universe_domain(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().universe_domain = Some(v.into());
        self
    }
    #[doc = "Set the field `user_project_override`.\n"]
    pub fn set_user_project_override(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().user_project_override = Some(v.into());
        self
    }
    #[doc = "Set the field `vector_search_custom_endpoint`.\n"]
    pub fn set_vector_search_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().vector_search_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `vertex_ai_custom_endpoint`.\n"]
    pub fn set_vertex_ai_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().vertex_ai_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `vmwareengine_custom_endpoint`.\n"]
    pub fn set_vmwareengine_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().vmwareengine_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_access_custom_endpoint`.\n"]
    pub fn set_vpc_access_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().vpc_access_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `workbench_custom_endpoint`.\n"]
    pub fn set_workbench_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().workbench_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `workflows_custom_endpoint`.\n"]
    pub fn set_workflows_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().workflows_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `workload_identity_custom_endpoint`.\n"]
    pub fn set_workload_identity_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().workload_identity_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `workstations_custom_endpoint`.\n"]
    pub fn set_workstations_custom_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().workstations_custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone = Some(v.into());
        self
    }
}
impl Provider for ProviderGoogle_ {
    fn extract_type_tf_id(&self) -> String {
        "google".into()
    }
    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json :: json ! ({ "source" : "hashicorp/google" , "version" : "7.33.0" , })
    }
    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildProviderGoogle {}
impl BuildProviderGoogle {
    pub fn build(self, stack: &mut Stack) -> ProviderGoogle {
        let out = ProviderGoogle(Rc::new(ProviderGoogle_ {
            data: RefCell::new(ProviderGoogleData {
                alias: None,
                access_approval_custom_endpoint: core::default::Default::default(),
                access_context_manager_custom_endpoint: core::default::Default::default(),
                access_token: core::default::Default::default(),
                active_directory_custom_endpoint: core::default::Default::default(),
                add_terraform_attribution_label: core::default::Default::default(),
                alloydb_custom_endpoint: core::default::Default::default(),
                apigee_custom_endpoint: core::default::Default::default(),
                apihub_custom_endpoint: core::default::Default::default(),
                apikeys_custom_endpoint: core::default::Default::default(),
                app_engine_custom_endpoint: core::default::Default::default(),
                apphub_custom_endpoint: core::default::Default::default(),
                artifact_registry_custom_endpoint: core::default::Default::default(),
                assured_workloads_custom_endpoint: core::default::Default::default(),
                backup_dr_custom_endpoint: core::default::Default::default(),
                beyondcorp_custom_endpoint: core::default::Default::default(),
                big_query_custom_endpoint: core::default::Default::default(),
                biglake_custom_endpoint: core::default::Default::default(),
                biglake_iceberg_custom_endpoint: core::default::Default::default(),
                bigquery_analytics_hub_custom_endpoint: core::default::Default::default(),
                bigquery_connection_custom_endpoint: core::default::Default::default(),
                bigquery_data_transfer_custom_endpoint: core::default::Default::default(),
                bigquery_datapolicy_custom_endpoint: core::default::Default::default(),
                bigquery_datapolicyv2_custom_endpoint: core::default::Default::default(),
                bigquery_reservation_custom_endpoint: core::default::Default::default(),
                bigtable_custom_endpoint: core::default::Default::default(),
                billing_budgets_custom_endpoint: core::default::Default::default(),
                billing_custom_endpoint: core::default::Default::default(),
                billing_project: core::default::Default::default(),
                binary_authorization_custom_endpoint: core::default::Default::default(),
                blockchain_node_engine_custom_endpoint: core::default::Default::default(),
                certificate_manager_custom_endpoint: core::default::Default::default(),
                ces_custom_endpoint: core::default::Default::default(),
                chronicle_custom_endpoint: core::default::Default::default(),
                cloud_asset_custom_endpoint: core::default::Default::default(),
                cloud_billing_custom_endpoint: core::default::Default::default(),
                cloud_build_custom_endpoint: core::default::Default::default(),
                cloud_functions_custom_endpoint: core::default::Default::default(),
                cloud_identity_custom_endpoint: core::default::Default::default(),
                cloud_ids_custom_endpoint: core::default::Default::default(),
                cloud_quotas_custom_endpoint: core::default::Default::default(),
                cloud_resource_manager_custom_endpoint: core::default::Default::default(),
                cloud_run_custom_endpoint: core::default::Default::default(),
                cloud_run_v2_custom_endpoint: core::default::Default::default(),
                cloud_scheduler_custom_endpoint: core::default::Default::default(),
                cloud_security_compliance_custom_endpoint: core::default::Default::default(),
                cloud_tasks_custom_endpoint: core::default::Default::default(),
                cloudbuildv2_custom_endpoint: core::default::Default::default(),
                clouddeploy_custom_endpoint: core::default::Default::default(),
                clouddomains_custom_endpoint: core::default::Default::default(),
                cloudfunctions2_custom_endpoint: core::default::Default::default(),
                colab_custom_endpoint: core::default::Default::default(),
                composer_custom_endpoint: core::default::Default::default(),
                compute_custom_endpoint: core::default::Default::default(),
                contact_center_insights_custom_endpoint: core::default::Default::default(),
                container_analysis_custom_endpoint: core::default::Default::default(),
                container_attached_custom_endpoint: core::default::Default::default(),
                container_aws_custom_endpoint: core::default::Default::default(),
                container_azure_custom_endpoint: core::default::Default::default(),
                container_custom_endpoint: core::default::Default::default(),
                core_billing_custom_endpoint: core::default::Default::default(),
                credentials: core::default::Default::default(),
                data_catalog_custom_endpoint: core::default::Default::default(),
                data_fusion_custom_endpoint: core::default::Default::default(),
                data_lineage_custom_endpoint: core::default::Default::default(),
                data_loss_prevention_custom_endpoint: core::default::Default::default(),
                data_pipeline_custom_endpoint: core::default::Default::default(),
                database_migration_service_custom_endpoint: core::default::Default::default(),
                dataflow_custom_endpoint: core::default::Default::default(),
                dataform_custom_endpoint: core::default::Default::default(),
                dataplex_custom_endpoint: core::default::Default::default(),
                dataproc_custom_endpoint: core::default::Default::default(),
                dataproc_gdc_custom_endpoint: core::default::Default::default(),
                dataproc_metastore_custom_endpoint: core::default::Default::default(),
                datastream_custom_endpoint: core::default::Default::default(),
                default_labels: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deployment_manager_custom_endpoint: core::default::Default::default(),
                developer_connect_custom_endpoint: core::default::Default::default(),
                dialogflow_custom_endpoint: core::default::Default::default(),
                dialogflow_cx_custom_endpoint: core::default::Default::default(),
                discovery_engine_custom_endpoint: core::default::Default::default(),
                dns_custom_endpoint: core::default::Default::default(),
                document_ai_custom_endpoint: core::default::Default::default(),
                document_ai_warehouse_custom_endpoint: core::default::Default::default(),
                edgecontainer_custom_endpoint: core::default::Default::default(),
                edgenetwork_custom_endpoint: core::default::Default::default(),
                essential_contacts_custom_endpoint: core::default::Default::default(),
                eventarc_custom_endpoint: core::default::Default::default(),
                filestore_custom_endpoint: core::default::Default::default(),
                firebase_app_check_custom_endpoint: core::default::Default::default(),
                firebase_app_hosting_custom_endpoint: core::default::Default::default(),
                firebase_data_connect_custom_endpoint: core::default::Default::default(),
                firebase_remote_config_custom_endpoint: core::default::Default::default(),
                firebaserules_custom_endpoint: core::default::Default::default(),
                firestore_custom_endpoint: core::default::Default::default(),
                gemini_custom_endpoint: core::default::Default::default(),
                gke_backup_custom_endpoint: core::default::Default::default(),
                gke_hub2_custom_endpoint: core::default::Default::default(),
                gke_hub_custom_endpoint: core::default::Default::default(),
                gkeonprem_custom_endpoint: core::default::Default::default(),
                healthcare_custom_endpoint: core::default::Default::default(),
                hypercomputecluster_custom_endpoint: core::default::Default::default(),
                iam2_custom_endpoint: core::default::Default::default(),
                iam3_custom_endpoint: core::default::Default::default(),
                iam_beta_custom_endpoint: core::default::Default::default(),
                iam_credentials_custom_endpoint: core::default::Default::default(),
                iam_custom_endpoint: core::default::Default::default(),
                iam_workforce_pool_custom_endpoint: core::default::Default::default(),
                iap_custom_endpoint: core::default::Default::default(),
                identity_platform_custom_endpoint: core::default::Default::default(),
                impersonate_service_account: core::default::Default::default(),
                impersonate_service_account_delegates: core::default::Default::default(),
                integration_connectors_custom_endpoint: core::default::Default::default(),
                integrations_custom_endpoint: core::default::Default::default(),
                kms_custom_endpoint: core::default::Default::default(),
                logging_custom_endpoint: core::default::Default::default(),
                looker_custom_endpoint: core::default::Default::default(),
                lustre_custom_endpoint: core::default::Default::default(),
                managed_kafka_custom_endpoint: core::default::Default::default(),
                memcache_custom_endpoint: core::default::Default::default(),
                memorystore_custom_endpoint: core::default::Default::default(),
                migration_center_custom_endpoint: core::default::Default::default(),
                ml_engine_custom_endpoint: core::default::Default::default(),
                model_armor_custom_endpoint: core::default::Default::default(),
                model_armor_global_custom_endpoint: core::default::Default::default(),
                monitoring_custom_endpoint: core::default::Default::default(),
                netapp_custom_endpoint: core::default::Default::default(),
                network_connectivity_custom_endpoint: core::default::Default::default(),
                network_connectivityv1_custom_endpoint: core::default::Default::default(),
                network_management_custom_endpoint: core::default::Default::default(),
                network_security_custom_endpoint: core::default::Default::default(),
                network_services_custom_endpoint: core::default::Default::default(),
                notebooks_custom_endpoint: core::default::Default::default(),
                observability_custom_endpoint: core::default::Default::default(),
                oracle_database_custom_endpoint: core::default::Default::default(),
                org_policy_custom_endpoint: core::default::Default::default(),
                os_config_custom_endpoint: core::default::Default::default(),
                os_config_v2_custom_endpoint: core::default::Default::default(),
                os_login_custom_endpoint: core::default::Default::default(),
                parallelstore_custom_endpoint: core::default::Default::default(),
                parameter_manager_custom_endpoint: core::default::Default::default(),
                parameter_manager_regional_custom_endpoint: core::default::Default::default(),
                poll_interval: core::default::Default::default(),
                prefer_global_endpoints: core::default::Default::default(),
                prefer_regional_endpoints: core::default::Default::default(),
                privateca_custom_endpoint: core::default::Default::default(),
                privileged_access_manager_custom_endpoint: core::default::Default::default(),
                project: core::default::Default::default(),
                public_ca_custom_endpoint: core::default::Default::default(),
                pubsub_custom_endpoint: core::default::Default::default(),
                pubsub_lite_custom_endpoint: core::default::Default::default(),
                recaptcha_enterprise_custom_endpoint: core::default::Default::default(),
                redis_custom_endpoint: core::default::Default::default(),
                region: core::default::Default::default(),
                request_reason: core::default::Default::default(),
                request_timeout: core::default::Default::default(),
                resource_manager3_custom_endpoint: core::default::Default::default(),
                resource_manager_custom_endpoint: core::default::Default::default(),
                resource_manager_v3_custom_endpoint: core::default::Default::default(),
                scopes: core::default::Default::default(),
                secret_manager_custom_endpoint: core::default::Default::default(),
                secret_manager_regional_custom_endpoint: core::default::Default::default(),
                secure_source_manager_custom_endpoint: core::default::Default::default(),
                security_center_custom_endpoint: core::default::Default::default(),
                security_center_management_custom_endpoint: core::default::Default::default(),
                security_center_v2_custom_endpoint: core::default::Default::default(),
                securityposture_custom_endpoint: core::default::Default::default(),
                service_directory_custom_endpoint: core::default::Default::default(),
                service_management_custom_endpoint: core::default::Default::default(),
                service_networking_custom_endpoint: core::default::Default::default(),
                service_usage_custom_endpoint: core::default::Default::default(),
                site_verification_custom_endpoint: core::default::Default::default(),
                source_repo_custom_endpoint: core::default::Default::default(),
                spanner_custom_endpoint: core::default::Default::default(),
                sql_custom_endpoint: core::default::Default::default(),
                storage_batch_operations_custom_endpoint: core::default::Default::default(),
                storage_control_custom_endpoint: core::default::Default::default(),
                storage_custom_endpoint: core::default::Default::default(),
                storage_insights_custom_endpoint: core::default::Default::default(),
                storage_transfer_custom_endpoint: core::default::Default::default(),
                tags_custom_endpoint: core::default::Default::default(),
                tags_location_custom_endpoint: core::default::Default::default(),
                terraform_attribution_label_addition_strategy: core::default::Default::default(),
                transcoder_custom_endpoint: core::default::Default::default(),
                universe_domain: core::default::Default::default(),
                user_project_override: core::default::Default::default(),
                vector_search_custom_endpoint: core::default::Default::default(),
                vertex_ai_custom_endpoint: core::default::Default::default(),
                vmwareengine_custom_endpoint: core::default::Default::default(),
                vpc_access_custom_endpoint: core::default::Default::default(),
                workbench_custom_endpoint: core::default::Default::default(),
                workflows_custom_endpoint: core::default::Default::default(),
                workload_identity_custom_endpoint: core::default::Default::default(),
                workstations_custom_endpoint: core::default::Default::default(),
                zone: core::default::Default::default(),
            }),
        }));
        stack.add_provider(out.0.clone());
        out
    }
}
