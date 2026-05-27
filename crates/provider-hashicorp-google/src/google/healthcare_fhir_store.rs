use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct HealthcareFhirStoreData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    complex_data_type_reference_parsing: Option<PrimField<String>>,
    dataset: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_search_handling_strict: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_referential_integrity: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_resource_versioning: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_history_import: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_update_create: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_config: Option<Vec<HealthcareFhirStoreNotificationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_configs: Option<Vec<HealthcareFhirStoreNotificationConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_configs: Option<Vec<HealthcareFhirStoreStreamConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<HealthcareFhirStoreTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_config: Option<Vec<HealthcareFhirStoreValidationConfigEl>>,
    dynamic: HealthcareFhirStoreDynamic,
}
struct HealthcareFhirStore_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<HealthcareFhirStoreData>,
}
#[derive(Clone)]
pub struct HealthcareFhirStore(Rc<HealthcareFhirStore_>);
impl HealthcareFhirStore {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }
    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }
    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }
    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => true,
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    }
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }
    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(r.extract_ref());
        self
    }
    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(attr.to_string());
        self
    }
    #[doc = "Set the field `complex_data_type_reference_parsing`.\nEnable parsing of references within complex FHIR data types such as Extensions. If this value is set to ENABLED, then features like referential integrity and Bundle reference rewriting apply to all references. If this flag has not been specified the behavior of the FHIR store will not change, references in complex data types will not be parsed. New stores will have this value set to ENABLED by default after a notification period. Warning: turning on this flag causes processing existing resources to fail if they contain references to non-existent resources. Possible values: [\"COMPLEX_DATA_TYPE_REFERENCE_PARSING_UNSPECIFIED\", \"DISABLED\", \"ENABLED\"]"]
    pub fn set_complex_data_type_reference_parsing(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().complex_data_type_reference_parsing = Some(v.into());
        self
    }
    #[doc = "Set the field `default_search_handling_strict`.\nIf true, overrides the default search behavior for this FHIR store to handling=strict which returns an error for unrecognized search parameters.\nIf false, uses the FHIR specification default handling=lenient which ignores unrecognized search parameters.\nThe handling can always be changed from the default on an individual API call by setting the HTTP header Prefer: handling=strict or Prefer: handling=lenient."]
    pub fn set_default_search_handling_strict(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().default_search_handling_strict = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_referential_integrity`.\nWhether to disable referential integrity in this FHIR store. This field is immutable after FHIR store\ncreation. The default value is false, meaning that the API will enforce referential integrity and fail the\nrequests that will result in inconsistent state in the FHIR store. When this field is set to true, the API\nwill skip referential integrity check. Consequently, operations that rely on references, such as\nPatient.get$everything, will not return all the results if broken references exist.\n\n** Changing this property may recreate the FHIR store (removing all data) **"]
    pub fn set_disable_referential_integrity(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_referential_integrity = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_resource_versioning`.\nWhether to disable resource versioning for this FHIR store. This field can not be changed after the creation\nof FHIR store. If set to false, which is the default behavior, all write operations will cause historical\nversions to be recorded automatically. The historical versions can be fetched through the history APIs, but\ncannot be updated. If set to true, no historical versions will be kept. The server will send back errors for\nattempts to read the historical versions.\n\n** Changing this property may recreate the FHIR store (removing all data) **"]
    pub fn set_disable_resource_versioning(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_resource_versioning = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_history_import`.\nWhether to allow the bulk import API to accept history bundles and directly insert historical resource\nversions into the FHIR store. Importing resource histories creates resource interactions that appear to have\noccurred in the past, which clients may not want to allow. If set to false, history bundles within an import\nwill fail with an error.\n\n** Changing this property may recreate the FHIR store (removing all data) **\n\n** This property can be changed manually in the Google Cloud Healthcare admin console without recreating the FHIR store **"]
    pub fn set_enable_history_import(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_history_import = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_update_create`.\nWhether this FHIR store has the updateCreate capability. This determines if the client can use an Update\noperation to create a new resource with a client-specified ID. If false, all IDs are server-assigned through\nthe Create operation and attempts to Update a non-existent resource will return errors. Please treat the audit\nlogs with appropriate levels of care if client-specified resource IDs contain sensitive data such as patient\nidentifiers, those IDs will be part of the FHIR resource path recorded in Cloud audit logs and Cloud Pub/Sub\nnotifications."]
    pub fn set_enable_update_create(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_update_create = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nUser-supplied key-value pairs used to organize FHIR stores.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must\nconform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\n\nLabel values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128\nbytes, and must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be associated with a given store.\n\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `notification_config`.\n"]
    pub fn set_notification_config(
        self,
        v: impl Into<BlockAssignable<HealthcareFhirStoreNotificationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `notification_configs`.\n"]
    pub fn set_notification_configs(
        self,
        v: impl Into<BlockAssignable<HealthcareFhirStoreNotificationConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification_configs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `stream_configs`.\n"]
    pub fn set_stream_configs(
        self,
        v: impl Into<BlockAssignable<HealthcareFhirStoreStreamConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stream_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stream_configs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<HealthcareFhirStoreTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `validation_config`.\n"]
    pub fn set_validation_config(
        self,
        v: impl Into<BlockAssignable<HealthcareFhirStoreValidationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().validation_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.validation_config = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `complex_data_type_reference_parsing` after provisioning.\nEnable parsing of references within complex FHIR data types such as Extensions. If this value is set to ENABLED, then features like referential integrity and Bundle reference rewriting apply to all references. If this flag has not been specified the behavior of the FHIR store will not change, references in complex data types will not be parsed. New stores will have this value set to ENABLED by default after a notification period. Warning: turning on this flag causes processing existing resources to fail if they contain references to non-existent resources. Possible values: [\"COMPLEX_DATA_TYPE_REFERENCE_PARSING_UNSPECIFIED\", \"DISABLED\", \"ENABLED\"]"]
    pub fn complex_data_type_reference_parsing(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.complex_data_type_reference_parsing", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataset` after provisioning.\nIdentifies the dataset addressed by this request. Must be in the format\n'projects/{project}/locations/{location}/datasets/{dataset}'"]
    pub fn dataset(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataset", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_search_handling_strict` after provisioning.\nIf true, overrides the default search behavior for this FHIR store to handling=strict which returns an error for unrecognized search parameters.\nIf false, uses the FHIR specification default handling=lenient which ignores unrecognized search parameters.\nThe handling can always be changed from the default on an individual API call by setting the HTTP header Prefer: handling=strict or Prefer: handling=lenient."]
    pub fn default_search_handling_strict(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_search_handling_strict", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disable_referential_integrity` after provisioning.\nWhether to disable referential integrity in this FHIR store. This field is immutable after FHIR store\ncreation. The default value is false, meaning that the API will enforce referential integrity and fail the\nrequests that will result in inconsistent state in the FHIR store. When this field is set to true, the API\nwill skip referential integrity check. Consequently, operations that rely on references, such as\nPatient.get$everything, will not return all the results if broken references exist.\n\n** Changing this property may recreate the FHIR store (removing all data) **"]
    pub fn disable_referential_integrity(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_referential_integrity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disable_resource_versioning` after provisioning.\nWhether to disable resource versioning for this FHIR store. This field can not be changed after the creation\nof FHIR store. If set to false, which is the default behavior, all write operations will cause historical\nversions to be recorded automatically. The historical versions can be fetched through the history APIs, but\ncannot be updated. If set to true, no historical versions will be kept. The server will send back errors for\nattempts to read the historical versions.\n\n** Changing this property may recreate the FHIR store (removing all data) **"]
    pub fn disable_resource_versioning(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_resource_versioning", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_history_import` after provisioning.\nWhether to allow the bulk import API to accept history bundles and directly insert historical resource\nversions into the FHIR store. Importing resource histories creates resource interactions that appear to have\noccurred in the past, which clients may not want to allow. If set to false, history bundles within an import\nwill fail with an error.\n\n** Changing this property may recreate the FHIR store (removing all data) **\n\n** This property can be changed manually in the Google Cloud Healthcare admin console without recreating the FHIR store **"]
    pub fn enable_history_import(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_history_import", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_update_create` after provisioning.\nWhether this FHIR store has the updateCreate capability. This determines if the client can use an Update\noperation to create a new resource with a client-specified ID. If false, all IDs are server-assigned through\nthe Create operation and attempts to Update a non-existent resource will return errors. Please treat the audit\nlogs with appropriate levels of care if client-specified resource IDs contain sensitive data such as patient\nidentifiers, those IDs will be part of the FHIR resource path recorded in Cloud audit logs and Cloud Pub/Sub\nnotifications."]
    pub fn enable_update_create(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_update_create", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-supplied key-value pairs used to organize FHIR stores.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must\nconform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\n\nLabel values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128\nbytes, and must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be associated with a given store.\n\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for the FhirStore.\n\n** Changing this property may recreate the FHIR store (removing all data) **"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe fully qualified name of this dataset"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe FHIR specification version. Possible values: [\"DSTU2\", \"STU3\", \"R4\"]"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(&self) -> ListRef<HealthcareFhirStoreNotificationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notification_configs` after provisioning.\n"]
    pub fn notification_configs(&self) -> ListRef<HealthcareFhirStoreNotificationConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stream_configs` after provisioning.\n"]
    pub fn stream_configs(&self) -> ListRef<HealthcareFhirStoreStreamConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stream_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> HealthcareFhirStoreTimeoutsElRef {
        HealthcareFhirStoreTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `validation_config` after provisioning.\n"]
    pub fn validation_config(&self) -> ListRef<HealthcareFhirStoreValidationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.validation_config", self.extract_ref()),
        )
    }
}
impl Referable for HealthcareFhirStore {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for HealthcareFhirStore {}
impl ToListMappable for HealthcareFhirStore {
    type O = ListRef<HealthcareFhirStoreRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for HealthcareFhirStore_ {
    fn extract_resource_type(&self) -> String {
        "google_healthcare_fhir_store".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildHealthcareFhirStore {
    pub tf_id: String,
    #[doc = "Identifies the dataset addressed by this request. Must be in the format\n'projects/{project}/locations/{location}/datasets/{dataset}'"]
    pub dataset: PrimField<String>,
    #[doc = "The resource name for the FhirStore.\n\n** Changing this property may recreate the FHIR store (removing all data) **"]
    pub name: PrimField<String>,
    #[doc = "The FHIR specification version. Possible values: [\"DSTU2\", \"STU3\", \"R4\"]"]
    pub version: PrimField<String>,
}
impl BuildHealthcareFhirStore {
    pub fn build(self, stack: &mut Stack) -> HealthcareFhirStore {
        let out = HealthcareFhirStore(Rc::new(HealthcareFhirStore_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(HealthcareFhirStoreData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                complex_data_type_reference_parsing: core::default::Default::default(),
                dataset: self.dataset,
                default_search_handling_strict: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                disable_referential_integrity: core::default::Default::default(),
                disable_resource_versioning: core::default::Default::default(),
                enable_history_import: core::default::Default::default(),
                enable_update_create: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                version: self.version,
                notification_config: core::default::Default::default(),
                notification_configs: core::default::Default::default(),
                stream_configs: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                validation_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct HealthcareFhirStoreRef {
    shared: StackShared,
    base: String,
}
impl Ref for HealthcareFhirStoreRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl HealthcareFhirStoreRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `complex_data_type_reference_parsing` after provisioning.\nEnable parsing of references within complex FHIR data types such as Extensions. If this value is set to ENABLED, then features like referential integrity and Bundle reference rewriting apply to all references. If this flag has not been specified the behavior of the FHIR store will not change, references in complex data types will not be parsed. New stores will have this value set to ENABLED by default after a notification period. Warning: turning on this flag causes processing existing resources to fail if they contain references to non-existent resources. Possible values: [\"COMPLEX_DATA_TYPE_REFERENCE_PARSING_UNSPECIFIED\", \"DISABLED\", \"ENABLED\"]"]
    pub fn complex_data_type_reference_parsing(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.complex_data_type_reference_parsing", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataset` after provisioning.\nIdentifies the dataset addressed by this request. Must be in the format\n'projects/{project}/locations/{location}/datasets/{dataset}'"]
    pub fn dataset(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataset", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_search_handling_strict` after provisioning.\nIf true, overrides the default search behavior for this FHIR store to handling=strict which returns an error for unrecognized search parameters.\nIf false, uses the FHIR specification default handling=lenient which ignores unrecognized search parameters.\nThe handling can always be changed from the default on an individual API call by setting the HTTP header Prefer: handling=strict or Prefer: handling=lenient."]
    pub fn default_search_handling_strict(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_search_handling_strict", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disable_referential_integrity` after provisioning.\nWhether to disable referential integrity in this FHIR store. This field is immutable after FHIR store\ncreation. The default value is false, meaning that the API will enforce referential integrity and fail the\nrequests that will result in inconsistent state in the FHIR store. When this field is set to true, the API\nwill skip referential integrity check. Consequently, operations that rely on references, such as\nPatient.get$everything, will not return all the results if broken references exist.\n\n** Changing this property may recreate the FHIR store (removing all data) **"]
    pub fn disable_referential_integrity(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_referential_integrity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disable_resource_versioning` after provisioning.\nWhether to disable resource versioning for this FHIR store. This field can not be changed after the creation\nof FHIR store. If set to false, which is the default behavior, all write operations will cause historical\nversions to be recorded automatically. The historical versions can be fetched through the history APIs, but\ncannot be updated. If set to true, no historical versions will be kept. The server will send back errors for\nattempts to read the historical versions.\n\n** Changing this property may recreate the FHIR store (removing all data) **"]
    pub fn disable_resource_versioning(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_resource_versioning", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_history_import` after provisioning.\nWhether to allow the bulk import API to accept history bundles and directly insert historical resource\nversions into the FHIR store. Importing resource histories creates resource interactions that appear to have\noccurred in the past, which clients may not want to allow. If set to false, history bundles within an import\nwill fail with an error.\n\n** Changing this property may recreate the FHIR store (removing all data) **\n\n** This property can be changed manually in the Google Cloud Healthcare admin console without recreating the FHIR store **"]
    pub fn enable_history_import(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_history_import", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_update_create` after provisioning.\nWhether this FHIR store has the updateCreate capability. This determines if the client can use an Update\noperation to create a new resource with a client-specified ID. If false, all IDs are server-assigned through\nthe Create operation and attempts to Update a non-existent resource will return errors. Please treat the audit\nlogs with appropriate levels of care if client-specified resource IDs contain sensitive data such as patient\nidentifiers, those IDs will be part of the FHIR resource path recorded in Cloud audit logs and Cloud Pub/Sub\nnotifications."]
    pub fn enable_update_create(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_update_create", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-supplied key-value pairs used to organize FHIR stores.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must\nconform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\n\nLabel values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128\nbytes, and must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be associated with a given store.\n\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for the FhirStore.\n\n** Changing this property may recreate the FHIR store (removing all data) **"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe fully qualified name of this dataset"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe FHIR specification version. Possible values: [\"DSTU2\", \"STU3\", \"R4\"]"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(&self) -> ListRef<HealthcareFhirStoreNotificationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notification_configs` after provisioning.\n"]
    pub fn notification_configs(&self) -> ListRef<HealthcareFhirStoreNotificationConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.notification_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stream_configs` after provisioning.\n"]
    pub fn stream_configs(&self) -> ListRef<HealthcareFhirStoreStreamConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stream_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> HealthcareFhirStoreTimeoutsElRef {
        HealthcareFhirStoreTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `validation_config` after provisioning.\n"]
    pub fn validation_config(&self) -> ListRef<HealthcareFhirStoreValidationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.validation_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct HealthcareFhirStoreNotificationConfigEl {
    pubsub_topic: PrimField<String>,
}
impl HealthcareFhirStoreNotificationConfigEl {}
impl ToListMappable for HealthcareFhirStoreNotificationConfigEl {
    type O = BlockAssignable<HealthcareFhirStoreNotificationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHealthcareFhirStoreNotificationConfigEl {
    #[doc = "The Cloud Pub/Sub topic that notifications of changes are published on. Supplied by the client.\nPubsubMessage.Data will contain the resource name. PubsubMessage.MessageId is the ID of this message.\nIt is guaranteed to be unique within the topic. PubsubMessage.PublishTime is the time at which the message\nwas published. Notifications are only sent if the topic is non-empty. Topic names must be scoped to a\nproject. service-PROJECT_NUMBER@gcp-sa-healthcare.iam.gserviceaccount.com must have publisher permissions on the given\nCloud Pub/Sub topic. Not having adequate permissions will cause the calls that send notifications to fail."]
    pub pubsub_topic: PrimField<String>,
}
impl BuildHealthcareFhirStoreNotificationConfigEl {
    pub fn build(self) -> HealthcareFhirStoreNotificationConfigEl {
        HealthcareFhirStoreNotificationConfigEl {
            pubsub_topic: self.pubsub_topic,
        }
    }
}
pub struct HealthcareFhirStoreNotificationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HealthcareFhirStoreNotificationConfigElRef {
    fn new(shared: StackShared, base: String) -> HealthcareFhirStoreNotificationConfigElRef {
        HealthcareFhirStoreNotificationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HealthcareFhirStoreNotificationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pubsub_topic` after provisioning.\nThe Cloud Pub/Sub topic that notifications of changes are published on. Supplied by the client.\nPubsubMessage.Data will contain the resource name. PubsubMessage.MessageId is the ID of this message.\nIt is guaranteed to be unique within the topic. PubsubMessage.PublishTime is the time at which the message\nwas published. Notifications are only sent if the topic is non-empty. Topic names must be scoped to a\nproject. service-PROJECT_NUMBER@gcp-sa-healthcare.iam.gserviceaccount.com must have publisher permissions on the given\nCloud Pub/Sub topic. Not having adequate permissions will cause the calls that send notifications to fail."]
    pub fn pubsub_topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pubsub_topic", self.base))
    }
}
#[derive(Serialize)]
pub struct HealthcareFhirStoreNotificationConfigsEl {
    pubsub_topic: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_full_resource: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_previous_resource_on_delete: Option<PrimField<bool>>,
}
impl HealthcareFhirStoreNotificationConfigsEl {
    #[doc = "Set the field `send_full_resource`.\nWhether to send full FHIR resource to this Pub/Sub topic for Create and Update operation.\nNote that setting this to true does not guarantee that all resources will be sent in the format of\nfull FHIR resource. When a resource change is too large or during heavy traffic, only the resource name will be\nsent. Clients should always check the \"payloadType\" label from a Pub/Sub message to determine whether\nit needs to fetch the full resource as a separate operation."]
    pub fn set_send_full_resource(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.send_full_resource = Some(v.into());
        self
    }
    #[doc = "Set the field `send_previous_resource_on_delete`.\nWhether to send full FHIR resource to this Pub/Sub topic for deleting FHIR resource. Note that setting this to\ntrue does not guarantee that all previous resources will be sent in the format of full FHIR resource. When a\nresource change is too large or during heavy traffic, only the resource name will be sent. Clients should always\ncheck the \"payloadType\" label from a Pub/Sub message to determine whether it needs to fetch the full previous\nresource as a separate operation."]
    pub fn set_send_previous_resource_on_delete(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.send_previous_resource_on_delete = Some(v.into());
        self
    }
}
impl ToListMappable for HealthcareFhirStoreNotificationConfigsEl {
    type O = BlockAssignable<HealthcareFhirStoreNotificationConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHealthcareFhirStoreNotificationConfigsEl {
    #[doc = "The Cloud Pub/Sub topic that notifications of changes are published on. Supplied by the client.\nPubsubMessage.Data will contain the resource name. PubsubMessage.MessageId is the ID of this message.\nIt is guaranteed to be unique within the topic. PubsubMessage.PublishTime is the time at which the message\nwas published. Notifications are only sent if the topic is non-empty. Topic names must be scoped to a\nproject. service-PROJECT_NUMBER@gcp-sa-healthcare.iam.gserviceaccount.com must have publisher permissions on the given\nCloud Pub/Sub topic. Not having adequate permissions will cause the calls that send notifications to fail."]
    pub pubsub_topic: PrimField<String>,
}
impl BuildHealthcareFhirStoreNotificationConfigsEl {
    pub fn build(self) -> HealthcareFhirStoreNotificationConfigsEl {
        HealthcareFhirStoreNotificationConfigsEl {
            pubsub_topic: self.pubsub_topic,
            send_full_resource: core::default::Default::default(),
            send_previous_resource_on_delete: core::default::Default::default(),
        }
    }
}
pub struct HealthcareFhirStoreNotificationConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HealthcareFhirStoreNotificationConfigsElRef {
    fn new(shared: StackShared, base: String) -> HealthcareFhirStoreNotificationConfigsElRef {
        HealthcareFhirStoreNotificationConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HealthcareFhirStoreNotificationConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pubsub_topic` after provisioning.\nThe Cloud Pub/Sub topic that notifications of changes are published on. Supplied by the client.\nPubsubMessage.Data will contain the resource name. PubsubMessage.MessageId is the ID of this message.\nIt is guaranteed to be unique within the topic. PubsubMessage.PublishTime is the time at which the message\nwas published. Notifications are only sent if the topic is non-empty. Topic names must be scoped to a\nproject. service-PROJECT_NUMBER@gcp-sa-healthcare.iam.gserviceaccount.com must have publisher permissions on the given\nCloud Pub/Sub topic. Not having adequate permissions will cause the calls that send notifications to fail."]
    pub fn pubsub_topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pubsub_topic", self.base))
    }
    #[doc = "Get a reference to the value of field `send_full_resource` after provisioning.\nWhether to send full FHIR resource to this Pub/Sub topic for Create and Update operation.\nNote that setting this to true does not guarantee that all resources will be sent in the format of\nfull FHIR resource. When a resource change is too large or during heavy traffic, only the resource name will be\nsent. Clients should always check the \"payloadType\" label from a Pub/Sub message to determine whether\nit needs to fetch the full resource as a separate operation."]
    pub fn send_full_resource(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.send_full_resource", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `send_previous_resource_on_delete` after provisioning.\nWhether to send full FHIR resource to this Pub/Sub topic for deleting FHIR resource. Note that setting this to\ntrue does not guarantee that all previous resources will be sent in the format of full FHIR resource. When a\nresource change is too large or during heavy traffic, only the resource name will be sent. Clients should always\ncheck the \"payloadType\" label from a Pub/Sub message to determine whether it needs to fetch the full previous\nresource as a separate operation."]
    pub fn send_previous_resource_on_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.send_previous_resource_on_delete", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigElLastUpdatedPartitionConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration_ms: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigElLastUpdatedPartitionConfigEl { # [doc = "Set the field `expiration_ms`.\nNumber of milliseconds for which to keep the storage for a partition."] pub fn set_expiration_ms (mut self , v : impl Into < PrimField < String > >) -> Self { self . expiration_ms = Some (v . into ()) ; self } }
impl ToListMappable for HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigElLastUpdatedPartitionConfigEl { type O = BlockAssignable < HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigElLastUpdatedPartitionConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildHealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigElLastUpdatedPartitionConfigEl
{
    #[doc = "Type of partitioning. Possible values: [\"PARTITION_TYPE_UNSPECIFIED\", \"HOUR\", \"DAY\", \"MONTH\", \"YEAR\"]"]
    pub type_: PrimField<String>,
}
impl BuildHealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigElLastUpdatedPartitionConfigEl { pub fn build (self) -> HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigElLastUpdatedPartitionConfigEl { HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigElLastUpdatedPartitionConfigEl { expiration_ms : core :: default :: Default :: default () , type_ : self . type_ , } } }
pub struct HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigElLastUpdatedPartitionConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigElLastUpdatedPartitionConfigElRef { fn new (shared : StackShared , base : String) -> HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigElLastUpdatedPartitionConfigElRef { HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigElLastUpdatedPartitionConfigElRef { shared : shared , base : base . to_string () , } } }
impl HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigElLastUpdatedPartitionConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `expiration_ms` after provisioning.\nNumber of milliseconds for which to keep the storage for a partition."] pub fn expiration_ms (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.expiration_ms" , self . base)) } # [doc = "Get a reference to the value of field `type_` after provisioning.\nType of partitioning. Possible values: [\"PARTITION_TYPE_UNSPECIFIED\", \"HOUR\", \"DAY\", \"MONTH\", \"YEAR\"]"] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } }
#[derive(Serialize, Default)]
struct HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigElDynamic { last_updated_partition_config : Option < DynamicBlock < HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigElLastUpdatedPartitionConfigEl >> , }
#[derive(Serialize)]
pub struct HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigEl { recursive_structure_depth : PrimField < f64 > , # [serde (skip_serializing_if = "Option::is_none")] schema_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] last_updated_partition_config : Option < Vec < HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigElLastUpdatedPartitionConfigEl > > , dynamic : HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigElDynamic , }
impl HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigEl {
    #[doc = "Set the field `schema_type`.\nSpecifies the output schema type.\n * ANALYTICS: Analytics schema defined by the FHIR community.\n  See https://github.com/FHIR/sql-on-fhir/blob/master/sql-on-fhir.md.\n * ANALYTICS_V2: Analytics V2, similar to schema defined by the FHIR community, with added support for extensions with one or more occurrences and contained resources in stringified JSON.\n * LOSSLESS: A data-driven schema generated from the fields present in the FHIR data being exported, with no additional simplification. Default value: \"ANALYTICS\" Possible values: [\"ANALYTICS\", \"ANALYTICS_V2\", \"LOSSLESS\"]"]
    pub fn set_schema_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schema_type = Some(v.into());
        self
    }
    #[doc = "Set the field `last_updated_partition_config`.\n"]
    pub fn set_last_updated_partition_config(
        mut self,
        v : impl Into < BlockAssignable < HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigElLastUpdatedPartitionConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.last_updated_partition_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.last_updated_partition_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigEl {
    type O = BlockAssignable<HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigEl {
    #[doc = "The depth for all recursive structures in the output analytics schema. For example, concept in the CodeSystem\nresource is a recursive structure; when the depth is 2, the CodeSystem table will have a column called\nconcept.concept but not concept.concept.concept. If not specified or set to 0, the server will use the default\nvalue 2. The maximum depth allowed is 5."]
    pub recursive_structure_depth: PrimField<f64>,
}
impl BuildHealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigEl {
    pub fn build(self) -> HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigEl {
        HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigEl {
            recursive_structure_depth: self.recursive_structure_depth,
            schema_type: core::default::Default::default(),
            last_updated_partition_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigElRef {
        HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `recursive_structure_depth` after provisioning.\nThe depth for all recursive structures in the output analytics schema. For example, concept in the CodeSystem\nresource is a recursive structure; when the depth is 2, the CodeSystem table will have a column called\nconcept.concept but not concept.concept.concept. If not specified or set to 0, the server will use the default\nvalue 2. The maximum depth allowed is 5."]
    pub fn recursive_structure_depth(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.recursive_structure_depth", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `schema_type` after provisioning.\nSpecifies the output schema type.\n * ANALYTICS: Analytics schema defined by the FHIR community.\n  See https://github.com/FHIR/sql-on-fhir/blob/master/sql-on-fhir.md.\n * ANALYTICS_V2: Analytics V2, similar to schema defined by the FHIR community, with added support for extensions with one or more occurrences and contained resources in stringified JSON.\n * LOSSLESS: A data-driven schema generated from the fields present in the FHIR data being exported, with no additional simplification. Default value: \"ANALYTICS\" Possible values: [\"ANALYTICS\", \"ANALYTICS_V2\", \"LOSSLESS\"]"]
    pub fn schema_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema_type", self.base))
    }
    #[doc = "Get a reference to the value of field `last_updated_partition_config` after provisioning.\n"]    pub fn last_updated_partition_config (& self) -> ListRef < HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigElLastUpdatedPartitionConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.last_updated_partition_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct HealthcareFhirStoreStreamConfigsElBigqueryDestinationElDynamic {
    schema_config:
        Option<DynamicBlock<HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigEl>>,
}
#[derive(Serialize)]
pub struct HealthcareFhirStoreStreamConfigsElBigqueryDestinationEl {
    dataset_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_config:
        Option<Vec<HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigEl>>,
    dynamic: HealthcareFhirStoreStreamConfigsElBigqueryDestinationElDynamic,
}
impl HealthcareFhirStoreStreamConfigsElBigqueryDestinationEl {
    #[doc = "Set the field `schema_config`.\n"]
    pub fn set_schema_config(
        mut self,
        v: impl Into<
            BlockAssignable<HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schema_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schema_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for HealthcareFhirStoreStreamConfigsElBigqueryDestinationEl {
    type O = BlockAssignable<HealthcareFhirStoreStreamConfigsElBigqueryDestinationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHealthcareFhirStoreStreamConfigsElBigqueryDestinationEl {
    #[doc = "BigQuery URI to a dataset, up to 2000 characters long, in the format bq://projectId.bqDatasetId"]
    pub dataset_uri: PrimField<String>,
}
impl BuildHealthcareFhirStoreStreamConfigsElBigqueryDestinationEl {
    pub fn build(self) -> HealthcareFhirStoreStreamConfigsElBigqueryDestinationEl {
        HealthcareFhirStoreStreamConfigsElBigqueryDestinationEl {
            dataset_uri: self.dataset_uri,
            schema_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct HealthcareFhirStoreStreamConfigsElBigqueryDestinationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HealthcareFhirStoreStreamConfigsElBigqueryDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HealthcareFhirStoreStreamConfigsElBigqueryDestinationElRef {
        HealthcareFhirStoreStreamConfigsElBigqueryDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HealthcareFhirStoreStreamConfigsElBigqueryDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset_uri` after provisioning.\nBigQuery URI to a dataset, up to 2000 characters long, in the format bq://projectId.bqDatasetId"]
    pub fn dataset_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `schema_config` after provisioning.\n"]
    pub fn schema_config(
        &self,
    ) -> ListRef<HealthcareFhirStoreStreamConfigsElBigqueryDestinationElSchemaConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct HealthcareFhirStoreStreamConfigsElDynamic {
    bigquery_destination:
        Option<DynamicBlock<HealthcareFhirStoreStreamConfigsElBigqueryDestinationEl>>,
}
#[derive(Serialize)]
pub struct HealthcareFhirStoreStreamConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_types: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bigquery_destination: Option<Vec<HealthcareFhirStoreStreamConfigsElBigqueryDestinationEl>>,
    dynamic: HealthcareFhirStoreStreamConfigsElDynamic,
}
impl HealthcareFhirStoreStreamConfigsEl {
    #[doc = "Set the field `resource_types`.\nSupply a FHIR resource type (such as \"Patient\" or \"Observation\"). See\nhttps://www.hl7.org/fhir/valueset-resource-types.html for a list of all FHIR resource types. The server treats\nan empty list as an intent to stream all the supported resource types in this FHIR store."]
    pub fn set_resource_types(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.resource_types = Some(v.into());
        self
    }
    #[doc = "Set the field `bigquery_destination`.\n"]
    pub fn set_bigquery_destination(
        mut self,
        v: impl Into<BlockAssignable<HealthcareFhirStoreStreamConfigsElBigqueryDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.bigquery_destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.bigquery_destination = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for HealthcareFhirStoreStreamConfigsEl {
    type O = BlockAssignable<HealthcareFhirStoreStreamConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHealthcareFhirStoreStreamConfigsEl {}
impl BuildHealthcareFhirStoreStreamConfigsEl {
    pub fn build(self) -> HealthcareFhirStoreStreamConfigsEl {
        HealthcareFhirStoreStreamConfigsEl {
            resource_types: core::default::Default::default(),
            bigquery_destination: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct HealthcareFhirStoreStreamConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HealthcareFhirStoreStreamConfigsElRef {
    fn new(shared: StackShared, base: String) -> HealthcareFhirStoreStreamConfigsElRef {
        HealthcareFhirStoreStreamConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HealthcareFhirStoreStreamConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_types` after provisioning.\nSupply a FHIR resource type (such as \"Patient\" or \"Observation\"). See\nhttps://www.hl7.org/fhir/valueset-resource-types.html for a list of all FHIR resource types. The server treats\nan empty list as an intent to stream all the supported resource types in this FHIR store."]
    pub fn resource_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_types", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bigquery_destination` after provisioning.\n"]
    pub fn bigquery_destination(
        &self,
    ) -> ListRef<HealthcareFhirStoreStreamConfigsElBigqueryDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigquery_destination", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct HealthcareFhirStoreTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl HealthcareFhirStoreTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for HealthcareFhirStoreTimeoutsEl {
    type O = BlockAssignable<HealthcareFhirStoreTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHealthcareFhirStoreTimeoutsEl {}
impl BuildHealthcareFhirStoreTimeoutsEl {
    pub fn build(self) -> HealthcareFhirStoreTimeoutsEl {
        HealthcareFhirStoreTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct HealthcareFhirStoreTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HealthcareFhirStoreTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> HealthcareFhirStoreTimeoutsElRef {
        HealthcareFhirStoreTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HealthcareFhirStoreTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize)]
pub struct HealthcareFhirStoreValidationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_fhirpath_validation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_profile_validation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_reference_type_validation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_required_field_validation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled_implementation_guides: Option<ListField<PrimField<String>>>,
}
impl HealthcareFhirStoreValidationConfigEl {
    #[doc = "Set the field `disable_fhirpath_validation`.\nWhether to disable FHIRPath validation for incoming resources. The default value is false. Set this to true to disable checking incoming resources for conformance against FHIRPath requirement defined in the FHIR specification. This property only affects resource types that do not have profiles configured for them, any rules in enabled implementation guides will still be enforced."]
    pub fn set_disable_fhirpath_validation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_fhirpath_validation = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_profile_validation`.\nWhether to disable profile validation for this FHIR store. The default value is false. Set this to true to disable checking incoming resources for conformance against structure definitions in this FHIR store."]
    pub fn set_disable_profile_validation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_profile_validation = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_reference_type_validation`.\nWhether to disable reference type validation for incoming resources. The default value is false. Set this to true to disable checking incoming resources for conformance against reference type requirement defined in the FHIR specification. This property only affects resource types that do not have profiles configured for them, any rules in enabled implementation guides will still be enforced."]
    pub fn set_disable_reference_type_validation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_reference_type_validation = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_required_field_validation`.\nWhether to disable required fields validation for incoming resources. The default value is false. Set this to true to disable checking incoming resources for conformance against required fields requirement defined in the FHIR specification. This property only affects resource types that do not have profiles configured for them, any rules in enabled implementation guides will still be enforced."]
    pub fn set_disable_required_field_validation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_required_field_validation = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled_implementation_guides`.\nA list of implementation guide URLs in this FHIR store that are used to configure the profiles to use for validation.\nWhen a URL cannot be resolved (for example, in a type assertion), the server does not return an error.\nFor example, to use the US Core profiles for validation, set enabledImplementationGuides to [\"http://hl7.org/fhir/us/core/ImplementationGuide/ig\"]. If enabledImplementationGuides is empty or omitted, then incoming resources are only required to conform to the base FHIR profiles. Otherwise, a resource must conform to at least one profile listed in the global property of one of the enabled ImplementationGuides.\nThe Cloud Healthcare API does not currently enforce all of the rules in a StructureDefinition. The following rules are supported:\n- min/max\n- minValue/maxValue\n- maxLength\n- type\n- fixed[x]\n- pattern[x] on simple types\n- slicing, when using \"value\" as the discriminator type"]
    pub fn set_enabled_implementation_guides(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.enabled_implementation_guides = Some(v.into());
        self
    }
}
impl ToListMappable for HealthcareFhirStoreValidationConfigEl {
    type O = BlockAssignable<HealthcareFhirStoreValidationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHealthcareFhirStoreValidationConfigEl {}
impl BuildHealthcareFhirStoreValidationConfigEl {
    pub fn build(self) -> HealthcareFhirStoreValidationConfigEl {
        HealthcareFhirStoreValidationConfigEl {
            disable_fhirpath_validation: core::default::Default::default(),
            disable_profile_validation: core::default::Default::default(),
            disable_reference_type_validation: core::default::Default::default(),
            disable_required_field_validation: core::default::Default::default(),
            enabled_implementation_guides: core::default::Default::default(),
        }
    }
}
pub struct HealthcareFhirStoreValidationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HealthcareFhirStoreValidationConfigElRef {
    fn new(shared: StackShared, base: String) -> HealthcareFhirStoreValidationConfigElRef {
        HealthcareFhirStoreValidationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HealthcareFhirStoreValidationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_fhirpath_validation` after provisioning.\nWhether to disable FHIRPath validation for incoming resources. The default value is false. Set this to true to disable checking incoming resources for conformance against FHIRPath requirement defined in the FHIR specification. This property only affects resource types that do not have profiles configured for them, any rules in enabled implementation guides will still be enforced."]
    pub fn disable_fhirpath_validation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_fhirpath_validation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disable_profile_validation` after provisioning.\nWhether to disable profile validation for this FHIR store. The default value is false. Set this to true to disable checking incoming resources for conformance against structure definitions in this FHIR store."]
    pub fn disable_profile_validation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_profile_validation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disable_reference_type_validation` after provisioning.\nWhether to disable reference type validation for incoming resources. The default value is false. Set this to true to disable checking incoming resources for conformance against reference type requirement defined in the FHIR specification. This property only affects resource types that do not have profiles configured for them, any rules in enabled implementation guides will still be enforced."]
    pub fn disable_reference_type_validation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_reference_type_validation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disable_required_field_validation` after provisioning.\nWhether to disable required fields validation for incoming resources. The default value is false. Set this to true to disable checking incoming resources for conformance against required fields requirement defined in the FHIR specification. This property only affects resource types that do not have profiles configured for them, any rules in enabled implementation guides will still be enforced."]
    pub fn disable_required_field_validation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_required_field_validation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enabled_implementation_guides` after provisioning.\nA list of implementation guide URLs in this FHIR store that are used to configure the profiles to use for validation.\nWhen a URL cannot be resolved (for example, in a type assertion), the server does not return an error.\nFor example, to use the US Core profiles for validation, set enabledImplementationGuides to [\"http://hl7.org/fhir/us/core/ImplementationGuide/ig\"]. If enabledImplementationGuides is empty or omitted, then incoming resources are only required to conform to the base FHIR profiles. Otherwise, a resource must conform to at least one profile listed in the global property of one of the enabled ImplementationGuides.\nThe Cloud Healthcare API does not currently enforce all of the rules in a StructureDefinition. The following rules are supported:\n- min/max\n- minValue/maxValue\n- maxLength\n- type\n- fixed[x]\n- pattern[x] on simple types\n- slicing, when using \"value\" as the discriminator type"]
    pub fn enabled_implementation_guides(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enabled_implementation_guides", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct HealthcareFhirStoreDynamic {
    notification_config: Option<DynamicBlock<HealthcareFhirStoreNotificationConfigEl>>,
    notification_configs: Option<DynamicBlock<HealthcareFhirStoreNotificationConfigsEl>>,
    stream_configs: Option<DynamicBlock<HealthcareFhirStoreStreamConfigsEl>>,
    validation_config: Option<DynamicBlock<HealthcareFhirStoreValidationConfigEl>>,
}
