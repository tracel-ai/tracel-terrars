use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataNetworkmanagerLinkData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    global_network_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    link_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}
struct DataNetworkmanagerLink_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataNetworkmanagerLinkData>,
}
#[derive(Clone)]
pub struct DataNetworkmanagerLink(Rc<DataNetworkmanagerLink_>);
impl DataNetworkmanagerLink {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `bandwidth` after provisioning.\n"]
    pub fn bandwidth(&self) -> ListRef<DataNetworkmanagerLinkBandwidthElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bandwidth", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `global_network_id` after provisioning.\n"]
    pub fn global_network_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.global_network_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `link_id` after provisioning.\n"]
    pub fn link_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.link_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `provider_name` after provisioning.\n"]
    pub fn provider_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provider_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `site_id` after provisioning.\n"]
    pub fn site_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.site_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
}
impl Referable for DataNetworkmanagerLink {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataNetworkmanagerLink {}
impl ToListMappable for DataNetworkmanagerLink {
    type O = ListRef<DataNetworkmanagerLinkRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataNetworkmanagerLink_ {
    fn extract_datasource_type(&self) -> String {
        "aws_networkmanager_link".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataNetworkmanagerLink {
    pub tf_id: String,
    #[doc = ""]
    pub global_network_id: PrimField<String>,
    #[doc = ""]
    pub link_id: PrimField<String>,
}
impl BuildDataNetworkmanagerLink {
    pub fn build(self, stack: &mut Stack) -> DataNetworkmanagerLink {
        let out = DataNetworkmanagerLink(Rc::new(DataNetworkmanagerLink_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataNetworkmanagerLinkData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                global_network_id: self.global_network_id,
                id: core::default::Default::default(),
                link_id: self.link_id,
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataNetworkmanagerLinkRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataNetworkmanagerLinkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataNetworkmanagerLinkRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `bandwidth` after provisioning.\n"]
    pub fn bandwidth(&self) -> ListRef<DataNetworkmanagerLinkBandwidthElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bandwidth", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `global_network_id` after provisioning.\n"]
    pub fn global_network_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.global_network_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `link_id` after provisioning.\n"]
    pub fn link_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.link_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `provider_name` after provisioning.\n"]
    pub fn provider_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provider_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `site_id` after provisioning.\n"]
    pub fn site_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.site_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataNetworkmanagerLinkBandwidthEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    download_speed: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upload_speed: Option<PrimField<f64>>,
}
impl DataNetworkmanagerLinkBandwidthEl {
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
impl ToListMappable for DataNetworkmanagerLinkBandwidthEl {
    type O = BlockAssignable<DataNetworkmanagerLinkBandwidthEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataNetworkmanagerLinkBandwidthEl {}
impl BuildDataNetworkmanagerLinkBandwidthEl {
    pub fn build(self) -> DataNetworkmanagerLinkBandwidthEl {
        DataNetworkmanagerLinkBandwidthEl {
            download_speed: core::default::Default::default(),
            upload_speed: core::default::Default::default(),
        }
    }
}
pub struct DataNetworkmanagerLinkBandwidthElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataNetworkmanagerLinkBandwidthElRef {
    fn new(shared: StackShared, base: String) -> DataNetworkmanagerLinkBandwidthElRef {
        DataNetworkmanagerLinkBandwidthElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataNetworkmanagerLinkBandwidthElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `download_speed` after provisioning.\n"]
    pub fn download_speed(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.download_speed", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `upload_speed` after provisioning.\n"]
    pub fn upload_speed(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.upload_speed", self.base))
    }
}
