use super::provider::ProviderGrafana;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct PlaylistData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    interval: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    org_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    item: Option<Vec<PlaylistItemEl>>,
    dynamic: PlaylistDynamic,
}
struct Playlist_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PlaylistData>,
}
#[derive(Clone)]
pub struct Playlist(Rc<Playlist_>);
impl Playlist {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGrafana) -> Self {
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
    #[doc = "Set the field `org_id`.\nThe Organization ID. If not set, the Org ID defined in the provider block will be used."]
    pub fn set_org_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().org_id = Some(v.into());
        self
    }
    #[doc = "Set the field `item`.\n"]
    pub fn set_item(self, v: impl Into<BlockAssignable<PlaylistItemEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().item = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.item = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of this resource."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interval", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the playlist."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Organization ID. If not set, the Org ID defined in the provider block will be used."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
}
impl Referable for Playlist {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for Playlist {}
impl ToListMappable for Playlist {
    type O = ListRef<PlaylistRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for Playlist_ {
    fn extract_resource_type(&self) -> String {
        "grafana_playlist".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildPlaylist {
    pub tf_id: String,
    #[doc = ""]
    pub interval: PrimField<String>,
    #[doc = "The name of the playlist."]
    pub name: PrimField<String>,
}
impl BuildPlaylist {
    pub fn build(self, stack: &mut Stack) -> Playlist {
        let out = Playlist(Rc::new(Playlist_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(PlaylistData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                interval: self.interval,
                name: self.name,
                org_id: core::default::Default::default(),
                item: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct PlaylistRef {
    shared: StackShared,
    base: String,
}
impl Ref for PlaylistRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl PlaylistRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of this resource."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interval", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the playlist."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Organization ID. If not set, the Org ID defined in the provider block will be used."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct PlaylistItemEl {
    order: PrimField<f64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl PlaylistItemEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for PlaylistItemEl {
    type O = BlockAssignable<PlaylistItemEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPlaylistItemEl {
    #[doc = ""]
    pub order: PrimField<f64>,
}
impl BuildPlaylistItemEl {
    pub fn build(self) -> PlaylistItemEl {
        PlaylistItemEl {
            order: self.order,
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct PlaylistItemElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PlaylistItemElRef {
    fn new(shared: StackShared, base: String) -> PlaylistItemElRef {
        PlaylistItemElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PlaylistItemElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `order` after provisioning.\n"]
    pub fn order(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct PlaylistDynamic {
    item: Option<DynamicBlock<PlaylistItemEl>>,
}
