use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AppsyncSourceApiAssociationData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    merged_api_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merged_api_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_api_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_api_association_config:
        Option<ListField<AppsyncSourceApiAssociationSourceApiAssociationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_api_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AppsyncSourceApiAssociationTimeoutsEl>,
}
struct AppsyncSourceApiAssociation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppsyncSourceApiAssociationData>,
}
#[derive(Clone)]
pub struct AppsyncSourceApiAssociation(Rc<AppsyncSourceApiAssociation_>);
impl AppsyncSourceApiAssociation {
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
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `merged_api_arn`.\n"]
    pub fn set_merged_api_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().merged_api_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `merged_api_id`.\n"]
    pub fn set_merged_api_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().merged_api_id = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `source_api_arn`.\n"]
    pub fn set_source_api_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_api_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `source_api_association_config`.\n"]
    pub fn set_source_api_association_config(
        self,
        v: impl Into<ListField<AppsyncSourceApiAssociationSourceApiAssociationConfigEl>>,
    ) -> Self {
        self.0.data.borrow_mut().source_api_association_config = Some(v.into());
        self
    }
    #[doc = "Set the field `source_api_id`.\n"]
    pub fn set_source_api_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_api_id = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<AppsyncSourceApiAssociationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.association_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `merged_api_arn` after provisioning.\n"]
    pub fn merged_api_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.merged_api_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `merged_api_id` after provisioning.\n"]
    pub fn merged_api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.merged_api_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_api_arn` after provisioning.\n"]
    pub fn source_api_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_api_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_api_association_config` after provisioning.\n"]
    pub fn source_api_association_config(
        &self,
    ) -> ListRef<AppsyncSourceApiAssociationSourceApiAssociationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_api_association_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_api_id` after provisioning.\n"]
    pub fn source_api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_api_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppsyncSourceApiAssociationTimeoutsElRef {
        AppsyncSourceApiAssociationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for AppsyncSourceApiAssociation {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AppsyncSourceApiAssociation {}
impl ToListMappable for AppsyncSourceApiAssociation {
    type O = ListRef<AppsyncSourceApiAssociationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AppsyncSourceApiAssociation_ {
    fn extract_resource_type(&self) -> String {
        "aws_appsync_source_api_association".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAppsyncSourceApiAssociation {
    pub tf_id: String,
}
impl BuildAppsyncSourceApiAssociation {
    pub fn build(self, stack: &mut Stack) -> AppsyncSourceApiAssociation {
        let out = AppsyncSourceApiAssociation(Rc::new(AppsyncSourceApiAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppsyncSourceApiAssociationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                merged_api_arn: core::default::Default::default(),
                merged_api_id: core::default::Default::default(),
                region: core::default::Default::default(),
                source_api_arn: core::default::Default::default(),
                source_api_association_config: core::default::Default::default(),
                source_api_id: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AppsyncSourceApiAssociationRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppsyncSourceApiAssociationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AppsyncSourceApiAssociationRef {
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
    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.association_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `merged_api_arn` after provisioning.\n"]
    pub fn merged_api_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.merged_api_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `merged_api_id` after provisioning.\n"]
    pub fn merged_api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.merged_api_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_api_arn` after provisioning.\n"]
    pub fn source_api_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_api_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_api_association_config` after provisioning.\n"]
    pub fn source_api_association_config(
        &self,
    ) -> ListRef<AppsyncSourceApiAssociationSourceApiAssociationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_api_association_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_api_id` after provisioning.\n"]
    pub fn source_api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_api_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppsyncSourceApiAssociationTimeoutsElRef {
        AppsyncSourceApiAssociationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AppsyncSourceApiAssociationSourceApiAssociationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_type: Option<PrimField<String>>,
}
impl AppsyncSourceApiAssociationSourceApiAssociationConfigEl {
    #[doc = "Set the field `merge_type`.\n"]
    pub fn set_merge_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.merge_type = Some(v.into());
        self
    }
}
impl ToListMappable for AppsyncSourceApiAssociationSourceApiAssociationConfigEl {
    type O = BlockAssignable<AppsyncSourceApiAssociationSourceApiAssociationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppsyncSourceApiAssociationSourceApiAssociationConfigEl {}
impl BuildAppsyncSourceApiAssociationSourceApiAssociationConfigEl {
    pub fn build(self) -> AppsyncSourceApiAssociationSourceApiAssociationConfigEl {
        AppsyncSourceApiAssociationSourceApiAssociationConfigEl {
            merge_type: core::default::Default::default(),
        }
    }
}
pub struct AppsyncSourceApiAssociationSourceApiAssociationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppsyncSourceApiAssociationSourceApiAssociationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppsyncSourceApiAssociationSourceApiAssociationConfigElRef {
        AppsyncSourceApiAssociationSourceApiAssociationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppsyncSourceApiAssociationSourceApiAssociationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `merge_type` after provisioning.\n"]
    pub fn merge_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.merge_type", self.base))
    }
}
#[derive(Serialize)]
pub struct AppsyncSourceApiAssociationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl AppsyncSourceApiAssociationTimeoutsEl {
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
    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for AppsyncSourceApiAssociationTimeoutsEl {
    type O = BlockAssignable<AppsyncSourceApiAssociationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppsyncSourceApiAssociationTimeoutsEl {}
impl BuildAppsyncSourceApiAssociationTimeoutsEl {
    pub fn build(self) -> AppsyncSourceApiAssociationTimeoutsEl {
        AppsyncSourceApiAssociationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct AppsyncSourceApiAssociationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppsyncSourceApiAssociationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> AppsyncSourceApiAssociationTimeoutsElRef {
        AppsyncSourceApiAssociationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppsyncSourceApiAssociationTimeoutsElRef {
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
    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
