use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppstreamImageBuilderData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    appstream_agent_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_default_internet_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_name: Option<PrimField<String>>,
    instance_type: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_endpoint: Option<Vec<AppstreamImageBuilderAccessEndpointEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_join_info: Option<Vec<AppstreamImageBuilderDomainJoinInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_config: Option<Vec<AppstreamImageBuilderVpcConfigEl>>,
    dynamic: AppstreamImageBuilderDynamic,
}

struct AppstreamImageBuilder_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppstreamImageBuilderData>,
}

#[derive(Clone)]
pub struct AppstreamImageBuilder(Rc<AppstreamImageBuilder_>);

impl AppstreamImageBuilder {
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

    #[doc = "Set the field `appstream_agent_version`.\n"]
    pub fn set_appstream_agent_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().appstream_agent_version = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_default_internet_access`.\n"]
    pub fn set_enable_default_internet_access(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_default_internet_access = Some(v.into());
        self
    }

    #[doc = "Set the field `iam_role_arn`.\n"]
    pub fn set_iam_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().iam_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `image_arn`.\n"]
    pub fn set_image_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().image_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `image_name`.\n"]
    pub fn set_image_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().image_name = Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `access_endpoint`.\n"]
    pub fn set_access_endpoint(self, v: impl Into<BlockAssignable<AppstreamImageBuilderAccessEndpointEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_endpoint = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_endpoint = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `domain_join_info`.\n"]
    pub fn set_domain_join_info(self, v: impl Into<BlockAssignable<AppstreamImageBuilderDomainJoinInfoEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().domain_join_info = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.domain_join_info = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(self, v: impl Into<BlockAssignable<AppstreamImageBuilderVpcConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_config = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `appstream_agent_version` after provisioning.\n"]
    pub fn appstream_agent_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.appstream_agent_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_default_internet_access` after provisioning.\n"]
    pub fn enable_default_internet_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_default_internet_access", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_arn` after provisioning.\n"]
    pub fn image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_join_info` after provisioning.\n"]
    pub fn domain_join_info(&self) -> ListRef<AppstreamImageBuilderDomainJoinInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.domain_join_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<AppstreamImageBuilderVpcConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

impl Referable for AppstreamImageBuilder {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AppstreamImageBuilder { }

impl ToListMappable for AppstreamImageBuilder {
    type O = ListRef<AppstreamImageBuilderRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AppstreamImageBuilder_ {
    fn extract_resource_type(&self) -> String {
        "aws_appstream_image_builder".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppstreamImageBuilder {
    pub tf_id: String,
    #[doc = ""]
    pub instance_type: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildAppstreamImageBuilder {
    pub fn build(self, stack: &mut Stack) -> AppstreamImageBuilder {
        let out = AppstreamImageBuilder(Rc::new(AppstreamImageBuilder_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppstreamImageBuilderData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                appstream_agent_version: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                enable_default_internet_access: core::default::Default::default(),
                iam_role_arn: core::default::Default::default(),
                id: core::default::Default::default(),
                image_arn: core::default::Default::default(),
                image_name: core::default::Default::default(),
                instance_type: self.instance_type,
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                access_endpoint: core::default::Default::default(),
                domain_join_info: core::default::Default::default(),
                vpc_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppstreamImageBuilderRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppstreamImageBuilderRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl AppstreamImageBuilderRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `appstream_agent_version` after provisioning.\n"]
    pub fn appstream_agent_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.appstream_agent_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_default_internet_access` after provisioning.\n"]
    pub fn enable_default_internet_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_default_internet_access", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_arn` after provisioning.\n"]
    pub fn image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_join_info` after provisioning.\n"]
    pub fn domain_join_info(&self) -> ListRef<AppstreamImageBuilderDomainJoinInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.domain_join_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<AppstreamImageBuilderVpcConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AppstreamImageBuilderAccessEndpointEl {
    endpoint_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpce_id: Option<PrimField<String>>,
}

impl AppstreamImageBuilderAccessEndpointEl {
    #[doc = "Set the field `vpce_id`.\n"]
    pub fn set_vpce_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpce_id = Some(v.into());
        self
    }
}

impl ToListMappable for AppstreamImageBuilderAccessEndpointEl {
    type O = BlockAssignable<AppstreamImageBuilderAccessEndpointEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppstreamImageBuilderAccessEndpointEl {
    #[doc = ""]
    pub endpoint_type: PrimField<String>,
}

impl BuildAppstreamImageBuilderAccessEndpointEl {
    pub fn build(self) -> AppstreamImageBuilderAccessEndpointEl {
        AppstreamImageBuilderAccessEndpointEl {
            endpoint_type: self.endpoint_type,
            vpce_id: core::default::Default::default(),
        }
    }
}

pub struct AppstreamImageBuilderAccessEndpointElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppstreamImageBuilderAccessEndpointElRef {
    fn new(shared: StackShared, base: String) -> AppstreamImageBuilderAccessEndpointElRef {
        AppstreamImageBuilderAccessEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppstreamImageBuilderAccessEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_type", self.base))
    }

    #[doc = "Get a reference to the value of field `vpce_id` after provisioning.\n"]
    pub fn vpce_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpce_id", self.base))
    }
}

#[derive(Serialize)]
pub struct AppstreamImageBuilderDomainJoinInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    directory_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organizational_unit_distinguished_name: Option<PrimField<String>>,
}

impl AppstreamImageBuilderDomainJoinInfoEl {
    #[doc = "Set the field `directory_name`.\n"]
    pub fn set_directory_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.directory_name = Some(v.into());
        self
    }

    #[doc = "Set the field `organizational_unit_distinguished_name`.\n"]
    pub fn set_organizational_unit_distinguished_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.organizational_unit_distinguished_name = Some(v.into());
        self
    }
}

impl ToListMappable for AppstreamImageBuilderDomainJoinInfoEl {
    type O = BlockAssignable<AppstreamImageBuilderDomainJoinInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppstreamImageBuilderDomainJoinInfoEl {}

impl BuildAppstreamImageBuilderDomainJoinInfoEl {
    pub fn build(self) -> AppstreamImageBuilderDomainJoinInfoEl {
        AppstreamImageBuilderDomainJoinInfoEl {
            directory_name: core::default::Default::default(),
            organizational_unit_distinguished_name: core::default::Default::default(),
        }
    }
}

pub struct AppstreamImageBuilderDomainJoinInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppstreamImageBuilderDomainJoinInfoElRef {
    fn new(shared: StackShared, base: String) -> AppstreamImageBuilderDomainJoinInfoElRef {
        AppstreamImageBuilderDomainJoinInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppstreamImageBuilderDomainJoinInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `directory_name` after provisioning.\n"]
    pub fn directory_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.directory_name", self.base))
    }

    #[doc = "Get a reference to the value of field `organizational_unit_distinguished_name` after provisioning.\n"]
    pub fn organizational_unit_distinguished_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.organizational_unit_distinguished_name", self.base))
    }
}

#[derive(Serialize)]
pub struct AppstreamImageBuilderVpcConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<SetField<PrimField<String>>>,
}

impl AppstreamImageBuilderVpcConfigEl {
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_group_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.subnet_ids = Some(v.into());
        self
    }
}

impl ToListMappable for AppstreamImageBuilderVpcConfigEl {
    type O = BlockAssignable<AppstreamImageBuilderVpcConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppstreamImageBuilderVpcConfigEl {}

impl BuildAppstreamImageBuilderVpcConfigEl {
    pub fn build(self) -> AppstreamImageBuilderVpcConfigEl {
        AppstreamImageBuilderVpcConfigEl {
            security_group_ids: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
        }
    }
}

pub struct AppstreamImageBuilderVpcConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppstreamImageBuilderVpcConfigElRef {
    fn new(shared: StackShared, base: String) -> AppstreamImageBuilderVpcConfigElRef {
        AppstreamImageBuilderVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppstreamImageBuilderVpcConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppstreamImageBuilderDynamic {
    access_endpoint: Option<DynamicBlock<AppstreamImageBuilderAccessEndpointEl>>,
    domain_join_info: Option<DynamicBlock<AppstreamImageBuilderDomainJoinInfoEl>>,
    vpc_config: Option<DynamicBlock<AppstreamImageBuilderVpcConfigEl>>,
}
