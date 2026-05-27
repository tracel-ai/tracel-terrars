use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct IamWorkloadIdentityPoolProviderData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute_condition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute_mapping: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    workload_identity_pool_id: PrimField<String>,
    workload_identity_pool_provider_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws: Option<Vec<IamWorkloadIdentityPoolProviderAwsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oidc: Option<Vec<IamWorkloadIdentityPoolProviderOidcEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saml: Option<Vec<IamWorkloadIdentityPoolProviderSamlEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IamWorkloadIdentityPoolProviderTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x509: Option<Vec<IamWorkloadIdentityPoolProviderX509El>>,
    dynamic: IamWorkloadIdentityPoolProviderDynamic,
}
struct IamWorkloadIdentityPoolProvider_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IamWorkloadIdentityPoolProviderData>,
}
#[derive(Clone)]
pub struct IamWorkloadIdentityPoolProvider(Rc<IamWorkloadIdentityPoolProvider_>);
impl IamWorkloadIdentityPoolProvider {
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
    #[doc = "Set the field `attribute_condition`.\n[A Common Expression Language](https://github.com/google/cel-spec) expression, in\nplain text, to restrict what otherwise valid authentication credentials issued by the\nprovider should not be accepted.\n\nThe expression must output a boolean representing whether to allow the federation.\n\nThe following keywords may be referenced in the expressions:\n  * 'assertion': JSON representing the authentication credential issued by the provider.\n  * 'google': The Google attributes mapped from the assertion in the 'attribute_mappings'.\n  * 'attribute': The custom attributes mapped from the assertion in the 'attribute_mappings'.\n\nThe maximum length of the attribute condition expression is 4096 characters. If\nunspecified, all valid authentication credential are accepted.\n\nThe following example shows how to only allow credentials with a mapped 'google.groups'\nvalue of 'admins':\n'''\n\"'admins' in google.groups\"\n'''"]
    pub fn set_attribute_condition(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().attribute_condition = Some(v.into());
        self
    }
    #[doc = "Set the field `attribute_mapping`.\nMaps attributes from authentication credentials issued by an external identity provider\nto Google Cloud attributes, such as 'subject' and 'segment'.\n\nEach key must be a string specifying the Google Cloud IAM attribute to map to.\n\nThe following keys are supported:\n  * 'google.subject': The principal IAM is authenticating. You can reference this value\n    in IAM bindings. This is also the subject that appears in Cloud Logging logs.\n    Cannot exceed 127 characters.\n  * 'google.groups': Groups the external identity belongs to. You can grant groups\n    access to resources using an IAM 'principalSet' binding; access applies to all\n    members of the group.\n\nYou can also provide custom attributes by specifying 'attribute.{custom_attribute}',\nwhere '{custom_attribute}' is the name of the custom attribute to be mapped. You can\ndefine a maximum of 50 custom attributes. The maximum length of a mapped attribute key\nis 100 characters, and the key may only contain the characters [a-z0-9_].\n\nYou can reference these attributes in IAM policies to define fine-grained access for a\nworkload to Google Cloud resources. For example:\n  * 'google.subject':\n    'principal://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/subject/{value}'\n  * 'google.groups':\n    'principalSet://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/group/{value}'\n  * 'attribute.{custom_attribute}':\n    'principalSet://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/attribute.{custom_attribute}/{value}'\n\nEach value must be a [Common Expression Language](https://github.com/google/cel-spec)\nfunction that maps an identity provider credential to the normalized attribute specified\nby the corresponding map key.\n\nYou can use the 'assertion' keyword in the expression to access a JSON representation of\nthe authentication credential issued by the provider.\n\nThe maximum length of an attribute mapping expression is 2048 characters. When evaluated,\nthe total size of all mapped attributes must not exceed 8KB.\n\nFor AWS providers, the following rules apply:\n  - If no attribute mapping is defined, the following default mapping applies:\n    '''\n    {\n      \"google.subject\":\"assertion.arn\",\n      \"attribute.aws_role\":\n        \"assertion.arn.contains('assumed-role')\"\n        \" ? assertion.arn.extract('{account_arn}assumed-role/')\"\n        \"   + 'assumed-role/'\"\n        \"   + assertion.arn.extract('assumed-role/{role_name}/')\"\n        \" : assertion.arn\",\n    }\n    '''\n  - If any custom attribute mappings are defined, they must include a mapping to the\n    'google.subject' attribute.\n\nFor OIDC providers, the following rules apply:\n  - Custom attribute mappings must be defined, and must include a mapping to the\n    'google.subject' attribute. For example, the following maps the 'sub' claim of the\n    incoming credential to the 'subject' attribute on a Google token.\n    '''\n    {\"google.subject\": \"assertion.sub\"}\n    '''"]
    pub fn set_attribute_mapping(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().attribute_mapping = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA description for the provider. Cannot exceed 256 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nWhether the provider is disabled. You cannot use a disabled provider to exchange tokens.\nHowever, existing tokens still grant access."]
    pub fn set_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nA display name for the provider. Cannot exceed 32 characters."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
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
    #[doc = "Set the field `aws`.\n"]
    pub fn set_aws(
        self,
        v: impl Into<BlockAssignable<IamWorkloadIdentityPoolProviderAwsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().aws = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.aws = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oidc`.\n"]
    pub fn set_oidc(
        self,
        v: impl Into<BlockAssignable<IamWorkloadIdentityPoolProviderOidcEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().oidc = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.oidc = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `saml`.\n"]
    pub fn set_saml(
        self,
        v: impl Into<BlockAssignable<IamWorkloadIdentityPoolProviderSamlEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().saml = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.saml = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<IamWorkloadIdentityPoolProviderTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `x509`.\n"]
    pub fn set_x509(
        self,
        v: impl Into<BlockAssignable<IamWorkloadIdentityPoolProviderX509El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().x509 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.x509 = Some(d);
            }
        }
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
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `aws` after provisioning.\n"]
    pub fn aws(&self) -> ListRef<IamWorkloadIdentityPoolProviderAwsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aws", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> ListRef<IamWorkloadIdentityPoolProviderOidcElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oidc", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> ListRef<IamWorkloadIdentityPoolProviderSamlElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.saml", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkloadIdentityPoolProviderTimeoutsElRef {
        IamWorkloadIdentityPoolProviderTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `x509` after provisioning.\n"]
    pub fn x509(&self) -> ListRef<IamWorkloadIdentityPoolProviderX509ElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.x509", self.extract_ref()),
        )
    }
}
impl Referable for IamWorkloadIdentityPoolProvider {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for IamWorkloadIdentityPoolProvider {}
impl ToListMappable for IamWorkloadIdentityPoolProvider {
    type O = ListRef<IamWorkloadIdentityPoolProviderRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for IamWorkloadIdentityPoolProvider_ {
    fn extract_resource_type(&self) -> String {
        "google_iam_workload_identity_pool_provider".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildIamWorkloadIdentityPoolProvider {
    pub tf_id: String,
    #[doc = "The ID used for the pool, which is the final component of the pool resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub workload_identity_pool_id: PrimField<String>,
    #[doc = "The ID for the provider, which becomes the final component of the resource name. This\nvalue must be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub workload_identity_pool_provider_id: PrimField<String>,
}
impl BuildIamWorkloadIdentityPoolProvider {
    pub fn build(self, stack: &mut Stack) -> IamWorkloadIdentityPoolProvider {
        let out = IamWorkloadIdentityPoolProvider(Rc::new(IamWorkloadIdentityPoolProvider_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IamWorkloadIdentityPoolProviderData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                attribute_condition: core::default::Default::default(),
                attribute_mapping: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                disabled: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                workload_identity_pool_id: self.workload_identity_pool_id,
                workload_identity_pool_provider_id: self.workload_identity_pool_provider_id,
                aws: core::default::Default::default(),
                oidc: core::default::Default::default(),
                saml: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                x509: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct IamWorkloadIdentityPoolProviderRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkloadIdentityPoolProviderRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl IamWorkloadIdentityPoolProviderRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `aws` after provisioning.\n"]
    pub fn aws(&self) -> ListRef<IamWorkloadIdentityPoolProviderAwsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aws", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> ListRef<IamWorkloadIdentityPoolProviderOidcElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oidc", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `saml` after provisioning.\n"]
    pub fn saml(&self) -> ListRef<IamWorkloadIdentityPoolProviderSamlElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.saml", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkloadIdentityPoolProviderTimeoutsElRef {
        IamWorkloadIdentityPoolProviderTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `x509` after provisioning.\n"]
    pub fn x509(&self) -> ListRef<IamWorkloadIdentityPoolProviderX509ElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.x509", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolProviderAwsEl {
    account_id: PrimField<String>,
}
impl IamWorkloadIdentityPoolProviderAwsEl {}
impl ToListMappable for IamWorkloadIdentityPoolProviderAwsEl {
    type O = BlockAssignable<IamWorkloadIdentityPoolProviderAwsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamWorkloadIdentityPoolProviderAwsEl {
    #[doc = "The AWS account ID."]
    pub account_id: PrimField<String>,
}
impl BuildIamWorkloadIdentityPoolProviderAwsEl {
    pub fn build(self) -> IamWorkloadIdentityPoolProviderAwsEl {
        IamWorkloadIdentityPoolProviderAwsEl {
            account_id: self.account_id,
        }
    }
}
pub struct IamWorkloadIdentityPoolProviderAwsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkloadIdentityPoolProviderAwsElRef {
    fn new(shared: StackShared, base: String) -> IamWorkloadIdentityPoolProviderAwsElRef {
        IamWorkloadIdentityPoolProviderAwsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamWorkloadIdentityPoolProviderAwsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nThe AWS account ID."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }
}
#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolProviderOidcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_audiences: Option<ListField<PrimField<String>>>,
    issuer_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jwks_json: Option<PrimField<String>>,
}
impl IamWorkloadIdentityPoolProviderOidcEl {
    #[doc = "Set the field `allowed_audiences`.\nAcceptable values for the 'aud' field (audience) in the OIDC token. Token exchange\nrequests are rejected if the token audience does not match one of the configured\nvalues. Each audience may be at most 256 characters. A maximum of 10 audiences may\nbe configured.\n\nIf this list is empty, the OIDC token audience must be equal to the full canonical\nresource name of the WorkloadIdentityPoolProvider, with or without the HTTPS prefix.\nFor example:\n'''\n//iam.googleapis.com/projects/<project-number>/locations/<location>/workloadIdentityPools/<pool-id>/providers/<provider-id>\nhttps://iam.googleapis.com/projects/<project-number>/locations/<location>/workloadIdentityPools/<pool-id>/providers/<provider-id>\n'''"]
    pub fn set_allowed_audiences(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_audiences = Some(v.into());
        self
    }
    #[doc = "Set the field `jwks_json`.\nOIDC JWKs in JSON String format. For details on definition of a\nJWK, see https:tools.ietf.org/html/rfc7517. If not set, then we\nuse the 'jwks_uri' from the discovery document fetched from the\n.well-known path for the 'issuer_uri'. Currently, RSA and EC asymmetric\nkeys are supported. The JWK must use following format and include only\nthe following fields:\n'''\n{\n  \"keys\": [\n    {\n          \"kty\": \"RSA/EC\",\n          \"alg\": \"<algorithm>\",\n          \"use\": \"sig\",\n          \"kid\": \"<key-id>\",\n          \"n\": \"\",\n          \"e\": \"\",\n          \"x\": \"\",\n          \"y\": \"\",\n          \"crv\": \"\"\n    }\n  ]\n}\n'''"]
    pub fn set_jwks_json(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.jwks_json = Some(v.into());
        self
    }
}
impl ToListMappable for IamWorkloadIdentityPoolProviderOidcEl {
    type O = BlockAssignable<IamWorkloadIdentityPoolProviderOidcEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamWorkloadIdentityPoolProviderOidcEl {
    #[doc = "The OIDC issuer URL."]
    pub issuer_uri: PrimField<String>,
}
impl BuildIamWorkloadIdentityPoolProviderOidcEl {
    pub fn build(self) -> IamWorkloadIdentityPoolProviderOidcEl {
        IamWorkloadIdentityPoolProviderOidcEl {
            allowed_audiences: core::default::Default::default(),
            issuer_uri: self.issuer_uri,
            jwks_json: core::default::Default::default(),
        }
    }
}
pub struct IamWorkloadIdentityPoolProviderOidcElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkloadIdentityPoolProviderOidcElRef {
    fn new(shared: StackShared, base: String) -> IamWorkloadIdentityPoolProviderOidcElRef {
        IamWorkloadIdentityPoolProviderOidcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamWorkloadIdentityPoolProviderOidcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_audiences` after provisioning.\nAcceptable values for the 'aud' field (audience) in the OIDC token. Token exchange\nrequests are rejected if the token audience does not match one of the configured\nvalues. Each audience may be at most 256 characters. A maximum of 10 audiences may\nbe configured.\n\nIf this list is empty, the OIDC token audience must be equal to the full canonical\nresource name of the WorkloadIdentityPoolProvider, with or without the HTTPS prefix.\nFor example:\n'''\n//iam.googleapis.com/projects/<project-number>/locations/<location>/workloadIdentityPools/<pool-id>/providers/<provider-id>\nhttps://iam.googleapis.com/projects/<project-number>/locations/<location>/workloadIdentityPools/<pool-id>/providers/<provider-id>\n'''"]
    pub fn allowed_audiences(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_audiences", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `issuer_uri` after provisioning.\nThe OIDC issuer URL."]
    pub fn issuer_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `jwks_json` after provisioning.\nOIDC JWKs in JSON String format. For details on definition of a\nJWK, see https:tools.ietf.org/html/rfc7517. If not set, then we\nuse the 'jwks_uri' from the discovery document fetched from the\n.well-known path for the 'issuer_uri'. Currently, RSA and EC asymmetric\nkeys are supported. The JWK must use following format and include only\nthe following fields:\n'''\n{\n  \"keys\": [\n    {\n          \"kty\": \"RSA/EC\",\n          \"alg\": \"<algorithm>\",\n          \"use\": \"sig\",\n          \"kid\": \"<key-id>\",\n          \"n\": \"\",\n          \"e\": \"\",\n          \"x\": \"\",\n          \"y\": \"\",\n          \"crv\": \"\"\n    }\n  ]\n}\n'''"]
    pub fn jwks_json(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.jwks_json", self.base))
    }
}
#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolProviderSamlEl {
    idp_metadata_xml: PrimField<String>,
}
impl IamWorkloadIdentityPoolProviderSamlEl {}
impl ToListMappable for IamWorkloadIdentityPoolProviderSamlEl {
    type O = BlockAssignable<IamWorkloadIdentityPoolProviderSamlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamWorkloadIdentityPoolProviderSamlEl {
    #[doc = "SAML Identity provider configuration metadata xml doc."]
    pub idp_metadata_xml: PrimField<String>,
}
impl BuildIamWorkloadIdentityPoolProviderSamlEl {
    pub fn build(self) -> IamWorkloadIdentityPoolProviderSamlEl {
        IamWorkloadIdentityPoolProviderSamlEl {
            idp_metadata_xml: self.idp_metadata_xml,
        }
    }
}
pub struct IamWorkloadIdentityPoolProviderSamlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkloadIdentityPoolProviderSamlElRef {
    fn new(shared: StackShared, base: String) -> IamWorkloadIdentityPoolProviderSamlElRef {
        IamWorkloadIdentityPoolProviderSamlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamWorkloadIdentityPoolProviderSamlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `idp_metadata_xml` after provisioning.\nSAML Identity provider configuration metadata xml doc."]
    pub fn idp_metadata_xml(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idp_metadata_xml", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolProviderTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl IamWorkloadIdentityPoolProviderTimeoutsEl {
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
impl ToListMappable for IamWorkloadIdentityPoolProviderTimeoutsEl {
    type O = BlockAssignable<IamWorkloadIdentityPoolProviderTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamWorkloadIdentityPoolProviderTimeoutsEl {}
impl BuildIamWorkloadIdentityPoolProviderTimeoutsEl {
    pub fn build(self) -> IamWorkloadIdentityPoolProviderTimeoutsEl {
        IamWorkloadIdentityPoolProviderTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct IamWorkloadIdentityPoolProviderTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkloadIdentityPoolProviderTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> IamWorkloadIdentityPoolProviderTimeoutsElRef {
        IamWorkloadIdentityPoolProviderTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamWorkloadIdentityPoolProviderTimeoutsElRef {
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
pub struct IamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pem_certificate: Option<PrimField<String>>,
}
impl IamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasEl {
    #[doc = "Set the field `pem_certificate`.\nPEM certificate of the PKI used for validation. Must only contain one\nca certificate(either root or intermediate cert)."]
    pub fn set_pem_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pem_certificate = Some(v.into());
        self
    }
}
impl ToListMappable for IamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasEl {
    type O = BlockAssignable<IamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasEl {}
impl BuildIamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasEl {
    pub fn build(self) -> IamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasEl {
        IamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasEl {
            pem_certificate: core::default::Default::default(),
        }
    }
}
pub struct IamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasElRef {
        IamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pem_certificate` after provisioning.\nPEM certificate of the PKI used for validation. Must only contain one\nca certificate(either root or intermediate cert)."]
    pub fn pem_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pem_certificate", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pem_certificate: Option<PrimField<String>>,
}
impl IamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsEl {
    #[doc = "Set the field `pem_certificate`.\nPEM certificate of the PKI used for validation. Must only contain one\nca certificate(either root or intermediate cert)."]
    pub fn set_pem_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pem_certificate = Some(v.into());
        self
    }
}
impl ToListMappable for IamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsEl {
    type O = BlockAssignable<IamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsEl {}
impl BuildIamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsEl {
    pub fn build(self) -> IamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsEl {
        IamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsEl {
            pem_certificate: core::default::Default::default(),
        }
    }
}
pub struct IamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsElRef {
        IamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pem_certificate` after provisioning.\nPEM certificate of the PKI used for validation. Must only contain one\nca certificate(either root or intermediate cert)."]
    pub fn pem_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pem_certificate", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct IamWorkloadIdentityPoolProviderX509ElTrustStoreElDynamic {
    intermediate_cas:
        Option<DynamicBlock<IamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasEl>>,
    trust_anchors:
        Option<DynamicBlock<IamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsEl>>,
}
#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolProviderX509ElTrustStoreEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    intermediate_cas:
        Option<Vec<IamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trust_anchors: Option<Vec<IamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsEl>>,
    dynamic: IamWorkloadIdentityPoolProviderX509ElTrustStoreElDynamic,
}
impl IamWorkloadIdentityPoolProviderX509ElTrustStoreEl {
    #[doc = "Set the field `intermediate_cas`.\n"]
    pub fn set_intermediate_cas(
        mut self,
        v: impl Into<
            BlockAssignable<IamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.intermediate_cas = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.intermediate_cas = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `trust_anchors`.\n"]
    pub fn set_trust_anchors(
        mut self,
        v: impl Into<BlockAssignable<IamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.trust_anchors = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.trust_anchors = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IamWorkloadIdentityPoolProviderX509ElTrustStoreEl {
    type O = BlockAssignable<IamWorkloadIdentityPoolProviderX509ElTrustStoreEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamWorkloadIdentityPoolProviderX509ElTrustStoreEl {}
impl BuildIamWorkloadIdentityPoolProviderX509ElTrustStoreEl {
    pub fn build(self) -> IamWorkloadIdentityPoolProviderX509ElTrustStoreEl {
        IamWorkloadIdentityPoolProviderX509ElTrustStoreEl {
            intermediate_cas: core::default::Default::default(),
            trust_anchors: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IamWorkloadIdentityPoolProviderX509ElTrustStoreElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkloadIdentityPoolProviderX509ElTrustStoreElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IamWorkloadIdentityPoolProviderX509ElTrustStoreElRef {
        IamWorkloadIdentityPoolProviderX509ElTrustStoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamWorkloadIdentityPoolProviderX509ElTrustStoreElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `intermediate_cas` after provisioning.\n"]
    pub fn intermediate_cas(
        &self,
    ) -> ListRef<IamWorkloadIdentityPoolProviderX509ElTrustStoreElIntermediateCasElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.intermediate_cas", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `trust_anchors` after provisioning.\n"]
    pub fn trust_anchors(
        &self,
    ) -> ListRef<IamWorkloadIdentityPoolProviderX509ElTrustStoreElTrustAnchorsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trust_anchors", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct IamWorkloadIdentityPoolProviderX509ElDynamic {
    trust_store: Option<DynamicBlock<IamWorkloadIdentityPoolProviderX509ElTrustStoreEl>>,
}
#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolProviderX509El {
    #[serde(skip_serializing_if = "Option::is_none")]
    trust_store: Option<Vec<IamWorkloadIdentityPoolProviderX509ElTrustStoreEl>>,
    dynamic: IamWorkloadIdentityPoolProviderX509ElDynamic,
}
impl IamWorkloadIdentityPoolProviderX509El {
    #[doc = "Set the field `trust_store`.\n"]
    pub fn set_trust_store(
        mut self,
        v: impl Into<BlockAssignable<IamWorkloadIdentityPoolProviderX509ElTrustStoreEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.trust_store = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.trust_store = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for IamWorkloadIdentityPoolProviderX509El {
    type O = BlockAssignable<IamWorkloadIdentityPoolProviderX509El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildIamWorkloadIdentityPoolProviderX509El {}
impl BuildIamWorkloadIdentityPoolProviderX509El {
    pub fn build(self) -> IamWorkloadIdentityPoolProviderX509El {
        IamWorkloadIdentityPoolProviderX509El {
            trust_store: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct IamWorkloadIdentityPoolProviderX509ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for IamWorkloadIdentityPoolProviderX509ElRef {
    fn new(shared: StackShared, base: String) -> IamWorkloadIdentityPoolProviderX509ElRef {
        IamWorkloadIdentityPoolProviderX509ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl IamWorkloadIdentityPoolProviderX509ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `trust_store` after provisioning.\n"]
    pub fn trust_store(&self) -> ListRef<IamWorkloadIdentityPoolProviderX509ElTrustStoreElRef> {
        ListRef::new(self.shared().clone(), format!("{}.trust_store", self.base))
    }
}
#[derive(Serialize, Default)]
struct IamWorkloadIdentityPoolProviderDynamic {
    aws: Option<DynamicBlock<IamWorkloadIdentityPoolProviderAwsEl>>,
    oidc: Option<DynamicBlock<IamWorkloadIdentityPoolProviderOidcEl>>,
    saml: Option<DynamicBlock<IamWorkloadIdentityPoolProviderSamlEl>>,
    x509: Option<DynamicBlock<IamWorkloadIdentityPoolProviderX509El>>,
}
