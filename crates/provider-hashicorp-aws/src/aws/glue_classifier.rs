use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct GlueClassifierData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    csv_classifier: Option<Vec<GlueClassifierCsvClassifierEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grok_classifier: Option<Vec<GlueClassifierGrokClassifierEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json_classifier: Option<Vec<GlueClassifierJsonClassifierEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    xml_classifier: Option<Vec<GlueClassifierXmlClassifierEl>>,
    dynamic: GlueClassifierDynamic,
}
struct GlueClassifier_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GlueClassifierData>,
}
#[derive(Clone)]
pub struct GlueClassifier(Rc<GlueClassifier_>);
impl GlueClassifier {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `csv_classifier`.\n"]
    pub fn set_csv_classifier(
        self,
        v: impl Into<BlockAssignable<GlueClassifierCsvClassifierEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().csv_classifier = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.csv_classifier = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `grok_classifier`.\n"]
    pub fn set_grok_classifier(
        self,
        v: impl Into<BlockAssignable<GlueClassifierGrokClassifierEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().grok_classifier = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.grok_classifier = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `json_classifier`.\n"]
    pub fn set_json_classifier(
        self,
        v: impl Into<BlockAssignable<GlueClassifierJsonClassifierEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().json_classifier = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.json_classifier = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `xml_classifier`.\n"]
    pub fn set_xml_classifier(
        self,
        v: impl Into<BlockAssignable<GlueClassifierXmlClassifierEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().xml_classifier = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.xml_classifier = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `csv_classifier` after provisioning.\n"]
    pub fn csv_classifier(&self) -> ListRef<GlueClassifierCsvClassifierElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.csv_classifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `grok_classifier` after provisioning.\n"]
    pub fn grok_classifier(&self) -> ListRef<GlueClassifierGrokClassifierElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.grok_classifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `json_classifier` after provisioning.\n"]
    pub fn json_classifier(&self) -> ListRef<GlueClassifierJsonClassifierElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.json_classifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `xml_classifier` after provisioning.\n"]
    pub fn xml_classifier(&self) -> ListRef<GlueClassifierXmlClassifierElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.xml_classifier", self.extract_ref()),
        )
    }
}
impl Referable for GlueClassifier {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for GlueClassifier {}
impl ToListMappable for GlueClassifier {
    type O = ListRef<GlueClassifierRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for GlueClassifier_ {
    fn extract_resource_type(&self) -> String {
        "aws_glue_classifier".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildGlueClassifier {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildGlueClassifier {
    pub fn build(self, stack: &mut Stack) -> GlueClassifier {
        let out = GlueClassifier(Rc::new(GlueClassifier_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GlueClassifierData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                csv_classifier: core::default::Default::default(),
                grok_classifier: core::default::Default::default(),
                json_classifier: core::default::Default::default(),
                xml_classifier: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct GlueClassifierRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueClassifierRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl GlueClassifierRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `csv_classifier` after provisioning.\n"]
    pub fn csv_classifier(&self) -> ListRef<GlueClassifierCsvClassifierElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.csv_classifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `grok_classifier` after provisioning.\n"]
    pub fn grok_classifier(&self) -> ListRef<GlueClassifierGrokClassifierElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.grok_classifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `json_classifier` after provisioning.\n"]
    pub fn json_classifier(&self) -> ListRef<GlueClassifierJsonClassifierElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.json_classifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `xml_classifier` after provisioning.\n"]
    pub fn xml_classifier(&self) -> ListRef<GlueClassifierXmlClassifierElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.xml_classifier", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct GlueClassifierCsvClassifierEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_single_column: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contains_header: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_datatype_configured: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_datatypes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delimiter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_value_trimming: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quote_symbol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serde: Option<PrimField<String>>,
}
impl GlueClassifierCsvClassifierEl {
    #[doc = "Set the field `allow_single_column`.\n"]
    pub fn set_allow_single_column(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_single_column = Some(v.into());
        self
    }
    #[doc = "Set the field `contains_header`.\n"]
    pub fn set_contains_header(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contains_header = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_datatype_configured`.\n"]
    pub fn set_custom_datatype_configured(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.custom_datatype_configured = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_datatypes`.\n"]
    pub fn set_custom_datatypes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.custom_datatypes = Some(v.into());
        self
    }
    #[doc = "Set the field `delimiter`.\n"]
    pub fn set_delimiter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delimiter = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_value_trimming`.\n"]
    pub fn set_disable_value_trimming(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_value_trimming = Some(v.into());
        self
    }
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.header = Some(v.into());
        self
    }
    #[doc = "Set the field `quote_symbol`.\n"]
    pub fn set_quote_symbol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.quote_symbol = Some(v.into());
        self
    }
    #[doc = "Set the field `serde`.\n"]
    pub fn set_serde(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.serde = Some(v.into());
        self
    }
}
impl ToListMappable for GlueClassifierCsvClassifierEl {
    type O = BlockAssignable<GlueClassifierCsvClassifierEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueClassifierCsvClassifierEl {}
impl BuildGlueClassifierCsvClassifierEl {
    pub fn build(self) -> GlueClassifierCsvClassifierEl {
        GlueClassifierCsvClassifierEl {
            allow_single_column: core::default::Default::default(),
            contains_header: core::default::Default::default(),
            custom_datatype_configured: core::default::Default::default(),
            custom_datatypes: core::default::Default::default(),
            delimiter: core::default::Default::default(),
            disable_value_trimming: core::default::Default::default(),
            header: core::default::Default::default(),
            quote_symbol: core::default::Default::default(),
            serde: core::default::Default::default(),
        }
    }
}
pub struct GlueClassifierCsvClassifierElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueClassifierCsvClassifierElRef {
    fn new(shared: StackShared, base: String) -> GlueClassifierCsvClassifierElRef {
        GlueClassifierCsvClassifierElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueClassifierCsvClassifierElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_single_column` after provisioning.\n"]
    pub fn allow_single_column(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_single_column", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `contains_header` after provisioning.\n"]
    pub fn contains_header(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.contains_header", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_datatype_configured` after provisioning.\n"]
    pub fn custom_datatype_configured(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_datatype_configured", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_datatypes` after provisioning.\n"]
    pub fn custom_datatypes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_datatypes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `delimiter` after provisioning.\n"]
    pub fn delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delimiter", self.base))
    }
    #[doc = "Get a reference to the value of field `disable_value_trimming` after provisioning.\n"]
    pub fn disable_value_trimming(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_value_trimming", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }
    #[doc = "Get a reference to the value of field `quote_symbol` after provisioning.\n"]
    pub fn quote_symbol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.quote_symbol", self.base))
    }
    #[doc = "Get a reference to the value of field `serde` after provisioning.\n"]
    pub fn serde(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.serde", self.base))
    }
}
#[derive(Serialize)]
pub struct GlueClassifierGrokClassifierEl {
    classification: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_patterns: Option<PrimField<String>>,
    grok_pattern: PrimField<String>,
}
impl GlueClassifierGrokClassifierEl {
    #[doc = "Set the field `custom_patterns`.\n"]
    pub fn set_custom_patterns(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.custom_patterns = Some(v.into());
        self
    }
}
impl ToListMappable for GlueClassifierGrokClassifierEl {
    type O = BlockAssignable<GlueClassifierGrokClassifierEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueClassifierGrokClassifierEl {
    #[doc = ""]
    pub classification: PrimField<String>,
    #[doc = ""]
    pub grok_pattern: PrimField<String>,
}
impl BuildGlueClassifierGrokClassifierEl {
    pub fn build(self) -> GlueClassifierGrokClassifierEl {
        GlueClassifierGrokClassifierEl {
            classification: self.classification,
            custom_patterns: core::default::Default::default(),
            grok_pattern: self.grok_pattern,
        }
    }
}
pub struct GlueClassifierGrokClassifierElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueClassifierGrokClassifierElRef {
    fn new(shared: StackShared, base: String) -> GlueClassifierGrokClassifierElRef {
        GlueClassifierGrokClassifierElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueClassifierGrokClassifierElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `classification` after provisioning.\n"]
    pub fn classification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.classification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_patterns` after provisioning.\n"]
    pub fn custom_patterns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_patterns", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `grok_pattern` after provisioning.\n"]
    pub fn grok_pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.grok_pattern", self.base))
    }
}
#[derive(Serialize)]
pub struct GlueClassifierJsonClassifierEl {
    json_path: PrimField<String>,
}
impl GlueClassifierJsonClassifierEl {}
impl ToListMappable for GlueClassifierJsonClassifierEl {
    type O = BlockAssignable<GlueClassifierJsonClassifierEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueClassifierJsonClassifierEl {
    #[doc = ""]
    pub json_path: PrimField<String>,
}
impl BuildGlueClassifierJsonClassifierEl {
    pub fn build(self) -> GlueClassifierJsonClassifierEl {
        GlueClassifierJsonClassifierEl {
            json_path: self.json_path,
        }
    }
}
pub struct GlueClassifierJsonClassifierElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueClassifierJsonClassifierElRef {
    fn new(shared: StackShared, base: String) -> GlueClassifierJsonClassifierElRef {
        GlueClassifierJsonClassifierElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueClassifierJsonClassifierElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `json_path` after provisioning.\n"]
    pub fn json_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.json_path", self.base))
    }
}
#[derive(Serialize)]
pub struct GlueClassifierXmlClassifierEl {
    classification: PrimField<String>,
    row_tag: PrimField<String>,
}
impl GlueClassifierXmlClassifierEl {}
impl ToListMappable for GlueClassifierXmlClassifierEl {
    type O = BlockAssignable<GlueClassifierXmlClassifierEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueClassifierXmlClassifierEl {
    #[doc = ""]
    pub classification: PrimField<String>,
    #[doc = ""]
    pub row_tag: PrimField<String>,
}
impl BuildGlueClassifierXmlClassifierEl {
    pub fn build(self) -> GlueClassifierXmlClassifierEl {
        GlueClassifierXmlClassifierEl {
            classification: self.classification,
            row_tag: self.row_tag,
        }
    }
}
pub struct GlueClassifierXmlClassifierElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueClassifierXmlClassifierElRef {
    fn new(shared: StackShared, base: String) -> GlueClassifierXmlClassifierElRef {
        GlueClassifierXmlClassifierElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueClassifierXmlClassifierElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `classification` after provisioning.\n"]
    pub fn classification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.classification", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `row_tag` after provisioning.\n"]
    pub fn row_tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.row_tag", self.base))
    }
}
#[derive(Serialize, Default)]
struct GlueClassifierDynamic {
    csv_classifier: Option<DynamicBlock<GlueClassifierCsvClassifierEl>>,
    grok_classifier: Option<DynamicBlock<GlueClassifierGrokClassifierEl>>,
    json_classifier: Option<DynamicBlock<GlueClassifierJsonClassifierEl>>,
    xml_classifier: Option<DynamicBlock<GlueClassifierXmlClassifierEl>>,
}
