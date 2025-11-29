use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DxConnectionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    bandwidth: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_macsec: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
}

struct DxConnection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DxConnectionData>,
}

#[derive(Clone)]
pub struct DxConnection(Rc<DxConnection_>);

impl DxConnection {
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

    #[doc = "Set the field `encryption_mode`.\n"]
    pub fn set_encryption_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().encryption_mode = Some(v.into());
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `request_macsec`.\n"]
    pub fn set_request_macsec(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().request_macsec = Some(v.into());
        self
    }

    #[doc = "Set the field `skip_destroy`.\n"]
    pub fn set_skip_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_destroy = Some(v.into());
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_device` after provisioning.\n"]
    pub fn aws_device(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_device", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bandwidth` after provisioning.\n"]
    pub fn bandwidth(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bandwidth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_mode` after provisioning.\n"]
    pub fn encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_mode", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `macsec_capable` after provisioning.\n"]
    pub fn macsec_capable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.macsec_capable", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `port_encryption_status` after provisioning.\n"]
    pub fn port_encryption_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port_encryption_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provider_name` after provisioning.\n"]
    pub fn provider_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_macsec` after provisioning.\n"]
    pub fn request_macsec(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_macsec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skip_destroy` after provisioning.\n"]
    pub fn skip_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.skip_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vlan_id` after provisioning.\n"]
    pub fn vlan_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vlan_id", self.extract_ref()))
    }
}

impl Referable for DxConnection {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DxConnection { }

impl ToListMappable for DxConnection {
    type O = ListRef<DxConnectionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DxConnection_ {
    fn extract_resource_type(&self) -> String {
        "aws_dx_connection".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDxConnection {
    pub tf_id: String,
    #[doc = ""]
    pub bandwidth: PrimField<String>,
    #[doc = ""]
    pub location: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildDxConnection {
    pub fn build(self, stack: &mut Stack) -> DxConnection {
        let out = DxConnection(Rc::new(DxConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DxConnectionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bandwidth: self.bandwidth,
                encryption_mode: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                provider_name: core::default::Default::default(),
                region: core::default::Default::default(),
                request_macsec: core::default::Default::default(),
                skip_destroy: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DxConnectionRef {
    shared: StackShared,
    base: String,
}

impl Ref for DxConnectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DxConnectionRef {
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

    #[doc = "Get a reference to the value of field `aws_device` after provisioning.\n"]
    pub fn aws_device(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_device", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bandwidth` after provisioning.\n"]
    pub fn bandwidth(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bandwidth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_mode` after provisioning.\n"]
    pub fn encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_mode", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `macsec_capable` after provisioning.\n"]
    pub fn macsec_capable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.macsec_capable", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `port_encryption_status` after provisioning.\n"]
    pub fn port_encryption_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port_encryption_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provider_name` after provisioning.\n"]
    pub fn provider_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_macsec` after provisioning.\n"]
    pub fn request_macsec(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_macsec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skip_destroy` after provisioning.\n"]
    pub fn skip_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.skip_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vlan_id` after provisioning.\n"]
    pub fn vlan_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vlan_id", self.extract_ref()))
    }
}
