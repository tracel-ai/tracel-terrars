use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataQuicksightThemeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    theme_id: PrimField<String>,
}
struct DataQuicksightTheme_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataQuicksightThemeData>,
}
#[derive(Clone)]
pub struct DataQuicksightTheme(Rc<DataQuicksightTheme_>);
impl DataQuicksightTheme {
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
    #[doc = "Set the field `aws_account_id`.\n"]
    pub fn set_aws_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().aws_account_id = Some(v.into());
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
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `base_theme_id` after provisioning.\n"]
    pub fn base_theme_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_theme_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<DataQuicksightThemeConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_updated_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> ListRef<DataQuicksightThemePermissionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.permissions", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `theme_id` after provisioning.\n"]
    pub fn theme_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.theme_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_description` after provisioning.\n"]
    pub fn version_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_number` after provisioning.\n"]
    pub fn version_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_number", self.extract_ref()),
        )
    }
}
impl Referable for DataQuicksightTheme {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataQuicksightTheme {}
impl ToListMappable for DataQuicksightTheme {
    type O = ListRef<DataQuicksightThemeRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataQuicksightTheme_ {
    fn extract_datasource_type(&self) -> String {
        "aws_quicksight_theme".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataQuicksightTheme {
    pub tf_id: String,
    #[doc = ""]
    pub theme_id: PrimField<String>,
}
impl BuildDataQuicksightTheme {
    pub fn build(self, stack: &mut Stack) -> DataQuicksightTheme {
        let out = DataQuicksightTheme(Rc::new(DataQuicksightTheme_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataQuicksightThemeData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                aws_account_id: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                theme_id: self.theme_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataQuicksightThemeRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightThemeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataQuicksightThemeRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `base_theme_id` after provisioning.\n"]
    pub fn base_theme_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_theme_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<DataQuicksightThemeConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_updated_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> ListRef<DataQuicksightThemePermissionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.permissions", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `theme_id` after provisioning.\n"]
    pub fn theme_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.theme_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_description` after provisioning.\n"]
    pub fn version_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_number` after provisioning.\n"]
    pub fn version_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_number", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataQuicksightThemeConfigurationElDataColorPaletteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    colors: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    empty_fill_color: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_max_gradient: Option<ListField<PrimField<String>>>,
}
impl DataQuicksightThemeConfigurationElDataColorPaletteEl {
    #[doc = "Set the field `colors`.\n"]
    pub fn set_colors(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.colors = Some(v.into());
        self
    }
    #[doc = "Set the field `empty_fill_color`.\n"]
    pub fn set_empty_fill_color(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.empty_fill_color = Some(v.into());
        self
    }
    #[doc = "Set the field `min_max_gradient`.\n"]
    pub fn set_min_max_gradient(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.min_max_gradient = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightThemeConfigurationElDataColorPaletteEl {
    type O = BlockAssignable<DataQuicksightThemeConfigurationElDataColorPaletteEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightThemeConfigurationElDataColorPaletteEl {}
impl BuildDataQuicksightThemeConfigurationElDataColorPaletteEl {
    pub fn build(self) -> DataQuicksightThemeConfigurationElDataColorPaletteEl {
        DataQuicksightThemeConfigurationElDataColorPaletteEl {
            colors: core::default::Default::default(),
            empty_fill_color: core::default::Default::default(),
            min_max_gradient: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightThemeConfigurationElDataColorPaletteElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightThemeConfigurationElDataColorPaletteElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightThemeConfigurationElDataColorPaletteElRef {
        DataQuicksightThemeConfigurationElDataColorPaletteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightThemeConfigurationElDataColorPaletteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `colors` after provisioning.\n"]
    pub fn colors(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.colors", self.base))
    }
    #[doc = "Get a reference to the value of field `empty_fill_color` after provisioning.\n"]
    pub fn empty_fill_color(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.empty_fill_color", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_max_gradient` after provisioning.\n"]
    pub fn min_max_gradient(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.min_max_gradient", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataQuicksightThemeConfigurationElSheetElTileElBorderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<PrimField<bool>>,
}
impl DataQuicksightThemeConfigurationElSheetElTileElBorderEl {
    #[doc = "Set the field `show`.\n"]
    pub fn set_show(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.show = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightThemeConfigurationElSheetElTileElBorderEl {
    type O = BlockAssignable<DataQuicksightThemeConfigurationElSheetElTileElBorderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightThemeConfigurationElSheetElTileElBorderEl {}
impl BuildDataQuicksightThemeConfigurationElSheetElTileElBorderEl {
    pub fn build(self) -> DataQuicksightThemeConfigurationElSheetElTileElBorderEl {
        DataQuicksightThemeConfigurationElSheetElTileElBorderEl {
            show: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightThemeConfigurationElSheetElTileElBorderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightThemeConfigurationElSheetElTileElBorderElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightThemeConfigurationElSheetElTileElBorderElRef {
        DataQuicksightThemeConfigurationElSheetElTileElBorderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightThemeConfigurationElSheetElTileElBorderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `show` after provisioning.\n"]
    pub fn show(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.show", self.base))
    }
}
#[derive(Serialize)]
pub struct DataQuicksightThemeConfigurationElSheetElTileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    border: Option<ListField<DataQuicksightThemeConfigurationElSheetElTileElBorderEl>>,
}
impl DataQuicksightThemeConfigurationElSheetElTileEl {
    #[doc = "Set the field `border`.\n"]
    pub fn set_border(
        mut self,
        v: impl Into<ListField<DataQuicksightThemeConfigurationElSheetElTileElBorderEl>>,
    ) -> Self {
        self.border = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightThemeConfigurationElSheetElTileEl {
    type O = BlockAssignable<DataQuicksightThemeConfigurationElSheetElTileEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightThemeConfigurationElSheetElTileEl {}
impl BuildDataQuicksightThemeConfigurationElSheetElTileEl {
    pub fn build(self) -> DataQuicksightThemeConfigurationElSheetElTileEl {
        DataQuicksightThemeConfigurationElSheetElTileEl {
            border: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightThemeConfigurationElSheetElTileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightThemeConfigurationElSheetElTileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightThemeConfigurationElSheetElTileElRef {
        DataQuicksightThemeConfigurationElSheetElTileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightThemeConfigurationElSheetElTileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `border` after provisioning.\n"]
    pub fn border(&self) -> ListRef<DataQuicksightThemeConfigurationElSheetElTileElBorderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.border", self.base))
    }
}
#[derive(Serialize)]
pub struct DataQuicksightThemeConfigurationElSheetElTileLayoutElGutterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<PrimField<bool>>,
}
impl DataQuicksightThemeConfigurationElSheetElTileLayoutElGutterEl {
    #[doc = "Set the field `show`.\n"]
    pub fn set_show(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.show = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightThemeConfigurationElSheetElTileLayoutElGutterEl {
    type O = BlockAssignable<DataQuicksightThemeConfigurationElSheetElTileLayoutElGutterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightThemeConfigurationElSheetElTileLayoutElGutterEl {}
impl BuildDataQuicksightThemeConfigurationElSheetElTileLayoutElGutterEl {
    pub fn build(self) -> DataQuicksightThemeConfigurationElSheetElTileLayoutElGutterEl {
        DataQuicksightThemeConfigurationElSheetElTileLayoutElGutterEl {
            show: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightThemeConfigurationElSheetElTileLayoutElGutterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightThemeConfigurationElSheetElTileLayoutElGutterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightThemeConfigurationElSheetElTileLayoutElGutterElRef {
        DataQuicksightThemeConfigurationElSheetElTileLayoutElGutterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightThemeConfigurationElSheetElTileLayoutElGutterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `show` after provisioning.\n"]
    pub fn show(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.show", self.base))
    }
}
#[derive(Serialize)]
pub struct DataQuicksightThemeConfigurationElSheetElTileLayoutElMarginEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<PrimField<bool>>,
}
impl DataQuicksightThemeConfigurationElSheetElTileLayoutElMarginEl {
    #[doc = "Set the field `show`.\n"]
    pub fn set_show(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.show = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightThemeConfigurationElSheetElTileLayoutElMarginEl {
    type O = BlockAssignable<DataQuicksightThemeConfigurationElSheetElTileLayoutElMarginEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightThemeConfigurationElSheetElTileLayoutElMarginEl {}
impl BuildDataQuicksightThemeConfigurationElSheetElTileLayoutElMarginEl {
    pub fn build(self) -> DataQuicksightThemeConfigurationElSheetElTileLayoutElMarginEl {
        DataQuicksightThemeConfigurationElSheetElTileLayoutElMarginEl {
            show: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightThemeConfigurationElSheetElTileLayoutElMarginElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightThemeConfigurationElSheetElTileLayoutElMarginElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightThemeConfigurationElSheetElTileLayoutElMarginElRef {
        DataQuicksightThemeConfigurationElSheetElTileLayoutElMarginElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightThemeConfigurationElSheetElTileLayoutElMarginElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `show` after provisioning.\n"]
    pub fn show(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.show", self.base))
    }
}
#[derive(Serialize)]
pub struct DataQuicksightThemeConfigurationElSheetElTileLayoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gutter: Option<ListField<DataQuicksightThemeConfigurationElSheetElTileLayoutElGutterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    margin: Option<ListField<DataQuicksightThemeConfigurationElSheetElTileLayoutElMarginEl>>,
}
impl DataQuicksightThemeConfigurationElSheetElTileLayoutEl {
    #[doc = "Set the field `gutter`.\n"]
    pub fn set_gutter(
        mut self,
        v: impl Into<ListField<DataQuicksightThemeConfigurationElSheetElTileLayoutElGutterEl>>,
    ) -> Self {
        self.gutter = Some(v.into());
        self
    }
    #[doc = "Set the field `margin`.\n"]
    pub fn set_margin(
        mut self,
        v: impl Into<ListField<DataQuicksightThemeConfigurationElSheetElTileLayoutElMarginEl>>,
    ) -> Self {
        self.margin = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightThemeConfigurationElSheetElTileLayoutEl {
    type O = BlockAssignable<DataQuicksightThemeConfigurationElSheetElTileLayoutEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightThemeConfigurationElSheetElTileLayoutEl {}
impl BuildDataQuicksightThemeConfigurationElSheetElTileLayoutEl {
    pub fn build(self) -> DataQuicksightThemeConfigurationElSheetElTileLayoutEl {
        DataQuicksightThemeConfigurationElSheetElTileLayoutEl {
            gutter: core::default::Default::default(),
            margin: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightThemeConfigurationElSheetElTileLayoutElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightThemeConfigurationElSheetElTileLayoutElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightThemeConfigurationElSheetElTileLayoutElRef {
        DataQuicksightThemeConfigurationElSheetElTileLayoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightThemeConfigurationElSheetElTileLayoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gutter` after provisioning.\n"]
    pub fn gutter(
        &self,
    ) -> ListRef<DataQuicksightThemeConfigurationElSheetElTileLayoutElGutterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gutter", self.base))
    }
    #[doc = "Get a reference to the value of field `margin` after provisioning.\n"]
    pub fn margin(
        &self,
    ) -> ListRef<DataQuicksightThemeConfigurationElSheetElTileLayoutElMarginElRef> {
        ListRef::new(self.shared().clone(), format!("{}.margin", self.base))
    }
}
#[derive(Serialize)]
pub struct DataQuicksightThemeConfigurationElSheetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tile: Option<ListField<DataQuicksightThemeConfigurationElSheetElTileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tile_layout: Option<ListField<DataQuicksightThemeConfigurationElSheetElTileLayoutEl>>,
}
impl DataQuicksightThemeConfigurationElSheetEl {
    #[doc = "Set the field `tile`.\n"]
    pub fn set_tile(
        mut self,
        v: impl Into<ListField<DataQuicksightThemeConfigurationElSheetElTileEl>>,
    ) -> Self {
        self.tile = Some(v.into());
        self
    }
    #[doc = "Set the field `tile_layout`.\n"]
    pub fn set_tile_layout(
        mut self,
        v: impl Into<ListField<DataQuicksightThemeConfigurationElSheetElTileLayoutEl>>,
    ) -> Self {
        self.tile_layout = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightThemeConfigurationElSheetEl {
    type O = BlockAssignable<DataQuicksightThemeConfigurationElSheetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightThemeConfigurationElSheetEl {}
impl BuildDataQuicksightThemeConfigurationElSheetEl {
    pub fn build(self) -> DataQuicksightThemeConfigurationElSheetEl {
        DataQuicksightThemeConfigurationElSheetEl {
            tile: core::default::Default::default(),
            tile_layout: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightThemeConfigurationElSheetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightThemeConfigurationElSheetElRef {
    fn new(shared: StackShared, base: String) -> DataQuicksightThemeConfigurationElSheetElRef {
        DataQuicksightThemeConfigurationElSheetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightThemeConfigurationElSheetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `tile` after provisioning.\n"]
    pub fn tile(&self) -> ListRef<DataQuicksightThemeConfigurationElSheetElTileElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tile", self.base))
    }
    #[doc = "Get a reference to the value of field `tile_layout` after provisioning.\n"]
    pub fn tile_layout(&self) -> ListRef<DataQuicksightThemeConfigurationElSheetElTileLayoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tile_layout", self.base))
    }
}
#[derive(Serialize)]
pub struct DataQuicksightThemeConfigurationElTypographyElFontFamiliesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    font_family: Option<PrimField<String>>,
}
impl DataQuicksightThemeConfigurationElTypographyElFontFamiliesEl {
    #[doc = "Set the field `font_family`.\n"]
    pub fn set_font_family(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.font_family = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightThemeConfigurationElTypographyElFontFamiliesEl {
    type O = BlockAssignable<DataQuicksightThemeConfigurationElTypographyElFontFamiliesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightThemeConfigurationElTypographyElFontFamiliesEl {}
impl BuildDataQuicksightThemeConfigurationElTypographyElFontFamiliesEl {
    pub fn build(self) -> DataQuicksightThemeConfigurationElTypographyElFontFamiliesEl {
        DataQuicksightThemeConfigurationElTypographyElFontFamiliesEl {
            font_family: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightThemeConfigurationElTypographyElFontFamiliesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightThemeConfigurationElTypographyElFontFamiliesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightThemeConfigurationElTypographyElFontFamiliesElRef {
        DataQuicksightThemeConfigurationElTypographyElFontFamiliesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightThemeConfigurationElTypographyElFontFamiliesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `font_family` after provisioning.\n"]
    pub fn font_family(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.font_family", self.base))
    }
}
#[derive(Serialize)]
pub struct DataQuicksightThemeConfigurationElTypographyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    font_families: Option<ListField<DataQuicksightThemeConfigurationElTypographyElFontFamiliesEl>>,
}
impl DataQuicksightThemeConfigurationElTypographyEl {
    #[doc = "Set the field `font_families`.\n"]
    pub fn set_font_families(
        mut self,
        v: impl Into<ListField<DataQuicksightThemeConfigurationElTypographyElFontFamiliesEl>>,
    ) -> Self {
        self.font_families = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightThemeConfigurationElTypographyEl {
    type O = BlockAssignable<DataQuicksightThemeConfigurationElTypographyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightThemeConfigurationElTypographyEl {}
impl BuildDataQuicksightThemeConfigurationElTypographyEl {
    pub fn build(self) -> DataQuicksightThemeConfigurationElTypographyEl {
        DataQuicksightThemeConfigurationElTypographyEl {
            font_families: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightThemeConfigurationElTypographyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightThemeConfigurationElTypographyElRef {
    fn new(shared: StackShared, base: String) -> DataQuicksightThemeConfigurationElTypographyElRef {
        DataQuicksightThemeConfigurationElTypographyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightThemeConfigurationElTypographyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `font_families` after provisioning.\n"]
    pub fn font_families(
        &self,
    ) -> ListRef<DataQuicksightThemeConfigurationElTypographyElFontFamiliesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.font_families", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataQuicksightThemeConfigurationElUiColorPaletteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accent_foreground: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    danger: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    danger_foreground: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimension_foreground: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    measure: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    measure_foreground: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_background: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_foreground: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_background: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_foreground: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_foreground: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warning: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warning_foreground: Option<PrimField<String>>,
}
impl DataQuicksightThemeConfigurationElUiColorPaletteEl {
    #[doc = "Set the field `accent`.\n"]
    pub fn set_accent(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accent = Some(v.into());
        self
    }
    #[doc = "Set the field `accent_foreground`.\n"]
    pub fn set_accent_foreground(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accent_foreground = Some(v.into());
        self
    }
    #[doc = "Set the field `danger`.\n"]
    pub fn set_danger(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.danger = Some(v.into());
        self
    }
    #[doc = "Set the field `danger_foreground`.\n"]
    pub fn set_danger_foreground(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.danger_foreground = Some(v.into());
        self
    }
    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dimension = Some(v.into());
        self
    }
    #[doc = "Set the field `dimension_foreground`.\n"]
    pub fn set_dimension_foreground(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dimension_foreground = Some(v.into());
        self
    }
    #[doc = "Set the field `measure`.\n"]
    pub fn set_measure(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.measure = Some(v.into());
        self
    }
    #[doc = "Set the field `measure_foreground`.\n"]
    pub fn set_measure_foreground(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.measure_foreground = Some(v.into());
        self
    }
    #[doc = "Set the field `primary_background`.\n"]
    pub fn set_primary_background(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.primary_background = Some(v.into());
        self
    }
    #[doc = "Set the field `primary_foreground`.\n"]
    pub fn set_primary_foreground(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.primary_foreground = Some(v.into());
        self
    }
    #[doc = "Set the field `secondary_background`.\n"]
    pub fn set_secondary_background(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secondary_background = Some(v.into());
        self
    }
    #[doc = "Set the field `secondary_foreground`.\n"]
    pub fn set_secondary_foreground(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secondary_foreground = Some(v.into());
        self
    }
    #[doc = "Set the field `success`.\n"]
    pub fn set_success(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.success = Some(v.into());
        self
    }
    #[doc = "Set the field `success_foreground`.\n"]
    pub fn set_success_foreground(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.success_foreground = Some(v.into());
        self
    }
    #[doc = "Set the field `warning`.\n"]
    pub fn set_warning(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.warning = Some(v.into());
        self
    }
    #[doc = "Set the field `warning_foreground`.\n"]
    pub fn set_warning_foreground(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.warning_foreground = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightThemeConfigurationElUiColorPaletteEl {
    type O = BlockAssignable<DataQuicksightThemeConfigurationElUiColorPaletteEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightThemeConfigurationElUiColorPaletteEl {}
impl BuildDataQuicksightThemeConfigurationElUiColorPaletteEl {
    pub fn build(self) -> DataQuicksightThemeConfigurationElUiColorPaletteEl {
        DataQuicksightThemeConfigurationElUiColorPaletteEl {
            accent: core::default::Default::default(),
            accent_foreground: core::default::Default::default(),
            danger: core::default::Default::default(),
            danger_foreground: core::default::Default::default(),
            dimension: core::default::Default::default(),
            dimension_foreground: core::default::Default::default(),
            measure: core::default::Default::default(),
            measure_foreground: core::default::Default::default(),
            primary_background: core::default::Default::default(),
            primary_foreground: core::default::Default::default(),
            secondary_background: core::default::Default::default(),
            secondary_foreground: core::default::Default::default(),
            success: core::default::Default::default(),
            success_foreground: core::default::Default::default(),
            warning: core::default::Default::default(),
            warning_foreground: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightThemeConfigurationElUiColorPaletteElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightThemeConfigurationElUiColorPaletteElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightThemeConfigurationElUiColorPaletteElRef {
        DataQuicksightThemeConfigurationElUiColorPaletteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightThemeConfigurationElUiColorPaletteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accent` after provisioning.\n"]
    pub fn accent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.accent", self.base))
    }
    #[doc = "Get a reference to the value of field `accent_foreground` after provisioning.\n"]
    pub fn accent_foreground(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accent_foreground", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `danger` after provisioning.\n"]
    pub fn danger(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.danger", self.base))
    }
    #[doc = "Get a reference to the value of field `danger_foreground` after provisioning.\n"]
    pub fn danger_foreground(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.danger_foreground", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dimension", self.base))
    }
    #[doc = "Get a reference to the value of field `dimension_foreground` after provisioning.\n"]
    pub fn dimension_foreground(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dimension_foreground", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `measure` after provisioning.\n"]
    pub fn measure(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.measure", self.base))
    }
    #[doc = "Get a reference to the value of field `measure_foreground` after provisioning.\n"]
    pub fn measure_foreground(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.measure_foreground", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `primary_background` after provisioning.\n"]
    pub fn primary_background(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_background", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `primary_foreground` after provisioning.\n"]
    pub fn primary_foreground(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_foreground", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_background` after provisioning.\n"]
    pub fn secondary_background(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secondary_background", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_foreground` after provisioning.\n"]
    pub fn secondary_foreground(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secondary_foreground", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `success` after provisioning.\n"]
    pub fn success(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.success", self.base))
    }
    #[doc = "Get a reference to the value of field `success_foreground` after provisioning.\n"]
    pub fn success_foreground(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.success_foreground", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `warning` after provisioning.\n"]
    pub fn warning(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.warning", self.base))
    }
    #[doc = "Get a reference to the value of field `warning_foreground` after provisioning.\n"]
    pub fn warning_foreground(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.warning_foreground", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataQuicksightThemeConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_color_palette: Option<ListField<DataQuicksightThemeConfigurationElDataColorPaletteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sheet: Option<ListField<DataQuicksightThemeConfigurationElSheetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    typography: Option<ListField<DataQuicksightThemeConfigurationElTypographyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ui_color_palette: Option<ListField<DataQuicksightThemeConfigurationElUiColorPaletteEl>>,
}
impl DataQuicksightThemeConfigurationEl {
    #[doc = "Set the field `data_color_palette`.\n"]
    pub fn set_data_color_palette(
        mut self,
        v: impl Into<ListField<DataQuicksightThemeConfigurationElDataColorPaletteEl>>,
    ) -> Self {
        self.data_color_palette = Some(v.into());
        self
    }
    #[doc = "Set the field `sheet`.\n"]
    pub fn set_sheet(
        mut self,
        v: impl Into<ListField<DataQuicksightThemeConfigurationElSheetEl>>,
    ) -> Self {
        self.sheet = Some(v.into());
        self
    }
    #[doc = "Set the field `typography`.\n"]
    pub fn set_typography(
        mut self,
        v: impl Into<ListField<DataQuicksightThemeConfigurationElTypographyEl>>,
    ) -> Self {
        self.typography = Some(v.into());
        self
    }
    #[doc = "Set the field `ui_color_palette`.\n"]
    pub fn set_ui_color_palette(
        mut self,
        v: impl Into<ListField<DataQuicksightThemeConfigurationElUiColorPaletteEl>>,
    ) -> Self {
        self.ui_color_palette = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightThemeConfigurationEl {
    type O = BlockAssignable<DataQuicksightThemeConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightThemeConfigurationEl {}
impl BuildDataQuicksightThemeConfigurationEl {
    pub fn build(self) -> DataQuicksightThemeConfigurationEl {
        DataQuicksightThemeConfigurationEl {
            data_color_palette: core::default::Default::default(),
            sheet: core::default::Default::default(),
            typography: core::default::Default::default(),
            ui_color_palette: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightThemeConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightThemeConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataQuicksightThemeConfigurationElRef {
        DataQuicksightThemeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightThemeConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_color_palette` after provisioning.\n"]
    pub fn data_color_palette(
        &self,
    ) -> ListRef<DataQuicksightThemeConfigurationElDataColorPaletteElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_color_palette", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sheet` after provisioning.\n"]
    pub fn sheet(&self) -> ListRef<DataQuicksightThemeConfigurationElSheetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sheet", self.base))
    }
    #[doc = "Get a reference to the value of field `typography` after provisioning.\n"]
    pub fn typography(&self) -> ListRef<DataQuicksightThemeConfigurationElTypographyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.typography", self.base))
    }
    #[doc = "Get a reference to the value of field `ui_color_palette` after provisioning.\n"]
    pub fn ui_color_palette(
        &self,
    ) -> ListRef<DataQuicksightThemeConfigurationElUiColorPaletteElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ui_color_palette", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataQuicksightThemePermissionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    actions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principal: Option<PrimField<String>>,
}
impl DataQuicksightThemePermissionsEl {
    #[doc = "Set the field `actions`.\n"]
    pub fn set_actions(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.actions = Some(v.into());
        self
    }
    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.principal = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightThemePermissionsEl {
    type O = BlockAssignable<DataQuicksightThemePermissionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightThemePermissionsEl {}
impl BuildDataQuicksightThemePermissionsEl {
    pub fn build(self) -> DataQuicksightThemePermissionsEl {
        DataQuicksightThemePermissionsEl {
            actions: core::default::Default::default(),
            principal: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightThemePermissionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightThemePermissionsElRef {
    fn new(shared: StackShared, base: String) -> DataQuicksightThemePermissionsElRef {
        DataQuicksightThemePermissionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightThemePermissionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }
    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}
