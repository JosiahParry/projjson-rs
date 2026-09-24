/// An object with `$schema`, its own members, then `id` / `ids`.
macro_rules! identified_object {
    ($(#[$m:meta])* pub struct $name:ident { $($body:tt)* }) => {
        $(#[$m])*
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct $name {
            #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
            pub schema: Option<String>,
            $($body)*
            #[serde(skip_serializing_if = "Option::is_none")]
            pub id: Option<$crate::common::Id>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ids: Option<Vec<$crate::common::Id>>,
        }

        impl $name {
            pub(crate) fn validate_common(&self) -> Result<(), $crate::validate::ValidationError> {
                $crate::validate::check_id_ids(&self.id, &self.ids)
            }
        }
    };
}

/// An object with `$schema`, its own members, then the `object_usage` members.
macro_rules! usage_object {
    ($(#[$m:meta])* pub struct $name:ident { $($body:tt)* }) => {
        $(#[$m])*
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct $name {
            #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
            pub schema: Option<String>,
            $($body)*
            #[serde(skip_serializing_if = "Option::is_none")]
            pub scope: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub area: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub bbox: Option<$crate::common::BBox>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub vertical_extent: Option<$crate::common::VerticalExtent>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub temporal_extent: Option<$crate::common::TemporalExtent>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub usages: Option<Vec<$crate::common::Usage>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub id: Option<$crate::common::Id>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ids: Option<Vec<$crate::common::Id>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub remarks: Option<String>,
        }

        impl $name {
            pub(crate) fn validate_common(&self) -> Result<(), $crate::validate::ValidationError> {
                use $crate::validate::Validate;
                self.vertical_extent
                    .validate()
                    .map_err(|e| e.in_field("vertical_extent"))?;
                self.usages.validate().map_err(|e| e.in_field("usages"))?;
                $crate::validate::check_id_ids(&self.id, &self.ids)
            }
        }
    };
}
