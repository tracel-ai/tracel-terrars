use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct PrometheusScraperData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    scrape_configuration: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination: Option<Vec<PrometheusScraperDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_configuration: Option<Vec<PrometheusScraperRoleConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Vec<PrometheusScraperSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<PrometheusScraperTimeoutsEl>,
    dynamic: PrometheusScraperDynamic,
}
struct PrometheusScraper_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PrometheusScraperData>,
}
#[derive(Clone)]
pub struct PrometheusScraper(Rc<PrometheusScraper_>);
impl PrometheusScraper {
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
    #[doc = "Set the field `alias`.\n"]
    pub fn set_alias(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().alias = Some(v.into());
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
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        self,
        v: impl Into<BlockAssignable<PrometheusScraperDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `role_configuration`.\n"]
    pub fn set_role_configuration(
        self,
        v: impl Into<BlockAssignable<PrometheusScraperRoleConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().role_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.role_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(self, v: impl Into<BlockAssignable<PrometheusScraperSourceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<PrometheusScraperTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alias", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scrape_configuration` after provisioning.\n"]
    pub fn scrape_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scrape_configuration", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> ListRef<PrometheusScraperDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role_configuration` after provisioning.\n"]
    pub fn role_configuration(&self) -> ListRef<PrometheusScraperRoleConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.role_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<PrometheusScraperSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrometheusScraperTimeoutsElRef {
        PrometheusScraperTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for PrometheusScraper {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for PrometheusScraper {}
impl ToListMappable for PrometheusScraper {
    type O = ListRef<PrometheusScraperRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for PrometheusScraper_ {
    fn extract_resource_type(&self) -> String {
        "aws_prometheus_scraper".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildPrometheusScraper {
    pub tf_id: String,
    #[doc = ""]
    pub scrape_configuration: PrimField<String>,
}
impl BuildPrometheusScraper {
    pub fn build(self, stack: &mut Stack) -> PrometheusScraper {
        let out = PrometheusScraper(Rc::new(PrometheusScraper_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(PrometheusScraperData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                alias: core::default::Default::default(),
                region: core::default::Default::default(),
                scrape_configuration: self.scrape_configuration,
                tags: core::default::Default::default(),
                destination: core::default::Default::default(),
                role_configuration: core::default::Default::default(),
                source: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct PrometheusScraperRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrometheusScraperRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl PrometheusScraperRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alias", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scrape_configuration` after provisioning.\n"]
    pub fn scrape_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scrape_configuration", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> ListRef<PrometheusScraperDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role_configuration` after provisioning.\n"]
    pub fn role_configuration(&self) -> ListRef<PrometheusScraperRoleConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.role_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<PrometheusScraperSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrometheusScraperTimeoutsElRef {
        PrometheusScraperTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct PrometheusScraperDestinationElAmpEl {
    workspace_arn: PrimField<String>,
}
impl PrometheusScraperDestinationElAmpEl {}
impl ToListMappable for PrometheusScraperDestinationElAmpEl {
    type O = BlockAssignable<PrometheusScraperDestinationElAmpEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrometheusScraperDestinationElAmpEl {
    #[doc = ""]
    pub workspace_arn: PrimField<String>,
}
impl BuildPrometheusScraperDestinationElAmpEl {
    pub fn build(self) -> PrometheusScraperDestinationElAmpEl {
        PrometheusScraperDestinationElAmpEl {
            workspace_arn: self.workspace_arn,
        }
    }
}
pub struct PrometheusScraperDestinationElAmpElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrometheusScraperDestinationElAmpElRef {
    fn new(shared: StackShared, base: String) -> PrometheusScraperDestinationElAmpElRef {
        PrometheusScraperDestinationElAmpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrometheusScraperDestinationElAmpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `workspace_arn` after provisioning.\n"]
    pub fn workspace_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workspace_arn", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct PrometheusScraperDestinationElDynamic {
    amp: Option<DynamicBlock<PrometheusScraperDestinationElAmpEl>>,
}
#[derive(Serialize)]
pub struct PrometheusScraperDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    amp: Option<Vec<PrometheusScraperDestinationElAmpEl>>,
    dynamic: PrometheusScraperDestinationElDynamic,
}
impl PrometheusScraperDestinationEl {
    #[doc = "Set the field `amp`.\n"]
    pub fn set_amp(
        mut self,
        v: impl Into<BlockAssignable<PrometheusScraperDestinationElAmpEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.amp = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.amp = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PrometheusScraperDestinationEl {
    type O = BlockAssignable<PrometheusScraperDestinationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrometheusScraperDestinationEl {}
impl BuildPrometheusScraperDestinationEl {
    pub fn build(self) -> PrometheusScraperDestinationEl {
        PrometheusScraperDestinationEl {
            amp: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PrometheusScraperDestinationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrometheusScraperDestinationElRef {
    fn new(shared: StackShared, base: String) -> PrometheusScraperDestinationElRef {
        PrometheusScraperDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrometheusScraperDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `amp` after provisioning.\n"]
    pub fn amp(&self) -> ListRef<PrometheusScraperDestinationElAmpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.amp", self.base))
    }
}
#[derive(Serialize)]
pub struct PrometheusScraperRoleConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    source_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_role_arn: Option<PrimField<String>>,
}
impl PrometheusScraperRoleConfigurationEl {
    #[doc = "Set the field `source_role_arn`.\n"]
    pub fn set_source_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `target_role_arn`.\n"]
    pub fn set_target_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_role_arn = Some(v.into());
        self
    }
}
impl ToListMappable for PrometheusScraperRoleConfigurationEl {
    type O = BlockAssignable<PrometheusScraperRoleConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrometheusScraperRoleConfigurationEl {}
impl BuildPrometheusScraperRoleConfigurationEl {
    pub fn build(self) -> PrometheusScraperRoleConfigurationEl {
        PrometheusScraperRoleConfigurationEl {
            source_role_arn: core::default::Default::default(),
            target_role_arn: core::default::Default::default(),
        }
    }
}
pub struct PrometheusScraperRoleConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrometheusScraperRoleConfigurationElRef {
    fn new(shared: StackShared, base: String) -> PrometheusScraperRoleConfigurationElRef {
        PrometheusScraperRoleConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrometheusScraperRoleConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `source_role_arn` after provisioning.\n"]
    pub fn source_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_role_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_role_arn` after provisioning.\n"]
    pub fn target_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_role_arn", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PrometheusScraperSourceElEksEl {
    cluster_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    subnet_ids: SetField<PrimField<String>>,
}
impl PrometheusScraperSourceElEksEl {
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_group_ids = Some(v.into());
        self
    }
}
impl ToListMappable for PrometheusScraperSourceElEksEl {
    type O = BlockAssignable<PrometheusScraperSourceElEksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrometheusScraperSourceElEksEl {
    #[doc = ""]
    pub cluster_arn: PrimField<String>,
    #[doc = ""]
    pub subnet_ids: SetField<PrimField<String>>,
}
impl BuildPrometheusScraperSourceElEksEl {
    pub fn build(self) -> PrometheusScraperSourceElEksEl {
        PrometheusScraperSourceElEksEl {
            cluster_arn: self.cluster_arn,
            security_group_ids: core::default::Default::default(),
            subnet_ids: self.subnet_ids,
        }
    }
}
pub struct PrometheusScraperSourceElEksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrometheusScraperSourceElEksElRef {
    fn new(shared: StackShared, base: String) -> PrometheusScraperSourceElEksElRef {
        PrometheusScraperSourceElEksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrometheusScraperSourceElEksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster_arn` after provisioning.\n"]
    pub fn cluster_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}
#[derive(Serialize, Default)]
struct PrometheusScraperSourceElDynamic {
    eks: Option<DynamicBlock<PrometheusScraperSourceElEksEl>>,
}
#[derive(Serialize)]
pub struct PrometheusScraperSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    eks: Option<Vec<PrometheusScraperSourceElEksEl>>,
    dynamic: PrometheusScraperSourceElDynamic,
}
impl PrometheusScraperSourceEl {
    #[doc = "Set the field `eks`.\n"]
    pub fn set_eks(
        mut self,
        v: impl Into<BlockAssignable<PrometheusScraperSourceElEksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.eks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.eks = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PrometheusScraperSourceEl {
    type O = BlockAssignable<PrometheusScraperSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrometheusScraperSourceEl {}
impl BuildPrometheusScraperSourceEl {
    pub fn build(self) -> PrometheusScraperSourceEl {
        PrometheusScraperSourceEl {
            eks: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PrometheusScraperSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrometheusScraperSourceElRef {
    fn new(shared: StackShared, base: String) -> PrometheusScraperSourceElRef {
        PrometheusScraperSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrometheusScraperSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `eks` after provisioning.\n"]
    pub fn eks(&self) -> ListRef<PrometheusScraperSourceElEksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.eks", self.base))
    }
}
#[derive(Serialize)]
pub struct PrometheusScraperTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl PrometheusScraperTimeoutsEl {
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
impl ToListMappable for PrometheusScraperTimeoutsEl {
    type O = BlockAssignable<PrometheusScraperTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPrometheusScraperTimeoutsEl {}
impl BuildPrometheusScraperTimeoutsEl {
    pub fn build(self) -> PrometheusScraperTimeoutsEl {
        PrometheusScraperTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct PrometheusScraperTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PrometheusScraperTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> PrometheusScraperTimeoutsElRef {
        PrometheusScraperTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PrometheusScraperTimeoutsElRef {
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
#[derive(Serialize, Default)]
struct PrometheusScraperDynamic {
    destination: Option<DynamicBlock<PrometheusScraperDestinationEl>>,
    role_configuration: Option<DynamicBlock<PrometheusScraperRoleConfigurationEl>>,
    source: Option<DynamicBlock<PrometheusScraperSourceEl>>,
}
