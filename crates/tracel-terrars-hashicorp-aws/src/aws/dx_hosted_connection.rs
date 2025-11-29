use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DxHostedConnectionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    bandwidth: PrimField<String>,
    connection_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    owner_account_id: PrimField<String>,
    vlan: PrimField<f64>,
}

struct DxHostedConnection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DxHostedConnectionData>,
}

#[derive(Clone)]
pub struct DxHostedConnection(Rc<DxHostedConnection_>);

impl DxHostedConnection {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `aws_device` after provisioning.\n"]
    pub fn aws_device(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_device", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bandwidth` after provisioning.\n"]
    pub fn bandwidth(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bandwidth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_id` after provisioning.\n"]
    pub fn connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_region` after provisioning.\n"]
    pub fn connection_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `has_logical_redundancy` after provisioning.\n"]
    pub fn has_logical_redundancy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_logical_redundancy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `jumbo_frame_capable` after provisioning.\n"]
    pub fn jumbo_frame_capable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.jumbo_frame_capable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lag_id` after provisioning.\n"]
    pub fn lag_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lag_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `loa_issue_time` after provisioning.\n"]
    pub fn loa_issue_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.loa_issue_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_account_id` after provisioning.\n"]
    pub fn owner_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partner_name` after provisioning.\n"]
    pub fn partner_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.partner_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provider_name` after provisioning.\n"]
    pub fn provider_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vlan` after provisioning.\n"]
    pub fn vlan(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vlan", self.extract_ref()))
    }
}

impl Referable for DxHostedConnection {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DxHostedConnection { }

impl ToListMappable for DxHostedConnection {
    type O = ListRef<DxHostedConnectionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DxHostedConnection_ {
    fn extract_resource_type(&self) -> String {
        "aws_dx_hosted_connection".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDxHostedConnection {
    pub tf_id: String,
    #[doc = ""]
    pub bandwidth: PrimField<String>,
    #[doc = ""]
    pub connection_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub owner_account_id: PrimField<String>,
    #[doc = ""]
    pub vlan: PrimField<f64>,
}

impl BuildDxHostedConnection {
    pub fn build(self, stack: &mut Stack) -> DxHostedConnection {
        let out = DxHostedConnection(Rc::new(DxHostedConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DxHostedConnectionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bandwidth: self.bandwidth,
                connection_id: self.connection_id,
                id: core::default::Default::default(),
                name: self.name,
                owner_account_id: self.owner_account_id,
                vlan: self.vlan,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DxHostedConnectionRef {
    shared: StackShared,
    base: String,
}

impl Ref for DxHostedConnectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DxHostedConnectionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_device` after provisioning.\n"]
    pub fn aws_device(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_device", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bandwidth` after provisioning.\n"]
    pub fn bandwidth(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bandwidth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_id` after provisioning.\n"]
    pub fn connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_region` after provisioning.\n"]
    pub fn connection_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `has_logical_redundancy` after provisioning.\n"]
    pub fn has_logical_redundancy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_logical_redundancy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `jumbo_frame_capable` after provisioning.\n"]
    pub fn jumbo_frame_capable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.jumbo_frame_capable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lag_id` after provisioning.\n"]
    pub fn lag_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lag_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `loa_issue_time` after provisioning.\n"]
    pub fn loa_issue_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.loa_issue_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_account_id` after provisioning.\n"]
    pub fn owner_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partner_name` after provisioning.\n"]
    pub fn partner_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.partner_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provider_name` after provisioning.\n"]
    pub fn provider_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vlan` after provisioning.\n"]
    pub fn vlan(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vlan", self.extract_ref()))
    }
}
