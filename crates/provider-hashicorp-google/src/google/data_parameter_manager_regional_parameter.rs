use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataParameterManagerRegionalParameterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    parameter_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataParameterManagerRegionalParameter_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataParameterManagerRegionalParameterData>,
}
#[derive(Clone)]
pub struct DataParameterManagerRegionalParameter(Rc<DataParameterManagerRegionalParameter_>);
impl DataParameterManagerRegionalParameter {
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
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time at which the regional Parameter was created."]
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
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `format` after provisioning.\nThe format type of the regional parameter. Default value: \"UNFORMATTED\" Possible values: [\"UNFORMATTED\", \"YAML\", \"JSON\"]"]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.format", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nThe resource name of the Cloud KMS CryptoKey used to encrypt regional parameter version payload. Format\n'projects/{{project}}/locations/{{location}}/keyRings/{{key_ring}}/cryptoKeys/{{crypto_key}}'"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels assigned to this regional Parameter.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,\nand must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\n\nLabel values must be between 0 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,\nand must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be assigned to a given resource.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the regional parameter. eg us-central1"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the regional Parameter. Format:\n'projects/{{project}}/locations/{{location}}/parameters/{{parameter_id}}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parameter_id` after provisioning.\nThis must be unique within the project."]
    pub fn parameter_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parameter_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_member` after provisioning.\nAn object containing a unique resource identity tied to the regional parameter."]
    pub fn policy_member(&self) -> ListRef<DataParameterManagerRegionalParameterPolicyMemberElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.policy_member", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time at which the regional Parameter was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
}
impl Referable for DataParameterManagerRegionalParameter {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataParameterManagerRegionalParameter {}
impl ToListMappable for DataParameterManagerRegionalParameter {
    type O = ListRef<DataParameterManagerRegionalParameterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataParameterManagerRegionalParameter_ {
    fn extract_datasource_type(&self) -> String {
        "google_parameter_manager_regional_parameter".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataParameterManagerRegionalParameter {
    pub tf_id: String,
    #[doc = "The location of the regional parameter. eg us-central1"]
    pub location: PrimField<String>,
    #[doc = "This must be unique within the project."]
    pub parameter_id: PrimField<String>,
}
impl BuildDataParameterManagerRegionalParameter {
    pub fn build(self, stack: &mut Stack) -> DataParameterManagerRegionalParameter {
        let out = DataParameterManagerRegionalParameter(Rc::new(
            DataParameterManagerRegionalParameter_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataParameterManagerRegionalParameterData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    id: core::default::Default::default(),
                    location: self.location,
                    parameter_id: self.parameter_id,
                    project: core::default::Default::default(),
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataParameterManagerRegionalParameterRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataParameterManagerRegionalParameterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataParameterManagerRegionalParameterRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time at which the regional Parameter was created."]
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
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `format` after provisioning.\nThe format type of the regional parameter. Default value: \"UNFORMATTED\" Possible values: [\"UNFORMATTED\", \"YAML\", \"JSON\"]"]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.format", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nThe resource name of the Cloud KMS CryptoKey used to encrypt regional parameter version payload. Format\n'projects/{{project}}/locations/{{location}}/keyRings/{{key_ring}}/cryptoKeys/{{crypto_key}}'"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels assigned to this regional Parameter.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,\nand must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\n\nLabel values must be between 0 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,\nand must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be assigned to a given resource.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the regional parameter. eg us-central1"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the regional Parameter. Format:\n'projects/{{project}}/locations/{{location}}/parameters/{{parameter_id}}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parameter_id` after provisioning.\nThis must be unique within the project."]
    pub fn parameter_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parameter_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_member` after provisioning.\nAn object containing a unique resource identity tied to the regional parameter."]
    pub fn policy_member(&self) -> ListRef<DataParameterManagerRegionalParameterPolicyMemberElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.policy_member", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time at which the regional Parameter was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataParameterManagerRegionalParameterPolicyMemberEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_policy_name_principal: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_policy_uid_principal: Option<PrimField<String>>,
}
impl DataParameterManagerRegionalParameterPolicyMemberEl {
    #[doc = "Set the field `iam_policy_name_principal`.\n"]
    pub fn set_iam_policy_name_principal(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.iam_policy_name_principal = Some(v.into());
        self
    }
    #[doc = "Set the field `iam_policy_uid_principal`.\n"]
    pub fn set_iam_policy_uid_principal(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.iam_policy_uid_principal = Some(v.into());
        self
    }
}
impl ToListMappable for DataParameterManagerRegionalParameterPolicyMemberEl {
    type O = BlockAssignable<DataParameterManagerRegionalParameterPolicyMemberEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataParameterManagerRegionalParameterPolicyMemberEl {}
impl BuildDataParameterManagerRegionalParameterPolicyMemberEl {
    pub fn build(self) -> DataParameterManagerRegionalParameterPolicyMemberEl {
        DataParameterManagerRegionalParameterPolicyMemberEl {
            iam_policy_name_principal: core::default::Default::default(),
            iam_policy_uid_principal: core::default::Default::default(),
        }
    }
}
pub struct DataParameterManagerRegionalParameterPolicyMemberElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataParameterManagerRegionalParameterPolicyMemberElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataParameterManagerRegionalParameterPolicyMemberElRef {
        DataParameterManagerRegionalParameterPolicyMemberElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataParameterManagerRegionalParameterPolicyMemberElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `iam_policy_name_principal` after provisioning.\n"]
    pub fn iam_policy_name_principal(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_policy_name_principal", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `iam_policy_uid_principal` after provisioning.\n"]
    pub fn iam_policy_uid_principal(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_policy_uid_principal", self.base),
        )
    }
}
