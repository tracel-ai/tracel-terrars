use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataPollyVoicesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_additional_language_codes: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    voices: Option<Vec<DataPollyVoicesVoicesEl>>,
    dynamic: DataPollyVoicesDynamic,
}

struct DataPollyVoices_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataPollyVoicesData>,
}

#[derive(Clone)]
pub struct DataPollyVoices(Rc<DataPollyVoices_>);

impl DataPollyVoices {
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

    #[doc = "Set the field `engine`.\n"]
    pub fn set_engine(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().engine = Some(v.into());
        self
    }

    #[doc = "Set the field `include_additional_language_codes`.\n"]
    pub fn set_include_additional_language_codes(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_additional_language_codes = Some(v.into());
        self
    }

    #[doc = "Set the field `language_code`.\n"]
    pub fn set_language_code(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().language_code = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `voices`.\n"]
    pub fn set_voices(self, v: impl Into<BlockAssignable<DataPollyVoicesVoicesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().voices = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.voices = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_additional_language_codes` after provisioning.\n"]
    pub fn include_additional_language_codes(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_additional_language_codes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `voices` after provisioning.\n"]
    pub fn voices(&self) -> ListRef<DataPollyVoicesVoicesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.voices", self.extract_ref()),
        )
    }
}

impl Referable for DataPollyVoices {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataPollyVoices {}

impl ToListMappable for DataPollyVoices {
    type O = ListRef<DataPollyVoicesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataPollyVoices_ {
    fn extract_datasource_type(&self) -> String {
        "aws_polly_voices".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataPollyVoices {
    pub tf_id: String,
}

impl BuildDataPollyVoices {
    pub fn build(self, stack: &mut Stack) -> DataPollyVoices {
        let out = DataPollyVoices(Rc::new(DataPollyVoices_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataPollyVoicesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                engine: core::default::Default::default(),
                include_additional_language_codes: core::default::Default::default(),
                language_code: core::default::Default::default(),
                region: core::default::Default::default(),
                voices: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataPollyVoicesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataPollyVoicesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataPollyVoicesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_additional_language_codes` after provisioning.\n"]
    pub fn include_additional_language_codes(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_additional_language_codes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `voices` after provisioning.\n"]
    pub fn voices(&self) -> ListRef<DataPollyVoicesVoicesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.voices", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataPollyVoicesVoicesEl {}

impl DataPollyVoicesVoicesEl {}

impl ToListMappable for DataPollyVoicesVoicesEl {
    type O = BlockAssignable<DataPollyVoicesVoicesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPollyVoicesVoicesEl {}

impl BuildDataPollyVoicesVoicesEl {
    pub fn build(self) -> DataPollyVoicesVoicesEl {
        DataPollyVoicesVoicesEl {}
    }
}

pub struct DataPollyVoicesVoicesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataPollyVoicesVoicesElRef {
    fn new(shared: StackShared, base: String) -> DataPollyVoicesVoicesElRef {
        DataPollyVoicesVoicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPollyVoicesVoicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_language_codes` after provisioning.\n"]
    pub fn additional_language_codes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_language_codes", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `gender` after provisioning.\n"]
    pub fn gender(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gender", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `language_name` after provisioning.\n"]
    pub fn language_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `supported_engines` after provisioning.\n"]
    pub fn supported_engines(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_engines", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct DataPollyVoicesDynamic {
    voices: Option<DynamicBlock<DataPollyVoicesVoicesEl>>,
}
