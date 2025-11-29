use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NetworkmanagerLinkData {
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
    global_network_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_name: Option<PrimField<String>>,
    site_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bandwidth: Option<Vec<NetworkmanagerLinkBandwidthEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkmanagerLinkTimeoutsEl>,
    dynamic: NetworkmanagerLinkDynamic,
}

struct NetworkmanagerLink_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkmanagerLinkData>,
}

#[derive(Clone)]
pub struct NetworkmanagerLink(Rc<NetworkmanagerLink_>);

impl NetworkmanagerLink {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `provider_name`.\n"]
    pub fn set_provider_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().provider_name = Some(v.into());
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

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `bandwidth`.\n"]
    pub fn set_bandwidth(self, v: impl Into<BlockAssignable<NetworkmanagerLinkBandwidthEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bandwidth = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bandwidth = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkmanagerLinkTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `global_network_id` after provisioning.\n"]
    pub fn global_network_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.global_network_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provider_name` after provisioning.\n"]
    pub fn provider_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_id` after provisioning.\n"]
    pub fn site_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.site_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bandwidth` after provisioning.\n"]
    pub fn bandwidth(&self) -> ListRef<NetworkmanagerLinkBandwidthElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bandwidth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkmanagerLinkTimeoutsElRef {
        NetworkmanagerLinkTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for NetworkmanagerLink {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for NetworkmanagerLink { }

impl ToListMappable for NetworkmanagerLink {
    type O = ListRef<NetworkmanagerLinkRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for NetworkmanagerLink_ {
    fn extract_resource_type(&self) -> String {
        "aws_networkmanager_link".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkmanagerLink {
    pub tf_id: String,
    #[doc = ""]
    pub global_network_id: PrimField<String>,
    #[doc = ""]
    pub site_id: PrimField<String>,
}

impl BuildNetworkmanagerLink {
    pub fn build(self, stack: &mut Stack) -> NetworkmanagerLink {
        let out = NetworkmanagerLink(Rc::new(NetworkmanagerLink_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkmanagerLinkData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                global_network_id: self.global_network_id,
                id: core::default::Default::default(),
                provider_name: core::default::Default::default(),
                site_id: self.site_id,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: core::default::Default::default(),
                bandwidth: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkmanagerLinkRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkmanagerLinkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl NetworkmanagerLinkRef {
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
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `global_network_id` after provisioning.\n"]
    pub fn global_network_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.global_network_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provider_name` after provisioning.\n"]
    pub fn provider_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_id` after provisioning.\n"]
    pub fn site_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.site_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bandwidth` after provisioning.\n"]
    pub fn bandwidth(&self) -> ListRef<NetworkmanagerLinkBandwidthElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bandwidth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkmanagerLinkTimeoutsElRef {
        NetworkmanagerLinkTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct NetworkmanagerLinkBandwidthEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    download_speed: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upload_speed: Option<PrimField<f64>>,
}

impl NetworkmanagerLinkBandwidthEl {
    #[doc = "Set the field `download_speed`.\n"]
    pub fn set_download_speed(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.download_speed = Some(v.into());
        self
    }

    #[doc = "Set the field `upload_speed`.\n"]
    pub fn set_upload_speed(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.upload_speed = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkmanagerLinkBandwidthEl {
    type O = BlockAssignable<NetworkmanagerLinkBandwidthEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkmanagerLinkBandwidthEl {}

impl BuildNetworkmanagerLinkBandwidthEl {
    pub fn build(self) -> NetworkmanagerLinkBandwidthEl {
        NetworkmanagerLinkBandwidthEl {
            download_speed: core::default::Default::default(),
            upload_speed: core::default::Default::default(),
        }
    }
}

pub struct NetworkmanagerLinkBandwidthElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkmanagerLinkBandwidthElRef {
    fn new(shared: StackShared, base: String) -> NetworkmanagerLinkBandwidthElRef {
        NetworkmanagerLinkBandwidthElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkmanagerLinkBandwidthElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `download_speed` after provisioning.\n"]
    pub fn download_speed(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.download_speed", self.base))
    }

    #[doc = "Get a reference to the value of field `upload_speed` after provisioning.\n"]
    pub fn upload_speed(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.upload_speed", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkmanagerLinkTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl NetworkmanagerLinkTimeoutsEl {
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

impl ToListMappable for NetworkmanagerLinkTimeoutsEl {
    type O = BlockAssignable<NetworkmanagerLinkTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkmanagerLinkTimeoutsEl {}

impl BuildNetworkmanagerLinkTimeoutsEl {
    pub fn build(self) -> NetworkmanagerLinkTimeoutsEl {
        NetworkmanagerLinkTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct NetworkmanagerLinkTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkmanagerLinkTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkmanagerLinkTimeoutsElRef {
        NetworkmanagerLinkTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkmanagerLinkTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct NetworkmanagerLinkDynamic {
    bandwidth: Option<DynamicBlock<NetworkmanagerLinkBandwidthEl>>,
}
