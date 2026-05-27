use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataOracleDatabaseCloudExadataInfrastructureData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cloud_exadata_infrastructure_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataOracleDatabaseCloudExadataInfrastructure_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOracleDatabaseCloudExadataInfrastructureData>,
}
#[derive(Clone)]
pub struct DataOracleDatabaseCloudExadataInfrastructure(
    Rc<DataOracleDatabaseCloudExadataInfrastructure_>,
);
impl DataOracleDatabaseCloudExadataInfrastructure {
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
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\nThe ID of the Exadata Infrastructure to create. This value is restricted\nto (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of 63\ncharacters in length. The value must start with a letter and end with\na letter or a number."]
    pub fn cloud_exadata_infrastructure_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe date and time that the Exadata Infrastructure was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether or not to allow Terraform to destroy the instance. Unless this field is set to false in Terraform state, a terraform destroy or terraform apply that would delete the instance will fail."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser friendly name for this resource."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entitlement_id` after provisioning.\nEntitlement ID of the private offer against which this infrastructure\nresource is provisioned."]
    pub fn entitlement_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entitlement_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcp_oracle_zone` after provisioning.\nGCP location where Oracle Exadata is hosted."]
    pub fn gcp_oracle_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_oracle_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels or tags associated with the resource. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. See documentation for resource type 'oracledatabase.googleapis.com/DbServer'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the Exadata Infrastructure resource with the following format:\nprojects/{project}/locations/{region}/cloudExadataInfrastructures/{cloud_exadata_infrastructure}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\nVarious properties of Exadata Infrastructure."]
    pub fn properties(
        &self,
    ) -> ListRef<DataOracleDatabaseCloudExadataInfrastructurePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
}
impl Referable for DataOracleDatabaseCloudExadataInfrastructure {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataOracleDatabaseCloudExadataInfrastructure {}
impl ToListMappable for DataOracleDatabaseCloudExadataInfrastructure {
    type O = ListRef<DataOracleDatabaseCloudExadataInfrastructureRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataOracleDatabaseCloudExadataInfrastructure_ {
    fn extract_datasource_type(&self) -> String {
        "google_oracle_database_cloud_exadata_infrastructure".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataOracleDatabaseCloudExadataInfrastructure {
    pub tf_id: String,
    #[doc = "The ID of the Exadata Infrastructure to create. This value is restricted\nto (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of 63\ncharacters in length. The value must start with a letter and end with\na letter or a number."]
    pub cloud_exadata_infrastructure_id: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. See documentation for resource type 'oracledatabase.googleapis.com/DbServer'."]
    pub location: PrimField<String>,
}
impl BuildDataOracleDatabaseCloudExadataInfrastructure {
    pub fn build(self, stack: &mut Stack) -> DataOracleDatabaseCloudExadataInfrastructure {
        let out = DataOracleDatabaseCloudExadataInfrastructure(Rc::new(
            DataOracleDatabaseCloudExadataInfrastructure_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataOracleDatabaseCloudExadataInfrastructureData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    cloud_exadata_infrastructure_id: self.cloud_exadata_infrastructure_id,
                    id: core::default::Default::default(),
                    location: self.location,
                    project: core::default::Default::default(),
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataOracleDatabaseCloudExadataInfrastructureRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseCloudExadataInfrastructureRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataOracleDatabaseCloudExadataInfrastructureRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\nThe ID of the Exadata Infrastructure to create. This value is restricted\nto (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of 63\ncharacters in length. The value must start with a letter and end with\na letter or a number."]
    pub fn cloud_exadata_infrastructure_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe date and time that the Exadata Infrastructure was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether or not to allow Terraform to destroy the instance. Unless this field is set to false in Terraform state, a terraform destroy or terraform apply that would delete the instance will fail."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser friendly name for this resource."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entitlement_id` after provisioning.\nEntitlement ID of the private offer against which this infrastructure\nresource is provisioned."]
    pub fn entitlement_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entitlement_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcp_oracle_zone` after provisioning.\nGCP location where Oracle Exadata is hosted."]
    pub fn gcp_oracle_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_oracle_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels or tags associated with the resource. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. See documentation for resource type 'oracledatabase.googleapis.com/DbServer'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the Exadata Infrastructure resource with the following format:\nprojects/{project}/locations/{region}/cloudExadataInfrastructures/{cloud_exadata_infrastructure}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\nVarious properties of Exadata Infrastructure."]
    pub fn properties(
        &self,
    ) -> ListRef<DataOracleDatabaseCloudExadataInfrastructurePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataOracleDatabaseCloudExadataInfrastructurePropertiesElCustomerContactsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
}
impl DataOracleDatabaseCloudExadataInfrastructurePropertiesElCustomerContactsEl {
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }
}
impl ToListMappable for DataOracleDatabaseCloudExadataInfrastructurePropertiesElCustomerContactsEl {
    type O =
        BlockAssignable<DataOracleDatabaseCloudExadataInfrastructurePropertiesElCustomerContactsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOracleDatabaseCloudExadataInfrastructurePropertiesElCustomerContactsEl {}
impl BuildDataOracleDatabaseCloudExadataInfrastructurePropertiesElCustomerContactsEl {
    pub fn build(
        self,
    ) -> DataOracleDatabaseCloudExadataInfrastructurePropertiesElCustomerContactsEl {
        DataOracleDatabaseCloudExadataInfrastructurePropertiesElCustomerContactsEl {
            email: core::default::Default::default(),
        }
    }
}
pub struct DataOracleDatabaseCloudExadataInfrastructurePropertiesElCustomerContactsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseCloudExadataInfrastructurePropertiesElCustomerContactsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOracleDatabaseCloudExadataInfrastructurePropertiesElCustomerContactsElRef {
        DataOracleDatabaseCloudExadataInfrastructurePropertiesElCustomerContactsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOracleDatabaseCloudExadataInfrastructurePropertiesElCustomerContactsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}
#[derive(Serialize)]
pub struct DataOracleDatabaseCloudExadataInfrastructurePropertiesElMaintenanceWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_action_timeout_mins: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    days_of_week: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hours_of_day: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_custom_action_timeout_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lead_time_week: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    months: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    patching_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preference: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weeks_of_month: Option<ListField<PrimField<f64>>>,
}
impl DataOracleDatabaseCloudExadataInfrastructurePropertiesElMaintenanceWindowEl {
    #[doc = "Set the field `custom_action_timeout_mins`.\n"]
    pub fn set_custom_action_timeout_mins(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.custom_action_timeout_mins = Some(v.into());
        self
    }
    #[doc = "Set the field `days_of_week`.\n"]
    pub fn set_days_of_week(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.days_of_week = Some(v.into());
        self
    }
    #[doc = "Set the field `hours_of_day`.\n"]
    pub fn set_hours_of_day(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.hours_of_day = Some(v.into());
        self
    }
    #[doc = "Set the field `is_custom_action_timeout_enabled`.\n"]
    pub fn set_is_custom_action_timeout_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_custom_action_timeout_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `lead_time_week`.\n"]
    pub fn set_lead_time_week(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.lead_time_week = Some(v.into());
        self
    }
    #[doc = "Set the field `months`.\n"]
    pub fn set_months(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.months = Some(v.into());
        self
    }
    #[doc = "Set the field `patching_mode`.\n"]
    pub fn set_patching_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.patching_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `preference`.\n"]
    pub fn set_preference(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.preference = Some(v.into());
        self
    }
    #[doc = "Set the field `weeks_of_month`.\n"]
    pub fn set_weeks_of_month(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.weeks_of_month = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataOracleDatabaseCloudExadataInfrastructurePropertiesElMaintenanceWindowEl
{
    type O = BlockAssignable<
        DataOracleDatabaseCloudExadataInfrastructurePropertiesElMaintenanceWindowEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOracleDatabaseCloudExadataInfrastructurePropertiesElMaintenanceWindowEl {}
impl BuildDataOracleDatabaseCloudExadataInfrastructurePropertiesElMaintenanceWindowEl {
    pub fn build(
        self,
    ) -> DataOracleDatabaseCloudExadataInfrastructurePropertiesElMaintenanceWindowEl {
        DataOracleDatabaseCloudExadataInfrastructurePropertiesElMaintenanceWindowEl {
            custom_action_timeout_mins: core::default::Default::default(),
            days_of_week: core::default::Default::default(),
            hours_of_day: core::default::Default::default(),
            is_custom_action_timeout_enabled: core::default::Default::default(),
            lead_time_week: core::default::Default::default(),
            months: core::default::Default::default(),
            patching_mode: core::default::Default::default(),
            preference: core::default::Default::default(),
            weeks_of_month: core::default::Default::default(),
        }
    }
}
pub struct DataOracleDatabaseCloudExadataInfrastructurePropertiesElMaintenanceWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseCloudExadataInfrastructurePropertiesElMaintenanceWindowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOracleDatabaseCloudExadataInfrastructurePropertiesElMaintenanceWindowElRef {
        DataOracleDatabaseCloudExadataInfrastructurePropertiesElMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOracleDatabaseCloudExadataInfrastructurePropertiesElMaintenanceWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_action_timeout_mins` after provisioning.\n"]
    pub fn custom_action_timeout_mins(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_action_timeout_mins", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `days_of_week` after provisioning.\n"]
    pub fn days_of_week(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.days_of_week", self.base))
    }
    #[doc = "Get a reference to the value of field `hours_of_day` after provisioning.\n"]
    pub fn hours_of_day(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.hours_of_day", self.base))
    }
    #[doc = "Get a reference to the value of field `is_custom_action_timeout_enabled` after provisioning.\n"]
    pub fn is_custom_action_timeout_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_custom_action_timeout_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lead_time_week` after provisioning.\n"]
    pub fn lead_time_week(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lead_time_week", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `months` after provisioning.\n"]
    pub fn months(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.months", self.base))
    }
    #[doc = "Get a reference to the value of field `patching_mode` after provisioning.\n"]
    pub fn patching_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.patching_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `preference` after provisioning.\n"]
    pub fn preference(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preference", self.base))
    }
    #[doc = "Get a reference to the value of field `weeks_of_month` after provisioning.\n"]
    pub fn weeks_of_month(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.weeks_of_month", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataOracleDatabaseCloudExadataInfrastructurePropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    activated_storage_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_storage_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    available_storage_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_contacts: Option<
        ListField<DataOracleDatabaseCloudExadataInfrastructurePropertiesElCustomerContactsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_storage_size_tb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_node_storage_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_server_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_window: Option<
        ListField<DataOracleDatabaseCloudExadataInfrastructurePropertiesElMaintenanceWindowEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_cpu_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_data_storage_tb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_db_node_storage_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_memory_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monthly_db_server_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monthly_storage_server_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_maintenance_run_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_maintenance_run_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_security_maintenance_run_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oci_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ocid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shape: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_server_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_storage_size_gb: Option<PrimField<f64>>,
}
impl DataOracleDatabaseCloudExadataInfrastructurePropertiesEl {
    #[doc = "Set the field `activated_storage_count`.\n"]
    pub fn set_activated_storage_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.activated_storage_count = Some(v.into());
        self
    }
    #[doc = "Set the field `additional_storage_count`.\n"]
    pub fn set_additional_storage_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.additional_storage_count = Some(v.into());
        self
    }
    #[doc = "Set the field `available_storage_size_gb`.\n"]
    pub fn set_available_storage_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.available_storage_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `compute_count`.\n"]
    pub fn set_compute_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.compute_count = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu_count`.\n"]
    pub fn set_cpu_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu_count = Some(v.into());
        self
    }
    #[doc = "Set the field `customer_contacts`.\n"]
    pub fn set_customer_contacts(
        mut self,
        v: impl Into<
            ListField<DataOracleDatabaseCloudExadataInfrastructurePropertiesElCustomerContactsEl>,
        >,
    ) -> Self {
        self.customer_contacts = Some(v.into());
        self
    }
    #[doc = "Set the field `data_storage_size_tb`.\n"]
    pub fn set_data_storage_size_tb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.data_storage_size_tb = Some(v.into());
        self
    }
    #[doc = "Set the field `db_node_storage_size_gb`.\n"]
    pub fn set_db_node_storage_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.db_node_storage_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `db_server_version`.\n"]
    pub fn set_db_server_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.db_server_version = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(
        mut self,
        v: impl Into<
            ListField<DataOracleDatabaseCloudExadataInfrastructurePropertiesElMaintenanceWindowEl>,
        >,
    ) -> Self {
        self.maintenance_window = Some(v.into());
        self
    }
    #[doc = "Set the field `max_cpu_count`.\n"]
    pub fn set_max_cpu_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_cpu_count = Some(v.into());
        self
    }
    #[doc = "Set the field `max_data_storage_tb`.\n"]
    pub fn set_max_data_storage_tb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_data_storage_tb = Some(v.into());
        self
    }
    #[doc = "Set the field `max_db_node_storage_size_gb`.\n"]
    pub fn set_max_db_node_storage_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_db_node_storage_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `max_memory_gb`.\n"]
    pub fn set_max_memory_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_memory_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_size_gb`.\n"]
    pub fn set_memory_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `monthly_db_server_version`.\n"]
    pub fn set_monthly_db_server_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.monthly_db_server_version = Some(v.into());
        self
    }
    #[doc = "Set the field `monthly_storage_server_version`.\n"]
    pub fn set_monthly_storage_server_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.monthly_storage_server_version = Some(v.into());
        self
    }
    #[doc = "Set the field `next_maintenance_run_id`.\n"]
    pub fn set_next_maintenance_run_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_maintenance_run_id = Some(v.into());
        self
    }
    #[doc = "Set the field `next_maintenance_run_time`.\n"]
    pub fn set_next_maintenance_run_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_maintenance_run_time = Some(v.into());
        self
    }
    #[doc = "Set the field `next_security_maintenance_run_time`.\n"]
    pub fn set_next_security_maintenance_run_time(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.next_security_maintenance_run_time = Some(v.into());
        self
    }
    #[doc = "Set the field `oci_url`.\n"]
    pub fn set_oci_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.oci_url = Some(v.into());
        self
    }
    #[doc = "Set the field `ocid`.\n"]
    pub fn set_ocid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ocid = Some(v.into());
        self
    }
    #[doc = "Set the field `shape`.\n"]
    pub fn set_shape(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.shape = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_count`.\n"]
    pub fn set_storage_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.storage_count = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_server_version`.\n"]
    pub fn set_storage_server_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_server_version = Some(v.into());
        self
    }
    #[doc = "Set the field `total_storage_size_gb`.\n"]
    pub fn set_total_storage_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.total_storage_size_gb = Some(v.into());
        self
    }
}
impl ToListMappable for DataOracleDatabaseCloudExadataInfrastructurePropertiesEl {
    type O = BlockAssignable<DataOracleDatabaseCloudExadataInfrastructurePropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOracleDatabaseCloudExadataInfrastructurePropertiesEl {}
impl BuildDataOracleDatabaseCloudExadataInfrastructurePropertiesEl {
    pub fn build(self) -> DataOracleDatabaseCloudExadataInfrastructurePropertiesEl {
        DataOracleDatabaseCloudExadataInfrastructurePropertiesEl {
            activated_storage_count: core::default::Default::default(),
            additional_storage_count: core::default::Default::default(),
            available_storage_size_gb: core::default::Default::default(),
            compute_count: core::default::Default::default(),
            cpu_count: core::default::Default::default(),
            customer_contacts: core::default::Default::default(),
            data_storage_size_tb: core::default::Default::default(),
            db_node_storage_size_gb: core::default::Default::default(),
            db_server_version: core::default::Default::default(),
            maintenance_window: core::default::Default::default(),
            max_cpu_count: core::default::Default::default(),
            max_data_storage_tb: core::default::Default::default(),
            max_db_node_storage_size_gb: core::default::Default::default(),
            max_memory_gb: core::default::Default::default(),
            memory_size_gb: core::default::Default::default(),
            monthly_db_server_version: core::default::Default::default(),
            monthly_storage_server_version: core::default::Default::default(),
            next_maintenance_run_id: core::default::Default::default(),
            next_maintenance_run_time: core::default::Default::default(),
            next_security_maintenance_run_time: core::default::Default::default(),
            oci_url: core::default::Default::default(),
            ocid: core::default::Default::default(),
            shape: core::default::Default::default(),
            state: core::default::Default::default(),
            storage_count: core::default::Default::default(),
            storage_server_version: core::default::Default::default(),
            total_storage_size_gb: core::default::Default::default(),
        }
    }
}
pub struct DataOracleDatabaseCloudExadataInfrastructurePropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseCloudExadataInfrastructurePropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOracleDatabaseCloudExadataInfrastructurePropertiesElRef {
        DataOracleDatabaseCloudExadataInfrastructurePropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOracleDatabaseCloudExadataInfrastructurePropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `activated_storage_count` after provisioning.\n"]
    pub fn activated_storage_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.activated_storage_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `additional_storage_count` after provisioning.\n"]
    pub fn additional_storage_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.additional_storage_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `available_storage_size_gb` after provisioning.\n"]
    pub fn available_storage_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_storage_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `compute_count` after provisioning.\n"]
    pub fn compute_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_count` after provisioning.\n"]
    pub fn cpu_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_count", self.base))
    }
    #[doc = "Get a reference to the value of field `customer_contacts` after provisioning.\n"]
    pub fn customer_contacts(
        &self,
    ) -> ListRef<DataOracleDatabaseCloudExadataInfrastructurePropertiesElCustomerContactsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.customer_contacts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_storage_size_tb` after provisioning.\n"]
    pub fn data_storage_size_tb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_storage_size_tb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `db_node_storage_size_gb` after provisioning.\n"]
    pub fn db_node_storage_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_node_storage_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `db_server_version` after provisioning.\n"]
    pub fn db_server_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_server_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(
        &self,
    ) -> ListRef<DataOracleDatabaseCloudExadataInfrastructurePropertiesElMaintenanceWindowElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_window", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_cpu_count` after provisioning.\n"]
    pub fn max_cpu_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_cpu_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_data_storage_tb` after provisioning.\n"]
    pub fn max_data_storage_tb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_data_storage_tb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_db_node_storage_size_gb` after provisioning.\n"]
    pub fn max_db_node_storage_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_db_node_storage_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_memory_gb` after provisioning.\n"]
    pub fn max_memory_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_memory_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_size_gb` after provisioning.\n"]
    pub fn memory_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `monthly_db_server_version` after provisioning.\n"]
    pub fn monthly_db_server_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monthly_db_server_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `monthly_storage_server_version` after provisioning.\n"]
    pub fn monthly_storage_server_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monthly_storage_server_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `next_maintenance_run_id` after provisioning.\n"]
    pub fn next_maintenance_run_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_maintenance_run_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `next_maintenance_run_time` after provisioning.\n"]
    pub fn next_maintenance_run_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_maintenance_run_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `next_security_maintenance_run_time` after provisioning.\n"]
    pub fn next_security_maintenance_run_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_security_maintenance_run_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oci_url` after provisioning.\n"]
    pub fn oci_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.base))
    }
    #[doc = "Get a reference to the value of field `ocid` after provisioning.\n"]
    pub fn ocid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.base))
    }
    #[doc = "Get a reference to the value of field `shape` after provisioning.\n"]
    pub fn shape(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shape", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_count` after provisioning.\n"]
    pub fn storage_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `storage_server_version` after provisioning.\n"]
    pub fn storage_server_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_server_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_storage_size_gb` after provisioning.\n"]
    pub fn total_storage_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_storage_size_gb", self.base),
        )
    }
}
