use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataRdsEngineVersionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_only: Option<PrimField<bool>>,
    engine: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_major_target: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_minor_target: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_all: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    latest: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter_group_family: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_major_targets: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_upgrade_targets: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_versions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataRdsEngineVersionFilterEl>>,
    dynamic: DataRdsEngineVersionDynamic,
}

struct DataRdsEngineVersion_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRdsEngineVersionData>,
}

#[derive(Clone)]
pub struct DataRdsEngineVersion(Rc<DataRdsEngineVersion_>);

impl DataRdsEngineVersion {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc = "Set the field `default_only`.\n"]
    pub fn set_default_only(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().default_only = Some(v.into());
        self
    }

    #[doc = "Set the field `has_major_target`.\n"]
    pub fn set_has_major_target(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().has_major_target = Some(v.into());
        self
    }

    #[doc = "Set the field `has_minor_target`.\n"]
    pub fn set_has_minor_target(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().has_minor_target = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `include_all`.\n"]
    pub fn set_include_all(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_all = Some(v.into());
        self
    }

    #[doc = "Set the field `latest`.\n"]
    pub fn set_latest(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().latest = Some(v.into());
        self
    }

    #[doc = "Set the field `parameter_group_family`.\n"]
    pub fn set_parameter_group_family(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parameter_group_family = Some(v.into());
        self
    }

    #[doc = "Set the field `preferred_major_targets`.\n"]
    pub fn set_preferred_major_targets(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().preferred_major_targets = Some(v.into());
        self
    }

    #[doc = "Set the field `preferred_upgrade_targets`.\n"]
    pub fn set_preferred_upgrade_targets(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().preferred_upgrade_targets = Some(v.into());
        self
    }

    #[doc = "Set the field `preferred_versions`.\n"]
    pub fn set_preferred_versions(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().preferred_versions = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version = Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataRdsEngineVersionFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `default_character_set` after provisioning.\n"]
    pub fn default_character_set(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_character_set", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_only` after provisioning.\n"]
    pub fn default_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_only", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `engine_description` after provisioning.\n"]
    pub fn engine_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `exportable_log_types` after provisioning.\n"]
    pub fn exportable_log_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.exportable_log_types", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `has_major_target` after provisioning.\n"]
    pub fn has_major_target(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.has_major_target", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `has_minor_target` after provisioning.\n"]
    pub fn has_minor_target(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.has_minor_target", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_all` after provisioning.\n"]
    pub fn include_all(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `latest` after provisioning.\n"]
    pub fn latest(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parameter_group_family` after provisioning.\n"]
    pub fn parameter_group_family(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parameter_group_family", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `preferred_major_targets` after provisioning.\n"]
    pub fn preferred_major_targets(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.preferred_major_targets", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `preferred_upgrade_targets` after provisioning.\n"]
    pub fn preferred_upgrade_targets(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.preferred_upgrade_targets", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `preferred_versions` after provisioning.\n"]
    pub fn preferred_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.preferred_versions", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `supported_character_sets` after provisioning.\n"]
    pub fn supported_character_sets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.supported_character_sets", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supported_feature_names` after provisioning.\n"]
    pub fn supported_feature_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.supported_feature_names", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supported_modes` after provisioning.\n"]
    pub fn supported_modes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.supported_modes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supported_timezones` after provisioning.\n"]
    pub fn supported_timezones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.supported_timezones", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supports_certificate_rotation_without_restart` after provisioning.\n"]
    pub fn supports_certificate_rotation_without_restart(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.supports_certificate_rotation_without_restart",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `supports_global_databases` after provisioning.\n"]
    pub fn supports_global_databases(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.supports_global_databases", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supports_integrations` after provisioning.\n"]
    pub fn supports_integrations(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.supports_integrations", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supports_limitless_database` after provisioning.\n"]
    pub fn supports_limitless_database(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.supports_limitless_database", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supports_local_write_forwarding` after provisioning.\n"]
    pub fn supports_local_write_forwarding(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.supports_local_write_forwarding", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supports_log_exports_to_cloudwatch` after provisioning.\n"]
    pub fn supports_log_exports_to_cloudwatch(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.supports_log_exports_to_cloudwatch", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supports_parallel_query` after provisioning.\n"]
    pub fn supports_parallel_query(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.supports_parallel_query", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supports_read_replica` after provisioning.\n"]
    pub fn supports_read_replica(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.supports_read_replica", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `valid_major_targets` after provisioning.\n"]
    pub fn valid_major_targets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.valid_major_targets", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `valid_minor_targets` after provisioning.\n"]
    pub fn valid_minor_targets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.valid_minor_targets", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `valid_upgrade_targets` after provisioning.\n"]
    pub fn valid_upgrade_targets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.valid_upgrade_targets", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version_actual` after provisioning.\n"]
    pub fn version_actual(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_actual", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version_description` after provisioning.\n"]
    pub fn version_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_description", self.extract_ref()),
        )
    }
}

