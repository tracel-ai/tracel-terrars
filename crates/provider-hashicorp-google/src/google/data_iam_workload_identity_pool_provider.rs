use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataIamWorkloadIdentityPoolProviderData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    workload_identity_pool_id: PrimField<String>,
    workload_identity_pool_provider_id: PrimField<String>,
}
struct DataIamWorkloadIdentityPoolProvider_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataIamWorkloadIdentityPoolProviderData>,
}
#[derive(Clone)]
pub struct DataIamWorkloadIdentityPoolProvider(Rc<DataIamWorkloadIdentityPoolProvider_>);
impl DataIamWorkloadIdentityPoolProvider {
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
    #[doc = "Get a reference to the value of field `attribute_condition` after provisioning.\n[A Common Expression Language](https://github.com/google/cel-spec) expression, in\nplain text, to restrict what otherwise valid authentication credentials issued by the\nprovider should not be accepted.\n\nThe expression must output a boolean representing whether to allow the federation.\n\nThe following keywords may be referenced in the expressions:\n  * 'assertion': JSON representing the authentication credential issued by the provider.\n  * 'google': The Google attributes mapped from the assertion in the 'attribute_mappings'.\n  * 'attribute': The custom attributes mapped from the assertion in the 'attribute_mappings'.\n\nThe maximum length of the attribute condition expression is 4096 characters. If\nunspecified, all valid authentication credential are accepted.\n\nThe following example shows how to only allow credentials with a mapped 'google.groups'\nvalue of 'admins':\n'''\n\"'admins' in google.groups\"\n'''"]
    pub fn attribute_condition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.attribute_condition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `attribute_mapping` after provisioning.\nMaps attributes from authentication credentials issued by an external identity provider\nto Google Cloud attributes, such as 'subject' and 'segment'.\n\nEach key must be a string specifying the Google Cloud IAM attribute to map to.\n\nThe following keys are supported:\n  * 'google.subject': The principal IAM is authenticating. You can reference this value\n    in IAM bindings. This is also the subject that appears in Cloud Logging logs.\n    Cannot exceed 127 characters.\n  * 'google.groups': Groups the external identity belongs to. You can grant groups\n    access to resources using an IAM 'principalSet' binding; access applies to all\n    members of the group.\n\nYou can also provide custom attributes by specifying 'attribute.{custom_attribute}',\nwhere '{custom_attribute}' is the name of the custom attribute to be mapped. You can\ndefine a maximum of 50 custom attributes. The maximum length of a mapped attribute key\nis 100 characters, and the key may only contain the characters [a-z0-9_].\n\nYou can reference these attributes in IAM policies to define fine-grained access for a\nworkload to Google Cloud resources. For example:\n  * 'google.subject':\n    'principal://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/subject/{value}'\n  * 'google.groups':\n    'principalSet://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/group/{value}'\n  * 'attribute.{custom_attribute}':\n    'principalSet://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/attribute.{custom_attribute}/{value}'\n\nEach value must be a [Common Expression Language](https://github.com/google/cel-spec)\nfunction that maps an identity provider credential to the normalized attribute specified\nby the corresponding map key.\n\nYou can use the 'assertion' keyword in the expression to access a JSON representation of\nthe authentication credential issued by the provider.\n\nThe maximum length of an attribute mapping expression is 2048 characters. When evaluated,\nthe total size of all mapped attributes must not exceed 8KB.\n\nFor AWS providers, the following rules apply:\n  - If no attribute mapping is defined, the following default mapping applies:\n    '''\n    {\n      \"google.subject\":\"assertion.arn\",\n      \"attribute.aws_role\":\n        \"assertion.arn.contains('assumed-role')\"\n        \" ? assertion.arn.extract('{account_arn}assumed-role/')\"\n        \"   + 'assumed-role/'\"\n        \"   + assertion.arn.extract('assumed-role/{role_name}/')\"\n        \" : assertion.arn\",\n    }\n    '''\n  - If any custom attribute mappings are defined, they must include a mapping to the\n    'google.subject' attribute.\n\nFor OIDC providers, the following rules apply:\n  - Custom attribute mappings must be defined, and must include a mapping to the\n    'google.subject' attribute. For example, the following maps the 'sub' claim of the\n    incoming credential to the 'subject' attribute on a Google token.\n    '''\n    {\"google.subject\": \"assertion.sub\"}\n    '''"]
    pub fn attribute_mapping(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.attribute_mapping", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `aws` after provisioning.\nAn Amazon Web Services identity provider. Not compatible with the property oidc or saml."]
    pub fn aws(&self) -> ListRef<DataIamWorkloadIdentityPoolProviderAwsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aws", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description for the provider. Cannot exceed 256 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the provider is disabled. You cannot use a disabled provider to exchange tokens.\nHowever, existing tokens still grant access."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA display name for the provider. Cannot exceed 32 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the provider as\n'projects/{project_number}/locations/global/workloadIdentityPools/{workload_identity_pool_id}/providers/{workload_identity_pool_provider_id}'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oidc` after provisioning.\nAn OpenId Connect 1.0 identity provider. Not compatible with the property aws or saml."]
    pub fn oidc(&self) -> ListRef<DataIamWorkloadIdentityPoolProviderOidcElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oidc", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `saml` after provisioning.\nAn SAML 2.0 identity provider. Not compatible with the property oidc or aws."]
    pub fn saml(&self) -> ListRef<DataIamWorkloadIdentityPoolProviderSamlElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.saml", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the provider.\n* STATE_UNSPECIFIED: State unspecified.\n* ACTIVE: The provider is active, and may be used to validate authentication credentials.\n* DELETED: The provider is soft-deleted. Soft-deleted providers are permanently deleted\n  after approximately 30 days. You can restore a soft-deleted provider using\n  UndeleteWorkloadIdentityPoolProvider. You cannot reuse the ID of a soft-deleted provider\n  until it is permanently deleted."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_identity_pool_id` after provisioning.\nThe ID used for the pool, which is the final component of the pool resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workload_identity_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_identity_pool_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_identity_pool_provider_id` after provisioning.\nThe ID for the provider, which becomes the final component of the resource name. This\nvalue must be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workload_identity_pool_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_identity_pool_provider_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `x509` after provisioning.\nAn X.509-type identity provider represents a CA. It is trusted to assert a\nclient identity if the client has a certificate that chains up to this CA."]
    pub fn x509(&self) -> ListRef<DataIamWorkloadIdentityPoolProviderX509ElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.x509", self.extract_ref()),
        )
    }
}
impl Referable for DataIamWorkloadIdentityPoolProvider {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataIamWorkloadIdentityPoolProvider {}
impl ToListMappable for DataIamWorkloadIdentityPoolProvider {
    type O = ListRef<DataIamWorkloadIdentityPoolProviderRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataIamWorkloadIdentityPoolProvider_ {
    fn extract_datasource_type(&self) -> String {
        "google_iam_workload_identity_pool_provider".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataIamWorkloadIdentityPoolProvider {
    pub tf_id: String,
    #[doc = "The ID used for the pool, which is the final component of the pool resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub workload_identity_pool_id: PrimField<String>,
    #[doc = "The ID for the provider, which becomes the final component of the resource name. This\nvalue must be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub workload_identity_pool_provider_id: PrimField<String>,
}
impl BuildDataIamWorkloadIdentityPoolProvider {
    pub fn build(self, stack: &mut Stack) -> DataIamWorkloadIdentityPoolProvider {
        let out =
            DataIamWorkloadIdentityPoolProvider(Rc::new(DataIamWorkloadIdentityPoolProvider_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataIamWorkloadIdentityPoolProviderData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    id: core::default::Default::default(),
                    project: core::default::Default::default(),
                    workload_identity_pool_id: self.workload_identity_pool_id,
                    workload_identity_pool_provider_id: self.workload_identity_pool_provider_id,
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataIamWorkloadIdentityPoolProviderRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamWorkloadIdentityPoolProviderRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataIamWorkloadIdentityPoolProviderRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `attribute_condition` after provisioning.\n[A Common Expression Language](https://github.com/google/cel-spec) expression, in\nplain text, to restrict what otherwise valid authentication credentials issued by the\nprovider should not be accepted.\n\nThe expression must output a boolean representing whether to allow the federation.\n\nThe following keywords may be referenced in the expressions:\n  * 'assertion': JSON representing the authentication credential issued by the provider.\n  * 'google': The Google attributes mapped from the assertion in the 'attribute_mappings'.\n  * 'attribute': The custom attributes mapped from the assertion in the 'attribute_mappings'.\n\nThe maximum length of the attribute condition expression is 4096 characters. If\nunspecified, all valid authentication credential are accepted.\n\nThe following example shows how to only allow credentials with a mapped 'google.groups'\nvalue of 'admins':\n'''\n\"'admins' in google.groups\"\n'''"]
    pub fn attribute_condition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.attribute_condition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `attribute_mapping` after provisioning.\nMaps attributes from authentication credentials issued by an external identity provider\nto Google Cloud attributes, such as 'subject' and 'segment'.\n\nEach key must be a string specifying the Google Cloud IAM attribute to map to.\n\nThe following keys are supported:\n  * 'google.subject': The principal IAM is authenticating. You can reference this value\n    in IAM bindings. This is also the subject that appears in Cloud Logging logs.\n    Cannot exceed 127 characters.\n  * 'google.groups': Groups the external identity belongs to. You can grant groups\n    access to resources using an IAM 'principalSet' binding; access applies to all\n    members of the group.\n\nYou can also provide custom attributes by specifying 'attribute.{custom_attribute}',\nwhere '{custom_attribute}' is the name of the custom attribute to be mapped. You can\ndefine a maximum of 50 custom attributes. The maximum length of a mapped attribute key\nis 100 characters, and the key may only contain the characters [a-z0-9_].\n\nYou can reference these attributes in IAM policies to define fine-grained access for a\nworkload to Google Cloud resources. For example:\n  * 'google.subject':\n    'principal://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/subject/{value}'\n  * 'google.groups':\n    'principalSet://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/group/{value}'\n  * 'attribute.{custom_attribute}':\n    'principalSet://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/attribute.{custom_attribute}/{value}'\n\nEach value must be a [Common Expression Language](https://github.com/google/cel-spec)\nfunction that maps an identity provider credential to the normalized attribute specified\nby the corresponding map key.\n\nYou can use the 'assertion' keyword in the expression to access a JSON representation of\nthe authentication credential issued by the provider.\n\nThe maximum length of an attribute mapping expression is 2048 characters. When evaluated,\nthe total size of all mapped attributes must not exceed 8KB.\n\nFor AWS providers, the following rules apply:\n  - If no attribute mapping is defined, the following default mapping applies:\n    '''\n    {\n      \"google.subject\":\"assertion.arn\",\n      \"attribute.aws_role\":\n        \"assertion.arn.contains('assumed-role')\"\n        \" ? assertion.arn.extract('{account_arn}assumed-role/')\"\n        \"   + 'assumed-role/'\"\n        \"   + assertion.arn.extract('assumed-role/{role_name}/')\"\n        \" : assertion.arn\",\n    }\n    '''\n  - If any custom attribute mappings are defined, they must include a mapping to the\n    'google.subject' attribute.\n\nFor OIDC providers, the following rules apply:\n  - Custom attribute mappings must be defined, and must include a mapping to the\n    'google.subject' attribute. For example, the following maps the 'sub' claim of the\n    incoming credential to the 'subject' attribute on a Google token.\n    '''\n    {\"google.subject\": \"assertion.sub\"}\n    '''"]
    pub fn attribute_mapping(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.attribute_mapping", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `aws` after provisioning.\nAn Amazon Web Services identity provider. Not compatible with the property oidc or saml."]
    pub fn aws(&self) -> ListRef<DataIamWorkloadIdentityPoolProviderAwsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aws", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description for the provider. Cannot exceed 256 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the provider is disabled. You cannot use a disabled provider to exchange tokens.\nHowever, existing tokens still grant access."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA display name for the provider. Cannot exceed 32 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the provider as\n'projects/{project_number}/locations/global/workloadIdentityPools/{workload_identity_pool_id}/providers/{workload_identity_pool_provider_id}'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `oidc` after provisioning.\nAn OpenId Connect 1.0 identity provider. Not compatible with the property aws or saml."]
    pub fn oidc(&self) -> ListRef<DataIamWorkloadIdentityPoolProviderOidcElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oidc", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `saml` after provisioning.\nAn SAML 2.0 identity provider. Not compatible with the property oidc or aws."]
    pub fn saml(&self) -> ListRef<DataIamWorkloadIdentityPoolProviderSamlElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.saml", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the provider.\n* STATE_UNSPECIFIED: State unspecified.\n* ACTIVE: The provider is active, and may be used to validate authentication credentials.\n* DELETED: The provider is soft-deleted. Soft-deleted providers are permanently deleted\n  after approximately 30 days. You can restore a soft-deleted provider using\n  UndeleteWorkloadIdentityPoolProvider. You cannot reuse the ID of a soft-deleted provider\n  until it is permanently deleted."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_identity_pool_id` after provisioning.\nThe ID used for the pool, which is the final component of the pool resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workload_identity_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_identity_pool_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_identity_pool_provider_id` after provisioning.\nThe ID for the provider, which becomes the final component of the resource name. This\nvalue must be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workload_identity_pool_provider_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_identity_pool_provider_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `x509` after provisioning.\nAn X.509-type identity provider represents a CA. It is trusted to assert a\nclient identity if the client has a certificate that chains up to this CA."]
    pub fn x509(&self) -> ListRef<DataIamWorkloadIdentityPoolProviderX509ElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.x509", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPoolProviderAwsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
}
impl DataIamWorkloadIdentityPoolProviderAwsEl {
    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.account_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataIamWorkloadIdentityPoolProviderAwsEl {
    type O = BlockAssignable<DataIamWorkloadIdentityPoolProviderAwsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIamWorkloadIdentityPoolProviderAwsEl {}
impl BuildDataIamWorkloadIdentityPoolProviderAwsEl {
    pub fn build(self) -> DataIamWorkloadIdentityPoolProviderAwsEl {
        DataIamWorkloadIdentityPoolProviderAwsEl {
            account_id: core::default::Default::default(),
        }
    }
}
pub struct DataIamWorkloadIdentityPoolProviderAwsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamWorkloadIdentityPoolProviderAwsElRef {
    fn new(shared: StackShared, base: String) -> DataIamWorkloadIdentityPoolProviderAwsElRef {
        DataIamWorkloadIdentityPoolProviderAwsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIamWorkloadIdentityPoolProviderAwsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }
}
#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPoolProviderOidcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_audiences: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jwks_json: Option<PrimField<String>>,
}
impl DataIamWorkloadIdentityPoolProviderOidcEl {
    #[doc = "Set the field `allowed_audiences`.\n"]
    pub fn set_allowed_audiences(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_audiences = Some(v.into());
        self
    }
    #[doc = "Set the field `issuer_uri`.\n"]
    pub fn set_issuer_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuer_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `jwks_json`.\n"]
    pub fn set_jwks_json(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.jwks_json = Some(v.into());
        self
    }
}
impl ToListMappable for DataIamWorkloadIdentityPoolProviderOidcEl {
    type O = BlockAssignable<DataIamWorkloadIdentityPoolProviderOidcEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIamWorkloadIdentityPoolProviderOidcEl {}
impl BuildDataIamWorkloadIdentityPoolProviderOidcEl {
    pub fn build(self) -> DataIamWorkloadIdentityPoolProviderOidcEl {
        DataIamWorkloadIdentityPoolProviderOidcEl {
            allowed_audiences: core::default::Default::default(),
            issuer_uri: core::default::Default::default(),
            jwks_json: core::default::Default::default(),
        }
    }
}
pub struct DataIamWorkloadIdentityPoolProviderOidcElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamWorkloadIdentityPoolProviderOidcElRef {
    fn new(shared: StackShared, base: String) -> DataIamWorkloadIdentityPoolProviderOidcElRef {
        DataIamWorkloadIdentityPoolProviderOidcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIamWorkloadIdentityPoolProviderOidcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_audiences` after provisioning.\n"]
    pub fn allowed_audiences(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_audiences", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `issuer_uri` after provisioning.\n"]
    pub fn issuer_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `jwks_json` after provisioning.\n"]
    pub fn jwks_json(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.jwks_json", self.base))
    }
}
#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPoolProviderSamlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idp_metadata_xml: Option<PrimField<String>>,
}
impl DataIamWorkloadIdentityPoolProviderSamlEl {
    #[doc = "Set the field `idp_metadata_xml`.\n"]
    pub fn set_idp_metadata_xml(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.idp_metadata_xml = Some(v.into());
        self
    }
}
impl ToListMappable for DataIamWorkloadIdentityPoolProviderSamlEl {
    type O = BlockAssignable<DataIamWorkloadIdentityPoolProviderSamlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIamWorkloadIdentityPoolProviderSamlEl {}
impl BuildDataIamWorkloadIdentityPoolProviderSamlEl {
    pub fn build(self) -> DataIamWorkloadIdentityPoolProviderSamlEl {
        DataIamWorkloadIdentityPoolProviderSamlEl {
            idp_metadata_xml: core::default::Default::default(),
        }
    }
}
pub struct DataIamWorkloadIdentityPoolProviderSamlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamWorkloadIdentityPoolProviderSamlElRef {
    fn new(shared: StackShared, base: String) -> DataIamWorkloadIdentityPoolProviderSamlElRef {
        DataIamWorkloadIdentityPoolProviderSamlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIamWorkloadIdentityPoolProviderSamlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `idp_metadata_xml` after provisioning.\n"]
    pub fn idp_metadata_xml(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idp_metadata_xml", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pem_certificate: Option<PrimField<String>>,
}
impl DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasEl {
    #[doc = "Set the field `pem_certificate`.\n"]
    pub fn set_pem_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pem_certificate = Some(v.into());
        self
    }
}
impl ToListMappable for DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasEl {
    type O =
        BlockAssignable<DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasEl {}
impl BuildDataIamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasEl {
    pub fn build(self) -> DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasEl {
        DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasEl {
            pem_certificate: core::default::Default::default(),
        }
    }
}
pub struct DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasElRef {
        DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pem_certificate` after provisioning.\n"]
    pub fn pem_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pem_certificate", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pem_certificate: Option<PrimField<String>>,
}
impl DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsEl {
    #[doc = "Set the field `pem_certificate`.\n"]
    pub fn set_pem_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pem_certificate = Some(v.into());
        self
    }
}
impl ToListMappable for DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsEl {
    type O = BlockAssignable<DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsEl {}
impl BuildDataIamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsEl {
    pub fn build(self) -> DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsEl {
        DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsEl {
            pem_certificate: core::default::Default::default(),
        }
    }
}
pub struct DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsElRef {
        DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pem_certificate` after provisioning.\n"]
    pub fn pem_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pem_certificate", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPoolProviderX509ElTrustStoreEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    intermediate_cas:
        Option<ListField<DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trust_anchors:
        Option<ListField<DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsEl>>,
}
impl DataIamWorkloadIdentityPoolProviderX509ElTrustStoreEl {
    #[doc = "Set the field `intermediate_cas`.\n"]
    pub fn set_intermediate_cas(
        mut self,
        v: impl Into<ListField<DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasEl>>,
    ) -> Self {
        self.intermediate_cas = Some(v.into());
        self
    }
    #[doc = "Set the field `trust_anchors`.\n"]
    pub fn set_trust_anchors(
        mut self,
        v: impl Into<ListField<DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsEl>>,
    ) -> Self {
        self.trust_anchors = Some(v.into());
        self
    }
}
impl ToListMappable for DataIamWorkloadIdentityPoolProviderX509ElTrustStoreEl {
    type O = BlockAssignable<DataIamWorkloadIdentityPoolProviderX509ElTrustStoreEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIamWorkloadIdentityPoolProviderX509ElTrustStoreEl {}
impl BuildDataIamWorkloadIdentityPoolProviderX509ElTrustStoreEl {
    pub fn build(self) -> DataIamWorkloadIdentityPoolProviderX509ElTrustStoreEl {
        DataIamWorkloadIdentityPoolProviderX509ElTrustStoreEl {
            intermediate_cas: core::default::Default::default(),
            trust_anchors: core::default::Default::default(),
        }
    }
}
pub struct DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElRef {
        DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `intermediate_cas` after provisioning.\n"]
    pub fn intermediate_cas(
        &self,
    ) -> ListRef<DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.intermediate_cas", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `trust_anchors` after provisioning.\n"]
    pub fn trust_anchors(
        &self,
    ) -> ListRef<DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trust_anchors", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPoolProviderX509El {
    #[serde(skip_serializing_if = "Option::is_none")]
    trust_store: Option<ListField<DataIamWorkloadIdentityPoolProviderX509ElTrustStoreEl>>,
}
impl DataIamWorkloadIdentityPoolProviderX509El {
    #[doc = "Set the field `trust_store`.\n"]
    pub fn set_trust_store(
        mut self,
        v: impl Into<ListField<DataIamWorkloadIdentityPoolProviderX509ElTrustStoreEl>>,
    ) -> Self {
        self.trust_store = Some(v.into());
        self
    }
}
impl ToListMappable for DataIamWorkloadIdentityPoolProviderX509El {
    type O = BlockAssignable<DataIamWorkloadIdentityPoolProviderX509El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIamWorkloadIdentityPoolProviderX509El {}
impl BuildDataIamWorkloadIdentityPoolProviderX509El {
    pub fn build(self) -> DataIamWorkloadIdentityPoolProviderX509El {
        DataIamWorkloadIdentityPoolProviderX509El {
            trust_store: core::default::Default::default(),
        }
    }
}
pub struct DataIamWorkloadIdentityPoolProviderX509ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamWorkloadIdentityPoolProviderX509ElRef {
    fn new(shared: StackShared, base: String) -> DataIamWorkloadIdentityPoolProviderX509ElRef {
        DataIamWorkloadIdentityPoolProviderX509ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIamWorkloadIdentityPoolProviderX509ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `trust_store` after provisioning.\n"]
    pub fn trust_store(&self) -> ListRef<DataIamWorkloadIdentityPoolProviderX509ElTrustStoreElRef> {
        ListRef::new(self.shared().clone(), format!("{}.trust_store", self.base))
    }
}
