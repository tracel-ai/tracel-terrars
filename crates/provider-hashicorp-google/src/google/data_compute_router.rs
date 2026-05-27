use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeRouterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataComputeRouter_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeRouterData>,
}
#[derive(Clone)]
pub struct DataComputeRouter(Rc<DataComputeRouter_>);
impl DataComputeRouter {
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
    #[doc = "Set the field `region`.\nRegion where the router resides."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `bgp` after provisioning.\nBGP information specific to this router."]
    pub fn bgp(&self) -> ListRef<DataComputeRouterBgpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bgp", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encrypted_interconnect_router` after provisioning.\nIndicates if a router is dedicated for use with encrypted VLAN\nattachments (interconnectAttachments)."]
    pub fn encrypted_interconnect_router(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encrypted_interconnect_router", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `md5_authentication_keys` after provisioning.\nKeys used for MD5 authentication."]
    pub fn md5_authentication_keys(&self) -> ListRef<DataComputeRouterMd5AuthenticationKeysElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.md5_authentication_keys", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?'\nwhich means the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nA reference to the network to which this router belongs."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\nAdditional params passed with the request, but not persisted as part of resource payload"]
    pub fn params(&self) -> ListRef<DataComputeRouterParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where the router resides."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
}
impl Referable for DataComputeRouter {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeRouter {}
impl ToListMappable for DataComputeRouter {
    type O = ListRef<DataComputeRouterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeRouter_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_router".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeRouter {
    pub tf_id: String,
    #[doc = "Name of the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?'\nwhich means the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub name: PrimField<String>,
    #[doc = "A reference to the network to which this router belongs."]
    pub network: PrimField<String>,
}
impl BuildDataComputeRouter {
    pub fn build(self, stack: &mut Stack) -> DataComputeRouter {
        let out = DataComputeRouter(Rc::new(DataComputeRouter_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeRouterData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                network: self.network,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeRouterRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRouterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeRouterRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `bgp` after provisioning.\nBGP information specific to this router."]
    pub fn bgp(&self) -> ListRef<DataComputeRouterBgpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bgp", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encrypted_interconnect_router` after provisioning.\nIndicates if a router is dedicated for use with encrypted VLAN\nattachments (interconnectAttachments)."]
    pub fn encrypted_interconnect_router(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encrypted_interconnect_router", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `md5_authentication_keys` after provisioning.\nKeys used for MD5 authentication."]
    pub fn md5_authentication_keys(&self) -> ListRef<DataComputeRouterMd5AuthenticationKeysElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.md5_authentication_keys", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?'\nwhich means the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nA reference to the network to which this router belongs."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\nAdditional params passed with the request, but not persisted as part of resource payload"]
    pub fn params(&self) -> ListRef<DataComputeRouterParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where the router resides."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRouterBgpElAdvertisedIpRangesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<PrimField<String>>,
}
impl DataComputeRouterBgpElAdvertisedIpRangesEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `range`.\n"]
    pub fn set_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.range = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRouterBgpElAdvertisedIpRangesEl {
    type O = BlockAssignable<DataComputeRouterBgpElAdvertisedIpRangesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRouterBgpElAdvertisedIpRangesEl {}
impl BuildDataComputeRouterBgpElAdvertisedIpRangesEl {
    pub fn build(self) -> DataComputeRouterBgpElAdvertisedIpRangesEl {
        DataComputeRouterBgpElAdvertisedIpRangesEl {
            description: core::default::Default::default(),
            range: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRouterBgpElAdvertisedIpRangesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRouterBgpElAdvertisedIpRangesElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRouterBgpElAdvertisedIpRangesElRef {
        DataComputeRouterBgpElAdvertisedIpRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRouterBgpElAdvertisedIpRangesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `range` after provisioning.\n"]
    pub fn range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.range", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRouterBgpEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    advertise_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advertised_groups: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advertised_ip_ranges: Option<SetField<DataComputeRouterBgpElAdvertisedIpRangesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    asn: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identifier_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keepalive_interval: Option<PrimField<f64>>,
}
impl DataComputeRouterBgpEl {
    #[doc = "Set the field `advertise_mode`.\n"]
    pub fn set_advertise_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.advertise_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `advertised_groups`.\n"]
    pub fn set_advertised_groups(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.advertised_groups = Some(v.into());
        self
    }
    #[doc = "Set the field `advertised_ip_ranges`.\n"]
    pub fn set_advertised_ip_ranges(
        mut self,
        v: impl Into<SetField<DataComputeRouterBgpElAdvertisedIpRangesEl>>,
    ) -> Self {
        self.advertised_ip_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `asn`.\n"]
    pub fn set_asn(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.asn = Some(v.into());
        self
    }
    #[doc = "Set the field `identifier_range`.\n"]
    pub fn set_identifier_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identifier_range = Some(v.into());
        self
    }
    #[doc = "Set the field `keepalive_interval`.\n"]
    pub fn set_keepalive_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.keepalive_interval = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRouterBgpEl {
    type O = BlockAssignable<DataComputeRouterBgpEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRouterBgpEl {}
impl BuildDataComputeRouterBgpEl {
    pub fn build(self) -> DataComputeRouterBgpEl {
        DataComputeRouterBgpEl {
            advertise_mode: core::default::Default::default(),
            advertised_groups: core::default::Default::default(),
            advertised_ip_ranges: core::default::Default::default(),
            asn: core::default::Default::default(),
            identifier_range: core::default::Default::default(),
            keepalive_interval: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRouterBgpElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRouterBgpElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRouterBgpElRef {
        DataComputeRouterBgpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRouterBgpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `advertise_mode` after provisioning.\n"]
    pub fn advertise_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.advertise_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `advertised_groups` after provisioning.\n"]
    pub fn advertised_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advertised_groups", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `advertised_ip_ranges` after provisioning.\n"]
    pub fn advertised_ip_ranges(&self) -> SetRef<DataComputeRouterBgpElAdvertisedIpRangesElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.advertised_ip_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `asn` after provisioning.\n"]
    pub fn asn(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.asn", self.base))
    }
    #[doc = "Get a reference to the value of field `identifier_range` after provisioning.\n"]
    pub fn identifier_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identifier_range", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `keepalive_interval` after provisioning.\n"]
    pub fn keepalive_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.keepalive_interval", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRouterMd5AuthenticationKeysEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataComputeRouterMd5AuthenticationKeysEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRouterMd5AuthenticationKeysEl {
    type O = BlockAssignable<DataComputeRouterMd5AuthenticationKeysEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRouterMd5AuthenticationKeysEl {}
impl BuildDataComputeRouterMd5AuthenticationKeysEl {
    pub fn build(self) -> DataComputeRouterMd5AuthenticationKeysEl {
        DataComputeRouterMd5AuthenticationKeysEl {
            key: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRouterMd5AuthenticationKeysElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRouterMd5AuthenticationKeysElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRouterMd5AuthenticationKeysElRef {
        DataComputeRouterMd5AuthenticationKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRouterMd5AuthenticationKeysElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRouterParamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
}
impl DataComputeRouterParamsEl {
    #[doc = "Set the field `resource_manager_tags`.\n"]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRouterParamsEl {
    type O = BlockAssignable<DataComputeRouterParamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRouterParamsEl {}
impl BuildDataComputeRouterParamsEl {
    pub fn build(self) -> DataComputeRouterParamsEl {
        DataComputeRouterParamsEl {
            resource_manager_tags: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRouterParamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRouterParamsElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRouterParamsElRef {
        DataComputeRouterParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRouterParamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\n"]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_manager_tags", self.base),
        )
    }
}
