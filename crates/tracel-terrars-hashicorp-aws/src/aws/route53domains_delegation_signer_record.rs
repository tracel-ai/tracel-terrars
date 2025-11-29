use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Route53domainsDelegationSignerRecordData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signing_attributes: Option<Vec<Route53domainsDelegationSignerRecordSigningAttributesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<Route53domainsDelegationSignerRecordTimeoutsEl>,
    dynamic: Route53domainsDelegationSignerRecordDynamic,
}

struct Route53domainsDelegationSignerRecord_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Route53domainsDelegationSignerRecordData>,
}

#[derive(Clone)]
pub struct Route53domainsDelegationSignerRecord(Rc<Route53domainsDelegationSignerRecord_>);

impl Route53domainsDelegationSignerRecord {
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

    #[doc = "Set the field `signing_attributes`.\n"]
    pub fn set_signing_attributes(
        self,
        v: impl Into<BlockAssignable<Route53domainsDelegationSignerRecordSigningAttributesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().signing_attributes = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.signing_attributes = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<Route53domainsDelegationSignerRecordTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `dnssec_key_id` after provisioning.\n"]
    pub fn dnssec_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dnssec_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signing_attributes` after provisioning.\n"]
    pub fn signing_attributes(&self) -> ListRef<Route53domainsDelegationSignerRecordSigningAttributesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.signing_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Route53domainsDelegationSignerRecordTimeoutsElRef {
        Route53domainsDelegationSignerRecordTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for Route53domainsDelegationSignerRecord {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Route53domainsDelegationSignerRecord { }

impl ToListMappable for Route53domainsDelegationSignerRecord {
    type O = ListRef<Route53domainsDelegationSignerRecordRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Route53domainsDelegationSignerRecord_ {
    fn extract_resource_type(&self) -> String {
        "aws_route53domains_delegation_signer_record".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRoute53domainsDelegationSignerRecord {
    pub tf_id: String,
    #[doc = ""]
    pub domain_name: PrimField<String>,
}

impl BuildRoute53domainsDelegationSignerRecord {
    pub fn build(self, stack: &mut Stack) -> Route53domainsDelegationSignerRecord {
        let out = Route53domainsDelegationSignerRecord(Rc::new(Route53domainsDelegationSignerRecord_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Route53domainsDelegationSignerRecordData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                domain_name: self.domain_name,
                signing_attributes: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Route53domainsDelegationSignerRecordRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53domainsDelegationSignerRecordRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl Route53domainsDelegationSignerRecordRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dnssec_key_id` after provisioning.\n"]
    pub fn dnssec_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dnssec_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signing_attributes` after provisioning.\n"]
    pub fn signing_attributes(&self) -> ListRef<Route53domainsDelegationSignerRecordSigningAttributesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.signing_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Route53domainsDelegationSignerRecordTimeoutsElRef {
        Route53domainsDelegationSignerRecordTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct Route53domainsDelegationSignerRecordSigningAttributesEl {
    algorithm: PrimField<f64>,
    flags: PrimField<f64>,
    public_key: PrimField<String>,
}

impl Route53domainsDelegationSignerRecordSigningAttributesEl { }

impl ToListMappable for Route53domainsDelegationSignerRecordSigningAttributesEl {
    type O = BlockAssignable<Route53domainsDelegationSignerRecordSigningAttributesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53domainsDelegationSignerRecordSigningAttributesEl {
    #[doc = ""]
    pub algorithm: PrimField<f64>,
    #[doc = ""]
    pub flags: PrimField<f64>,
    #[doc = ""]
    pub public_key: PrimField<String>,
}

impl BuildRoute53domainsDelegationSignerRecordSigningAttributesEl {
    pub fn build(self) -> Route53domainsDelegationSignerRecordSigningAttributesEl {
        Route53domainsDelegationSignerRecordSigningAttributesEl {
            algorithm: self.algorithm,
            flags: self.flags,
            public_key: self.public_key,
        }
    }
}

pub struct Route53domainsDelegationSignerRecordSigningAttributesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53domainsDelegationSignerRecordSigningAttributesElRef {
    fn new(shared: StackShared, base: String) -> Route53domainsDelegationSignerRecordSigningAttributesElRef {
        Route53domainsDelegationSignerRecordSigningAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53domainsDelegationSignerRecordSigningAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }

    #[doc = "Get a reference to the value of field `flags` after provisioning.\n"]
    pub fn flags(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.flags", self.base))
    }

    #[doc = "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53domainsDelegationSignerRecordTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl Route53domainsDelegationSignerRecordTimeoutsEl {
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

impl ToListMappable for Route53domainsDelegationSignerRecordTimeoutsEl {
    type O = BlockAssignable<Route53domainsDelegationSignerRecordTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53domainsDelegationSignerRecordTimeoutsEl {}

impl BuildRoute53domainsDelegationSignerRecordTimeoutsEl {
    pub fn build(self) -> Route53domainsDelegationSignerRecordTimeoutsEl {
        Route53domainsDelegationSignerRecordTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct Route53domainsDelegationSignerRecordTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53domainsDelegationSignerRecordTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> Route53domainsDelegationSignerRecordTimeoutsElRef {
        Route53domainsDelegationSignerRecordTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53domainsDelegationSignerRecordTimeoutsElRef {
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
struct Route53domainsDelegationSignerRecordDynamic {
    signing_attributes: Option<DynamicBlock<Route53domainsDelegationSignerRecordSigningAttributesEl>>,
}
