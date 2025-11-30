use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct AthenaWorkgroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<Vec<AthenaWorkgroupConfigurationEl>>,
    dynamic: AthenaWorkgroupDynamic,
}

struct AthenaWorkgroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AthenaWorkgroupData>,
}

#[derive(Clone)]
pub struct AthenaWorkgroup(Rc<AthenaWorkgroup_>);

impl AthenaWorkgroup {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `force_destroy`.\n"]
    pub fn set_force_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().state = Some(v.into());
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

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl Into<BlockAssignable<AthenaWorkgroupConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_destroy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<AthenaWorkgroupConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }
}

impl Referable for AthenaWorkgroup {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for AthenaWorkgroup {}

impl ToListMappable for AthenaWorkgroup {
    type O = ListRef<AthenaWorkgroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AthenaWorkgroup_ {
    fn extract_resource_type(&self) -> String {
        "aws_athena_workgroup".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAthenaWorkgroup {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildAthenaWorkgroup {
    pub fn build(self, stack: &mut Stack) -> AthenaWorkgroup {
        let out = AthenaWorkgroup(Rc::new(AthenaWorkgroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AthenaWorkgroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                state: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AthenaWorkgroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for AthenaWorkgroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl AthenaWorkgroupRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_destroy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<AthenaWorkgroupConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct AthenaWorkgroupConfigurationElEngineVersionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    selected_engine_version: Option<PrimField<String>>,
}

impl AthenaWorkgroupConfigurationElEngineVersionEl {
    #[doc = "Set the field `selected_engine_version`.\n"]
    pub fn set_selected_engine_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.selected_engine_version = Some(v.into());
        self
    }
}

impl ToListMappable for AthenaWorkgroupConfigurationElEngineVersionEl {
    type O = BlockAssignable<AthenaWorkgroupConfigurationElEngineVersionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroupConfigurationElEngineVersionEl {}

impl BuildAthenaWorkgroupConfigurationElEngineVersionEl {
    pub fn build(self) -> AthenaWorkgroupConfigurationElEngineVersionEl {
        AthenaWorkgroupConfigurationElEngineVersionEl {
            selected_engine_version: core::default::Default::default(),
        }
    }
}

pub struct AthenaWorkgroupConfigurationElEngineVersionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AthenaWorkgroupConfigurationElEngineVersionElRef {
    fn new(shared: StackShared, base: String) -> AthenaWorkgroupConfigurationElEngineVersionElRef {
        AthenaWorkgroupConfigurationElEngineVersionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroupConfigurationElEngineVersionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `effective_engine_version` after provisioning.\n"]
    pub fn effective_engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_engine_version", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `selected_engine_version` after provisioning.\n"]
    pub fn selected_engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.selected_engine_version", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct AthenaWorkgroupConfigurationElIdentityCenterConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_identity_center: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_center_instance_arn: Option<PrimField<String>>,
}

impl AthenaWorkgroupConfigurationElIdentityCenterConfigurationEl {
    #[doc = "Set the field `enable_identity_center`.\n"]
    pub fn set_enable_identity_center(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_identity_center = Some(v.into());
        self
    }

    #[doc = "Set the field `identity_center_instance_arn`.\n"]
    pub fn set_identity_center_instance_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_center_instance_arn = Some(v.into());
        self
    }
}

impl ToListMappable for AthenaWorkgroupConfigurationElIdentityCenterConfigurationEl {
    type O = BlockAssignable<AthenaWorkgroupConfigurationElIdentityCenterConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroupConfigurationElIdentityCenterConfigurationEl {}

impl BuildAthenaWorkgroupConfigurationElIdentityCenterConfigurationEl {
    pub fn build(self) -> AthenaWorkgroupConfigurationElIdentityCenterConfigurationEl {
        AthenaWorkgroupConfigurationElIdentityCenterConfigurationEl {
            enable_identity_center: core::default::Default::default(),
            identity_center_instance_arn: core::default::Default::default(),
        }
    }
}

pub struct AthenaWorkgroupConfigurationElIdentityCenterConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AthenaWorkgroupConfigurationElIdentityCenterConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AthenaWorkgroupConfigurationElIdentityCenterConfigurationElRef {
        AthenaWorkgroupConfigurationElIdentityCenterConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroupConfigurationElIdentityCenterConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_identity_center` after provisioning.\n"]
    pub fn enable_identity_center(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_identity_center", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `identity_center_instance_arn` after provisioning.\n"]
    pub fn identity_center_instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_center_instance_arn", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationEl {
    s3_acl_option: PrimField<String>,
}

impl AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationEl {}

impl ToListMappable for AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationEl {
    type O = BlockAssignable<AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationEl {
    #[doc = ""]
    pub s3_acl_option: PrimField<String>,
}

impl BuildAthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationEl {
    pub fn build(self) -> AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationEl {
        AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationEl {
            s3_acl_option: self.s3_acl_option,
        }
    }
}

pub struct AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationElRef {
        AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_acl_option` after provisioning.\n"]
    pub fn s3_acl_option(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_acl_option", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_option: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
}

impl AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationEl {
    #[doc = "Set the field `encryption_option`.\n"]
    pub fn set_encryption_option(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_option = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_arn = Some(v.into());
        self
    }
}

impl ToListMappable
    for AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationEl
{
    type O = BlockAssignable<
        AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationEl {}

impl BuildAthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationEl {
    pub fn build(
        self,
    ) -> AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationEl {
        AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationEl {
            encryption_option: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
        }
    }
}

pub struct AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationElRef {
        AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_option` after provisioning.\n"]
    pub fn encryption_option(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_option", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct AthenaWorkgroupConfigurationElResultConfigurationElDynamic {
    acl_configuration:
        Option<DynamicBlock<AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationEl>>,
    encryption_configuration: Option<
        DynamicBlock<AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct AthenaWorkgroupConfigurationElResultConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expected_bucket_owner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    acl_configuration:
        Option<Vec<AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_configuration:
        Option<Vec<AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationEl>>,
    dynamic: AthenaWorkgroupConfigurationElResultConfigurationElDynamic,
}

impl AthenaWorkgroupConfigurationElResultConfigurationEl {
    #[doc = "Set the field `expected_bucket_owner`.\n"]
    pub fn set_expected_bucket_owner(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expected_bucket_owner = Some(v.into());
        self
    }

    #[doc = "Set the field `output_location`.\n"]
    pub fn set_output_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_location = Some(v.into());
        self
    }

    #[doc = "Set the field `acl_configuration`.\n"]
    pub fn set_acl_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.acl_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.acl_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.encryption_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AthenaWorkgroupConfigurationElResultConfigurationEl {
    type O = BlockAssignable<AthenaWorkgroupConfigurationElResultConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroupConfigurationElResultConfigurationEl {}

impl BuildAthenaWorkgroupConfigurationElResultConfigurationEl {
    pub fn build(self) -> AthenaWorkgroupConfigurationElResultConfigurationEl {
        AthenaWorkgroupConfigurationElResultConfigurationEl {
            expected_bucket_owner: core::default::Default::default(),
            output_location: core::default::Default::default(),
            acl_configuration: core::default::Default::default(),
            encryption_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AthenaWorkgroupConfigurationElResultConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AthenaWorkgroupConfigurationElResultConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AthenaWorkgroupConfigurationElResultConfigurationElRef {
        AthenaWorkgroupConfigurationElResultConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroupConfigurationElResultConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expected_bucket_owner", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `output_location` after provisioning.\n"]
    pub fn output_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_location", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `acl_configuration` after provisioning.\n"]
    pub fn acl_configuration(
        &self,
    ) -> ListRef<AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.acl_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> ListRef<AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct AthenaWorkgroupConfigurationElDynamic {
    engine_version: Option<DynamicBlock<AthenaWorkgroupConfigurationElEngineVersionEl>>,
    identity_center_configuration:
        Option<DynamicBlock<AthenaWorkgroupConfigurationElIdentityCenterConfigurationEl>>,
    result_configuration: Option<DynamicBlock<AthenaWorkgroupConfigurationElResultConfigurationEl>>,
}

#[derive(Serialize)]
pub struct AthenaWorkgroupConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bytes_scanned_cutoff_per_query: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_workgroup_configuration: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publish_cloudwatch_metrics_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requester_pays_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine_version: Option<Vec<AthenaWorkgroupConfigurationElEngineVersionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_center_configuration:
        Option<Vec<AthenaWorkgroupConfigurationElIdentityCenterConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result_configuration: Option<Vec<AthenaWorkgroupConfigurationElResultConfigurationEl>>,
    dynamic: AthenaWorkgroupConfigurationElDynamic,
}

impl AthenaWorkgroupConfigurationEl {
    #[doc = "Set the field `bytes_scanned_cutoff_per_query`.\n"]
    pub fn set_bytes_scanned_cutoff_per_query(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.bytes_scanned_cutoff_per_query = Some(v.into());
        self
    }

    #[doc = "Set the field `enforce_workgroup_configuration`.\n"]
    pub fn set_enforce_workgroup_configuration(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enforce_workgroup_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `execution_role`.\n"]
    pub fn set_execution_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.execution_role = Some(v.into());
        self
    }

    #[doc = "Set the field `publish_cloudwatch_metrics_enabled`.\n"]
    pub fn set_publish_cloudwatch_metrics_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.publish_cloudwatch_metrics_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `requester_pays_enabled`.\n"]
    pub fn set_requester_pays_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.requester_pays_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `engine_version`.\n"]
    pub fn set_engine_version(
        mut self,
        v: impl Into<BlockAssignable<AthenaWorkgroupConfigurationElEngineVersionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.engine_version = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.engine_version = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `identity_center_configuration`.\n"]
    pub fn set_identity_center_configuration(
        mut self,
        v: impl Into<BlockAssignable<AthenaWorkgroupConfigurationElIdentityCenterConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.identity_center_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.identity_center_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `result_configuration`.\n"]
    pub fn set_result_configuration(
        mut self,
        v: impl Into<BlockAssignable<AthenaWorkgroupConfigurationElResultConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.result_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.result_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AthenaWorkgroupConfigurationEl {
    type O = BlockAssignable<AthenaWorkgroupConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroupConfigurationEl {}

impl BuildAthenaWorkgroupConfigurationEl {
    pub fn build(self) -> AthenaWorkgroupConfigurationEl {
        AthenaWorkgroupConfigurationEl {
            bytes_scanned_cutoff_per_query: core::default::Default::default(),
            enforce_workgroup_configuration: core::default::Default::default(),
            execution_role: core::default::Default::default(),
            publish_cloudwatch_metrics_enabled: core::default::Default::default(),
            requester_pays_enabled: core::default::Default::default(),
            engine_version: core::default::Default::default(),
            identity_center_configuration: core::default::Default::default(),
            result_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AthenaWorkgroupConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AthenaWorkgroupConfigurationElRef {
    fn new(shared: StackShared, base: String) -> AthenaWorkgroupConfigurationElRef {
        AthenaWorkgroupConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroupConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bytes_scanned_cutoff_per_query` after provisioning.\n"]
    pub fn bytes_scanned_cutoff_per_query(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bytes_scanned_cutoff_per_query", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `enforce_workgroup_configuration` after provisioning.\n"]
    pub fn enforce_workgroup_configuration(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_workgroup_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_role", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `publish_cloudwatch_metrics_enabled` after provisioning.\n"]
    pub fn publish_cloudwatch_metrics_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.publish_cloudwatch_metrics_enabled", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `requester_pays_enabled` after provisioning.\n"]
    pub fn requester_pays_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.requester_pays_enabled", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> ListRef<AthenaWorkgroupConfigurationElEngineVersionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.engine_version", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `identity_center_configuration` after provisioning.\n"]
    pub fn identity_center_configuration(
        &self,
    ) -> ListRef<AthenaWorkgroupConfigurationElIdentityCenterConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.identity_center_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `result_configuration` after provisioning.\n"]
    pub fn result_configuration(
        &self,
    ) -> ListRef<AthenaWorkgroupConfigurationElResultConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.result_configuration", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct AthenaWorkgroupDynamic {
    configuration: Option<DynamicBlock<AthenaWorkgroupConfigurationEl>>,
}
