use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudfrontVpcOriginData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudfrontVpcOriginTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_origin_endpoint_config: Option<Vec<CloudfrontVpcOriginVpcOriginEndpointConfigEl>>,
    dynamic: CloudfrontVpcOriginDynamic,
}

struct CloudfrontVpcOrigin_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudfrontVpcOriginData>,
}

#[derive(Clone)]
pub struct CloudfrontVpcOrigin(Rc<CloudfrontVpcOrigin_>);

impl CloudfrontVpcOrigin {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CloudfrontVpcOriginTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_origin_endpoint_config`.\n"]
    pub fn set_vpc_origin_endpoint_config(
        self,
        v: impl Into<BlockAssignable<CloudfrontVpcOriginVpcOriginEndpointConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_origin_endpoint_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_origin_endpoint_config = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
    pub fn timeouts(&self) -> CloudfrontVpcOriginTimeoutsElRef {
        CloudfrontVpcOriginTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_origin_endpoint_config` after provisioning.\n"]
    pub fn vpc_origin_endpoint_config(&self) -> ListRef<CloudfrontVpcOriginVpcOriginEndpointConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_origin_endpoint_config", self.extract_ref()))
    }
}

impl Referable for CloudfrontVpcOrigin {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for CloudfrontVpcOrigin { }

impl ToListMappable for CloudfrontVpcOrigin {
    type O = ListRef<CloudfrontVpcOriginRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CloudfrontVpcOrigin_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudfront_vpc_origin".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudfrontVpcOrigin {
    pub tf_id: String,
}

impl BuildCloudfrontVpcOrigin {
    pub fn build(self, stack: &mut Stack) -> CloudfrontVpcOrigin {
        let out = CloudfrontVpcOrigin(Rc::new(CloudfrontVpcOrigin_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudfrontVpcOriginData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                tags: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_origin_endpoint_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudfrontVpcOriginRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontVpcOriginRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl CloudfrontVpcOriginRef {
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

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
    pub fn timeouts(&self) -> CloudfrontVpcOriginTimeoutsElRef {
        CloudfrontVpcOriginTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_origin_endpoint_config` after provisioning.\n"]
    pub fn vpc_origin_endpoint_config(&self) -> ListRef<CloudfrontVpcOriginVpcOriginEndpointConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_origin_endpoint_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudfrontVpcOriginTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl CloudfrontVpcOriginTimeoutsEl {
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

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for CloudfrontVpcOriginTimeoutsEl {
    type O = BlockAssignable<CloudfrontVpcOriginTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontVpcOriginTimeoutsEl {}

impl BuildCloudfrontVpcOriginTimeoutsEl {
    pub fn build(self) -> CloudfrontVpcOriginTimeoutsEl {
        CloudfrontVpcOriginTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontVpcOriginTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontVpcOriginTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontVpcOriginTimeoutsElRef {
        CloudfrontVpcOriginTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontVpcOriginTimeoutsElRef {
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

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontVpcOriginVpcOriginEndpointConfigElOriginSslProtocolsEl {
    items: SetField<PrimField<String>>,
    quantity: PrimField<f64>,
}

impl CloudfrontVpcOriginVpcOriginEndpointConfigElOriginSslProtocolsEl { }

impl ToListMappable for CloudfrontVpcOriginVpcOriginEndpointConfigElOriginSslProtocolsEl {
    type O = BlockAssignable<CloudfrontVpcOriginVpcOriginEndpointConfigElOriginSslProtocolsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontVpcOriginVpcOriginEndpointConfigElOriginSslProtocolsEl {
    #[doc = ""]
    pub items: SetField<PrimField<String>>,
    #[doc = ""]
    pub quantity: PrimField<f64>,
}

impl BuildCloudfrontVpcOriginVpcOriginEndpointConfigElOriginSslProtocolsEl {
    pub fn build(self) -> CloudfrontVpcOriginVpcOriginEndpointConfigElOriginSslProtocolsEl {
        CloudfrontVpcOriginVpcOriginEndpointConfigElOriginSslProtocolsEl {
            items: self.items,
            quantity: self.quantity,
        }
    }
}

pub struct CloudfrontVpcOriginVpcOriginEndpointConfigElOriginSslProtocolsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontVpcOriginVpcOriginEndpointConfigElOriginSslProtocolsElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontVpcOriginVpcOriginEndpointConfigElOriginSslProtocolsElRef {
        CloudfrontVpcOriginVpcOriginEndpointConfigElOriginSslProtocolsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontVpcOriginVpcOriginEndpointConfigElOriginSslProtocolsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }

    #[doc = "Get a reference to the value of field `quantity` after provisioning.\n"]
    pub fn quantity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.quantity", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontVpcOriginVpcOriginEndpointConfigElDynamic {
    origin_ssl_protocols: Option<DynamicBlock<CloudfrontVpcOriginVpcOriginEndpointConfigElOriginSslProtocolsEl>>,
}

#[derive(Serialize)]
pub struct CloudfrontVpcOriginVpcOriginEndpointConfigEl {
    arn: PrimField<String>,
    http_port: PrimField<f64>,
    https_port: PrimField<f64>,
    name: PrimField<String>,
    origin_protocol_policy: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_ssl_protocols: Option<Vec<CloudfrontVpcOriginVpcOriginEndpointConfigElOriginSslProtocolsEl>>,
    dynamic: CloudfrontVpcOriginVpcOriginEndpointConfigElDynamic,
}

impl CloudfrontVpcOriginVpcOriginEndpointConfigEl {
    #[doc = "Set the field `origin_ssl_protocols`.\n"]
    pub fn set_origin_ssl_protocols(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontVpcOriginVpcOriginEndpointConfigElOriginSslProtocolsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.origin_ssl_protocols = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.origin_ssl_protocols = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CloudfrontVpcOriginVpcOriginEndpointConfigEl {
    type O = BlockAssignable<CloudfrontVpcOriginVpcOriginEndpointConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontVpcOriginVpcOriginEndpointConfigEl {
    #[doc = ""]
    pub arn: PrimField<String>,
    #[doc = ""]
    pub http_port: PrimField<f64>,
    #[doc = ""]
    pub https_port: PrimField<f64>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub origin_protocol_policy: PrimField<String>,
}

impl BuildCloudfrontVpcOriginVpcOriginEndpointConfigEl {
    pub fn build(self) -> CloudfrontVpcOriginVpcOriginEndpointConfigEl {
        CloudfrontVpcOriginVpcOriginEndpointConfigEl {
            arn: self.arn,
            http_port: self.http_port,
            https_port: self.https_port,
            name: self.name,
            origin_protocol_policy: self.origin_protocol_policy,
            origin_ssl_protocols: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudfrontVpcOriginVpcOriginEndpointConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontVpcOriginVpcOriginEndpointConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontVpcOriginVpcOriginEndpointConfigElRef {
        CloudfrontVpcOriginVpcOriginEndpointConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontVpcOriginVpcOriginEndpointConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `http_port` after provisioning.\n"]
    pub fn http_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_port", self.base))
    }

    #[doc = "Get a reference to the value of field `https_port` after provisioning.\n"]
    pub fn https_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.https_port", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_protocol_policy` after provisioning.\n"]
    pub fn origin_protocol_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin_protocol_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_ssl_protocols` after provisioning.\n"]
    pub fn origin_ssl_protocols(&self) -> ListRef<CloudfrontVpcOriginVpcOriginEndpointConfigElOriginSslProtocolsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.origin_ssl_protocols", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontVpcOriginDynamic {
    vpc_origin_endpoint_config: Option<DynamicBlock<CloudfrontVpcOriginVpcOriginEndpointConfigEl>>,
}
