use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentcoreBrowserData {
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
    execution_role_arn: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_configuration: Option<Vec<BedrockagentcoreBrowserNetworkConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recording: Option<Vec<BedrockagentcoreBrowserRecordingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BedrockagentcoreBrowserTimeoutsEl>,
    dynamic: BedrockagentcoreBrowserDynamic,
}

struct BedrockagentcoreBrowser_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BedrockagentcoreBrowserData>,
}

#[derive(Clone)]
pub struct BedrockagentcoreBrowser(Rc<BedrockagentcoreBrowser_>);

impl BedrockagentcoreBrowser {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `execution_role_arn`.\n"]
    pub fn set_execution_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().execution_role_arn = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        self,
        v: impl Into<BlockAssignable<BedrockagentcoreBrowserNetworkConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `recording`.\n"]
    pub fn set_recording(self, v: impl Into<BlockAssignable<BedrockagentcoreBrowserRecordingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().recording = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.recording = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BedrockagentcoreBrowserTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `browser_arn` after provisioning.\n"]
    pub fn browser_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.browser_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `browser_id` after provisioning.\n"]
    pub fn browser_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.browser_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> ListRef<BedrockagentcoreBrowserNetworkConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recording` after provisioning.\n"]
    pub fn recording(&self) -> ListRef<BedrockagentcoreBrowserRecordingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.recording", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreBrowserTimeoutsElRef {
        BedrockagentcoreBrowserTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for BedrockagentcoreBrowser {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for BedrockagentcoreBrowser { }

impl ToListMappable for BedrockagentcoreBrowser {
    type O = ListRef<BedrockagentcoreBrowserRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BedrockagentcoreBrowser_ {
    fn extract_resource_type(&self) -> String {
        "aws_bedrockagentcore_browser".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentcoreBrowser {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildBedrockagentcoreBrowser {
    pub fn build(self, stack: &mut Stack) -> BedrockagentcoreBrowser {
        let out = BedrockagentcoreBrowser(Rc::new(BedrockagentcoreBrowser_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BedrockagentcoreBrowserData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                execution_role_arn: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                network_configuration: core::default::Default::default(),
                recording: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentcoreBrowserRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreBrowserRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl BedrockagentcoreBrowserRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `browser_arn` after provisioning.\n"]
    pub fn browser_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.browser_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `browser_id` after provisioning.\n"]
    pub fn browser_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.browser_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> ListRef<BedrockagentcoreBrowserNetworkConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recording` after provisioning.\n"]
    pub fn recording(&self) -> ListRef<BedrockagentcoreBrowserRecordingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.recording", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreBrowserTimeoutsElRef {
        BedrockagentcoreBrowserTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreBrowserNetworkConfigurationElVpcConfigEl {
    security_groups: SetField<PrimField<String>>,
    subnets: SetField<PrimField<String>>,
}

impl BedrockagentcoreBrowserNetworkConfigurationElVpcConfigEl { }

impl ToListMappable for BedrockagentcoreBrowserNetworkConfigurationElVpcConfigEl {
    type O = BlockAssignable<BedrockagentcoreBrowserNetworkConfigurationElVpcConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreBrowserNetworkConfigurationElVpcConfigEl {
    #[doc = ""]
    pub security_groups: SetField<PrimField<String>>,
    #[doc = ""]
    pub subnets: SetField<PrimField<String>>,
}

impl BuildBedrockagentcoreBrowserNetworkConfigurationElVpcConfigEl {
    pub fn build(self) -> BedrockagentcoreBrowserNetworkConfigurationElVpcConfigEl {
        BedrockagentcoreBrowserNetworkConfigurationElVpcConfigEl {
            security_groups: self.security_groups,
            subnets: self.subnets,
        }
    }
}

pub struct BedrockagentcoreBrowserNetworkConfigurationElVpcConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreBrowserNetworkConfigurationElVpcConfigElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentcoreBrowserNetworkConfigurationElVpcConfigElRef {
        BedrockagentcoreBrowserNetworkConfigurationElVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreBrowserNetworkConfigurationElVpcConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreBrowserNetworkConfigurationElDynamic {
    vpc_config: Option<DynamicBlock<BedrockagentcoreBrowserNetworkConfigurationElVpcConfigEl>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreBrowserNetworkConfigurationEl {
    network_mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_config: Option<Vec<BedrockagentcoreBrowserNetworkConfigurationElVpcConfigEl>>,
    dynamic: BedrockagentcoreBrowserNetworkConfigurationElDynamic,
}

impl BedrockagentcoreBrowserNetworkConfigurationEl {
    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentcoreBrowserNetworkConfigurationElVpcConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vpc_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentcoreBrowserNetworkConfigurationEl {
    type O = BlockAssignable<BedrockagentcoreBrowserNetworkConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreBrowserNetworkConfigurationEl {
    #[doc = ""]
    pub network_mode: PrimField<String>,
}

impl BuildBedrockagentcoreBrowserNetworkConfigurationEl {
    pub fn build(self) -> BedrockagentcoreBrowserNetworkConfigurationEl {
        BedrockagentcoreBrowserNetworkConfigurationEl {
            network_mode: self.network_mode,
            vpc_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentcoreBrowserNetworkConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreBrowserNetworkConfigurationElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentcoreBrowserNetworkConfigurationElRef {
        BedrockagentcoreBrowserNetworkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreBrowserNetworkConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_mode` after provisioning.\n"]
    pub fn network_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<BedrockagentcoreBrowserNetworkConfigurationElVpcConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreBrowserRecordingElS3LocationEl {
    bucket: PrimField<String>,
    prefix: PrimField<String>,
}

impl BedrockagentcoreBrowserRecordingElS3LocationEl { }

impl ToListMappable for BedrockagentcoreBrowserRecordingElS3LocationEl {
    type O = BlockAssignable<BedrockagentcoreBrowserRecordingElS3LocationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreBrowserRecordingElS3LocationEl {
    #[doc = ""]
    pub bucket: PrimField<String>,
    #[doc = ""]
    pub prefix: PrimField<String>,
}

impl BuildBedrockagentcoreBrowserRecordingElS3LocationEl {
    pub fn build(self) -> BedrockagentcoreBrowserRecordingElS3LocationEl {
        BedrockagentcoreBrowserRecordingElS3LocationEl {
            bucket: self.bucket,
            prefix: self.prefix,
        }
    }
}

pub struct BedrockagentcoreBrowserRecordingElS3LocationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreBrowserRecordingElS3LocationElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentcoreBrowserRecordingElS3LocationElRef {
        BedrockagentcoreBrowserRecordingElS3LocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreBrowserRecordingElS3LocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreBrowserRecordingElDynamic {
    s3_location: Option<DynamicBlock<BedrockagentcoreBrowserRecordingElS3LocationEl>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreBrowserRecordingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_location: Option<Vec<BedrockagentcoreBrowserRecordingElS3LocationEl>>,
    dynamic: BedrockagentcoreBrowserRecordingElDynamic,
}

impl BedrockagentcoreBrowserRecordingEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_location`.\n"]
    pub fn set_s3_location(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentcoreBrowserRecordingElS3LocationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_location = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_location = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentcoreBrowserRecordingEl {
    type O = BlockAssignable<BedrockagentcoreBrowserRecordingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreBrowserRecordingEl {}

impl BuildBedrockagentcoreBrowserRecordingEl {
    pub fn build(self) -> BedrockagentcoreBrowserRecordingEl {
        BedrockagentcoreBrowserRecordingEl {
            enabled: core::default::Default::default(),
            s3_location: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentcoreBrowserRecordingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreBrowserRecordingElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentcoreBrowserRecordingElRef {
        BedrockagentcoreBrowserRecordingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreBrowserRecordingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_location` after provisioning.\n"]
    pub fn s3_location(&self) -> ListRef<BedrockagentcoreBrowserRecordingElS3LocationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_location", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreBrowserTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl BedrockagentcoreBrowserTimeoutsEl {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentcoreBrowserTimeoutsEl {
    type O = BlockAssignable<BedrockagentcoreBrowserTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreBrowserTimeoutsEl {}

impl BuildBedrockagentcoreBrowserTimeoutsEl {
    pub fn build(self) -> BedrockagentcoreBrowserTimeoutsEl {
        BedrockagentcoreBrowserTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentcoreBrowserTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentcoreBrowserTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentcoreBrowserTimeoutsElRef {
        BedrockagentcoreBrowserTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreBrowserTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreBrowserDynamic {
    network_configuration: Option<DynamicBlock<BedrockagentcoreBrowserNetworkConfigurationEl>>,
    recording: Option<DynamicBlock<BedrockagentcoreBrowserRecordingEl>>,
}
