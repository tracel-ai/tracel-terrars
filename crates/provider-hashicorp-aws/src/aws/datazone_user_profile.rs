use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DatazoneUserProfileData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    domain_identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    user_identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DatazoneUserProfileTimeoutsEl>,
}
struct DatazoneUserProfile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DatazoneUserProfileData>,
}
#[derive(Clone)]
pub struct DatazoneUserProfile(Rc<DatazoneUserProfile_>);
impl DatazoneUserProfile {
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
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().status = Some(v.into());
        self
    }
    #[doc = "Set the field `user_type`.\n"]
    pub fn set_user_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user_type = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DatazoneUserProfileTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> ListRef<DatazoneUserProfileDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_identifier` after provisioning.\n"]
    pub fn domain_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_identifier` after provisioning.\n"]
    pub fn user_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_type` after provisioning.\n"]
    pub fn user_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatazoneUserProfileTimeoutsElRef {
        DatazoneUserProfileTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DatazoneUserProfile {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DatazoneUserProfile {}
impl ToListMappable for DatazoneUserProfile {
    type O = ListRef<DatazoneUserProfileRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DatazoneUserProfile_ {
    fn extract_resource_type(&self) -> String {
        "aws_datazone_user_profile".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDatazoneUserProfile {
    pub tf_id: String,
    #[doc = ""]
    pub domain_identifier: PrimField<String>,
    #[doc = ""]
    pub user_identifier: PrimField<String>,
}
impl BuildDatazoneUserProfile {
    pub fn build(self, stack: &mut Stack) -> DatazoneUserProfile {
        let out = DatazoneUserProfile(Rc::new(DatazoneUserProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DatazoneUserProfileData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                domain_identifier: self.domain_identifier,
                region: core::default::Default::default(),
                status: core::default::Default::default(),
                user_identifier: self.user_identifier,
                user_type: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DatazoneUserProfileRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatazoneUserProfileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DatazoneUserProfileRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> ListRef<DatazoneUserProfileDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_identifier` after provisioning.\n"]
    pub fn domain_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_identifier` after provisioning.\n"]
    pub fn user_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_type` after provisioning.\n"]
    pub fn user_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatazoneUserProfileTimeoutsElRef {
        DatazoneUserProfileTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DatazoneUserProfileDetailsElIamEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
}
impl DatazoneUserProfileDetailsElIamEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }
}
impl ToListMappable for DatazoneUserProfileDetailsElIamEl {
    type O = BlockAssignable<DatazoneUserProfileDetailsElIamEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatazoneUserProfileDetailsElIamEl {}
impl BuildDatazoneUserProfileDetailsElIamEl {
    pub fn build(self) -> DatazoneUserProfileDetailsElIamEl {
        DatazoneUserProfileDetailsElIamEl {
            arn: core::default::Default::default(),
        }
    }
}
pub struct DatazoneUserProfileDetailsElIamElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatazoneUserProfileDetailsElIamElRef {
    fn new(shared: StackShared, base: String) -> DatazoneUserProfileDetailsElIamElRef {
        DatazoneUserProfileDetailsElIamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatazoneUserProfileDetailsElIamElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
}
#[derive(Serialize)]
pub struct DatazoneUserProfileDetailsElSsoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_name: Option<PrimField<String>>,
}
impl DatazoneUserProfileDetailsElSsoEl {
    #[doc = "Set the field `first_name`.\n"]
    pub fn set_first_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.first_name = Some(v.into());
        self
    }
    #[doc = "Set the field `last_name`.\n"]
    pub fn set_last_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_name = Some(v.into());
        self
    }
    #[doc = "Set the field `user_name`.\n"]
    pub fn set_user_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_name = Some(v.into());
        self
    }
}
impl ToListMappable for DatazoneUserProfileDetailsElSsoEl {
    type O = BlockAssignable<DatazoneUserProfileDetailsElSsoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatazoneUserProfileDetailsElSsoEl {}
impl BuildDatazoneUserProfileDetailsElSsoEl {
    pub fn build(self) -> DatazoneUserProfileDetailsElSsoEl {
        DatazoneUserProfileDetailsElSsoEl {
            first_name: core::default::Default::default(),
            last_name: core::default::Default::default(),
            user_name: core::default::Default::default(),
        }
    }
}
pub struct DatazoneUserProfileDetailsElSsoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatazoneUserProfileDetailsElSsoElRef {
    fn new(shared: StackShared, base: String) -> DatazoneUserProfileDetailsElSsoElRef {
        DatazoneUserProfileDetailsElSsoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatazoneUserProfileDetailsElSsoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `first_name` after provisioning.\n"]
    pub fn first_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.first_name", self.base))
    }
    #[doc = "Get a reference to the value of field `last_name` after provisioning.\n"]
    pub fn last_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_name", self.base))
    }
    #[doc = "Get a reference to the value of field `user_name` after provisioning.\n"]
    pub fn user_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DatazoneUserProfileDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iam: Option<ListField<DatazoneUserProfileDetailsElIamEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sso: Option<ListField<DatazoneUserProfileDetailsElSsoEl>>,
}
impl DatazoneUserProfileDetailsEl {
    #[doc = "Set the field `iam`.\n"]
    pub fn set_iam(mut self, v: impl Into<ListField<DatazoneUserProfileDetailsElIamEl>>) -> Self {
        self.iam = Some(v.into());
        self
    }
    #[doc = "Set the field `sso`.\n"]
    pub fn set_sso(mut self, v: impl Into<ListField<DatazoneUserProfileDetailsElSsoEl>>) -> Self {
        self.sso = Some(v.into());
        self
    }
}
impl ToListMappable for DatazoneUserProfileDetailsEl {
    type O = BlockAssignable<DatazoneUserProfileDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatazoneUserProfileDetailsEl {}
impl BuildDatazoneUserProfileDetailsEl {
    pub fn build(self) -> DatazoneUserProfileDetailsEl {
        DatazoneUserProfileDetailsEl {
            iam: core::default::Default::default(),
            sso: core::default::Default::default(),
        }
    }
}
pub struct DatazoneUserProfileDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatazoneUserProfileDetailsElRef {
    fn new(shared: StackShared, base: String) -> DatazoneUserProfileDetailsElRef {
        DatazoneUserProfileDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatazoneUserProfileDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `iam` after provisioning.\n"]
    pub fn iam(&self) -> ListRef<DatazoneUserProfileDetailsElIamElRef> {
        ListRef::new(self.shared().clone(), format!("{}.iam", self.base))
    }
    #[doc = "Get a reference to the value of field `sso` after provisioning.\n"]
    pub fn sso(&self) -> ListRef<DatazoneUserProfileDetailsElSsoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sso", self.base))
    }
}
#[derive(Serialize)]
pub struct DatazoneUserProfileTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DatazoneUserProfileTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for DatazoneUserProfileTimeoutsEl {
    type O = BlockAssignable<DatazoneUserProfileTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatazoneUserProfileTimeoutsEl {}
impl BuildDatazoneUserProfileTimeoutsEl {
    pub fn build(self) -> DatazoneUserProfileTimeoutsEl {
        DatazoneUserProfileTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DatazoneUserProfileTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatazoneUserProfileTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DatazoneUserProfileTimeoutsElRef {
        DatazoneUserProfileTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatazoneUserProfileTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
