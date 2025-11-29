use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DirectoryServiceTrustData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditional_forwarder_ip_addrs: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_associated_conditional_forwarder: Option<PrimField<bool>>,
    directory_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    remote_domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selective_auth: Option<PrimField<String>>,
    trust_direction: PrimField<String>,
    trust_password: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trust_type: Option<PrimField<String>>,
}

struct DirectoryServiceTrust_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DirectoryServiceTrustData>,
}

#[derive(Clone)]
pub struct DirectoryServiceTrust(Rc<DirectoryServiceTrust_>);

impl DirectoryServiceTrust {
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

    #[doc = "Set the field `conditional_forwarder_ip_addrs`.\n"]
    pub fn set_conditional_forwarder_ip_addrs(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().conditional_forwarder_ip_addrs = Some(v.into());
        self
    }

    #[doc = "Set the field `delete_associated_conditional_forwarder`.\n"]
    pub fn set_delete_associated_conditional_forwarder(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_associated_conditional_forwarder = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `selective_auth`.\n"]
    pub fn set_selective_auth(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().selective_auth = Some(v.into());
        self
    }

    #[doc = "Set the field `trust_type`.\n"]
    pub fn set_trust_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().trust_type = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `conditional_forwarder_ip_addrs` after provisioning.\n"]
    pub fn conditional_forwarder_ip_addrs(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.conditional_forwarder_ip_addrs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_date_time` after provisioning.\n"]
    pub fn created_date_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_date_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_associated_conditional_forwarder` after provisioning.\n"]
    pub fn delete_associated_conditional_forwarder(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_associated_conditional_forwarder", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date_time` after provisioning.\n"]
    pub fn last_updated_date_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_domain_name` after provisioning.\n"]
    pub fn remote_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.remote_domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `selective_auth` after provisioning.\n"]
    pub fn selective_auth(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.selective_auth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state_last_updated_date_time` after provisioning.\n"]
    pub fn state_last_updated_date_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state_last_updated_date_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trust_direction` after provisioning.\n"]
    pub fn trust_direction(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trust_direction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trust_password` after provisioning.\n"]
    pub fn trust_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trust_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trust_state` after provisioning.\n"]
    pub fn trust_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trust_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trust_state_reason` after provisioning.\n"]
    pub fn trust_state_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trust_state_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trust_type` after provisioning.\n"]
    pub fn trust_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trust_type", self.extract_ref()))
    }
}

impl Referable for DirectoryServiceTrust {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DirectoryServiceTrust { }

impl ToListMappable for DirectoryServiceTrust {
    type O = ListRef<DirectoryServiceTrustRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DirectoryServiceTrust_ {
    fn extract_resource_type(&self) -> String {
        "aws_directory_service_trust".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDirectoryServiceTrust {
    pub tf_id: String,
    #[doc = ""]
    pub directory_id: PrimField<String>,
    #[doc = ""]
    pub remote_domain_name: PrimField<String>,
    #[doc = ""]
    pub trust_direction: PrimField<String>,
    #[doc = ""]
    pub trust_password: PrimField<String>,
}

impl BuildDirectoryServiceTrust {
    pub fn build(self, stack: &mut Stack) -> DirectoryServiceTrust {
        let out = DirectoryServiceTrust(Rc::new(DirectoryServiceTrust_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DirectoryServiceTrustData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                conditional_forwarder_ip_addrs: core::default::Default::default(),
                delete_associated_conditional_forwarder: core::default::Default::default(),
                directory_id: self.directory_id,
                region: core::default::Default::default(),
                remote_domain_name: self.remote_domain_name,
                selective_auth: core::default::Default::default(),
                trust_direction: self.trust_direction,
                trust_password: self.trust_password,
                trust_type: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DirectoryServiceTrustRef {
    shared: StackShared,
    base: String,
}

impl Ref for DirectoryServiceTrustRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DirectoryServiceTrustRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `conditional_forwarder_ip_addrs` after provisioning.\n"]
    pub fn conditional_forwarder_ip_addrs(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.conditional_forwarder_ip_addrs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_date_time` after provisioning.\n"]
    pub fn created_date_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_date_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_associated_conditional_forwarder` after provisioning.\n"]
    pub fn delete_associated_conditional_forwarder(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_associated_conditional_forwarder", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date_time` after provisioning.\n"]
    pub fn last_updated_date_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_domain_name` after provisioning.\n"]
    pub fn remote_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.remote_domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `selective_auth` after provisioning.\n"]
    pub fn selective_auth(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.selective_auth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state_last_updated_date_time` after provisioning.\n"]
    pub fn state_last_updated_date_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state_last_updated_date_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trust_direction` after provisioning.\n"]
    pub fn trust_direction(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trust_direction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trust_password` after provisioning.\n"]
    pub fn trust_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trust_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trust_state` after provisioning.\n"]
    pub fn trust_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trust_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trust_state_reason` after provisioning.\n"]
    pub fn trust_state_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trust_state_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trust_type` after provisioning.\n"]
    pub fn trust_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trust_type", self.extract_ref()))
    }
}
