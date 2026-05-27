use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataMonitoringUptimeCheckIpsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}
struct DataMonitoringUptimeCheckIps_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataMonitoringUptimeCheckIpsData>,
}
#[derive(Clone)]
pub struct DataMonitoringUptimeCheckIps(Rc<DataMonitoringUptimeCheckIps_>);
impl DataMonitoringUptimeCheckIps {
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `uptime_check_ips` after provisioning.\n"]
    pub fn uptime_check_ips(&self) -> ListRef<DataMonitoringUptimeCheckIpsUptimeCheckIpsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.uptime_check_ips", self.extract_ref()),
        )
    }
}
impl Referable for DataMonitoringUptimeCheckIps {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataMonitoringUptimeCheckIps {}
impl ToListMappable for DataMonitoringUptimeCheckIps {
    type O = ListRef<DataMonitoringUptimeCheckIpsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataMonitoringUptimeCheckIps_ {
    fn extract_datasource_type(&self) -> String {
        "google_monitoring_uptime_check_ips".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataMonitoringUptimeCheckIps {
    pub tf_id: String,
}
impl BuildDataMonitoringUptimeCheckIps {
    pub fn build(self, stack: &mut Stack) -> DataMonitoringUptimeCheckIps {
        let out = DataMonitoringUptimeCheckIps(Rc::new(DataMonitoringUptimeCheckIps_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataMonitoringUptimeCheckIpsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataMonitoringUptimeCheckIpsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMonitoringUptimeCheckIpsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataMonitoringUptimeCheckIpsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `uptime_check_ips` after provisioning.\n"]
    pub fn uptime_check_ips(&self) -> ListRef<DataMonitoringUptimeCheckIpsUptimeCheckIpsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.uptime_check_ips", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataMonitoringUptimeCheckIpsUptimeCheckIpsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
impl DataMonitoringUptimeCheckIpsUptimeCheckIpsEl {
    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
}
impl ToListMappable for DataMonitoringUptimeCheckIpsUptimeCheckIpsEl {
    type O = BlockAssignable<DataMonitoringUptimeCheckIpsUptimeCheckIpsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMonitoringUptimeCheckIpsUptimeCheckIpsEl {}
impl BuildDataMonitoringUptimeCheckIpsUptimeCheckIpsEl {
    pub fn build(self) -> DataMonitoringUptimeCheckIpsUptimeCheckIpsEl {
        DataMonitoringUptimeCheckIpsUptimeCheckIpsEl {
            ip_address: core::default::Default::default(),
            location: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}
pub struct DataMonitoringUptimeCheckIpsUptimeCheckIpsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMonitoringUptimeCheckIpsUptimeCheckIpsElRef {
    fn new(shared: StackShared, base: String) -> DataMonitoringUptimeCheckIpsUptimeCheckIpsElRef {
        DataMonitoringUptimeCheckIpsUptimeCheckIpsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMonitoringUptimeCheckIpsUptimeCheckIpsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}
