use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VpcRouteServerPeerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    peer_address: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    route_server_endpoint_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bgp_options: Option<Vec<VpcRouteServerPeerBgpOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VpcRouteServerPeerTimeoutsEl>,
    dynamic: VpcRouteServerPeerDynamic,
}
struct VpcRouteServerPeer_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VpcRouteServerPeerData>,
}
#[derive(Clone)]
pub struct VpcRouteServerPeer(Rc<VpcRouteServerPeer_>);
impl VpcRouteServerPeer {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `bgp_options`.\n"]
    pub fn set_bgp_options(
        self,
        v: impl Into<BlockAssignable<VpcRouteServerPeerBgpOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bgp_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bgp_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VpcRouteServerPeerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `endpoint_eni_address` after provisioning.\n"]
    pub fn endpoint_eni_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_eni_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_eni_id` after provisioning.\n"]
    pub fn endpoint_eni_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_eni_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peer_address` after provisioning.\n"]
    pub fn peer_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `route_server_endpoint_id` after provisioning.\n"]
    pub fn route_server_endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.route_server_endpoint_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `route_server_id` after provisioning.\n"]
    pub fn route_server_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.route_server_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `route_server_peer_id` after provisioning.\n"]
    pub fn route_server_peer_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.route_server_peer_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnet_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bgp_options` after provisioning.\n"]
    pub fn bgp_options(&self) -> ListRef<VpcRouteServerPeerBgpOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bgp_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcRouteServerPeerTimeoutsElRef {
        VpcRouteServerPeerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for VpcRouteServerPeer {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for VpcRouteServerPeer {}
impl ToListMappable for VpcRouteServerPeer {
    type O = ListRef<VpcRouteServerPeerRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for VpcRouteServerPeer_ {
    fn extract_resource_type(&self) -> String {
        "aws_vpc_route_server_peer".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVpcRouteServerPeer {
    pub tf_id: String,
    #[doc = ""]
    pub peer_address: PrimField<String>,
    #[doc = ""]
    pub route_server_endpoint_id: PrimField<String>,
}
impl BuildVpcRouteServerPeer {
    pub fn build(self, stack: &mut Stack) -> VpcRouteServerPeer {
        let out = VpcRouteServerPeer(Rc::new(VpcRouteServerPeer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VpcRouteServerPeerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                peer_address: self.peer_address,
                region: core::default::Default::default(),
                route_server_endpoint_id: self.route_server_endpoint_id,
                tags: core::default::Default::default(),
                bgp_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct VpcRouteServerPeerRef {
    shared: StackShared,
    base: String,
}
impl Ref for VpcRouteServerPeerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VpcRouteServerPeerRef {
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
    #[doc = "Get a reference to the value of field `endpoint_eni_address` after provisioning.\n"]
    pub fn endpoint_eni_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_eni_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_eni_id` after provisioning.\n"]
    pub fn endpoint_eni_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_eni_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peer_address` after provisioning.\n"]
    pub fn peer_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `route_server_endpoint_id` after provisioning.\n"]
    pub fn route_server_endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.route_server_endpoint_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `route_server_id` after provisioning.\n"]
    pub fn route_server_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.route_server_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `route_server_peer_id` after provisioning.\n"]
    pub fn route_server_peer_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.route_server_peer_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnet_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bgp_options` after provisioning.\n"]
    pub fn bgp_options(&self) -> ListRef<VpcRouteServerPeerBgpOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bgp_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcRouteServerPeerTimeoutsElRef {
        VpcRouteServerPeerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct VpcRouteServerPeerBgpOptionsEl {
    peer_asn: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_liveness_detection: Option<PrimField<String>>,
}
impl VpcRouteServerPeerBgpOptionsEl {
    #[doc = "Set the field `peer_liveness_detection`.\n"]
    pub fn set_peer_liveness_detection(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.peer_liveness_detection = Some(v.into());
        self
    }
}
impl ToListMappable for VpcRouteServerPeerBgpOptionsEl {
    type O = BlockAssignable<VpcRouteServerPeerBgpOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVpcRouteServerPeerBgpOptionsEl {
    #[doc = ""]
    pub peer_asn: PrimField<f64>,
}
impl BuildVpcRouteServerPeerBgpOptionsEl {
    pub fn build(self) -> VpcRouteServerPeerBgpOptionsEl {
        VpcRouteServerPeerBgpOptionsEl {
            peer_asn: self.peer_asn,
            peer_liveness_detection: core::default::Default::default(),
        }
    }
}
pub struct VpcRouteServerPeerBgpOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VpcRouteServerPeerBgpOptionsElRef {
    fn new(shared: StackShared, base: String) -> VpcRouteServerPeerBgpOptionsElRef {
        VpcRouteServerPeerBgpOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VpcRouteServerPeerBgpOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `peer_asn` after provisioning.\n"]
    pub fn peer_asn(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.peer_asn", self.base))
    }
    #[doc = "Get a reference to the value of field `peer_liveness_detection` after provisioning.\n"]
    pub fn peer_liveness_detection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_liveness_detection", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VpcRouteServerPeerTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl VpcRouteServerPeerTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}
impl ToListMappable for VpcRouteServerPeerTimeoutsEl {
    type O = BlockAssignable<VpcRouteServerPeerTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVpcRouteServerPeerTimeoutsEl {}
impl BuildVpcRouteServerPeerTimeoutsEl {
    pub fn build(self) -> VpcRouteServerPeerTimeoutsEl {
        VpcRouteServerPeerTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct VpcRouteServerPeerTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VpcRouteServerPeerTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VpcRouteServerPeerTimeoutsElRef {
        VpcRouteServerPeerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VpcRouteServerPeerTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}
#[derive(Serialize, Default)]
struct VpcRouteServerPeerDynamic {
    bgp_options: Option<DynamicBlock<VpcRouteServerPeerBgpOptionsEl>>,
}
