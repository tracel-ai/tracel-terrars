use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NetworkfirewallVpcEndpointAssociationData {
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
    firewall_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    vpc_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_mapping: Option<Vec<NetworkfirewallVpcEndpointAssociationSubnetMappingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkfirewallVpcEndpointAssociationTimeoutsEl>,
    dynamic: NetworkfirewallVpcEndpointAssociationDynamic,
}

struct NetworkfirewallVpcEndpointAssociation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkfirewallVpcEndpointAssociationData>,
}

#[derive(Clone)]
pub struct NetworkfirewallVpcEndpointAssociation(Rc<NetworkfirewallVpcEndpointAssociation_>);

impl NetworkfirewallVpcEndpointAssociation {
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

    #[doc = "Set the field `subnet_mapping`.\n"]
    pub fn set_subnet_mapping(
        self,
        v: impl Into<BlockAssignable<NetworkfirewallVpcEndpointAssociationSubnetMappingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().subnet_mapping = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.subnet_mapping = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkfirewallVpcEndpointAssociationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_arn` after provisioning.\n"]
    pub fn firewall_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.firewall_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `vpc_endpoint_association_arn` after provisioning.\n"]
    pub fn vpc_endpoint_association_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_association_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_association_id` after provisioning.\n"]
    pub fn vpc_endpoint_association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_association_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_association_status` after provisioning.\n"]
    pub fn vpc_endpoint_association_status(
        &self,
    ) -> ListRef<NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_endpoint_association_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_mapping` after provisioning.\n"]
    pub fn subnet_mapping(&self) -> ListRef<NetworkfirewallVpcEndpointAssociationSubnetMappingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.subnet_mapping", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkfirewallVpcEndpointAssociationTimeoutsElRef {
        NetworkfirewallVpcEndpointAssociationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for NetworkfirewallVpcEndpointAssociation {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for NetworkfirewallVpcEndpointAssociation { }

impl ToListMappable for NetworkfirewallVpcEndpointAssociation {
    type O = ListRef<NetworkfirewallVpcEndpointAssociationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for NetworkfirewallVpcEndpointAssociation_ {
    fn extract_resource_type(&self) -> String {
        "aws_networkfirewall_vpc_endpoint_association".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkfirewallVpcEndpointAssociation {
    pub tf_id: String,
    #[doc = ""]
    pub firewall_arn: PrimField<String>,
    #[doc = ""]
    pub vpc_id: PrimField<String>,
}

impl BuildNetworkfirewallVpcEndpointAssociation {
    pub fn build(self, stack: &mut Stack) -> NetworkfirewallVpcEndpointAssociation {
        let out = NetworkfirewallVpcEndpointAssociation(Rc::new(NetworkfirewallVpcEndpointAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkfirewallVpcEndpointAssociationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                firewall_arn: self.firewall_arn,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                vpc_id: self.vpc_id,
                subnet_mapping: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkfirewallVpcEndpointAssociationRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallVpcEndpointAssociationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl NetworkfirewallVpcEndpointAssociationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_arn` after provisioning.\n"]
    pub fn firewall_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.firewall_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `vpc_endpoint_association_arn` after provisioning.\n"]
    pub fn vpc_endpoint_association_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_association_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_association_id` after provisioning.\n"]
    pub fn vpc_endpoint_association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_association_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_association_status` after provisioning.\n"]
    pub fn vpc_endpoint_association_status(
        &self,
    ) -> ListRef<NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_endpoint_association_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_mapping` after provisioning.\n"]
    pub fn subnet_mapping(&self) -> ListRef<NetworkfirewallVpcEndpointAssociationSubnetMappingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.subnet_mapping", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkfirewallVpcEndpointAssociationTimeoutsElRef {
        NetworkfirewallVpcEndpointAssociationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateElAttachmentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_id: Option<PrimField<String>>,
}

impl NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateElAttachmentEl {
    #[doc = "Set the field `endpoint_id`.\n"]
    pub fn set_endpoint_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint_id = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }

    #[doc = "Set the field `status_message`.\n"]
    pub fn set_status_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status_message = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnet_id = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateElAttachmentEl {
    type O =
        BlockAssignable<
            NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateElAttachmentEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateElAttachmentEl {}

impl BuildNetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateElAttachmentEl {
    pub fn build(
        self,
    ) -> NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateElAttachmentEl {
        NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateElAttachmentEl {
            endpoint_id: core::default::Default::default(),
            status: core::default::Default::default(),
            status_message: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateElAttachmentElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateElAttachmentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateElAttachmentElRef {
        NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateElAttachmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateElAttachmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_id", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attachment: Option<
        ListField<
            NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateElAttachmentEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone: Option<PrimField<String>>,
}

impl NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateEl {
    #[doc = "Set the field `attachment`.\n"]
    pub fn set_attachment(
        mut self,
        v:
            impl

                    Into<
                        ListField<
                            NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateElAttachmentEl,
                        >,
                    >,
    ) -> Self {
        self.attachment = Some(v.into());
        self
    }

    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.availability_zone = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateEl {
    type O =
        BlockAssignable<NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateEl {}

impl BuildNetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateEl {
    pub fn build(self) -> NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateEl {
        NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateEl {
            attachment: core::default::Default::default(),
            availability_zone: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateElRef {
        NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attachment` after provisioning.\n"]
    pub fn attachment(
        &self,
    ) -> ListRef<
        NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateElAttachmentElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.attachment", self.base))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    association_sync_state: Option<
        SetField<NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateEl>,
    >,
}

impl NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusEl {
    #[doc = "Set the field `association_sync_state`.\n"]
    pub fn set_association_sync_state(
        mut self,
        v:
            impl

                    Into<
                        SetField<
                            NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateEl,
                        >,
                    >,
    ) -> Self {
        self.association_sync_state = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusEl {
    type O = BlockAssignable<NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusEl {}

impl BuildNetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusEl {
    pub fn build(self) -> NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusEl {
        NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusEl {
            association_sync_state: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElRef {
        NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `association_sync_state` after provisioning.\n"]
    pub fn association_sync_state(
        &self,
    ) -> SetRef<NetworkfirewallVpcEndpointAssociationVpcEndpointAssociationStatusElAssociationSyncStateElRef> {
        SetRef::new(self.shared().clone(), format!("{}.association_sync_state", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallVpcEndpointAssociationSubnetMappingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address_type: Option<PrimField<String>>,
    subnet_id: PrimField<String>,
}

impl NetworkfirewallVpcEndpointAssociationSubnetMappingEl {
    #[doc = "Set the field `ip_address_type`.\n"]
    pub fn set_ip_address_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address_type = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkfirewallVpcEndpointAssociationSubnetMappingEl {
    type O = BlockAssignable<NetworkfirewallVpcEndpointAssociationSubnetMappingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallVpcEndpointAssociationSubnetMappingEl {
    #[doc = ""]
    pub subnet_id: PrimField<String>,
}

impl BuildNetworkfirewallVpcEndpointAssociationSubnetMappingEl {
    pub fn build(self) -> NetworkfirewallVpcEndpointAssociationSubnetMappingEl {
        NetworkfirewallVpcEndpointAssociationSubnetMappingEl {
            ip_address_type: core::default::Default::default(),
            subnet_id: self.subnet_id,
        }
    }
}

pub struct NetworkfirewallVpcEndpointAssociationSubnetMappingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallVpcEndpointAssociationSubnetMappingElRef {
    fn new(shared: StackShared, base: String) -> NetworkfirewallVpcEndpointAssociationSubnetMappingElRef {
        NetworkfirewallVpcEndpointAssociationSubnetMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallVpcEndpointAssociationSubnetMappingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallVpcEndpointAssociationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl NetworkfirewallVpcEndpointAssociationTimeoutsEl {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkfirewallVpcEndpointAssociationTimeoutsEl {
    type O = BlockAssignable<NetworkfirewallVpcEndpointAssociationTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallVpcEndpointAssociationTimeoutsEl {}

impl BuildNetworkfirewallVpcEndpointAssociationTimeoutsEl {
    pub fn build(self) -> NetworkfirewallVpcEndpointAssociationTimeoutsEl {
        NetworkfirewallVpcEndpointAssociationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallVpcEndpointAssociationTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallVpcEndpointAssociationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkfirewallVpcEndpointAssociationTimeoutsElRef {
        NetworkfirewallVpcEndpointAssociationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallVpcEndpointAssociationTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallVpcEndpointAssociationDynamic {
    subnet_mapping: Option<DynamicBlock<NetworkfirewallVpcEndpointAssociationSubnetMappingEl>>,
}
