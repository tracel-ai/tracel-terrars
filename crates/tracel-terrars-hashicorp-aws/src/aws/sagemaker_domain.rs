use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerDomainData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_network_access_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_security_group_management: Option<PrimField<String>>,
    auth_mode: PrimField<String>,
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    subnet_ids: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_propagation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    vpc_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_space_settings: Option<Vec<SagemakerDomainDefaultSpaceSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_user_settings: Option<Vec<SagemakerDomainDefaultUserSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_settings: Option<Vec<SagemakerDomainDomainSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_policy: Option<Vec<SagemakerDomainRetentionPolicyEl>>,
    dynamic: SagemakerDomainDynamic,
}

struct SagemakerDomain_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SagemakerDomainData>,
}

#[derive(Clone)]
pub struct SagemakerDomain(Rc<SagemakerDomain_>);

impl SagemakerDomain {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `app_network_access_type`.\n"]
    pub fn set_app_network_access_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().app_network_access_type = Some(v.into());
        self
    }

    #[doc = "Set the field `app_security_group_management`.\n"]
    pub fn set_app_security_group_management(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().app_security_group_management = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tag_propagation`.\n"]
    pub fn set_tag_propagation(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tag_propagation = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `default_space_settings`.\n"]
    pub fn set_default_space_settings(
        self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultSpaceSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_space_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_space_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_user_settings`.\n"]
    pub fn set_default_user_settings(self, v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_user_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_user_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `domain_settings`.\n"]
    pub fn set_domain_settings(self, v: impl Into<BlockAssignable<SagemakerDomainDomainSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().domain_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.domain_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retention_policy`.\n"]
    pub fn set_retention_policy(self, v: impl Into<BlockAssignable<SagemakerDomainRetentionPolicyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retention_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retention_policy = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `app_network_access_type` after provisioning.\n"]
    pub fn app_network_access_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_network_access_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_security_group_management` after provisioning.\n"]
    pub fn app_security_group_management(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_security_group_management", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_mode` after provisioning.\n"]
    pub fn auth_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `home_efs_file_system_id` after provisioning.\n"]
    pub fn home_efs_file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.home_efs_file_system_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_id_for_domain_boundary` after provisioning.\n"]
    pub fn security_group_id_for_domain_boundary(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_group_id_for_domain_boundary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `single_sign_on_application_arn` after provisioning.\n"]
    pub fn single_sign_on_application_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.single_sign_on_application_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `single_sign_on_managed_application_instance_id` after provisioning.\n"]
    pub fn single_sign_on_managed_application_instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.single_sign_on_managed_application_instance_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag_propagation` after provisioning.\n"]
    pub fn tag_propagation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_propagation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_space_settings` after provisioning.\n"]
    pub fn default_space_settings(&self) -> ListRef<SagemakerDomainDefaultSpaceSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_space_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_user_settings` after provisioning.\n"]
    pub fn default_user_settings(&self) -> ListRef<SagemakerDomainDefaultUserSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_user_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_settings` after provisioning.\n"]
    pub fn domain_settings(&self) -> ListRef<SagemakerDomainDomainSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.domain_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_policy` after provisioning.\n"]
    pub fn retention_policy(&self) -> ListRef<SagemakerDomainRetentionPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retention_policy", self.extract_ref()))
    }
}

impl Referable for SagemakerDomain {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for SagemakerDomain { }

impl ToListMappable for SagemakerDomain {
    type O = ListRef<SagemakerDomainRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SagemakerDomain_ {
    fn extract_resource_type(&self) -> String {
        "aws_sagemaker_domain".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerDomain {
    pub tf_id: String,
    #[doc = ""]
    pub auth_mode: PrimField<String>,
    #[doc = ""]
    pub domain_name: PrimField<String>,
    #[doc = ""]
    pub subnet_ids: SetField<PrimField<String>>,
    #[doc = ""]
    pub vpc_id: PrimField<String>,
}

impl BuildSagemakerDomain {
    pub fn build(self, stack: &mut Stack) -> SagemakerDomain {
        let out = SagemakerDomain(Rc::new(SagemakerDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SagemakerDomainData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                app_network_access_type: core::default::Default::default(),
                app_security_group_management: core::default::Default::default(),
                auth_mode: self.auth_mode,
                domain_name: self.domain_name,
                id: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                region: core::default::Default::default(),
                subnet_ids: self.subnet_ids,
                tag_propagation: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                vpc_id: self.vpc_id,
                default_space_settings: core::default::Default::default(),
                default_user_settings: core::default::Default::default(),
                domain_settings: core::default::Default::default(),
                retention_policy: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerDomainRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl SagemakerDomainRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_network_access_type` after provisioning.\n"]
    pub fn app_network_access_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_network_access_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_security_group_management` after provisioning.\n"]
    pub fn app_security_group_management(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_security_group_management", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_mode` after provisioning.\n"]
    pub fn auth_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `home_efs_file_system_id` after provisioning.\n"]
    pub fn home_efs_file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.home_efs_file_system_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_id_for_domain_boundary` after provisioning.\n"]
    pub fn security_group_id_for_domain_boundary(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_group_id_for_domain_boundary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `single_sign_on_application_arn` after provisioning.\n"]
    pub fn single_sign_on_application_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.single_sign_on_application_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `single_sign_on_managed_application_instance_id` after provisioning.\n"]
    pub fn single_sign_on_managed_application_instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.single_sign_on_managed_application_instance_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag_propagation` after provisioning.\n"]
    pub fn tag_propagation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_propagation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_space_settings` after provisioning.\n"]
    pub fn default_space_settings(&self) -> ListRef<SagemakerDomainDefaultSpaceSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_space_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_user_settings` after provisioning.\n"]
    pub fn default_user_settings(&self) -> ListRef<SagemakerDomainDefaultUserSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_user_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_settings` after provisioning.\n"]
    pub fn domain_settings(&self) -> ListRef<SagemakerDomainDomainSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.domain_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_policy` after provisioning.\n"]
    pub fn retention_policy(&self) -> ListRef<SagemakerDomainRetentionPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retention_policy", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl {
    file_system_id: PrimField<String>,
    file_system_path: PrimField<String>,
}

impl SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl { }

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl {
    type O = BlockAssignable<SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl {
    #[doc = ""]
    pub file_system_id: PrimField<String>,
    #[doc = ""]
    pub file_system_path: PrimField<String>,
}

impl BuildSagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl {
        SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl {
            file_system_id: self.file_system_id,
            file_system_path: self.file_system_path,
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigElEfsFileSystemConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigElEfsFileSystemConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigElEfsFileSystemConfigElRef {
        SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigElEfsFileSystemConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigElEfsFileSystemConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_path` after provisioning.\n"]
    pub fn file_system_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_system_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigElDynamic {
    efs_file_system_config: Option<
        DynamicBlock<SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    efs_file_system_config: Option<
        Vec<SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl>,
    >,
    dynamic: SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigElDynamic,
}

impl SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigEl {
    #[doc = "Set the field `efs_file_system_config`.\n"]
    pub fn set_efs_file_system_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.efs_file_system_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.efs_file_system_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigEl {
    type O = BlockAssignable<SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigEl {}

impl BuildSagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigEl {
        SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigEl {
            efs_file_system_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigElRef {
        SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `efs_file_system_config` after provisioning.\n"]
    pub fn efs_file_system_config(
        &self,
    ) -> ListRef<SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigElEfsFileSystemConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.efs_file_system_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElCustomPosixUserConfigEl {
    gid: PrimField<f64>,
    uid: PrimField<f64>,
}

impl SagemakerDomainDefaultSpaceSettingsElCustomPosixUserConfigEl { }

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsElCustomPosixUserConfigEl {
    type O = BlockAssignable<SagemakerDomainDefaultSpaceSettingsElCustomPosixUserConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElCustomPosixUserConfigEl {
    #[doc = ""]
    pub gid: PrimField<f64>,
    #[doc = ""]
    pub uid: PrimField<f64>,
}

impl BuildSagemakerDomainDefaultSpaceSettingsElCustomPosixUserConfigEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsElCustomPosixUserConfigEl {
        SagemakerDomainDefaultSpaceSettingsElCustomPosixUserConfigEl {
            gid: self.gid,
            uid: self.uid,
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElCustomPosixUserConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElCustomPosixUserConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultSpaceSettingsElCustomPosixUserConfigElRef {
        SagemakerDomainDefaultSpaceSettingsElCustomPosixUserConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElCustomPosixUserConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gid` after provisioning.\n"]
    pub fn gid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.gid", self.base))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_timeout_in_minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_management: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_idle_timeout_in_minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_idle_timeout_in_minutes: Option<PrimField<f64>>,
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl {
    #[doc = "Set the field `idle_timeout_in_minutes`.\n"]
    pub fn set_idle_timeout_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.idle_timeout_in_minutes = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_management`.\n"]
    pub fn set_lifecycle_management(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_management = Some(v.into());
        self
    }

    #[doc = "Set the field `max_idle_timeout_in_minutes`.\n"]
    pub fn set_max_idle_timeout_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_idle_timeout_in_minutes = Some(v.into());
        self
    }

    #[doc = "Set the field `min_idle_timeout_in_minutes`.\n"]
    pub fn set_min_idle_timeout_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_idle_timeout_in_minutes = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl {
    type O =
        BlockAssignable<
            SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl {}

impl BuildSagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl {
    pub fn build(
        self,
    ) -> SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl {
        SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl {
            idle_timeout_in_minutes: core::default::Default::default(),
            lifecycle_management: core::default::Default::default(),
            max_idle_timeout_in_minutes: core::default::Default::default(),
            min_idle_timeout_in_minutes: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsElRef {
        SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle_timeout_in_minutes` after provisioning.\n"]
    pub fn idle_timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_management` after provisioning.\n"]
    pub fn lifecycle_management(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_management", self.base))
    }

    #[doc = "Get a reference to the value of field `max_idle_timeout_in_minutes` after provisioning.\n"]
    pub fn max_idle_timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_idle_timeout_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `min_idle_timeout_in_minutes` after provisioning.\n"]
    pub fn min_idle_timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_idle_timeout_in_minutes", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElDynamic {
    idle_settings: Option<
        DynamicBlock<
            SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_settings: Option<
        Vec<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl>,
    >,
    dynamic: SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElDynamic,
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl {
    #[doc = "Set the field `idle_settings`.\n"]
    pub fn set_idle_settings(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.idle_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.idle_settings = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl {
    type O = BlockAssignable<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl {}

impl BuildSagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl {
        SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl {
            idle_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElRef {
        SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle_settings` after provisioning.\n"]
    pub fn idle_settings(
        &self,
    ) -> ListRef<
        SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.idle_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryEl {
    repository_url: PrimField<String>,
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryEl { }

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryEl {
    type O = BlockAssignable<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryEl {
    #[doc = ""]
    pub repository_url: PrimField<String>,
}

impl BuildSagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryEl {
        SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryEl {
            repository_url: self.repository_url,
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryElRef {
        SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_url", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCustomImageEl {
    app_image_config_name: PrimField<String>,
    image_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_version_number: Option<PrimField<f64>>,
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCustomImageEl {
    #[doc = "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.image_version_number = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCustomImageEl {
    type O = BlockAssignable<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCustomImageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCustomImageEl {
    #[doc = ""]
    pub app_image_config_name: PrimField<String>,
    #[doc = ""]
    pub image_name: PrimField<String>,
}

impl BuildSagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCustomImageEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCustomImageEl {
        SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCustomImageEl {
            app_image_config_name: self.app_image_config_name,
            image_name: self.image_name,
            image_version_number: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCustomImageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCustomImageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCustomImageElRef {
        SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCustomImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCustomImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_image_config_name", self.base))
    }

    #[doc = "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_name", self.base))
    }

    #[doc = "Get a reference to the value of field `image_version_number` after provisioning.\n"]
    pub fn image_version_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_version_number", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_alias`.\n"]
    pub fn set_sagemaker_image_version_alias(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_alias = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {
    type O = BlockAssignable<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {
        SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_alias: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecElRef {
        SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_alias` after provisioning.\n"]
    pub fn sagemaker_image_version_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_alias", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElEmrSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    assumable_role_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_role_arns: Option<SetField<PrimField<String>>>,
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElEmrSettingsEl {
    #[doc = "Set the field `assumable_role_arns`.\n"]
    pub fn set_assumable_role_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.assumable_role_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `execution_role_arns`.\n"]
    pub fn set_execution_role_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.execution_role_arns = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElEmrSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElEmrSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElEmrSettingsEl {}

impl BuildSagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElEmrSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElEmrSettingsEl {
        SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElEmrSettingsEl {
            assumable_role_arns: core::default::Default::default(),
            execution_role_arns: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElEmrSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElEmrSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElEmrSettingsElRef {
        SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElEmrSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElEmrSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `assumable_role_arns` after provisioning.\n"]
    pub fn assumable_role_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.assumable_role_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_role_arns` after provisioning.\n"]
    pub fn execution_role_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.execution_role_arns", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElDynamic {
    app_lifecycle_management: Option<
        DynamicBlock<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl>,
    >,
    code_repository: Option<
        DynamicBlock<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryEl>,
    >,
    custom_image: Option<DynamicBlock<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCustomImageEl>>,
    default_resource_spec: Option<
        DynamicBlock<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl>,
    >,
    emr_settings: Option<DynamicBlock<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElEmrSettingsEl>>,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    built_in_lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_lifecycle_management: Option<
        Vec<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_repository: Option<Vec<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_image: Option<Vec<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCustomImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec: Option<Vec<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emr_settings: Option<Vec<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElEmrSettingsEl>>,
    dynamic: SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElDynamic,
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsEl {
    #[doc = "Set the field `built_in_lifecycle_config_arn`.\n"]
    pub fn set_built_in_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.built_in_lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.lifecycle_config_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `app_lifecycle_management`.\n"]
    pub fn set_app_lifecycle_management(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.app_lifecycle_management = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.app_lifecycle_management = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `code_repository`.\n"]
    pub fn set_code_repository(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCodeRepositoryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_repository = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_repository = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCustomImageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_image = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_image = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `emr_settings`.\n"]
    pub fn set_emr_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElEmrSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.emr_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.emr_settings = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsEl {}

impl BuildSagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsEl {
        SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsEl {
            built_in_lifecycle_config_arn: core::default::Default::default(),
            lifecycle_config_arns: core::default::Default::default(),
            app_lifecycle_management: core::default::Default::default(),
            code_repository: core::default::Default::default(),
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            emr_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElRef {
        SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `built_in_lifecycle_config_arn` after provisioning.\n"]
    pub fn built_in_lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.built_in_lifecycle_config_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `app_lifecycle_management` after provisioning.\n"]
    pub fn app_lifecycle_management(
        &self,
    ) -> ListRef<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElAppLifecycleManagementElRef> {
        ListRef::new(self.shared().clone(), format!("{}.app_lifecycle_management", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(&self) -> ListRef<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElCustomImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElDefaultResourceSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `emr_settings` after provisioning.\n"]
    pub fn emr_settings(&self) -> ListRef<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElEmrSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.emr_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    repository_url: PrimField<String>,
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl { }

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    type O = BlockAssignable<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    #[doc = ""]
    pub repository_url: PrimField<String>,
}

impl BuildSagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
        SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
            repository_url: self.repository_url,
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
        SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_url", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_alias`.\n"]
    pub fn set_sagemaker_image_version_alias(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_alias = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    type O = BlockAssignable<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
        SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_alias: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
        SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_alias` after provisioning.\n"]
    pub fn sagemaker_image_version_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_alias", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDynamic {
    code_repository: Option<
        DynamicBlock<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl>,
    >,
    default_resource_spec: Option<
        DynamicBlock<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_repository: Option<Vec<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec: Option<
        Vec<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>,
    >,
    dynamic: SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDynamic,
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl {
    #[doc = "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.lifecycle_config_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `code_repository`.\n"]
    pub fn set_code_repository(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElCodeRepositoryEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_repository = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_repository = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl {}

impl BuildSagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl {
        SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl {
            lifecycle_config_arns: core::default::Default::default(),
            code_repository: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElRef {
        SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    app_image_config_name: PrimField<String>,
    image_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_version_number: Option<PrimField<f64>>,
}

impl SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    #[doc = "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.image_version_number = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    type O = BlockAssignable<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    #[doc = ""]
    pub app_image_config_name: PrimField<String>,
    #[doc = ""]
    pub image_name: PrimField<String>,
}

impl BuildSagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
        SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl {
            app_image_config_name: self.app_image_config_name,
            image_name: self.image_name,
            image_version_number: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
        SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_image_config_name", self.base))
    }

    #[doc = "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_name", self.base))
    }

    #[doc = "Get a reference to the value of field `image_version_number` after provisioning.\n"]
    pub fn image_version_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_version_number", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_alias`.\n"]
    pub fn set_sagemaker_image_version_alias(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_alias = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    type O = BlockAssignable<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
        SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_alias: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
        SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_alias` after provisioning.\n"]
    pub fn sagemaker_image_version_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_alias", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDynamic {
    custom_image: Option<DynamicBlock<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    default_resource_spec: Option<
        DynamicBlock<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_image: Option<Vec<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec: Option<
        Vec<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>,
    >,
    dynamic: SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDynamic,
}

impl SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl {
    #[doc = "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.lifecycle_config_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_image = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_image = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl {}

impl BuildSagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl {
        SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl {
            lifecycle_config_arns: core::default::Default::default(),
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElRef {
        SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(
        &self,
    ) -> ListRef<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElCustomImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl {
    default_ebs_volume_size_in_gb: PrimField<f64>,
    maximum_ebs_volume_size_in_gb: PrimField<f64>,
}

impl SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl { }

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl {
    #[doc = ""]
    pub default_ebs_volume_size_in_gb: PrimField<f64>,
    #[doc = ""]
    pub maximum_ebs_volume_size_in_gb: PrimField<f64>,
}

impl BuildSagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl {
        SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl {
            default_ebs_volume_size_in_gb: self.default_ebs_volume_size_in_gb,
            maximum_ebs_volume_size_in_gb: self.maximum_ebs_volume_size_in_gb,
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsElRef {
        SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_ebs_volume_size_in_gb` after provisioning.\n"]
    pub fn default_ebs_volume_size_in_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_ebs_volume_size_in_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_ebs_volume_size_in_gb` after provisioning.\n"]
    pub fn maximum_ebs_volume_size_in_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_ebs_volume_size_in_gb", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsElDynamic {
    default_ebs_storage_settings: Option<
        DynamicBlock<SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_ebs_storage_settings: Option<
        Vec<SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl>,
    >,
    dynamic: SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsElDynamic,
}

impl SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsEl {
    #[doc = "Set the field `default_ebs_storage_settings`.\n"]
    pub fn set_default_ebs_storage_settings(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_ebs_storage_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_ebs_storage_settings = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsEl {}

impl BuildSagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsEl {
        SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsEl {
            default_ebs_storage_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsElRef {
        SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_ebs_storage_settings` after provisioning.\n"]
    pub fn default_ebs_storage_settings(
        &self,
    ) -> ListRef<SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_ebs_storage_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultSpaceSettingsElDynamic {
    custom_file_system_config: Option<DynamicBlock<SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigEl>>,
    custom_posix_user_config: Option<DynamicBlock<SagemakerDomainDefaultSpaceSettingsElCustomPosixUserConfigEl>>,
    jupyter_lab_app_settings: Option<DynamicBlock<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsEl>>,
    jupyter_server_app_settings: Option<
        DynamicBlock<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl>,
    >,
    kernel_gateway_app_settings: Option<
        DynamicBlock<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl>,
    >,
    space_storage_settings: Option<DynamicBlock<SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsEl>>,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultSpaceSettingsEl {
    execution_role: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_file_system_config: Option<Vec<SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_posix_user_config: Option<Vec<SagemakerDomainDefaultSpaceSettingsElCustomPosixUserConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jupyter_lab_app_settings: Option<Vec<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jupyter_server_app_settings: Option<Vec<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kernel_gateway_app_settings: Option<Vec<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    space_storage_settings: Option<Vec<SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsEl>>,
    dynamic: SagemakerDomainDefaultSpaceSettingsElDynamic,
}

impl SagemakerDomainDefaultSpaceSettingsEl {
    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_groups = Some(v.into());
        self
    }

    #[doc = "Set the field `custom_file_system_config`.\n"]
    pub fn set_custom_file_system_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_file_system_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_file_system_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_posix_user_config`.\n"]
    pub fn set_custom_posix_user_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultSpaceSettingsElCustomPosixUserConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_posix_user_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_posix_user_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `jupyter_lab_app_settings`.\n"]
    pub fn set_jupyter_lab_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.jupyter_lab_app_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.jupyter_lab_app_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `jupyter_server_app_settings`.\n"]
    pub fn set_jupyter_server_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.jupyter_server_app_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.jupyter_server_app_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kernel_gateway_app_settings`.\n"]
    pub fn set_kernel_gateway_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kernel_gateway_app_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kernel_gateway_app_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `space_storage_settings`.\n"]
    pub fn set_space_storage_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.space_storage_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.space_storage_settings = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultSpaceSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultSpaceSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultSpaceSettingsEl {
    #[doc = ""]
    pub execution_role: PrimField<String>,
}

impl BuildSagemakerDomainDefaultSpaceSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultSpaceSettingsEl {
        SagemakerDomainDefaultSpaceSettingsEl {
            execution_role: self.execution_role,
            security_groups: core::default::Default::default(),
            custom_file_system_config: core::default::Default::default(),
            custom_posix_user_config: core::default::Default::default(),
            jupyter_lab_app_settings: core::default::Default::default(),
            jupyter_server_app_settings: core::default::Default::default(),
            kernel_gateway_app_settings: core::default::Default::default(),
            space_storage_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultSpaceSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultSpaceSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultSpaceSettingsElRef {
        SagemakerDomainDefaultSpaceSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultSpaceSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_role", self.base))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_file_system_config` after provisioning.\n"]
    pub fn custom_file_system_config(&self) -> ListRef<SagemakerDomainDefaultSpaceSettingsElCustomFileSystemConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_file_system_config", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_posix_user_config` after provisioning.\n"]
    pub fn custom_posix_user_config(&self) -> ListRef<SagemakerDomainDefaultSpaceSettingsElCustomPosixUserConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_posix_user_config", self.base))
    }

    #[doc = "Get a reference to the value of field `jupyter_lab_app_settings` after provisioning.\n"]
    pub fn jupyter_lab_app_settings(&self) -> ListRef<SagemakerDomainDefaultSpaceSettingsElJupyterLabAppSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.jupyter_lab_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `jupyter_server_app_settings` after provisioning.\n"]
    pub fn jupyter_server_app_settings(
        &self,
    ) -> ListRef<SagemakerDomainDefaultSpaceSettingsElJupyterServerAppSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.jupyter_server_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `kernel_gateway_app_settings` after provisioning.\n"]
    pub fn kernel_gateway_app_settings(
        &self,
    ) -> ListRef<SagemakerDomainDefaultSpaceSettingsElKernelGatewayAppSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kernel_gateway_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `space_storage_settings` after provisioning.\n"]
    pub fn space_storage_settings(&self) -> ListRef<SagemakerDomainDefaultSpaceSettingsElSpaceStorageSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.space_storage_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElDirectDeploySettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElDirectDeploySettingsEl {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElDirectDeploySettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElDirectDeploySettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElCanvasAppSettingsElDirectDeploySettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElCanvasAppSettingsElDirectDeploySettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElDirectDeploySettingsEl {
        SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElDirectDeploySettingsEl {
            status: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElDirectDeploySettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElDirectDeploySettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElDirectDeploySettingsElRef {
        SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElDirectDeploySettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElDirectDeploySettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElEmrServerlessSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElEmrServerlessSettingsEl {
    #[doc = "Set the field `execution_role_arn`.\n"]
    pub fn set_execution_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.execution_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElEmrServerlessSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElEmrServerlessSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElCanvasAppSettingsElEmrServerlessSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElCanvasAppSettingsElEmrServerlessSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElEmrServerlessSettingsEl {
        SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElEmrServerlessSettingsEl {
            execution_role_arn: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElEmrServerlessSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElEmrServerlessSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElEmrServerlessSettingsElRef {
        SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElEmrServerlessSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElEmrServerlessSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElGenerativeAiSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    amazon_bedrock_role_arn: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElGenerativeAiSettingsEl {
    #[doc = "Set the field `amazon_bedrock_role_arn`.\n"]
    pub fn set_amazon_bedrock_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.amazon_bedrock_role_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElGenerativeAiSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElGenerativeAiSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElCanvasAppSettingsElGenerativeAiSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElCanvasAppSettingsElGenerativeAiSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElGenerativeAiSettingsEl {
        SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElGenerativeAiSettingsEl {
            amazon_bedrock_role_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElGenerativeAiSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElGenerativeAiSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElGenerativeAiSettingsElRef {
        SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElGenerativeAiSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElGenerativeAiSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amazon_bedrock_role_arn` after provisioning.\n"]
    pub fn amazon_bedrock_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.amazon_bedrock_role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_source_name: Option<PrimField<String>>,
    secret_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsEl {
    #[doc = "Set the field `data_source_name`.\n"]
    pub fn set_data_source_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_source_name = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsEl {
    #[doc = ""]
    pub secret_arn: PrimField<String>,
}

impl BuildSagemakerDomainDefaultUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsEl {
        SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsEl {
            data_source_name: core::default::Default::default(),
            secret_arn: self.secret_arn,
            status: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsElRef {
        SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_source_name` after provisioning.\n"]
    pub fn data_source_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_source_name", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElKendraSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElKendraSettingsEl {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElKendraSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElKendraSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElCanvasAppSettingsElKendraSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElCanvasAppSettingsElKendraSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElKendraSettingsEl {
        SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElKendraSettingsEl {
            status: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElKendraSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElKendraSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElKendraSettingsElRef {
        SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElKendraSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElKendraSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElModelRegisterSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cross_account_model_register_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElModelRegisterSettingsEl {
    #[doc = "Set the field `cross_account_model_register_role_arn`.\n"]
    pub fn set_cross_account_model_register_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cross_account_model_register_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElModelRegisterSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElModelRegisterSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElCanvasAppSettingsElModelRegisterSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElCanvasAppSettingsElModelRegisterSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElModelRegisterSettingsEl {
        SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElModelRegisterSettingsEl {
            cross_account_model_register_role_arn: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElModelRegisterSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElModelRegisterSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElModelRegisterSettingsElRef {
        SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElModelRegisterSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElModelRegisterSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cross_account_model_register_role_arn` after provisioning.\n"]
    pub fn cross_account_model_register_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cross_account_model_register_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    amazon_forecast_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
    #[doc = "Set the field `amazon_forecast_role_arn`.\n"]
    pub fn set_amazon_forecast_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.amazon_forecast_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
        SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
            amazon_forecast_role_arn: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
        SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amazon_forecast_role_arn` after provisioning.\n"]
    pub fn amazon_forecast_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.amazon_forecast_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElWorkspaceSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_artifact_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_kms_key_id: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElWorkspaceSettingsEl {
    #[doc = "Set the field `s3_artifact_path`.\n"]
    pub fn set_s3_artifact_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_artifact_path = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_kms_key_id`.\n"]
    pub fn set_s3_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_kms_key_id = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElWorkspaceSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElWorkspaceSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElCanvasAppSettingsElWorkspaceSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElCanvasAppSettingsElWorkspaceSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElWorkspaceSettingsEl {
        SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElWorkspaceSettingsEl {
            s3_artifact_path: core::default::Default::default(),
            s3_kms_key_id: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElWorkspaceSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElWorkspaceSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElWorkspaceSettingsElRef {
        SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElWorkspaceSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElWorkspaceSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_artifact_path` after provisioning.\n"]
    pub fn s3_artifact_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_artifact_path", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_kms_key_id` after provisioning.\n"]
    pub fn s3_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_kms_key_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElDynamic {
    direct_deploy_settings: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElDirectDeploySettingsEl>,
    >,
    emr_serverless_settings: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElEmrServerlessSettingsEl>,
    >,
    generative_ai_settings: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElGenerativeAiSettingsEl>,
    >,
    identity_provider_oauth_settings: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsEl>,
    >,
    kendra_settings: Option<DynamicBlock<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElKendraSettingsEl>>,
    model_register_settings: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElModelRegisterSettingsEl>,
    >,
    time_series_forecasting_settings: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl>,
    >,
    workspace_settings: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElWorkspaceSettingsEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_deploy_settings: Option<Vec<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElDirectDeploySettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emr_serverless_settings: Option<Vec<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElEmrServerlessSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generative_ai_settings: Option<Vec<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElGenerativeAiSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_oauth_settings: Option<
        Vec<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    kendra_settings: Option<Vec<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElKendraSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_register_settings: Option<Vec<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElModelRegisterSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_series_forecasting_settings: Option<
        Vec<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    workspace_settings: Option<Vec<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElWorkspaceSettingsEl>>,
    dynamic: SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElDynamic,
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl {
    #[doc = "Set the field `direct_deploy_settings`.\n"]
    pub fn set_direct_deploy_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElDirectDeploySettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.direct_deploy_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.direct_deploy_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `emr_serverless_settings`.\n"]
    pub fn set_emr_serverless_settings(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElEmrServerlessSettingsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.emr_serverless_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.emr_serverless_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `generative_ai_settings`.\n"]
    pub fn set_generative_ai_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElGenerativeAiSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.generative_ai_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.generative_ai_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `identity_provider_oauth_settings`.\n"]
    pub fn set_identity_provider_oauth_settings(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.identity_provider_oauth_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.identity_provider_oauth_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kendra_settings`.\n"]
    pub fn set_kendra_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElKendraSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kendra_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kendra_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `model_register_settings`.\n"]
    pub fn set_model_register_settings(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElModelRegisterSettingsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.model_register_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.model_register_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `time_series_forecasting_settings`.\n"]
    pub fn set_time_series_forecasting_settings(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.time_series_forecasting_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.time_series_forecasting_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workspace_settings`.\n"]
    pub fn set_workspace_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElWorkspaceSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.workspace_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.workspace_settings = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl {
        SagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl {
            direct_deploy_settings: core::default::Default::default(),
            emr_serverless_settings: core::default::Default::default(),
            generative_ai_settings: core::default::Default::default(),
            identity_provider_oauth_settings: core::default::Default::default(),
            kendra_settings: core::default::Default::default(),
            model_register_settings: core::default::Default::default(),
            time_series_forecasting_settings: core::default::Default::default(),
            workspace_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElRef {
        SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `direct_deploy_settings` after provisioning.\n"]
    pub fn direct_deploy_settings(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElDirectDeploySettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.direct_deploy_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `emr_serverless_settings` after provisioning.\n"]
    pub fn emr_serverless_settings(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElEmrServerlessSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.emr_serverless_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `generative_ai_settings` after provisioning.\n"]
    pub fn generative_ai_settings(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElGenerativeAiSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.generative_ai_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_provider_oauth_settings` after provisioning.\n"]
    pub fn identity_provider_oauth_settings(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.identity_provider_oauth_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `kendra_settings` after provisioning.\n"]
    pub fn kendra_settings(&self) -> ListRef<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElKendraSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kendra_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `model_register_settings` after provisioning.\n"]
    pub fn model_register_settings(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElModelRegisterSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.model_register_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `time_series_forecasting_settings` after provisioning.\n"]
    pub fn time_series_forecasting_settings(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.time_series_forecasting_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `workspace_settings` after provisioning.\n"]
    pub fn workspace_settings(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElWorkspaceSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.workspace_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_timeout_in_minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_management: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_idle_timeout_in_minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_idle_timeout_in_minutes: Option<PrimField<f64>>,
}

impl SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl {
    #[doc = "Set the field `idle_timeout_in_minutes`.\n"]
    pub fn set_idle_timeout_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.idle_timeout_in_minutes = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_management`.\n"]
    pub fn set_lifecycle_management(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_management = Some(v.into());
        self
    }

    #[doc = "Set the field `max_idle_timeout_in_minutes`.\n"]
    pub fn set_max_idle_timeout_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_idle_timeout_in_minutes = Some(v.into());
        self
    }

    #[doc = "Set the field `min_idle_timeout_in_minutes`.\n"]
    pub fn set_min_idle_timeout_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_idle_timeout_in_minutes = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl {
    type O =
        BlockAssignable<
            SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl {
    pub fn build(
        self,
    ) -> SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl {
        SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl {
            idle_timeout_in_minutes: core::default::Default::default(),
            lifecycle_management: core::default::Default::default(),
            max_idle_timeout_in_minutes: core::default::Default::default(),
            min_idle_timeout_in_minutes: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsElRef {
        SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle_timeout_in_minutes` after provisioning.\n"]
    pub fn idle_timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_management` after provisioning.\n"]
    pub fn lifecycle_management(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_management", self.base))
    }

    #[doc = "Get a reference to the value of field `max_idle_timeout_in_minutes` after provisioning.\n"]
    pub fn max_idle_timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_idle_timeout_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `min_idle_timeout_in_minutes` after provisioning.\n"]
    pub fn min_idle_timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_idle_timeout_in_minutes", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElDynamic {
    idle_settings: Option<
        DynamicBlock<
            SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_settings: Option<
        Vec<SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl>,
    >,
    dynamic: SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElDynamic,
}

impl SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl {
    #[doc = "Set the field `idle_settings`.\n"]
    pub fn set_idle_settings(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.idle_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.idle_settings = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl {}

impl BuildSagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl {
        SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl {
            idle_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElRef {
        SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle_settings` after provisioning.\n"]
    pub fn idle_settings(
        &self,
    ) -> ListRef<
        SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.idle_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElCustomImageEl {
    app_image_config_name: PrimField<String>,
    image_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_version_number: Option<PrimField<f64>>,
}

impl SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElCustomImageEl {
    #[doc = "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.image_version_number = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElCustomImageEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElCustomImageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElCustomImageEl {
    #[doc = ""]
    pub app_image_config_name: PrimField<String>,
    #[doc = ""]
    pub image_name: PrimField<String>,
}

impl BuildSagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElCustomImageEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElCustomImageEl {
        SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElCustomImageEl {
            app_image_config_name: self.app_image_config_name,
            image_name: self.image_name,
            image_version_number: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElCustomImageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElCustomImageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElCustomImageElRef {
        SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElCustomImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElCustomImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_image_config_name", self.base))
    }

    #[doc = "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_name", self.base))
    }

    #[doc = "Get a reference to the value of field `image_version_number` after provisioning.\n"]
    pub fn image_version_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_version_number", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_alias`.\n"]
    pub fn set_sagemaker_image_version_alias(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_alias = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl {
        SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_alias: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecElRef {
        SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_alias` after provisioning.\n"]
    pub fn sagemaker_image_version_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_alias", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElDynamic {
    app_lifecycle_management: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl>,
    >,
    custom_image: Option<DynamicBlock<SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElCustomImageEl>>,
    default_resource_spec: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    built_in_lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_lifecycle_management: Option<
        Vec<SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_image: Option<Vec<SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElCustomImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec: Option<Vec<SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElDynamic,
}

impl SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsEl {
    #[doc = "Set the field `built_in_lifecycle_config_arn`.\n"]
    pub fn set_built_in_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.built_in_lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.lifecycle_config_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `app_lifecycle_management`.\n"]
    pub fn set_app_lifecycle_management(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.app_lifecycle_management = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.app_lifecycle_management = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElCustomImageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_image = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_image = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsEl {
        SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsEl {
            built_in_lifecycle_config_arn: core::default::Default::default(),
            lifecycle_config_arns: core::default::Default::default(),
            app_lifecycle_management: core::default::Default::default(),
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElRef {
        SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `built_in_lifecycle_config_arn` after provisioning.\n"]
    pub fn built_in_lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.built_in_lifecycle_config_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `app_lifecycle_management` after provisioning.\n"]
    pub fn app_lifecycle_management(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElRef> {
        ListRef::new(self.shared().clone(), format!("{}.app_lifecycle_management", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(&self) -> ListRef<SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElCustomImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl {
    file_system_id: PrimField<String>,
    file_system_path: PrimField<String>,
}

impl SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl { }

impl ToListMappable for SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl {
    #[doc = ""]
    pub file_system_id: PrimField<String>,
    #[doc = ""]
    pub file_system_path: PrimField<String>,
}

impl BuildSagemakerDomainDefaultUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl {
        SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl {
            file_system_id: self.file_system_id,
            file_system_path: self.file_system_path,
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigElRef {
        SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_path` after provisioning.\n"]
    pub fn file_system_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_system_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigElDynamic {
    efs_file_system_config: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    efs_file_system_config: Option<
        Vec<SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl>,
    >,
    dynamic: SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigElDynamic,
}

impl SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigEl {
    #[doc = "Set the field `efs_file_system_config`.\n"]
    pub fn set_efs_file_system_config(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.efs_file_system_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.efs_file_system_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElCustomFileSystemConfigEl {}

impl BuildSagemakerDomainDefaultUserSettingsElCustomFileSystemConfigEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigEl {
        SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigEl {
            efs_file_system_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigElRef {
        SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `efs_file_system_config` after provisioning.\n"]
    pub fn efs_file_system_config(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.efs_file_system_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElCustomPosixUserConfigEl {
    gid: PrimField<f64>,
    uid: PrimField<f64>,
}

impl SagemakerDomainDefaultUserSettingsElCustomPosixUserConfigEl { }

impl ToListMappable for SagemakerDomainDefaultUserSettingsElCustomPosixUserConfigEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElCustomPosixUserConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElCustomPosixUserConfigEl {
    #[doc = ""]
    pub gid: PrimField<f64>,
    #[doc = ""]
    pub uid: PrimField<f64>,
}

impl BuildSagemakerDomainDefaultUserSettingsElCustomPosixUserConfigEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElCustomPosixUserConfigEl {
        SagemakerDomainDefaultUserSettingsElCustomPosixUserConfigEl {
            gid: self.gid,
            uid: self.uid,
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElCustomPosixUserConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElCustomPosixUserConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultUserSettingsElCustomPosixUserConfigElRef {
        SagemakerDomainDefaultUserSettingsElCustomPosixUserConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElCustomPosixUserConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gid` after provisioning.\n"]
    pub fn gid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.gid", self.base))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_timeout_in_minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_management: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_idle_timeout_in_minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_idle_timeout_in_minutes: Option<PrimField<f64>>,
}

impl SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl {
    #[doc = "Set the field `idle_timeout_in_minutes`.\n"]
    pub fn set_idle_timeout_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.idle_timeout_in_minutes = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_management`.\n"]
    pub fn set_lifecycle_management(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_management = Some(v.into());
        self
    }

    #[doc = "Set the field `max_idle_timeout_in_minutes`.\n"]
    pub fn set_max_idle_timeout_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_idle_timeout_in_minutes = Some(v.into());
        self
    }

    #[doc = "Set the field `min_idle_timeout_in_minutes`.\n"]
    pub fn set_min_idle_timeout_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_idle_timeout_in_minutes = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl {
    type O =
        BlockAssignable<
            SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl {
    pub fn build(
        self,
    ) -> SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl {
        SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl {
            idle_timeout_in_minutes: core::default::Default::default(),
            lifecycle_management: core::default::Default::default(),
            max_idle_timeout_in_minutes: core::default::Default::default(),
            min_idle_timeout_in_minutes: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsElRef {
        SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle_timeout_in_minutes` after provisioning.\n"]
    pub fn idle_timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_management` after provisioning.\n"]
    pub fn lifecycle_management(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_management", self.base))
    }

    #[doc = "Get a reference to the value of field `max_idle_timeout_in_minutes` after provisioning.\n"]
    pub fn max_idle_timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_idle_timeout_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `min_idle_timeout_in_minutes` after provisioning.\n"]
    pub fn min_idle_timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_idle_timeout_in_minutes", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElDynamic {
    idle_settings: Option<
        DynamicBlock<
            SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_settings: Option<
        Vec<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl>,
    >,
    dynamic: SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElDynamic,
}

impl SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl {
    #[doc = "Set the field `idle_settings`.\n"]
    pub fn set_idle_settings(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.idle_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.idle_settings = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl {}

impl BuildSagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl {
        SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl {
            idle_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElRef {
        SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle_settings` after provisioning.\n"]
    pub fn idle_settings(
        &self,
    ) -> ListRef<
        SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.idle_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCodeRepositoryEl {
    repository_url: PrimField<String>,
}

impl SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCodeRepositoryEl { }

impl ToListMappable for SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCodeRepositoryEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCodeRepositoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCodeRepositoryEl {
    #[doc = ""]
    pub repository_url: PrimField<String>,
}

impl BuildSagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCodeRepositoryEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCodeRepositoryEl {
        SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCodeRepositoryEl {
            repository_url: self.repository_url,
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCodeRepositoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCodeRepositoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCodeRepositoryElRef {
        SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCodeRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCodeRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_url", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCustomImageEl {
    app_image_config_name: PrimField<String>,
    image_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_version_number: Option<PrimField<f64>>,
}

impl SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCustomImageEl {
    #[doc = "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.image_version_number = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCustomImageEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCustomImageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCustomImageEl {
    #[doc = ""]
    pub app_image_config_name: PrimField<String>,
    #[doc = ""]
    pub image_name: PrimField<String>,
}

impl BuildSagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCustomImageEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCustomImageEl {
        SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCustomImageEl {
            app_image_config_name: self.app_image_config_name,
            image_name: self.image_name,
            image_version_number: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCustomImageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCustomImageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCustomImageElRef {
        SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCustomImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCustomImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_image_config_name", self.base))
    }

    #[doc = "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_name", self.base))
    }

    #[doc = "Get a reference to the value of field `image_version_number` after provisioning.\n"]
    pub fn image_version_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_version_number", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_alias`.\n"]
    pub fn set_sagemaker_image_version_alias(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_alias = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {
        SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_alias: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecElRef {
        SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_alias` after provisioning.\n"]
    pub fn sagemaker_image_version_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_alias", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElEmrSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    assumable_role_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_role_arns: Option<SetField<PrimField<String>>>,
}

impl SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElEmrSettingsEl {
    #[doc = "Set the field `assumable_role_arns`.\n"]
    pub fn set_assumable_role_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.assumable_role_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `execution_role_arns`.\n"]
    pub fn set_execution_role_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.execution_role_arns = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElEmrSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElEmrSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElEmrSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElEmrSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElEmrSettingsEl {
        SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElEmrSettingsEl {
            assumable_role_arns: core::default::Default::default(),
            execution_role_arns: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElEmrSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElEmrSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElEmrSettingsElRef {
        SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElEmrSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElEmrSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `assumable_role_arns` after provisioning.\n"]
    pub fn assumable_role_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.assumable_role_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_role_arns` after provisioning.\n"]
    pub fn execution_role_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.execution_role_arns", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElDynamic {
    app_lifecycle_management: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl>,
    >,
    code_repository: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCodeRepositoryEl>,
    >,
    custom_image: Option<DynamicBlock<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCustomImageEl>>,
    default_resource_spec: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl>,
    >,
    emr_settings: Option<DynamicBlock<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElEmrSettingsEl>>,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    built_in_lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_lifecycle_management: Option<
        Vec<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_repository: Option<Vec<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCodeRepositoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_image: Option<Vec<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCustomImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec: Option<Vec<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emr_settings: Option<Vec<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElEmrSettingsEl>>,
    dynamic: SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElDynamic,
}

impl SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsEl {
    #[doc = "Set the field `built_in_lifecycle_config_arn`.\n"]
    pub fn set_built_in_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.built_in_lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.lifecycle_config_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `app_lifecycle_management`.\n"]
    pub fn set_app_lifecycle_management(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.app_lifecycle_management = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.app_lifecycle_management = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `code_repository`.\n"]
    pub fn set_code_repository(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCodeRepositoryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_repository = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_repository = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCustomImageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_image = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_image = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `emr_settings`.\n"]
    pub fn set_emr_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElEmrSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.emr_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.emr_settings = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsEl {
        SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsEl {
            built_in_lifecycle_config_arn: core::default::Default::default(),
            lifecycle_config_arns: core::default::Default::default(),
            app_lifecycle_management: core::default::Default::default(),
            code_repository: core::default::Default::default(),
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            emr_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElRef {
        SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `built_in_lifecycle_config_arn` after provisioning.\n"]
    pub fn built_in_lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.built_in_lifecycle_config_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `app_lifecycle_management` after provisioning.\n"]
    pub fn app_lifecycle_management(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElRef> {
        ListRef::new(self.shared().clone(), format!("{}.app_lifecycle_management", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(&self) -> ListRef<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElCustomImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `emr_settings` after provisioning.\n"]
    pub fn emr_settings(&self) -> ListRef<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElEmrSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.emr_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    repository_url: PrimField<String>,
}

impl SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl { }

impl ToListMappable for SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    #[doc = ""]
    pub repository_url: PrimField<String>,
}

impl BuildSagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
        SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
            repository_url: self.repository_url,
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
        SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_url", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_alias`.\n"]
    pub fn set_sagemaker_image_version_alias(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_alias = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
        SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_alias: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
        SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_alias` after provisioning.\n"]
    pub fn sagemaker_image_version_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_alias", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDynamic {
    code_repository: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl>,
    >,
    default_resource_spec: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_repository: Option<Vec<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec: Option<
        Vec<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>,
    >,
    dynamic: SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDynamic,
}

impl SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl {
    #[doc = "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.lifecycle_config_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `code_repository`.\n"]
    pub fn set_code_repository(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_repository = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_repository = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl {
        SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl {
            lifecycle_config_arns: core::default::Default::default(),
            code_repository: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElRef {
        SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    app_image_config_name: PrimField<String>,
    image_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_version_number: Option<PrimField<f64>>,
}

impl SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    #[doc = "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.image_version_number = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    #[doc = ""]
    pub app_image_config_name: PrimField<String>,
    #[doc = ""]
    pub image_name: PrimField<String>,
}

impl BuildSagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
        SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
            app_image_config_name: self.app_image_config_name,
            image_name: self.image_name,
            image_version_number: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
        SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_image_config_name", self.base))
    }

    #[doc = "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_name", self.base))
    }

    #[doc = "Get a reference to the value of field `image_version_number` after provisioning.\n"]
    pub fn image_version_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_version_number", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_alias`.\n"]
    pub fn set_sagemaker_image_version_alias(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_alias = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
        SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_alias: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
        SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_alias` after provisioning.\n"]
    pub fn sagemaker_image_version_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_alias", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDynamic {
    custom_image: Option<DynamicBlock<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    default_resource_spec: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_image: Option<Vec<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec: Option<
        Vec<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>,
    >,
    dynamic: SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDynamic,
}

impl SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl {
    #[doc = "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.lifecycle_config_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_image = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_image = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl {
        SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl {
            lifecycle_config_arns: core::default::Default::default(),
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElRef {
        SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.lifecycle_config_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElCustomImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl {
    app_image_config_name: PrimField<String>,
    image_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_version_number: Option<PrimField<f64>>,
}

impl SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl {
    #[doc = "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.image_version_number = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl {
    #[doc = ""]
    pub app_image_config_name: PrimField<String>,
    #[doc = ""]
    pub image_name: PrimField<String>,
}

impl BuildSagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl {
        SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl {
            app_image_config_name: self.app_image_config_name,
            image_name: self.image_name,
            image_version_number: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageElRef {
        SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_image_config_name", self.base))
    }

    #[doc = "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_name", self.base))
    }

    #[doc = "Get a reference to the value of field `image_version_number` after provisioning.\n"]
    pub fn image_version_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_version_number", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_alias`.\n"]
    pub fn set_sagemaker_image_version_alias(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_alias = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
        SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_alias: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
        SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_alias` after provisioning.\n"]
    pub fn sagemaker_image_version_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_alias", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDynamic {
    custom_image: Option<DynamicBlock<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl>>,
    default_resource_spec: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_image: Option<Vec<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec: Option<Vec<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDynamic,
}

impl SagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl {
    #[doc = "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_image = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_image = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl {
        SagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl {
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElRef {
        SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(&self) -> ListRef<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElCustomImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElRStudioServerProAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_group: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultUserSettingsElRStudioServerProAppSettingsEl {
    #[doc = "Set the field `access_status`.\n"]
    pub fn set_access_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_status = Some(v.into());
        self
    }

    #[doc = "Set the field `user_group`.\n"]
    pub fn set_user_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_group = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElRStudioServerProAppSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElRStudioServerProAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElRStudioServerProAppSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElRStudioServerProAppSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElRStudioServerProAppSettingsEl {
        SagemakerDomainDefaultUserSettingsElRStudioServerProAppSettingsEl {
            access_status: core::default::Default::default(),
            user_group: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElRStudioServerProAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElRStudioServerProAppSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElRStudioServerProAppSettingsElRef {
        SagemakerDomainDefaultUserSettingsElRStudioServerProAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElRStudioServerProAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_status` after provisioning.\n"]
    pub fn access_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_status", self.base))
    }

    #[doc = "Get a reference to the value of field `user_group` after provisioning.\n"]
    pub fn user_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_group", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElSharingSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    notebook_output_option: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_output_path: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultUserSettingsElSharingSettingsEl {
    #[doc = "Set the field `notebook_output_option`.\n"]
    pub fn set_notebook_output_option(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.notebook_output_option = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_kms_key_id`.\n"]
    pub fn set_s3_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_output_path`.\n"]
    pub fn set_s3_output_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_output_path = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElSharingSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElSharingSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElSharingSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElSharingSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElSharingSettingsEl {
        SagemakerDomainDefaultUserSettingsElSharingSettingsEl {
            notebook_output_option: core::default::Default::default(),
            s3_kms_key_id: core::default::Default::default(),
            s3_output_path: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElSharingSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElSharingSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultUserSettingsElSharingSettingsElRef {
        SagemakerDomainDefaultUserSettingsElSharingSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElSharingSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `notebook_output_option` after provisioning.\n"]
    pub fn notebook_output_option(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.notebook_output_option", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_kms_key_id` after provisioning.\n"]
    pub fn s3_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_output_path` after provisioning.\n"]
    pub fn s3_output_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_output_path", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl {
    default_ebs_volume_size_in_gb: PrimField<f64>,
    maximum_ebs_volume_size_in_gb: PrimField<f64>,
}

impl SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl { }

impl ToListMappable for SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl {
    #[doc = ""]
    pub default_ebs_volume_size_in_gb: PrimField<f64>,
    #[doc = ""]
    pub maximum_ebs_volume_size_in_gb: PrimField<f64>,
}

impl BuildSagemakerDomainDefaultUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl {
        SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl {
            default_ebs_volume_size_in_gb: self.default_ebs_volume_size_in_gb,
            maximum_ebs_volume_size_in_gb: self.maximum_ebs_volume_size_in_gb,
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsElRef {
        SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_ebs_volume_size_in_gb` after provisioning.\n"]
    pub fn default_ebs_volume_size_in_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_ebs_volume_size_in_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_ebs_volume_size_in_gb` after provisioning.\n"]
    pub fn maximum_ebs_volume_size_in_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_ebs_volume_size_in_gb", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsElDynamic {
    default_ebs_storage_settings: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_ebs_storage_settings: Option<
        Vec<SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl>,
    >,
    dynamic: SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsElDynamic,
}

impl SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsEl {
    #[doc = "Set the field `default_ebs_storage_settings`.\n"]
    pub fn set_default_ebs_storage_settings(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_ebs_storage_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_ebs_storage_settings = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElSpaceStorageSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElSpaceStorageSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsEl {
        SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsEl {
            default_ebs_storage_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsElRef {
        SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_ebs_storage_settings` after provisioning.\n"]
    pub fn default_ebs_storage_settings(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_ebs_storage_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElStudioWebPortalSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hidden_app_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hidden_instance_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hidden_ml_tools: Option<SetField<PrimField<String>>>,
}

impl SagemakerDomainDefaultUserSettingsElStudioWebPortalSettingsEl {
    #[doc = "Set the field `hidden_app_types`.\n"]
    pub fn set_hidden_app_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.hidden_app_types = Some(v.into());
        self
    }

    #[doc = "Set the field `hidden_instance_types`.\n"]
    pub fn set_hidden_instance_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.hidden_instance_types = Some(v.into());
        self
    }

    #[doc = "Set the field `hidden_ml_tools`.\n"]
    pub fn set_hidden_ml_tools(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.hidden_ml_tools = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElStudioWebPortalSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElStudioWebPortalSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElStudioWebPortalSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElStudioWebPortalSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElStudioWebPortalSettingsEl {
        SagemakerDomainDefaultUserSettingsElStudioWebPortalSettingsEl {
            hidden_app_types: core::default::Default::default(),
            hidden_instance_types: core::default::Default::default(),
            hidden_ml_tools: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElStudioWebPortalSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElStudioWebPortalSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultUserSettingsElStudioWebPortalSettingsElRef {
        SagemakerDomainDefaultUserSettingsElStudioWebPortalSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElStudioWebPortalSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hidden_app_types` after provisioning.\n"]
    pub fn hidden_app_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.hidden_app_types", self.base))
    }

    #[doc = "Get a reference to the value of field `hidden_instance_types` after provisioning.\n"]
    pub fn hidden_instance_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.hidden_instance_types", self.base))
    }

    #[doc = "Get a reference to the value of field `hidden_ml_tools` after provisioning.\n"]
    pub fn hidden_ml_tools(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.hidden_ml_tools", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_alias`.\n"]
    pub fn set_sagemaker_image_version_alias(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_alias = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
        SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_alias: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
        SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_alias` after provisioning.\n"]
    pub fn sagemaker_image_version_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_alias", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDynamic {
    default_resource_spec: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec: Option<Vec<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDynamic,
}

impl SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl {
    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl {}

impl BuildSagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl {
        SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl {
            default_resource_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElRef {
        SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDefaultUserSettingsElDynamic {
    canvas_app_settings: Option<DynamicBlock<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl>>,
    code_editor_app_settings: Option<DynamicBlock<SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsEl>>,
    custom_file_system_config: Option<DynamicBlock<SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigEl>>,
    custom_posix_user_config: Option<DynamicBlock<SagemakerDomainDefaultUserSettingsElCustomPosixUserConfigEl>>,
    jupyter_lab_app_settings: Option<DynamicBlock<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsEl>>,
    jupyter_server_app_settings: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl>,
    >,
    kernel_gateway_app_settings: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl>,
    >,
    r_session_app_settings: Option<DynamicBlock<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl>>,
    r_studio_server_pro_app_settings: Option<
        DynamicBlock<SagemakerDomainDefaultUserSettingsElRStudioServerProAppSettingsEl>,
    >,
    sharing_settings: Option<DynamicBlock<SagemakerDomainDefaultUserSettingsElSharingSettingsEl>>,
    space_storage_settings: Option<DynamicBlock<SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsEl>>,
    studio_web_portal_settings: Option<DynamicBlock<SagemakerDomainDefaultUserSettingsElStudioWebPortalSettingsEl>>,
    tensor_board_app_settings: Option<DynamicBlock<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl>>,
}

#[derive(Serialize)]
pub struct SagemakerDomainDefaultUserSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_mount_home_efs: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_landing_uri: Option<PrimField<String>>,
    execution_role: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    studio_web_portal: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    canvas_app_settings: Option<Vec<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_editor_app_settings: Option<Vec<SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_file_system_config: Option<Vec<SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_posix_user_config: Option<Vec<SagemakerDomainDefaultUserSettingsElCustomPosixUserConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jupyter_lab_app_settings: Option<Vec<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jupyter_server_app_settings: Option<Vec<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kernel_gateway_app_settings: Option<Vec<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r_session_app_settings: Option<Vec<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r_studio_server_pro_app_settings: Option<Vec<SagemakerDomainDefaultUserSettingsElRStudioServerProAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sharing_settings: Option<Vec<SagemakerDomainDefaultUserSettingsElSharingSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    space_storage_settings: Option<Vec<SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    studio_web_portal_settings: Option<Vec<SagemakerDomainDefaultUserSettingsElStudioWebPortalSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tensor_board_app_settings: Option<Vec<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl>>,
    dynamic: SagemakerDomainDefaultUserSettingsElDynamic,
}

impl SagemakerDomainDefaultUserSettingsEl {
    #[doc = "Set the field `auto_mount_home_efs`.\n"]
    pub fn set_auto_mount_home_efs(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auto_mount_home_efs = Some(v.into());
        self
    }

    #[doc = "Set the field `default_landing_uri`.\n"]
    pub fn set_default_landing_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_landing_uri = Some(v.into());
        self
    }

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_groups = Some(v.into());
        self
    }

    #[doc = "Set the field `studio_web_portal`.\n"]
    pub fn set_studio_web_portal(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.studio_web_portal = Some(v.into());
        self
    }

    #[doc = "Set the field `canvas_app_settings`.\n"]
    pub fn set_canvas_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.canvas_app_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.canvas_app_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `code_editor_app_settings`.\n"]
    pub fn set_code_editor_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_editor_app_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_editor_app_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_file_system_config`.\n"]
    pub fn set_custom_file_system_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_file_system_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_file_system_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_posix_user_config`.\n"]
    pub fn set_custom_posix_user_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElCustomPosixUserConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_posix_user_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_posix_user_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `jupyter_lab_app_settings`.\n"]
    pub fn set_jupyter_lab_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.jupyter_lab_app_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.jupyter_lab_app_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `jupyter_server_app_settings`.\n"]
    pub fn set_jupyter_server_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.jupyter_server_app_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.jupyter_server_app_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kernel_gateway_app_settings`.\n"]
    pub fn set_kernel_gateway_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kernel_gateway_app_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kernel_gateway_app_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `r_session_app_settings`.\n"]
    pub fn set_r_session_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.r_session_app_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.r_session_app_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `r_studio_server_pro_app_settings`.\n"]
    pub fn set_r_studio_server_pro_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElRStudioServerProAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.r_studio_server_pro_app_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.r_studio_server_pro_app_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sharing_settings`.\n"]
    pub fn set_sharing_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElSharingSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sharing_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sharing_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `space_storage_settings`.\n"]
    pub fn set_space_storage_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.space_storage_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.space_storage_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `studio_web_portal_settings`.\n"]
    pub fn set_studio_web_portal_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElStudioWebPortalSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.studio_web_portal_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.studio_web_portal_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tensor_board_app_settings`.\n"]
    pub fn set_tensor_board_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tensor_board_app_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tensor_board_app_settings = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDefaultUserSettingsEl {
    type O = BlockAssignable<SagemakerDomainDefaultUserSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDefaultUserSettingsEl {
    #[doc = ""]
    pub execution_role: PrimField<String>,
}

impl BuildSagemakerDomainDefaultUserSettingsEl {
    pub fn build(self) -> SagemakerDomainDefaultUserSettingsEl {
        SagemakerDomainDefaultUserSettingsEl {
            auto_mount_home_efs: core::default::Default::default(),
            default_landing_uri: core::default::Default::default(),
            execution_role: self.execution_role,
            security_groups: core::default::Default::default(),
            studio_web_portal: core::default::Default::default(),
            canvas_app_settings: core::default::Default::default(),
            code_editor_app_settings: core::default::Default::default(),
            custom_file_system_config: core::default::Default::default(),
            custom_posix_user_config: core::default::Default::default(),
            jupyter_lab_app_settings: core::default::Default::default(),
            jupyter_server_app_settings: core::default::Default::default(),
            kernel_gateway_app_settings: core::default::Default::default(),
            r_session_app_settings: core::default::Default::default(),
            r_studio_server_pro_app_settings: core::default::Default::default(),
            sharing_settings: core::default::Default::default(),
            space_storage_settings: core::default::Default::default(),
            studio_web_portal_settings: core::default::Default::default(),
            tensor_board_app_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDefaultUserSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDefaultUserSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDefaultUserSettingsElRef {
        SagemakerDomainDefaultUserSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDefaultUserSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_mount_home_efs` after provisioning.\n"]
    pub fn auto_mount_home_efs(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_mount_home_efs", self.base))
    }

    #[doc = "Get a reference to the value of field `default_landing_uri` after provisioning.\n"]
    pub fn default_landing_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_landing_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_role", self.base))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `studio_web_portal` after provisioning.\n"]
    pub fn studio_web_portal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.studio_web_portal", self.base))
    }

    #[doc = "Get a reference to the value of field `canvas_app_settings` after provisioning.\n"]
    pub fn canvas_app_settings(&self) -> ListRef<SagemakerDomainDefaultUserSettingsElCanvasAppSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.canvas_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `code_editor_app_settings` after provisioning.\n"]
    pub fn code_editor_app_settings(&self) -> ListRef<SagemakerDomainDefaultUserSettingsElCodeEditorAppSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.code_editor_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_file_system_config` after provisioning.\n"]
    pub fn custom_file_system_config(&self) -> ListRef<SagemakerDomainDefaultUserSettingsElCustomFileSystemConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_file_system_config", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_posix_user_config` after provisioning.\n"]
    pub fn custom_posix_user_config(&self) -> ListRef<SagemakerDomainDefaultUserSettingsElCustomPosixUserConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_posix_user_config", self.base))
    }

    #[doc = "Get a reference to the value of field `jupyter_lab_app_settings` after provisioning.\n"]
    pub fn jupyter_lab_app_settings(&self) -> ListRef<SagemakerDomainDefaultUserSettingsElJupyterLabAppSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.jupyter_lab_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `jupyter_server_app_settings` after provisioning.\n"]
    pub fn jupyter_server_app_settings(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElJupyterServerAppSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.jupyter_server_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `kernel_gateway_app_settings` after provisioning.\n"]
    pub fn kernel_gateway_app_settings(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElKernelGatewayAppSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kernel_gateway_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `r_session_app_settings` after provisioning.\n"]
    pub fn r_session_app_settings(&self) -> ListRef<SagemakerDomainDefaultUserSettingsElRSessionAppSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.r_session_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `r_studio_server_pro_app_settings` after provisioning.\n"]
    pub fn r_studio_server_pro_app_settings(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElRStudioServerProAppSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.r_studio_server_pro_app_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `sharing_settings` after provisioning.\n"]
    pub fn sharing_settings(&self) -> ListRef<SagemakerDomainDefaultUserSettingsElSharingSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sharing_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `space_storage_settings` after provisioning.\n"]
    pub fn space_storage_settings(&self) -> ListRef<SagemakerDomainDefaultUserSettingsElSpaceStorageSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.space_storage_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `studio_web_portal_settings` after provisioning.\n"]
    pub fn studio_web_portal_settings(
        &self,
    ) -> ListRef<SagemakerDomainDefaultUserSettingsElStudioWebPortalSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.studio_web_portal_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `tensor_board_app_settings` after provisioning.\n"]
    pub fn tensor_board_app_settings(&self) -> ListRef<SagemakerDomainDefaultUserSettingsElTensorBoardAppSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tensor_board_app_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDomainSettingsElDockerSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_docker_access: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_only_trusted_accounts: Option<SetField<PrimField<String>>>,
}

impl SagemakerDomainDomainSettingsElDockerSettingsEl {
    #[doc = "Set the field `enable_docker_access`.\n"]
    pub fn set_enable_docker_access(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enable_docker_access = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_only_trusted_accounts`.\n"]
    pub fn set_vpc_only_trusted_accounts(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.vpc_only_trusted_accounts = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDomainSettingsElDockerSettingsEl {
    type O = BlockAssignable<SagemakerDomainDomainSettingsElDockerSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDomainSettingsElDockerSettingsEl {}

impl BuildSagemakerDomainDomainSettingsElDockerSettingsEl {
    pub fn build(self) -> SagemakerDomainDomainSettingsElDockerSettingsEl {
        SagemakerDomainDomainSettingsElDockerSettingsEl {
            enable_docker_access: core::default::Default::default(),
            vpc_only_trusted_accounts: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDomainSettingsElDockerSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDomainSettingsElDockerSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDomainSettingsElDockerSettingsElRef {
        SagemakerDomainDomainSettingsElDockerSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDomainSettingsElDockerSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_docker_access` after provisioning.\n"]
    pub fn enable_docker_access(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_docker_access", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_only_trusted_accounts` after provisioning.\n"]
    pub fn vpc_only_trusted_accounts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.vpc_only_trusted_accounts", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsElDefaultResourceSpecEl {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_alias`.\n"]
    pub fn set_sagemaker_image_version_alias(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_alias = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsElDefaultResourceSpecEl {
    type O = BlockAssignable<SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsElDefaultResourceSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDomainSettingsElRStudioServerProDomainSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerDomainDomainSettingsElRStudioServerProDomainSettingsElDefaultResourceSpecEl {
    pub fn build(self) -> SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsElDefaultResourceSpecEl {
        SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_alias: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsElDefaultResourceSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsElDefaultResourceSpecElRef {
        SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_config_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_alias` after provisioning.\n"]
    pub fn sagemaker_image_version_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_alias", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sagemaker_image_version_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsElDynamic {
    default_resource_spec: Option<
        DynamicBlock<SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsEl {
    domain_execution_role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r_studio_connect_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r_studio_package_manager_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec: Option<
        Vec<SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsElDefaultResourceSpecEl>,
    >,
    dynamic: SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsElDynamic,
}

impl SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsEl {
    #[doc = "Set the field `r_studio_connect_url`.\n"]
    pub fn set_r_studio_connect_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.r_studio_connect_url = Some(v.into());
        self
    }

    #[doc = "Set the field `r_studio_package_manager_url`.\n"]
    pub fn set_r_studio_package_manager_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.r_studio_package_manager_url = Some(v.into());
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsElDefaultResourceSpecEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsEl {
    type O = BlockAssignable<SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDomainSettingsElRStudioServerProDomainSettingsEl {
    #[doc = ""]
    pub domain_execution_role_arn: PrimField<String>,
}

impl BuildSagemakerDomainDomainSettingsElRStudioServerProDomainSettingsEl {
    pub fn build(self) -> SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsEl {
        SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsEl {
            domain_execution_role_arn: self.domain_execution_role_arn,
            r_studio_connect_url: core::default::Default::default(),
            r_studio_package_manager_url: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsElRef {
        SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_execution_role_arn` after provisioning.\n"]
    pub fn domain_execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_execution_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `r_studio_connect_url` after provisioning.\n"]
    pub fn r_studio_connect_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.r_studio_connect_url", self.base))
    }

    #[doc = "Get a reference to the value of field `r_studio_package_manager_url` after provisioning.\n"]
    pub fn r_studio_package_manager_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.r_studio_package_manager_url", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsElDefaultResourceSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_resource_spec", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDomainSettingsElDynamic {
    docker_settings: Option<DynamicBlock<SagemakerDomainDomainSettingsElDockerSettingsEl>>,
    r_studio_server_pro_domain_settings: Option<
        DynamicBlock<SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerDomainDomainSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_role_identity_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    docker_settings: Option<Vec<SagemakerDomainDomainSettingsElDockerSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r_studio_server_pro_domain_settings: Option<Vec<SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsEl>>,
    dynamic: SagemakerDomainDomainSettingsElDynamic,
}

impl SagemakerDomainDomainSettingsEl {
    #[doc = "Set the field `execution_role_identity_config`.\n"]
    pub fn set_execution_role_identity_config(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.execution_role_identity_config = Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_group_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `docker_settings`.\n"]
    pub fn set_docker_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDomainSettingsElDockerSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.docker_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.docker_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `r_studio_server_pro_domain_settings`.\n"]
    pub fn set_r_studio_server_pro_domain_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.r_studio_server_pro_domain_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.r_studio_server_pro_domain_settings = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerDomainDomainSettingsEl {
    type O = BlockAssignable<SagemakerDomainDomainSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainDomainSettingsEl {}

impl BuildSagemakerDomainDomainSettingsEl {
    pub fn build(self) -> SagemakerDomainDomainSettingsEl {
        SagemakerDomainDomainSettingsEl {
            execution_role_identity_config: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            docker_settings: core::default::Default::default(),
            r_studio_server_pro_domain_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerDomainDomainSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainDomainSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainDomainSettingsElRef {
        SagemakerDomainDomainSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainDomainSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `execution_role_identity_config` after provisioning.\n"]
    pub fn execution_role_identity_config(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_role_identity_config", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `docker_settings` after provisioning.\n"]
    pub fn docker_settings(&self) -> ListRef<SagemakerDomainDomainSettingsElDockerSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.docker_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `r_studio_server_pro_domain_settings` after provisioning.\n"]
    pub fn r_studio_server_pro_domain_settings(
        &self,
    ) -> ListRef<SagemakerDomainDomainSettingsElRStudioServerProDomainSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.r_studio_server_pro_domain_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerDomainRetentionPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    home_efs_file_system: Option<PrimField<String>>,
}

impl SagemakerDomainRetentionPolicyEl {
    #[doc = "Set the field `home_efs_file_system`.\n"]
    pub fn set_home_efs_file_system(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.home_efs_file_system = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerDomainRetentionPolicyEl {
    type O = BlockAssignable<SagemakerDomainRetentionPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerDomainRetentionPolicyEl {}

impl BuildSagemakerDomainRetentionPolicyEl {
    pub fn build(self) -> SagemakerDomainRetentionPolicyEl {
        SagemakerDomainRetentionPolicyEl { home_efs_file_system: core::default::Default::default() }
    }
}

pub struct SagemakerDomainRetentionPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerDomainRetentionPolicyElRef {
    fn new(shared: StackShared, base: String) -> SagemakerDomainRetentionPolicyElRef {
        SagemakerDomainRetentionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerDomainRetentionPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `home_efs_file_system` after provisioning.\n"]
    pub fn home_efs_file_system(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.home_efs_file_system", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerDomainDynamic {
    default_space_settings: Option<DynamicBlock<SagemakerDomainDefaultSpaceSettingsEl>>,
    default_user_settings: Option<DynamicBlock<SagemakerDomainDefaultUserSettingsEl>>,
    domain_settings: Option<DynamicBlock<SagemakerDomainDomainSettingsEl>>,
    retention_policy: Option<DynamicBlock<SagemakerDomainRetentionPolicyEl>>,
}