impl Referable for DataRdsEngineVersion {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataRdsEngineVersion {}

impl ToListMappable for DataRdsEngineVersion {
    type O = ListRef<DataRdsEngineVersionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRdsEngineVersion_ {
    fn extract_datasource_type(&self) -> String {
        "aws_rds_engine_version".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRdsEngineVersion {
    pub tf_id: String,
    #[doc = ""]
    pub engine: PrimField<String>,
}

impl BuildDataRdsEngineVersion {
    pub fn build(self, stack: &mut Stack) -> DataRdsEngineVersion {
        let out = DataRdsEngineVersion(Rc::new(DataRdsEngineVersion_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRdsEngineVersionData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                default_only: core::default::Default::default(),
                engine: self.engine,
                has_major_target: core::default::Default::default(),
                has_minor_target: core::default::Default::default(),
                id: core::default::Default::default(),
                include_all: core::default::Default::default(),
                latest: core::default::Default::default(),
                parameter_group_family: core::default::Default::default(),
                preferred_major_targets: core::default::Default::default(),
                preferred_upgrade_targets: core::default::Default::default(),
                preferred_versions: core::default::Default::default(),
                region: core::default::Default::default(),
                version: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRdsEngineVersionRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRdsEngineVersionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataRdsEngineVersionRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `default_character_set` after provisioning.\n"]
    pub fn default_character_set(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_character_set", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_only` after provisioning.\n"]
    pub fn default_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_only", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `engine_description` after provisioning.\n"]
    pub fn engine_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `exportable_log_types` after provisioning.\n"]
    pub fn exportable_log_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.exportable_log_types", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `has_major_target` after provisioning.\n"]
    pub fn has_major_target(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.has_major_target", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `has_minor_target` after provisioning.\n"]
    pub fn has_minor_target(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.has_minor_target", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_all` after provisioning.\n"]
    pub fn include_all(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `latest` after provisioning.\n"]
    pub fn latest(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parameter_group_family` after provisioning.\n"]
    pub fn parameter_group_family(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parameter_group_family", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `preferred_major_targets` after provisioning.\n"]
    pub fn preferred_major_targets(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.preferred_major_targets", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `preferred_upgrade_targets` after provisioning.\n"]
    pub fn preferred_upgrade_targets(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.preferred_upgrade_targets", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `preferred_versions` after provisioning.\n"]
    pub fn preferred_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.preferred_versions", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `supported_character_sets` after provisioning.\n"]
    pub fn supported_character_sets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.supported_character_sets", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supported_feature_names` after provisioning.\n"]
    pub fn supported_feature_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.supported_feature_names", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supported_modes` after provisioning.\n"]
    pub fn supported_modes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.supported_modes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supported_timezones` after provisioning.\n"]
    pub fn supported_timezones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.supported_timezones", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supports_certificate_rotation_without_restart` after provisioning.\n"]
    pub fn supports_certificate_rotation_without_restart(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.supports_certificate_rotation_without_restart",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `supports_global_databases` after provisioning.\n"]
    pub fn supports_global_databases(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.supports_global_databases", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supports_integrations` after provisioning.\n"]
    pub fn supports_integrations(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.supports_integrations", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supports_limitless_database` after provisioning.\n"]
    pub fn supports_limitless_database(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.supports_limitless_database", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supports_local_write_forwarding` after provisioning.\n"]
    pub fn supports_local_write_forwarding(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.supports_local_write_forwarding", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supports_log_exports_to_cloudwatch` after provisioning.\n"]
    pub fn supports_log_exports_to_cloudwatch(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.supports_log_exports_to_cloudwatch", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supports_parallel_query` after provisioning.\n"]
    pub fn supports_parallel_query(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.supports_parallel_query", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supports_read_replica` after provisioning.\n"]
    pub fn supports_read_replica(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.supports_read_replica", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `valid_major_targets` after provisioning.\n"]
    pub fn valid_major_targets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.valid_major_targets", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `valid_minor_targets` after provisioning.\n"]
    pub fn valid_minor_targets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.valid_minor_targets", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `valid_upgrade_targets` after provisioning.\n"]
    pub fn valid_upgrade_targets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.valid_upgrade_targets", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version_actual` after provisioning.\n"]
    pub fn version_actual(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_actual", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version_description` after provisioning.\n"]
    pub fn version_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_description", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataRdsEngineVersionFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl DataRdsEngineVersionFilterEl {}

impl ToListMappable for DataRdsEngineVersionFilterEl {
    type O = BlockAssignable<DataRdsEngineVersionFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRdsEngineVersionFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildDataRdsEngineVersionFilterEl {
    pub fn build(self) -> DataRdsEngineVersionFilterEl {
        DataRdsEngineVersionFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataRdsEngineVersionFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRdsEngineVersionFilterElRef {
    fn new(shared: StackShared, base: String) -> DataRdsEngineVersionFilterElRef {
        DataRdsEngineVersionFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRdsEngineVersionFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataRdsEngineVersionDynamic {
    filter: Option<DynamicBlock<DataRdsEngineVersionFilterEl>>,
}
