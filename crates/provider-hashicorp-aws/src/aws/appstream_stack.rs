use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppstreamStackData {
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
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    embed_host_domains: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    feedback_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_endpoints: Option<Vec<AppstreamStackAccessEndpointsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_settings: Option<Vec<AppstreamStackApplicationSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_connectors: Option<Vec<AppstreamStackStorageConnectorsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    streaming_experience_settings: Option<Vec<AppstreamStackStreamingExperienceSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_settings: Option<Vec<AppstreamStackUserSettingsEl>>,
    dynamic: AppstreamStackDynamic,
}

struct AppstreamStack_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppstreamStackData>,
}

#[derive(Clone)]
pub struct AppstreamStack(Rc<AppstreamStack_>);

impl AppstreamStack {
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

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }

    #[doc = "Set the field `embed_host_domains`.\n"]
    pub fn set_embed_host_domains(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().embed_host_domains = Some(v.into());
        self
    }

    #[doc = "Set the field `feedback_url`.\n"]
    pub fn set_feedback_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().feedback_url = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `redirect_url`.\n"]
    pub fn set_redirect_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().redirect_url = Some(v.into());
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

    #[doc = "Set the field `access_endpoints`.\n"]
    pub fn set_access_endpoints(self, v: impl Into<BlockAssignable<AppstreamStackAccessEndpointsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_endpoints = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_endpoints = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `application_settings`.\n"]
    pub fn set_application_settings(self, v: impl Into<BlockAssignable<AppstreamStackApplicationSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().application_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.application_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_connectors`.\n"]
    pub fn set_storage_connectors(self, v: impl Into<BlockAssignable<AppstreamStackStorageConnectorsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().storage_connectors = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.storage_connectors = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `streaming_experience_settings`.\n"]
    pub fn set_streaming_experience_settings(
        self,
        v: impl Into<BlockAssignable<AppstreamStackStreamingExperienceSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().streaming_experience_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.streaming_experience_settings = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_settings`.\n"]
    pub fn set_user_settings(self, v: impl Into<BlockAssignable<AppstreamStackUserSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user_settings = Some(d);
            },
        }
        self
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

    #[doc = "Get a reference to the value of field `embed_host_domains` after provisioning.\n"]
    pub fn embed_host_domains(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.embed_host_domains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feedback_url` after provisioning.\n"]
    pub fn feedback_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.feedback_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redirect_url` after provisioning.\n"]
    pub fn redirect_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.redirect_url", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `application_settings` after provisioning.\n"]
    pub fn application_settings(&self) -> ListRef<AppstreamStackApplicationSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.application_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `streaming_experience_settings` after provisioning.\n"]
    pub fn streaming_experience_settings(&self) -> ListRef<AppstreamStackStreamingExperienceSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.streaming_experience_settings", self.extract_ref()))
    }
}

