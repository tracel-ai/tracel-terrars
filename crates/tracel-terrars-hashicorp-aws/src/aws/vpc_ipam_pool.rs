use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VpcIpamPoolData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    address_family: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allocation_default_netmask_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allocation_max_netmask_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allocation_min_netmask_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allocation_resource_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_import: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cascade: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    ipam_scope_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locale: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_ip_source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publicly_advertisable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_ipam_pool_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VpcIpamPoolTimeoutsEl>,
}

struct VpcIpamPool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VpcIpamPoolData>,
}

#[derive(Clone)]
pub struct VpcIpamPool(Rc<VpcIpamPool_>);

impl VpcIpamPool {
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

    #[doc = "Set the field `allocation_default_netmask_length`.\n"]
    pub fn set_allocation_default_netmask_length(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().allocation_default_netmask_length = Some(v.into());
        self
    }

    #[doc = "Set the field `allocation_max_netmask_length`.\n"]
    pub fn set_allocation_max_netmask_length(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().allocation_max_netmask_length = Some(v.into());
        self
    }

    #[doc = "Set the field `allocation_min_netmask_length`.\n"]
    pub fn set_allocation_min_netmask_length(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().allocation_min_netmask_length = Some(v.into());
        self
    }

    #[doc = "Set the field `allocation_resource_tags`.\n"]
    pub fn set_allocation_resource_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().allocation_resource_tags = Some(v.into());
        self
    }

    #[doc = "Set the field `auto_import`.\n"]
    pub fn set_auto_import(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_import = Some(v.into());
        self
    }

    #[doc = "Set the field `aws_service`.\n"]
    pub fn set_aws_service(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().aws_service = Some(v.into());
        self
    }

    #[doc = "Set the field `cascade`.\n"]
    pub fn set_cascade(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().cascade = Some(v.into());
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

    #[doc = "Set the field `locale`.\n"]
    pub fn set_locale(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().locale = Some(v.into());
        self
    }

    #[doc = "Set the field `public_ip_source`.\n"]
    pub fn set_public_ip_source(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().public_ip_source = Some(v.into());
        self
    }

    #[doc = "Set the field `publicly_advertisable`.\n"]
    pub fn set_publicly_advertisable(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().publicly_advertisable = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `source_ipam_pool_id`.\n"]
    pub fn set_source_ipam_pool_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_ipam_pool_id = Some(v.into());
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VpcIpamPoolTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `address_family` after provisioning.\n"]
    pub fn address_family(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allocation_default_netmask_length` after provisioning.\n"]
    pub fn allocation_default_netmask_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.allocation_default_netmask_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allocation_max_netmask_length` after provisioning.\n"]
    pub fn allocation_max_netmask_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.allocation_max_netmask_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allocation_min_netmask_length` after provisioning.\n"]
    pub fn allocation_min_netmask_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.allocation_min_netmask_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allocation_resource_tags` after provisioning.\n"]
    pub fn allocation_resource_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.allocation_resource_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_import` after provisioning.\n"]
    pub fn auto_import(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_import", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_service` after provisioning.\n"]
    pub fn aws_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cascade` after provisioning.\n"]
    pub fn cascade(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cascade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_scope_id` after provisioning.\n"]
    pub fn ipam_scope_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipam_scope_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_scope_type` after provisioning.\n"]
    pub fn ipam_scope_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipam_scope_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locale` after provisioning.\n"]
    pub fn locale(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.locale", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pool_depth` after provisioning.\n"]
    pub fn pool_depth(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.pool_depth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_ip_source` after provisioning.\n"]
    pub fn public_ip_source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_ip_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publicly_advertisable` after provisioning.\n"]
    pub fn publicly_advertisable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.publicly_advertisable", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_ipam_pool_id` after provisioning.\n"]
    pub fn source_ipam_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_ipam_pool_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcIpamPoolTimeoutsElRef {
        VpcIpamPoolTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for VpcIpamPool {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for VpcIpamPool { }

impl ToListMappable for VpcIpamPool {
    type O = ListRef<VpcIpamPoolRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for VpcIpamPool_ {
    fn extract_resource_type(&self) -> String {
        "aws_vpc_ipam_pool".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVpcIpamPool {
    pub tf_id: String,
    #[doc = ""]
    pub address_family: PrimField<String>,
    #[doc = ""]
    pub ipam_scope_id: PrimField<String>,
}

impl BuildVpcIpamPool {
    pub fn build(self, stack: &mut Stack) -> VpcIpamPool {
        let out = VpcIpamPool(Rc::new(VpcIpamPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VpcIpamPoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                address_family: self.address_family,
                allocation_default_netmask_length: core::default::Default::default(),
                allocation_max_netmask_length: core::default::Default::default(),
                allocation_min_netmask_length: core::default::Default::default(),
                allocation_resource_tags: core::default::Default::default(),
                auto_import: core::default::Default::default(),
                aws_service: core::default::Default::default(),
                cascade: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                ipam_scope_id: self.ipam_scope_id,
                locale: core::default::Default::default(),
                public_ip_source: core::default::Default::default(),
                publicly_advertisable: core::default::Default::default(),
                region: core::default::Default::default(),
                source_ipam_pool_id: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VpcIpamPoolRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpcIpamPoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl VpcIpamPoolRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_family` after provisioning.\n"]
    pub fn address_family(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allocation_default_netmask_length` after provisioning.\n"]
    pub fn allocation_default_netmask_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.allocation_default_netmask_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allocation_max_netmask_length` after provisioning.\n"]
    pub fn allocation_max_netmask_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.allocation_max_netmask_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allocation_min_netmask_length` after provisioning.\n"]
    pub fn allocation_min_netmask_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.allocation_min_netmask_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allocation_resource_tags` after provisioning.\n"]
    pub fn allocation_resource_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.allocation_resource_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_import` after provisioning.\n"]
    pub fn auto_import(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_import", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_service` after provisioning.\n"]
    pub fn aws_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cascade` after provisioning.\n"]
    pub fn cascade(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cascade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_scope_id` after provisioning.\n"]
    pub fn ipam_scope_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipam_scope_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_scope_type` after provisioning.\n"]
    pub fn ipam_scope_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipam_scope_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locale` after provisioning.\n"]
    pub fn locale(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.locale", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pool_depth` after provisioning.\n"]
    pub fn pool_depth(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.pool_depth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_ip_source` after provisioning.\n"]
    pub fn public_ip_source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_ip_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publicly_advertisable` after provisioning.\n"]
    pub fn publicly_advertisable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.publicly_advertisable", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_ipam_pool_id` after provisioning.\n"]
    pub fn source_ipam_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_ipam_pool_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcIpamPoolTimeoutsElRef {
        VpcIpamPoolTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct VpcIpamPoolTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl VpcIpamPoolTimeoutsEl {
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

impl ToListMappable for VpcIpamPoolTimeoutsEl {
    type O = BlockAssignable<VpcIpamPoolTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcIpamPoolTimeoutsEl {}

impl BuildVpcIpamPoolTimeoutsEl {
    pub fn build(self) -> VpcIpamPoolTimeoutsEl {
        VpcIpamPoolTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct VpcIpamPoolTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpcIpamPoolTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VpcIpamPoolTimeoutsElRef {
        VpcIpamPoolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcIpamPoolTimeoutsElRef {
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
