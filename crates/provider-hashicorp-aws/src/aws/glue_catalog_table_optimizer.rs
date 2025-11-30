use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct GlueCatalogTableOptimizerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    catalog_id: PrimField<String>,
    database_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    table_name: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<Vec<GlueCatalogTableOptimizerConfigurationEl>>,
    dynamic: GlueCatalogTableOptimizerDynamic,
}

struct GlueCatalogTableOptimizer_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GlueCatalogTableOptimizerData>,
}

#[derive(Clone)]
pub struct GlueCatalogTableOptimizer(Rc<GlueCatalogTableOptimizer_>);

impl GlueCatalogTableOptimizer {
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

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl Into<BlockAssignable<GlueCatalogTableOptimizerConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.catalog_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<GlueCatalogTableOptimizerConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }
}

impl Referable for GlueCatalogTableOptimizer {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for GlueCatalogTableOptimizer {}

impl ToListMappable for GlueCatalogTableOptimizer {
    type O = ListRef<GlueCatalogTableOptimizerRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GlueCatalogTableOptimizer_ {
    fn extract_resource_type(&self) -> String {
        "aws_glue_catalog_table_optimizer".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGlueCatalogTableOptimizer {
    pub tf_id: String,
    #[doc = ""]
    pub catalog_id: PrimField<String>,
    #[doc = ""]
    pub database_name: PrimField<String>,
    #[doc = ""]
    pub table_name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildGlueCatalogTableOptimizer {
    pub fn build(self, stack: &mut Stack) -> GlueCatalogTableOptimizer {
        let out = GlueCatalogTableOptimizer(Rc::new(GlueCatalogTableOptimizer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GlueCatalogTableOptimizerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                catalog_id: self.catalog_id,
                database_name: self.database_name,
                region: core::default::Default::default(),
                table_name: self.table_name,
                type_: self.type_,
                configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GlueCatalogTableOptimizerRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueCatalogTableOptimizerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl GlueCatalogTableOptimizerRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.catalog_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<GlueCatalogTableOptimizerConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationElIcebergConfigurationEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orphan_file_retention_period_in_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_rate_in_hours: Option<PrimField<f64>>,
}

impl
    GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationElIcebergConfigurationEl
{
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }

    #[doc = "Set the field `orphan_file_retention_period_in_days`.\n"]
    pub fn set_orphan_file_retention_period_in_days(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.orphan_file_retention_period_in_days = Some(v.into());
        self
    }

    #[doc = "Set the field `run_rate_in_hours`.\n"]
    pub fn set_run_rate_in_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.run_rate_in_hours = Some(v.into());
        self
    }
}

impl ToListMappable for GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationElIcebergConfigurationEl {
    type O =
        BlockAssignable<
            GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationElIcebergConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationElIcebergConfigurationEl
{}

impl BuildGlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationElIcebergConfigurationEl {
    pub fn build(
        self,
    ) -> GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationElIcebergConfigurationEl {
        GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationElIcebergConfigurationEl {
            location: core::default::Default::default(),
            orphan_file_retention_period_in_days: core::default::Default::default(),
            run_rate_in_hours: core::default::Default::default(),
        }
    }
}

pub struct GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationElIcebergConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationElIcebergConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationElIcebergConfigurationElRef {
        GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationElIcebergConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationElIcebergConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `orphan_file_retention_period_in_days` after provisioning.\n"]
    pub fn orphan_file_retention_period_in_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.orphan_file_retention_period_in_days", self.base))
    }

    #[doc = "Get a reference to the value of field `run_rate_in_hours` after provisioning.\n"]
    pub fn run_rate_in_hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.run_rate_in_hours", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationElDynamic {
    iceberg_configuration: Option<
        DynamicBlock<GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationElIcebergConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iceberg_configuration: Option<
        Vec<GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationElIcebergConfigurationEl>,
    >,
    dynamic: GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationElDynamic,
}

impl GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationEl {
    #[doc = "Set the field `iceberg_configuration`.\n"]
    pub fn set_iceberg_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationElIcebergConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.iceberg_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.iceberg_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationEl {
    type O =
        BlockAssignable<GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationEl {}

impl BuildGlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationEl {
    pub fn build(
        self,
    ) -> GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationEl {
        GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationEl {
            iceberg_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationElRef {
        GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iceberg_configuration` after provisioning.\n"]
    pub fn iceberg_configuration(
        &self,
    ) -> ListRef<GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationElIcebergConfigurationElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.iceberg_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTableOptimizerConfigurationElRetentionConfigurationElIcebergConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    clean_expired_files: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number_of_snapshots_to_retain: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_rate_in_hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_retention_period_in_days: Option<PrimField<f64>>,
}

impl GlueCatalogTableOptimizerConfigurationElRetentionConfigurationElIcebergConfigurationEl {
    #[doc = "Set the field `clean_expired_files`.\n"]
    pub fn set_clean_expired_files(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.clean_expired_files = Some(v.into());
        self
    }

    #[doc = "Set the field `number_of_snapshots_to_retain`.\n"]
    pub fn set_number_of_snapshots_to_retain(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.number_of_snapshots_to_retain = Some(v.into());
        self
    }

    #[doc = "Set the field `run_rate_in_hours`.\n"]
    pub fn set_run_rate_in_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.run_rate_in_hours = Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_retention_period_in_days`.\n"]
    pub fn set_snapshot_retention_period_in_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.snapshot_retention_period_in_days = Some(v.into());
        self
    }
}

impl ToListMappable
    for GlueCatalogTableOptimizerConfigurationElRetentionConfigurationElIcebergConfigurationEl
{
    type O = BlockAssignable<
        GlueCatalogTableOptimizerConfigurationElRetentionConfigurationElIcebergConfigurationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableOptimizerConfigurationElRetentionConfigurationElIcebergConfigurationEl
{}

impl BuildGlueCatalogTableOptimizerConfigurationElRetentionConfigurationElIcebergConfigurationEl {
    pub fn build(
        self,
    ) -> GlueCatalogTableOptimizerConfigurationElRetentionConfigurationElIcebergConfigurationEl
    {
        GlueCatalogTableOptimizerConfigurationElRetentionConfigurationElIcebergConfigurationEl {
            clean_expired_files: core::default::Default::default(),
            number_of_snapshots_to_retain: core::default::Default::default(),
            run_rate_in_hours: core::default::Default::default(),
            snapshot_retention_period_in_days: core::default::Default::default(),
        }
    }
}

pub struct GlueCatalogTableOptimizerConfigurationElRetentionConfigurationElIcebergConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref
    for GlueCatalogTableOptimizerConfigurationElRetentionConfigurationElIcebergConfigurationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> GlueCatalogTableOptimizerConfigurationElRetentionConfigurationElIcebergConfigurationElRef
    {
        GlueCatalogTableOptimizerConfigurationElRetentionConfigurationElIcebergConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableOptimizerConfigurationElRetentionConfigurationElIcebergConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `clean_expired_files` after provisioning.\n"]
    pub fn clean_expired_files(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.clean_expired_files", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `number_of_snapshots_to_retain` after provisioning.\n"]
    pub fn number_of_snapshots_to_retain(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.number_of_snapshots_to_retain", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `run_rate_in_hours` after provisioning.\n"]
    pub fn run_rate_in_hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.run_rate_in_hours", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `snapshot_retention_period_in_days` after provisioning.\n"]
    pub fn snapshot_retention_period_in_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.snapshot_retention_period_in_days", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogTableOptimizerConfigurationElRetentionConfigurationElDynamic {
    iceberg_configuration: Option<
        DynamicBlock<
            GlueCatalogTableOptimizerConfigurationElRetentionConfigurationElIcebergConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct GlueCatalogTableOptimizerConfigurationElRetentionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iceberg_configuration: Option<
        Vec<GlueCatalogTableOptimizerConfigurationElRetentionConfigurationElIcebergConfigurationEl>,
    >,
    dynamic: GlueCatalogTableOptimizerConfigurationElRetentionConfigurationElDynamic,
}

impl GlueCatalogTableOptimizerConfigurationElRetentionConfigurationEl {
    #[doc = "Set the field `iceberg_configuration`.\n"]
    pub fn set_iceberg_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            GlueCatalogTableOptimizerConfigurationElRetentionConfigurationElIcebergConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.iceberg_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.iceberg_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for GlueCatalogTableOptimizerConfigurationElRetentionConfigurationEl {
    type O = BlockAssignable<GlueCatalogTableOptimizerConfigurationElRetentionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableOptimizerConfigurationElRetentionConfigurationEl {}

impl BuildGlueCatalogTableOptimizerConfigurationElRetentionConfigurationEl {
    pub fn build(self) -> GlueCatalogTableOptimizerConfigurationElRetentionConfigurationEl {
        GlueCatalogTableOptimizerConfigurationElRetentionConfigurationEl {
            iceberg_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct GlueCatalogTableOptimizerConfigurationElRetentionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueCatalogTableOptimizerConfigurationElRetentionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GlueCatalogTableOptimizerConfigurationElRetentionConfigurationElRef {
        GlueCatalogTableOptimizerConfigurationElRetentionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableOptimizerConfigurationElRetentionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iceberg_configuration` after provisioning.\n"]
    pub fn iceberg_configuration(
        &self,
    ) -> ListRef<
        GlueCatalogTableOptimizerConfigurationElRetentionConfigurationElIcebergConfigurationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.iceberg_configuration", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogTableOptimizerConfigurationElDynamic {
    orphan_file_deletion_configuration: Option<
        DynamicBlock<GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationEl>,
    >,
    retention_configuration:
        Option<DynamicBlock<GlueCatalogTableOptimizerConfigurationElRetentionConfigurationEl>>,
}

#[derive(Serialize)]
pub struct GlueCatalogTableOptimizerConfigurationEl {
    enabled: PrimField<bool>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orphan_file_deletion_configuration:
        Option<Vec<GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_configuration:
        Option<Vec<GlueCatalogTableOptimizerConfigurationElRetentionConfigurationEl>>,
    dynamic: GlueCatalogTableOptimizerConfigurationElDynamic,
}

impl GlueCatalogTableOptimizerConfigurationEl {
    #[doc = "Set the field `orphan_file_deletion_configuration`.\n"]
    pub fn set_orphan_file_deletion_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.orphan_file_deletion_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.orphan_file_deletion_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `retention_configuration`.\n"]
    pub fn set_retention_configuration(
        mut self,
        v: impl Into<BlockAssignable<GlueCatalogTableOptimizerConfigurationElRetentionConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.retention_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.retention_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for GlueCatalogTableOptimizerConfigurationEl {
    type O = BlockAssignable<GlueCatalogTableOptimizerConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableOptimizerConfigurationEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}

impl BuildGlueCatalogTableOptimizerConfigurationEl {
    pub fn build(self) -> GlueCatalogTableOptimizerConfigurationEl {
        GlueCatalogTableOptimizerConfigurationEl {
            enabled: self.enabled,
            role_arn: self.role_arn,
            orphan_file_deletion_configuration: core::default::Default::default(),
            retention_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct GlueCatalogTableOptimizerConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueCatalogTableOptimizerConfigurationElRef {
    fn new(shared: StackShared, base: String) -> GlueCatalogTableOptimizerConfigurationElRef {
        GlueCatalogTableOptimizerConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableOptimizerConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `orphan_file_deletion_configuration` after provisioning.\n"]
    pub fn orphan_file_deletion_configuration(
        &self,
    ) -> ListRef<GlueCatalogTableOptimizerConfigurationElOrphanFileDeletionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.orphan_file_deletion_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `retention_configuration` after provisioning.\n"]
    pub fn retention_configuration(
        &self,
    ) -> ListRef<GlueCatalogTableOptimizerConfigurationElRetentionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retention_configuration", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogTableOptimizerDynamic {
    configuration: Option<DynamicBlock<GlueCatalogTableOptimizerConfigurationEl>>,
}