impl Referable for AppstreamStack {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AppstreamStack { }

impl ToListMappable for AppstreamStack {
    type O = ListRef<AppstreamStackRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AppstreamStack_ {
    fn extract_resource_type(&self) -> String {
        "aws_appstream_stack".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppstreamStack {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildAppstreamStack {
    pub fn build(self, stack: &mut Stack) -> AppstreamStack {
        let out = AppstreamStack(Rc::new(AppstreamStack_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppstreamStackData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                embed_host_domains: core::default::Default::default(),
                feedback_url: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                redirect_url: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                access_endpoints: core::default::Default::default(),
                application_settings: core::default::Default::default(),
                storage_connectors: core::default::Default::default(),
                streaming_experience_settings: core::default::Default::default(),
                user_settings: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppstreamStackRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppstreamStackRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl AppstreamStackRef {
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

    #[doc = "Get a reference to the value of field `embed_host_domains` after provisioning.\n"]
    pub fn embed_host_domains(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.embed_host_domains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feedback_url` after provisioning.\n"]
    pub fn feedback_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.feedback_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redirect_url` after provisioning.\n"]
    pub fn redirect_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.redirect_url", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `application_settings` after provisioning.\n"]
    pub fn application_settings(&self) -> ListRef<AppstreamStackApplicationSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.application_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `streaming_experience_settings` after provisioning.\n"]
    pub fn streaming_experience_settings(&self) -> ListRef<AppstreamStackStreamingExperienceSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.streaming_experience_settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AppstreamStackAccessEndpointsEl {
    endpoint_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpce_id: Option<PrimField<String>>,
}

impl AppstreamStackAccessEndpointsEl {
    #[doc = "Set the field `vpce_id`.\n"]
    pub fn set_vpce_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpce_id = Some(v.into());
        self
    }
}

impl ToListMappable for AppstreamStackAccessEndpointsEl {
    type O = BlockAssignable<AppstreamStackAccessEndpointsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppstreamStackAccessEndpointsEl {
    #[doc = ""]
    pub endpoint_type: PrimField<String>,
}

impl BuildAppstreamStackAccessEndpointsEl {
    pub fn build(self) -> AppstreamStackAccessEndpointsEl {
        AppstreamStackAccessEndpointsEl {
            endpoint_type: self.endpoint_type,
            vpce_id: core::default::Default::default(),
        }
    }
}

pub struct AppstreamStackAccessEndpointsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppstreamStackAccessEndpointsElRef {
    fn new(shared: StackShared, base: String) -> AppstreamStackAccessEndpointsElRef {
        AppstreamStackAccessEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppstreamStackAccessEndpointsElRef {
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
pub struct AppstreamStackApplicationSettingsEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings_group: Option<PrimField<String>>,
}

impl AppstreamStackApplicationSettingsEl {
    #[doc = "Set the field `settings_group`.\n"]
    pub fn set_settings_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.settings_group = Some(v.into());
        self
    }
}

impl ToListMappable for AppstreamStackApplicationSettingsEl {
    type O = BlockAssignable<AppstreamStackApplicationSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppstreamStackApplicationSettingsEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}

impl BuildAppstreamStackApplicationSettingsEl {
    pub fn build(self) -> AppstreamStackApplicationSettingsEl {
        AppstreamStackApplicationSettingsEl {
            enabled: self.enabled,
            settings_group: core::default::Default::default(),
        }
    }
}

pub struct AppstreamStackApplicationSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppstreamStackApplicationSettingsElRef {
    fn new(shared: StackShared, base: String) -> AppstreamStackApplicationSettingsElRef {
        AppstreamStackApplicationSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppstreamStackApplicationSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `settings_group` after provisioning.\n"]
    pub fn settings_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.settings_group", self.base))
    }
}

#[derive(Serialize)]
pub struct AppstreamStackStorageConnectorsEl {
    connector_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domains: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_identifier: Option<PrimField<String>>,
}

impl AppstreamStackStorageConnectorsEl {
    #[doc = "Set the field `domains`.\n"]
    pub fn set_domains(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.domains = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_identifier`.\n"]
    pub fn set_resource_identifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_identifier = Some(v.into());
        self
    }
}

impl ToListMappable for AppstreamStackStorageConnectorsEl {
    type O = BlockAssignable<AppstreamStackStorageConnectorsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppstreamStackStorageConnectorsEl {
    #[doc = ""]
    pub connector_type: PrimField<String>,
}

impl BuildAppstreamStackStorageConnectorsEl {
    pub fn build(self) -> AppstreamStackStorageConnectorsEl {
        AppstreamStackStorageConnectorsEl {
            connector_type: self.connector_type,
            domains: core::default::Default::default(),
            resource_identifier: core::default::Default::default(),
        }
    }
}

pub struct AppstreamStackStorageConnectorsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppstreamStackStorageConnectorsElRef {
    fn new(shared: StackShared, base: String) -> AppstreamStackStorageConnectorsElRef {
        AppstreamStackStorageConnectorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppstreamStackStorageConnectorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connector_type` after provisioning.\n"]
    pub fn connector_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connector_type", self.base))
    }

    #[doc = "Get a reference to the value of field `domains` after provisioning.\n"]
    pub fn domains(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.domains", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_identifier` after provisioning.\n"]
    pub fn resource_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct AppstreamStackStreamingExperienceSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_protocol: Option<PrimField<String>>,
}

impl AppstreamStackStreamingExperienceSettingsEl {
    #[doc = "Set the field `preferred_protocol`.\n"]
    pub fn set_preferred_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.preferred_protocol = Some(v.into());
        self
    }
}

impl ToListMappable for AppstreamStackStreamingExperienceSettingsEl {
    type O = BlockAssignable<AppstreamStackStreamingExperienceSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppstreamStackStreamingExperienceSettingsEl {}

impl BuildAppstreamStackStreamingExperienceSettingsEl {
    pub fn build(self) -> AppstreamStackStreamingExperienceSettingsEl {
        AppstreamStackStreamingExperienceSettingsEl { preferred_protocol: core::default::Default::default() }
    }
}

pub struct AppstreamStackStreamingExperienceSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppstreamStackStreamingExperienceSettingsElRef {
    fn new(shared: StackShared, base: String) -> AppstreamStackStreamingExperienceSettingsElRef {
        AppstreamStackStreamingExperienceSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppstreamStackStreamingExperienceSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `preferred_protocol` after provisioning.\n"]
    pub fn preferred_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preferred_protocol", self.base))
    }
}

#[derive(Serialize)]
pub struct AppstreamStackUserSettingsEl {
    action: PrimField<String>,
    permission: PrimField<String>,
}

impl AppstreamStackUserSettingsEl { }

impl ToListMappable for AppstreamStackUserSettingsEl {
    type O = BlockAssignable<AppstreamStackUserSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppstreamStackUserSettingsEl {
    #[doc = ""]
    pub action: PrimField<String>,
    #[doc = ""]
    pub permission: PrimField<String>,
}

impl BuildAppstreamStackUserSettingsEl {
    pub fn build(self) -> AppstreamStackUserSettingsEl {
        AppstreamStackUserSettingsEl {
            action: self.action,
            permission: self.permission,
        }
    }
}

pub struct AppstreamStackUserSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppstreamStackUserSettingsElRef {
    fn new(shared: StackShared, base: String) -> AppstreamStackUserSettingsElRef {
        AppstreamStackUserSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppstreamStackUserSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `permission` after provisioning.\n"]
    pub fn permission(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppstreamStackDynamic {
    access_endpoints: Option<DynamicBlock<AppstreamStackAccessEndpointsEl>>,
    application_settings: Option<DynamicBlock<AppstreamStackApplicationSettingsEl>>,
    storage_connectors: Option<DynamicBlock<AppstreamStackStorageConnectorsEl>>,
    streaming_experience_settings: Option<DynamicBlock<AppstreamStackStreamingExperienceSettingsEl>>,
    user_settings: Option<DynamicBlock<AppstreamStackUserSettingsEl>>,
}
