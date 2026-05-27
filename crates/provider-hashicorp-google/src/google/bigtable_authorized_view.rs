use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BigtableAuthorizedViewData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_name: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    table_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subset_view: Option<Vec<BigtableAuthorizedViewSubsetViewEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BigtableAuthorizedViewTimeoutsEl>,
    dynamic: BigtableAuthorizedViewDynamic,
}
struct BigtableAuthorizedView_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BigtableAuthorizedViewData>,
}
#[derive(Clone)]
pub struct BigtableAuthorizedView(Rc<BigtableAuthorizedView_>);
impl BigtableAuthorizedView {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_protection`.\nA field to make the authorized view protected against data loss i.e. when set to PROTECTED, deleting the authorized view, the table containing the authorized view, and the instance containing the authorized view would be prohibited.\nIf not provided, currently deletion protection will be set to UNPROTECTED as it is the API default value. Note this field configs the deletion protection provided by the API in the backend, and should not be confused with Terraform-side deletion protection."]
    pub fn set_deletion_protection(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `subset_view`.\n"]
    pub fn set_subset_view(
        self,
        v: impl Into<BlockAssignable<BigtableAuthorizedViewSubsetViewEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().subset_view = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.subset_view = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BigtableAuthorizedViewTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nA field to make the authorized view protected against data loss i.e. when set to PROTECTED, deleting the authorized view, the table containing the authorized view, and the instance containing the authorized view would be prohibited.\nIf not provided, currently deletion protection will be set to UNPROTECTED as it is the API default value. Note this field configs the deletion protection provided by the API in the backend, and should not be confused with Terraform-side deletion protection."]
    pub fn deletion_protection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_name` after provisioning.\nThe name of the Bigtable instance in which the authorized view belongs."]
    pub fn instance_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the authorized view. Must be 1-50 characters and must only contain hyphens, underscores, periods, letters and numbers."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table_name` after provisioning.\nThe name of the Bigtable table in which the authorized view belongs."]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subset_view` after provisioning.\n"]
    pub fn subset_view(&self) -> ListRef<BigtableAuthorizedViewSubsetViewElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subset_view", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigtableAuthorizedViewTimeoutsElRef {
        BigtableAuthorizedViewTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BigtableAuthorizedView {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BigtableAuthorizedView {}
impl ToListMappable for BigtableAuthorizedView {
    type O = ListRef<BigtableAuthorizedViewRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BigtableAuthorizedView_ {
    fn extract_resource_type(&self) -> String {
        "google_bigtable_authorized_view".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBigtableAuthorizedView {
    pub tf_id: String,
    #[doc = "The name of the Bigtable instance in which the authorized view belongs."]
    pub instance_name: PrimField<String>,
    #[doc = "The name of the authorized view. Must be 1-50 characters and must only contain hyphens, underscores, periods, letters and numbers."]
    pub name: PrimField<String>,
    #[doc = "The name of the Bigtable table in which the authorized view belongs."]
    pub table_name: PrimField<String>,
}
impl BuildBigtableAuthorizedView {
    pub fn build(self, stack: &mut Stack) -> BigtableAuthorizedView {
        let out = BigtableAuthorizedView(Rc::new(BigtableAuthorizedView_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BigtableAuthorizedViewData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_name: self.instance_name,
                name: self.name,
                project: core::default::Default::default(),
                table_name: self.table_name,
                subset_view: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BigtableAuthorizedViewRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigtableAuthorizedViewRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BigtableAuthorizedViewRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nA field to make the authorized view protected against data loss i.e. when set to PROTECTED, deleting the authorized view, the table containing the authorized view, and the instance containing the authorized view would be prohibited.\nIf not provided, currently deletion protection will be set to UNPROTECTED as it is the API default value. Note this field configs the deletion protection provided by the API in the backend, and should not be confused with Terraform-side deletion protection."]
    pub fn deletion_protection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_name` after provisioning.\nThe name of the Bigtable instance in which the authorized view belongs."]
    pub fn instance_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the authorized view. Must be 1-50 characters and must only contain hyphens, underscores, periods, letters and numbers."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table_name` after provisioning.\nThe name of the Bigtable table in which the authorized view belongs."]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subset_view` after provisioning.\n"]
    pub fn subset_view(&self) -> ListRef<BigtableAuthorizedViewSubsetViewElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subset_view", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigtableAuthorizedViewTimeoutsElRef {
        BigtableAuthorizedViewTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BigtableAuthorizedViewSubsetViewElFamilySubsetsEl {
    family_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    qualifier_prefixes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    qualifiers: Option<SetField<PrimField<String>>>,
}
impl BigtableAuthorizedViewSubsetViewElFamilySubsetsEl {
    #[doc = "Set the field `qualifier_prefixes`.\nBase64-encoded prefixes for qualifiers of the column family to be included in the authorized view. Every qualifier starting with one of these prefixes is included in the authorized view. To provide access to all qualifiers, include the empty string as a prefix (\"\")."]
    pub fn set_qualifier_prefixes(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.qualifier_prefixes = Some(v.into());
        self
    }
    #[doc = "Set the field `qualifiers`.\nBase64-encoded individual exact column qualifiers of the column family to be included in the authorized view."]
    pub fn set_qualifiers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.qualifiers = Some(v.into());
        self
    }
}
impl ToListMappable for BigtableAuthorizedViewSubsetViewElFamilySubsetsEl {
    type O = BlockAssignable<BigtableAuthorizedViewSubsetViewElFamilySubsetsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigtableAuthorizedViewSubsetViewElFamilySubsetsEl {
    #[doc = "Name of the column family to be included in the authorized view."]
    pub family_name: PrimField<String>,
}
impl BuildBigtableAuthorizedViewSubsetViewElFamilySubsetsEl {
    pub fn build(self) -> BigtableAuthorizedViewSubsetViewElFamilySubsetsEl {
        BigtableAuthorizedViewSubsetViewElFamilySubsetsEl {
            family_name: self.family_name,
            qualifier_prefixes: core::default::Default::default(),
            qualifiers: core::default::Default::default(),
        }
    }
}
pub struct BigtableAuthorizedViewSubsetViewElFamilySubsetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigtableAuthorizedViewSubsetViewElFamilySubsetsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BigtableAuthorizedViewSubsetViewElFamilySubsetsElRef {
        BigtableAuthorizedViewSubsetViewElFamilySubsetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigtableAuthorizedViewSubsetViewElFamilySubsetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `family_name` after provisioning.\nName of the column family to be included in the authorized view."]
    pub fn family_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.family_name", self.base))
    }
    #[doc = "Get a reference to the value of field `qualifier_prefixes` after provisioning.\nBase64-encoded prefixes for qualifiers of the column family to be included in the authorized view. Every qualifier starting with one of these prefixes is included in the authorized view. To provide access to all qualifiers, include the empty string as a prefix (\"\")."]
    pub fn qualifier_prefixes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.qualifier_prefixes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `qualifiers` after provisioning.\nBase64-encoded individual exact column qualifiers of the column family to be included in the authorized view."]
    pub fn qualifiers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.qualifiers", self.base))
    }
}
#[derive(Serialize, Default)]
struct BigtableAuthorizedViewSubsetViewElDynamic {
    family_subsets: Option<DynamicBlock<BigtableAuthorizedViewSubsetViewElFamilySubsetsEl>>,
}
#[derive(Serialize)]
pub struct BigtableAuthorizedViewSubsetViewEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    row_prefixes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    family_subsets: Option<Vec<BigtableAuthorizedViewSubsetViewElFamilySubsetsEl>>,
    dynamic: BigtableAuthorizedViewSubsetViewElDynamic,
}
impl BigtableAuthorizedViewSubsetViewEl {
    #[doc = "Set the field `row_prefixes`.\nBase64-encoded row prefixes to be included in the authorized view. To provide access to all rows, include the empty string as a prefix (\"\")."]
    pub fn set_row_prefixes(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.row_prefixes = Some(v.into());
        self
    }
    #[doc = "Set the field `family_subsets`.\n"]
    pub fn set_family_subsets(
        mut self,
        v: impl Into<BlockAssignable<BigtableAuthorizedViewSubsetViewElFamilySubsetsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.family_subsets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.family_subsets = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BigtableAuthorizedViewSubsetViewEl {
    type O = BlockAssignable<BigtableAuthorizedViewSubsetViewEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigtableAuthorizedViewSubsetViewEl {}
impl BuildBigtableAuthorizedViewSubsetViewEl {
    pub fn build(self) -> BigtableAuthorizedViewSubsetViewEl {
        BigtableAuthorizedViewSubsetViewEl {
            row_prefixes: core::default::Default::default(),
            family_subsets: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BigtableAuthorizedViewSubsetViewElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigtableAuthorizedViewSubsetViewElRef {
    fn new(shared: StackShared, base: String) -> BigtableAuthorizedViewSubsetViewElRef {
        BigtableAuthorizedViewSubsetViewElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigtableAuthorizedViewSubsetViewElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `row_prefixes` after provisioning.\nBase64-encoded row prefixes to be included in the authorized view. To provide access to all rows, include the empty string as a prefix (\"\")."]
    pub fn row_prefixes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.row_prefixes", self.base))
    }
}
#[derive(Serialize)]
pub struct BigtableAuthorizedViewTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BigtableAuthorizedViewTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for BigtableAuthorizedViewTimeoutsEl {
    type O = BlockAssignable<BigtableAuthorizedViewTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigtableAuthorizedViewTimeoutsEl {}
impl BuildBigtableAuthorizedViewTimeoutsEl {
    pub fn build(self) -> BigtableAuthorizedViewTimeoutsEl {
        BigtableAuthorizedViewTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BigtableAuthorizedViewTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigtableAuthorizedViewTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BigtableAuthorizedViewTimeoutsElRef {
        BigtableAuthorizedViewTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigtableAuthorizedViewTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct BigtableAuthorizedViewDynamic {
    subset_view: Option<DynamicBlock<BigtableAuthorizedViewSubsetViewEl>>,
}
