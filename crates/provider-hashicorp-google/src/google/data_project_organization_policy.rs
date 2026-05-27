use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataProjectOrganizationPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    constraint: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    project: PrimField<String>,
}
struct DataProjectOrganizationPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataProjectOrganizationPolicyData>,
}
#[derive(Clone)]
pub struct DataProjectOrganizationPolicy(Rc<DataProjectOrganizationPolicy_>);
impl DataProjectOrganizationPolicy {
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
    #[doc = "Get a reference to the value of field `boolean_policy` after provisioning.\nA boolean policy is a constraint that is either enforced or not."]
    pub fn boolean_policy(&self) -> ListRef<DataProjectOrganizationPolicyBooleanPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.boolean_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `constraint` after provisioning.\nThe name of the Constraint the Policy is configuring, for example, serviceuser.services."]
    pub fn constraint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.constraint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe etag of the organization policy. etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `list_policy` after provisioning.\nA policy that can define specific values that are allowed or denied for the given constraint. It can also be used to allow or deny all values. "]
    pub fn list_policy(&self) -> ListRef<DataProjectOrganizationPolicyListPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.list_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project ID."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restore_policy` after provisioning.\nA restore policy is a constraint to restore the default policy."]
    pub fn restore_policy(&self) -> ListRef<DataProjectOrganizationPolicyRestorePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restore_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds, representing when the variable was last updated. Example: \"2016-10-09T12:33:37.578138407Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion of the Policy. Default version is 0."]
    pub fn version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
}
impl Referable for DataProjectOrganizationPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataProjectOrganizationPolicy {}
impl ToListMappable for DataProjectOrganizationPolicy {
    type O = ListRef<DataProjectOrganizationPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataProjectOrganizationPolicy_ {
    fn extract_datasource_type(&self) -> String {
        "google_project_organization_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataProjectOrganizationPolicy {
    pub tf_id: String,
    #[doc = "The name of the Constraint the Policy is configuring, for example, serviceuser.services."]
    pub constraint: PrimField<String>,
    #[doc = "The project ID."]
    pub project: PrimField<String>,
}
impl BuildDataProjectOrganizationPolicy {
    pub fn build(self, stack: &mut Stack) -> DataProjectOrganizationPolicy {
        let out = DataProjectOrganizationPolicy(Rc::new(DataProjectOrganizationPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataProjectOrganizationPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                constraint: self.constraint,
                id: core::default::Default::default(),
                project: self.project,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataProjectOrganizationPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataProjectOrganizationPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataProjectOrganizationPolicyRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `boolean_policy` after provisioning.\nA boolean policy is a constraint that is either enforced or not."]
    pub fn boolean_policy(&self) -> ListRef<DataProjectOrganizationPolicyBooleanPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.boolean_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `constraint` after provisioning.\nThe name of the Constraint the Policy is configuring, for example, serviceuser.services."]
    pub fn constraint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.constraint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe etag of the organization policy. etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `list_policy` after provisioning.\nA policy that can define specific values that are allowed or denied for the given constraint. It can also be used to allow or deny all values. "]
    pub fn list_policy(&self) -> ListRef<DataProjectOrganizationPolicyListPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.list_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project ID."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restore_policy` after provisioning.\nA restore policy is a constraint to restore the default policy."]
    pub fn restore_policy(&self) -> ListRef<DataProjectOrganizationPolicyRestorePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restore_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds, representing when the variable was last updated. Example: \"2016-10-09T12:33:37.578138407Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion of the Policy. Default version is 0."]
    pub fn version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataProjectOrganizationPolicyBooleanPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enforced: Option<PrimField<bool>>,
}
impl DataProjectOrganizationPolicyBooleanPolicyEl {
    #[doc = "Set the field `enforced`.\n"]
    pub fn set_enforced(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enforced = Some(v.into());
        self
    }
}
impl ToListMappable for DataProjectOrganizationPolicyBooleanPolicyEl {
    type O = BlockAssignable<DataProjectOrganizationPolicyBooleanPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataProjectOrganizationPolicyBooleanPolicyEl {}
impl BuildDataProjectOrganizationPolicyBooleanPolicyEl {
    pub fn build(self) -> DataProjectOrganizationPolicyBooleanPolicyEl {
        DataProjectOrganizationPolicyBooleanPolicyEl {
            enforced: core::default::Default::default(),
        }
    }
}
pub struct DataProjectOrganizationPolicyBooleanPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataProjectOrganizationPolicyBooleanPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataProjectOrganizationPolicyBooleanPolicyElRef {
        DataProjectOrganizationPolicyBooleanPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataProjectOrganizationPolicyBooleanPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enforced` after provisioning.\n"]
    pub fn enforced(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforced", self.base))
    }
}
#[derive(Serialize)]
pub struct DataProjectOrganizationPolicyListPolicyElAllowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}
impl DataProjectOrganizationPolicyListPolicyElAllowEl {
    #[doc = "Set the field `all`.\n"]
    pub fn set_all(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.all = Some(v.into());
        self
    }
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable for DataProjectOrganizationPolicyListPolicyElAllowEl {
    type O = BlockAssignable<DataProjectOrganizationPolicyListPolicyElAllowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataProjectOrganizationPolicyListPolicyElAllowEl {}
impl BuildDataProjectOrganizationPolicyListPolicyElAllowEl {
    pub fn build(self) -> DataProjectOrganizationPolicyListPolicyElAllowEl {
        DataProjectOrganizationPolicyListPolicyElAllowEl {
            all: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct DataProjectOrganizationPolicyListPolicyElAllowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataProjectOrganizationPolicyListPolicyElAllowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataProjectOrganizationPolicyListPolicyElAllowElRef {
        DataProjectOrganizationPolicyListPolicyElAllowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataProjectOrganizationPolicyListPolicyElAllowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `all` after provisioning.\n"]
    pub fn all(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.all", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct DataProjectOrganizationPolicyListPolicyElDenyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}
impl DataProjectOrganizationPolicyListPolicyElDenyEl {
    #[doc = "Set the field `all`.\n"]
    pub fn set_all(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.all = Some(v.into());
        self
    }
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable for DataProjectOrganizationPolicyListPolicyElDenyEl {
    type O = BlockAssignable<DataProjectOrganizationPolicyListPolicyElDenyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataProjectOrganizationPolicyListPolicyElDenyEl {}
impl BuildDataProjectOrganizationPolicyListPolicyElDenyEl {
    pub fn build(self) -> DataProjectOrganizationPolicyListPolicyElDenyEl {
        DataProjectOrganizationPolicyListPolicyElDenyEl {
            all: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct DataProjectOrganizationPolicyListPolicyElDenyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataProjectOrganizationPolicyListPolicyElDenyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataProjectOrganizationPolicyListPolicyElDenyElRef {
        DataProjectOrganizationPolicyListPolicyElDenyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataProjectOrganizationPolicyListPolicyElDenyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `all` after provisioning.\n"]
    pub fn all(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.all", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct DataProjectOrganizationPolicyListPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow: Option<ListField<DataProjectOrganizationPolicyListPolicyElAllowEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deny: Option<ListField<DataProjectOrganizationPolicyListPolicyElDenyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inherit_from_parent: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_value: Option<PrimField<String>>,
}
impl DataProjectOrganizationPolicyListPolicyEl {
    #[doc = "Set the field `allow`.\n"]
    pub fn set_allow(
        mut self,
        v: impl Into<ListField<DataProjectOrganizationPolicyListPolicyElAllowEl>>,
    ) -> Self {
        self.allow = Some(v.into());
        self
    }
    #[doc = "Set the field `deny`.\n"]
    pub fn set_deny(
        mut self,
        v: impl Into<ListField<DataProjectOrganizationPolicyListPolicyElDenyEl>>,
    ) -> Self {
        self.deny = Some(v.into());
        self
    }
    #[doc = "Set the field `inherit_from_parent`.\n"]
    pub fn set_inherit_from_parent(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.inherit_from_parent = Some(v.into());
        self
    }
    #[doc = "Set the field `suggested_value`.\n"]
    pub fn set_suggested_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suggested_value = Some(v.into());
        self
    }
}
impl ToListMappable for DataProjectOrganizationPolicyListPolicyEl {
    type O = BlockAssignable<DataProjectOrganizationPolicyListPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataProjectOrganizationPolicyListPolicyEl {}
impl BuildDataProjectOrganizationPolicyListPolicyEl {
    pub fn build(self) -> DataProjectOrganizationPolicyListPolicyEl {
        DataProjectOrganizationPolicyListPolicyEl {
            allow: core::default::Default::default(),
            deny: core::default::Default::default(),
            inherit_from_parent: core::default::Default::default(),
            suggested_value: core::default::Default::default(),
        }
    }
}
pub struct DataProjectOrganizationPolicyListPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataProjectOrganizationPolicyListPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataProjectOrganizationPolicyListPolicyElRef {
        DataProjectOrganizationPolicyListPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataProjectOrganizationPolicyListPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow` after provisioning.\n"]
    pub fn allow(&self) -> ListRef<DataProjectOrganizationPolicyListPolicyElAllowElRef> {
        ListRef::new(self.shared().clone(), format!("{}.allow", self.base))
    }
    #[doc = "Get a reference to the value of field `deny` after provisioning.\n"]
    pub fn deny(&self) -> ListRef<DataProjectOrganizationPolicyListPolicyElDenyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deny", self.base))
    }
    #[doc = "Get a reference to the value of field `inherit_from_parent` after provisioning.\n"]
    pub fn inherit_from_parent(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inherit_from_parent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `suggested_value` after provisioning.\n"]
    pub fn suggested_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.suggested_value", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataProjectOrganizationPolicyRestorePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<PrimField<bool>>,
}
impl DataProjectOrganizationPolicyRestorePolicyEl {
    #[doc = "Set the field `default`.\n"]
    pub fn set_default(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.default = Some(v.into());
        self
    }
}
impl ToListMappable for DataProjectOrganizationPolicyRestorePolicyEl {
    type O = BlockAssignable<DataProjectOrganizationPolicyRestorePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataProjectOrganizationPolicyRestorePolicyEl {}
impl BuildDataProjectOrganizationPolicyRestorePolicyEl {
    pub fn build(self) -> DataProjectOrganizationPolicyRestorePolicyEl {
        DataProjectOrganizationPolicyRestorePolicyEl {
            default: core::default::Default::default(),
        }
    }
}
pub struct DataProjectOrganizationPolicyRestorePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataProjectOrganizationPolicyRestorePolicyElRef {
    fn new(shared: StackShared, base: String) -> DataProjectOrganizationPolicyRestorePolicyElRef {
        DataProjectOrganizationPolicyRestorePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataProjectOrganizationPolicyRestorePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default` after provisioning.\n"]
    pub fn default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }
}
